# frankenscipy-6m75u - Wolfe Trial-Point Scratch Reuse

Agent: cod-b / MistyBirch
Date: 2026-06-19
Decision: REJECT and restore the public Wolfe source path to the parent implementation.

## Lever

`fcbcbaf4` replaced per-probe `x + alpha*d` trial `Vec` construction in the
public Wolfe line-search path with a reusable trial buffer threaded through
`zoom`. The separate gradient-probe path already has scratch reuse and was not
the target of this bead.

## Same-Worker Rust Gate

Environment:

- `CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b`
- `RCH_REQUIRE_REMOTE=1`
- `RCH_WORKER=hz2`
- Actual worker: `hz2`
- Command:
  `rch exec -- cargo bench -p fsci-opt --bench optimize_bench -- rosenbrock_exact_gradient/10 --sample-size 10 --measurement-time 1 --warm-up-time 1`

Baseline source: `fcbcbaf4^` (`1ff11f68`)

| Row | Baseline time | Scratch time | Median ratio | Decision |
| --- | ---: | ---: | ---: | --- |
| `bfgs/rosenbrock_exact_gradient/10` | `[62.235 us 64.178 us 65.430 us]` | `[60.956 us 63.085 us 65.816 us]` | `1.017x` | neutral |
| `cg/rosenbrock_exact_gradient/10` | `[155.95 us 170.54 us 188.05 us]` | `[150.26 us 157.46 us 175.94 us]` | `1.083x` | neutral |

The medians moved in the right direction, but both rows have overlapping
intervals and the repeated current Criterion run reported no performance change
(`p = 0.48` for BFGS, `p = 0.85` for CG). This is not a credible keep gate for
a scratch-only allocation lever, so the source was restored.

## SciPy Comparator

RCH does not remote arbitrary Python in this environment: `rch exec -- python3`
warned `exec called with non-compilation command` and ran on `thinkstation1`.
Direct SSH to `hz2` was refused (`Permission denied`). Therefore this is
cross-host SciPy routing evidence, not the same-worker keep gate.

Environment:

- Host: `thinkstation1`
- SciPy: `1.17.1`
- NumPy: `2.4.3`
- Python workload: `scipy.optimize.minimize` on Rosenbrock-10 from zeros with
  exact gradient, `gtol=1e-5`, `maxiter=1000`, 50 timed runs after warmup.

| Row | Final Rust source time | SciPy median | SciPy p95 | Ratio vs SciPy |
| --- | ---: | ---: | ---: | ---: |
| BFGS exact gradient | `64.178 us` | `5,655,554 ns` | `5,771,679 ns` | `88.1x` faster |
| CG exact gradient | `170.54 us` | `20,630,993 ns` | `20,841,727 ns` | `121.0x` faster |

SciPy win/loss/neutral scorecard for final source: `2/0/0`.

## Correctness / Revert Guard

The final `crates/fsci-opt/src/linesearch.rs` content is text-identical to
`fcbcbaf4^` after the rollback:

`git diff --exit-code fcbcbaf4^ -- crates/fsci-opt/src/linesearch.rs`

Focused tests are recorded in the session and should pass on the restored path:

`rch exec -- cargo test -p fsci-opt wolfe -- --nocapture`

## Negative Evidence

Do not retry public Wolfe trial-point scratch reuse unless a fresh allocation
profile puts public Wolfe trial construction back in the top opt hotspots.
Probe-path scratch reuse is a separate path and is not rejected by this bead.
