# fsci-cluster silhouette_samples baseline

Bead: `frankenscipy-8l8r1.30`

## Target

`silhouette_samples` currently computes the same anchor-to-all-samples distance
families through repeated cluster scans:

- one full scan for same-cluster `a(i)`,
- then one full label scan per other cluster for `b(i)`.

The measured high-`k` target is the repeated label scan, not the distance kernel.

## RCH Baseline

Command family:

```bash
rch exec -- bash -lc 'cargo build -p fsci-cluster --profile release-perf --bin perf_cluster; hyperfine ...'
```

Focused high-`k` shape:

| mode | shape | mean |
| --- | --- | ---: |
| `silhouette-samples-base` | `n=2000 d=2 k=256 repeats=1` | `614.2 ms +/- 51.2 ms` |
| `silhouette-samples` | `n=2000 d=2 k=256 repeats=1` | `656.1 ms +/- 25.6 ms` |

The in-binary baseline mirror and the current library path have identical
checksums on the smoke shape:

```text
n=256 d=2 k=64 checksum = 2.366775423494e2
```

The library path is the keep-gate baseline: `656.1 ms`.

## Golden Before

```text
perf_cluster golden sha256:
37b0fd42300f4133fed4af0625f9657a66e5ff47038e005d816a6fb98d6e955d

sorted silhouette_samples test-output sha256:
a8b744b19577e3b60b84a3e47b2507d8613e5668c9ad91af5c6886c55b72f279
```

## Isomorphism Contract

- Ordering preserved: output index `i` remains input sample order.
- Tie-breaking unchanged: `b(i)` keeps first lower mean by cluster-order scan;
  equal means do not replace the incumbent.
- Floating-point preserved: for each anchor, distances are still computed in
  ascending `j` order and accumulated into each cluster in the same within-cluster
  `j` order. The final `(b - a) / max(a, b)` formula stays unchanged.
- RNG: none.
- Golden outputs: `perf_cluster golden` and sorted `silhouette_samples` test
  output must have identical SHA-256 before and after.
