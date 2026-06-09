# frankenscipy-44bce keep: scaled-U public SVD acceptance reconstruction

## Target

- Bead: `frankenscipy-44bce`
- Profile-backed hotspot: public square `svd()` route on 512x512 deterministic input.
- Lever: replace public acceptance reconstruction `U * Sigma * Vt` with an equivalent scaled-`U` reconstruction that avoids dense `Sigma` materialization and one dense GEMM.

## Same-worker benchmark

RCH worker: `vmi1227854`

| Probe | Before | After | Speedup |
|---|---:|---:|---:|
| `public_square_svd_route_perf_probe` routed path | `209.530510 ms` | `164.061981 ms` | `1.277142x` |

Unchanged public-route metrics:

- `recon_err = 9.88027437642813311e-11`
- `singular_value_max_diff_vs_nalgebra = 4.07453626394271851e-10`
- `svdvals_max_diff_vs_svd = 4.07453626394271851e-10`

## Isomorphism proof

- Ordering preserved: yes. Singular values, rank filtering, sorting, and sign normalization are unchanged; only the certification reconstruction is changed after the SVD is already computed.
- Tie-breaking unchanged: yes. No comparison, ordering, or threshold policy changed.
- Floating-point behavior: public outputs unchanged. The certification product computes the same `U * Sigma * Vt` matrix by scaling columns of `U` before the final multiply. The proof test compares the old dense-Sigma acceptance decision to the new scaled-`U` decision on deterministic fixtures.
- RNG preserved: yes. No RNG inputs or seeds are touched.
- Golden output SHA: `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.

## Validation

- `proof_scaled_reconstruction_acceptance_rch.txt`: RCH acceptance-equivalence proof passed.
- `public_golden_rch.txt`: RCH public SVD/lstsq/pinv golden passed.
- `check_fsci_linalg_rch.txt`: RCH `cargo check -p fsci-linalg --all-targets --locked` passed.
- `clippy_fsci_linalg_no_deps_rch.txt`: RCH `cargo clippy -p fsci-linalg --all-targets --locked --no-deps -- -D warnings` passed.
- Local crate-scoped `cargo fmt -p fsci-linalg --check` passed.
- `ubs_linalg_acceptance.txt`: zero critical UBS findings for `crates/fsci-linalg/src/lib.rs`.

## Score and next residual

Score: `Impact 3 * Confidence 5 / Effort 1 = 15.0`.

The public-route residual now routes back to the core `deterministic_thin_svd` reduction/backend. Rejected families from this campaign include scalar structural-zero elision, compact-WY/packed-panel replay, QR-first/TSQR public replacement, normal-equation SVD replacement, one-sided Jacobi replacement, square-pinv cutoff/certificate tweaks, and direct column-threading of fused reduction updates. The next primitive should be communication-avoiding/cache-blocked Golub-Kahan reduction or a divide-and-conquer bidiagonal/SVD backend with a fail-closed public acceptance gate.
