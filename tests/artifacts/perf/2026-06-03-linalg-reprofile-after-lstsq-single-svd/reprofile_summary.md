# Linalg Reprofile After Rectangular Lstsq Single-SVD

- Timestamp: 2026-06-03T16:27:48-04:00
- Worker: `vmi1153651`
- Head: `73d0eac3`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- 'matmul|lstsq/512x256|pinv/512x256|baseline_solve/1000x1000|baseline_lstsq/1000x500|baseline_pinv/1000x500' --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`

## Median Ranking

| rank | benchmark | median |
| ---: | --- | ---: |
| 1 | `matmul/1024x1024` | `931.92 ms` |
| 2 | `baseline_pinv/1000x500` | `906.16 ms` |
| 3 | `baseline_lstsq/1000x500` | `879.63 ms` |
| 4 | `matmul/768x768` | `635.95 ms` |
| 5 | `baseline_solve/1000x1000` | `392.99 ms` |
| 6 | `matmul/512x512` | `159.12 ms` |
| 7 | `lstsq/512x256` | `117.82 ms` |
| 8 | `pinv/512x256` | `117.51 ms` |
| 9 | `matmul/256x256` | `11.087 ms` |

## Target Implication

The rectangular `baseline_lstsq/1000x500` row is no longer the top row after single-SVD reuse. The next profile-backed target is large `fsci_linalg::matmul`, specifically `matmul/1024x1024` at `931.92 ms`.

Given the failed row-panel, lane-width, and MR6 register-count trials, the next GEMM primitive should be a deeper data-movement change rather than another accumulator-count calibration. A bounded packed-B panel that preserves monotonic `k` order per output cell is the next candidate because it attacks cache/TLB reuse without changing public API, validation order, output order, RNG, or tie behavior.
