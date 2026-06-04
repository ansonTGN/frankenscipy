# Primitive Selection: silhouette_samples Symmetric Pairwise Accumulation

Bead: `frankenscipy-8l8r1.30`

## Profile Evidence

- RCH-built `perf_cluster` baseline:
  - `silhouette-samples 1200 16 24 4`: 158.6 ms +/- 4.4 ms
  - `silhouette-samples 1600 32 32 2`: 239.3 ms +/- 11.1 ms
- Kernel profilers were blocked by `perf_event_paranoid=4`; see
  `perf_record.stderr` and `samply.stderr`.
- Exact dynamic operation profile for the first candidate:
  - `n=1600, k=32`: old label scans 81,920,000; bucket pass 2,560,000.
  - `n=1200, k=24`: old label scans 34,560,000; bucket pass 1,440,000.
  - Distance evaluations are unchanged, preserving the numeric kernel surface.
- Bucket-only result: rejected. `after_high_k_hyperfine.txt` showed only
  1.03x at `n=1000,d=4,k=200` and a regression at `n=1400,d=4,k=350`.
  The remaining bottleneck is Euclidean distance work, not label scans.

## Harvested Primitive

Alien-graveyard fit: vectorized/data-local execution discipline and the "constants
kill you" gate. The final lever is symmetric pairwise accumulation: compute each
Euclidean distance once for pair `(i,j)` and add it to both anchors' cluster
buckets.

Alien-artifact proof shape: finite partition aggregation. For fixed anchor `i`,
the old algorithm computes one ordered sum for the own cluster and one ordered sum
for each other cluster. The symmetric loop visits anchor `i`'s neighbors in the
same effective order: prior indices were added during earlier outer loops in
ascending order, and later indices are added during `i`'s own loop in ascending
order. For every cluster bucket, the relative order of additions is unchanged.

## Opportunity Score

| Lever | Impact | Confidence | Effort | Score |
| --- | --- | --- | --- | --- |
| Symmetric pairwise cluster buckets | 5 | 4 | 2 | 10.0 |

Proceed because Score >= 2.0.

## Isomorphism Obligations

- Output ordering: preserve `samples[i]` order for `i = 0..n`.
- Tie-breaking: no tie-breaking surface changes; `b(i)` still chooses the first
  strictly smaller mean by scanning cluster ids in ascending order.
- Floating point: each pair distance is computed once instead of twice across the
  two anchors, but the formula and each anchor's per-cluster addition order are
  unchanged.
- RNG: none.
- Golden: `perf_cluster golden` sha256 must match before/after.

## Fallback Trigger

Revert the lever if golden sha256 changes, crate tests fail, or RCH hyperfine
shows no real speedup against the current bucket-pass baseline.
