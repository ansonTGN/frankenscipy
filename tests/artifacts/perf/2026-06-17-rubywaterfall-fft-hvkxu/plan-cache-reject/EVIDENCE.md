# frankenscipy-hvkxu plan-cache rejection

Agent: RubyWaterfall
Date: 2026-06-17
Crate: `fsci-fft`

## Target

`frankenscipy-hvkxu` residual FFT gap after the landed radix-2^2 fused
radix-4 kernel. `ts1` is offline, so this pass used local `cargo` and
`hyperfine` against the current `origin/main` scratch worktree.

## Candidate

Temporary source lever: cache the power-of-two bit-reversal swap plan and
feed it into the existing radix-4 Cooley-Tukey sweep. Arithmetic order,
twiddle table entries, public dispatch, normalization, output ordering, and
RNG behavior were unchanged.

## Baseline

Command:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_baseline.json './target-fft-hvkxu/release/perf_fft_vs_scipy'
```

Wall time: `5.432856667 s +/- 0.117723249 s`.

Representative internal timings:

- `2^18`: fft `4.38-5.29 ms`, rfft `2.42-3.06 ms`
- `2^20`: fft `32.29-41.38 ms`, rfft `13.18-15.96 ms`
- `2^22`: fft `223.37-246.78 ms`, rfft `150.57-165.86 ms`

## Proof

`cargo test -p fsci-fft --lib radix4_bit_identical_to_radix2 --release --locked -- --nocapture`
passed. This proof compares the fused radix-4 kernel against the radix-2
sweep by exact tuple equality across even and odd log2 lengths for forward and
inverse transforms.

## Rebench

Command:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_after_plan_cache.json './target-fft-hvkxu/release/perf_fft_vs_scipy'
```

Wall time: `5.639287760 s +/- 0.627872191 s`.

Representative internal timings:

- `2^18`: fft `4.70-6.49 ms`, rfft `2.69-4.06 ms`
- `2^20`: fft `31.26-67.53 ms`, rfft `13.70-24.31 ms`
- `2^22`: fft `208.56-294.03 ms`, rfft `143.59-235.15 ms`

## Verdict

Rejected/no-ship. Behavior proof passed, but local hyperfine wall time regressed
from `5.432856667 s` to `5.639287760 s` (`0.963396x`) and variance increased.

Score: `Impact 0.0 * Confidence 4.0 / Effort 1.0 = 0.0`.

Source status after restore: `git diff -- crates/fsci-fft/src/transforms.rs` is
empty.

Next route: do not retry bit-reversal planning/cache variants. Continue with a
deeper FFT primitive: either a true split-radix complex kernel, or a native
SoA data path that avoids per-call AoS/SoA conversion.
