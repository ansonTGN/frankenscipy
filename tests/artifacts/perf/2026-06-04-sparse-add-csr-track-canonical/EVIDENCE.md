# CSR Add Canonical Metadata Tracking

Bead: `frankenscipy-vx0jw`

## Target

Profile-backed hotspot:

- `sparse_arithmetic/10000x10000_d0_add/10000`
- Baseline artifact: `baseline_add_csr_rch.txt`
- Baseline worker: `ts2`
- Baseline median: `2.3997 ms`

## Lever

`add_csr` and `sub_csr` now skip the separate canonical-input verification scan when both CSR inputs already carry canonical metadata. The direct row merge constructs the output with unchecked components and tracks the output `CanonicalMeta` while emitting nonzero entries.

This is a sparse-kernel metadata/payload split: use the canonical metadata as the selector and update output metadata in the merge stream instead of paying a second validation pass.

## Behavior Proof

- Ordering: row traversal and per-row merge comparisons are unchanged.
- Tie-breaking: equal-column handling remains the same branch, with the same zero-elision rule.
- Floating point: value expressions and per-column addition order are unchanged; metadata tracking only inspects column order after `value != 0.0`.
- RNG: benchmark and golden harness seeds are unchanged.
- Mislabelled input coverage: `add_csr_mislabelled_canonical_input_keeps_validating_path` still covers the noncanonical output metadata path.
- Golden SHA: `a3a9d49d373b8d28f1aca881ab2b2322229b8befc0cb91c1e85a0820bc318da8`.

Golden verification:

```text
sha256sum -c tests/artifacts/perf/2026-06-04-sparse-add-csr-track-canonical/golden_after_add_csr.sha256
tests/artifacts/perf/2026-06-04-sparse-add-csr-track-canonical/golden_after_add_csr.txt: OK
```

## Performance

Primary same-worker comparison:

| Run | Worker | Median | Interval |
| --- | --- | ---: | --- |
| Baseline | `ts2` | `2.3997 ms` | `[2.3926 ms, 2.4128 ms]` |
| After confirmation | `ts2` | `2.1703 ms` | `[2.1352 ms, 2.2414 ms]` |

Delta: `1.106x` faster, about `9.6%` lower median.

The `after_add_csr_rch.txt` VPS run on `vmi1264463` is retained as a noisy cross-worker artifact, not the keep gate: it reported `3.5456 ms` median with severe high outliers and is not comparable to the `ts2` baseline.

Score: `3.0 = Impact 2 * Confidence 3 / Effort 2`. This is above the `2.0` keep threshold.

## Validation

Passed:

- `RCH_WORKER=ts2 rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- sparse_arithmetic/10000x10000_d0_add/10000 --warm-up-time 1 --measurement-time 2 --sample-size 10 --noplot`
- `RCH_WORKER=ts2 rch exec -- cargo run -p fsci-sparse --bin perf_sparse --release --locked -- add-csr-golden`
- `sha256sum -c tests/artifacts/perf/2026-06-04-sparse-add-csr-track-canonical/golden_after_add_csr.sha256`
- `cargo fmt -p fsci-sparse --check`
- `ubs crates/fsci-sparse/src/ops.rs` (exit `0`; existing noncritical warnings only)
- `RCH_WORKER=ts2 rch exec -- cargo check -p fsci-sparse --all-targets --locked`
- `RCH_WORKER=ts2 rch exec -- cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`
- `RCH_WORKER=ts2 rch exec -- cargo test -p fsci-sparse --locked add_csr -- --nocapture`
