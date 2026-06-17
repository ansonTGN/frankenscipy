# `frankenscipy-8l8r1.82` Baseline Contract

Bead: `frankenscipy-8l8r1.82`
Target: `fsci-linalg::matmul` flat-workspace GEMM register tile.

## Baseline

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench --locked -- matmul
```

Artifact:

```text
acf22e56e9a0b312c597f289f273f48c92158331ce4c1077b38076be7d3b47c0  tests/artifacts/perf/2026-06-11-linalg-8l8r1-82/baseline_matmul_criterion_rch.txt
```

RCH worker: `vmi1227854`.

Criterion means:

| Benchmark | Mean |
|---|---:|
| `matmul/256x256` | `6.3882 ms` |
| `matmul/512x512` | `79.521 ms` |
| `matmul/768x768` | `136.32 ms` |
| `matmul/1024x1024` | `284.08 ms` |

The 256 row is a sentinel below the flat-workspace dispatch gate. The score gate is based on 512+ affected rows, with same-worker comparison required.

## Alien Route

Mapped graveyard sections:

- `alien_cs_graveyard.md` section 0.12 classifies numeric kernels by cache locality, SIMD, and numerical proof obligations.
- `alien_cs_graveyard.md` section 6.5 maps affine loop nests to locality-preserving tiling/interchange.
- `alien_cs_graveyard.md` section 9.6 names dense submatrix BLAS-3 kernels as the inner primitive for communication-avoiding linear algebra.
- The FrankenSuite summary risk table calls out "constants kill you", so this lever is benchmark gated and will be restored if constants lose.

Candidate card:

```text
Change: widen the existing full flat-workspace register tile from 4x16 to 4x24 over three packed-B NR panels.
Hotspot evidence: current `matmul/512+` Criterion rows dominate the linalg GEMM bead and prior `.81` compute-spelling levers failed.
EV score: Impact 4 * Confidence 3 / Effort 2 = 6.0.
Priority: A, but only behind the same-worker Score >= 2.0 keep gate.
Fallback: restore source and close `.82` rejected if any proof fails or affected geomean ratio is <= 1.0 / Score < 2.0.
Baseline comparator: current 4x16 flat-workspace register tile on `vmi1227854`.
```

## Isomorphism Plan

- Ordering/tie behavior: GEMM has no comparison or tie surface; public row/column order remains unchanged.
- Floating point: each output cell must still execute the same monotonic `k = 0..ka` multiply-add sequence.
- RNG: none.
- Shape and error behavior: public rectangularity and shape gates remain unchanged.
- Golden outputs: run matmul bit proofs plus `matmul_medium_flat_workspace_route_golden_digest`; hash proof artifacts with `sha256sum`.
