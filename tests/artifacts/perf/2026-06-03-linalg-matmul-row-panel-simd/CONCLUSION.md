# frankenscipy-8l8r1.23 conclusion

Verdict: REJECTED / SOURCE RESTORED

One lever tried: an 8-row by 8-column SIMD row-panel accumulator in
`matmul_flat_workspace`, intended to reuse each loaded `B[k][j0..j0+8]` vector
across 8 row accumulators.

Behavior proof passed before benchmarking:

- RCH release matmul tests passed.
- Canonical before/after sorted test-line SHA-256:
  `8cf1b2bfc464356d5a115074ca1ddb3f665f5a825a43ecb93d7c43ac265e7288`.
- Canonical golden diff was empty.
- Raw sorted output differed only in a non-behavioral elapsed-time field.

Performance failed the keep gate:

- Baseline `matmul/1024x1024`: `219.58 ms`
- Trial `matmul/1024x1024`: `507.79 ms`
- Score: `0.0` because impact is negative.

Interpretation: the row-panel shape increases live SIMD accumulators enough to
trigger register pressure/spills on the measured worker class. The next
profile-backed primitive should preserve the 4-row accumulator footprint and
attack a different communication path, such as `k`-striped rank-k updates,
autotuned register blocking, or a packing strategy with bounded scratch and
verified per-cell monotonic `k` order.
