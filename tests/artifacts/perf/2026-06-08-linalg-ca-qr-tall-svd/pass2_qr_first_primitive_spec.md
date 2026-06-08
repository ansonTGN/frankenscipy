# Pass 2 - QR-First Tall SVD Primitive Spec

Bead: `frankenscipy-8l8r1.54`

## Profile-Backed Target

Remote RCH criterion baseline on `vmi1153651`:

- `lstsq/512x256`: `[174.74 ms 191.04 ms 209.93 ms]`
- `pinv/512x256`: `[233.44 ms 267.15 ms 307.79 ms]`

Public golden SHA anchor:

- `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`

## One Lever

Add one private QR-first tall SVD candidate:

1. Factor tall full-rank `A` as `A = Q R`.
2. Take the thin factors `Q1 = Q[:, 0..n]` and `R1 = R[0..n, :]`.
3. Compute the existing deterministic thin SVD of the small square `R1`.
4. Form `U = Q1 * U_R`, keep `S` and `Vt` from `R1`, then call the existing SVD sign canonicalizer.
5. Let the existing public reconstruction/rank/tie guard accept or fall through.

This is the smallest proofable CA-QR/TSQR stepping stone. The deeper follow-up is replacing the QR factorization internals with block-local QR plus stacked-`R` tree reduction/replay while preserving the same public hook and guard.

## Hook Points

Add the helper beside `public_bidiag_thin_svd_candidate` in `crates/fsci-linalg/src/lib.rs`.

Try it before the existing bidiagonal candidate in:

- `lstsq_with_casp`
- `pinv_with_casp`
- `svd`
- `svdvals`

The current bidiagonal route remains the fallback.

## Behavior Contract

- Ordering: singular values must remain nonincreasing; clustered/tied spectra are rejected by the same adjacent-gap guard.
- Tie-breaking: sign canonicalization uses the existing first-nonzero pivot rule in `canonicalize_svd_factor_signs`.
- Floating point: public outputs may differ only within the current guarded tolerance; the golden payload SHA must remain unchanged.
- RNG: none.
- Rank: only full-rank tall matrices pass; thresholds remain caller-specific (`cond * max_s`, `atol + rtol * max_s`, or public default).
- Safety: safe Rust only; no unsafe, no C BLAS/LAPACK/MKL/XLA, no thread fanout.

## Score

Impact `4` x Confidence `3` / Effort `3` = `4.0`.

Keep gate: same-worker RCH rebench must improve the criterion middle estimate to at most:

- `lstsq/512x256 <= 152.83 ms`
- `pinv/512x256 <= 213.72 ms`
