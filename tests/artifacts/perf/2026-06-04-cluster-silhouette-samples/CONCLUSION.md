# fsci-cluster silhouette_samples optimization

Bead: `frankenscipy-8l8r1.30`

## Profile target

`silhouette_samples` spent its work in repeated all-points cluster scans. Kernel
profilers were unavailable on this host (`perf_event_paranoid=4`), so the target
was established with a deterministic operation profile and RCH-built hyperfine
runs. The counted work for `n=1600, k=32` was:

- old cluster scans: 81,920,000 label checks
- bucket-pass scans: 2,560,000 label checks
- pair distances: 2,558,400

The first bucket-pass lever was rejected: it preserved output but only measured
about 1.03x on high-k cases and regressed one shape. The kept lever is symmetric
pairwise accumulation: compute each pair distance once, add it to both anchors'
cluster buckets, and derive `a(i)` and `b(i)` from those buckets.

## Kept result

Golden output SHA-256 stayed unchanged:

`37b0fd42300f4133fed4af0625f9657a66e5ff47038e005d816a6fb98d6e955d`

RCH-built hyperfine evidence:

| Case | Baseline | New | Speedup |
| --- | ---: | ---: | ---: |
| `2600x64 k=64 r=1` | 248.3 ms | 132.9 ms | 1.87x |
| `2600x64 k=64 r=1` repeat | 258.6 ms | 126.5 ms | 2.04x |
| `2600x32 k=64 r=1` | 135.2 ms | 67.8 ms | 1.99x |
| `1600x16 k=64 r=2` | 53.3 ms | 29.3 ms | 1.82x |

Score: Impact 5 x Confidence 4 / Effort 2 = 10.0.

## Isomorphism proof

- Labels are still densified by `validate_cluster_metric_data`.
- For each sample, distances are accumulated in increasing peer index order
  within each target cluster, matching the bucket-pass reference order.
- Tie-breaking for the minimum other-cluster mean is unchanged because cluster
  means are scanned from `0..k`.
- No RNG is used in `silhouette_samples`.
- Floating-point outputs are byte-identical for the golden harness.
- The code falls back to the bucket-pass path for `n < 256`, checked overflow,
  or `n * k > 8,000,000`, avoiding memory-heavy cases.

## Validation

- `cargo fmt -p fsci-cluster --check`
- `AGENT_NAME=CodexOpt rch exec -- cargo check -p fsci-cluster --all-targets`
- `AGENT_NAME=CodexOpt rch exec -- cargo clippy -p fsci-cluster --all-targets -- -D warnings`
- `AGENT_NAME=CodexOpt rch exec -- cargo test -p fsci-cluster --all-targets`
- `ubs crates/fsci-cluster/src/lib.rs crates/fsci-cluster/src/bin/perf_cluster.rs`
