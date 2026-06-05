# fsci-ndimage separable filters: single-threaded -> multithreaded output pixels

## Target (bead frankenscipy-n2mqb; follows median/rank da24dd11)

correlate1d and the generic per-axis filter1d (the core of uniform_filter /
gaussian_filter) computed each output pixel in a sequential `for flat_out` loop. Each
output pixel is an independent reduction over a read-only window of `input`.

## Lever (one)

Add a generic `fill_pixels_parallel<G: Fn(usize,&mut Vec<f64>)->f64 + Sync>` driver:
split output.data into disjoint chunks_mut across std::thread::scope workers; each
thread reuses one scratch buffer across its pixels. Applied to
correlate1d_with_origin and filter1d_axis_with_origin (added `+ Sync` to its reduce
bound) -> covers correlate1d / uniform_filter / gaussian_filter. Gated by
ndimage_filter_thread_count (work = pixels * size) below 2^18.

## Isomorphism / proof (BYTE-IDENTICAL)

output[flat_out] depends only on a read-only window; disjoint outputs, same per-pixel
computation regardless of owning thread. New test
separable_filter_parallel_is_bit_identical (600x600 > gate; Reflect/Constant/Nearest;
both axes; f64::to_bits equal vs a verbatim sequential reference). fsci-ndimage 224
passed / 0 failed; clippy + fmt clean.

## Rebench (perf_separable_filter, square image, same worker via stash)

| op | size | before (seq) | after (parallel) | speedup |
| --- | --- | ---: | ---: | ---: |
| correlate1d | 800x800 k9 | 95.45 ms | 7.76 ms | 12.3x |
| correlate1d | 1200x1200 k21 | 464.21 ms | 26.08 ms | 17.8x |
| uniform_filter | 800x800 k9 | 198.30 ms | 17.55 ms | 11.3x |
| uniform_filter | 1200x1200 k21 | 956.67 ms | 65.21 ms | 14.7x |

Byte-identical, Score >> 2.0. ndimage filter family (median/rank + separable
correlate/uniform/gaussian) now all multicore. Same vein as cdist/pdist.
