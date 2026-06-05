# SpMM Epoch-Stamped Sparse Accumulator

Bead: `frankenscipy-1gtas`
Crate: `fsci-sparse`
File: `crates/fsci-sparse/src/linalg.rs`

## Target

After the row-band and work-proportional capacity trials were rejected,
`sparse_spmm/2000x2000_d1/2000` remained the dominant sparse hotspot. This pass
changes the sparse accumulator state machine rather than worker count or buffer
capacity.

## Lever

The row-local Gustavson accumulator now uses a `u32` epoch stamp per column. For
each output row, first touch sets `seen_epoch[j] = row_epoch` and overwrites
`acc[j]`; later touches in the same row accumulate into the same slot.

This removes the old per-output `seen[j] = false` and `acc[j] = 0.0` clearing
writes. Stale accumulator values are ignored because their epoch does not match
the current row.

## Isomorphism

- Ordering preserved: yes; row ranges, row traversal, and chunk concatenation
  are unchanged.
- Tie-breaking unchanged: yes; the first-seen column list is pushed at exactly
  the same first touch points and emitted in the same reverse order.
- Floating-point unchanged: yes; multiply-add encounter order within a row is
  unchanged.
- RNG unchanged: yes; `spmm` uses no runtime RNG.
- Golden SHA-256:
  `5cf7b4b120948e85cb2067b21c66a02909f97232be9aad895e2bc205a023f809`

## Performance

Current-source RCH baseline:

- `ts1`: `17.147 ms` median

Same-worker keep gate using the immediately prior pre-lever source baseline:

- `ts2` pre-lever: `12.787 ms` median
- `ts2` epoch after: `12.161 ms` median
- ratio: `1.051x`, about `4.9%` lower median

Score: `4.0 = impact 2 * confidence 4 / effort 2`.

## Validation

- `cargo fmt -p fsci-sparse --check`: passed
- RCH `cargo check -p fsci-sparse --all-targets --locked`: passed
- RCH `cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`: passed
- RCH `cargo test -p fsci-sparse --locked -- --nocapture`: passed
- `ubs crates/fsci-sparse/src/linalg.rs`: exit 0

Next pass should reprofile before selecting the following sparse primitive.
