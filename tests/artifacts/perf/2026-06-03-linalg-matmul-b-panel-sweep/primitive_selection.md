# frankenscipy-e3713 primitive selection

## Profile-backed target

RCH Criterion baseline on `vmi1227854`:

| row | median |
| --- | ---: |
| `matmul/512x512` | `38.543 ms` |
| `matmul/768x768` | `129.09 ms` |
| `matmul/1024x1024` | `421.95 ms` |

The prior reprofile artifact still ranks `matmul/1024x1024` first at median
`504.32 ms`, so the target remains dense GEMM.

## Alien primitive

Selected: communication-local B-panel sweep for the large flat GEMM workspace.

The current flat-workspace kernel walks row tiles first and revisits every B
panel for each 4-row tile. The selected lever changes only the large rectangular
flat-workspace traversal so each 8-column B panel is swept across all row tiles
before moving to the next panel.

This follows the alien-graveyard communication-avoiding guidance for numerical
kernels: attack data movement, not another scalar/tile micro-tweak.

## Isomorphism contract

- Validation and error order unchanged.
- Public output order unchanged: final return is still row-major `Vec<Vec<f64>>`.
- Floating-point order unchanged per cell: each `c[i][j]` accumulates `k = 0..ka`
  monotonically using the same separate `acc += a * b` updates.
- RNG unchanged: no RNG surface exists.
- Tie-breaking unchanged: GEMM has no tie-breaking surface.
- Golden before normalized SHA-256:
  `a4aa5d477c80be9ea1cf176b46277723e6c6a1893957bdb57d6949498f08bb4a`.

## Keep gate

Keep only if same-style RCH Criterion confirms a real `1024x1024` win with
Score >= 2.0 and the after golden SHA matches the before SHA.
