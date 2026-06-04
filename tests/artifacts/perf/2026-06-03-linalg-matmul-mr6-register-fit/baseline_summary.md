# frankenscipy-8l8r1.24 baseline

Command:

```text
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- matmul --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot
```

Remote worker: `vmi1153651`

Current HEAD: `c8efcf0a0a1d5872a511d72267d94f0236c06adc`

Criterion medians:

- `matmul/256x256`: `11.614 ms`
- `matmul/512x512`: `140.36 ms`
- `matmul/768x768`: `845.49 ms`
- `matmul/1024x1024`: `906.45 ms`

Profile-backed target: `matmul/1024x1024` remains the production gate and the
largest focused `matmul` row in this baseline. The prior accepted reprofile
`tests/artifacts/perf/2026-06-03-linalg-matmul-simd-lanes/reprofile_summary.txt`
also ranked `matmul/1024x1024` first at `216.09 ms`.

Keep gate: the MR6 trial must show a real `matmul/1024x1024` win against
`906.45 ms` on comparable RCH evidence and Score must be at least `2.0`.
