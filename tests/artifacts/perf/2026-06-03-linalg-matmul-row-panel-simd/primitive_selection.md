# frankenscipy-8l8r1.23 primitive selection

Pass: 3 - Alien Primitive Selection

## Symptom

After `frankenscipy-8l8r1.22`, the shifted RCH linalg reprofile still ranks
`matmul/1024x1024` first at median `216.09 ms`. The fresh focused baseline for
this pass measured `matmul/1024x1024` at median `219.58 ms` on `vmi1149989`.

## Selected Primitive

Use a communication-avoiding row-panel SIMD accumulator in the full 8-column
flat-workspace GEMM path.

The current SIMD tile reloads the same contiguous `B[k][j0..j0+8]` vector once
per four-row tile. The selected lever increases the bounded full-tile row panel
to 8 rows while preserving the existing 8-column SIMD lane width. For each
`k`, one `Simd<f64, 8>` B vector is loaded and reused across 8 row
accumulators before those rows are stored.

This is a deeper data-reuse primitive than lane vectorization alone: it attacks
communication pressure in the hot tile loop without changing public shape,
validation, output layout, or edge handling.

## Exclusions

- No C BLAS, MKL, LAPACK, XLA, or unsafe code.
- Not a packed-panel retry.
- Not an output-materialization tweak.
- Not a wider scalar tile retry.
- Not a change to the ragged/edge scalar path.

## Isomorphism Contract

- Public API and error behavior remain unchanged.
- Validation order remains unchanged.
- Output row order remains unchanged.
- Each output cell/lane still accumulates `k = 0..ka` monotonically.
- The source uses lane-wise multiply followed by add; no reduction tree or
  reordered summation is introduced.
- Edge tiles retain the existing scalar loop.
- No RNG, tie-breaking, or global-state surface exists.
- Golden sorted test-line SHA-256 must remain
  `61e12eb58f34ccba1dcedd29425ff3292fd7df5769f7411352cd2a617a58d6c7`.

## Keep Gate

Reject unless focused RCH Criterion shows a real `matmul/1024x1024` win against
the pass baseline `219.58 ms` and Score is at least `2.0`.
