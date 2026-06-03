# Rectangular Lstsq Single-SVD Conclusion

- Timestamp: 2026-06-03T16:15:37-04:00
- Bead: `frankenscipy-8l8r1.25`
- Decision: kept
- Score: `6.0 = impact 4 * confidence 3 / effort 2`

## Performance

| benchmark | baseline median | after median | delta |
| --- | ---: | ---: | ---: |
| `baseline_lstsq/1000x500` | `1.0625 s` | `852.17 ms` | `1.25x faster` |

The focused RCH Criterion gate ran on `vmi1153651` before and after the one source lever.

## Isomorphism Proof

- Stable golden SHA-256 before: `bdf491ce0154bec5825fcdd3d68a23ec5941bbe4308584ceb2440c136a8722b6`
- Stable golden SHA-256 after: `bdf491ce0154bec5825fcdd3d68a23ec5941bbe4308584ceb2440c136a8722b6`
- Stable before/after diff: empty
- `golden_before_after_tests_stable_cmp.exit`: `0`

The lever reuses the same full SVD result for rectangular `lstsq_with_casp` condition/rank/certificate inputs and pseudo-inverse solve. Square inputs keep the existing condition-SVD path. Validation/error order, selected action, singular-value ordering, tolerance semantics, residual computation, output ordering, RNG absence, tie-breaking absence, and global-state absence remain unchanged.

## Gates

- `cargo fmt -p fsci-linalg --check`: `0`
- `ubs crates/fsci-linalg/src/lib.rs`: `0`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --locked lstsq -- --nocapture`: `0`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo check -p fsci-linalg --all-targets --locked`: `0`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`: `0`

## Next Profile Target

Reprofile after landing this commit before selecting the next primitive. The prior post-MR6 ranking had `matmul/768x768`, `baseline_pinv/1000x500`, and `matmul/1024x1024` behind `baseline_lstsq/1000x500`; the kept lstsq lever may shift that order.
