# Sparse SpMM Stack Workspace Trial

- Bead: `frankenscipy-2gmb9`
- Target: `sparse_spmm/2000x2000_d1/2000`
- Profile source: post-epoch-reject RCH sparse reprofile on `ts2` still ranked
  this row first at 13.018 ms median `[12.921, 13.136]`.
- Lever: for `n <= 4096`, trial fixed stack-backed per-worker Gustavson
  workspaces for `acc` and `seen`, falling back to the existing heap `Vec`
  path for larger products.
- Baseline: focused RCH `ts1` median 9.8629 ms `[9.6920, 10.016]`.
- After: focused RCH `ts1` retry median 9.5005 ms `[9.3712, 9.6189]`.
- Confirmation: remote RCH landed on `ts2`, where after was 13.261 ms
  `[12.930, 13.596]`, overlapping and slightly above the `ts2` profile
  baseline. A separate local fallback run is ignored as non-campaign evidence.
- Verdict: rejected. The `ts1` win did not confirm across remote evidence, so
  confidence is below the keep gate and Score is 0.0. Source was backed out.

## Behavior Proof

- Strict golden SHA before:
  `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Strict golden SHA after:
  `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Strict payload diff: empty.
- Isomorphism checked: row ranges, row traversal, B-row encounter order,
  first-seen tracking, reverse emission order, floating-point accumulation
  order, explicit zero elision, sorted flag semantics, metadata, and RNG
  absence were preserved by the trial.

## Next Primitive

The next SpMM pass should skip cleanup/capacity/epoch/stack-workspace
micro-levers. Attack a genuinely different GraphBLAS kernel shape, such as
CSC/column-panel traversal with row-local order replay or a semiring-style
symbolic structure cache that changes the algorithmic work rather than the
workspace allocation.
