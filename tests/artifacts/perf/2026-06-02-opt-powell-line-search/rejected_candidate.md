# fsci-opt Powell line-search scratch reuse: superseded rejection note

Bead: frankenscipy-okrh6
Date: 2026-06-02
Agent: OliveSnow

## Profile-backed target

Fresh RCH Criterion profiling for `fsci-opt` ranked `powell/rosenbrock/10` as
the highest-latency row in the broad optimize bench matrix:

- `powell/rosenbrock/10`: `[359.18 us, 375.91 us, 401.09 us]`, worker vmi1293453
- `cg/rosenbrock/10`: `[309.30 us, 318.93 us, 328.93 us]`
- `powell/rosenbrock/5`: `[82.172 us, 85.335 us, 89.392 us]`
- `bfgs/rosenbrock/10`: `[74.371 us, 75.792 us, 77.263 us]`

The focused baseline artifact later measured `powell/rosenbrock/10` at
`[874.58 us, 965.52 us, 1.1121 ms]` on worker vmi1156319, but that result was
treated as a cross-worker outlier relative to the broad-profile row.

A scratch HEAD control run also measured the target at
`[441.97 us, 507.44 us, 608.62 us]` on worker vmi1227854.

## Candidate

The one-lever candidate reused one scratch `candidate_x` vector inside
`golden_section_direction_search` instead of allocating a new vector through
`add_scaled` for each one-dimensional objective sample.

Opportunity score before measurement:

| Hotspot | Impact | Confidence | Effort | Score |
|---------|--------|------------|--------|-------|
| Powell line-search candidate vector allocation | 2 | 3 | 2 | 3.0 |

## Behavior proof

Golden output was captured by the temporary `powell_rosenbrock10_golden_snapshot`
test before and after the candidate:

- before sha256: `d527d69305d175a37261d73e404bcb25996dc7fdac1f1c58ccbc0c987b5abf5e`
- after sha256: `d527d69305d175a37261d73e404bcb25996dc7fdac1f1c58ccbc0c987b5abf5e`

Isomorphism:

- Ordering preserved: alpha samples and objective calls stayed in the same order.
- Tie-breaking unchanged: the same `fb > fx`, `f_next > fb`, `fc < fd`, and
  `candidate_f <= fx` comparisons were used.
- Floating-point preserved: each candidate component remained
  `left + scale * right`; no summation, coefficient, tolerance, or comparison
  formula changed.
- RNG: N/A.

## RCH measurements

Post-candidate focused benchmark:

- `powell/rosenbrock/10`: `[297.10 us, 306.95 us, 315.05 us]`, worker vmi1149989
- repeat after-run: `[767.98 us, 803.58 us, 836.45 us]`, worker vmi1153651

The first pass rejected this candidate because the early post-candidate run was
too close to the broad profile baseline. A later same-window HEAD-control run was
captured by temporarily reversing the source hunk and measuring
`powell/rosenbrock/10` at `[441.97 us, 507.44 us, 608.62 us]` on vmi1227854.
The restored candidate then measured `[297.10 us, 306.95 us, 315.05 us]` on
vmi1149989, and the repeat run stayed below the original focused baseline of
`[874.58 us, 965.52 us, 1.1121 ms]` on vmi1156319.

## Decision

Superseded. The tracked filename is retained to avoid deleting or renaming an
artifact, but the candidate was restored and kept after the HEAD-control evidence
above. Final closeout is in `closeout.md`.
