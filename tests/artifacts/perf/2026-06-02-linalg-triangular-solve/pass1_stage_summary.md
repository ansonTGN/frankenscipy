# Pass 1 - Fresh Stage Baseline

Date: 2026-06-02T01:58:00-04:00
Agent: OliveSnow
Target bead: `frankenscipy-perf-linalg-directlu-triangular-solve-v82ao`

## Scope

This pass re-confirmed the DirectLU triangular solve stage before any
optimization. No Rust or Cargo files were edited, and no bead was closed,
committed, pushed, or reservation-released.

## Baseline

Artifact: `pass1_baseline_rch.json`

| mode | mean | median | stddev | user | system | min | max |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `solve 1000 1 42` | 122.439 ms | 121.337 ms | 6.262 ms | 93.330 ms | 28.732 ms | 113.765 ms | 134.902 ms |
| `lu_factor 1000 1 42` | 99.199 ms | 99.128 ms | 6.422 ms | 85.191 ms | 13.574 ms | 89.657 ms | 113.017 ms |
| `lu_solve 1000 1 42` | 118.534 ms | 116.207 ms | 10.170 ms | 94.293 ms | 23.839 ms | 107.930 ms | 142.353 ms |
| `solve_triangular 1000 1 42` | 10.121 ms | 10.131 ms | 0.694 ms | 4.410 ms | 5.586 ms | 8.591 ms | 10.866 ms |

Stage splits:

- `lu_solve - lu_factor`: 19.335 ms mean.
- `solve - lu_factor`: 23.240 ms mean.
- Standalone `solve_triangular`: 10.121 ms mean under one-call hyperfine.

## Golden

Artifacts:

- `pass1_golden_rch_raw.txt`
- `pass1_golden.txt`
- `pass1_golden.sha256`

Fresh golden sha256:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass1_golden.txt
```

This matches the retained dense-solve campaign hash from
`tests/artifacts/perf/2026-06-01-linalg-solve/pass5_final_golden.txt`.
`sha256sum -c pass1_golden.sha256` passed.

Because this pass made no code/config changes, ordering, tie-breaking,
floating-point operation order, RNG, warnings, certificates, and
`backward_error` are unchanged by construction.

## Profile Evidence

Artifacts:

- `pass1_profile_samples.txt`
- `pass1_profile_counts.txt`
- `pass1_profile_lu_solve_run.txt`
- `pass1_profile_solve_triangular_run.txt`
- `pass1_profile_solve_triangular_long_run.txt`

Reduced sample counts:

```text
lu_solve: total_success=25, failed=0, factorization=22, axcpy=15, lu_solve_or_rcond=2, triangular_internal=0, residual=0, validation=0, gemm=0
solve_triangular_short: total_success=5, failed=15, factorization=0, axcpy=0, lu_solve_or_rcond=0, triangular_internal=3, residual=2, validation=2, gemm=0
solve_triangular_long: total_success=16, failed=9, factorization=0, axcpy=0, lu_solve_or_rcond=0, triangular_internal=13, residual=11, validation=3, gemm=0
```

Representative `lu_solve` stacks are still dominated by nalgebra LU
factorization:

```text
fsci_linalg::lu_factor
nalgebra::Matrix::lu
nalgebra::linalg::lu::LU::new
nalgebra::linalg::lu::gauss_step
nalgebra::base::blas_uninit::array_axcpy
```

The standalone triangular workload samples separate the stage cleanly:

```text
fsci_linalg::solve_triangular
fsci_linalg::solve_triangular_internal
fsci_linalg::compute_backward_error_dense
```

## Verdict

The triangular solve stage is still a real profile-backed target: the fresh
stage delta is 19.335 ms (`lu_solve - lu_factor`), and the standalone
`solve_triangular` path costs 10.121 ms per one-call hyperfine run.

The mixed `lu_solve` profile is still dominated by LU factorization
(`22/25` successful samples), so Pass 2 should isolate a precomputed-LU
solve-only target before editing production code. The standalone triangular
profile also shows residual/backward-error work is intertwined with the
triangular stage, so any optimization must preserve observable
`backward_error` bits.

## Command Notes

- `RCH_FORCE_REMOTE=1 rch exec -- bash -lc ... hyperfine ...` emitted an rch
  warning that `bash -lc` was a non-compilation command. The baseline JSON is
  present and valid, but this provenance caveat is recorded.
- The first quiet golden redirect produced an empty file because rch forwarded
  remote program output on stderr. This was corrected by capturing a raw rch
  transcript into `pass1_golden_rch_raw.txt`, filtering deterministic `n=`
  lines into `pass1_golden.txt`, and re-checksumming.
- A read-only diagnostic `stat` and a broad process listing hung under host
  process noise; both were terminated. They were not part of the benchmark,
  golden, or profile evidence.
