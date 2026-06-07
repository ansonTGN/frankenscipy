# perf: solve_sylvester / solve_continuous_lyapunov — upper-triangular fast path

## Lever (ONE)
The Bartels-Stewart column sweep solves, for each 1×1 Schur block of `T_B`, the
shifted system `(T_A + s·I) y_j = rhs`. `T_A` is the real Schur factor of A, i.e.
**upper quasi-triangular**, yet the code solved each column with a general
O(m³) LU (`ta.clone().lu().solve(..)`).

When `T_A` is **strictly upper triangular** (A has only real eigenvalues — the
common case; detected once via `ta[(i+1,i)] == 0` for all i), `(T_A + s·I)` is
upper triangular and is solved by O(m²) back-substitution
(`solve_shifted_upper_triangular`). The column sweep drops from O(n·m³) to
O(n·m²). A 2×2 Schur block in `T_A` (complex eigenpair) falls back to the LU
path unchanged.

## Parity — tolerance (this solver's established standard)
`solve_sylvester` is documented as conformance-parity at 1e-9 (not bit-exact),
because Bartels-Stewart already replaces the Kronecker LU. The fast path keeps
that bar:
- An upper-triangular matrix needs **no pivoting**, so a partial-pivot LU on it
  has `L = I`, `U = T_A + s·I` and reduces to the *same* back-substitution; only
  the dot-product summation order differs → results agree to rounding.
- Independent correctness proof: residual `‖A X + X Aᵀ − Q‖_max` stays at machine
  level for BOTH builds — 1.9e-15 / 9.3e-15 / 2.8e-14 for n = 6/20/50 (see
  `golden_payload.txt`); the timing-case residuals are 4e-14…5e-13 for both.
  `acc` (X[0][0] sums) match to 6+ digits; solution checksums differ only in low
  mantissa bits.
- All 15 `fsci-linalg` sylvester/lyapunov unit tests pass, including
  `solve_sylvester_matches_scipy_reference`,
  `solve_continuous_lyapunov_matches_scipy_reference`,
  `solve_sylvester_complex_eigenvalues_2x2_block` (LU fallback) and
  `solve_sylvester_singular_system_errors` (zero-diagonal → `SingularMatrix`,
  matching the LU `None`).

## Timing — rch remote, 64 cores, `--profile release-perf`
`solve_continuous_lyapunov` on upper-triangular A (real eigenvalues), Q symmetric.
Same machine, back-to-back (baseline via stashing the change).

| n   | baseline (LU/col) | upper-tri back-sub | speedup |
|-----|-------------------|--------------------|---------|
| 64  | 2.317 ms          | 1.368 ms           | 1.69x   |
| 128 | 27.661 ms         | 8.597 ms           | 3.22x   |
| 256 | 429.540 ms        | 93.389 ms          | 4.60x   |

Score ≥ 2.0 cleared at n ≥ 128; the win grows with n (the eliminated term is the
O(n·m³) sweep — for larger matrices the remaining O(m³) Schur + transform become
the floor).

Harness: `crates/fsci-linalg/src/bin/perf_sylvester_tritriangular.rs`
Run: `cargo run --profile release-perf -p fsci-linalg --bin perf_sylvester_tritriangular`

## Notes
- `solve_continuous_lyapunov` routes through `solve_sylvester` (B = Aᵀ), so it
  inherits the win; `solve_discrete_lyapunov` uses a separate solve path and is
  untouched.
- Conformance harness could not run on the rch worker (no scipy); the baked-in
  `*_matches_scipy_reference` unit tests provide the SciPy parity guarantee.
