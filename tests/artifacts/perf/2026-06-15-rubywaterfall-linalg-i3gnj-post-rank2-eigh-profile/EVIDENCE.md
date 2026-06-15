# frankenscipy-i3gnj Evidence

## Target

- Bead: `frankenscipy-i3gnj`
- Crate: `fsci-linalg`
- Hotspot: native symmetric `eigh` Householder reduction after the `psn7x.1` rank-2 keep.
- One lever: native reduction now stores only the lower triangle for trailing rank-2 updates and reads symmetric values from lower storage, avoiding unused upper-mirror stores.

## Profile

Fresh RCH `ovh-a` stage split after `psn7x.1`:

- 800x800: reduction `110.812483 ms` of total `177.972857 ms`
- 1200x1200: reduction `373.956451 ms` of total `585.074864 ms`

The reduction stage remained dominant, so the lever targets reduction storage traffic.

## Baseline

RCH `ovh-a` public route:

- 400x400: `46.164252 ms`, digest `0x4b8334c92ce624eb`
- 800x800: `209.060605 ms`, digest `0xad8a7e5fa1980bfb`
- 1200x1200: `650.198729 ms`, digest `0x181b3486089d0e4a`

RCH `ovh-a` native probe:

- 400x400: `30.9 ms`
- 800x800: `198.5 ms`
- 1200x1200: `608.1 ms`

RCH `ovh-a` Criterion:

- `eigh_dense/256x256`: median `10.991 ms`
- `eigh_dense/512x512`: median `97.576 ms`

## Proof

- `symmetric_rank2_lower_storage_matches_full_update_lower_bits` passed: p/w vectors and stored lower triangle match the full symmetric rank-2 update by `f64::to_bits()` across multiple reflector starts.
- `cargo test -j 1 -p fsci-linalg --lib eigh --release --locked -- --nocapture --test-threads=1` passed 16 behavior tests.
- Public golden digest remained `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`.
- Public route value digests after the lever remained:
  - 400x400: `0x4b8334c92ce624eb`
  - 800x800: `0xad8a7e5fa1980bfb`
  - 1200x1200: `0x181b3486089d0e4a`
- Ordering/tie behavior, residual/orthogonality tests, deterministic RNG seeds, and fallback routing were unchanged.

## After

RCH `ovh-a` public route:

- 400x400: `44.085061 ms` (below native threshold; treated as noise)
- 800x800: `187.708423 ms` (`1.113752x`, `10.213%`)
- 1200x1200: `569.041634 ms` (`1.142621x`, `12.482%`)

RCH `ovh-a` native probe:

- 400x400: `32.2 ms` (below public native threshold)
- 800x800: `189.4 ms` (`1.048046x`, `4.584%`)
- 1200x1200: `575.4 ms` (`1.056830x`, `5.377%`)

RCH `ovh-a` Criterion:

- `eigh_dense/256x256`: median `10.921 ms` (`1.006410x`; below native threshold)
- `eigh_dense/512x512`: median `93.226 ms` (`1.046661x`, `4.458%`)

## Gates

- `cargo fmt -p fsci-linalg -- --check`: pass
- RCH `cargo check -j 1 -p fsci-linalg --lib --locked`: pass
- RCH `cargo clippy -j 1 -p fsci-linalg --lib --no-deps --locked -- -D warnings`: pass
- `ubs crates/fsci-linalg/src/lib.rs`: `Critical issues: 0`
- Known unrelated warnings observed in dependency/bin builds:
  - `fsci-fft/src/helpers.rs:58` unused `total`
  - `crates/fsci-linalg/src/bin/perf_cwt.rs:20` unused `j` during Criterion bench

## Score

`Impact 2.0 * Confidence 4.0 / Effort 1.0 = 8.0`

Verdict: KEEP.
