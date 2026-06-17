# Reject: eigenvalues-only tall lstsq spectrum

Bead: `frankenscipy-8l8r1.63`

## Baseline correction

The first broad public-route reprofile used ignored tests without
`--test-threads=1`, so per-route timings were contaminated by concurrent probe
execution. The corrected single-probe baseline was:

- Worker: `ovh-a`
- Command: `RCH_REQUIRE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib --locked public_band_tall_lstsq_route_perf_probe -- --ignored --nocapture --test-threads=1`
- `reference_lstsq_ms=79.033940`
- `routed_lstsq_ms=13.079870`
- `lstsq_speedup=6.042410`
- `rank=256`
- `lstsq_max_abs_diff=1.92290627865077113e-13`

## Lever Tried

Inside `lstsq_full_rank_tall_cholesky`, replace the unused full
`gram.clone().symmetric_eigen()` eigensystem with
`gram.symmetric_eigenvalues()`, preserving the same eigenvalue-derived
singular values and all solve/refinement gates.

## Result

Same-worker after on `ovh-a`:

- `reference_lstsq_ms=78.085900`
- `routed_lstsq_ms=13.178585`
- `lstsq_speedup=5.925211`
- `rank=256`
- `lstsq_max_abs_diff=1.92290627865077113e-13`

Behavior stayed unchanged, but routed time regressed
`13.079870 ms -> 13.178585 ms` (`0.9925x`). Source was restored.

## Verdict

Rejected. Do not repeat eigenvalues-only cleanup for the tall Cholesky
`lstsq` route. The corrected serial public-route reprofile shifts the next
absolute-cost target to square `pinv`.
