# frankenscipy-qnpl5 rejection summary

Status: rejected. No `fsci-fft` library optimization code was kept.

Target: transpose-based first-axis and outer-axis parallelization for `fftn`/`fft2`.

Baseline, RCH `ts2`:

- Command: `cargo bench -p fsci-fft --bench fft_bench --locked -- baseline_fft2/fft2/512x512 --noplot`
- Result: `baseline_fft2/fft2/512x512 time: [8.6131 ms 8.6470 ms 8.6820 ms]`

Attempted lever, same worker `ts2`:

- Axis-0 transpose/gather/scatter path plus across-outer parallel block splitting.
- Bit identity proof during the attempt: `cargo test -p fsci-fft --lib fft_axis0_transpose_transform_is_bit_identical_to_sequential_reference --locked -- --nocapture` passed.
- Golden proof during the attempt:
  - `golden_fft2_after.txt` SHA256 `79b17591371e8b8472ce6e9a89264628b960ae9fcde9f0e5c6b0c6de57bba5d8`
  - `golden_fft2_cmp.txt`: `fft2_golden_cmp=identical`
  - `golden_nd_after.txt` SHA256 `1e1cab2a6d719cb752e4776a840c48d848acfdb0df3bccb4a21e415a59d345a5`

After benchmark, RCH `ts2`:

- Command: `cargo bench -p fsci-fft --bench fft_bench --locked -- baseline_fft2/fft2/512x512 --noplot`
- Result: `baseline_fft2/fft2/512x512 time: [14.030 ms 14.473 ms 14.920 ms]`
- Delta: `8.6470 ms -> 14.473 ms`, 0.597x throughput equivalent, 67.38% slower.
- Keep score: rejected. Impact is negative, so Impact x Confidence / Effort is below the 2.0 keep gate.

Final kept source state:

- `crates/fsci-fft/src/transforms.rs`: no diff after rejection (`transforms_diff_after_revert.txt`).
- `crates/fsci-fft/src/bin/perf_fft.rs`: retained only deterministic ND golden/timing harness modes (`nd-golden`, `fft2t`, `fftnt`) for future profile-backed passes.

Final verification:

- `cargo fmt -p fsci-fft --check`: pass.
- RCH `cargo check -p fsci-fft --all-targets --locked`: pass (`cargo_check_final_harness_only_fsci_fft_rch.txt`).
- RCH `cargo clippy -p fsci-fft --all-targets --locked -- -D warnings`: pass (`cargo_clippy_final_harness_only_fsci_fft_rch.txt`).
- `ubs crates/fsci-fft/src/bin/perf_fft.rs`: exit 0 (`ubs_perf_fft.txt`).

Next deeper primitive:

- Attack a planned six-step/Stockham ND FFT primitive with plan-owned reusable workspaces and tiled in-place transposes. Target ratio: at least 2.5x on `fft2/512x512` by removing per-call full transposed allocation, avoiding OS-thread spawn per axis pass, and converting strided axis work into cache-blocked contiguous tiles without changing the floating-point operation order inside each 1D lane.
