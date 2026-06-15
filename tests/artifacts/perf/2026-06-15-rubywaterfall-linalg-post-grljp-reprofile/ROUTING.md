# Post-grljp linalg reprofile

Agent: RubyWaterfall
Date: 2026-06-15
Head: `401fa419`
Target: `fsci-linalg` dense symmetric `eigh`

## Public route timing

RCH `ovh-a`, transcript: `public_native_route_rch.txt`.

| n | routed native `eigh` | nalgebra `symmetric_eigen` | speedup | values digest |
| ---: | ---: | ---: | ---: | --- |
| 400 | 36.914117 ms | 47.840413 ms | 1.295992x | `0x0dbbde75b75c8612` |
| 800 | 244.354968 ms | 363.460487 ms | 1.487428x | `0xad8a7e5fa1980bfb` |
| 1200 | 870.622551 ms | 1182.259198 ms | 1.357947x | `0x181b3486089d0e4a` |

The values digests match the earlier native-route proof, so this is a routing/perf refresh rather than a behavior change.

## Criterion baseline

RCH `ovh-a`, transcript: `baseline_eigh_dense_criterion_rch.txt`.

| Benchmark | Criterion interval |
| --- | ---: |
| `eigh_dense/256x256` | [12.883 ms, 12.966 ms, 13.149 ms] |
| `eigh_dense/512x512` | [100.81 ms, 101.41 ms, 101.84 ms] |

## Full-to-band replay baseline

RCH `vmi1152480`, transcript: `baseline_compact_wy_full_to_band_replay_rch.txt`.

| Shape | scalar full replay | compact-WY panel replay | compact speedup | max abs diff |
| ---: | ---: | ---: | ---: | ---: |
| 256x256 | 95.825050 ms | 11.595805 ms | 8.263769x | 1.59161572810262442e-12 |
| 512x512 | 239.935447 ms | 123.880705 ms | 1.936827x | 6.13908923696726561e-12 |

This selected `frankenscipy-o3gu7`: the compact-WY update is already much faster than scalar replay, but the full replay still spends 123.880705 ms at 512x512.

## Profiler note

`perf` could not be run through `rch` with `RCH_REQUIRE_REMOTE=1` because the wrapper refuses non-compilation shell commands. Transcript: `perf_probe_capability_rch.txt`.
