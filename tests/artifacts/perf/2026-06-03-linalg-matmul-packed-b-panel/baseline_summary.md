# Packed-B Panel GEMM Baseline

- Timestamp: 2026-06-03T16:34:14-04:00
- Bead: `frankenscipy-8l8r1.26`
- Worker: `vmi1153651`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- matmul --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`
- Profile-backed target: `matmul/1024x1024`

## Criterion Median

| benchmark | lower | median | upper |
| --- | ---: | ---: | ---: |
| `matmul/256x256` | `12.970 ms` | `13.448 ms` | `14.090 ms` |
| `matmul/512x512` | `117.78 ms` | `128.07 ms` | `138.20 ms` |
| `matmul/768x768` | `739.42 ms` | `800.45 ms` | `858.45 ms` |
| `matmul/1024x1024` | `881.98 ms` | `931.06 ms` | `978.02 ms` |

## Gate

The keep gate is a real `matmul/1024x1024` median win versus `931.06 ms`, with unchanged stable golden-output SHA-256 and Score `>= 2.0`.
