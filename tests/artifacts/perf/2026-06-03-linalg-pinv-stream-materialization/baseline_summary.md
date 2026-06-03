# Stream Pinv Materialization Baseline

- Timestamp: 2026-06-03T17:55:18-04:00
- Bead: `frankenscipy-8l8r1.29`
- Worker: `vmi1152480`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- 'baseline_pinv/1000x500' --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`
- Profile-backed target: `baseline_pinv/1000x500`

## Criterion Median

| benchmark | lower | median | upper |
| --- | ---: | ---: | ---: |
| `baseline_pinv/1000x500` | `415.00 ms` | `437.29 ms` | `460.08 ms` |

## Gate

The keep gate is a real `baseline_pinv/1000x500` median win versus `437.29 ms`, with unchanged stable golden-output SHA-256 and Score `>= 2.0`.
