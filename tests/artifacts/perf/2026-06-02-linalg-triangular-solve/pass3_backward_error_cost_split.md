# Pass 3 - Backward Error Cost Split

Bead: `frankenscipy-perf-linalg-directlu-triangular-solve-v82ao`

Mission: isolate the DirectLU post-solve residual/backward-error cost without
weakening the observable `SolveResult.backward_error` contract.

## Files

- `crates/fsci-linalg/src/bin/perf_solve.rs`: added a measurement-only
  `backward_error_probe` mode. It precomputes a normal DirectLU `solve`, then
  times only the same nalgebra residual formula used by production
  `compute_backward_error`.
- `tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3*`: benchmark,
  profile, golden, candidate, probe, and summary artifacts for this pass.
- `.skill-loop-progress.md`: pass-3 status updated.
- `crates/fsci-linalg/src/lib.rs`: temporarily changed DirectLU
  `backward_error` to `None` for measurement, then manually restored. No
  pass-owned DirectLU production change was kept. A concurrent rcond-cache diff
  is present in the shared tree and was not touched by this pass.

## Fresh DirectLU Baseline

Command shape:

```bash
RCH_FORCE_REMOTE=1 rch exec -- hyperfine \
  --setup 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_tri_pass3_baseline_20260602a RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve --locked' \
  --warmup 3 \
  --runs 10 \
  --export-json tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3_directlu_baseline_rch.json \
  '/tmp/rch_target_fsci_linalg_tri_pass3_baseline_20260602a/release-perf/perf_solve solve 1000 10 42'
```

Result:

- Mean: `987.727 ms` per 10 solves, `98.773 ms/solve`.
- Median: `983.392 ms` per 10 solves, `98.339 ms/solve`.
- Stddev: `20.602 ms`.
- User/system: `869.808 ms` / `117.160 ms`.

Fresh baseline golden:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3_golden_before.txt
```

## Profile Attribution

GDB sampler over `perf_solve solve 1000 180 42`:

```text
total_sample_blocks=60
successful_fsci_blocks=60
compute_backward_error_blocks=0
directlu_dispatch_blocks=0
condition_diagnostics_blocks=57
fast_rcond_blocks=15
lu_factorization_blocks=44
array_axcpy_or_blas_blocks=39
dmatrix_from_rows_blocks=0
allocation_clone_memcpy_blocks=23
```

Interpretation: residual/backward-error did not appear in 60 successful samples.
That puts it below the visible hotspot tier for the full DirectLU solve; samples
remain dominated by condition diagnostics, LU factorization, rcond, and
nalgebra axcpy work.

## Residual Probe

The harness-only `backward_error_probe` mode precomputes one DirectLU solution
and then times only this production-equivalent residual formula:

```rust
let residual = matrix * x - rhs;
let residual_norm = residual.norm();
let denom = matrix.norm() * x.norm() + rhs.norm();
```

Remote hyperfine for `backward_error_probe 1000 1000 42`:

- Mean command time: `413.283 ms` (includes one precompute solve plus 1000
  residual evaluations).
- Stddev: `11.055 ms`.
- Median: `409.801 ms`.

Raw internal harness probes:

- Runs: `5`.
- Mean per residual: `0.29362 ms`.
- Min/max per residual: `0.2786 ms` / `0.3058 ms`.
- Unique checksum: `5.353532e-14`.

Estimated residual share against the fresh DirectLU solve baseline:
`0.29362 / 98.77268 = 0.30%` per solve.

## Candidate Lever

Temporary production candidate:

```rust
backward_error: None
```

This replaced the DirectLU dispatch path's:

```rust
let backward_err = compute_backward_error(&matrix, &x, &rhs);
backward_error: Some(backward_err)
```

Candidate benchmark for `solve 1000 10 42`:

- Mean: `1001.672 ms` per 10 solves, `100.167 ms/solve`.
- Median: `1001.019 ms` per 10 solves, `100.102 ms/solve`.
- Stddev: `18.150 ms`.
- Delta vs baseline: `+13.945 ms` per 10 solves, slower.

Candidate golden:

```text
eb296e5a1ca9c153c6b3fa5b793709ca6cd0957ce41108b9b0d97d4a938c2b4c  tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3_golden_no_backward_error.txt
```

`cmp_before_vs_no_backward_error_exit=1`, so the candidate changed observable
golden output. The candidate is rejected on both behavior and performance.

Score: `0.0` kept score. Impact is effectively zero after the probe, confidence
is zero because the golden changed, and the A/B benchmark did not improve.

## Isomorphism Proof

- Ordering preserved: retained code preserves the existing DirectLU solve order.
  The added harness mode runs a normal solve once, then measures residual only;
  it does not alter existing modes.
- Tie-breaking unchanged: yes. No solver selection, action ordering, fallback,
  or policy ranking code was retained.
- Floating-point: retained production output is restored byte-for-byte. The
  no-backward-error candidate changed `backward_error` bits and was rejected.
- RNG seeds unchanged: yes. The deterministic SplitMix/LCG-style workload
  generation still uses the same seeds.
- Golden outputs: restored golden sha256 is
  `5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd`, and
  `cmp -s pass3_golden_before.txt pass3_golden_restored.txt` passed.

## Validation

Passed:

- `jq empty` on `pass3_directlu_baseline_rch.json`,
  `pass3_directlu_no_backward_error_rch.json`,
  `pass3_backward_error_probe_rch.json`, and
  `pass3_backward_error_probe_summary.json`.
- `sha256sum -c pass3_golden_before.sha256`.
- `sha256sum -c pass3_golden_no_backward_error.sha256`.
- `sha256sum -c pass3_golden_restored.sha256`.
- `cmp -s pass3_golden_before.txt pass3_golden_restored.txt`.
- `git diff --check -- crates/fsci-linalg/src/lib.rs crates/fsci-linalg/src/bin/perf_solve.rs .skill-loop-progress.md`.
- `RCH_FORCE_REMOTE=1 rch exec -- cargo check -p fsci-linalg --bin perf_solve --locked`.
- `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg solve_matches_scipy_reference_values --lib --locked` (`3` tests passed).
- `RCH_FORCE_REMOTE=1 rch exec -- cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`.

Failed:

- `cargo fmt -p fsci-linalg --check` on existing unrelated lower-file
  formatting drift in `crates/fsci-linalg/src/lib.rs` tests.

Verdict: `REJECTED / ZERO PRODUCTION OPTIMIZATION`. Backward-error computation
is observable and costs only about `0.30%` of the profiled DirectLU solve at
`n=1000`; removing it changes golden output and was slower in the A/B run.
