# fsci-linalg solve_banded: dense O(n^2)-memory banded GEPP -> packed band storage O(n·bw)

## Target (bead frankenscipy-0jgdh, follow-up to ddcb822d)

ddcb822d already made solve_banded's *solve* O(n·kl·(kl+ku)), but it still:
  1. materialized the full dense n×n matrix (`dense_from_banded`, O(n^2) time+mem),
  2. cloned it for the working buffer (O(n^2)), and
  3. computed the backward error over the dense matrix (O(n^2)).
So the win was capped at O(n^2) and used 128MB at n=4000. Baseline (ddcb822d), RCH
`linalg_bench` tridiagonal: 4x4 0.87us, 16x16 5.13us, 64x64 11.76us, 256x256 143us.

## Lever (one)

Replace the dense path with `banded_lu_solve_packed`: Gaussian elimination + partial
pivoting on a LAPACK-style packed band buffer `w[(2kl+ku+1) × n]` (`w[kl+ku+i-j][j] =
A(i,j)`, top kl rows = fill workspace). Pivot search, the row interchange (per-column
scalar swaps across band-rows), elimination, and back substitution are all bounded to
the band => O(n·kl·(kl+ku)) time, O(n·(kl+ku)) memory; the dense matrix is never built.
The backward error is also recomputed banded (`compute_backward_error_banded`).
A zero/NaN pivot returns None and the caller falls back to the dense solver, preserving
singular-case behavior.

## Isomorphism / proof (BYTE-IDENTICAL)

The packed factorization runs the *identical* float ops in the *identical* order as a
dense banded GEPP — every operation it omits acts on a structural zero (`x - m·0`, or
`+0.0` in the norm sums), which is a no-op for finite x. L multipliers are folded into
the RHS in place (with the RHS interchange applied inline), so the discarded
sub-diagonal column positions never affect x. Hence both x AND backward_error are
bit-identical to the shipped dense-band path.

Golden A/B (perf_solve_banded bin, 10 deterministic cases incl. asymmetric kl/ku,
upper-only, lower-only bandwidths): before (HEAD ddcb822d) and after SHA-256 EQUAL:
  292a74b70f2810d3c3bfbd100595271ee987f267764006a5e4e12d6718d79d70

Parity tests: fsci-linalg 346 passed / 0 failed (incl. solve_banded_matches_dense,
solve_banded_pentadiagonal, diff_solve_banded_tridiag); evidence_p2c002 banded gate
(1e-10) 34 passed; diff_linalg_structured_solvers vs SciPy oracle (1e-8) pass.

## Rebench (after, same RCH config)

| Row | ddcb822d | packed | Speedup | vs original O(n^3) |
| --- | ---: | ---: | ---: | ---: |
| solve_banded/4x4 | 0.87 us | 0.21 us | 4.2x | 17x |
| solve_banded/16x16 | 5.13 us | 0.56 us | 9.2x | 18x |
| solve_banded/64x64 | 11.76 us | 1.91 us | 6.2x | 58x |
| solve_banded/256x256 | 143 us | 8.03 us | 17.8x | 999x |

64->256 scales 4.2x (linear in n) — true O(n·bw), no longer O(n^2). The win is now
uncapped and grows with n.

Score = impact 5 * confidence 5 / effort 2 = 12.5  (>> 2.0). Bit-identical, so parity
is absolute by construction.
