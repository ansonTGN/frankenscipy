# frankenscipy-va60h linkage flat-arena gauntlet

Agent: cod-a / MistyBirch

Decision: keep the flat row-major linkage arena as an internal win, but record
the measured SciPy head-to-head as a loss. A manual production revert probe made
the nested route slower on both measured rows, so the flat route remains in the
release candidate.

## Workload

- Rust benchmark: `cargo bench -p fsci-cluster --bench cluster_bench -- va60h_gauntlet_linkage --noplot`
- Dataset: deterministic `blobs(n=800, d=4)`
- SciPy oracle: Python 3.13.7, NumPy 2.4.3, SciPy 1.17.1,
  `scipy.cluster.hierarchy.linkage`

## Results

| Workload / route | Mean | Ratio | Verdict |
| --- | ---: | ---: | --- |
| Rust current flat `linkage(Average)` | 6.1713 ms | 1.385x slower than SciPy | SciPy loss, internal keep |
| Rust legacy nested helper `linkage(Average)` | 6.9616 ms | current flat is 1.128x faster | internal win |
| SciPy `linkage(method="average")` | 4.4550 ms | 1.00x oracle | reference |
| Rust current flat `linkage(Ward)` | 7.5250 ms | 1.497x slower than SciPy | SciPy loss, internal neutral/win |
| Rust legacy nested helper `linkage(Ward)` | 7.6707 ms | current flat is 1.019x faster | internal neutral/win |
| SciPy `linkage(method="ward")` | 5.0256 ms | 1.00x oracle | reference |

Post-revert probe:

| Workload / route | Mean | Ratio | Verdict |
| --- | ---: | ---: | --- |
| Reverted production nested `linkage(Average)` | 9.0669 ms | flat route is 1.290x faster | undo revert |
| Flat route reconstruction `linkage(Average)` | 7.0279 ms | 1.00x comparison route | keep flat |
| Reverted production nested `linkage(Ward)` | 9.1589 ms | flat route is 1.251x faster | undo revert |
| Flat route reconstruction `linkage(Ward)` | 7.3221 ms | 1.00x comparison route | keep flat |

## Transcripts

- `bench_va60h_linkage_criterion.txt`: initial Criterion run, current flat vs
  legacy nested helper vs SciPy.
- `bench_va60h_linkage_post_revert_criterion.txt`: manual production revert
  probe; revert was undone after this run.
- `bench_va60h_linkage_scipy_stdin_smoke.txt`: smoke run for the committed
  stdin-fed SciPy oracle invocation; no change in SciPy row performance was
  detected.
- `cargo_check_fsci_cluster_benches.txt`: compile gate, passed with existing
  `perf_kmeans.rs` warning.
- `cargo_test_fsci_cluster_linkage_rch.txt`: filtered linkage tests via rch,
  passed.
- `cargo_test_diff_cluster_linkage_from_distances.txt`: SciPy-backed conformance
  test, passed.
- `cargo_fmt_fsci_cluster_check.txt`: blocked on existing `perf_isomap.rs`
  formatting drift outside this gauntlet.
- `cargo_clippy_fsci_cluster_benches.txt`: blocked on existing `fsci-linalg`
  dependency lints before this benchmark file was linted.
