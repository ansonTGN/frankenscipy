# fsci-runtime policy decision perf closeout

Bead: `frankenscipy-vb9hk`

Profile-backed target:
- Post-calibrator RCH profile on worker `vmi1156319`: `../2026-06-02-runtime-post-profile/post_runtime_bench_rch.txt`.
- Distinct hot rows after the closed calibrator bead:
  - `policy_decide_hardened`: 784.19 ns mean.
  - `policy_decide_strict`: 753.95 ns mean.
  - `solver_select_*`: 34.22-51.91 ns mean.

Focused baseline:
- RCH worker `vmi1153651`, artifact `baseline_policy_decide_rch.txt`.
- `policy_decide_strict`: 781.04 ns mean.
- `policy_decide_hardened`: 748.64 ns mean.

Evaluated lever:
- Temporarily expanded `expected_loss` fixed 3-element iterator dot products into explicit left-to-right arithmetic over the same `decision_loss_matrix` rows.
- Production implementation was manually restored after the benchmark gate failed.

After candidate:
- RCH worker `vmi1156319`, artifact `after_policy_decide_rch.txt`.
- `policy_decide_strict`: 767.82 ns mean.
- `policy_decide_hardened`: 817.79 ns mean.
- Verdict: reject. Strict improved by only 1.7%, while hardened regressed by 9.1%.

Isomorphism proof:
- Ordering and tie-breaking: `select_action` and `top_risk_state` were not changed.
- Floating point: candidate used the same row order and left-to-right addition intent; golden output verified exact decision bits, but the performance gate failed.
- RNG: not used.
- Golden output: `golden_before.txt`, `golden_after.txt`, and `golden_restored.txt` match byte-for-byte at sha256 `f6540cf2b7bef2509c6daf84a8f004badaa7518543fe167125350a1e20a1e2a0`.

Retained change:
- Added `policy_decision_golden_snapshot` in `crates/fsci-runtime/src/policy.rs` so future policy optimizations have a deterministic decision-output sha harness.
- No production policy optimization was retained.

Validation:
- `cargo fmt -p fsci-runtime --check`: `cargo_fmt_check_fsci_runtime_policy.txt`, exit 0.
- `rch exec -- cargo test -p fsci-runtime --lib policy_decision_golden_snapshot --locked -- --nocapture`: before, after, and restored runs exit 0.
- `rch exec -- cargo clippy -p fsci-runtime --all-targets --locked -- -D warnings`: `cargo_clippy_fsci_runtime_policy_rch.txt`, exit 0.
- `ubs crates/fsci-runtime/src/policy.rs`: `ubs_runtime_policy_rs.txt`, exit 0; existing warnings only, no critical issues.

Score:
- Production lever score: 0.0. Do not keep.
