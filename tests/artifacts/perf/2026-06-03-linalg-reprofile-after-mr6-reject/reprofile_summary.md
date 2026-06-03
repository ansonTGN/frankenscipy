# Linalg Reprofile After MR6 Rejection

- Timestamp: 2026-06-03T15:48:43-04:00
- Worker: `vmi1153651`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- 'matmul|lstsq/512x256|pinv/512x256|baseline_solve/1000x1000|baseline_lstsq/1000x500|baseline_pinv/1000x500' --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`

## Ranking By Median

| rank | benchmark | median |
| ---: | --- | ---: |
| 1 | `baseline_lstsq/1000x500` | `1.1552 s` |
| 2 | `matmul/768x768` | `1.0011 s` |
| 3 | `baseline_pinv/1000x500` | `880.08 ms` |
| 4 | `matmul/1024x1024` | `863.03 ms` |
| 5 | `baseline_solve/1000x1000` | `405.44 ms` |
| 6 | `lstsq/512x256` | `158.71 ms` |
| 7 | `matmul/512x512` | `129.21 ms` |
| 8 | `pinv/512x256` | `108.81 ms` |
| 9 | `matmul/256x256` | `11.413 ms` |

## Next Target

The top profile-backed target is rectangular least squares. Code inspection shows `lstsq_with_casp` performs a condition-only SVD and then a full SVD on the same rectangular matrix, so the next child is `frankenscipy-8l8r1.25`: single full-SVD reuse for rectangular `lstsq`.
