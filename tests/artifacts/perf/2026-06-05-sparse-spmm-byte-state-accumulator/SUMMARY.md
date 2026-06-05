# Sparse SpMM Byte-State Accumulator

Bead: `frankenscipy-8l8r1.35`

Verdict: kept. The row-local Gustavson SpMM symbolic and numeric kernels now use a byte-state marker for the dense seen set instead of `Vec<bool>`.

## Target

- Crate: `fsci-sparse`
- Benchmark: `sparse_spmm/2000x2000_d1/2000`
- Profile evidence: post-`frankenscipy-8l8r1.34` RCH reprofile ranked this benchmark first at `12.874 ms` median on `ts2`.
- Lever: change only the dense seen marker representation in `spmm_row_chunk` and `spmm_row_counts_chunk`.

## Benchmark

- Fresh baseline: RCH `ts1`, `10.028 ms` median, CI `[9.6960, 10.359]`, `baseline_rch.txt`
- Same-worker after: RCH `ts1`, `9.6090 ms` median, CI `[9.5093, 9.7383]`, `after_confirm_rch.txt`
- Same-worker delta: `4.2%` faster by median, `1.04x`.
- Confirmation: RCH `ts2` prior reprofile `12.874 ms` median -> byte-state after `12.146 ms` median, `5.7%` faster, `after_rch.txt`.
- Score: `8.0 = impact 2 * confidence 4 / effort 1`, above keep threshold.

## Isomorphism Proof

- Ordering: row partitions, row traversal, A nonzero traversal, B row traversal, and reverse first-seen column emission are unchanged.
- Tie-breaking: first-seen detection still occurs exactly once per row/column before emission; only the marker type changes from `bool` to byte state.
- Floating-point: multiply/add order and zero-elision predicate are unchanged; `acc[j]` receives the same arithmetic sequence.
- RNG: absent from production SpMM; benchmark input construction remains outside the changed kernel.
- Metadata: sorted/deduplicated handling is unchanged.

## Golden Proof

- Previous strict payload: `tests/artifacts/perf/2026-06-05-sparse-spmm-work-balanced-rows/golden_after_payload.strict.txt`
- Current strict payload: `golden_after_payload.strict.txt`
- `cmp -s` exit: `0`
- SHA256: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`

## Validation

- `cargo fmt -p fsci-sparse --check`
- RCH `cargo check -p fsci-sparse --all-targets --locked`
- RCH `cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`
- RCH `cargo test -p fsci-sparse --locked -- --nocapture`: `309 passed`, `1 ignored`, plus `56` metamorphic tests
- `ubs crates/fsci-sparse/src/linalg.rs`: exit `0`, zero critical issues; remaining findings are existing broad warning inventory.

## Next

Reprofile `fsci-sparse`. If SpMM remains dominant, the next target should be a deeper structural primitive such as a sparse accumulator workspace with row-local dense state reuse across symbolic and numeric phases, or a CSC/column-panel SpGEMM path; avoid replaying scheduling-only variants.
