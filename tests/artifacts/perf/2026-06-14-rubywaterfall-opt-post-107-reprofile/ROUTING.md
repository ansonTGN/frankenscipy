# Post-107 fsci-opt reprofile

## Completed keep

`frankenscipy-8l8r1.107` landed as commit `57f30ae2`, adding opt-in exact-gradient nonlinear CG.

## Valid profile evidence

Artifact: `current_cg_group_rch.txt`

- Worker: `vmi1149989`
- Command: `cargo bench -p fsci-opt --bench optimize_bench --locked -- cg --sample-size 10 --warm-up-time 0.5 --measurement-time 1`

Rows:

| Row | p50 |
| --- | --- |
| `cg/rosenbrock/2` | `38.564 us` |
| `cg/rosenbrock_exact_gradient/2` | `45.989 us` |
| `cg/quadratic/2` | `525.79 ns` |
| `cg/rosenbrock/5` | `113.39 us` |
| `cg/rosenbrock_exact_gradient/5` | `99.997 us` |
| `cg/quadratic/5` | `589.63 ns` |
| `cg/rosenbrock/10` | `331.77 us` |
| `cg/rosenbrock_exact_gradient/10` | `240.29 us` |
| `cg/quadratic/10` | `993.51 ns` |

Top valid post-land row remains default `cg/rosenbrock/10`.

## Non-evidence

- `current_bfgs_group_rch.txt`: failed before timing because stale probe-bin targets were not synced to the worker.
- `current_bfgs_group_lib_bench_rch.txt`: remote-required refusal under RCH pressure; no timing row.

## Next route

Filed `frankenscipy-8l8r1.108`: `[perf][opt] post-107 CG derivative-interface primitive`.

Do not repeat accepted-point/materialization, scratch-only, or finite-difference micro-levers. Next source lever should be a deeper derivative primitive: full AD-tape/dual-number objective artifact or fused value-gradient callback that eliminates separate `fun` + `gradient` calls and callback `Vec` allocation in Wolfe/PR+ while preserving the default finite-difference fallback bit-identically.
