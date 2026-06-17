# Split-Radix Rejection - 2026-06-17

## Target

- Bead: `frankenscipy-hvkxu`
- Crate: `fsci-fft`
- Candidate source file: `crates/fsci-fft/src/transforms.rs`
- Exact baseline commit: `527ff87f1bb83cc95fe4c4dae83ad9bc7815067c`

## Candidate

A true split-radix out-of-place path was temporarily added for large power-of-two 1D transforms:

- public `transform_1d_unscaled` routed large power-of-two inputs directly from `input` into a freshly allocated output buffer;
- in-place calls cloned the current input and wrote split-radix output back into the original slice;
- the recursive helper reused the root twiddle table, avoiding per-recursion cache lookups;
- the decomposition used one `n/2` even transform plus two `n/4` odd transforms.

## Behavior Proof

Command:

```text
CARGO_TARGET_DIR=target-fft-hvkxu-split cargo test -p fsci-fft --lib split_radix_matches_radix4_tolerance --release --locked -- --nocapture
```

Result: passed. The temporary test compared split-radix output to the existing radix-4 kernel across forward and inverse power-of-two sizes `2^1..2^12` with max component error `<= 1e-8`. Public ordering, normalization, shape/error behavior, and deterministic no-RNG behavior were unchanged by construction.

## Benchmark

Command:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_split_radix_ab.json '/data/projects/.scratch/frankenscipy-rubywaterfall-fft-baseline-527ff87f-20260617/target-fft-hvkxu-baseline/release/perf_fft_vs_scipy' './target-fft-hvkxu-split/release/perf_fft_vs_scipy'
```

Result:

- Baseline: `5.296 s +/- 0.408 s`
- Candidate: `7.891 s +/- 0.163 s`
- Hyperfine summary: baseline ran `1.49 +/- 0.12x` faster than candidate.

Representative candidate internal timings showed the large complex path regressed:

- `2^20` complex FFT: roughly `69-79 ms` versus baseline usually `30-35 ms`.
- `2^22` complex FFT: roughly `353-378 ms` versus baseline usually `221-230 ms`.

## Score

`Impact 0.0 * Confidence 5.0 / Effort 3.0 = 0.0`

## Verdict

Rejected. Reducing arithmetic via recursive split-radix is not enough here; recursive scatter/gather traversal and branch overhead lose to the current fused in-place radix-4 sweep. The next candidate must stay in-place and improve the existing stage traversal, such as a blocked/tiled radix sweep or a larger fused stage that preserves contiguous writes.
