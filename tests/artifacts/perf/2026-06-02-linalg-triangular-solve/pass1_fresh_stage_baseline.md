# Pass 1 - Fresh Stage Baseline

Date: 2026-06-02T01:52:00-0400
Agent: OliveSnow
Target bead: `frankenscipy-perf-linalg-directlu-triangular-solve-v82ao`

## Mission

Reconfirm the DirectLU triangular solve stage with a fresh remote
`release-perf` baseline, deterministic golden output, and focused samples
before any Rust or Cargo edit. This pass did not edit Rust/Cargo code, did not
commit, did not push, and did not close the bead.

## Remote Hyperfine Baseline

Command shape:

```bash
RCH_FORCE_REMOTE=1 rch exec -- hyperfine \
  --setup 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_tri_pass1_bench_20260602b RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve --locked' \
  --warmup 3 \
  --runs 10 \
  --export-json tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass1_baseline_rch.json \
  '/tmp/rch_target_fsci_linalg_tri_pass1_bench_20260602b/release-perf/perf_solve solve 1000 1 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass1_bench_20260602b/release-perf/perf_solve lu_factor 1000 1 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass1_bench_20260602b/release-perf/perf_solve lu_solve 1000 1 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass1_bench_20260602b/release-perf/perf_solve solve_triangular 1000 1 42'
```

| mode | mean | stddev | median | min | max | user | system |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `solve 1000 1 42` | 122.439 ms | 6.262 ms | 121.337 ms | 113.765 ms | 134.902 ms | 93.330 ms | 28.732 ms |
| `lu_factor 1000 1 42` | 99.199 ms | 6.422 ms | 99.128 ms | 89.657 ms | 113.017 ms | 85.191 ms | 13.574 ms |
| `lu_solve 1000 1 42` | 118.534 ms | 10.170 ms | 116.207 ms | 107.930 ms | 142.353 ms | 94.293 ms | 23.839 ms |
| `solve_triangular 1000 1 42` | 10.121 ms | 0.694 ms | 10.131 ms | 8.591 ms | 10.866 ms | 4.410 ms | 5.586 ms |

Stage arithmetic:

- `lu_solve - lu_factor` mean delta: 19.335 ms. This includes post-factor
  solve work plus `fast_rcond_from_lu` inside `lu_solve`.
- `solve - lu_factor` mean delta: 23.240 ms. This is not a pure DirectLU
  triangular solve measurement because full `solve` also performs condition
  diagnostics, portfolio dispatch, and backward-error work.
- Public `solve_triangular 1000 1 42` mean: 10.121 ms. This confirms the
  standalone triangular solver is measurable, but it is not the same call path
  as nalgebra `LU::solve(&rhs)`.

## Golden Proof

Golden command shape:

```bash
RCH_FORCE_REMOTE=1 rch exec -- zsh -lc 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_tri_pass1_golden_20260602c RUSTFLAGS="-C force-frame-pointers=yes" cargo run -p fsci-linalg --profile release-perf --bin perf_solve --locked -- golden > tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass1_golden.txt'
sha256sum tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass1_golden.txt > tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass1_golden.sha256
sha256sum -c tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass1_golden.sha256
```

Result:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass1_golden.txt
```

The checksum file verified successfully. Since this pass changed no code or
configuration, ordering, tie-breaking, floating-point operation order, RNG
seeds, warnings, certificates, and `backward_error` are unchanged by
construction.

## Focused Profile Samples

Raw artifacts:

- `pass1_gdb_solve_samples.txt`: 40 gdb samples over `perf_solve solve 1000 200 42`.
- `pass1_gdb_lu_solve_samples.txt`: 40 gdb samples over `perf_solve lu_solve 1000 200 42`.
- `pass1_profile_counts.txt`: reduced sample-block counts.

Additional `rch`-retrieved pass-1 files in this directory were preserved for
audit continuity but were not used for the decision below:
`pass1_golden_rch_raw.txt`, `pass1_profile_samples.txt`,
`pass1_profile_lu_solve_run.txt`, `pass1_profile_solve_triangular_run.txt`,
and `pass1_profile_solve_triangular_long_run.txt`.

Full `solve` profile counts:

| signal | samples |
| --- | ---: |
| total samples | 40 |
| `condition_diagnostics_with_assumption` | 39 |
| nalgebra `gauss_step` | 31 |
| `array_axcpy` / `axcpy_uninit` / `blas_uninit` | 30 |
| `fast_rcond_from_lu` | 8 |
| DirectLU dispatch triangular solve | 1 |
| `compute_backward_error` | 0 |
| matrix copy setup | 0 |
| `matrixmultiply` / GEMM | 0 |

`lu_solve` profile counts:

| signal | samples |
| --- | ---: |
| total samples | 40 |
| `array_axcpy` / `axcpy_uninit` / `blas_uninit` | 31 |
| nalgebra `gauss_step` | 30 |
| `fast_rcond_from_lu` | 8 |
| rcond triangular solve | 1 |
| DirectLU dispatch triangular solve | 0 |
| `compute_backward_error` | 0 |
| matrix copy setup | 2 |
| `matrixmultiply` / GEMM | 0 |

Representative full-solve DirectLU triangular sample:

```text
nalgebra::base::matrix::Matrix<...>::solve_upper_triangular_vector_mut
fsci_linalg::dispatch_solve_action (... SolverAction::DirectLU ...) at crates/fsci-linalg/src/lib.rs:1500
fsci_linalg::solve_with_portfolio_internal::{closure#2} at crates/fsci-linalg/src/lib.rs:1617
```

## Interpretation

Fresh evidence refutes treating the final DirectLU `LU::solve(&rhs)` triangular
solve as the next dominant profile-backed optimization target. The stage is
real and measurable, but in the full-solve gdb profile it appears in only
1/40 samples. The dominant sampled work remains the condition-diagnostics LU
factorization path and nalgebra scalar `gauss_step` / `array_axcpy` frames.

The `lu_solve - lu_factor` delta is still about 19.335 ms, but the focused
samples attribute the visible post-factor work more to `fast_rcond_from_lu`
and nalgebra scalar triangular helper frames than to the final DirectLU
dispatch solve alone.

## Decision

Pass 2 should not proceed as a production optimization of final DirectLU
triangular solve yet. The next safe pass should first isolate a precomputed-LU
solve-only workload, or retarget to a profile-backed post-factor lever in
`fast_rcond_from_lu`, before any production edit. A production lever against
`dispatch_solve_action` `LU::solve(&rhs)` is not sufficiently supported by this
fresh profile.

## Validation

- `sha256sum -c pass1_golden.sha256`: passed.
- `git diff -- crates/fsci-linalg/src/lib.rs crates/fsci-linalg/src/bin/perf_solve.rs`: no diff.
- `jq` parsed `pass1_baseline_rch.json` for the table above.
