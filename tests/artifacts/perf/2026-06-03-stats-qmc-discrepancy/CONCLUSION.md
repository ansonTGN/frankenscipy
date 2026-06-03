# fsci-stats QMC 2D discrepancy invariant cache

Bead: `frankenscipy-8l8r1.2`
Agent: `OliveSnow`
Date: 2026-06-02 EDT

## Profile-backed target

Source profile: `tests/artifacts/perf/2026-06-02-stats-profile/reprofile_after_psd_twiddle_broad_rch.txt`.

Shifted top rows after the PSD twiddle pass:

- `qmc_discrepancy/mixture/512x2`: 1.8570 ms median
- `qmc_discrepancy/centered/512x2`: 1.8384 ms median
- `qmc_discrepancy/wraparound/512x2`: 1.0788 ms median
- `qmc_discrepancy/l2_star/512x2`: 1.0455 ms median

## One lever

Added a `dimension == 2` fast path for the four QMC discrepancy metrics. The helper materializes each row's two coordinates, centered offsets, and absolute centered offsets once, then reuses those invariants in the existing O(n^2) single/double sums.

No external BLAS/LAPACK/XLA linkage and no unsafe code.

## Isomorphism proof

- Validation order is unchanged: zero dimension, shape, empty sample, then coordinate finite/range checks all happen before dispatching to the 2D fast path.
- Public API, error messages, and return types are unchanged.
- Row order remains sample row-major.
- Single sums still visit points in ascending row order.
- Double sums still visit `i` outer, `j` inner in ascending row order.
- Dimension order remains coordinate 0 then coordinate 1.
- Floating-point formulas are unchanged for each metric; cached values only avoid recomputing `x - 0.5` and `abs(x - 0.5)`.
- No RNG, tie-breaking, or global-state surface exists in these discrepancy routines.

## Golden proof

RCH `perf_stats qmc-golden` before/after output is byte-identical.

- Before sha256: `1fb5885cc35367f57b0e818e165a28f87cbb0b9a43fdc7ba4728a6778af44daf`
- After sha256: `1fb5885cc35367f57b0e818e165a28f87cbb0b9a43fdc7ba4728a6778af44daf`
- Comparison: `golden_cmp=identical`

Golden values:

```text
case=qmc_discrepancy_512x2 len=1024
centered=3eef42483d6e0000
mixture=3eef147788540000
l2_star=3f67c5072f2a347c
wraparound=3ee455e9ae6c0000
```

## Benchmark gate

Focused RCH Criterion baseline:

- `centered/512x2`: 2.2093 ms median
- `mixture/512x2`: 6.1542 ms median
- `l2_star/512x2`: 1.2412 ms median
- `wraparound/512x2`: 1.2667 ms median

Focused RCH Criterion after, conservative current-run medians from remote worker `vmi1167313`:

- `centered/512x2`: 457.82 us median, 4.83x faster
- `mixture/512x2`: 604.55 us median, 10.18x faster
- `l2_star/512x2`: 420.64 us median, 2.95x faster
- `wraparound/512x2`: 463.48 us median, 2.73x faster

The raw after log also contains an initial local fallback caused by temporary worker slot exclusion; that block is ignored for acceptance evidence. The accepted after numbers above come from the subsequent remote RCH block.

Score: `10.7 = impact 4.0 * confidence 4.0 / effort 1.5`.

Verdict: keep.

## Validation

- RCH `cargo test -p fsci-stats --lib qmc --locked -- --nocapture`: 54 passed, 0 failed.
- RCH `cargo clippy -p fsci-stats --lib --bin perf_stats --bench stats_bench --locked -- -D warnings`: pass.
- `cargo fmt -p fsci-stats --check`: pass.
- `ubs crates/fsci-stats/src/qmc.rs crates/fsci-stats/src/bin/perf_stats.rs crates/fsci-stats/benches/stats_bench.rs`: exit 0, critical 0.

## Reprofile after kept lever

RCH broad stats Criterion reprofile: `reprofile_after_qmc_broad_rch.txt`.

Shifted top rows after this lever:

- `time_series/psd_welch/4096_w128_o64`: 1.1153 ms median
- `qmc_sampling/halton_4d/4096`: 695.73 us median
- `qmc_discrepancy/mixture/512x2`: 606.43 us median
- `qmc_sampling/sobol_2d/4096`: 579.69 us median
- `qmc_discrepancy/l2_star/512x2`: 539.88 us median
