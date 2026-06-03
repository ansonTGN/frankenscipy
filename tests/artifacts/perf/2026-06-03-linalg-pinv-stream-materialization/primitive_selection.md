# Primitive Selection: Row-Major GEMM Pinv Materialization

## Profile Target

Fresh RCH reprofile after the direct rectangular `lstsq` win ranked `baseline_pinv/1000x500` first at `499.29 ms`. The focused RCH keep gate for this bead is `437.29 ms`.

## Selected Primitive

Materialize the full pseudoinverse through the project safe-Rust row-major GEMM pipeline:

```text
pinv(A) = V * Sigma^-1 * U^T
```

The current path computes this with nalgebra `DMatrix` products and then copies the result into the API's row-major `Vec<Vec<f64>>`. The selected lever builds row-major `V * Sigma^-1` and `U^T` panels directly from the same SVD factors, then uses the existing safe-Rust GEMM kernel to produce the returned layout.

## Graveyard And Artifact Grounding

- `alien-graveyard`: Communication-Avoiding Algorithms (§9.6) targets dense linear-algebra data movement and pushes large matrix work toward cache-aware BLAS-3 kernels.
- `alien-artifact-coding`: Numerical Linear Algebra requires explicit method selection, factorization proof, condition/rank contracts, and verification for SVD/pseudoinverse paths.

## Exclusions

- Not a retry of the rejected direct diagonal-scaling trial.
- Not an SVD algorithm replacement.
- Not randomized/truncated SVD.
- Not QR/TSQR.
- Not a threshold, rank, warning, or certificate policy change.
- Not a benchmark-harness edit.
- No unsafe code and no external BLAS/LAPACK/XLA backend.

## Score Target

Score target: `6.0 = impact 4 * confidence 3 / effort 2`. Reject if golden proof changes or focused RCH timing lacks a real `baseline_pinv/1000x500` win against `437.29 ms`.
