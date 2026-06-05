# fsci-spatial pdist: single-threaded condensed pairwise -> multithreaded row split

## Target (bead frankenscipy-n2mqb, pdist part; follows cdist 7decfe77)

pdist computed the condensed n*(n-1)/2 pairwise-distance vector in a single-threaded
i<j double loop (used by hierarchical clustering / linkage). Each pair is an
independent pure `metric_distance`.

## Lever (one)

Row i writes a contiguous run of (n-1-i) condensed entries at offset
i·(n-1) − i·(i−1)/2. Split the rows across threads at PAIR-BALANCED boundaries
(`pdist_row_bounds`: equal cumulative pairs, since early rows weigh more), `split_at_mut`
each thread's disjoint contiguous slice, fill it in the same i<j order. Gated by the
shared `cdist_thread_count` (work = n·n·dim).

## Isomorphism / proof (BYTE-IDENTICAL)

Each entry = metric_distance(x[i], x[j], metric) in the identical i<j order; the
balanced boundaries tile the output exactly. New test pdist_parallel_is_bit_identical
(n=900 > gate, Euclidean/Cityblock/Chebyshev, f64::to_bits equal); perf_cdist reports
bit_identical=true at scale. fsci-spatial 182 passed / 0 failed; clippy + fmt clean.

## Same-process A/B (perf_cdist bin, 64-core worker)

| n | dim | seq | par | speedup |
| ---: | ---: | ---: | ---: | ---: |
| 3000 | 3 | 33.47 ms | 8.77 ms | 3.81x |
| 4000 | 16 | 107.30 ms | 14.69 ms | 7.31x |

Speedup grows with dim. Byte-identical, Score >> 2.0. Zero-threaded-crate vein
continues (cdist + pdist done in fsci-spatial; ndimage median/rank done).
