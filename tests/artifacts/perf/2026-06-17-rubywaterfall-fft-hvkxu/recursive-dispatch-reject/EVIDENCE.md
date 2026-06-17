# Recursive Dispatch Rejection - 2026-06-17

## Target

- Bead: `frankenscipy-hvkxu`
- Crate: `fsci-fft`
- Candidate source file: `crates/fsci-fft/src/transforms.rs`
- Exact baseline commit: `527ff87f1bb83cc95fe4c4dae83ad9bc7815067c`

## Candidate

Large power-of-two complex FFTs (`n >= 2^20`) were temporarily routed from the fused radix-4 in-place kernel to the existing recursive `mixed_radix_fft` path:

- `data.to_vec()` created the recursive input buffer.
- a same-length scratch buffer received the output.
- the scratch result was copied back into the public in-place slice.

This was a structural probe, not a shipped change. It tested whether the existing recursive layout could harvest a cache-oblivious traversal win before writing a dedicated split-radix kernel.

## Behavior Proof

Command:

```text
CARGO_TARGET_DIR=target-fft-hvkxu cargo test -p fsci-fft --lib fft --release --locked -- --nocapture
```

Result: passed 80 FFT/RFFT/IFFT/IRFFT tests, including SciPy references, round trips, Parseval/impulse/constant/sine invariants, normalization behavior, and public error behavior. RNG behavior is absent for these deterministic transforms.

## Benchmark

Command:

```text
hyperfine --warmup 2 --runs 7 --show-output --export-json /tmp/frankenscipy_hvkxu_recursive_radix_ab.json '/data/projects/.scratch/frankenscipy-rubywaterfall-fft-baseline-527ff87f-20260617/target-fft-hvkxu-baseline/release/perf_fft_vs_scipy' './target-fft-hvkxu-recursive/release/perf_fft_vs_scipy'
```

Result:

- Baseline: `5.396 s +/- 0.212 s`
- Candidate: `10.220 s +/- 0.203 s`
- Hyperfine summary: baseline ran `1.89 +/- 0.08x` faster than candidate.

Representative candidate internal timings showed the large complex path regressed:

- `2^20` complex FFT: roughly `89-102 ms` versus baseline roughly `33-45 ms`.
- `2^22` complex FFT: roughly `414-445 ms` versus baseline roughly `227-255 ms`.

## Score

`Impact 0.0 * Confidence 5.0 / Effort 1.0 = 0.0`

## Verdict

Rejected. The existing recursive mixed-radix engine is not a viable drop-in large-power-of-two replacement because allocation plus recursive scatter/gather traversal overwhelms any locality benefit. The next structural lever must be a dedicated split-radix or blocked in-place kernel rather than a dispatch to the current recursive path.
