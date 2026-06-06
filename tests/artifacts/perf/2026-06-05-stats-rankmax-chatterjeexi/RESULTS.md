# perf(fsci-stats): rank_max O(n^2)->O(n log n) (chatterjeexi)

## Lever
rank_max[i] = #{j: data[j] <= data[i]} (the "max" rank method, used by chatterjeexi) was
an O(n^2) double loop. `<=` is false whenever either operand is NaN, so NaN entries never
count and a NaN query gets 0. Sort the non-NaN values once; for a non-NaN query v the count
is the number of sorted values <= v — a monotone predicate over the ascending array, so a
single partition_point gives it. O(n log n). Makes chatterjeexi O(n log n) overall.

## Bit-identity
Counts are exact integers, so the result is bit-identical to the double loop. Unit test
rank_max_matches_quadratic_reference checks dup/-0.0/0.0/NaN/+-inf cases bit-for-bit.
perf_chatterjeexi statistic+pvalue bits OLD(O(n^2))==NEW(O(n log n)):
  n=2000  stat=3fe870f8a69bc1c8 pvalue=0
  n=5000  stat=3fe5f13a38ac1518 pvalue=0
  n=10000 stat=3fe851a207dfe381 pvalue=0

## Bench (perf_chatterjeexi, release-perf, min of 3)
| n     | OLD O(n^2) | NEW O(n log n) | speedup |
|-------|-----------:|---------------:|--------:|
| 2000  |   6.285 ms |     0.137 ms   |  45.8x  |
| 5000  |  36.482 ms |     0.987 ms   |  37.0x  |
| 10000 | 200.890 ms |     1.089 ms   | 184.5x  |
Grows ~n. 4 chatterjeexi tests + rank_max iso test pass; clippy clean.
