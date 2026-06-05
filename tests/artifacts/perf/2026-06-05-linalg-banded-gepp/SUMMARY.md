# fsci-linalg solve_banded: dense O(n^3) LU -> band-bounded GEPP O(n·kl·(kl+ku))

## Target (profile-backed)

`solve_banded` materialized the banded matrix into a full dense buffer and ran the
general LU solver — O(n^3) time, O(n^2) memory — for a system whose bandwidth is
tiny. The evidence harness (`evidence_p2c002.rs:499`) already documented this as a
known gap. Fresh RCH Criterion baseline (`linalg_bench`, solve_banded, tridiagonal):

| Row | Baseline median |
| --- | ---: |
| solve_banded/4x4 | 3.65 µs |
| solve_banded/16x16 | 10.25 µs |
| solve_banded/64x64 | 111.2 µs |
| solve_banded/256x256 | 8.02 ms |

64->256 (4x size) was 111µs->8.02ms = 72x => clearly cubic.

## Lever (one)

Replace the `dense_from_banded` + general `solve` call with `banded_gepp_solve`:
Gaussian elimination with partial pivoting whose pivot search, row updates, and
back-substitution are all bounded to the band + fill region. With partial pivoting
U's upper bandwidth grows to at most kl+ku and L's lower bandwidth stays kl, so the
work drops from O(n^3) to O(n·kl·(kl+ku)) (O(n) for tridiagonal). On a zero/NaN
pivot it returns None and the caller falls back to the original dense solver, so
singular/degenerate behavior is preserved byte-for-byte.

## Isomorphism / parity

- The banded GEPP performs the SAME Gaussian elimination as a dense GEPP: operations
  on the structurally-zero entries outside the band are `x - m·0` no-ops, and the
  partial-pivot column scan is identical because below-band column entries are zero.
- This is an algorithm replacement (vs the prior nalgebra dense LU), so the result
  is numerically equivalent, not bit-identical. Parity is proven by tolerance, the
  authoritative standard for these float solvers (tests use 1e-8 / 1e-10):
  - fsci-linalg: 346 passed / 0 failed, incl. `solve_banded_matches_dense`,
    `solve_banded_pentadiagonal`, `diff_solve_banded_tridiag`.
  - `evidence_p2c002` banded parity gate (banded vs dense, tol 1e-10): 2 passed.
  - `diff_linalg_structured_solvers` (vs SciPy oracle, ABS_TOL 1e-8): pass.

## Rebench (after, same RCH config)

| Row | After median | Speedup |
| --- | ---: | ---: |
| solve_banded/4x4 | 0.87 µs | 4.2x |
| solve_banded/16x16 | 5.13 µs | 2.0x |
| solve_banded/64x64 | 11.76 µs | 9.5x |
| solve_banded/256x256 | 143.0 µs | 56x |

The win grows with n; the residual cost is now the O(n^2) dense materialization +
dense backward-error, NOT the cubic solve. Band-storage (true O(n·bw) memory) is a
clean follow-up lever.

Score = impact 5 * confidence 5 / effort 2 = 12.5  (>> 2.0).
