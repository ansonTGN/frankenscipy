# Rectangular Lstsq Single-SVD After Benchmark

- Timestamp: 2026-06-03T16:08:01-04:00
- Bead: `frankenscipy-8l8r1.25`
- Worker: `vmi1153651`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- baseline_lstsq/1000x500 --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`
- Golden SHA: `bdf491ce0154bec5825fcdd3d68a23ec5941bbe4308584ceb2440c136a8722b6`

## Median Delta

| benchmark | baseline median | after median | delta |
| --- | ---: | ---: | ---: |
| `baseline_lstsq/1000x500` | `1.0625 s` | `852.17 ms` | `1.25x faster` |

## Decision

Kept. The stable golden SHA stayed unchanged and the focused profile-backed row improved by `210.33 ms` median.

Score: `6.0 = impact 4 * confidence 3 / effort 2`.
