# rand_index: O(n²) pairwise count → O(n + k²) contingency table

Bench: `rand_index` (fsci-stats), rch ts2. Function: `rand_index`.

## Lever
The Rand index counted, over all O(n²) point pairs, how many are grouped together
(a) or apart (b) by both labelings. Those counts come directly from the
true-vs-pred contingency table in O(n + k²) (k = #clusters): with C(m)=m(m-1)/2,
`a = sum over table cells of C(cell)`, and by inclusion-exclusion
`b = total - C(rows) - C(cols) + a`. Built with a HashMap pass over the points;
gated at n >= 64 (tiny inputs keep the pair loop).

## Isomorphism
a, b are exact integer pair counts identical to the double loop's, and the final
`(a+b) as f64 / total_pairs as f64` is the same float op, so the result is
bit-for-bit identical. Proven by `rand_index_contingency_matches_pair_loop`:
`.to_bits()` equality vs the O(n²) pair loop across n in {2,63,64,65,500,2000},
cluster counts k in {1,2,5,30}, and perfect-agreement cases. The 6 existing
scipy-reference rand_index tests pass; clippy + fmt clean.

## Benchmark (k=10 clusters, rch ts2, pair loop vs contingency)
| case             | pairwise O(n²) | contingency | Score |
|------------------|----------------|-------------|-------|
| rand_index/2000  | 12.285 ms      | 154.0 µs    | 79.8x |
| rand_index/8000  | 196.91 ms      | 604.0 µs    | 326x  |

Pairwise scales O(n²) (×16 per 2× n); contingency O(n) (×4), so the ratio grows
without bound with the point count. A different primitive (contingency-table
counting) rather than another sort/selection lever.
