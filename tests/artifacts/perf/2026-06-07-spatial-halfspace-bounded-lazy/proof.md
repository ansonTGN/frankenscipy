# perf: HalfspaceIntersection::from_nd — lazy short-circuiting boundedness test

## Lever (ONE)
`halfspace_region_is_bounded_nd` decided boundedness by enumerating every
`(ndim+1)`-subset of the `m` normals and checking whether the origin lies strictly
inside that subset's positive cone, via:

```rust
let mut combos = Vec::new();
combinations_recursive(normals.len(), ndim + 1, 0, &mut Vec::new(), &mut combos);
combos.into_iter().any(|combo| { ... solve_linear_system ... })
```

`combinations_recursive` **materializes all C(m, ndim+1) subsets** (each a heap
`Vec<usize>`) before `.any` runs — and `C(m, ndim+1)` is `m/(ndim+1)×` larger than
the C(m, ndim) used by the vertex enumeration (e.g. C(120,4)=8.2M tiny Vecs for
m=120, ndim=3). That allocation dominated `from_nd`.

Replace with `combinations_any(n, k, pred)`, which streams the same subsets in the
**same lexicographic order** and short-circuits on the first witness — no full
materialization. For bounded regions a witness is found almost immediately;
unbounded regions still scan every subset but no longer pay the up-front
O(C(m,ndim+1)) allocation.

## Parity — BYTE-IDENTICAL
- `combinations_any` mirrors `combinations_recursive`'s loop/guards exactly (same
  order, same subsets), and `pred` is the unchanged per-subset test, so the
  boolean returned is identical to `combinations_recursive(..).into_iter().any(..)`.
- `is_bounded` is the only thing affected, and only its *speed*; the vertex
  enumeration / dual fields are untouched. The full `intersections` +
  `dual_facets` + `dual_vertices` FNV checksum is IDENTICAL between the serial
  baseline and the lazy build for (m,ndim) ∈ {(10,3),(20,3),(16,4)}.
  See `golden_payload.txt`, sha256 3801b285…
- All 7 `fsci-spatial` halfspace_intersection unit tests pass, including the
  bounded 3D tetrahedron, the **unbounded**-region cases (`.any` scans all,
  returns false — the worst case), and the invalid-input rejections.

## Timing — rch remote, 64 cores, `--profile release-perf`
`HalfspaceIntersection::from_nd` on m random outward halfspaces (bounded polytope
≈ unit ball). Same machine, back-to-back (baseline via stashing the change).

| m, ndim    | baseline  | lazy bounded | speedup |
|------------|-----------|--------------|---------|
| 40,  3D    | 5.878 ms  | 2.757 ms     | 2.13x   |
| 120, 3D    | 419.850 ms| 82.208 ms    | 5.11x   |
| 60,  4D    | 423.987 ms| 186.931 ms   | 2.27x   |

Score ≥ 2.0 cleared across the board.

Harness: `crates/fsci-spatial/src/bin/perf_halfspace_nd.rs`
Run: `cargo run --profile release-perf -p fsci-spatial --bin perf_halfspace_nd`

## Notes
- After this fix the residual cost is the vertex enumeration's own
  `combinations_recursive(m, ndim)` materialize + per-combo solve (≈80ms at
  m=120,3D; ≈187ms at m=60,4D). Parallelizing that independent solve+feasibility
  pass is a clean follow-up lever (filed separately) — it gave ~1.3–1.4× alone,
  so it is NOT bundled here.
