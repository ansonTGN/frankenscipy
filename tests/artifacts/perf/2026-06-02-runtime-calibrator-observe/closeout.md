# fsci-runtime calibrator observe perf closeout

Bead: frankenscipy-eehzf

Fresh profile target:
- RCH worker vmi1293453, artifact `../2026-06-02-runtime-fresh-profile/baseline_runtime_bench_rch.txt`.
- `calibrator_observe_200`: 1.0111 us mean, top row.
- `policy_decide_strict`: 440.93 ns mean.
- `policy_decide_hardened`: 392.41 ns mean.
- `solver_select_*`: 18.99-25.29 ns mean.

Focused baseline:
- RCH worker vmi1149989, artifact `baseline_calibrator_observe_rch.txt`.
- `calibrator_observe_200`: 1.0812 us mean.

Lever:
- Initialize `ConformalCalibrator::scores` with `VecDeque::with_capacity(capacity.max(10))`.
- Preserve the existing capacity clamp and all observe/fallback logic.

After:
- RCH worker vmi1293453, artifact `after_calibrator_observe_rch.txt`.
- `calibrator_observe_200`: 927.53 ns mean.
- Delta vs focused baseline: 14.2% faster.
- Delta vs same-worker broad profile row: 8.3% faster.

Isomorphism proof:
- Ordering and tie-breaking: observe order and eviction order are unchanged; only initial queue allocation changes.
- Floating point: score normalization, threshold comparisons, empirical-miscoverage division, and fallback predicate are unchanged.
- RNG: not used.
- Golden output: `golden_before.txt` and `golden_after.txt` match byte-for-byte; sha256 `90417ed3aa8d621e1ab600eb70cae398eecc15dc16b5236b2248819df9af1356`.

Validation:
- `cargo fmt -p fsci-runtime --check`: `cargo_fmt_check_fsci_runtime.txt`, exit 0.
- `rch exec -- cargo test -p fsci-runtime --lib conformal_calibrator --locked`: `cargo_test_fsci_runtime_calibrator_final_rch.txt`, 3 passed, exit 0.
- `rch exec -- cargo clippy -p fsci-runtime --all-targets --locked -- -D warnings`: `cargo_clippy_fsci_runtime_all_targets_final_rch.txt`, exit 0.
- `ubs crates/fsci-runtime/src/lib.rs`: `ubs_runtime_lib_rs.txt`, exit 0; existing warnings only, no critical issues.

Score:
- Impact 3, confidence 5, effort 1 => 15.0. Keep.
