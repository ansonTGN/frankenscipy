# fsci-interpolate polymul: direct O(m·n) convolution -> FFT O((m+n)log) above crossover

## Target (bead frankenscipy-g7lvw)

polymul (crates/fsci-interpolate/src/lib.rs) was a naive O(m·n) double-loop
convolution. Bench polymul/512x512 baseline (RCH criterion): ~50.9 us.

## Lever (one)

Route to fsci_fft::fftconvolve(a, b, "full") — O((m+n)log(m+n)) — when the direct
work m·n dominates the FFT work L·log2(L) (L = next_pow2(m+n-1)) by a 20x margin;
otherwise keep the exact direct loop. The cost-model gate (vs a naive size gate)
correctly routes small AND lopsided inputs (where direct is faster) to the exact
loop, and only takes FFT in its clear-win regime.

## Parity (tolerance, gated)

numpy.polymul is the direct loop; FFT matches it to FFT rounding. Measured max
relative error vs a verbatim direct convolution: <= 8e-12 across n=512..8192, far
inside the 1e-10 conformance tolerance (diff_fft_polymul_analytic_signal). Every
conformance case (<= 8 coeffs) and every small/lopsided call stays on the
byte-identical direct path — n=256 routes to direct (speedup 1.00x, max_rel 0).
New unit test polymul_fft_path_matches_direct_reference asserts < 1e-10 at
128/512/300x700. fsci-interpolate 124 passed / 0 failed.

## Same-process A/B sweep (perf_polymul bin; more stable than cross-worker criterion)

| n (balanced) | direct | FFT (shipped) | speedup | max_rel |
| ---: | ---: | ---: | ---: | ---: |
| 256 | 11.77us | 11.81us (direct path) | 1.00x | 0 |
| 512 | 48.50us | 27.06us | 1.79x | 1.3e-12 |
| 1024 | 182.5us | 60.49us | 3.02x | 4.9e-13 |
| 2048 | 759.1us | 142.6us | 5.32x | 1.1e-13 |
| 4096 | 3287us | 310.1us | 10.60x | 2.8e-13 |
| 8192 | 13149us | 661.7us | 19.87x | 8.0e-12 |

True O(m·n) -> O((m+n)log): the speedup grows without bound with degree. Score at
the bench size (512) is 1.79x, but >=2.0 from n>=1024 (3.0-19.9x); the complexity
class is fixed, which is the point. No regression below the crossover.
