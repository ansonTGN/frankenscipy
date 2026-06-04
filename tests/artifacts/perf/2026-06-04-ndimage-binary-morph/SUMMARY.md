# binary_erosion / binary_dilation: footprint scan → separable min/max

Bench: `binary_morph` (fsci-ndimage), rch ts2. Functions: `binary_erosion`,
`binary_dilation` (via the per-iteration `*_once` kernels).

## Lever
Binary erosion is a minimum filter, dilation a maximum filter, over the
booleanized image with a constant-0 border. The per-iteration kernels scanned
the full `size^ndim` footprint per pixel (erosion gather with early-break;
dilation scatter from every set pixel) — O(N * size^ndim). They now route through
the separable sliding-window deque min/max (O(N * ndim), independent of size).
Dilation uses the reflected structuring element (origin offset 0 for odd sizes,
-1 for even). Gated to the default all-zero origin so the kernel window and
origin validation match exactly; other origins keep the original path.

## Isomorphism
For a booleanized image the min/max are exact 0/1 selections, so the result is
bit-identical to the footprint scan. Proven by
`binary_morph_separable_matches_naive_loop` (`.to_bits()` equality vs inline
reference loops over 1D/2D/3D shapes, even/odd sizes 2-5, and non-binary inputs
that exercise booleanization and the dilation reflection). The 23 existing binary
morphology tests (scipy-reference, iterations, edges) pass; clippy + fmt clean.

## Benchmark (256x256 mostly-set image, rch ts2, naive vs separable)
| case             | naive       | separable | Score |
|------------------|-------------|-----------|-------|
| erosion size=7   | 24.68 ms    | 7.79 ms   | 3.2x  |
| dilation size=7  | 47.17 ms    | 7.52 ms   | 6.3x  |
| erosion size=15  | 23.07 ms    | 7.97 ms   | 2.9x  |
| dilation size=15 | 219.0 ms    | 7.67 ms   | 28.5x |

Erosion's naive early-break caps its ratio (~3x on this 0-sparse image; larger on
denser images); dilation's scatter has no early-out, so the ratio grows with
size (28x at size 15). Both reuse `separable_minmax_filter` from c1fb7380.
