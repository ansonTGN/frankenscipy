# perf: csr_matvec parallel SpMV — byte-identical, 2-3.8x kernel / 2.3-2.7x eigsh

## Lever (ONE)
`csr_matvec` (crates/fsci-sparse/src/linalg.rs) — the sparse matrix-vector
product — ran a serial row loop. It is the inner kernel of every sparse iterative
solver (`cg`, `pcg`, `gmres`, `lgmres`, `bicg`, `bicgstab`, `cgs`, `qmr`,
`minres`, `lsqr`, `lsmr`), the eigensolvers (`eigsh`, `eigs`, `svds` power
iteration), and `onenormest`, so it runs once per iteration of all of them.

Each output row `result[i] = Σ_idx data[idx]·x[indices[idx]]` is an independent
dot product accumulated in CSR index order. Split the rows across threads → the
per-row value is identical regardless of which thread computes it, so the result
is **byte-identical** to the serial sweep. Workers are scaled by WORK
(≈128K nnz/thread) and gated above ~256K nnz so small/medium matvecs don't pay
unamortized thread-spawn overhead.

## Parity — BYTE-IDENTICAL
- Same per-row dot product, same CSR accumulation order, written to its own
  `result[i]` slot in row order → identical `f64` bits. Same-process A/B
  (serial vs parallel kernel) reports `identical=true` (`.to_bits()` equality)
  for every size, n=20K…500K, nnz=200K…10M. See `golden_payload.txt`.
- All 310 `fsci-sparse` tests pass (incl. cg/gmres/bicgstab/eigsh/svds suites).
- The "CSR-add parallelism is bandwidth-bound dead end" prior result does NOT
  apply: that was scatter-merge of two sparse operands; SpMV is a clean
  gather-reduce per row with dense-vector reuse, which scales on many cores.

## Timing — rch remote, 64 cores, `--profile release-perf`
Kernel A/B (200 reps), random CSR matrices:

| n      | nnz    | serial    | parallel  | speedup |
|--------|--------|-----------|-----------|---------|
| 20000  | 200K   | 258 µs    | 235 µs    | ~1.0x (serial gate) |
| 50000  | 1.0M   | 1.435 ms  | 678 µs    | 2.1x    |
| 200000 | 4.0M   | 6.998 ms  | 2.584 ms  | 2.7x    |
| 500000 | 10.0M  | 18.706 ms | 4.965 ms  | 3.8x    |

End-to-end, public `eigsh` (power iteration, many matvecs), baseline serial vs
new parallel (stash A/B):

| n      | nnz    | baseline  | new       | speedup |
|--------|--------|-----------|-----------|---------|
| 100000 | 1.5M   | 314.4 ms  | 137.1 ms  | 2.29x   |
| 300000 | 4.5M   | 937.3 ms  | 350.0 ms  | 2.68x   |

Score ≥ 2.0 cleared from ~1M nnz up; no small-matrix regression (gate keeps
< 256K nnz serial). Win grows with nnz and compounds across solver iterations.

Harness: `crates/fsci-sparse/src/bin/perf_csr_matvec.rs`
Run: `cargo run --profile release-perf -p fsci-sparse --bin perf_csr_matvec`
