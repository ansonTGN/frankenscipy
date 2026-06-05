# perf(fsci-interpolate): make_lsq_spline A^T A build O(m*n^2)->O(m*n*k) (frankenscipy-nf753)

## Lever
make_lsq_spline builds the least-squares normal equations A^T A by accumulating, for
each of m samples, the outer product of a length-n B-spline basis vector. The basis has
local support (~k+1 nonzeros via Cox-de Boor), so the dense `for l in 0..n` inner loop is
O(m*n^2) of which all but O(m*k^2) is `0*0`. Restrict the A^T A accumulation to the
nonzero basis indices (`nz`). The A^T y loop is left full (O(m*n), not the bottleneck).

## Bit-identity proof
basis values are finite (in [0,1]), so every skipped (j,l) term is basis[j]*basis[l] with
a zero factor = +/-0.0, and `v + (+/-0.0) == v`. The i-major accumulation order is
preserved. Same recipe as the adjacent (already shipped) make_smoothing_spline_impl.

perf_lsq_spline coeff FNV digests, OLD (dense) vs NEW (nz), IDENTICAL:
  m=1500 ncoeff=200 k=3  digest=8117653a3d4335a0
  m=3000 ncoeff=400 k=3  digest=d4069f023726cecb
  m=5000 ncoeff=600 k=3  digest=2fbddeee741f0330

## Bench (perf_lsq_spline, release-perf, ms/build, same machine, mean of 5)
| m    | ncoeff | k | OLD (dense) | NEW (nz) | speedup |
|------|-------:|--:|------------:|---------:|--------:|
| 1500 |    200 | 3 |    51.35 ms |  4.14 ms |  12.4x  |
| 3000 |    400 | 3 |   373.28 ms | 19.93 ms |  18.7x  |
| 5000 |    600 | 3 |  1373.0  ms | 67.14 ms |  20.5x  |
Speedup grows ~n (O(m*n^2) -> O(m*n*k)). 125 fsci-interpolate tests pass; clippy clean.

Follow-up: eval_basis_all is O(n*k)/call (full-length loops + clone per de-Boor level);
making it sparse would drop the build to O(m*k^2). solve_dense_system on the banded A^T A
(bandwidth ~2k+1) is another O(n^3)->O(n*k^2) banded-solve lever.
