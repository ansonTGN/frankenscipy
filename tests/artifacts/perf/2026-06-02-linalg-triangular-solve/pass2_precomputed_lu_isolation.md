# Pass 2 - Precomputed-LU Solve/Rcond Isolation

Date: 2026-06-02T02:11:23-0400
Agent: OliveSnow
Target bead: `frankenscipy-perf-linalg-directlu-triangular-solve-v82ao`

## Mission

Pass 1 refuted final DirectLU dispatch solve as the next dominant full-solve
hotspot. This pass isolated post-factor `lu_solve` work before any production
optimization. The only Rust edit was a harness-only `lu_solve_cached` mode in
`crates/fsci-linalg/src/bin/perf_solve.rs`.

No production library code was edited.

## Harness Change

`lu_solve_cached` factors once before the timed loop, then repeatedly calls the
public `lu_solve(&factor, &b)` path:

- Existing `solve`, `lu_factor`, `lu_solve`, `solve_triangular`, and `golden`
  modes keep their prior operation order.
- New cached mode preserves matrix/RHS generation and seed use; it only moves
  factorization outside the timed section for isolation.
- The timed operation is still the public API under test: `lu_solve(&factor,
  &b)`.

## Golden / Isomorphism

Artifact:

- `pass2_golden.txt`
- `pass2_golden.sha256`
- `pass2_golden_rch_raw.txt`

Golden sha256:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass2_golden.txt
```

`sha256sum -c pass2_golden.sha256` passed.

Isomorphism proof for the harness edit:

- Ordering preserved: yes for existing modes; the new mode intentionally
  isolates a precomputed factor and then calls public `lu_solve` in the same
  loop order.
- Tie-breaking unchanged: N/A; no branch ordering or solver selection policy
  was changed.
- Floating-point unchanged for existing modes: yes; `golden` output is bit
  identical to pass 1. The new cached mode uses the same public `lu_solve`
  arithmetic after a precomputed factor.
- RNG seeds unchanged: yes; deterministic matrix/RHS generators are unchanged.
- Golden outputs: `sha256sum -c pass2_golden.sha256` passed.

## Remote Hyperfine Baseline

Command shape:

```bash
RCH_FORCE_REMOTE=1 rch exec -- hyperfine \
  --setup 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_tri_pass2_bench_20260602a RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve --locked' \
  --warmup 3 \
  --runs 10 \
  --export-json tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass2_cached_baseline_rch.json \
  '/tmp/rch_target_fsci_linalg_tri_pass2_bench_20260602a/release-perf/perf_solve lu_factor 1000 1 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass2_bench_20260602a/release-perf/perf_solve lu_solve 1000 1 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass2_bench_20260602a/release-perf/perf_solve lu_solve_cached 1000 200 42'
```

| mode | mean | stddev | median | min | max | user | system |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `lu_factor 1000 1 42` | 99.606 ms | 4.268 ms | 98.685 ms | 94.460 ms | 106.835 ms | 84.494 ms | 14.461 ms |
| `lu_solve 1000 1 42` | 115.628 ms | 7.622 ms | 114.443 ms | 106.179 ms | 132.032 ms | 90.784 ms | 24.584 ms |
| `lu_solve_cached 1000 200 42` | 3797.411 ms | 157.794 ms | 3734.666 ms | 3636.933 ms | 4024.806 ms | 1842.109 ms | 1952.769 ms |

Cached per-call hyperfine arithmetic:

- Mean: 18.987 ms/call (`3797.411 ms / 200`).
- Median: 18.673 ms/call.
- Stddev: 0.789 ms/call.
- User/system: 9.211 ms/call user, 9.764 ms/call system.

Raw harness timer probes from `pass2_cached_probe.jsonl` exclude precomputed
factorization from the measured region:

| run | mode | per-call | total | checksum |
| ---: | --- | ---: | ---: | ---: |
| 1 | `lu_solve_cached 1000 200 42` | 19.1013 ms | 3820.259 ms | -196.8966 |
| 2 | `lu_solve_cached 1000 200 42` | 18.5522 ms | 3710.448 ms | -196.8966 |
| 3 | `lu_solve_cached 1000 200 42` | 18.5636 ms | 3712.725 ms | -196.8966 |

Harness-timer mean: 18.739 ms/call.

## Focused Samples

Artifacts:

- `pass2_cached_gdb_samples.txt`
- `pass2_cached_profile_counts.txt`
- `pass2_cached_profile_run.txt`

Long sampled run:

```text
{"mode":"lu_solve_cached","n":1000,"repeats":2000,"total_ms":46981.243,"per_call_ms":23.4906,"checksum":-1.968966e3}
```

The long-run timer includes gdb attach overhead, so it is attribution evidence
rather than the wall-clock baseline.

Reduced sample counts:

```text
total_sample_blocks=50
setup_factorization_samples=1
timed_sample_blocks=49
fast_rcond_from_lu=49
triangular_solve_frames=8
transpose_rcond_solver=4
array_axcpy_or_blas=8
allocation_or_clone=16
rhs_from_column_slice=0
final_lu_solve_without_rcond=0
dealloc_or_malloc_trim=5
```

Interpretation:

- One sample caught the intentionally pre-timer cached LU factorization setup
  and was excluded from timed attribution.
- All 49 timed samples were inside `fast_rcond_from_lu`.
- The final cached `lu_internal.solve(&rhs)` path did not appear in any timed
  sample.
- RHS `DVector::from_column_slice` did not appear in any timed sample.
- Visible substructure inside rcond was a mix of transpose/clone/allocation and
  nalgebra triangular solve frames; this supports retargeting the next pass to
  rcond internals, not final DirectLU solve.

## Candidate Scores

| Candidate | Evidence | Impact | Confidence | Effort | Score | Decision |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| Optimize final cached `lu_internal.solve(&rhs)` | 0/49 timed samples | 0.2 | 5.0 | 1.0 | 1.0 | Reject |
| Optimize RHS clone/construction | 0/49 timed samples for `from_column_slice` | 0.1 | 4.0 | 1.0 | 0.4 | Reject |
| Cache or restructure rcond transpose/allocation work across repeated `LuFactorResult` solves | `allocation_or_clone=16/49`, but production shape would alter public result internals or introduce lazy cached state | 2.0 | 2.0 | 3.0 | 1.3 | Defer |
| Change rcond estimator algorithm / iteration policy | `fast_rcond_from_lu=49/49`, but tolerance and warning behavior risk is high without a narrower oracle | 5.0 | 1.0 | 5.0 | 1.0 | Reject for this pass |

No single production lever reached the keep threshold of 2.0 for this pass, so
no `crates/fsci-linalg/src/lib.rs` edit was made.

## Pass 3 Target

Pass 3 should target `fast_rcond_from_lu` directly with a narrower profile that
separates:

1. transpose materialization (`lu.u().transpose()`, `lu.l().transpose()`),
2. sign-vector map/allocation,
3. `solve_lu_transpose_with_transposes`,
4. `lu.solve(&sign_w)`,
5. final `rcond` warning propagation.

The likely next profiler-backed question is whether `LuFactorResult` can safely
cache rcond-side derived state for repeated `lu_solve` calls without changing
warnings, traces, `Debug` observability, or floating-point bits.
