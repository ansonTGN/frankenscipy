# Linalg Reprofile After Direct Lstsq SVD Solve

- Timestamp: 2026-06-03T17:51:24-04:00
- Commit: `9fd0ea65`
- Worker: `ts2`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- 'matmul|lstsq/512x256|pinv/512x256|baseline_solve/1000x1000|baseline_lstsq/1000x500|baseline_pinv/1000x500' --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`

## Median Ranking

| rank | benchmark | median |
| ---: | --- | ---: |
| 1 | `baseline_pinv/1000x500` | `499.29 ms` |
| 2 | `baseline_lstsq/1000x500` | `454.31 ms` |
| 3 | `matmul/1024x1024` | `327.08 ms` |
| 4 | `matmul/768x768` | `215.10 ms` |
| 5 | `baseline_solve/1000x1000` | `160.57 ms` |
| 6 | `pinv/512x256` | `134.57 ms` |
| 7 | `lstsq/512x256` | `128.20 ms` |
| 8 | `matmul/512x512` | `60.910 ms` |
| 9 | `matmul/256x256` | `7.5011 ms` |

## Next Target

`baseline_pinv/1000x500` is the next profile-backed target. The rejected direct diagonal-scaling trial showed that tuning the same diagonal operator is the wrong direction; the next primitive should attack full pseudoinverse materialization with a different data-movement model, such as streaming outer-product accumulation into the returned row-major layout.
