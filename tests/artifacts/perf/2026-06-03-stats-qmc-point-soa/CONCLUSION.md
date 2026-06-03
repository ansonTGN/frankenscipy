# QMC 2D Point SoA Negative Result

Bead: `frankenscipy-rh43r`

## Verdict

Abandoned. Splitting the 2D discrepancy point cache into parallel arrays was byte-identical but slower across the focused RCH benchmark group, so production source was restored and no code change is kept.

## Profile-Backed Target

Source profile:

`tests/artifacts/perf/2026-06-03-stats-psd-twiddle-soa/reprofile_stats_after_psd_soa_rch.txt`

Relevant rows:

- `qmc_discrepancy/mixture/512x2`: `243.16 us` median
- `qmc_discrepancy/l2_star/512x2`: `203.31 us` median
- `qmc_discrepancy/wraparound/512x2`: `180.68 us` median
- `qmc_discrepancy/centered/512x2`: `177.19 us` median

## One Lever Tested

Changed the internal 2D discrepancy cache from `Vec<DiscrepancyPoint2>` to parallel coordinate/centered/abs arrays.

No public API, validation, formula, RNG, tie-breaking, or global-state behavior was changed during the trial.

## Behavior Proof

QMC golden before and after the candidate was byte-identical.

SHA256:

`1fb5885cc35367f57b0e818e165a28f87cbb0b9a43fdc7ba4728a6778af44daf`

Preserved surfaces:

- Validation order and error messages.
- Row-major sample order.
- Single-loop row order.
- Pair-loop order: `point_i` outer, `point_j` inner.
- Coordinate order: 0 then 1.
- Formula term order and floating-point operation sequence.
- RNG absence.
- Tie-breaking absence.
- Global-state absence.

## Benchmark Gate

Focused RCH baseline:

- `centered/512x2`: `[180.84 us, 184.85 us, 189.44 us]`
- `mixture/512x2`: `[246.66 us, 253.29 us, 260.84 us]`
- `l2_star/512x2`: `[212.68 us, 219.44 us, 226.32 us]`
- `wraparound/512x2`: `[190.46 us, 197.39 us, 204.12 us]`

Focused RCH after:

- `centered/512x2`: `[336.12 us, 347.02 us, 358.57 us]`
- `mixture/512x2`: `[394.36 us, 401.06 us, 407.86 us]`
- `l2_star/512x2`: `[283.27 us, 285.73 us, 288.51 us]`
- `wraparound/512x2`: `[257.99 us, 261.83 us, 266.24 us]`

Result: regression on all four rows.

Score: `0.0`

## Restoration

Production `crates/fsci-stats/src/qmc.rs` was restored to the pre-trial implementation.

`tests/artifacts/perf/2026-06-03-stats-qmc-point-soa/source_restored_diff.txt` is empty, and `cargo fmt -p fsci-stats --check` passes after restoration.
