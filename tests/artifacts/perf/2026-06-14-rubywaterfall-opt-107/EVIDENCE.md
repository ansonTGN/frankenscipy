# frankenscipy-8l8r1.107 - differentiated CG exact-gradient route

## Target

Profile-backed hotspot: `cg/rosenbrock/10` in `fsci-opt` nonlinear CG. The stage profile from `frankenscipy-8l8r1.106` showed the row dominated by finite-difference plus Wolfe line-search objective-call budget:

- `nfev=18359`
- `actual_calls=11679`
- `finite_diff_budget_calls=7380`
- `line_search_budget_calls=10978`

Selected alien primitive: opt-in differentiated objective route for CG, via `MinimizeOptions::gradient`, preserving the default finite-difference path exactly when no callback is provided.

## Baseline

Focused RCH Criterion baseline before this route:

- Artifact: `baseline_cg_rosenbrock10_vmi1167313_rch.txt`
- Worker: `vmi1167313`
- Row: `cg/rosenbrock/10`
- Time: `[709.53 us, 771.31 us, 845.31 us]`

Same-worker comparison baseline for the keep gate:

- Artifact: `baseline_cg_rosenbrock10_vmi1149989_match_rch.txt`
- Worker: `vmi1149989`
- Row: `cg/rosenbrock/10`
- Time: `[199.62 us, 204.99 us, 210.20 us]`

## Change

One lever:

- Add `GradientFunc = fn(&[f64]) -> Vec<f64>` and `MinimizeOptions::gradient`.
- Teach `cg_pr_plus` to use the callback for initial, Wolfe-probe, and accepted-point gradients when present.
- Keep the existing finite-difference branch as the fallback for `gradient: None`.
- Add Criterion row `cg/rosenbrock_exact_gradient/10`.

No unsafe code, no C BLAS/LAPACK/XLA linkage, no RNG, no algorithm-selection side channel.

## Isomorphism proof

Default behavior (`gradient: None`) is unchanged:

- RCH artifact: `golden_cg_current_rch.txt`
- Extracted payload: `golden_cg_current_payload.txt`
- SHA-256: `92830169fd8409f2c5b7566f2378c0c79b93135ae3bda05e6d7c0a4f655e8887`
- The payload is byte-identical to the prior finite-difference CG golden payload from `tests/artifacts/perf/2026-06-13-rubywaterfall-opt-reprofile/golden_cg_before_payload.txt`.

Ordering/tie/floating-point/RNG contract:

- `gradient: None` still calls `finite_diff_gradient` with the same central-difference component order, same `gradient_eps`, same objective evaluation order, and same PR+ recurrence.
- Strong-Wolfe alpha/tie behavior for the default path is unchanged because the existing finite-difference callback branch is retained.
- Exact-gradient mode is opt-in only; it validates gradient length and nonfinite values before using callback output.
- Exact-gradient mode reuses the Wolfe accepted gradient when the curvature test produced one, preserving the line-search decision already made by the existing probe.
- No RNG is introduced.

Proof tests:

- Artifact: `test_cg_exact_gradient_rch.txt`
- Worker: `vmi1293453`
- Result: `3 passed; 0 failed`
- Coverage: exact callback jacobian used, bad callback shape rejected, existing exact-gradient helper check.

## Benchmark gate

Same-worker RCH keep gate:

| Worker | Row | Artifact | p50 |
| --- | --- | --- | --- |
| `vmi1149989` | `cg/rosenbrock/10` | `baseline_cg_rosenbrock10_vmi1149989_match_rch.txt` | `204.99 us` |
| `vmi1149989` | `cg/rosenbrock_exact_gradient/10` | `after_cg_rosenbrock10_exact_gradient_vmi1167313_rch.txt` | `126.41 us` |

Speedup: `204.99 / 126.41 = 1.622x`.

The original `vmi1167313` baseline remains the campaign baseline. The `vmi1149989` pair is the same-worker keep gate.

## Gates

- `rustfmt --edition 2024 --check crates/fsci-opt/src/minimize.rs crates/fsci-opt/src/types.rs crates/fsci-opt/src/lib.rs crates/fsci-opt/benches/optimize_bench.rs`: pass (`rustfmt_fsci_opt_check.txt`)
- `rch exec -- cargo check -p fsci-opt --lib --locked`: pass on `vmi1293453` (`check_fsci_opt_lib_rch.txt`)
- `rch exec -- cargo clippy -p fsci-opt --lib --locked --no-deps -- -D warnings`: pass on `vmi1293453` (`clippy_fsci_opt_lib_no_deps_rch.txt`)
- `ubs` touched files: returned pre-existing fsci-opt test/bench warnings and heuristic criticals; no cleanup was folded into this perf lever (`ubs_fsci_opt_exact_gradient.txt`)

## Score

`Impact 3.2 * Confidence 4.0 / Effort 1.5 = 8.53`

Verdict: KEEP. Close `frankenscipy-8l8r1.107` and reprofile before selecting the next opt/linalg primitive.
