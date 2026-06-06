# perf(fsci-cluster): parallelize silhouette per-anchor distance pass (byte-identical)

## Lever
silhouette_samples used an n*k upper-triangle matrix accumulation; silhouette_score had a
separate serial per-anchor loop. Each anchor's coefficient depends only on its distances to
all other points bucketed by cluster, so anchors are independent. Unified both on ONE
parallel per-anchor pass (thread::scope + chunked i-ranges, per-thread reused length-k
scratch buffers, no per-anchor alloc). silhouette_score sums the per-anchor values in index
order (matches its original total += s).

## Byte-identity
The per-anchor bucket sum accumulates in increasing j order, identical to the upper-triangle
matrix fill (dist(i,j)==dist(j,i) bit-for-bit; each matrix cluster_sum[i][c] also fills in
increasing source order). perf_silhouette_samples FNV digests (samples + score) OLD==NEW:
  n=1500 d=8  k=5   samples=642e89f110d8615c score=3fe970ef106009ee
  n=3000 d=12 k=8   samples=dae95360792701bd score=3fe9526b55d7bbcc
  n=5000 d=16 k=10  samples=c758c797cd776ad0 score=3fe95dfc44f9046a

## Bench (perf_silhouette_samples, release-perf, min of 3, 64 cores)
| n    | d  | k  | OLD (matrix) | NEW (parallel) | speedup |
|------|----|----|-------------:|---------------:|--------:|
| 1500 | 8  | 5  |    5.31 ms   |    2.87 ms     |  1.85x  |
| 3000 | 12 | 8  |   32.48 ms   |    7.73 ms     |  4.20x  |
| 5000 | 16 | 10 |  121.38 ms   |   19.92 ms     |  6.10x  |
Grows ~n. 7 silhouette tests pass; clippy clean.
