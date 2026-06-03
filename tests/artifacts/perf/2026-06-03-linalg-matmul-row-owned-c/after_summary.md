# frankenscipy-8l8r1.21 after benchmark

Pass: 5 - RCH Gate

RCH worker: `vmi1153651`

Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- matmul --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot
```

Criterion rows:

| row | lower | median | upper |
| --- | ---: | ---: | ---: |
| `matmul/256x256` | `6.4989 ms` | `7.0916 ms` | `8.3126 ms` |
| `matmul/512x512` | `57.414 ms` | `62.075 ms` | `67.338 ms` |
| `matmul/768x768` | `140.10 ms` | `145.65 ms` | `157.05 ms` |
| `matmul/1024x1024` | `355.84 ms` | `417.98 ms` | `481.61 ms` |

Decision:
Rejected. The saved after run does not show a keepable win against the pass
baseline `348.89 ms` median for `matmul/1024x1024`.
