# perf(fsci-stats): weightedtau O(n^2) -> O(n log n) (Fenwick weighted Kendall tau)

## Lever
weightedtau averages two one-side weighted Kendall taus; each was an O(n^2) all-pairs sum
`sum_{i<j} (wi+wj)*sign((xi-xj)(yi-yj)) / sum_{i<j}(wi+wj)`, wi=1/(rank_i+1). Replace with
O(n log n):
- Denominator: each element is in n-1 pairs -> D = (n-1)*sum(wi).
- Numerator: sort by x (group x-ties, which contribute 0); process in x order, query two
  Fenwick/BIT trees over the y-rank (one weights, one counts) for the weighted
  concordant-minus-discordant contribution from already-inserted (smaller-x) points.
  y-ties contribute 0 (neither < nor > side).

## Parity (tolerance, not bit-exact: the weighted sums accumulate in a different order)
Unit test weightedtau_one_side_matches_reference: O(n log n) vs the retained O(n^2)
reference, max diff < 1e-9, across distinct values + x-ties + y-ties, n up to 200.
perf_weightedtau statistic OLD(O(n^2)) vs NEW(O(n log n)) agree to ~13 digits (~2e-14 rel):
  n=2000  0.8364991711748193 vs 0.8364991711748417
  n=5000  0.8579053064730573 vs 0.8579053064731181
  n=10000 0.8587580783556951 vs 0.8587580783557760

## Bench (perf_weightedtau, release-perf, min of 3)
| n     | OLD O(n^2) | NEW O(n log n) | speedup |
|-------|-----------:|---------------:|--------:|
| 2000  |   9.83 ms  |    0.377 ms    |  26.1x  |
| 5000  |  64.21 ms  |    1.334 ms    |  48.1x  |
| 10000 | >100 ms    |    2.849 ms    |  >35x   |
Grows ~n/log n. 8 weightedtau tests pass; clippy clean.
