# Keep: deterministic thin SVD reconstruction stage 3

Bead: frankenscipy-egf12

## Profile target

The profiler-backed target remains the full-rank rectangular SVD fallback surface:
`lstsq/512x256` and `pinv/512x256`. The current public route still uses nalgebra
SVD; this stage intentionally does not wire public routing. It adds the missing
private reconstruction primitive needed before the public route can move to the
safe-Rust Golub-Kahan path.

Baseline guard, RCH `ts1`:

- `lstsq/512x256`: `[86.278 ms 86.819 ms 87.374 ms]`
- `pinv/512x256`: `[92.424 ms 93.199 ms 93.957 ms]`

Post-change public guard, RCH `vmi1227854`:

- `lstsq/512x256`: `[114.59 ms 115.49 ms 116.39 ms]`
- `pinv/512x256`: `[118.34 ms 119.65 ms 121.03 ms]`

The post-change guard is cross-worker only because `rch exec` exposes no worker
pin. No public speedup is claimed for this bead; the next wiring bead must use a
same-worker before/after confirmation.

## Lever

One lever: compose Stage 1 Golub-Kahan reflectors with Stage 2 bidiagonal SVD
factors:

`A = Q * B * V^T`, `B = Ub * Sigma * Vb^T`, so
`A = (Q * Ub) * Sigma * (Vb^T * V^T)`.

The new private `DeterministicThinSvd` stores the reconstructed thin `U`, `s`,
and `Vt`, plus private pinv/lstsq consumers used by proof tests.

## Isomorphism proof

- Ordering: Stage 2 singular values keep descending eigenvalue order with
  deterministic original-index ties; Stage 3 does not reorder factors.
- Sign: final `U` columns and `Vt` rows are flipped as paired factors based on a
  deterministic first-nonzero pivot. This preserves `U * Sigma * Vt` exactly up
  to the same floating-point operations.
- Floating point: the public API paths are untouched. Private reconstruction
  uses fixed-order Householder products and matrix products; no RNG or
  data-dependent tie randomness is introduced.
- Rank thresholds: proof tests use existing public `lstsq` and `pinv`
  thresholds and verify reconstructed-factor consumers match the public routes.

## Proof artifacts

- Private thin SVD golden SHA-256:
  `086c0c88cc52d431b9a497f7da60d64a25f2acde49ab0b387f6c03f44547fc73`
- Public `svd`/`svdvals`/`lstsq`/`pinv` golden SHA-256:
  `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`
- Private golden reconstruction error:
  `9.76996261670137756e-15`
- Private thin U orthogonality error:
  `1.88737914186276612e-15`
- Private Vt orthogonality error:
  `1.11022302462515654e-15`

Gates:

- `rch exec -- cargo test -p fsci-linalg --lib thin_bidiag_svd --locked -- --nocapture`
- `rch exec -- cargo test -p fsci-linalg --lib public_svd_lstsq_pinv_golden_payload --locked -- --nocapture`
- `rch exec -- cargo check -p fsci-linalg --all-targets --locked`
- `rch exec -- cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`
- `cargo fmt -p fsci-linalg --check`
- `ubs crates/fsci-linalg/src/lib.rs`

Score: Impact 4 x Confidence 4 / Effort 4 = 4.0. Keep.
