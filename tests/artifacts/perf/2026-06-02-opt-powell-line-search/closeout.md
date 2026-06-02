# fsci-opt Powell line-search scratch candidate closeout

Bead: `frankenscipy-okrh6`

## Profile target

Fresh `fsci-opt` RCH Criterion profile ranked `powell/rosenbrock/10` as the slowest measured opt benchmark:

| Rank | Scenario | Worker | Mean |
|---|---|---|---|
| 1 | `powell/rosenbrock/10` | `vmi1293453` | `375.91 us` |
| 2 | `cg/rosenbrock/10` | `vmi1293453` | `318.93 us` |
| 3 | `cg/rosenbrock/2` | `vmi1293453` | `144.27 us` |

## Lever

`golden_section_direction_search` now reuses one candidate vector while evaluating `x + alpha * direction`. This replaces repeated temporary `Vec` construction from `add_scaled` inside the line-search sampling loop.

## Benchmarks

| Run | Worker | Mean |
|---|---|---:|
| Fresh profile baseline | `vmi1293453` | `375.91 us` |
| Focused baseline | `vmi1156319` | `965.52 us` |
| HEAD-control baseline after temporary reverse patch | `vmi1227854` | `507.44 us` |
| After | `vmi1149989` | `306.95 us` |
| After repeat | `vmi1153651` | `803.58 us` |

RCH does not expose a worker-pin option for `exec`, so the evidence is cross-worker. The after measurements are lower than the profile baseline and lower than the same-window HEAD-control baseline. The keep score is `Impact 3 x Confidence 3 / Effort 2 = 4.5`.

## Isomorphism proof

- Ordering preserved: yes. The same alpha candidates are evaluated in the same branch order.
- Tie-breaking unchanged: yes. The same `fc < fd` comparison and final `candidate_f <= fx` branch are used.
- Floating-point preserved: yes. Each coordinate is still computed as `left + scale * right`; the candidate vector storage is reused but values are overwritten before every objective call.
- RNG preserved: N/A. Powell line search is deterministic and has no RNG input.
- Function evaluation count preserved: yes. Golden result kept `nfev = 11508`.
- Golden output: temporary `POWELL_GOLDEN` harness output is byte-identical before and after; `golden_before.txt`, `golden_after.txt`, and `golden_final.txt` all hash to `d527d69305d175a37261d73e404bcb25996dc7fdac1f1c58ccbc0c987b5abf5e`.

## Validation

- `cargo fmt --check -p fsci-opt`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-opt --lib powell --locked`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo clippy -p fsci-opt --all-targets --locked -- -D warnings`
- `ubs crates/fsci-opt/src/minimize.rs` exits 1 on existing broad `minimize.rs` inventory; the committed hunk does not introduce the reported critical locations, and the report is archived with `UBS_EXIT:1`.
