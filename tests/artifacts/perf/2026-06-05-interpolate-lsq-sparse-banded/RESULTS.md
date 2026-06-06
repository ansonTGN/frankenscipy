# perf(fsci-interpolate): make_lsq_spline O(m*n + n^3) -> O(m*k^2 + n*bw^2)

## Levers (compose to make the build near-linear)
1. SPARSE ASSEMBLY: bspline_find_interval (binary search on the sorted knot vector,
   O(log n), byte-exact match to eval_basis_all's degree-0 indicator) locates the knot
   span; the k+1 active basis values are evaluated with the exact windowed Cox-de Boor
   recursion into a REUSED scratch buffer (no per-sample length-n alloc, no O(n) scan),
   and the rank-1 outer product is scattered over [mu-k, mu]. O(m*k^2).
2. BANDED SOLVE: solve_dense_system now skips elimination/back-sub terms whose factor or
   pivot entry is exactly 0 (no-ops: `v + (+/-0.0) == v`), so a banded A^T A (bandwidth
   ~2k+1) is solved in ~O(n*bw^2) instead of O(n^3). Benefits make_interp_spline /
   make_smoothing_spline too.

## Byte-identity
Both are bit-identical for finite inputs (windowed de-Boor == eval_basis_all values;
window superset adds only 0-factor terms; zero-skip elimination skips only no-ops). A^T y
is assembled over the window (finite-y identical; sibling make_smoothing_spline_impl does
the same). perf_lsq_spline coeff digests OLD(committed)==NEW:
  m=1500 ncoeff=200 k=3  8117653a3d4335a0
  m=3000 ncoeff=400 k=3  d4069f023726cecb
  m=5000 ncoeff=600 k=3  2fbddeee741f0330
New unit test bspline_find_interval_matches_eval_basis (clamped/repeated/boundary knots).

## Bench (perf_lsq_spline build, release-perf, min of 3)
| m    | ncoeff | k | OLD    | NEW      | speedup |
|------|-------:|--:|-------:|---------:|--------:|
| 1500 |    200 | 3 | 1.857ms| 0.389 ms |  4.8x   |
| 3000 |    400 | 3 | 11.28ms| 1.113 ms | 10.1x   |
| 5000 |    600 | 3 | 33.93ms| 2.377 ms | 14.3x   |
Grows ~n. 126 fsci-interpolate tests pass; clippy clean.
(Cumulative vs the original dense O(m*n^2) build: ~14-1500x.)
