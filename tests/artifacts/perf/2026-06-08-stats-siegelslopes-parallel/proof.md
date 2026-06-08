# perf: siegelslopes repeated-median — parallel over anchors, 3.9-14.3x byte-identical

## Lever (ONE)
`siegelslopes` (Siegel repeated-median robust regression) computes, for each
anchor point j, the median of the n-1 slopes to all other points (O(n) each),
then the median of those n per-anchor medians — overall O(n^2), serial. The
anchors are independent (read-only x/y), so compute the per-anchor medians in
parallel into ordered slots and collect the Some values in j order.

## Parity — BYTE-IDENTICAL
- Each anchor's median is computed identically (same slope set, same skip of
  near-zero dx, same `median` select); collecting in j order reproduces the
  sequential push exactly (including the empty-slope-anchor skip). slope and
  intercept bits match the serial build exactly for n=50/500/2000. See
  golden_payload.txt.
- All 4 fsci-stats siegelslopes tests pass; clippy clean.

## Timing — rch remote, 64 cores, --profile release-perf
| n    | serial     | parallel  | speedup |
|------|------------|-----------|---------|
| 2000 | 12.858 ms  | 3.283 ms  | 3.92x   |
| 4000 | 49.731 ms  | 5.183 ms  | 9.59x   |
| 8000 | 201.662 ms | 14.087 ms | 14.3x   |

Compute-bound (slope divisions + select medians), so it scales with cores; the
win grows with n (O(n^2)). Harness: crates/fsci-stats/src/bin/perf_siegelslopes.rs
