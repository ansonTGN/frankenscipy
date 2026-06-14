# Evidence: frankenscipy-8l8r1.105 CG fused Wolfe value-gradient evaluator

## Target

- Bead: `frankenscipy-8l8r1.105`
- Skill loop: `repeatedly-apply-skill` -> `extreme-software-optimization`, with `alien-graveyard` and `alien-artifact-coding` routing
- Profile-backed hotspot: `fsci-opt` Criterion row `cg/rosenbrock/10`
- Lever: fuse the CG strong-Wolfe value and finite-difference gradient probe so the trial point and accepted gradient computed for the curvature check are reused by PR+.

## Benchmark

Both kept rows are RCH Criterion runs on worker `vmi1149989`.

| Artifact | Row | p50 |
| --- | --- | ---: |
| `baseline_cg_rosenbrock10_rch.txt` | `cg/rosenbrock/10 time: [294.41 us 304.43 us 316.34 us]` | `304.43 us` |
| `after_cg_rosenbrock10_final_move_rch.txt` | `cg/rosenbrock/10 time: [193.60 us 196.93 us 201.62 us]` | `196.93 us` |

Speedup: `304.43 / 196.93 = 1.546x`.

Progressive deepening recorded and rejected subvariants before the kept lever:

- `after_cg_rosenbrock10_rch.txt`: accepted-gradient carry without fused finite-difference probe, `[248.29 us 263.57 us 284.41 us]`.
- `after_cg_rosenbrock10_probe_final_rch.txt`: reusable gradient probe, `[216.03 us 229.25 us 243.39 us]`.
- `after_cg_rosenbrock10_probe_repeat_rch.txt`: repeat probe check, `[224.29 us 233.11 us 241.38 us]`.
- `after_cg_rosenbrock10_trial_reuse_rch.txt`: trial-buffer reuse, `[220.62 us 230.40 us 241.24 us]`.
- `after_cg_rosenbrock10_inplace_rch.txt`: in-place finite-difference probe, `[197.81 us 204.83 us 214.13 us]`.

The final retained variant also moves the accepted gradient out of the reusable gradient buffer with `std::mem::take`, avoiding the accepted-gradient clone.

## Behavior proof

- Public API is unchanged: `line_search_wolfe1` and `line_search_wolfe2` still return `LineSearchResult`.
- Public strong-Wolfe decisions are unchanged: `wolfe2_with_gradient_matches_public_result_and_carries_gradient` compares public `line_search_wolfe2` against the probe path for alpha, function value, directional derivative, and evaluation count bit-for-bit.
- Wolfe ordering is unchanged for CG: each candidate still evaluates `f(alpha)` before the directional derivative; the Armijo, bracket, zoom, and curvature comparisons use the same inequalities and alpha updates.
- Finite-difference order is unchanged: for each component, the probe evaluates `x[i] + step`, then `x[i] - step`, restores the component, and advances in index order.
- Floating-point dot order is unchanged: the directional derivative still uses the existing `dot(gradient, &direction)` helper.
- Counter behavior is preserved: when CG consumes the accepted Wolfe gradient, `Objective::reserve_evaluations(n * 2)` reserves the duplicate finite-difference calls that the old post-line-search gradient would have spent, preserving `nfev`, `njev`, and maxfev threshold behavior.
- RNG surface is unchanged: this path has no random input and adds no random state.
- Safety surface is unchanged: no `unsafe`, no C BLAS, no MKL, no XLA.

Golden payloads:

- CG golden SHA-256 unchanged:
  - before: `92830169fd8409f2c5b7566f2378c0c79b93135ae3bda05e6d7c0a4f655e8887`
  - after:  `92830169fd8409f2c5b7566f2378c0c79b93135ae3bda05e6d7c0a4f655e8887`
  - artifacts: `golden_cg_after_final_payload.txt`, `../2026-06-13-rubywaterfall-opt-reprofile/golden_cg_before_payload.txt`
- Minimize golden SHA-256 unchanged:
  - before: `f02b24201c2844e1cb1577159ebb29535e2d16a8ccd3676279670e8b6fffad27`
  - after:  `f02b24201c2844e1cb1577159ebb29535e2d16a8ccd3676279670e8b6fffad27`
  - artifacts: `golden_minimize_after_final_payload.txt`, `../2026-06-13-rubywaterfall-opt-reprofile/golden_minimize_before_payload.txt`

## Gates

- `rustfmt --edition 2024 --check crates/fsci-opt/src/linesearch.rs crates/fsci-opt/src/minimize.rs crates/fsci-opt/src/bin/perf_cg_golden.rs`: passed.
- `git diff --check -- crates/fsci-opt/src/linesearch.rs crates/fsci-opt/src/minimize.rs`: passed.
- RCH `cargo test -p fsci-opt wolfe2_with_gradient_matches_public_result_and_carries_gradient --lib --locked -- --nocapture`: passed on `vmi1149989`; artifact `test_wolfe_gradient_probe_final_move_rch.txt`.
- RCH `cargo check -p fsci-opt --all-targets --locked`: passed; artifact `check_fsci_opt_all_targets_final_move_rch.txt`.
- RCH `cargo clippy -p fsci-opt --lib --locked --no-deps -- -D warnings`: passed; artifact `clippy_fsci_opt_lib_no_deps_final_move_rch.txt`.
- RCH `cargo clippy -p fsci-opt --all-targets --locked --no-deps -- -D warnings`: blocked by pre-existing probe-bin lint debt in `crates/fsci-opt/src/bin/diff_opt.rs` and `crates/fsci-opt/src/bin/diff_root.rs` (`type_complexity`, `clone_on_copy`, `needless_borrows_for_generic_args`); artifact `clippy_fsci_opt_all_targets_no_deps_rch.txt`. This was not fixed in the perf commit to keep the lever single-purpose.
- UBS on the touched opt files and proof bin still reports broad pre-existing/test/probe warnings; UBS internal fmt, clippy, check, and test sections are clean, and the final accepted-gradient clone warning is gone. Committed summary artifact `ubs_fsci_opt_cg_fused_final_summary.txt`; raw UBS transcript retained locally but not committed because its banner contains trailing whitespace.

## Score

`(Impact 3.2 * Confidence 4.0) / Effort 1.5 = 8.53`.

Verdict: KEEP.

Next route after close: reprofile `fsci-opt` and pick the next profile-backed shifted bottleneck, not another CG micro-lever unless profiling moves back to CG.
