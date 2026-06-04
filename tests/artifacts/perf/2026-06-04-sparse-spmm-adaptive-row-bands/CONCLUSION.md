# Sparse SpMM Adaptive Row-Bands Trial

Bead: `frankenscipy-cbotb`

Target: `sparse_spmm/2000x2000_d1/2000`, selected from the fresh RCH sparse reprofile on `ts2` after CSR-add restore:

- `sparse_spmm/2000x2000_d1/2000`: `11.701 ms` median
- `sparse_arithmetic/10000x10000_d0_add/10000`: `1.5765 ms` median
- `sparse_spmm/1000x1000_d1/1000`: `1.0261 ms` median
- `sparse_format_conversion/10000x10000_d0_csr_to_csc/10000`: `825.31 us` median

## One Lever

Trialed exactly one source change in `spmm_chunk_count`: widen large SpMM row bands from a `16` thread cap and about `128` rows per worker to a `32` thread cap and about `64` rows per worker.

## Isomorphism Proof

Changing only the worker count preserves:

- contiguous row ranges
- per-row A and B encounter order
- reverse first-seen column emission inside each row
- chunk concatenation in ascending row-range order
- floating-point operation order within each output row
- explicit zero elision
- `sorted_indices` AND semantics
- deduplicated metadata
- RNG absence

Golden payload SHA-256 before and after:

`0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`

`golden_payload.diff` is empty.

## Benchmarks

All benchmark commands were crate-scoped and run through RCH on `ts2`.

- Baseline: `11.991 ms` median, CI `[11.861 ms, 12.127 ms]`
- After: `11.565 ms` median, CI `[11.460 ms, 11.669 ms]`
- After confirm: `11.761 ms` median, CI `[11.296 ms, 12.307 ms]`

The first after run was a small `1.037x` win, but the confirmation interval overlapped the baseline and included the baseline median.

## Decision

Rejected. Score `0.0`.

Source was restored; `git diff --quiet -- crates/fsci-sparse/src/linalg.rs` exited `0`, and `cargo fmt -p fsci-sparse --check` passed after restore.

Next SpMM attack should replace this thread-count micro-lever with a deeper structural primitive, such as row-work prefix partitioning plus work-proportional output capacity, or a CSC/column-panel SpGEMM variant that changes the memory traversal model while preserving row-local accumulation order.
