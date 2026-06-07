# perf: solve_discrete_lyapunov — upper-triangular fast path

## Lever (ONE)
The discrete Bartels-Stewart column sweep solves, for each 1×1 Schur block,
`(T[j,j]·T − I) y_j = rhs`, where `T` is the real Schur factor of A (upper
**quasi**-triangular). The code materialized the full n×n matrix
`sys = T[j,j]·T − I` (O(n²)) and solved it with a general LU (O(n³)) — per column.

When `T` is **strictly upper triangular** (A has only real eigenvalues — the
common case; detected once via `t[(i+1,i)] == 0` for all i), `T[j,j]·T − I` is
upper triangular, so the column is solved by O(n²) back-substitution directly
against `T` (`solve_scaled_upper_triangular_minus_id`), skipping both the n×n
build and the LU. The sweep drops from O(n⁴) to O(n³). A 2×2 Schur block
(complex eigenpair) falls back to the LU path unchanged.

## Parity — tolerance (this solver's established standard)
- An upper-triangular matrix needs **no pivoting**, so a partial-pivot LU on it
  reduces to the same back-substitution (`L = I`); the off-diagonal term
  `scale·U[i][k]` is formed exactly as the LU path materialized `sys[i][k]`. Only
  the dot-product summation order differs → results agree to rounding.
- Independent correctness proof: residual `‖A X Aᵀ − X + Q‖_max` stays at machine
  level for BOTH builds (2.2e-16 … 5.3e-15 across n = 6…256, well-conditioned
  matrices). `acc` (X[0][0] sums) are **identical** across builds at every size;
  n=6 is bit-identical, larger n differ only in low mantissa bits. See
  `golden_payload.txt`.
- All 5 `fsci-linalg` discrete-Lyapunov unit tests pass, including
  `solve_discrete_lyapunov_complex_eigenvalues_2x2_block` (LU fallback),
  `solve_discrete_lyapunov_diagonal`, `solve_discrete_lyapunov_stable`, and
  `solve_discrete_lyapunov_unit_eigenvalue_errors` (singular → error, matching the
  LU `None` path).

## Timing — rch remote, 64 cores, `--profile release-perf`
`solve_discrete_lyapunov` on upper-triangular A (real eigenvalues, |λ|<0.5),
Q symmetric. Same machine, back-to-back (baseline via stashing the change).

| n   | baseline (LU/col) | upper-tri back-sub | speedup |
|-----|-------------------|--------------------|---------|
| 64  | 2.055 ms          | 0.429 ms           | 4.79x   |
| 128 | 25.548 ms         | 2.975 ms           | 8.59x   |
| 256 | 429.603 ms        | 33.199 ms          | 12.94x  |

Score ≥ 2.0 cleared at every size; the win grows with n (the eliminated terms are
the per-column O(n²) build + O(n³) LU — for larger matrices the O(n³) Schur +
transform become the floor).

Harness: `crates/fsci-linalg/src/bin/perf_discrete_lyapunov_tri.rs`
Run: `cargo run --profile release-perf -p fsci-linalg --bin perf_discrete_lyapunov_tri`

## Notes
- Sibling of the `solve_sylvester`/`solve_continuous_lyapunov` upper-triangular
  fast path (commit e8c25124); same structural lever on the separate discrete
  solve path.
- Conformance harness could not run on the rch worker (no scipy); the baked-in
  discrete-Lyapunov unit tests provide the parity guarantee.
