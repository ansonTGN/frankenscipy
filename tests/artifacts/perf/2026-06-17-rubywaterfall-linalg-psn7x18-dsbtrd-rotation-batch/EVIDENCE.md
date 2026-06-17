# frankenscipy-psn7x.18 - DSBTRD Diagonal-Lane Envelope Update

Date: 2026-06-17
Agent: RubyWaterfall
Target: `fsci-linalg` compact lower-band frontier chase harness
Worker: local only, per ts1/RCH-offline override

## Lever

One source lever was retained in `crates/fsci-linalg/src/lib.rs`: route the compact lower-band envelope through direct diagonal-lane storage for both adjacent-rotation updates and transformed dense emission.

The previous helper called `index()`/`get()` for every rotated pair and every dense output cell. The retained path preserves the same lower-band layout but computes the lane offsets directly:

- left of the active adjacent pair: update `(p, col)` and `(q, col)` by diagonal distance
- right of the active adjacent pair: update `(row, p)` and `(row, q)` by diagonal distance
- transformed storage emission: fill dense symmetric output by stored envelope diagonals, not by probing every dense coordinate

No public API changed. No RNG is introduced. Rotation order remains the original `rotations` order. Ordering and tie-breaking are not touched. Floating-point expressions for each Givens pair use the same scalar formulas as the dense oracle.

## Baseline

Command:

```bash
env RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-psn7x17-target \
  hyperfine --warmup 1 --runs 5 --show-output \
  'cargo test -j 1 -p fsci-linalg --lib lower_band_fused_frontier_chase_perf_probe --release --locked -- --ignored --nocapture'
```

Transcript: `baseline_fused_frontier_local_hyperfine.txt`

Result:

- Wall: `328.9 ms +/- 11.1 ms`
- 128x128 compact range in transcript: `4.724442-5.949203 ms`
- 256x256 compact range in transcript: `14.705564-17.438192 ms`
- 512x512 compact range in transcript: `30.157252-34.161409 ms`
- Compact digests stayed:
  - 128: `0x325d2bcbcda2d434`
  - 256: `0xcf9442ea75930712`
  - 512: `0xd5488334a24b80bc`

## Proof

Command:

```bash
env RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-psn7x17-target \
  cargo test -j 1 -p fsci-linalg --lib lower_band_fused_frontier_chase_matches_dense_oracle --release --locked -- --nocapture
```

Transcript: `proof_diagonal_lanes_and_emission_dense_oracle.txt`

Result: passed.

- n=18: transformed drift `7.10542735760100186e-15`, eigenvector drift `3.46944695195361419e-18`, digest `0x1c3ef0d9d3dc1083`
- n=37: transformed drift `2.84217094304040074e-14`, eigenvector drift `2.22044604925031308e-16`, digest `0x8a89e08b8c6dc6dd`
- n=64: transformed drift `5.68434188608080149e-14`, eigenvector drift `6.93889390390722838e-18`, digest `0xf1534a90714d38a0`

The final perf probe also rechecked transformed/eigenvector drift on each run:

- transformed drift: `1.13686837721616030e-13` for 128 and 256, `2.27373675443232059e-13` for 512
- eigenvector drift: `6.93889390390722838e-18` for 128, `1.11022302462515654e-16` for 256 and 512

## After

Command: same as baseline.

Transcript: `after_diagonal_lanes_and_emission_fused_frontier_local_hyperfine.txt`

Result:

- Wall: `314.3 ms +/- 6.3 ms`
- Wall delta: `328.9 -> 314.3 ms`, `1.046x` faster
- 128x128 compact range in transcript: `4.589014-5.859792 ms`
- 256x256 compact range in transcript: `14.343385-19.674266 ms`
- 512x512 compact range in transcript: `26.602222-31.144187 ms`
- Compact digests stayed:
  - 128: `0x325d2bcbcda2d434`
  - 256: `0xcf9442ea75930712`
  - 512: `0xd5488334a24b80bc`

Score:

- Impact: `1.046`
- Confidence: `4.0`
- Effort: `1.5`
- Score: `2.79`
- Verdict: keep

## Validation

Passed:

- `git diff --check`
- `cargo test -j 1 -p fsci-linalg --lib lower_band_fused_frontier_chase_matches_dense_oracle --release --locked -- --nocapture`
- `cargo check -j 1 -p fsci-linalg --tests --locked`
- `cargo clippy -j 1 -p fsci-linalg --lib --tests --locked --no-deps`
- `ubs crates/fsci-linalg/src/lib.rs` exited 0, with no critical issues

Known pre-existing blockers, not introduced by this lever:

- Workspace `cargo fmt --check` fails on unrelated `fsci-cluster` and `fsci-stats` formatting drift.
- `cargo fmt -p fsci-linalg --check` fails on existing bin/cossin formatting drift outside this edit.
- Strict `cargo clippy -j 1 -p fsci-linalg --lib --tests --locked -- -D warnings` is blocked before this crate by `crates/fsci-fft/src/helpers.rs:58` unused variable `total`.
- Non-deny clippy still reports existing `fsci-linalg::cossin` and perf-bin warnings; related hygiene beads already exist (`frankenscipy-8ykh7`, `frankenscipy-6eqgx`).

## Artifacts

- `baseline_fused_frontier_local_hyperfine.txt`
- `proof_diagonal_lanes_dense_oracle.txt`
- `after_diagonal_lanes_fused_frontier_local_hyperfine.txt`
- `after_diagonal_lanes_envelope_subkernel.txt`
- `proof_diagonal_lanes_and_emission_dense_oracle.txt`
- `after_diagonal_lanes_and_emission_fused_frontier_local_hyperfine.txt`
- `evidence_checksums.sha256`
