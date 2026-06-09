# Keep: route public square + band `svd`/`svdvals` through our deterministic thin SVD

Bead: `frankenscipy-08v3m`
Worker fleet: rch

## Lever

Public `svd()` / `svdvals()` used our fast `deterministic_thin_svd` only when
`rows >= 2*cols`. Square and `1 < rows/cols < 2` tall matrices fell to nalgebra's
single-threaded Golub-Kahan SVD, which is ~7× slower than our deterministic
thin SVD (512×512 raw: nalgebra 1553 ms vs deterministic 209 ms).

Added `public_square_or_tall_thin_svd_candidate` (`rows >= cols`, `cols >= 64`),
used **only** by `svd()` / `svdvals()` — deliberately not by `lstsq`/`pinv`,
which have cheaper Cholesky/LU full-rank routes and would *regress* paying the
thin SVD's vector accumulation on near-square inputs. The existing
reconstruction / rank-gap / tie acceptance gate (`public_bidiag_svd_accepts`)
validates every acceptance; an inaccurate or clustered-spectrum factorization
returns `false` and falls back to nalgebra (fail-closed).

## Isomorphism / behavior parity

- Routed factors reconstruct `A` to `recon_err = 9.88e-11`; singular values
  agree with nalgebra to `4.07e-10` (well within the 1e-7 route tolerance, the
  same tolerance bar the existing tall≥2× deterministic route already meets).
- The `lstsq`/`pinv` thin route still uses the unchanged `rows >= 2*cols`
  candidate — no regression to the square-lstsq (values-only SVD + QR) path.
- Public golden payload (`10×5`, `cols=5 < 64`) never enters the new candidate;
  golden SHA-256 byte-identical:
  `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
- Full `fsci-linalg` release suite: 388 passed, 0 failed. `fsci-conformance`
  compiles clean.

## Performance (rch, 512×512 public `svd()`, same-worker A/B)

`previous_route_ms` is the prior nalgebra full SVD.

- run 1: `1150.47 ms → 232.85 ms` = **4.94×**
- run 2: `1144.32 ms → 236.13 ms` = **4.85×**
- `recon_err = 9.88e-11`, `singular_value_max_diff_vs_nalgebra = 4.07e-10`

## Score

`≈ 4.85–4.94×` on the public square `svd()`/`svdvals()` path. Clears Score ≥ 2.0.
**Keep.** Follow-up: a transpose-based route for **wide** (`rows < cols`)
`svd()`, which still uses nalgebra.
