# Packed-B Panel GEMM Primitive Selection

- Bead: `frankenscipy-8l8r1.26`
- Profile target: `matmul/1024x1024`
- Focused baseline: `931.06 ms`
- Golden SHA before edits: `48613a728da5350067a920bf0e68b27fc11efd4537046584e2b28a25e75dd771`

## Selected Lever

Use one bounded packed-B panel for full 8-column SIMD blocks in the large `matmul_flat_workspace` path. Pack each complete `B[k][j0..j0+8]` panel into contiguous memory once, then the existing 4-row x 8-column SIMD tile loop consumes that panel in increasing `k` order.

## Exclusions

- Not a row-panel accumulator retry.
- Not a SIMD lane-width split.
- Not an MR6 or other row-count calibration.
- Not a scalar/ragged-edge change.
- Not a public API, validation, tolerance, output layout, RNG, tie-breaking, or global-state change.
- No C BLAS, MKL, LAPACK, XLA, unsafe code, or fast-math contraction.

## Isomorphism Obligations

For every output cell, the multiply/add sequence remains `k = 0..ka` in increasing order. The B operand source changes from strided `b_flat[k * n + j0 + lane]` loads to contiguous packed-panel loads containing the same values in the same lane order. Row order, column order, residual behavior, RNG absence, and tie-breaking absence are unchanged.

## Keep Gate

Keep only if the after benchmark shows a real `matmul/1024x1024` median win versus `931.06 ms`, the stable golden SHA remains unchanged, RCH release matmul tests pass, and Score is at least `2.0`.

Target score: `6.0 = impact 4 * confidence 3 / effort 2`.
