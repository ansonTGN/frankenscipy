# frankenscipy-4czwo alien recommendation

## Intake

- Project: FrankenSciPy, `fsci-interpolate`
- Workload: `griddata(&points, &values, &queries, GriddataMethod::Linear)` on 576 scattered 2D points and 1024 queries
- Measured target: `scattered_2d/griddata_linear/576x1024`
- Focused baseline: `14.471 ms 14.704 ms 14.986 ms` on `vmi1156319`
- Post-vd5m2 broad profile: `14.175 ms 14.429 ms 14.714 ms` on `vmi1153651`
- Correctness constraints: preserve query order, output bits, NaN outside hull, error ordering, simplex tie behavior, and RNG non-surface

## Graveyard match

Symptom route: compute-side algorithmic/cache hotspot. Canonical router points cache thrash and algorithmic lookup pain toward §7.x data structures and §8.x vectorized/cache-sized processing. Relevant entries:

- §7.2 Cache-Oblivious Data Structures: profile probe paths, cache misses, layout cost, and prove semantic parity across structural transitions.
- §7.7 Swiss Tables: contiguous metadata and payload separation as a cache-local lookup pattern; use only after probe paths are confirmed hot.
- §8.2 Vectorized Execution: batch query processing in cache-sized groups, with row-isomorphism proof against scalar/materialized execution.

## Recommendation card

Name: Query-local Delaunay simplex hint walk with scalar fallback

Primitive: cache/layout-aware spatial lookup. Reuse the previous successful simplex while evaluating the input query stream, walk across Delaunay neighbor edges when barycentric coordinates show the query crossed an edge, and fall back to the existing linear scan when the walk fails or exceeds a small step budget.

Why this fits: `Delaunay2D::find_simplex` currently linearly scans every simplex for every query. The benchmark query stream is deterministic and locality-friendly; consecutive queries often lie near the previous query in the triangulation. The existing neighbor table is already computed, so the first lever can reuse existing topology without changing triangulation construction or public API shape.

EV:

- Impact: 4.0
- Confidence: 3.5
- Reuse: 3.0
- Effort: 2.0
- Adoption friction: 1.2
- EV: 17.5

Fallback trigger:

- If the hinted walk reaches an invalid neighbor, exceeds the step budget, encounters non-finite barycentric coordinates, or fails to find an enclosing simplex, immediately run the existing linear scan in the original simplex order.

Proof obligations:

- Golden SHA before/after must match exactly for the 576x1024 griddata-linear fixture.
- The fallback linear scan must preserve original simplex-order tie-breaking.
- A successful walk may only return a simplex whose barycentric coordinates satisfy the same containment predicate as the original path.
- Query output order remains input order.
- No RNG is introduced.

Risk gate:

- Primary risk: performance unpredictability from pathologically ordered query streams.
- Countermeasure: bounded walk steps plus exact original linear fallback.
- Rollback: revert the one optimization commit.
