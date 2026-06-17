# frankenscipy-psn7x.17 Evidence - Rejected Bounded Rotation Packets

## Target

- Bead: `frankenscipy-psn7x.17`
- Crate: `fsci-linalg`
- Profile-backed route: successor to `frankenscipy-psn7x.16`, which proved a progressive lower-band bulge chase but regressed because the full active lower-envelope storage was too expensive.
- Environment: local cargo + hyperfine with `RCH_REQUIRE_REMOTE=0`; `ts1`/remote RCH path is offline by directive.

## Baseline

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-psn7x17-target hyperfine --warmup 1 --runs 3 --show-output 'cargo test -j 1 -p fsci-linalg --lib lower_band_fused_frontier_chase_perf_probe --release --locked -- --ignored --nocapture'
```

Baseline transcript: `baseline_fused_frontier_local_hyperfine.txt`.

Local hyperfine wall time: `314.7 ms +/- 23.7 ms`.

Internal compact frontier timings:

| shape | bandwidth | compact ms range | speedup range vs dense |
| --- | ---: | ---: | ---: |
| 128x128 | 32 | `4.703175`-`7.015850` | `1.737561x`-`2.353467x` |
| 256x256 | 32 | `14.562285`-`16.627852` | `1.613216x`-`1.680296x` |
| 512x512 | 32 | `27.395413`-`34.545187` | `1.519917x`-`1.995336x` |

Baseline digests stayed stable:

- 128 compact digest: `0x325d2bcbcda2d434`
- 256 compact digest: `0xcf9442ea75930712`
- 512 compact digest: `0xd5488334a24b80bc`

## Candidate

Temporary source lever:

- Added a bounded `LowerBandRotationPacket` representation for adjacent Givens rotations.
- Applied compact-envelope updates through direct packet index ranges instead of repeated envelope lookup helpers.
- Replayed Q metadata through packetized row rotations.
- Routed only the private lower-band frontier proof helper through the packet path; no public API or dispatch changed.

## Proof

Focused release proof passed before the rebench:

```text
cargo test -j 1 -p fsci-linalg --lib lower_band_fused_frontier_chase_matches_dense_oracle --release --locked -- --nocapture
```

Observed proof rows:

```text
lower_band_fused_chase n=18 bandwidth=4 cols=7 rotations=3 transformed_drift=7.10542735760100186e-15 eigenvector_drift=3.46944695195361419e-18 compact_digest=0x1c3ef0d9d3dc1083
lower_band_fused_chase n=37 bandwidth=8 cols=13 rotations=4 transformed_drift=2.84217094304040074e-14 eigenvector_drift=2.22044604925031308e-16 compact_digest=0x8a89e08b8c6dc6dd
lower_band_fused_chase n=64 bandwidth=12 cols=17 rotations=5 transformed_drift=5.68434188608080149e-14 eigenvector_drift=6.93889390390722838e-18 compact_digest=0xf1534a90714d38a0
```

Behavior verdict: private proof-clean. Public ordering, tie-breaking, floating-point public contract, RNG behavior, and safe-Rust constraints were untouched because the candidate only changed private ignored proof helpers.

## Rebench

After transcript: `after_packet_fused_frontier_local_hyperfine.txt`.

Local hyperfine wall time regressed: `314.7 ms +/- 23.7 ms -> 342.3 ms +/- 21.8 ms`.

Internal compact frontier timings:

| shape | bandwidth | baseline compact ms range | packet compact ms range | verdict |
| --- | ---: | ---: | ---: | --- |
| 128x128 | 32 | `4.703175`-`7.015850` | `5.020355`-`6.221312` | mixed |
| 256x256 | 32 | `14.562285`-`16.627852` | `18.494764`-`22.150393` | regressed |
| 512x512 | 32 | `27.395413`-`34.545187` | `27.899600`-`38.421572` | mixed/regressed |

Candidate digests matched the baseline compact digests for all measured shapes.

## Score

- Impact: `0.0` because the wall time and the 256/512 compact kernels regressed.
- Confidence: `4.0` because the same local command and warmed target directory were used.
- Effort: `1.0`.
- Score: `0.0`.

Verdict: REJECTED / NO-SHIP. Source restored; `git diff -- crates/fsci-linalg/src/lib.rs` is empty after restore.

## Next Primitive

Do not retry direct-index bounded packet wrappers around the current independent frontier rotations. The next route should implement a true DSBTRD-style rotation batch over compact diagonal/off-diagonal/envelope lanes: a DLAR2V/DLARTV-class packet schedule that batches adjacent rotations by affected diagonal bands and avoids materializing or scanning the full active lower envelope.
