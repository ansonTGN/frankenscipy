# SpMM Symbolic Row-Count Prepass

Bead: `frankenscipy-mnqxr`
Crate: `fsci-sparse`
Target: `sparse_spmm/2000x2000_d1/2000`

## Profile Context

After rejected row-band, capacity, epoch, stack-workspace, and morsel trials,
large CSR SpMM remained the dominant sparse hotspot. Focused RCH baseline on
current source:

- `ts2`: `13.224 ms` median, CI `[13.000 ms, 13.519 ms]`

## Lever

The parallel CSR SpMM path now runs a GraphBLAS-style symbolic row-count pass
before the numeric pass. The symbolic pass uses the same row traversal, B-row
encounter order, first-seen list, reverse emission, and zero-elision predicate
to compute exact per-row output counts. The numeric pass then reuses the
existing row kernel with exact chunk capacities and builds `indptr` from the
symbolic counts.

## Isomorphism

- Ordering preserved: yes; contiguous row partitions and chunk concatenation are
  unchanged.
- Tie-breaking unchanged: yes; first-seen columns are discovered at the same
  points and emitted in the same reverse order.
- Floating-point unchanged: yes; emitted values are produced by the existing
  numeric row kernel with the same product/addition order.
- RNG unchanged: yes; SpMM uses no runtime RNG.
- Metadata unchanged: yes; `sorted_indices` still comes from the numeric row
  kernel and `deduplicated` remains true.
- Strict golden SHA-256:
  `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Strict golden diff: empty.

## Performance

Same-worker RCH Criterion on `ts2`:

- baseline: `13.224 ms` median, CI `[13.000 ms, 13.519 ms]`
- after: `12.316 ms` median, CI `[12.164 ms, 12.488 ms]`
- confirmation: `12.807 ms` median, CI `[12.715 ms, 12.889 ms]`

Primary after ratio: `1.074x`, about `6.9%` lower median. Confirmation remains
below the baseline interval.

Score: `4.0 = impact 2 * confidence 4 / effort 2`.

## Validation

- `cargo fmt -p fsci-sparse --check`: passed
- RCH `cargo check -p fsci-sparse --all-targets --locked`: passed
- RCH `cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`: passed
- RCH `cargo test -p fsci-sparse --locked -- --nocapture`: passed
- `ubs crates/fsci-sparse/src/linalg.rs`: exit 0

Next step: reprofile `fsci-sparse`; if SpMM remains dominant, attack a different
GraphBLAS kernel shape such as CSC/column-panel traversal with row-local order
replay.
