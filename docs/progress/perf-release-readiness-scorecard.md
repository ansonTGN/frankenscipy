# Performance Release-Readiness Scorecard

## 2026-06-19 - fsci-linalg wide lstsq row-streaming gauntlet

- Agent: cod-a / MistyBirch
- Bead: `frankenscipy-u0ucw`
- Decision: REVERT row-streamed wide `lstsq`; KEEP current materialized
  normal-equation route. Score for this sub-cluster: 4/5.
- Artifact: `tests/artifacts/perf/2026-06-19-u0ucw-wide-lstsq-gauntlet/`

| Gate | Result | Notes |
| --- | --- | --- |
| Rust per-crate compile | PASS | `rch exec -- env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a cargo check -p fsci-linalg --benches` |
| Criterion focused bench | PASS | same-worker row-streaming vs materialized A/B on `vmi1227854`; local SciPy row for original-SciPy ratio |
| SciPy head-to-head oracle | PASS | SciPy 1.17.1 / NumPy 2.4.3, `scipy.linalg.lstsq(check_finite=False)` |
| Targeted linalg tests | PASS | `wide_pinv` filtered tests passed, preserving the surviving row-major wide `pinv` helpers |
| Release route probe | PASS | ignored release probe reported `lstsq_max_abs_diff=3.38840067115597776e-13` |
| Changed bench formatting | PASS | `rustfmt --edition 2024 --check crates/fsci-linalg/benches/linalg_bench.rs` |
| Direct `src/lib.rs` formatting | BLOCKED | file-wide pre-existing rustfmt drift outside this revert; broad-formatting would create unrelated churn in the shared checkout |
| Clippy `-D warnings` | BLOCKED | pre-existing `src/lib.rs` lints plus concurrent `src/cossin.rs` excessive-precision lints; no row-streaming revert-specific issue identified |

| Workload / route | Mean | Ratio | Verdict |
| --- | ---: | ---: | --- |
| Rust row-streamed wide `lstsq`, 500x1000 | 139.965 ms | 0.966x vs materialized | loss, reverted |
| Rust materialized wide `lstsq`, 500x1000 | 135.206 ms | 1.035x vs row-streamed | keep old route |
| Rust current materialized wide `lstsq`, 500x1000 | 109.370 ms | 11.46x vs SciPy | keep |
| SciPy `scipy.linalg.lstsq`, 500x1000 | 1.253347 s | 1.00x oracle | reference |

Readiness notes:

- The negative result is specific to replacing the wide `lstsq` materialized
  transpose products with row streaming. It does not invalidate the public wide
  normal-equation route, which remains faster than SciPy on the measured
  workload.
- `rch exec -- env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a cargo clippy -p fsci-linalg --benches -- -D warnings`
  currently fails on existing lints unrelated to the measured revert, including
  `needless_range_loop` / `needless_borrow` rows in `src/lib.rs` and
  excessive-precision rows in concurrently modified `src/cossin.rs`.
- Future retries need a fresh allocation/cache profile and a same-worker >10%
  win over the materialized route before this formulation should be reconsidered.

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
