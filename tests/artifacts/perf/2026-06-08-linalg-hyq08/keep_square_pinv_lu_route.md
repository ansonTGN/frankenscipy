# Keep: solve-only full-rank square `pinv` via LU inverse (skip full SVD)

Bead: `frankenscipy-hyq08` (sibling of `frankenscipy-8l8r1.62` / `g8u9m`)
Worker fleet: rch

## Lever

`pinv` of a full-rank **square** matrix fell through to
`safe_svd(matrix, true, true)` — a full SVD computing both U and V — because
`public_tall_thin_svd_candidate` is tall-only, so square matrices never reached
a fast route. But for a full-rank square matrix the Moore-Penrose pseudo-inverse
**equals the ordinary inverse**, which the in-house LU computes far cheaper
(`inv_blocked` for `n >= 1024`, nalgebra LU otherwise).

`pinv_full_rank_square_lu`:

1. Guard like the tall Cholesky route: square, `n >= FULL_RANK_TALL_PINV_MIN_COLS
   (128)`, default thresholds only (`atol == 0`, `rtol <= n·ε`), all finite.
2. Compute the inverse via `inv_blocked` (large `n`) or nalgebra LU `try_inverse`.
3. **Accept only when `A·X ≈ I`** to `FULL_RANK_TALL_PINV_RIGHT_INVERSE_REL_TOL
   · √n` — the full-rank + well-conditioned certificate. A singular,
   ill-conditioned, or rank-deficient matrix produces a large residual and is
   rejected → falls back to the rank-revealing SVD route (fail-closed).
4. `rank = n`; cheap 1-norm condition estimate `rcond ≈ 1/(‖A‖₁·‖A⁻¹‖₁)`.

Wired into `pinv_with_casp` under `RuntimeMode::Strict`, after the tall Cholesky
route and before the public thin-SVD candidate.

## Isomorphism / behavior parity

- New CI test `pinv_full_rank_square_lu_route_matches_svd_and_inv` (n=128):
  asserts the route fires (`rank == 128`), the output matches the SVD
  pseudo-inverse to `1e-9`, matches the ordinary `inv()` to `1e-9`, and
  satisfies the Moore-Penrose identity `A·A⁺·A == A`.
- Small existing `pinv` tests (2×2) stay on the SVD path via the `n >= 128`
  guard.
- Public contract unchanged: `public_svd_lstsq_pinv_golden_payload` uses a 10×5
  (rectangular) matrix and never enters the square route. Golden SHA-256
  byte-identical: `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
- Full `fsci-linalg` release suite: 387 passed, 0 failed.
- Routed pseudo-inverse agrees with the SVD reference to
  `pinv_max_abs_diff = 1.55e-14` (machine precision — the inverse is essentially
  exact for this well-conditioned matrix).

## Performance (rch, 512×512 probe, same-worker A/B)

The probe's `reference_pinv_ms` is exactly the pre-change route (full SVD).

- run 1: `reference 1152.94 ms → routed 53.91 ms` = **21.39×**
- run 2: `reference 1032.68 ms → routed 51.42 ms` = **20.08×**
- `pinv_rank = 512`, `pinv_max_abs_diff = 1.55e-14` (stable)

## Score

`≈ 20–21×` vs the prior full-SVD square route. Clears Score ≥ 2.0 by a wide
margin. **Keep.** Square SVD (computing the full n×n V) is the most expensive
factorization in the family; LU inversion sidesteps it entirely. Even larger
gains expected at `n >= 1024` where the parallel blocked `inv_blocked` engages.
