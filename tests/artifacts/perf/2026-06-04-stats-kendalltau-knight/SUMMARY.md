# kendalltau: O(n²) pair counting → Knight (1966) O(n log n)

Bench: `rank_correlation` (fsci-stats), rch ts2. Functions: `kendalltau`,
`kendalltau_alternative` (shared `kendall_pair_counts`).

## Lever
Both functions counted concordant/discordant/x-tie/y-tie pairs with an O(n²)
double loop. scipy.stats.kendalltau uses Knight's O(n log n) merge-sort method;
this closes that vs-upstream algorithmic gap. The new `kendall_pair_counts`:
- x_ties / y_ties: sum t(t-1)/2 over equal-value runs (one sort each).
- joint ties: equal (x,y) runs after a lexicographic (x,y) sort.
- discordant: strictly-inverted y pairs (merge-sort inversion count) over the
  (x,y)-lexsorted order.
- concordant = tot - x_ties - y_ties + joint_ties - discordant.

Dispatched: Knight for n >= 256 and NaN-free inputs; the original O(n²) loop
otherwise (small n and the NaN edge keep the exact legacy behaviour, including
the small-n exact p-value path).

## Isomorphism
The Knight path returns the *same integer* (concordant, discordant, x_ties,
y_ties) as the O(n²) loop, so the downstream tau and p-value arithmetic is
bit-for-bit identical. Proven by `kendall_knight_matches_naive_on_large_inputs`
(asserts Knight == naive counts across n ∈ {256,257,512,1000,2048} and tie
densities from continuous to heavily tied). Existing
`kendalltau_matches_scipy_reference_values` and the 10 kendall tests pass.

## Benchmark (rch ts2, same-worker A/B: forced-naive vs Knight)
| case                | naive (O(n²)) | Knight (O(n log n)) | Score |
|---------------------|---------------|---------------------|-------|
| kendalltau / 2048   | 4.012 ms      | 204.5 µs            | 19.6x |
| kendalltau / 4096   | 15.772 ms     | 495.1 µs            | 31.9x |

Quadratic vs log-linear scaling is visible: naive grows ~4x per doubling,
Knight ~2.4x. clippy + fmt clean.

## Follow-up
`mannkendall` (S statistic) and `kendalltau_alternative`'s exact-p-value path
share the inversion-count primitive — `kendall_strict_inversions` /
`kendall_tie_pairs` are reusable for the same O(n²)→O(n log n) treatment.
