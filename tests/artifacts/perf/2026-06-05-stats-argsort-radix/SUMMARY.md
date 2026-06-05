# argsort: index-comparator sort → keyed (decorate-sort-undecorate)

Bench: `ordering_and_bins/argsort` (fsci-stats), rch ts2. Function: `argsort`.

## Lever
`argsort` sorted an index vector with a comparator that re-gathered
`data[a]`/`data[b]` — two random loads per comparison (cache-hostile). For
NaN-free inputs it now materializes `(key, index)` pairs and `sort_unstable`s
them: the key is a monotonic u64 transform of the float bits
(`sign ? !bits : bits | 1<<63`, with -0.0 normalized to +0.0), so the contiguous
pair sort touches no scattered loads.

An LSD radix sort on the same keys was tried first but LOST at large n (its
16-byte-pair scatter blows the cache: 65536 was 3.42 ms vs the comparison sort's
2.65 ms). The decorate-sort-undecorate keyed sort wins because it keeps the
comparison sort's cache behavior while removing the indirection.

## Isomorphism
Sorting `(key, index)` lexicographically is bit-identical to the stable
partial_cmp index sort: the key is monotonic in value, equal values share a key
and tie-break on index (ascending = original order, exactly what a stable sort
gives), and -0.0/+0.0 share a key (compare equal, as partial_cmp does). Proven by
`argsort_keyed_matches_stable_partial_cmp`: identical index permutation vs the
stable partial_cmp reference across n in {256,257,1000,5000}, tie densities
continuous→heavy, negatives, ±inf, and signed zeros. Gated NaN-free (NaN keeps
the partial_cmp path, which treats NaN as Equal); clippy + fmt clean.

## Benchmark (rch ts2, comparator-index sort vs keyed)
| case          | index-comparator | keyed     | Score |
|---------------|------------------|-----------|-------|
| argsort/4096  | 176.07 µs        | 86.5 µs   | 2.04x |
| argsort/65536 | 2.654 ms         | 2.382 ms  | 1.11x |

The win is largest at moderate n where the per-comparison gather dominates; at
very large n the larger (key,index) element size offsets some of the gain (still
positive). argsort is a common primitive (percentiles, rank stats, ordering).
