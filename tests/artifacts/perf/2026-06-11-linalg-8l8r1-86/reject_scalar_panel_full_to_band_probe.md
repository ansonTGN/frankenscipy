# frankenscipy-8l8r1.86 scalar-panel full-to-band probe rejection

Date: 2026-06-11
Agent: TopazGorge
Worker: `vmi1227854`

## Target

`frankenscipy-8l8r1.86` targeted the measured `eigh_dense` residual after the
lower-triangle packing route failed Score >= 2.0. The requested first slice was
a private blocked full-to-band symmetric reduction with accumulated orthogonal
transforms, no public dispatch, and proof/perf evidence before retention.

The candidate tested a private safe-Rust full-to-band reducer with semi-bandwidth
32, panel width 8, explicit `Q` accumulation, and a band-structure proof probe.
Public `eigh`, `eigvalsh`, output sorting, validation, tracing, and nalgebra
fallback were untouched during the candidate.

## Baseline

Fresh pre-edit Criterion baseline:

```text
artifact: tests/artifacts/perf/2026-06-11-linalg-8l8r1-86/baseline_eigh_dense_rch_retry2.txt
sha256: 77ffa06fdc6336e3f19ce993917e98c2101d05f9bfeaafebd3e24f2f493f88a8
worker: vmi1227854
eigh_dense/256x256 mean: 13.844 ms
eigh_dense/512x512 mean: 110.14 ms
```

Public golden pre-proof:

```text
artifact: tests/artifacts/perf/2026-06-11-linalg-8l8r1-86/proof_public_eigh_golden_rch.txt
sha256: 58b623e2e51891e5487202579d4efdda0cc1109ae55c23db6e053dc302cf0bdb
digest: eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a
```

## Candidate proof

Focused RCH proof:

```text
artifact: tests/artifacts/perf/2026-06-11-linalg-8l8r1-86/proof_blocked_full_to_band_probe_rch.txt
sha256: 83cfcd92c8bc82ba065d84d2fa4d8a1faf5e305f025173df2cf9012e288bd9a6
```

Result:

- 2 non-ignored private proof tests passed.
- 1 ignored perf probe was listed and left for release-mode timing.
- The probe preserved `B = Q^T A Q` within tolerance, `Q` orthogonality, and
  zero outside-band entries for deterministic 8/17/33 fixtures.
- Fixture digests:
  - `n=8, bandwidth=2`: `0x749f0e316ee77f15`
  - `n=17, bandwidth=4`: `0x60bf85844bc738f6`
  - `n=33, bandwidth=5`: `0xac608234e586d56e`

## Release perf probe

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 \
  rch exec -- env CARGO_BUILD_JOBS=1 \
  cargo test --release -j 1 -p fsci-linalg --lib --locked -- \
    blocked_symmetric_full_to_band_perf_probe --ignored --nocapture
```

Artifact:

```text
tests/artifacts/perf/2026-06-11-linalg-8l8r1-86/after_blocked_full_to_band_perf_probe_rch.txt
sha256: b9edacff48fd082f76d111005867df799e502abe3a4a346c118583b6e72ee95f
worker: vmi1227854
```

Stage timings:

| shape | public `eigh` side-probe | private full-to-band stage | stage ratio vs public |
| --- | ---: | ---: | ---: |
| `256x256` | `21.288556 ms` | `19.061320 ms` | `1.116846x` |
| `512x512` | `105.830953 ms` | `166.288718 ms` | `0.636429x` |

Invariant measurements:

| shape | outside band max | similarity residual | Q orthogonality |
| --- | ---: | ---: | ---: |
| `256x256` | `0.0` | `4.536850639075586e-15` | `4.662936703425657e-15` |
| `512x512` | `0.0` | `5.569593505967783e-15` | `6.217248937900877e-15` |

## Verdict

Rejected. The candidate is proof-clean, but the 512 stage is slower than public
`eigh` before solving the band problem or backtransforming eigenvectors. It
therefore fails the Score >= 2.0 keep gate and the `.86` target direction.

Score:

```text
(Impact 0.5 * Confidence 4) / Effort 4 = 0.5 < 2.0
```

Source retained: no. `git diff -- crates/fsci-linalg/src/lib.rs | wc -l`
returned `0` after restoring the candidate.

## Next primitive

Do not repeat scalar reflector replay, panel-chunked scalar full-to-band, direct
scalar tridiagonalization, output materialization, or packing-only wedges.

Next route: true compact-WY blocked symmetric reduction with BLAS-3-style far
updates. The first child bead should prove and time the compact panel kernel
itself: build `V/T/Y` panel state, apply the trailing update as rank-k symmetric
matrix operations, and compare against this rejected scalar-panel route on
`512x512` before any public dispatch.
