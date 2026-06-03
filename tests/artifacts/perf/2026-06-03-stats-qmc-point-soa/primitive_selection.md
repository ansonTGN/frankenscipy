# QMC 2D Point SoA Primitive Selection

## Profile Target

Source: `tests/artifacts/perf/2026-06-03-stats-psd-twiddle-soa/reprofile_stats_after_psd_soa_rch.txt`

Relevant post-change rows:

- `qmc_discrepancy/mixture/512x2`: `243.16 us` median
- `qmc_discrepancy/l2_star/512x2`: `203.31 us` median
- `qmc_discrepancy/wraparound/512x2`: `180.68 us` median
- `qmc_discrepancy/centered/512x2`: `177.19 us` median

Focused current-code baseline:

- `centered/512x2`: `[180.84 us, 184.85 us, 189.44 us]`
- `mixture/512x2`: `[246.66 us, 253.29 us, 260.84 us]`
- `l2_star/512x2`: `[212.68 us, 219.44 us, 226.32 us]`
- `wraparound/512x2`: `[190.46 us, 197.39 us, 204.12 us]`

## Prior Results

- Kept: 2D invariant cache materializing row coordinates, centered offsets, and absolute centered offsets once.
- Rejected: direct `delta.powi(2)` replacement in the 2D mixture pair loop; it was byte-identical but slower under focused RCH.

## Selected Primitive

Split the internal cached point data from an array of `DiscrepancyPoint2` structs into parallel arrays:

- `x0`
- `x1`
- `centered0`
- `centered1`
- `abs0`
- `abs1`

This applies the alien-graveyard SoA/cache-locality guidance: keep hot columns contiguous when loops consume the same field across many rows.

## Behavior Contract

The trial must preserve:

- Validation order and error messages.
- Row-major sample order.
- Single-loop row order.
- Pair-loop order: `point_i` outer, `point_j` inner.
- Coordinate order: 0 then 1.
- Formula term order and floating-point operation sequence.
- RNG absence.
- Tie-breaking absence.
- Global-state absence.

Golden before SHA:

`1fb5885cc35367f57b0e818e165a28f87cbb0b9a43fdc7ba4728a6778af44daf`

## Score Gate

Target score: `3.0 = impact 2 * confidence 3 / effort 2`

Reject if golden output changes, validation fails, or focused RCH does not show a real win.
