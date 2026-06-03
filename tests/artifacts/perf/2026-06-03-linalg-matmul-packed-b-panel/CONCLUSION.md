# Packed-B Panel GEMM Conclusion

Decision: keep.

The profile-backed target was `matmul/1024x1024`, with a focused RCH baseline median of `931.06 ms`. The one source lever packed complete 8-column B panels once and consumed those packed panels from the existing SIMD tile loop.

Behavior is unchanged:

- Ordering preserved: output rows and columns are written in the same order.
- Tie-breaking unchanged: not applicable; no comparisons or tie decisions changed.
- Floating-point preserved: every output cell still accumulates `k = 0..ka` in increasing order with the same lane order.
- RNG preserved: not applicable; the path is deterministic and seed-free.
- Golden outputs: stable SHA-256 stayed `48613a728da5350067a920bf0e68b27fc11efd4537046584e2b28a25e75dd771`; before/after stable diff is empty.

Performance:

- Full RCH sweep: `matmul/1024x1024` median `931.06 ms -> 224.80 ms` (`4.14x`).
- Confirm RCH run on `vmi1149989`: `matmul/1024x1024` median `202.96 ms` (`4.59x` against the keep gate).

Validation:

- `cargo fmt -p fsci-linalg --check`: exit `0`.
- `cargo test -p fsci-linalg --release --locked matmul -- --nocapture` via RCH: exit `0`.
- `cargo check -p fsci-linalg --all-targets --locked` via RCH: exit `0`.
- `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings` via RCH: exit `0`.
- `ubs crates/fsci-linalg/src/lib.rs`: exit `0` with no critical findings.

Score: `6.0 = impact 4 * confidence 3 / effort 2`.

Next action: close `frankenscipy-8l8r1.26`, commit/push, then run a fresh linalg reprofile before selecting the next alien primitive.
