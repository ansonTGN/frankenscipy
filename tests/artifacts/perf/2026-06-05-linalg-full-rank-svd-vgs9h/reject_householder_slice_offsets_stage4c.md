# Rejected Stage 4c Householder Slice-Offset Kernel Trial

Bead: `frankenscipy-z65tz`

## Baseline

- Command: RCH `cargo test -p fsci-linalg --release --lib bidiag_large_reduction_perf_probe --locked -- --ignored --nocapture`
- Worker: `vmi1227854`
- Shape: `1024x512`
- Time: `204.413911 ms`
- Digest: `0x90cdd3f8f71ed2c1`

Public behavior guard before the trial passed on RCH `vmi1149989` via
`public_svd_lstsq_pinv_golden_payload`.

## Trial

The trial replaced `DMatrix[(row, col)]` indexing in the existing left and right
Householder application kernels with explicit column-major `as_mut_slice()`
offsets. It preserved the same dot-product/update loop order and did not change
the reduction algorithm.

## Proof

- Command: RCH `cargo test -p fsci-linalg --release --lib bidiag_right_workspace_matches_rowwise_reference_bits --locked -- --nocapture`
- Worker: `ts1`
- Result: passed; the slice-offset reducer stayed bit-identical to the rowwise
  reference.

## Rebench

- Command: RCH `cargo test -p fsci-linalg --release --lib bidiag_large_reduction_perf_probe --locked -- --ignored --nocapture`
- Worker: `ts1`
- Shape: `1024x512`
- Time: `353.428465 ms`
- Digest: `0x90cdd3f8f71ed2c1`

## Decision

Rejected with score `0.0`.

The source was restored to the Stage 4a workspace reducer. This was a
profile-aligned but too-shallow micro-kernel cleanup; it preserved bits but did
not improve runtime. The next attempt must be the deeper packed/tiled
block-reflector far update for `V*Y^T + X*U^T`, with contiguous small-K panels
and either exact summation-order proof or an intentional private golden
migration before public wiring.
