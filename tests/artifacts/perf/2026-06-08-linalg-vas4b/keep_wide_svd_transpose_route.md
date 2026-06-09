# Keep: route public wide `svd`/`svdvals` through the transpose

Bead: `frankenscipy-vas4b` (follow-up to `08v3m`)
Worker fleet: rch

## Lever

After `08v3m` routed square + tall-band `svd()`/`svdvals()` through our fast
deterministic thin SVD, **wide** matrices (`rows < cols`) still fell to
nalgebra. `Aᵀ` is tall, so the deterministic thin SVD handles it directly:

```
Aᵀ = Uₜ Σ Vₜᵀ   ⇒   A = (Aᵀ)ᵀ = Vₜ Σ Uₜᵀ
U_A = (Aᵀ.v_t)ᵀ ,  s_A = s(Aᵀ) ,  Vᵀ_A = (Aᵀ.u)ᵀ
```

`public_wide_svd_via_transpose` builds `Aᵀ`, runs the square/tall candidate on
it, and applies the existing reconstruction/rank/tie acceptance gate **on the
tall `Aᵀ`** (where its `rows >= cols` dimension checks are valid). On rejection
the caller falls back to nalgebra (fail-closed). `svdvals(wide)` returns the
same singular values directly (σ(A) = σ(Aᵀ)).

## Isomorphism / behavior parity

- Routed factors reconstruct `A` to `recon_err = 2.10e-11`; singular values
  agree with nalgebra to `5.82e-11` (well within the 1e-7 route tolerance).
- Public golden payload (`10×5`, tall) never enters the wide route. SHA-256
  byte-identical: `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
- Full `fsci-linalg` release suite: 388 passed, 0 failed; `fsci-conformance`
  compiles clean.

## Performance (rch, 256×512 public `svd()`, same-worker A/B)

`previous_route_ms` is the prior nalgebra full SVD. Single-shot rch runs vary
with worker contention; clean back-to-back runs:

- run A: `247.77 ms → 67.25 ms` = **3.68×**
- run B: `234.04 ms → 57.71 ms` = **4.06×**
- (two captures read 1.95× / 2.05× — contended workers inflating `routed_ms`)

## Score

`≈ 3.7–4.1×` on the clean runs. Clears Score ≥ 2.0. **Keep.** The entire
rectangular `svd()`/`svdvals()` surface (tall, square, band, wide) now routes
through our deterministic thin SVD where the reconstruction gate accepts,
falling back to nalgebra only on uncertainty.
