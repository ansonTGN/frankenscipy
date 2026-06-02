# fsci-special incomplete elliptic vector perf closeout

Bead: frankenscipy-perf-special-elliptic-vector-733cd

Fresh profile target:
- RCH worker vmi1156319, artifact `../2026-06-02-special-fresh-profile/baseline_special_bench_rch.txt`.
- `special_incomplete_elliptic/ellipkinc_broadcast_m/scalar_phi_over_m_vec`: 800.69 ns mean in the broad profile.
- `special_incomplete_elliptic/ellipeinc_pairwise_vec/phi_vec_m_vec`: 756.99 ns mean in the broad profile.

Focused baseline:
- RCH worker vmi1149989, artifact `baseline_incomplete_elliptic_rch.txt`.
- `ellipkinc_broadcast_m/scalar_phi_over_m_vec`: 421.15 ns mean.
- `ellipeinc_pairwise_vec/phi_vec_m_vec`: 418.71 ns mean.

Lever:
- Add a real `ellipkinc` scalar-phi/vector-m branch that precomputes the 15-point quadrature `sin²` nodes once per scalar `phi`.
- Fall back to the original scalar loop for NaN, domain, and complete-integral cases so error ordering and hardened-mode behavior stay unchanged.
- Preserve each per-`m` accumulation order and the `m == 0` exact fast path.

After:
- RCH worker vmi1149989, artifact `after_incomplete_elliptic_rch.txt`.
- `ellipkinc_broadcast_m/scalar_phi_over_m_vec`: 233.80 ns mean, 44.5% faster.
- `ellipeinc_pairwise_vec/phi_vec_m_vec`: 415.79 ns mean, effectively unchanged.

Isomorphism proof:
- Ordering and tie-breaking: vector element order remains left-to-right; invalid/NaN/hardened cases use the old scalar loop.
- Floating point: hot-path `sin²` node values are computed with the same `half_phi * (1 +/- node)` and `sin().powi(2)` formulas, then each `m` uses the same weighted sum order.
- RNG: not used.
- Golden output: `golden_before.txt` and `golden_after.txt` match byte-for-byte; sha256 `4c45d0f6c9719d0ae0cd847c76d9caeeef0f5d2134f9c15659ca4e57cbcfa95e`.

Validation:
- `cargo fmt -p fsci-special --check`: `cargo_fmt_check_fsci_special_final3.txt`, exit 0.
- `rch exec -- cargo test -p fsci-special --lib elliptic --locked`: `cargo_test_fsci_special_elliptic_final_rch.txt`, 99 passed, exit 0.
- `rch exec -- cargo clippy -p fsci-special --all-targets --locked -- -D warnings`: `cargo_clippy_fsci_special_all_targets_final2_rch.txt`, exit 0.
- `ubs crates/fsci-special/src/elliptic.rs`: `ubs_elliptic_rs.txt`, exit 0; existing warnings only, no critical issues.

Score:
- Impact 4, confidence 5, effort 2 => 10.0. Keep.
