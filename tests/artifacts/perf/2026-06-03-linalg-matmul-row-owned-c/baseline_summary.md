# frankenscipy-8l8r1.21 baseline

Pass: 1 - Fresh Baseline And Profile

HEAD: `8fe23444607b2d9893ee881750784263217dfb5e`

RCH worker: `vmi1227854`

Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- matmul --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot
```

Criterion rows:

| row | lower | median | upper |
| --- | ---: | ---: | ---: |
| `matmul/256x256` | `5.1972 ms` | `5.3319 ms` | `5.6347 ms` |
| `matmul/512x512` | `40.391 ms` | `41.678 ms` | `42.911 ms` |
| `matmul/768x768` | `125.98 ms` | `130.94 ms` | `137.17 ms` |
| `matmul/1024x1024` | `336.44 ms` | `348.89 ms` | `363.56 ms` |

Profile-backed target:
The previous RCH linalg reprofile in
`tests/artifacts/perf/2026-06-03-linalg-matmul-macro-block/reprofile_summary.txt`
still ranks large GEMM first, so the trial target remains `matmul/1024x1024`.
