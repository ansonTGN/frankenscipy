# Keep: fused bidiagonal rank-k far update (Stage 4d)

Bead: `frankenscipy-z65tz`

## Target

The Stage 4b scalar DLABRD-style panel trial failed because it still spent the
far trailing update in scalar per-cell correction loops. This keep isolates the
next primitive: a contiguous small-k update kernel for
`A := A - V * Y^T - X * U^T`.

The helper is private and unwired. Public `svd`, `svdvals`, `lstsq`, and `pinv`
remain on the existing `safe_svd` route.

## Baseline

Clean-source Stage 4a private bidiagonal reducer context:

- RCH worker: `vmi1264463`
- Probe: `bidiag_large_reduction_perf_probe`
- Shape: `1024x512`
- Elapsed: `702.259972 ms`
- Digest: `0x90cdd3f8f71ed2c1`

Scalar far-update reference for this isolated kernel:

- RCH worker: `ts1`
- Probe: `bidiag_fused_rank_k_update_perf_probe`
- Shape: `1024x512`
- Panel start: `16,16`
- `k_count`: `16`
- Reference time: `18.463455 ms`
- Reference digest: `0xd60df77cdefac734`

## Lever

Add `apply_bidiag_fused_rank_k_update`, which validates packed row and column
panels and writes each trailing cell once after applying the exact scalar
correction order:

1. `value -= V[row, k] * Y[col, k]`
2. `value -= X[row, k] * U[col, k]`

This preserves per-cell k ordering and floating-point operation ordering against
the scalar reference.

## Proof

- RCH proof command: `cargo test -p fsci-linalg --release --lib bidiag_fused_rank_k_update_matches_scalar_reference_bits --locked -- --nocapture`
- RCH worker: `ts1`
- Result: passed, `1` test, `0` failed
- Equality policy: every output cell compared by `f64::to_bits`
- RNG: none
- Ordering/tie-breaking: public SVD-family routes unchanged
- Golden digest: scalar and fused probe outputs both `0xd60df77cdefac734`

RCH retrieval reintroduced stale rejected blocked-panel helper fragments after
the proof run; those fragments were removed again before staging. The measured
fused helper itself is unchanged by that stale retrieval.

## Rebench

- RCH worker: `ts1`
- Probe: `bidiag_fused_rank_k_update_perf_probe`
- Reference time: `18.463455 ms`
- Fused time: `6.830909 ms`
- Speedup: `2.702928x`
- Digest: `0xd60df77cdefac734 == 0xd60df77cdefac734`

## Decision

Keep.

Score: `6.75 = Impact 3.0 * Confidence 4.5 / Effort 2.0`

Next primitive: wire this kernel into a separate private DLABRD reducer/probe
with packed block-reflector panels, reconstruction proof, and same-worker RCH
A/B before any public SVD-family route changes.
