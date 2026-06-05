# Sparse SpMM Work-Balanced Row Partitions

Bead: `frankenscipy-8l8r1.32`

Verdict: kept. The parallel CSR SpMM path now uses contiguous row ranges chosen by estimated row multiply work instead of equal row counts.

## Baseline

- Command: `rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- sparse_spmm/2000x2000_d1/2000 --warm-up-time 1 --measurement-time 5 --sample-size 30 --noplot`
- Worker: `ts1`
- Time: `33.813 ms` median, CI `[30.322, 37.725]`
- Log: `baseline_rch.txt`

## After

- Same-worker worker: `ts1`
- Same-worker time: `10.592 ms` median, CI `[10.425, 10.737]`
- Same-worker ratio: `3.19x`
- Confirmation worker: `ts2`
- Confirmation time: `12.689 ms` median, CI `[12.380, 12.935]`
- Logs: `after_rch.txt`, `after_confirm_rch.txt`

## Behavior Proof

- Syntax-strict golden before SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Syntax-strict golden after SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Syntax-strict before/after `cmp`: `0`
- Preserved contract: contiguous range concatenation keeps row order, and the row-local kernel keeps A row traversal, B row encounter order, reverse first-seen output order, floating-point accumulation order, explicit zero elision, metadata, and RNG absence.

## Validation

- `cargo fmt -p fsci-sparse --check`
- RCH `cargo check -p fsci-sparse --all-targets --locked`
- RCH `cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`
- RCH `cargo test -p fsci-sparse --locked spmm -- --nocapture`
- RCH `cargo test -p fsci-sparse --locked -- --nocapture`
- `ubs crates/fsci-sparse/src/linalg.rs` exited `0` with pre-existing warning noise and no critical issues.

Score: `6.0 = impact 4 * confidence 3 / effort 2`.

Next step: reprofile `fsci-sparse`; the dominant sparse hotspot may have shifted after range balancing.
