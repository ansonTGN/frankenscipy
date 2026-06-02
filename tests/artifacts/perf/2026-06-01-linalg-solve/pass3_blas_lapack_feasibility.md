# Pass 3: BLAS/LAPACK Feasibility

Active bead: `frankenscipy-perf-linalg-lu-scalar-2y3wp`
Mission: check whether a linked `getrf` path is available and measurably improves
the profiled `n=1000` DirectLU scenario without weakening pivoting, tolerance,
observable output, or evidence semantics.

## Baseline

All baseline evidence was collected through `rch` with a crate-scoped
`fsci-linalg` `release-perf` build:

```text
RCH_FORCE_REMOTE=1 rch exec -- bash -lc '
  RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve
  hyperfine --warmup 3 --runs 10 --export-json tests/artifacts/perf/2026-06-01-linalg-solve/pass3_blas_lapack_baseline.json "/data/tmp/cargo-target/release-perf/perf_solve solve 1000 1 42"
  hyperfine --warmup 3 --runs 7 --export-json tests/artifacts/perf/2026-06-01-linalg-solve/pass3_blas_lapack_stage.json "/data/tmp/cargo-target/release-perf/perf_solve lu_factor 1000 1 42" "/data/tmp/cargo-target/release-perf/perf_solve lu_solve 1000 1 42"
  /data/tmp/cargo-target/release-perf/perf_solve golden | sha256sum
'
```

| probe | mean | median | stddev | min..max | approx p95 | user | system |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `solve 1000 1 42` | 125.6 ms | 123.6 ms | 7.6 ms | 118.0..138.8 ms | 138.0 ms | 98.7 ms | 26.5 ms |
| `lu_factor 1000 1 42` | 98.4 ms | 99.8 ms | 6.3 ms | 90.6..107.1 ms | 104.3 ms | 83.8 ms | 14.3 ms |
| `lu_solve 1000 1 42` | 115.1 ms | 114.2 ms | 4.8 ms | 110.4..124.5 ms | 117.1 ms | 91.9 ms | 22.8 ms |

Default golden checksum:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  -
```

## Feasibility Checks

1. `nalgebra` 0.34.2 does not expose a feature that changes
   `DMatrix::lu()` into a LAPACK-backed implementation. The local nalgebra
   source lists `nalgebra-lapack` as a separate workspace crate, and
   `DMatrix::lu()` still returns `nalgebra::LU`.
2. `nalgebra-lapack` 0.27.0 provides its own `nalgebra_lapack::LU` type.
   Its `LU::new` path calls `xgetrf`, and the f64 scalar implementation maps
   that to `lapack::dgetrf`; solves map to `lapack::dgetrs`.
3. Provider selection is mandatory and exclusive. The default feature is
   `lapack-netlib`; `lapack-custom` skips `lapack_src` and requires ABI-compatible
   LAPACK/BLAS symbols at link time.
4. Local `ldconfig` shows runtime `liblapack.so.3` and `libblas.so.3`, but the
   required `rch` worker proof lane could not link `-llapack` or `-lblas`.
5. A direct call-site replacement is not a one-line backend switch. The current
   solve/inverse paths cache `nalgebra::LU`, compute rcond from its `l/u/p`
   accessors, and reuse that type in `lu_factor`, `lu_solve`, and inverse
   dispatch. A product replacement would need a typed backend wrapper plus
   separate behavior proof for rcond, pivot semantics, and `backward_error`.

## Candidate Attempt

A single probe candidate was evaluated with `nalgebra-lapack = "0.27.0"` using
`default-features = false` and `features = ["lapack-custom"]`, plus explicit
remote build link args:

```text
RUSTFLAGS="-C force-frame-pointers=yes -C link-arg=-llapack -C link-arg=-lblas"
cargo build -p fsci-linalg --profile release-perf --bin perf_solve
```

The candidate did not reach golden comparison or benchmark execution. The
remote worker failed while linking build scripts:

```text
rust-lld: error: unable to find library -llapack
rust-lld: error: unable to find library -lblas
```

Because the required `rch` lane cannot link the candidate provider, no linked
`getrf` timing was accepted as evidence and no code/Cargo changes were kept.

## Isomorphism Proof

Change kept: none.

- Ordering preserved: yes, no implementation change kept.
- Tie-breaking unchanged: yes, no implementation change kept.
- Floating point: default golden output unchanged.
- RNG seeds: unchanged; `perf_solve` deterministic seed path unchanged.
- Golden outputs: default checksum stayed
  `5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd`.

The candidate itself has no accepted isomorphism proof because it did not link
on the required remote worker.

## Verdict

No linked-getrf candidate was kept.

Opportunity score for the `nalgebra-lapack`/`lapack-custom` lever in this
project state: `0.5 = Impact 2 x Confidence 1 / Effort 4`.

Rationale: the hotspot is real, but the required remote proof lane lacks a
linkable LAPACK/BLAS provider, and a default-path integration would be a
multi-surface backend replacement rather than a narrow one-lever optimization.
