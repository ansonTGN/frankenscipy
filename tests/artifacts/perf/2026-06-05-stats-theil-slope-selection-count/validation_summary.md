# Theil Slope In-Place Median Validation

Bead: `frankenscipy-79ttn`

Lever: compute Theil slope medians in place on the materialized slope buffer and
leave the existing confidence-interval order-statistic selection sequence
unchanged. Test-only `count_slopes_le` helpers prove the safe inversion-count
gate needed by the follow-up randomized slope-selection primitive.

## Baseline

- RCH detailed harness: `baseline_perf_theilslopes_rch.txt`
- RCH hyperfine: `baseline_hyperfine_theilslopes_rch.txt`
- Baseline `new` times:
  - `n=1000`: `10.765 ms`
  - `n=2000`: `44.360 ms`
  - `n=3000`: `107.687 ms`
- Baseline old/full-sort-normalized ratios:
  - `n=1000`: `2.3x`
  - `n=2000`: `2.6x`
  - `n=3000`: `2.7x`
- Hyperfine wall clock: `1.422 s +/- 0.024 s`

## After

- RCH detailed harness: `after_inplace_median_perf_theilslopes_rch.txt`
- RCH hyperfine: `after_inplace_median_hyperfine_theilslopes_rch.txt`
- After `new` times:
  - `n=1000`: `3.071 ms`
  - `n=2000`: `13.723 ms`
  - `n=3000`: `38.996 ms`
- After old/full-sort-normalized ratios:
  - `n=1000`: `5.4x`
  - `n=2000`: `5.3x`
  - `n=3000`: `4.6x`
- Hyperfine wall clock: `1.306 s +/- 0.107 s`

## Behavior

- Golden payload SHA before: `18d932a97e4167cf5fdcadd0f0c6f0bd63a399ffc9ab3c38031012a370e1d47c`
- Golden payload SHA after: `18d932a97e4167cf5fdcadd0f0c6f0bd63a399ffc9ab3c38031012a370e1d47c`
- `cmp golden_before_payload.txt golden_after_inplace_payload.txt`: exit `0`
- Harness parity after: `0` numeric mismatches across `180` cases and `720`
  fields; the existing `20` zero-sign-only differences remain numerically equal.

## Validation Commands

- `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-stats --locked count_slopes_le -- --nocapture`
  - Passed `3` focused count tests.
- `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-stats --locked theil -- --nocapture`
  - Passed `11` library tests plus `mr_theil_sen_recovers_linear_params`.
- `RCH_FORCE_REMOTE=1 rch exec -- cargo check -p fsci-stats --all-targets --locked`
  - Passed.
- `RCH_FORCE_REMOTE=1 rch exec -- cargo clippy -p fsci-stats --all-targets --no-deps --locked -- -D warnings`
  - Passed.
- `cargo fmt -p fsci-stats --check`
  - Passed.
- `git diff --check`
  - Passed.
- `ubs crates/fsci-stats/src/lib.rs crates/fsci-stats/src/bin/perf_theilslopes.rs`
  - Exit `0`; zero critical issues, with only the existing broad stats-file
    warning inventory.

## Next Primitive

Follow-up bead `frankenscipy-g0d8t` tracks exact randomized Theil slope interval
selection with bounded final enumeration. The key open proof item is
pivot-near-slope behavior, where the z-transform count can disagree with the
current division-based `slope <= threshold` predicate if a threshold is razor
close to an actual pair slope.
