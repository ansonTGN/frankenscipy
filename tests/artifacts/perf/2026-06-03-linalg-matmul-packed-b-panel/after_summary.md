# Packed-B Panel GEMM After Benchmark

- Timestamp: 2026-06-03T16:57:20-04:00
- Bead: `frankenscipy-8l8r1.26`
- Lever: pre-pack complete 8-column B panels for the large flat-workspace GEMM path.
- Baseline keep gate: `matmul/1024x1024` median `931.06 ms`.

## Criterion Medians

| run | worker | benchmark | lower | median | upper | speedup vs baseline |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| full sweep | `vmi1293453` | `matmul/256x256` | `5.4831 ms` | `5.8491 ms` | `6.0420 ms` | `2.30x` |
| full sweep | `vmi1293453` | `matmul/512x512` | `42.568 ms` | `43.993 ms` | `45.472 ms` | `2.91x` |
| full sweep | `vmi1293453` | `matmul/768x768` | `141.02 ms` | `146.26 ms` | `150.96 ms` | `5.47x` |
| full sweep | `vmi1293453` | `matmul/1024x1024` | `219.17 ms` | `224.80 ms` | `230.50 ms` | `4.14x` |
| confirm | `vmi1149989` | `matmul/1024x1024` | `175.25 ms` | `202.96 ms` | `237.93 ms` | `4.59x` |

## Gate

The packed-B panel lever clears the `matmul/1024x1024` keep gate on both the full sweep and the confirm-only run. Behavior proof stayed bit-identical under the stable golden SHA `48613a728da5350067a920bf0e68b27fc11efd4537046584e2b28a25e75dd771`.
