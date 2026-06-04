# separable min/max: eliminate per-element Vec allocations (2.4-2.8x)

Bench: `minmax_filter` / `binary_morph` (fsci-ndimage), rch ts2. Function:
`minmax_filter_along_axis` (shared by minimum/maximum_filter, grey
erosion/dilation, binary erosion/dilation, morphological gradient/laplace).

## Lever
The O(N*ndim) separable sliding-window deque already had the right complexity,
but allocated two Vecs *per element*: `arr.unravel(flat)` (to find line heads)
and `arr.get_boundary(&full, ...)` (a fresh `vec![ndim]` per neighbourhood read).
Removed both:
- line heads via the O(1) test `(flat / stride).is_multiple_of(shape[axis])`
  instead of an unravel allocation;
- a `boundary_index_1d` helper that maps one axis coordinate under the boundary
  mode and indexes `arr.data` directly, replacing the per-read `get_boundary`
  allocation;
- the per-line `VecDeque` is hoisted and `clear()`-ed instead of reallocated.

## Isomorphism
`boundary_index_1d` mirrors `NdArray::get_boundary`'s per-axis arithmetic exactly
(Reflect/Constant/Nearest/Wrap), so every neighbourhood value is bit-identical
and the deque/total_cmp logic is unchanged. The existing
`separable_minmax_matches_rank_filter_byte_for_byte` (`.to_bits()` vs the rank
filter over 1D/2D/3D, all modes, sizes, origins, NaN/±0.0) and
`binary_morph_separable_matches_naive_loop` both still pass. 6 min/max + 12 grey
+ 23 binary morphology tests pass; clippy + fmt clean.

## Benchmark (256x256, rch ts2, same-worker A/B: get_boundary vs 1D mapper)
| case             | before   | after    | Score |
|------------------|----------|----------|-------|
| minimum size=15  | 7.91 ms  | 3.27 ms  | 2.42x |
| maximum size=15  | 7.34 ms  | 2.62 ms  | 2.80x |
| dilation size=15 | 7.64 ms  | 3.25 ms  | 2.35x |

Constant-factor (allocation-elimination) win that compounds on top of the
algorithmic c1fb7380/d41afd80 separable rewrite and propagates to every
function that routes through `separable_minmax_filter`.
