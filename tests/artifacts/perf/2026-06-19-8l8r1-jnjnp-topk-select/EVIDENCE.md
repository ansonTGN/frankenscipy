# frankenscipy-8l8r1.124 jnjnp_zeros top-k select rejection

Agent: cod-a / MistyBirch
Crate: `fsci-special`
Bead: `frankenscipy-8l8r1.124`
Decision: reject and revert. Production remains on the cutoff-driven full-sort
candidate prefix from `frankenscipy-8l8r1.123`.

## Lever

Replace full candidate sorting inside `jnjnp_zeros` with
`select_nth_unstable_by(nt, comparator)` followed by sorting only the retained
prefix. The comparator and output collection were unchanged, so the expected
behavior-preservation proof was bit-identical `(zo, n, m, t)` output against
the original full-sort route.

Mapped route:
- output-sensitive top-k selection,
- monotone frontier certificate retained from the cutoff generator,
- cache and comparison-count reduction before prefix ordering.

## Measurements

Baseline current route on RCH worker `vmi1153651`:

| Workload | Current full-sort mean | Legacy duplicate mean |
| --- | ---: | ---: |
| `jnjnp_zeros(nt=64)` | 1.5407 ms | 207.35 ms |
| `jnjnp_zeros(nt=128)` | 3.5199 ms | 816.72 ms |

Directional candidate Criterion run on RCH worker `hz2`:

| Workload | Candidate top-k mean | Note |
| --- | ---: | --- |
| `jnjnp_zeros(nt=64)` | 715.19 us | cross-worker directional only |
| `jnjnp_zeros(nt=128)` | 1.4243 ms | cross-worker directional only |

Same-binary release probe comparing original full-sort against candidate top-k:

| Worker | Workload | Full-sort mean | Top-k mean | Speedup | Verdict |
| --- | --- | ---: | ---: | ---: | --- |
| `hz2` | `nt=64` | 0.820444 ms | 0.725988 ms | 1.130x | initial win |
| `hz2` | `nt=128` | 1.396257 ms | 1.409626 ms | 0.991x | neutral/loss |
| `hz1` | `nt=64` | 0.928939 ms | 0.911730 ms | 1.019x | neutral |
| `hz1` | `nt=128` | 1.737776 ms | 1.715855 ms | 1.013x | neutral |

The longer probe did not reproduce a meaningful win. The candidate is below the
keep threshold and was reverted.

Local SciPy oracle for the restored final source:

| Workload | Restored Rust mean | SciPy median | Rust/SciPy | Verdict |
| --- | ---: | ---: | ---: | --- |
| `jnjnp_zeros(nt=64)` | 1.5407 ms | 421.59 us | 3.65x slower | residual loss |
| `jnjnp_zeros(nt=128)` | 3.5199 ms | 774.75 us | 4.54x slower | residual loss |

SciPy win/loss/neutral for final source: `0/2/0`.
Candidate internal keep/loss/neutral: `0/0/2`.

## Gates

- PASS: `RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo test -p fsci-special jnjnp_topk_select_perf_probe --release -- --ignored --nocapture` during the probe; it asserted bit-identical output before timing.
- PASS: `RCH_REQUIRE_REMOTE=1 RCH_WORKER=ovh-b CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo test -p fsci-special jnjnp -- --nocapture` during the probe (`3 passed`).
- NOTE: RCH worker images skipped SciPy rows because they could not import `scipy.special`; SciPy timing was run locally with SciPy 1.17.1.

## Retry Condition

Do not retry top-k partitioning, partial sorting, or candidate-order-only work
for `jnjnp_zeros` unless a fresh profile shows candidate sorting is a top
hotspot and the same-binary gate clears at least a 10 percent end-to-end win on
both `nt=64` and `nt=128`. Route deeper to lower-cost Bessel root generation or
SciPy-style compiled recurrence/root-polishing constants instead.
