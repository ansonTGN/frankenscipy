# frankenscipy-o19mg rejection: parallel register-blocked LU trailing update

## Target
Parallelize the committed single-threaded register-blocked trailing update
`A22 -= L21·U12` (e45ebe15) over disjoint output-row ranges, targeting solve/1000 >= 2x.

## Lever (implemented, stashed)
- Extracted the tile kernel into `lu_trailing_update_rows(out_rows, upper, n, k, kb)`.
- `data.split_at_mut(kb*n)` -> read-only `upper` (U12 rows) + mutable `lower` (A22+L21 rows);
  partitioned `lower` into row-chunks via `std::thread::scope` + `chunks_mut`, work-gated
  (`trailing_update_thread_count`, threshold 8M macs, cap cores.min(rows/64)).
- Stash: `o19mg-parallel-trailing-REJECT-contention-BlackThrush`.

## Isomorphism proof (PASSED)
Every output element computes `out - Σ_{p∈k..kb} L[p]·U[p]` with monotonic-p reduction,
and the SIMD-MR4 and scalar-tail paths are per-element identical, so the result is
bit-identical regardless of row partition. Verified: full release lib suite
`cargo test -p fsci-linalg --release --lib -- --include-ignored` = **429 passed, 0 failed,
0 ignored**, including `flat_lu_golden_digest` (asserts golden == 0x2fc8ed294ef0427c,
UNCHANGED). Parity/reference solve/inv tests (n=130/200/270) green.

## Benchmark (REJECT)
Same bench as e45ebe15, `baseline_solve/1000x1000`, sample-size 60-100, RCH fleet:
- committed single-threaded register-blocked (e45ebe15): ~96-104 ms (tight CI), vmi1149989/vmi1152480
- parallel (this lever), vmi1153651 (2 slots busy): **[138.66 ms 152.67 ms 171.83 ms]**
- parallel (this lever), vmi1156319 (1 slot busy):  **[109.90 ms 112.60 ms 115.48 ms]**

Both runs regress (1.1x–1.5x SLOWER): the first under heavy contention (±11% CI), the second
under lighter load (±2.5% CI) still ~10% slower than the sequential kernel. No measurable win.

## Root cause
The n=1000 trailing update is memory-bandwidth-bound (A22 ~7 MB, already read once by the
register-blocked kernel). Spawning cores.min(rows/64) threads per panel step on a shared
multi-tenant RCH worker contends for cores + DRAM bandwidth, adding scheduling/sync variance
that dwarfs any compute parallelism. Confirms the prior 8l8r1.96 finding ("row-chunked /
parallel trailing updates remain slow", 179 ms on vmi1152480).

## Score
Impact 0 (regression) * Confidence 4 / Effort 3 = 0.0. Verdict: REJECT on the RCH fleet.
The bit-identical parallel kernel is preserved in the stash for a future single-tenant
deployment path, but it is NOT a measurable Score>=2.0 win in this benchmark environment.

## Next route (not micro-tuning this loop)
Solve/1000 hot path is bandwidth-bound, not compute-bound; thread-level parallelism does not
pay here. A genuine next lever must reduce DRAM traffic or total work, e.g. recursive
(cache-oblivious) LU that keeps panels resident, or mixed-precision iterative refinement
(factor in f32, refine in f64) to halve factorization bandwidth — both algorithmically
different, both preserving pivoting/residual parity.
