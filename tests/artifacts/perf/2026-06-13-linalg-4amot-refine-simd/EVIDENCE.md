# Mixed-precision LU: SIMD-vectorize the iterative-refinement residual (follow-up to 4amot)

## Profile that motivated this
Env/n-gated phase timers inside `lu_factor_blocked_f32` (n=1000, removed before commit):

    LU_PROFILE_F32 n=1000 panel+usolve=13.6 ms  trailing=26.3 ms   (f32 factor ≈ 40 ms)

The full mixed solve was ~66 ms, so ~26 ms (≈39%) was NOT in the factorization — it was the
iterative-refinement residual `r = b − A·x`, computed as a scalar, bounds-checked dot product
over `&[Vec<f64>]` (re-reading the whole 8 MB f64 matrix each refinement step). On fast workers
that overhead ate the entire f32-factorization saving, leaving mixed precision ≈ break-even.

## Lever
Replace the scalar residual dot product with a 4×8-wide SIMD reduction (`Simd<f64,8>`, four
accumulators) over the contiguous row + x slices. Math is the same f64 dot product (blocked
reduction order); the residual stays full f64 and the refinement still converges to the f64
backward-error bar. Only `lu_solve_mixed_precision`'s residual loop changed.

## Parity (unchanged, absolute)
`cargo test -p fsci-linalg --release --lib -- --include-ignored`: **430 passed, 0 failed**.
- `flat_lu_golden_digest` still `0x2fc8ed294ef0427c` (f64 LU path untouched).
- `lu_solve_mixed_precision_matches_f64_and_falls_back` green (matches f64 ref < 1e-9, residual
  < 1e-9, ill-conditioned still declines → exact f64).

## Benchmark
Same-worker A/B (vmi1149989, one binary, `DISABLE_MIXED_LU` toggle):
| arm                              | time (median)            |
|----------------------------------|--------------------------|
| `baseline_solve/1000x1000`     (mixed) | **46.249 ms** [45.388 47.109] |
| `baseline_solve/1000x1000_f64` (f64)   | **61.949 ms** [60.927 62.994] |
=> **1.34x** vs f64 on this (fast) worker.

Same-worker before/after for THIS lever (mixed arm, vmi1149989):
- baee89ed (scalar refinement): **61.786 ms**
- this commit (SIMD refinement): **46.249 ms**  → **1.34x faster mixed solve**

Pre-vectorization, mixed was ≈ break-even with f64 on this fast worker (61.79 vs 61.95 ms); the
SIMD refinement makes the mixed-precision win robust across worker speeds (it was 1.49x on the
slower vmi1152480 in baee89ed and is larger still there now that refinement is cheap).

## Next route
Phase profile now: trailing GEMM 26 ms is the single largest factor phase. Next lever is
cache-blocking (L2-resident U12 panel) of the register-blocked trailing update — bit-identical
(loop-reorder only, same per-element monotonic-p reduction) so it preserves the golden digest,
and it helps the f64 path too. Filed under frankenscipy-l566o (re-scoped from recursive-panel,
which the profile shows caps low: panel+usolve is only ~20% of solve time).
