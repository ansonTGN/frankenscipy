# perf: Gauss-quadrature roots — golub_welsch dense eigh → tridiagonal eigh

## Lever (ONE)
`golub_welsch` (the shared kernel of `roots_legendre`/`roots_hermite`/
`roots_jacobi`/`roots_laguerre`/`roots_gegenbauer`/…) built the Golub-Welsch
**symmetric tridiagonal** Jacobi matrix as a DENSE n×n array and solved it with
the general dense symmetric eigensolver (`fsci_linalg::eigh`, which Householder-
tridiagonalizes the already-tridiagonal matrix, then runs QL). Replace with the
dedicated tridiagonal eigensolver `eigh_tridiagonal(d, e, …)`, skipping the n×n
densification and the wasted Householder reduction. (Unblocked now that
frankenscipy-me4pf — eigh_tridiagonal's ±-pair eigenvector bug — is fixed; this
is exactly SciPy's `roots_legendre` path.)

## Parity — tolerance (this is iterative eigensolution; ~1 ULP vs the dense path)
- Nodes/weights match the dense build to ~1 ULP (n=8: w[0] 1.012285362903761e-1
  vs 1.012285362903762e-1). Independent correctness proof: Gauss-Legendre
  integrates `x^(2n-2)` exactly, and the quadrature-exactness error is
  3.1e-16 / 2.5e-15 / 6.5e-16 for n = 8/32/100 (was 0.65 before me4pf was
  fixed). See `golden_payload.txt`.
- All 89 `fsci-special` roots/orthopoly unit tests pass (incl. scipy-reference
  values). This path is the same algorithm class SciPy uses, so it is at least as
  close to SciPy as the dense path.

## Timing — rch remote, 64 cores, `--profile release-perf`
`roots_legendre(n)`:

| n    | dense build+eigh | tridiagonal eigh | speedup |
|------|------------------|------------------|---------|
| 200  | 6.728 ms         | 4.279 ms         | 1.57x   |
| 500  | 94.175 ms        | 44.383 ms        | 2.12x   |
| 1000 | 740.256 ms       | 339.788 ms       | 2.18x   |

Score ≥ 2.0 cleared at n ≥ ~400; the win grows with n. Benefits every
Gauss-quadrature root finder routed through `golub_welsch`.

Harness: `crates/fsci-special/src/bin/perf_roots_quadrature.rs`
Run: `cargo run --profile release-perf -p fsci-special --bin perf_roots_quadrature`

## Notes
- This is a constant-factor structural win (both paths are O(n³): the QL
  eigenvector accumulation is O(n³)). The deeper O(n²) Golub-Welsch — track only
  the FIRST eigenvector component (first row of Q) through the QL rotations —
  would be ~100x at n=1000 and is filed as a follow-up.
