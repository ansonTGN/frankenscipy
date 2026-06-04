# frankenscipy-8l8r1.24 primitive selection

Pass: 3 - Alien Primitive Selection

## Symptom

Large `fsci_linalg::matmul` remains the profile-backed linalg bottleneck. The
latest accepted broad RCH reprofile ranked `matmul/1024x1024` first at
`216.09 ms`; the fresh focused baseline for this pass measured the same
production gate at `906.45 ms` on `vmi1153651`.

The prior 8-row row-panel SIMD trial preserved behavior but regressed to
`507.79 ms` against its pass baseline, indicating the row-panel shape exceeded
the useful register footprint on the measured worker class.

## Selected Primitive

Use an autotuned register-blocked GEMM microkernel shape: a 6-row by 8-column
SIMD full-tile path ahead of the existing 4-row by 8-column path.

This keeps the existing `f64x8` column vectorization and reuses each loaded
`B[k][j0..j0+8]` vector across six rows, but it does not keep the failed 8-row
live accumulator footprint. The existing 4-row SIMD path remains the fallback
for row remainders, so 64-row macro-blocks can process ten 6-row groups plus
one 4-row group instead of dropping remainders to scalar code.

## Exclusions

- No C BLAS, MKL, LAPACK, XLA, or unsafe code.
- Not the rejected 8-row row-panel shape.
- Not a packed-panel retry.
- Not an output-materialization tweak.
- Not a change to the ragged/edge scalar path.

## Isomorphism Contract

- Public API and error behavior remain unchanged.
- Validation order remains unchanged.
- Output row order remains unchanged.
- Each output cell/lane still accumulates `k = 0..ka` monotonically.
- The source uses lane-wise multiply followed by add; no reduction tree or
  reordered summation is introduced.
- Edge tiles retain the existing scalar loop.
- The 4-row SIMD path remains available for row remainders.
- No RNG, tie-breaking, or global-state surface exists.
- Canonical sorted test-line SHA-256 must remain
  `8cf1b2bfc464356d5a115074ca1ddb3f665f5a825a43ecb93d7c43ac265e7288`.

## Keep Gate

Reject unless focused RCH Criterion shows a real `matmul/1024x1024` win against
the pass baseline `906.45 ms` on defensible RCH evidence and Score is at least
`2.0`.

