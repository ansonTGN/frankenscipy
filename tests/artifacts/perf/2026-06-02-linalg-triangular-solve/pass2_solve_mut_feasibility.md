# Pass 2 - Nalgebra Solve API Feasibility

Bead: `frankenscipy-perf-linalg-directlu-triangular-solve-v82ao`

Mission: determine whether replacing the public `lu_solve` call to
`LU::solve(&rhs)` with `solve_mut(&mut x)` is a valid one-lever optimization
for the post-factor solve path.

## Target Isolation

`perf_solve` now has a measurement-only `lu_solve_cached` mode. It factors the
matrix once before the timed loop and calls public `lu_solve` inside the timed
loop, so the measurement excludes fresh LU factorization while preserving the
public rcond/warning path.

Local nalgebra 0.34.2 source confirms the API relationship:

- `LU::solve(&rhs)` clones the RHS with `clone_owned()`.
- It then calls `solve_mut(&mut res)`.
- `solve_mut` returns `false` on singular input and may overwrite the mutable
  RHS on failure.

## Baseline

Command:

```text
RCH_FORCE_REMOTE=1 rch exec -- bash -lc 'cargo build -p fsci-linalg --profile release-perf --bin perf_solve --locked; hyperfine --warmup 3 --runs 10 ... "perf_solve lu_solve_cached 1000 10 42"; perf_solve golden'
```

Artifact: `pass2_cached_baseline_rch.json`

Remote hyperfine result for 10 cached public `lu_solve` calls:

- mean: `301.502 ms`
- median: `301.129 ms`
- stddev: `12.642 ms`
- user/system: `183.730 ms` / `117.412 ms`
- per-call mean: `30.150 ms`

Golden:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass2_golden_before.txt
```

`sha256sum -c pass2_golden_before.sha256` passed.

## Cached Profile

Artifacts:

- `pass2_cached_profile_samples.txt`
- `pass2_cached_profile_counts.txt`

Counts:

```text
total sample markers: 40
successful backtraces with fsci_linalg: 32
LU solve_mut frames: 2
LU solve frames: 2
fast_rcond_from_lu frames: 15
public lu_solve frames: 16
array_axcpy/axcpy/blas frames: 6
```

Interpretation: the public cached solve path is dominated by `fast_rcond_from_lu`
and nalgebra triangular/transpose work. The exact `LU::solve` clone avoided by
the candidate is visible but not dominant.

## Candidate Lever

Temporary production candidate in `crates/fsci-linalg/src/lib.rs`:

```rust
let mut x = DVector::from_column_slice(b);
if !lu_factor.lu_internal.solve_mut(&mut x) {
    return Err(LinalgError::SingularMatrix);
}
```

This replaced only:

```rust
let rhs = DVector::from_column_slice(b);
let x = lu_factor
    .lu_internal
    .solve(&rhs)
    .ok_or(LinalgError::SingularMatrix)?;
```

No DirectLU `dispatch_solve_action` code was touched.

## Candidate Results

Artifact: `pass2_cached_after_solve_mut_rch.json`

- mean: `285.191 ms`
- median: `286.464 ms`
- stddev: `9.344 ms`
- user/system: `168.744 ms` / `116.023 ms`
- per-call mean: `28.519 ms`

Artifact: `pass2_cached_after_solve_mut_confirm_rch.json`

- mean: `291.760 ms`
- median: `288.280 ms`
- stddev: `15.786 ms`
- user/system: `176.400 ms` / `114.666 ms`
- per-call mean: `29.176 ms`

Golden after candidate:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass2_golden_after_solve_mut.txt
```

`sha256sum -c pass2_golden_after_solve_mut.sha256` passed, and
`cmp -s pass2_golden_before.txt pass2_golden_after_solve_mut.txt` passed.

## Manual A/B Rejection Check

Because the candidate win was small and overlapped timing noise, the exact code
hunk was manually restored to the original `LU::solve(&rhs)` form and the cached
baseline was re-run.

Artifact: `pass2_cached_baseline_repeat_rch.json`

- mean: `280.516 ms`
- median: `279.850 ms`
- stddev: `10.439 ms`
- user/system: `170.091 ms` / `110.071 ms`
- per-call mean: `28.052 ms`

The restored baseline repeat is faster than both candidate runs. The candidate
therefore fails the required performance threshold despite exact behavior
preservation.

## Isomorphism Proof

- Ordering preserved: yes. The call sequence remains shape check, solve,
  rcond estimate, trace emission, and result construction.
- Tie-breaking unchanged: N/A. No branch ordering or solver selection changed.
- Floating-point: candidate output was bit-identical for the golden solve set;
  the candidate only removed nalgebra's extra RHS clone before the same
  `solve_mut` triangular operations.
- RNG: unchanged. The harness uses deterministic SplitMix/LCG-style input
  generation with the same seeds.
- Golden outputs: before and after sha256 both
  `5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd`.

## Verdict

Rejected. `LU::solve(&rhs)` to `solve_mut(&mut x)` is behavior-preserving in
public `lu_solve`, but the profile does not make it a dominant target and the
manual A/B check failed performance proof. Production code was manually restored
to the original `LU::solve(&rhs)` implementation.

Score: `0.0` kept score because the measured candidate failed the performance
gate.

## Validation

Passed:

- `jq empty pass2_cached_baseline_rch.json pass2_cached_after_solve_mut_rch.json pass2_cached_after_solve_mut_confirm_rch.json pass2_cached_baseline_repeat_rch.json`
- `sha256sum -c pass2_golden_before.sha256`
- `sha256sum -c pass2_golden_after_solve_mut.sha256`
- `cmp -s pass2_golden_before.txt pass2_golden_after_solve_mut.txt`
- `git diff --check -- crates/fsci-linalg/src/lib.rs crates/fsci-linalg/src/bin/perf_solve.rs .skill-loop-progress.md tests/artifacts/perf/2026-06-02-linalg-triangular-solve`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo check -p fsci-linalg --bin perf_solve --locked`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg lu_solve_matches_scipy_reference_values --lib --locked`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`

Failed:

- `cargo fmt -p fsci-linalg --check`

The fmt failure is unrelated lower-file formatting drift in
`crates/fsci-linalg/src/lib.rs` tests (`qz_q_and_z_are_orthogonal`,
Schur/eigvalsh/svdvals/matrix-function assertions). It was not reformatted in
this pass because doing so would rewrite unrelated peer work.
