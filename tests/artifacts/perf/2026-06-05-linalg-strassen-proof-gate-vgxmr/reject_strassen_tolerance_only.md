# Strassen GEMM Proof-Gate Rejection - `frankenscipy-vgxmr`

## Target

`frankenscipy-vgxmr` proposed recursive Strassen GEMM on top of the current parallel blocked GEMM.

The bead itself marks the lever as "tolerance-only" because Strassen changes the arithmetic tree.

## RCH Baseline

Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- cargo run --release -p fsci-linalg --bin perf_matmul --locked
```

Worker: `ts2`

Artifact: `baseline_perf_matmul_rch.txt`

SHA-256: `b77ad75c41f82fee9cc2cd4b79ca990513573760074aff3f7f975eecdacd0abe`

Rows:

- `n=1024`: `51.363 ms/matmul`, `41.8 GFLOP/s`
- `n=2048`: `195.251 ms/matmul`, `88.0 GFLOP/s`
- `n=4096`: `893.249 ms/matmul`, `153.9 GFLOP/s`

## Behavior Gate

Current public `matmul` is not tolerance-only. The in-crate contract is bit identity:

- `matmul_microkernel_is_bit_identical_to_flat_ikj` compares `f64::to_bits`.
- `matmul_flat_workspace_is_bit_identical_to_naive_ijk` compares `f64::to_bits`.
- `matmul_flat_compute_rows_row_split_is_bit_identical` compares full-row output splits with `f64::to_bits`.
- `matmul_microkernel_golden_digest` freezes the raw output bit digest.

RCH proof command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --lib matmul_microkernel_golden_digest --locked -- --nocapture
```

Artifact: `cargo_test_matmul_golden_rch.txt`

SHA-256: `d609a3252901a786d7417de2bb258d663cfbab18ebf77c238f014766a0304d5f`

Result: `1 passed; 0 failed`.

## Rejection

Strassen computes each output tile through seven linear combinations and recursive products. That changes the per-entry reduction tree and therefore changes ordinary IEEE-754 rounding for general `f64` matrices. A `max_rel < 1e-9` proof would not preserve the existing `to_bits`/golden-SHA contract, and it would fail this campaign's explicit floating-point parity requirement.

No source code was changed. Score: `0.0`.

## Next Exact Primitive

Continue the no-gaps GEMM work through exact-order kernels rather than tolerance-only Strassen:

- autotuned `MR x NR` register blocks that keep `k` monotonic per `c[i][j]`
- packed panel/layout improvements that do not change each output's reduction order
- exact-order parallel row/column partitioning with disjoint writes
- cache-oblivious traversal only when each scalar output still performs the same `k = 0..ka` accumulation sequence

The umbrella safe-Rust BLAS/LAPACK lane remains `frankenscipy-8l8r1`, currently owned by `OliveSnow`; this bead should not collide with that in-progress claim.
