# CSR Add Prefix-Fill Trial

Bead: `frankenscipy-0tuq3`
Crate: `fsci-sparse`
File: `crates/fsci-sparse/src/ops.rs`

## Target

Post-SpMM RCH sparse reprofile on `ts2` ranked
`sparse_arithmetic/10000x10000_d0_add/10000` first at `1.6465 ms`
median. The trial replaced chunk-local append/concatenate CSR add with exact
symbolic row counts, prefix-summed `indptr`, exact allocation, and disjoint
numeric fill.

## Isomorphism

The trial kept row traversal, column emission order, explicit zero elision,
floating-point expression order, metadata semantics, and RNG absence unchanged.
The add-CSR golden SHA-256 remained:

`a3a9d49d373b8d28f1aca881ab2b2322229b8befc0cb91c1e85a0820bc318da8`

## Result

The prefix-fill lever regressed on same-worker `ts2`:

- before profile row: `1.6465 ms` median
- prefix-fill after: `2.1683 ms` median
- score: `0.0`, rejected

The prefix-fill code landed through shared-tree churn in `39fe9cbb`, so this
pass restored only the CSR-add parallel path back to the prior chunk-buffer
implementation. Restore benchmark on same-worker `ts2`:

- regressed prefix-fill: `2.1683 ms` median
- restored chunk-buffer: `1.5438 ms` median
- restore score: `12.0 = impact 3 * confidence 4 / effort 1`

## Validation

- `cargo fmt -p fsci-sparse --check`: passed
- RCH `cargo check -p fsci-sparse --all-targets --locked`: passed on `ts2`
- RCH `cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`: passed on `ts2`
- RCH `cargo test -p fsci-sparse --locked -- --nocapture`: passed on `ts2`
- `ubs crates/fsci-sparse/src/ops.rs`: exit 0

Next pass should pivot to direct format conversion or CSR construction, not
another CSR-add counting/prefix variant.
