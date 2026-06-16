# frankenscipy-psn7x.13 Evidence - Compact-Envelope DSBTRD Frontier

## Target

- Bead: `frankenscipy-psn7x.13`
- Crate: `fsci-linalg`
- Profile-backed route: successor to `frankenscipy-psn7x.12`, where the isolated lower-band rank-2 oracle harness proved `Q^T A Q = T` and showed local scalar-oracle speedups. The next bottleneck is replacing dense-harness expansion with compact lower-band/envelope DSBTRD storage before public `eig_banded` wiring.
- Environment: local cargo + hyperfine with `RCH_REQUIRE_REMOTE=0`; `ts1` RCH path remains offline.

## Baseline

Previous kept oracle harness (`frankenscipy-psn7x.12`) established the local scalar dense-similarity baseline and proof gate:

| shape | bandwidth | scalar oracle ms | rank-2 oracle ms | speedup range |
| --- | ---: | ---: | ---: | ---: |
| 128x128 | 32 | `3.650413`-`4.235000` | `1.809587`-`2.319893` | `1.732119x`-`2.017263x` |
| 256x256 | 32 | `22.655886`-`24.770361` | `14.590632`-`17.321955` | `1.384673x`-`1.637072x` |
| 512x512 | 32 | `177.444358`-`188.801557` | `130.346534`-`148.291297` | `1.273180x`-`1.427941x` |

Public `eig_banded` behavior after `psn7x.12` remained unchanged:

| shape | bandwidth | values digest | vectors digest |
| --- | ---: | --- | --- |
| 128x128 | 32 | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

## Lever

One source lever was kept: add an isolated compact lower-band/envelope frontier harness.

The harness introduces:

- `LowerBandEnvelopeProbe`: contiguous lower-band/envelope storage with explicit width.
- Dense oracle: full two-sided adjacent rotations `A := G^T A G`.
- Compact candidate: envelope-local adjacent rotations that update only the active bulge frontier.
- Q proof: accumulated rotation matrix verifies `Q^T A Q` against the dense oracle.

This is still proof/probe code only. It does not change public `eig_banded`, thresholds, sorting, fallback selection, RNG behavior, or any user-visible numerical route.

## Proof

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target cargo test -j 1 -p fsci-linalg --lib lower_band_envelope_frontier_rotations_match_dense_oracle --release --locked -- --nocapture
```

Proof output:

```text
lower_band_envelope_frontier n=18 bandwidth=4 rotations=3 max_abs=7.10542735760100186e-15 q_residual=7.10542735760100186e-15 envelope_digest=0x11992ad9528bab17
lower_band_envelope_frontier n=37 bandwidth=8 rotations=4 max_abs=2.84217094304040074e-14 q_residual=1.42108547152020037e-14 envelope_digest=0xd37e608b43f6f070
lower_band_envelope_frontier n=64 bandwidth=12 rotations=5 max_abs=5.68434188608080149e-14 q_residual=2.84217094304040074e-14 envelope_digest=0x4a42d14f0fdd8bd0
```

Golden output artifact:

```text
5e71d00e7056ecdd89d3d4ce1acd94faab3c1407f31e32f7623b7f92fcea98e8  tests/artifacts/perf/2026-06-16-rubywaterfall-linalg-psn7x13-dsbtrd-envelope/lower-band-envelope-frontier-golden-output.txt
```

Proof obligations:

- Compact envelope output matched dense two-sided rotations within tolerance.
- Dense oracle stayed inside the explicit envelope width.
- Accumulated Q satisfied `Q^T A Q = A_rotated` within tolerance.
- Q orthogonality passed.
- Ordering/tie-breaking unchanged: no public eigenvalue/eigenvector ordering path changed.
- Floating point: public route unchanged; the envelope frontier has an explicit tolerance contract against the dense oracle.
- RNG unchanged: deterministic fixtures only.
- Safety unchanged: safe Rust only; no C BLAS/LAPACK/MKL/XLA linkage.

## Rebench

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target hyperfine --warmup 1 --runs 7 --show-output 'cargo test -j 1 -p fsci-linalg --lib lower_band_envelope_frontier_rotation_perf_probe --release --locked -- --ignored --nocapture'
```

Hyperfine wall time: `165.6 ms +/- 6.3 ms` over 7 runs.

Internal probe ranges:

| shape | bandwidth | dense ms | envelope ms | speedup range | max abs diff | dense digest | envelope digest |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | `0.136107`-`0.203726` | `0.073981`-`0.099860` | `1.736768x`-`2.413854x` | `1.19371179607696831e-12` | `0xb140a829862f372c` | `0x2f5c5316092aa29d` |
| 256x256 | 32 | `0.764087`-`1.058615` | `0.128323`-`0.187565` | `5.408642x`-`6.322052x` | `3.63797880709171295e-12` | `0xd776a4456bccc16d` | `0x17bd69b191265bed` |
| 512x512 | 32 | `5.309596`-`6.833492` | `0.344172`-`0.482473` | `11.687908x`-`15.658180x` | `8.64019966684281826e-12` | `0xd75092c23f6da1ef` | `0x32e784d545f458c6` |

## Score

- Impact: `3.0` (compact envelope frontier is substantially faster than dense two-sided rotations and supplies the missing DSBTRD storage primitive).
- Confidence: `4.0` (dense-oracle proof, `Q^T A Q`, Q orthogonality, deterministic digests, repeated local hyperfine).
- Effort: `1.0`.
- Score: `12.0`.

Verdict: KEEP.

## Next Profile Route

Re-profile after this proof slice. The next primitive should extend the frontier from independent rotations into a real chase sequence that emits tridiagonal D/E and replayable Q metadata, still behind the scalar-oracle harness before public `eig_banded` integration.
