# order_filter: per-window sort → sliding ordered-multiset rank query

Bench: `order_filter` (fsci-signal), rch ts2. Function: `order_filter`.

## Lever
The 1D rank/order filter sorted every length-k window (O(n·k·log k)) and indexed
the requested rank. The `SlidingMedian` from the medfilt commit (9c69bd13) was
generalized to `SlidingRankWindow` — two `total_cmp`-ordered `BTreeMap`
multisets with `lower` held at `min(rank+1, total)` elements, so `lower.max()` is
the element at sorted index `min(rank, total-1)`. `order_filter` now slides this
window (O(log k) per step) for window sizes >= 32 → O(n·log k). The 1D window is
*clipped* (not zero-padded) at the borders, so the driver grows/holds/shrinks the
monotone `[start, end)` bounds and the multiset tracks exactly those contents.
medfilt now reuses the same structure with rank = k/2.

## Isomorphism
`value()` returns the rank element of the current window, exactly what
`sort` + `[rank.min(len-1)]` returns (out-of-range ranks clamp identically).
Proven by `order_filter_sliding_matches_naive_sort`: `.to_bits()` equality vs the
per-window sort across n in {1,20,200,777}, window sizes {31,32,65,200}, ranks
(low/mid/high/out-of-range), tie densities continuous→heavy, and signed zeros —
including the clipped boundary windows. The medfilt equivalence test and all
order_filter/medfilt tests still pass; clippy + fmt clean.

## Benchmark (8192-sample signal, rank = window/4, rch ts2, naive vs sliding)
| case                    | naive (sort) | sliding  | Score |
|-------------------------|--------------|----------|-------|
| order_filter 8192 w65   | 6.237 ms     | 1.426 ms | 4.37x |
| order_filter 8192 w257  | 39.315 ms    | 2.210 ms | 17.8x |

Naive scales O(n·k·log k); sliding O(n·log k), so the ratio grows with window
size (bigger wins than medfilt, whose naive used O(k) selection rather than a
sort). One generalized structure now powers both medfilt and order_filter.
