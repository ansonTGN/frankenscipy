# frankenscipy-hvkxu rfft unpack-loop rejection

Agent: RubyWaterfall
Date: 2026-06-17
Crate: `fsci-fft`

## Target

Residual `rfft` overhead after the landed radix-2^2 fused radix-4 complex
kernel. Prior profile notes reported the rfft path as mostly half-size complex
FFT core plus a smaller pack/unpack component.

## Candidate

Temporary source lever: specialize `real_fft_specialized` packing with
`chunks_exact(2)` and split endpoint handling out of the unpack loop to remove
the `k == 0 || k == half` branch from the hot bins. The half-size complex FFT,
twiddle table, output bins, public API, normalization, error behavior, and RNG
behavior were unchanged.

## Baseline

Command:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_baseline.json './target-fft-hvkxu/release/perf_fft_vs_scipy'
```

Wall time: `5.432856667 s +/- 0.117723249 s`.

## Proof

`cargo test -p fsci-fft --lib rfft --release --locked -- --nocapture` passed:
21 rfft/irfft tests including SciPy reference values, rfft shape, round trips,
and the metamorphic check that `rfft(real)` matches the first half of
`fft(real_as_complex)`.

## Rebench

Command:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_after_rfft_unpack.json './target-fft-hvkxu/release/perf_fft_vs_scipy'
```

Wall time: `5.407746276 s +/- 0.163966564 s`.

## Verdict

Rejected/no-ship. Behavior proof passed, but the full-command speedup was only
`1.004644x`, below the project keep threshold.

Score: `Impact 0.25 * Confidence 3.0 / Effort 1.0 = 0.75`.

Source status after restore: `git diff -- crates/fsci-fft/src/transforms.rs` is
empty.

Next route: stop loop-spelling cleanup in `real_fft_specialized`; the next
candidate must be a true split-radix complex kernel or a native long-lived SoA
data path that avoids per-call conversion.
