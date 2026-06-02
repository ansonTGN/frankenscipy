# Closeout

Bead: `frankenscipy-rqh0o`

Change:
`ellipkinc_scalar` and `ellipeinc_scalar` now skip 15-point quadrature for real scalar `m == 0.0` while preserving the exact prior rounded bits with `0.5 * phi * 1.9999999999999998`.

Before/after:
- `ellipkinc_scalar/phi0.524_m0.0`: 292.86 ns -> 8.9452 ns mean, 96.9% faster.
- `ellipeinc_scalar/phi0.524_m0.0`: 258.90 ns -> 8.5350 ns mean, 96.7% faster.
- `ellipkinc_broadcast_m/scalar_phi_over_m_vec`: 1.1300 us -> 449.52 ns mean, 60.2% faster.
- `ellipeinc_pairwise_vec/phi_vec_m_vec`: 981.22 ns -> 468.04 ns mean, 52.3% faster.

Behavior proof:
- Bitwise golden contract recorded in `golden_elliptic_m0_bits.txt`.
- Focused test `incomplete_elliptic_m_zero_preserves_quadrature_bits` verifies F and E exact bits for representative m == 0.0 inputs and preserves the existing pi/2 complete-integral edge path.
- No vector traversal order, tie-breaking, or RNG behavior changed.
