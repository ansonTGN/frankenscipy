# frankenscipy-8l8r1.23 after benchmark

Command:

```text
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- matmul --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot
```

Remote worker: `vmi1156319`

## Baseline medians

- `matmul/256x256`: `4.9700 ms`
- `matmul/512x512`: `39.810 ms`
- `matmul/768x768`: `142.48 ms`
- `matmul/1024x1024`: `219.58 ms`

## Trial medians

- `matmul/256x256`: `10.915 ms`
- `matmul/512x512`: `98.081 ms`
- `matmul/768x768`: `568.25 ms`
- `matmul/1024x1024`: `507.79 ms`

## Decision

Rejected. The production gate row regressed from `219.58 ms` to `507.79 ms`
(`0.43x` of baseline throughput), below the Score `2.0` keep gate.

The source edit was manually restored. `git diff -- crates/fsci-linalg/src/lib.rs`
was empty after restore, and `cargo fmt -p fsci-linalg --check` passed.
