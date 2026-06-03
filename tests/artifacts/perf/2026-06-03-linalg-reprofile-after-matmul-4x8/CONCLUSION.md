# Conclusion

Context: reprofile after commit `cc42945c`, which kept
`frankenscipy-8l8r1.16` and widened the no-pack dense matmul full tile to
`4 x 8`.

Command:

```text
rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- '(^solve/|^solve_triangular/|^solve_banded/|^inv/|^det/|^lstsq/|^pinv/|^matmul/|^baseline_solve/1000x1000$)' --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot
```

Worker: `vmi1149989`.

Top completed rows by median:

| Rank | Case | Median |
| ---: | --- | ---: |
| 1 | `matmul/1024x1024` | `321.49 ms` |
| 2 | `matmul/768x768` | `123.64 ms` |
| 3 | `lstsq/512x256` | `118.05 ms` |
| 4 | `pinv/512x256` | `113.19 ms` |
| 5 | `baseline_solve/1000x1000` | `93.224 ms` |
| 6 | `matmul/512x512` | `32.956 ms` |
| 7 | `inv/256x256` | `9.4122 ms` |

Conclusion: dense GEMM remains the shifted linalg hotspot after the 4x8
register-tile win. The next profile-backed perf bead should stay on
`matmul/1024x1024`, but must avoid already rejected adjacent levers:
B-flat buffer, A row-ref hoist, store unroll, scalar accumulator, naive row
blocking, and simple B-panel 4x8 packing in the current `Vec<Vec<f64>>` layout.
