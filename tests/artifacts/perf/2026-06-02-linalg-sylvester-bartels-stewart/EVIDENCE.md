# solve_sylvester: Bartels–Stewart back-substitution — perf evidence

Bead: frankenscipy-8l8r1 ([perf][no-gaps] safe-Rust BLAS/LAPACK-class kernels)
Crate: fsci-linalg
Function: `solve_sylvester` (and `solve_continuous_lyapunov`, which delegates to it)
Lever: replace the full Kronecker operator + O((mn)^3) full-pivot LU with the
       Schur-structured column-block back-substitution SciPy/LAPACK use.
Agent: SapphireDove (claude-opus-4-8)
Date: 2026-06-02
Win type: ALGORITHMIC (host-noise-independent), O(m^3·n^3) → O(n·m^3).

## The gap

`solve_sylvester` reduced A,B to real Schur form (T_A, T_B) — correct — but then
built the full (mn × mn) operator (I_n ⊗ T_A + T_B^T ⊗ I_m), vectorised F, and
solved with `full_piv_lu`. That is O((mn)^3) work and O((mn)^2) memory for what is
structurally a triangular back-substitution. SciPy (LAPACK *trsyl) exploits the
Schur structure; we did not.

## The lever

T_B is upper quasi-triangular, so column j of Y depends only on columns k ≤ j:
(T_A Y + Y T_B)[:,j] = T_A y_j + Σ_k y_k·tb[k,j], with tb[k,j]=0 for k>j+1.
Sweep columns left→right:
- 1×1 diagonal block: solve (T_A + tb[j,j] I) y_j = f_j − Σ_{k<j} tb[k,j] y_k
  (one m×m LU).
- 2×2 block (complex eigenpair, nonzero subdiagonal): solve the coupled 2m×2m
  system for [y_j; y_{j+1}].
Overall O(n·m^3) instead of O(m^3·n^3); same Schur reduction as before.

## Parity proof

- Numerically equivalent to the old Kronecker path: same-run max|Δ| over the full
  solution = 2.35e-16 / 3.33e-16 / 3.79e-16 at n=16/24/32 (machine precision).
- SciPy differential conformance `diff_linalg_solve_sylvester` (1e-9 abs): PASS —
  matches the upstream reference (SciPy uses the same algorithm).
- New test `solve_sylvester_complex_eigenvalues_2x2_block`: A=[[0,-1],[1,0]]
  (eigenvalues ±i) forces the 2×2 branch; residual < 1e-9.
- Singular operator still surfaces as `SingularMatrix` (per-block LU returns no
  solution exactly when a diagonal block is singular) — existing error test green.
- Full `cargo test -p fsci-linalg --release --lib`: all pass; clippy 0 warnings;
  rustfmt clean.

## Perf witness (old Kronecker → Bartels–Stewart, same run, --release)

    solve_sylvester 16x16: kronecker=0.0059s bartels_stewart=0.0001s  58.18x
    solve_sylvester 24x24: kronecker=0.0521s bartels_stewart=0.0003s 181.58x
    solve_sylvester 32x32: kronecker=0.3315s bartels_stewart=0.0006s 514.84x

Speedup grows with n (matches O((mn)^3)/O(n·m^3)). Score >> 2.0. Reproduce:
    cargo test -p fsci-linalg --release solve_sylvester_perf -- --ignored --nocapture

## Follow-up (not in this lever)

`solve_discrete_lyapunov` builds its OWN (A⊗A − I) n²×n² Kronecker operator — the
same O(n^6) shape — and was left unchanged here (one lever per commit). It is a
direct candidate for the analogous Schur-structured treatment.
