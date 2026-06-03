# frankenscipy-8l8r1.23 baseline

Pass: 1 - Fresh Baseline And Profile

HEAD: `54600c5f2754859ac85b964a91887e10ce2a5745`

RCH worker: `vmi1149989`

Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- matmul --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot
```

Criterion rows:

| row | lower | median | upper |
| --- | ---: | ---: | ---: |
| `matmul/256x256` | `4.8446 ms` | `4.9700 ms` | `5.0636 ms` |
| `matmul/512x512` | `38.156 ms` | `39.810 ms` | `41.387 ms` |
| `matmul/768x768` | `136.45 ms` | `142.48 ms` | `149.55 ms` |
| `matmul/1024x1024` | `211.28 ms` | `219.58 ms` | `227.49 ms` |

Profile-backed target:
The previous RCH linalg reprofile in
`tests/artifacts/perf/2026-06-03-linalg-matmul-simd-lanes/reprofile_summary.txt`
still ranks `matmul/1024x1024` first at median `216.09 ms`. This pass uses
the fresh `219.58 ms` focused median as the keep-gate baseline.
