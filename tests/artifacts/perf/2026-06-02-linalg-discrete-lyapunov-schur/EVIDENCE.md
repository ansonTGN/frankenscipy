# solve_discrete_lyapunov: Schur/Stein back-substitution — perf evidence

Bead: frankenscipy-kti79 (child of 8l8r1 no-gaps umbrella)
Crate: fsci-linalg
Function: `solve_discrete_lyapunov` (Stein equation A X A^T − X + Q = 0)
Lever: replace the full (A⊗A − I) n²×n² operator + O(n^6) full-pivot LU with the
       Schur-structured column-block back-substitution SciPy/SLICOT use.
Agent: SapphireDove (claude-opus-4-8)
Date: 2026-06-02
Win type: ALGORITHMIC (host-noise-independent), O(n^6) → O(n·n^3).

## The gap

`solve_discrete_lyapunov` formed the full Kronecker operator (A⊗A − I) of size
n²×n² and solved (A⊗A − I)·vec(X) = −vec(Q) with `full_piv_lu` — O(n^6) work and
O(n^4) memory. The sibling `solve_sylvester` had the same shape and was fixed in
commit dbaaba44 (bead omw7n); this is the analogous treatment for Stein.

## The lever

Reduce A to real Schur form A = U T U^T (T upper quasi-triangular). With
Y = U^T X U and C = −U^T Q U the equation becomes T Y T^T − Y = C. Column j of
(T Y T^T) is Σ_q T[j,q]·(T y_q), and T[j,q] = 0 for q < j except a 2×2 block's
subdiagonal. Sweeping columns bottom→top, all y_q with q > j are known, so:
- 1×1 block: solve (T[j,j]·T − I) y_j = c_j − Σ_{q>j} T[j,q]·(T y_q)  (one n×n LU).
- 2×2 block (complex eigenpair, nonzero subdiagonal): solve the coupled 2n×2n
  system for its two columns.
T·y_q is cached per solved column. Overall O(n·n^3) instead of O(n^6).

## Parity proof

- Numerically equivalent to the old Kronecker path: same-run max|Δ| over the full
  solution = 4.66e-15 / 5.33e-15 / 1.69e-14 at n=12/16/24 (machine precision).
- SciPy differential `diff_linalg_lyapunov` (1e-9 abs, continuous + discrete):
  PASS (run locally; the rch worker lacked numpy/scipy — environment-only, not a
  code regression).
- New `solve_discrete_lyapunov_complex_eigenvalues_2x2_block`: a 0.6·rotation
  (complex pair |λ|=0.6) forces the 2×2 branch; residual < 1e-9.
- Unit-modulus eigenvalue (operator singular, a=[[1]]) still fails closed as
  `SingularMatrix`; non-square still rejected; full fsci-linalg lib suite green
  (343 tests); clippy 0 warnings; rustfmt clean.

## Perf witness (old Kronecker → Schur/Stein, same run, --release)

    solve_discrete_lyapunov 12x12: kronecker=0.0009s schur_stein=0.0001s  15.56x
    solve_discrete_lyapunov 16x16: kronecker=0.0045s schur_stein=0.0001s  77.35x
    solve_discrete_lyapunov 24x24: kronecker=0.0515s schur_stein=0.0002s 339.46x

Speedup grows with n (matches O(n^6)/O(n^4)). Score >> 2.0. Reproduce:
    cargo test -p fsci-linalg --release solve_discrete_lyapunov_perf -- --ignored --nocapture

## Family status

- solve_sylvester: Bartels–Stewart (dbaaba44 / omw7n). DONE.
- solve_continuous_lyapunov: delegates to solve_sylvester → already O(n^3). DONE.
- solve_discrete_lyapunov: this lever. DONE.
The O(n^6) Kronecker/full_piv_lu pattern is now cleared from the dense
Lyapunov/Sylvester family in fsci-linalg.
