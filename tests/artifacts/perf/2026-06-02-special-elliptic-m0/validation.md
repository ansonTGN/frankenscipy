# Validation

Bead: `frankenscipy-rqh0o`

Passed:
- Remote focused bit-contract test on `vmi1167313`: `RCH_FORCE_REMOTE=1 CARGO_TARGET_DIR=/data/tmp/cargo-target-frankenscipy-olivesnow-special rch exec -- cargo test -p fsci-special incomplete_elliptic_m_zero_preserves_quadrature_bits --lib --locked -- --nocapture`
- Remote after benchmark on `vmi1293453`: `cargo bench -p fsci-special --bench special_bench -- special_incomplete_elliptic --warm-up-time 1 --measurement-time 2 --sample-size 10`
- Crate lib test suite passed: `937 passed; 0 failed`. `rch` reported local fallback because no workers were admissible for that command.
- Remote production clippy on `vmi1153651`: `cargo clippy -p fsci-special --lib --locked -- -D warnings`
- UBS touched-file scan: `ubs crates/fsci-special/src/elliptic.rs`, exit 0, no critical issues.

Known validation caveats:
- `cargo fmt -p fsci-special --check` failed on pre-existing formatting drift across multiple fsci-special files. The optimization diff was kept scoped and no unrelated formatting rewrite was applied.
- Remote `cargo clippy -p fsci-special --all-targets --locked -- -D warnings` failed on pre-existing test literal lints in `bessel.rs`, `convenience.rs`, `elliptic.rs`, and `lib.rs`. The production lib clippy command passed remotely after that.
