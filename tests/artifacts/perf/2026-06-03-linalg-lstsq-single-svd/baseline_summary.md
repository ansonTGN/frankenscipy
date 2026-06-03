# Rectangular Lstsq Single-SVD Baseline

- Timestamp: 2026-06-03T15:54:54-04:00
- Bead: `frankenscipy-8l8r1.25`
- Worker: `vmi1153651`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- baseline_lstsq/1000x500 --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`
- Profile-backed target: `baseline_lstsq/1000x500`

## Criterion Median

| benchmark | lower | median | upper |
| --- | ---: | ---: | ---: |
| `baseline_lstsq/1000x500` | `1.0397 s` | `1.0625 s` | `1.0871 s` |

## Gate

The keep gate is a real `baseline_lstsq/1000x500` median win versus `1.0625 s`, with unchanged stable golden-output SHA-256 and Score `>= 2.0`.
