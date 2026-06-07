# perf: SphericalVoronoi::new — parallel face detection

## Lever (ONE)
`SphericalVoronoi::new` builds the diagram by brute-force gift-wrapping: for every
triplet `(i,j,k)` (O(n³)) it forms the supporting plane normal and tests all `n`
points for being on one side (O(n), early-break) — overall **O(n⁴)**. Each
triplet's face test is independent.

Detect faces **in parallel** over pair-balanced `i`-ranges (reusing
`pdist_row_bounds`, since early `i` carry more `(j,k)` pairs), collect each range's
accepted faces in order, and flatten in `i`-order — so the accepted triplets land
in exactly the sequential `(i,j,k)` order. The dedup check + push into `vertices`
(which read the growing `vertices`) stay serial in that order.

## Parity — BYTE-IDENTICAL
- Phase 1 produces the same accepted `((i,j,k), vertex)` list, in the same order,
  with the same float values as the sequential loop (same normal/orientation/
  validation arithmetic). Phase 2 does the same dedup check against the same
  growing prefix and pushes in the same order ⇒ identical `vertices` and
  `regions`. The duplicate-generator error is still raised at the first duplicate
  in `(i,j,k)` order.
- A convex-hull-dual rewrite (O(n log n)) would emit vertices in a *different*
  order, breaking the established byte-exact output; parallelizing the brute force
  is therefore the parity-preserving lever.
- `vertices`+`regions` FNV checksum is IDENTICAL between the serial baseline and
  the parallel build for n = 8/16/30 (see `golden_payload.txt`,
  sha256 9f7def15…). nverts = 2n−4 confirms correct hull facets.
- All 4 `fsci-spatial` spherical_voronoi unit tests pass, including
  `spherical_voronoi_rejects_duplicate_points` and
  `spherical_voronoi_rejects_coplanar_great_circle_points` (the preserved error
  paths) and `spherical_voronoi_tetrahedron_has_four_vertices`.

## Timing — rch remote, 64 cores, `--profile release-perf`
Random distinct points on the unit sphere. Same machine, back-to-back (baseline
via stashing the change).

| n   | baseline | parallel | speedup |
|-----|----------|----------|---------|
| 64  | 1.310 ms | 1.375 ms | ~1.0x (serial path, n<128 gate) |
| 200 | 32.511 ms| 5.231 ms | 6.21x   |
| 320 | 135.167 ms| 9.815 ms| 13.77x  |

Score ≥ 2.0 cleared for n ≥ ~150; the win grows with n. A work-based gate keeps
n < 128 on the serial path (no regression: n=64 is unchanged within noise) and
caps workers at n/8 so medium n doesn't pay unamortized spawn overhead.

Harness: `crates/fsci-spatial/src/bin/perf_spherical_voronoi.rs`
Run: `cargo run --profile release-perf -p fsci-spatial --bin perf_spherical_voronoi`

## Notes
- The deeper O(n⁴)→O(n log n) algorithmic replacement (Voronoi as the dual of the
  convex hull / Delaunay on the sphere) is the natural follow-up, but it reorders
  the output and so is a behavior change, not a byte-identical perf lever.
