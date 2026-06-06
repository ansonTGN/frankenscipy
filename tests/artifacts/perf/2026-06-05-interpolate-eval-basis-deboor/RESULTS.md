# perf(fsci-interpolate): eval_basis_all restricted in-place de-Boor (O(n*k)->O(n+k^2))

## Lever
eval_basis_all evaluated the length-n B-spline basis by sweeping all n indices at each
of the k Cox-de Boor levels, cloning the length-n vector every level -> O(n*k) work plus
k allocations per call. The basis has local support [mu-k, mu]; restrict each level's
recursion to the active span and recurse IN PLACE (ascending, so basis[i]/basis[i+1] are
still the previous level's values when read) -> O(n + k^2), zero per-level clones.

## Bit-identity
Same float expressions, same guards, same accumulation; skipped indices are provably 0
(they were 0 at the prior level and stay 0). perf_lsq_spline coeff digests OLD==NEW:
  8117653a3d4335a0 / d4069f023726cecb / 2fbddeee741f0330

## Bench (perf_lsq_spline make_lsq_spline build, release-perf, min of 3, ms/build)
| ncoeff | OLD  | NEW  | speedup |
|-------:|-----:|-----:|--------:|
| 200    | 4.54 | 1.93 | 2.35x   |
| 400    | 21.9 | 12.3 | 1.79x   |
| 600    | 59.9 | 35.7 | 1.68x   |
Ratio decays vs n because make_lsq_spline still has untouched O(n) per-sample work
(degree-0 interval scan, nz scan, full A^T y loop, length-n alloc). Benefits ALL spline
callers (make_smoothing_spline, BSpline eval); 125 fsci-interpolate tests pass.

## NEXT (to reach O(m*k^2) build): binary-search the knot interval (replace the O(n)
degree-0 scan) + sparse active-window assembly (scratch buffer, O(k^2) aty/ata). Needs a
byte-exact interval finder validated against the linear predicate incl. clamped/repeated
knots and the x==t[n] right-boundary clause.
