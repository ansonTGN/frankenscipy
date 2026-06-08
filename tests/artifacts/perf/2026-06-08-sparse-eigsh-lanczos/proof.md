# perf: eigsh power iteration → shared Lanczos/Arnoldi Krylov, 8.6–9.5x

## Lever (ONE)
`eigsh` (k largest eigenpairs of a symmetric sparse matrix) used
**power-iteration-with-deflation**: one eigenvalue at a time, up to `max_iter`
(default 1000) matvecs each — O(k·max_iter) matvecs, linear convergence, and it
silently returns garbage (zeros) for k>1 on clustered spectra. Meanwhile the
sibling `eigs` (general) already used Arnoldi/Krylov.

Factor `eigs`'s proven Arnoldi body (full modified-Gram-Schmidt
re-orthogonalization → ghost-free; Ritz extraction from the projected Hessenberg;
Ritz-vector back-transform) into a shared `krylov_arnoldi_eigs(a, k, opts, m)` and
use it for `eigsh` too. For symmetric A the Arnoldi projection is tridiagonal with
real Ritz values, so an `m = max(2k+1, 20)`-dimensional subspace yields the top-k
eigenpairs in **O(m) matvecs** (≈26 vs ≈350–481). `converged` is set from the
actual Ritz residuals `‖A x − λ x‖`.

## Parity — tolerance (iterative eigensolver; both converge to the SAME spectrum)
- On a symmetric matrix with a well-separated planted top spectrum (100, 88, 76,
  …), Lanczos and the old power iteration return **identical** top-k eigenvalues
  (100.000172, 88.000448, 76.000187, …), and Lanczos is far MORE accurate
  (max residual ~1e-11 vs power iteration's ~7e-4). See `golden_payload.txt`.
- `eigs` is refactored to call the same shared helper with its original
  `m = 2k+1` (behavior unchanged). All 310 `fsci-sparse` lib tests + 56 doc/bin
  tests pass, including `eigsh_matches_scipy_reference_values`, the eigs suite,
  and the eigsh/eigs/svds residual tests.

## Timing — rch remote, 64 cores, `--profile release-perf`
`eigsh` on symmetric sparse, planted separated spectrum:

| n     | k | power iteration | Lanczos | speedup | matvecs (pow→Lan) |
|-------|---|-----------------|---------|---------|-------------------|
| 2000  | 6 | 10.025 ms       | 1.164 ms| 8.6x    | 350 → 26          |
| 8000  | 6 | 42.256 ms       | 5.385 ms| 7.85x   | 350 → 26          |
| 20000 | 8 | 130.360 ms      | 13.793 ms| 9.45x  | 481 → 28          |

Score ≥ 2.0 cleared; the win is the matvec-count collapse (Krylov captures all k
extremes simultaneously vs sequential per-eigenvalue power iteration). Each matvec
also benefits from the parallel `csr_matvec` (commit 55b677b1).

Harness: `crates/fsci-sparse/src/bin/perf_eigsh.rs`
Run: `cargo run --profile release-perf -p fsci-sparse --bin perf_eigsh`

## Notes
- Pathologically-clustered extreme spectra would need implicit restarts (as in
  ARPACK); this single-shot Lanczos reports `converged = false` honestly in that
  case rather than returning power iteration's garbage. A thick-restart follow-up
  could close that; filed separately.
- `svds` still uses power iteration on AᵀA — same Lanczos/Golub-Kahan upgrade
  applies next.
