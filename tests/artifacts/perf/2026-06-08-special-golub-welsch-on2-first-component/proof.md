# perf: Golub-Welsch O(n²) — first-eigenvector-component-only QL

## Lever (ONE)
Gauss-quadrature weights are `w_i = mu0 · v_i[0]²` — they need only the FIRST
component of each eigenvector of the Jacobi matrix, i.e. the first ROW of the
eigenvector matrix. The previous `golub_welsch` (commit 91fd275a) called
`eigh_tridiagonal(..., eigvals_only=false)`, which accumulates the FULL n×n
eigenvector matrix through the QL rotations — O(n³).

Replace with `gw_tridiagonal_eigen_first_row`: a faithful port of
`fsci_linalg::symmetric_tridiagonal_qr_eigen` (the routine `eigh_tridiagonal`
uses) that applies the IDENTICAL Givens rotations / diagonal updates / Wilkinson
shifts / deflation, but rotates only a single length-n row `z0` (the first row of
the identity) instead of the full matrix. That drops eigenvector accumulation
from O(n³) to **O(n²)** (each rotation now touches 2 entries of `z0`, not n rows).
Implemented self-contained in `fsci-special` (no `fsci-linalg` edit).

## Parity — BYTE-IDENTICAL
- The QL arithmetic (diag/off updates, c/s, shifts, deflation order, 2×2 block
  handling) is copied verbatim, so eigenvalues are identical and
  `z0[j] == eigenvectors[(0, j)]` of the full solver bit-for-bit; the same
  ascending eigenvalue sort is applied. Hence nodes and weights `mu0·z0[j]²` are
  identical to the O(n³) full-eigenvector path.
- Stash A/B vs the committed O(n³) build: every golden value matches EXACTLY at
  15 sig digits — nodes, weights, wsum, and quadrature-exactness — including the
  subnormal Hermite weights (7.31e-23, 2.69e-32). See `golden_payload.txt`.
- All 89 `fsci-special` roots/orthopoly unit tests pass.

## Timing — rch remote, 64 cores, `--profile release-perf`
`roots_legendre(n)`, O(n³) full-eigenvector vs O(n²) first-row:

| n    | O(n³)      | O(n²)     | speedup |
|------|------------|-----------|---------|
| 200  | 4.304 ms   | 1.116 ms  | 3.86x   |
| 500  | 45.335 ms  | 7.225 ms  | 6.27x   |
| 1000 | 356.155 ms | 27.844 ms | 12.79x  |

Speedup grows with n (O(n³)→O(n²)); vs the ORIGINAL dense build+eigh path
(740 ms at n=1000) this is ~27×. Benefits every Gauss-quadrature root finder
routed through `golub_welsch` (legendre/hermite/jacobi/laguerre/gegenbauer/…).

Harness: `crates/fsci-special/src/bin/perf_roots_quadrature.rs`
Run: `cargo run --profile release-perf -p fsci-special --bin perf_roots_quadrature`

## Notes
- Numerically stable (it's the standard Golub-Welsch eigenvector approach, just
  tracking one row) — unlike computing weights from the orthonormal-polynomial
  recurrence, which loses precision at high degree.
- Completes frankenscipy-pg8vg.
