# fsci-spatial jensenshannon ordered SIMD

Bead: `frankenscipy-8l8r1.97`

## Target

Profile-backed hotspot: `scipy.spatial.distance.jensenshannon`-compatible
distance spends most of this probe in per-element normalization plus two
logarithms. The prior committed SIMD route vectorized the lane work but used a
horizontal SIMD reduction, which changed floating-point accumulation order.

Lever in this commit: keep the vectorized per-lane normalization/logarithm work,
but replay each SIMD lane into the scalar accumulator in original index order.
This preserves the old normalized-Vec result bits while retaining the allocation
elision and SIMD math kernel.

Alien mapping:

- Canonical graveyard: vectorized execution/hot-kernel guidance and deterministic
  fallback/proof contracts in
  `/data/projects/alien_cs_graveyard/high_level_summary_of_frankensuite_planned_and_implemented_features_and_concepts.md`.
- Alien artifact family: certified rewrite pipeline, with equivalence evidence
  and deterministic replay fixture.

## Baseline and After

RCH hyperfine baseline:

- Command: `rch exec -- cargo run --profile release-perf -p fsci-spatial --bin perf_cdist --locked --quiet -- jensenshannon`
- Mean: 162.139 s +/- 27.509 s, 3 runs
- Artifact: `baseline_hyperfine.txt`, `baseline_hyperfine.json`

RCH hyperfine after:

- Same command.
- Mean: 125.515 s +/- 2.151 s, 3 runs
- Artifact: `after_ordered_simd_hyperfine.txt`, `after_ordered_simd_hyperfine.json`

Focused RCH rows:

| n | baseline ms/call | after ms/call | speedup |
|---:|---:|---:|---:|
| 256 | 0.005708 | 0.002911 | 1.96x |
| 1024 | 0.025082 | 0.012827 | 1.96x |
| 4096 | 0.101914 | 0.049398 | 2.06x |
| 16384 | 0.422666 | 0.200958 | 2.10x |

Score: `Impact 4 * Confidence 5 / Effort 2 = 10.0`, keep.

## Golden Parity

Golden payload strips timing and keeps deterministic checksum lines only.

`sha256sum -c golden_payload.sha256`:

```text
baseline_golden_payload.txt: OK
after_ordered_simd_golden_payload.txt: OK
```

Shared SHA-256:

```text
5ab6b7d54fc5a8e16d496e97cb7ee1211ac17d2abeb22c65299c46bdc7e3caf7
```

`golden_payload.diff` is 0 bytes.

## Isomorphism Proof

- Ordering preserved: yes. The loop still visits elements in increasing index
  order; each 8-lane block is converted to an array and added lane by lane.
- Tie-breaking unchanged: N/A. The routine has no ordering/tie branch.
- Floating point: old result bits preserved by
  `jensenshannon_simd_matches_old_normalized_vec_bits`, which compares against
  the former normalized-Vec implementation over mixed sizes and base values.
- RNG seeds: N/A. No RNG in production or benchmark input generation.
- Error behavior: input length, empty input, non-positive/infinite mass, base
  handling, and final sqrt path are unchanged.

## Validation

- RCH focused proof: `proof_jensenshannon_bits_rch.txt`
- RCH perf rows: `baseline_perf_rows_rch.txt`, `after_ordered_simd_perf_rows_rch.txt`
- Golden SHA check: `golden_payload_sha256_check.txt`

Completed gates:

- RCH `cargo test -j 1 -p fsci-spatial --lib jensenshannon --locked -- --nocapture`:
  passed, 7 tests.
- RCH `cargo check -j 1 -p fsci-spatial --all-targets --locked`: passed.
- RCH `cargo clippy -j 1 -p fsci-spatial --all-targets --no-deps --locked -- -D warnings`:
  passed.
- Local `rustfmt --edition 2024 --check` on touched Rust files: passed.
- `git diff --check` on touched files: passed.
- `ubs crates/fsci-spatial/src/lib.rs crates/fsci-spatial/src/bin/perf_cdist.rs`:
  0 critical issues.
