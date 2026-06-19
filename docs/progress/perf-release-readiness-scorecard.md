# Performance Release-Readiness Scorecard

## 2026-06-19 - fsci-opt least_squares scratch cluster

- Agent: cod-b / MistyBirch
- Commit under verification: `41bf34a4`
- Beads: `frankenscipy-szky7`, `frankenscipy-y1mzk`
- Decision: KEEP, no revert. Score for this cluster: 4/5.
- Artifact: `tests/artifacts/perf/2026-06-19-opt-least-squares-gauntlet/least_squares_vs_scipy.json`

| Gate | Result | Notes |
| --- | --- | --- |
| Rust per-crate compile | PASS | `rch exec -- env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b cargo check -p fsci-opt` |
| Criterion focused bench | PASS | `least_squares` group only, remote worker `vmi1227854` |
| SciPy head-to-head oracle | PASS | SciPy 1.17.1 / NumPy 2.4.3, warmed single-process `method="lm"` |
| Targeted metamorphic tests | PASS | 2 least-squares rows passed via `--test metamorphic_tests mr_least_squares` |
| Release diff probes | PASS | `diff_lsq` and `diff_leastsq` release binaries converged |
| Broad fsci-opt unit gate | BLOCKED | Pre-existing `src/lib.rs` test imports fail before the target tests run; follow-up `frankenscipy-uxs8k` |

| Workload | Rust p50 (us) | SciPy p50 (us) | SciPy p99 (us) | SciPy/Rust p50 | Verdict |
| --- | ---: | ---: | ---: | ---: | --- |
| `least_squares/rosenbrock_residual` | 2.558 | 1404.547 | 2645.985 | 549.08x | win |
| `least_squares/exp_curve_64` | 16.932 | 753.120 | 1452.558 | 44.48x | win |
| `least_squares/exp_linear_curve_128` | 49.724 | 893.946 | 1414.085 | 17.98x | win |

Readiness notes:

- The measured workloads include Python callback overhead on the SciPy side.
  That is intentional for the original-SciPy realistic usage path targeted by
  this gauntlet; it is not evidence about lower-level C-only kernels.
- No neutral or loss rows were observed for this cluster, so the scratch-reuse
  optimization remains in tree.
- The remaining release risk is test-harness hygiene, not this optimization:
  broad `fsci-opt` unit-test compilation needs the missing helper imports fixed
  before this lane can claim a full crate test pass.
