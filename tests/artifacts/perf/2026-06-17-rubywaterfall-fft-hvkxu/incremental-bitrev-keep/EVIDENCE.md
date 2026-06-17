# Incremental Bit-Reversal Keep - 2026-06-17

## Target

- Bead: `frankenscipy-hvkxu`
- Crate: `fsci-fft`
- Source lever: `crates/fsci-fft/src/transforms.rs`
- Exact baseline commit: `527ff87f1bb83cc95fe4c4dae83ad9bc7815067c`
- Environment: local cargo + hyperfine because `ts1` RCH worker was offline.

## Candidate

Replace the fused radix-4 kernel's bit-reversal setup loop:

- Before: for every index `i`, compute `bit_reverse(i, log_n)` via `usize::reverse_bits()`.
- After: generate the next bit-reversed index incrementally with carry propagation and perform the same `i < j` swaps.

The FFT butterfly arithmetic, twiddle indices, normalization, output ordering, public errors, and deterministic no-RNG behavior are unchanged.

## Baseline

Public mixed FFT/RFFT harness:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_baseline.json './target-fft-hvkxu/release/perf_fft_vs_scipy'
```

Baseline wall time: `5.432856667 s +/- 0.117723249 s`.

Focused exact-base probe baseline:

```text
hyperfine --warmup 1 --runs 5 --show-output --export-json /tmp/frankenscipy_hvkxu_incremental_fftprobe_ab.json '<baseline>/perf_fft fftprobe 4194304 5' '<candidate>/perf_fft fftprobe 4194304 5'
```

Baseline focused wall time: `2.971 s +/- 0.059 s`.

## Behavior Proof

Bit identity:

```text
CARGO_TARGET_DIR=target-fft-hvkxu-incremental cargo test -p fsci-fft --lib radix4_bit_identical_to_radix2 --release --locked -- --nocapture
```

Result: passed. This proves the new permutation produces the same radix-4 input order as the existing radix-2 reference path, and the fused radix-4 output remains bit-identical to radix-2 for both forward and inverse power-of-two sizes.

Public FFT/RFFT/IFFT/IRFFT proof:

```text
CARGO_TARGET_DIR=target-fft-hvkxu-incremental cargo test -p fsci-fft --lib fft --release --locked -- --nocapture
```

Result: 80 passed, 0 failed.

Golden byte equality:

- `rfft`: `d3e41795d153a1f884a968b318d23815a19ae7faeafbf01dc3d1f125af99c16a` before and after.
- `irfft`: `a4e7b37420123aa4df646c157dc6831ceca76875d62d9b2f54295bf7e1430a26` before and after.
- `fft2`: `79b17591371e8b8472ce6e9a89264628b960ae9fcde9f0e5c6b0c6de57bba5d8` before and after.
- `nd`: `1380555e387aac11257160337e7613baf5dfe2b84fcf1ab9cd16d0ad2df5b773` before and after.

Each pair also passed `cmp -s`.

## Rebench

Focused complex `2^22` probe:

- Baseline: `2.971 s +/- 0.059 s`
- Candidate: `2.598 s +/- 0.051 s`
- Hyperfine summary: candidate ran `1.14 +/- 0.03x` faster.

Representative internal focused timings:

- Baseline complex FFT medians: mostly `218-233 ms`.
- Candidate complex FFT medians: mostly `181-198 ms`.

Public mixed FFT/RFFT harness:

- Baseline exact-base A/B: `5.284 s +/- 0.098 s`
- Candidate: `5.185 s +/- 0.245 s`
- Hyperfine summary: candidate ran `1.02 +/- 0.05x` faster.

The public harness includes smaller sizes and `rfft` paths that dilute the targeted `2^22` complex-kernel win, but the focused profile-backed hot path is a stable improvement.

## Quality Gates

```text
rustfmt --edition 2024 --check crates/fsci-fft/src/transforms.rs crates/fsci-fft/src/helpers.rs crates/fsci-fft/src/bin/diff_fftsizes.rs crates/fsci-fft/src/bin/diff_dct.rs crates/fsci-fft/src/bin/diff_dctn.rs
CARGO_TARGET_DIR=target-fft-hvkxu-incremental cargo check -p fsci-fft --all-targets --locked
CARGO_TARGET_DIR=target-fft-hvkxu-incremental cargo clippy -p fsci-fft --all-targets --locked -- -D warnings
CARGO_TARGET_DIR=target-fft-hvkxu-incremental cargo test -p fsci-fft --all-targets --locked
ubs crates/fsci-fft/src/transforms.rs crates/fsci-fft/src/helpers.rs crates/fsci-fft/src/bin/diff_dct.rs crates/fsci-fft/src/bin/diff_dctn.rs crates/fsci-fft/src/bin/diff_fftsizes.rs
```

Results:

- Format: passed.
- Check: passed.
- Clippy: passed.
- Tests: 152 lib tests, 54 metamorphic tests, bin test builds, and bench smoke tests passed.
- UBS: exit 0; no critical findings. Existing warning-class inventory remains outside this lever.

## Score

`Impact 2.0 * Confidence 4.0 / Effort 1.0 = 8.0`

## Verdict

Kept. This is a low-risk in-place setup improvement for the current radix-4 power-of-two kernel. It preserves bit identity and improves the focused `2^22` complex FFT hot path by `1.14x`.
