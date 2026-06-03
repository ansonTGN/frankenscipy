# frankenscipy-8l8r1.21 primitive selection

Pass: 3 - Alien Primitive Selection

## Symptom

The post-`frankenscipy-8l8r1.20` linalg reprofile still ranks
`matmul/1024x1024` first. Fresh RCH Criterion for this pass measured
`matmul/1024x1024` at median `348.89 ms` on `vmi1227854`.

## Selected Primitive

Use a row-owned C output layout in the large flat-workspace GEMM path.

The current path computes into a contiguous `c_flat` result buffer, then
materializes the public `Vec<Vec<f64>>` output by copying every row through
`chunks(n).to_vec()`. The trial lever allocates the public row-major output
up front and stores each completed 4x8 tile directly into the owned C rows.

This is an output-materialization and memory-layout primitive, not a new
tile-shape retry. It keeps the already-landed 4x8 micro-kernel and 64-row
macro-block traversal.

## Exclusions

- Not a packed-B retry. `frankenscipy-jhtc6` already landed full-width packed B
  panels, and later packed-panel attempts were recorded separately.
- Not a wider register-tile retry. Prior 8x8 and 4x16 no-pack attempts were
  rejected.
- Not a row-cache blocking retry over the old `Vec<Vec<_>>` kernel. This lever
  only changes the large flat-workspace path that already has profile-backed
  `1024x1024` evidence.

## Isomorphism Contract

- Public API and error behavior remain unchanged.
- Validation order remains unchanged.
- Output row order remains unchanged.
- Each output cell still accumulates `k = 0..ka` monotonically with separate
  multiply/add operations.
- No RNG, tie-breaking, or global-state surface exists.
- Golden sorted test-line SHA-256 must remain
  `61e12eb58f34ccba1dcedd29425ff3292fd7df5769f7411352cd2a617a58d6c7`.

## Keep Gate

Reject unless focused RCH Criterion shows a real `matmul/1024x1024` win against
the pass baseline `348.89 ms` and Score is at least `2.0`.
