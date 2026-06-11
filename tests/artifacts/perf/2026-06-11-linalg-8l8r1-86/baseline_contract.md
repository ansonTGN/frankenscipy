# frankenscipy-8l8r1.86 baseline and proof contract

Bead: `frankenscipy-8l8r1.86`

## Coordination

- Claimed `frankenscipy-8l8r1.86` as `in_progress` for `TopazGorge`.
- Agent Mail reservations were granted for:
  - `crates/fsci-linalg/src/**`
  - `crates/fsci-linalg/benches/**`
  - `tests/artifacts/perf/2026-06-11-linalg-8l8r1-86/**`
  - `.skill-loop-progress.md`
  - `.beads/**`
- Agent Mail message send to `BlackThrush` failed because the daemon HTTP
  endpoint refused the request while a CLI lock was present. The Beads claim,
  comment, and file reservations are the coordination record for this pass.

## Baseline command

Executed from clean detached worktree:

```text
/data/projects/.scratch/frankenscipy-8l8r1-86-baseline-20260611T1407
HEAD 78be731f116761f946dc0107423e707c630a0d8c
```

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 \
  rch exec -- env CARGO_BUILD_JOBS=1 \
  cargo bench -j 1 -p fsci-linalg --bench linalg_bench --locked -- eigh_dense
```

Two initial attempts were remote-required refusals because no admissible worker
was available. The successful baseline used worker `vmi1227854`.

## Baseline results

Artifact:

```text
tests/artifacts/perf/2026-06-11-linalg-8l8r1-86/baseline_eigh_dense_rch_retry2.txt
sha256: 77ffa06fdc6336e3f19ce993917e98c2101d05f9bfeaafebd3e24f2f493f88a8
worker: vmi1227854
```

Criterion intervals:

| shape | lower | mean | upper |
| --- | ---: | ---: | ---: |
| `eigh_dense/256x256` | `13.372 ms` | `13.844 ms` | `14.444 ms` |
| `eigh_dense/512x512` | `106.98 ms` | `110.14 ms` | `116.28 ms` |

Historical same-worker reference from `.85` remains useful for trend context,
but `.86` scoring must compare against the fresh baseline above.

## Public golden proof

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 \
  rch exec -- env CARGO_BUILD_JOBS=1 \
  cargo test -j 1 -p fsci-linalg --lib --locked -- \
    eigh_index_sort_matches_materialized_pair_sort_bits --nocapture
```

Artifact:

```text
tests/artifacts/perf/2026-06-11-linalg-8l8r1-86/proof_public_eigh_golden_rch.txt
sha256: 58b623e2e51891e5487202579d4efdda0cc1109ae55c23db6e053dc302cf0bdb
worker: vmi1227854
digest: eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a
```

## Isomorphism contract for the next source lever

- Public validation, finite checks, square-matrix errors, dimension guards, and
  trace behavior must stay unchanged.
- Eigenvalue ordering remains ascending with the same `f64::total_cmp` policy.
- No RNG, unsafe code, or external BLAS/LAPACK/MKL/XLA linkage may be introduced.
- Floating-point tolerance contracts must be preserved; if raw eigenvectors
  differ, residual, orthogonality, and sign/subspace proof must be explicit.
- Before public dispatch, the blocked full-to-band route must live behind a
  private proof/perf probe.
- Keep only with same-worker RCH rebench and Score >= 2.0; target public-dispatch
  threshold is at least `1.25x` on `eigh_dense/512x512`.
