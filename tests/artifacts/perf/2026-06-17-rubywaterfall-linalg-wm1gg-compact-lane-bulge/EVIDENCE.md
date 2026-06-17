# frankenscipy-wm1gg - Compact Lane-Local DSBTRD Bulge Chase Baseline

Date: 2026-06-17
Agent: RubyWaterfall
Target: `eig_banded(lower=true, eigvals_only=false)`
Worker: local only, per ts1/RCH-offline override after `rch exec` timed out while syncing for predecessor `frankenscipy-psn7x.19`

## Reason

`frankenscipy-psn7x.19` closed rejected/no-ship after two proofed routes:

- lower-storage/mirror cleanup was behavior-preserving but flat (`205.5 ms +/- 9.6 ms -> 206.0 ms +/- 10.3 ms`)
- dense adjacent-Givens DSBTRD was numerically valid but much slower (`205.5 ms +/- 9.6 ms -> 267.6 ms +/- 32.0 ms`)

This successor attacks the algorithmic gap left by those failures: compact diagonal/band-lane bulge chasing with direct Q metadata replay, avoiding dense full-row/full-column rotations.

## Baseline

Command:

```bash
env RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-psn7x19-target \
  hyperfine --warmup 1 --runs 5 --show-output \
  'cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture --test-threads=1'
```

Transcripts:

- `baseline_probe_local.txt`
- `baseline_hyperfine_local.txt`

Result:

- Wall: `198.7 ms +/- 5.6 ms`
- 128x128 bw32 candidate range: `3.062364-4.340820 ms`
- 256x256 bw32 candidate range: `12.771252-16.866366 ms`
- Residuals: `1.64845914696343243e-12` at 128 and `7.73070496506989002e-12` at 256
- Values digests: `0xd6dbb9200f65bd92` and `0x09ed4d367faab431`
- Vector digests: `0x6cf3573b5b50c275` and `0xc32797c0d224a75a`

## Required Next Lever

Allowed:

- compact diagonal/band-lane DSBTRD-style bulge chase
- direct Q metadata replay into eigenvector rows/columns
- dense oracle residual, orthogonality, ordering, and deterministic no-RNG proof

Do not retry:

- lower-storage native entrypoints or upper-mirror/clone cleanup
- dense full-row/full-column adjacent rotations
- direct-index packet wrappers
- full active lower-envelope storage
- fixed envelope width guesses
- Lanczos vectors
- shifted inverse iteration over band solves
- worker-count retuning
- raw/stale compact-WY panels
- scalar spelling or SIMD rank-2 spelling
