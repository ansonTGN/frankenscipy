# Primitive Selection: Rectangular Lstsq Single-SVD Reuse

- Bead: `frankenscipy-8l8r1.25`
- Target: `fsci-linalg::lstsq_with_casp`
- Baseline worker: `vmi1153651`
- Baseline keep row: `baseline_lstsq/1000x500` median `1.0625 s`
- Golden-before stable SHA-256: `bdf491ce0154bec5825fcdd3d68a23ec5941bbe4308584ceb2440c136a8722b6`

## Selected Lever

For rectangular least-squares inputs that must take the SVD solve path, compute one full SVD and reuse its singular values for condition estimate, rank, certificate data, pseudo-inverse construction, and solve output. This removes the current condition-only SVD plus full-SVD duplication for the top profile target.

This is a decomposition-reuse primitive: eliminate redundant factorization work while preserving the exact solver family and tolerance policy, instead of retuning GEMM row shape again.

## Explicit Exclusions

- Not a QR, SVD, or LAPACK algorithm replacement.
- Not a GEMM, packing, SIMD-width, or row-count retry.
- Not a tolerance, condition threshold, portfolio posterior, or fallback-policy change.
- Not a validation, shape, finite-check, error-order, or API-contract change.
- Not a residual formula or output layout change.

## Isomorphism Contract

- Matrix/vector validation and finite-check order remain unchanged.
- The selected action for rectangular inputs remains `SVDFallback`.
- Portfolio posterior and expected-loss calculation still use the same `rcond_estimate` inputs.
- Singular-value ordering and threshold semantics remain unchanged if the golden SHA matches.
- The solve result still comes from the full SVD pseudo-inverse path.
- Residual computation, output ordering, certificate fields, RNG absence, tie-breaking absence, and global-state absence remain unchanged.

## Keep Gate

Keep only if the after run preserves the stable golden SHA and delivers a real `baseline_lstsq/1000x500` median win versus `1.0625 s` with Score `>= 2.0`.
