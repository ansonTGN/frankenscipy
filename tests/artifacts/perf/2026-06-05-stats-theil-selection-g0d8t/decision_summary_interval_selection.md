frankenscipy-g0d8t interval-selection keep
================================================

Lever
-----
- Added a large-input fast path for clean finite distinct-x `theil_sen` and `theilslopes`.
- The fast path uses deterministic slope sampling to bracket target ranks, inversion-count certification, and a final exact division scan over `(lower, upper]` to collect only the bounded candidate interval.
- Fallback to the previous materialized implementation is mandatory for small n, x ties, x gaps <= `1e-15`, non-finite x/y, non-finite transforms, certificate mismatch, excessive candidate intervals, and selected zero-valued ranks.

Behavior proof
--------------
- Pair filtering is unchanged: only unordered pairs with `abs(dx) > 1e-15`.
- Returned fast-path slopes come from the same division expression as the old slope buffer.
- Median, CI rank formulas, intercept formulas, comparator choices, and public RNG behavior are unchanged.
- Golden harness: `0` numeric mismatches across `180` cases / `1080` fields; benign `+0.0/-0.0` sign-only differences remain numeric-equal only in the old-vs-current harness comparison.
- Golden payload SHA-256: `fad47e96df30fc90981139d4bb2b954e9c5d9ff7ff2cc94e99ede7bbf65af9c9`.

Benchmarks
----------
- Baseline RCH harness on `ts1`:
  - n=1000: `4.011 ms`
  - n=2000: `15.570 ms`
  - n=3000: `59.197 ms`
- After RCH harness on `ts1`:
  - n=1000: `3.465 ms` (`1.16x`)
  - n=2000: `14.970 ms` (`1.04x`)
  - n=3000: `33.114 ms` (`1.79x`)
- Cross-worker after RCH harness on `ts2`: n=3000 `51.969 ms`, `0` numeric mismatches.
- Hyperfine baseline: `1.411 s +/- 0.077 s`.
- Hyperfine after: one run had an `85.460 s` cache/build outlier; min was `1.187 s`. The repeat failed one local `cargo run` iteration, so hyperfine is retained as noisy wall-clock context, not the keep decision.

Validation
----------
- `rch exec -- cargo test -p fsci-stats --lib theil --locked -- --nocapture`
- `rch exec -- cargo test -p fsci-stats --lib min_slope_gt --locked -- --nocapture`
- `rch exec -- cargo test -p fsci-stats --lib interval_enumeration --locked -- --nocapture`
- `rch exec -- cargo test -p fsci-stats --lib rank_selection --locked -- --nocapture`
- `rch exec -- cargo check -p fsci-stats --all-targets --locked`
- `rch exec -- cargo clippy -p fsci-stats --lib --bin perf_theilslopes --bench stats_bench --no-deps --locked -- -D warnings`
- `cargo fmt -p fsci-stats --check`
- `ubs crates/fsci-stats/src/lib.rs crates/fsci-stats/src/bin/perf_theilslopes.rs`

Score
-----
`3.0 = Impact 3 * Confidence 4 / Effort 4`

