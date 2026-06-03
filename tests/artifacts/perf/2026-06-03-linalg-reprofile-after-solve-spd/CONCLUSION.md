# Linalg Reprofile After Solve SPD-Probe Skip

Commit profiled: `b9d432ef1d3626dbcb54bad074853518b8a023eb`

Purpose:
- Re-profile after `frankenscipy-8l8r1.14` because the solve bottleneck shifted after the kept SPD-probe skip.
- Keep the run crate-scoped to `fsci-linalg` and avoid the full workspace.

RCH Criterion results:

| Row | Median |
| --- | ---: |
| `matmul/256x256` | `9.6564 ms` |
| `matmul/512x512` | `91.706 ms` |
| `matmul/768x768` | `690.64 ms` |
| `matmul/1024x1024` | `1.5452 s` |
| `baseline_solve/1000x1000` | `104.21 ms` |

Interpretation:
- `baseline_solve/1000x1000` stayed low after the kept change: `104.21 ms` median, consistent with the gate run at `100.42 ms`.
- Dense `matmul/1024x1024` is again the dominant measured linalg row at `1.5452 s`; `matmul/768x768` is second at `690.64 ms`.
- There is active untracked matmul evidence in `tests/artifacts/perf/2026-06-03-linalg-matmul-a-panel/` and `tests/artifacts/perf/2026-06-03-linalg-matmul-pack/`, so any next GEMM work needs a fresh bead claim and file reservation before touching `crates/fsci-linalg/src/lib.rs`.

Artifacts:
- `criterion_matmul_rch.txt`
- `criterion_baseline_solve_1000_rch.txt`
- `head.txt`
