# frankenscipy-psn7x.14 Evidence - Compact-Band Q Metadata Replay

## Target

- Bead: `frankenscipy-psn7x.14`
- Crate: `fsci-linalg`
- Profile-backed route: successor to `frankenscipy-psn7x.13`, which proved compact lower-band/envelope frontier rotations and showed local dense-vs-envelope speedups up to `15.66x`. The next measured wall in the broader route is dense Q/backtransform materialization.
- Environment: local cargo + hyperfine with `RCH_REQUIRE_REMOTE=0`; `ts1` RCH path remains offline.

## Baseline

Previous compact-envelope frontier proof (`frankenscipy-psn7x.13`) established the rotation metadata and envelope storage contract:

| shape | bandwidth | dense ms | envelope ms | speedup range |
| --- | ---: | ---: | ---: | ---: |
| 128x128 | 32 | `0.136107`-`0.203726` | `0.073981`-`0.099860` | `1.736768x`-`2.413854x` |
| 256x256 | 32 | `0.764087`-`1.058615` | `0.128323`-`0.187565` | `5.408642x`-`6.322052x` |
| 512x512 | 32 | `5.309596`-`6.833492` | `0.344172`-`0.482473` | `11.687908x`-`15.658180x` |

Public `eig_banded` behavior after `psn7x.13` remained unchanged:

| shape | bandwidth | values digest | vectors digest |
| --- | ---: | --- | --- |
| 128x128 | 32 | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

## Lever

One source lever was kept: add an isolated compact-band Q metadata replay harness.

The harness compares:

- Dense Q oracle: materialize Q from the frontier rotations, then compute `Q * V`.
- Metadata replay candidate: apply the stored frontier rotations directly to eigenvector rows, in reverse order, without materializing dense Q.

This is proof/probe code only. It does not change public `eig_banded`, thresholds, sorting, fallback selection, RNG behavior, or any user-visible numerical route.

## Proof

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target cargo test -j 1 -p fsci-linalg --lib lower_band_frontier_q_metadata_replay_matches_dense_q --release --locked -- --nocapture
```

Proof output:

```text
lower_band_q_replay n=18 bandwidth=4 cols=7 rotations=3 max_abs=3.46944695195361419e-18 replay_digest=0x516ba55735b6f73f
lower_band_q_replay n=37 bandwidth=8 cols=13 rotations=4 max_abs=2.22044604925031308e-16 replay_digest=0xe454bbf9fe2b4d1a
lower_band_q_replay n=64 bandwidth=12 cols=17 rotations=5 max_abs=6.93889390390722838e-18 replay_digest=0x3ddf781e354fd1ec
```

Golden output artifact:

```text
tests/artifacts/perf/2026-06-16-rubywaterfall-linalg-psn7x14-dsbtrd-q-replay/lower-band-q-metadata-replay-golden-output.txt
```

Golden output sha256: `6a839584907a6f3eb2db11665d40f54f98d50d291087d198e5eb8ad6735ff100`

Proof obligations:

- Metadata replay matched dense `Q * V` within tolerance.
- Materialized Q orthogonality passed.
- Ordering/tie-breaking unchanged: no public eigenvalue/eigenvector ordering path changed.
- Floating point: public route unchanged; replay has an explicit tolerance contract against dense Q materialization.
- RNG unchanged: deterministic fixtures only.
- Safety unchanged: safe Rust only; no C BLAS/LAPACK/MKL/XLA linkage.

## Rebench

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target hyperfine --warmup 1 --runs 5 --show-output 'cargo test -j 1 -p fsci-linalg --lib lower_band_frontier_q_metadata_replay_perf_probe --release --locked -- --ignored --nocapture'
```

Hyperfine wall time: `301.4 ms +/- 29.6 ms` over 5 runs.

Internal probe ranges:

| shape | bandwidth | dense Q ms | replay ms | speedup range | max abs diff | dense digest | replay digest |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | `8.926597`-`11.707374` | `0.636535`-`0.747957` | `13.910315x`-`16.439270x` | `1.11022302462515654e-16` | `0xa2bd002a5d42ffd5` | `0xd69819b0d69819b0` |
| 256x256 | 32 | `22.066475`-`32.794483` | `1.599640`-`2.420104` | `12.763414x`-`14.307625x` | `2.22044604925031308e-16` | `0x6836faf442d6d014` | `0x63cad5274b5b0249` |
| 512x512 | 32 | `63.940534`-`72.171474` | `4.058277`-`4.333869` | `14.983161x`-`17.221173x` | `2.22044604925031308e-16` | `0x5916bc422a7537d9` | `0x88ceaa321f71a2ed` |

## Score

- Impact: `3.0` (metadata replay replaces dense Q materialization/multiply in the harness and is >12x faster at all measured sizes).
- Confidence: `4.0` (dense-Q oracle proof, Q orthogonality, deterministic digests, repeated local hyperfine).
- Effort: `1.0`.
- Score: `12.0`.

Verdict: KEEP.

## Next Profile Route

Re-profile after this proof slice. The next primitive should integrate compact frontier rotations, D/E emission, and Q metadata replay into one tridiagonal chase harness before public `eig_banded` wiring.
