# frankenscipy-hvkxu radix4 stage-1 specialization rejection

Agent: RubyWaterfall
Date: 2026-06-17
Crate: `fsci-fft`

## Candidate

Temporary source lever: specialize the first fused radix-4 stage for even-log
power-of-two sizes, avoiding the two identity complex multiplies per four
inputs while preserving the existing table-driven +/-i multiply.

## Proof

`cargo test -p fsci-fft --lib radix4_bit_identical_to_radix2 --release --locked -- --nocapture`
passed, so the candidate preserved the existing bit-identical radix4-vs-radix2
proof fixture.

## Rebench

Initial candidate-only local hyperfine reported wall `5.300 s +/- 0.151 s`
versus the earlier baseline `5.432856667 s +/- 0.117723249 s`.

Because that margin was small, a paired exact-base A/B was run:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_stage1_ab.json \
  '/data/projects/.scratch/frankenscipy-rubywaterfall-fft-baseline-527ff87f-20260617/target-fft-hvkxu-baseline/release/perf_fft_vs_scipy' \
  '/data/projects/.scratch/frankenscipy-rubywaterfall-fft-20260617/target-fft-hvkxu-stage1/release/perf_fft_vs_scipy'
```

Paired A/B:

- baseline: `5.405 s +/- 0.127 s`
- candidate: `5.394 s +/- 0.312 s`
- hyperfine summary: candidate ran `1.00 +/- 0.06x` faster

## Verdict

Rejected/no-ship. The bit-identity proof passed, but the exact-base A/B reduced
the measured change to noise.

Score: `Impact 0.1 * Confidence 4.0 / Effort 1.0 = 0.4`.

Source status after restore: `git diff -- crates/fsci-fft/src/transforms.rs` is
empty.

Next route: stop radix-4 spelling variants and implement the selected
split-radix large-power-of-two candidate.
