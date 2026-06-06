# perf(fsci-spatial): parallelize KDTree dual-tree queries (byte-identical)

## Lever
count_neighbors / query_ball_tree / query_pairs each loop over independent per-anchor tree
descents. Parallelize across anchors (thread::scope, gated by cdist_thread_count):
- count_neighbors: exact usize sum -> order-independent -> sum per-thread partials.
- query_ball_tree: each result stored at its own node.index -> collect (index, result) in
  parallel, scatter into the output Vec.
- query_pairs: collect per-thread pair lists, concat, then the existing sort_unstable makes
  the set order-independent.

## Byte-identity
perf_kdtree_queries digests (count value + ball FNV + pairs FNV + len) OLD(serial)==NEW.

## Bench (perf_kdtree_queries, release-perf, 64 cores, d=3, r=0.12)
| n     | count_neighbors | query_ball_tree | query_pairs |
|-------|----------------:|----------------:|------------:|
| 5000  |       1.0x      |       1.7x      |     1.5x    |
| 15000 |       5.2x      |       5.8x      |     3.2x    |
| 40000 |       8.2x      |       7.0x      |     3.0x    |
Grows ~n (small trees are overhead-bound). 182 fsci-spatial tests pass; clippy clean.
