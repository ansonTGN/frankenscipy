# QMC L2-Star Coordinate-Only Cache Primitive Selection

Bead: `frankenscipy-ilfbq`

Target: `qmc_discrepancy/l2_star/512x2`

Profile evidence:
- QMC point-SoA negative result kept `l2_star/512x2` visible at `219.44 us` median in `tests/artifacts/perf/2026-06-03-stats-qmc-point-soa/CONCLUSION.md`.
- Fresh focused RCH baseline for this bead: `[224.20 us, 228.55 us, 233.04 us]` in `baseline_l2_star_512x2_rch.txt`.

Rejected prior levers:
- Broad 2D invariant cache was already shipped.
- Direct `delta.powi(2)` replacement was rejected.
- Broad SoA point-cache layout was rejected.

Candidate primitive:
- Change only `l2_star_discrepancy_2d` to build a coordinate-only cache for `(x0, x1)` instead of using the full `DiscrepancyPoint2` cache with unused centered/abs fields.
- The alien-graveyard match is a narrower data-layout primitive: keep hot numeric streams compact and avoid unused field materialization in the focused kernel.

Behavior contract:
- Public validation order and error surfaces remain unchanged.
- Row order remains ascending input row order.
- Pair loop order remains `i` outer, `j` inner, both ascending.
- Coordinate order remains coordinate 0 then coordinate 1.
- Formula term order and final `sqrt` remain unchanged.
- Golden QMC output must remain byte-identical: `1fb5885cc35367f57b0e818e165a28f87cbb0b9a43fdc7ba4728a6778af44daf`.
- No RNG, tie-breaking, or global state exists in this routine.

Score target:
- Impact: 2.0. The target is visible but this is a small post-cache lever.
- Confidence: 3.0. The change removes unnecessary per-row work while preserving arithmetic order.
- Effort: 1.5. One private helper plus a single call-site.
- Score: `4.0 = 2.0 * 3.0 / 1.5`.

Decision:
- Trial this l2-star-only lever.
- Reject and restore source if golden changes, validation fails, or focused RCH timing lacks a real win.
