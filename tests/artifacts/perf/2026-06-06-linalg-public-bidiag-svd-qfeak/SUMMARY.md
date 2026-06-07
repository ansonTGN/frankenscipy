# qfeak public bidiagonal SVD route

Bead: `frankenscipy-qfeak`
Lever: route full-rank tall public `svd`, `svdvals`, `lstsq`, and `pinv` through the proven deterministic bidiagonal thin-SVD backend when strict gates pass; otherwise keep the existing `safe_svd` path.

## Baseline

Command:

```text
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- 512x256
```

Worker: `ts1`

```text
lstsq/512x256 time: [83.653 ms 84.177 ms 84.728 ms]
pinv/512x256  time: [85.502 ms 85.901 ms 86.289 ms]
```

## After

Command:

```text
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- 512x256
```

Worker: `ts1`

```text
lstsq/512x256 time: [72.754 ms 73.620 ms 74.622 ms]
pinv/512x256  time: [74.882 ms 75.456 ms 76.088 ms]
```

Same-worker Criterion deltas:

```text
lstsq speedup: 84.177 / 73.620 = 1.14x
pinv speedup:  85.901 / 75.456 = 1.14x
```

Same-binary A/B probe on `vmi1149989`, comparing the old `safe_svd` public reference to the routed public calls:

```text
reference_lstsq_ms=149.552773
routed_lstsq_ms=77.740945
lstsq_speedup=1.923732
reference_pinv_ms=145.095593
routed_pinv_ms=77.801023
pinv_speedup=1.864957
lstsq_rank=256
pinv_rank=256
lstsq_max_abs_diff=2.17292850379635638e-12
pinv_max_abs_diff=9.15656439559597857e-14
```

## Behavior proof

Golden:

```text
public_svd_lstsq_pinv_golden_payload passed on ts1
payload_sha256=1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
```

Isomorphism:

- Ordering: routed singular values remain descending; the public gate rejects non-descending spectra.
- Tie handling: adjacent gaps less than `64 * EPSILON * max_s` are treated as clustered and fall back to `safe_svd`.
- Rank boundary: full-rank route requires `min_s > max(threshold, 64 * EPSILON * max_s)`.
- Floating point: route requires finite inputs, deterministic thin-SVD success, and reconstruction error within `1e-8 * max_abs(A) * sqrt(cols)`. Public proof test compares routed `svd`/`svdvals`/`lstsq`/`pinv` against the old `safe_svd` reference.
- RNG: none.
- Certificates: public `lstsq` and `pinv` still report `SolverAction::SVDFallback`, preserve matrix shape/rcond fields, and record portfolio evidence.

Focused proof commands:

```text
RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib public_bidiag_svd_route --locked -- --nocapture
RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib public_bidiag_svd_route_perf_probe --locked -- --ignored --nocapture
RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib public_svd_lstsq_pinv_golden_payload --locked -- --nocapture
```

Validation:

```text
cargo fmt -p fsci-linalg --check
git diff --check -- crates/fsci-linalg/src/lib.rs
ubs crates/fsci-linalg/src/lib.rs
RCH_FORCE_REMOTE=1 rch exec -- cargo check -p fsci-linalg --all-targets --locked
RCH_FORCE_REMOTE=1 rch exec -- cargo clippy -p fsci-linalg --all-targets --no-deps --locked -- -D warnings
```

UBS reported zero critical issues; existing broad warnings remain in the large linalg file.

## Score

Score: `4.2 = Impact 2.8 * Confidence 4.5 / Effort 3.0`

Keep rationale: the public Criterion cases improved on the same worker, the same-binary A/B probe shows the route itself removes roughly half the old SVD-route cost, and the route has conservative fallbacks for non-finite, rank-boundary, clustered-tie, and reconstruction-proof failures.
