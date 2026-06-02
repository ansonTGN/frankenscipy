# fsci-stats PSD Welch Twiddle Reuse Closeout

Bead: `frankenscipy-5jy17`

## Profile-Backed Target

Fresh RCH Criterion profile before source edits:

- Artifact: `criterion_broad_rch.txt`
- Worker: `vmi1149989`
- `time_series/psd_welch/4096_w128_o64`: `[7.0216 ms, 7.1619 ms, 7.3048 ms]`
- Next rows: `qmc_discrepancy/mixture/512x2` median `757.77 us`; `qmc_discrepancy/centered/512x2` median `635.47 us`.

Source evidence: `psd_welch` recomputed `angle.cos()` and `angle.sin()` for
every segment, frequency, and sample. The trig values depend only on
`window_size`, frequency, and sample index, not on the segment.

## One Lever

Kept one source lever in `crates/fsci-stats/src/lib.rs`: precompute the DFT
twiddle `(cos, sin)` table once per `psd_welch` call and reuse it for every
segment.

## Behavior Proof

- Golden before sha256: `85048a3c06ab045815cbeb238fee9e1e07a05c27ceed3c3782ec0fd5ea97c6b1`
- Golden after sha256: `85048a3c06ab045815cbeb238fee9e1e07a05c27ceed3c3782ec0fd5ea97c6b1`
- Byte comparison: `golden_before.txt` and `golden_after.txt` matched with `cmp -s`.
- Ordering/tie-breaking: no ordering or tie-breaking path changed; output frequency order stays `0..=window_size/2`.
- Floating point: windowed segment values are computed in the same order; for each segment and frequency, the accumulation still runs samples in ascending order with the same `re += s * cos`, `im -= s * sin`, `power`, and `psd[k] += power` operations. Trig values are computed from the same angle expression, just once per call instead of once per segment.
- RNG: no RNG path exists in `psd_welch` or the golden harness.

## Benchmarks

Focused baseline:

- Artifact: `baseline_psd_welch_focused_rch.txt`
- Worker: `vmi1153651`
- `time_series/psd_welch/4096_w128_o64`: `[15.305 ms, 15.638 ms, 16.012 ms]`

Focused after:

- Artifact: `after_psd_welch_focused_rch.txt`
- Worker: `vmi1153651`
- `time_series/psd_welch/4096_w128_o64`: `[1.1127 ms, 1.1896 ms, 1.2774 ms]`

Final broad re-profile:

- Artifact: `reprofile_after_psd_twiddle_broad_rch.txt`
- Worker: `vmi1153651`
- `qmc_discrepancy/mixture/512x2`: `[1.8216 ms, 1.8570 ms, 1.8949 ms]`
- `qmc_discrepancy/centered/512x2`: `[1.7384 ms, 1.8384 ms, 1.9396 ms]`
- `qmc_discrepancy/wraparound/512x2`: `[1.0563 ms, 1.0788 ms, 1.1020 ms]`
- `qmc_discrepancy/l2_star/512x2`: `[1.0200 ms, 1.0455 ms, 1.0756 ms]`
- `time_series/psd_welch/4096_w128_o64`: `[1.0113 ms, 1.0244 ms, 1.0398 ms]`

Score: `23.3 = impact 5.0 * confidence 4.2 / effort 0.9`. Kept because the
focused before/after ran on the same RCH worker, exact golden output is
unchanged, and the top row moved from PSD to QMC discrepancy after the lever.

## Validation

- RCH `cargo test -p fsci-stats hypothesis_tests_match_scipy --lib --locked`: passed `1/1`, `1698` filtered; compiles lib test target and covers the fixed `ttest_rel` test call.
- RCH `cargo check -p fsci-stats --all-targets --locked`: passed on `vmi1153651`.
- RCH `cargo clippy -p fsci-stats --lib --bin perf_stats --bench stats_bench --locked -- -D warnings`: passed.
- `cargo fmt -p fsci-stats --check`: passed.
- `ubs crates/fsci-stats/src/lib.rs crates/fsci-stats/src/bin/perf_stats.rs crates/fsci-stats/benches/stats_bench.rs`: exit `0`, critical `0`.

Known validation blocker: RCH all-targets clippy for `fsci-stats` still fails
on pre-existing legacy test/bin lint backlog unrelated to this perf lever.
Follow-up bead: `frankenscipy-symv0`.
