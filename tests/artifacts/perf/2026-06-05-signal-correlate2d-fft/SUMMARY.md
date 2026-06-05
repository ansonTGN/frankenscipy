# fsci-signal correlate2d: direct O(ar·ac·vr·vc) -> FFT 2D convolution above crossover

## Target (bead frankenscipy-tjars)

correlate2d (crates/fsci-signal/src/lib.rs) was pure direct 2D convolution
O(ar·ac·vr·vc) with NO FFT path — catastrophic for large kernels (e.g. 200x200 *
200x200 ~ 4.8e8 mul-adds/call, and it grows as kernel^2).

## Lever (one)

Add `correlate2d_fft_full_into`: zero-pad `a` and the reversed kernel to lr×lc
(powers of two >= full_r/full_c), fft2 both, pointwise-multiply, ifft2, take the
real full_r×full_c block — the existing mode/region extraction is unchanged.
Gate with a cost model: take FFT only when direct work dominates FFT work,
`direct_ops(=ar·ac·vr·vc) > 24 · L·log2(L)` (L = lr·lc). Measured break-even is
~21 (direct ≈ 0.29 ns/op, FFT ≈ 6 ns per L·log2 L unit); 24 adds a safety margin.

## Isomorphism / parity

- Small or big-image/small-kernel shapes stay on the byte-identical direct loop
  (max_abs = 0 in the A/B for 64x8 .. 512x32). EVERY conformance case (<=6x6 *
  <=3x3) is far below the crossover => direct => the 1e-12 conformance test
  (diff_signal_correlate2d) passes unchanged.
- The FFT path matches direct to FFT rounding (max_rel <= 5e-10 across the win
  cases), the same tolerance trade the crate already makes in fftconvolve /
  convolve auto-dispatch. New unit test correlate2d_fft_path_matches_direct_reference
  checks all three modes (Full/Same/Valid) at 96x48, 128x96, 150x150 (< 1e-7).
  fsci-signal correlate2d tests pass; lib clippy + fmt clean.

## Same-process A/B sweep (perf_correlate2d bin)

| img | kernel | direct | FFT | speedup | parity |
| ---: | ---: | ---: | ---: | ---: | --- |
| 64  | 8   | 0.217ms | (direct) | 1.0x | byte-identical |
| 128 | 16  | 2.711ms | (direct) | 1.0x | byte-identical |
| 256 | 16  | 9.310ms | (direct) | 1.0x | byte-identical |
| 512 | 32  | 83.85ms | (direct) | 1.0x | byte-identical |
| 256 | 64  | 72.62ms | 27.77ms | 2.61x | max_rel 7e-11 |
| 256 | 128 | 286.5ms | 30.88ms | 9.28x | max_rel 5e-10 |
| 384 | 96  | 334.8ms | 31.07ms | 10.78x | max_rel 2e-10 |
| 200 | 200 | 485.4ms | 28.65ms | 16.94x | max_rel 1e-10 |

True O(ar·ac·vr·vc) -> O(L·log L): speedup grows without bound as the kernel grows.
No regression below the crossover. Score >> 2.0.
