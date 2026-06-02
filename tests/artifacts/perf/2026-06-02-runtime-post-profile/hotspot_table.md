# fsci-runtime post-calibrator profile

Scenario:
- Crate-scoped RCH Criterion profile after `frankenscipy-eehzf`.
- Command: `cargo bench -p fsci-runtime --bench runtime_bench --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`.
- Artifact: `post_runtime_bench_rch.txt`.
- Worker: `vmi1156319`.

Ranked hotspots:

| Rank | Benchmark | Mean | Range | Category | Evidence |
|------|-----------|------|-------|----------|----------|
| 1 | `calibrator_observe_200` | 1.4999 us | 1.4411-1.5581 us | allocation/queue update | `post_runtime_bench_rch.txt` |
| 2 | `policy_decide_hardened` | 784.19 ns | 775.82-797.57 ns | CPU/allocation | `post_runtime_bench_rch.txt` |
| 3 | `policy_decide_strict` | 753.95 ns | 742.35-767.45 ns | CPU/allocation | `post_runtime_bench_rch.txt` |
| 4 | `solver_select_ModerateCondition` | 51.905 ns | 50.901-53.319 ns | CPU | `post_runtime_bench_rch.txt` |
| 5 | `solver_select_IllConditioned` | 44.441 ns | 42.279-46.281 ns | CPU | `post_runtime_bench_rch.txt` |

Hypothesis ledger:
- Calibrator queue growth: supports but already addressed by `frankenscipy-eehzf`; the post-profile still shows the row as largest on a different worker, so it should not be re-opened without a same-worker focused repro.
- Policy decision fixed-size loss math: supports; `policy_decide_*` are the next distinct runtime rows, and `expected_loss` runs on every finite decision using fixed 3x3 matrices through iterator/zip/sum.
- Policy evidence FIFO shifting: rejects; `PolicyEvidenceLedger` already uses `VecDeque` and `pop_front`.
- Solver selection: rejects for next pass; `solver_select_*` rows are under 52 ns and far below policy decision rows.

Next profile-backed bead:
- Target `PolicyController::decide` finite path in `crates/fsci-runtime/src/policy.rs`.
- Candidate one lever: expand `expected_loss` fixed 3-element dot products into explicit left-to-right arithmetic while preserving loss matrix values, action ordering, tie-breaking, and floating-point operation order.
