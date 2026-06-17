# frankenscipy-psn7x.19 - Production Band-To-Tridiagonal Baseline

Date: 2026-06-17
Agent: RubyWaterfall
Target: `eig_banded(lower=true, eigvals_only=false)` and native symmetric-eigh reduction
Worker: local only, per ts1/RCH-offline override

## Reason

After `frankenscipy-psn7x.18` kept the compact diagonal-lane envelope update, `br ready --json` had no ready perf beads. This bead was created from fresh local profile evidence instead of waiting idle.

The next primitive must be production-facing: move the compact band work toward `eig_banded(..., eigvals_only=false)` so it avoids dense `symmetric_eigh_native` reduction for lower-band inputs. Do not spend another pass on benchmark-only transformed dense emission.

## Public Banded Baseline

Command:

```bash
env RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-psn7x17-target \
  hyperfine --warmup 1 --runs 5 --show-output \
  'cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture'
```

Transcript: `baseline_eig_banded_eigenvectors_local_hyperfine.txt`

Result:

- Wall: `210.6 ms +/- 5.6 ms`
- 128x128 bw32 candidate range: `3.321615-3.841108 ms`
- 256x256 bw32 candidate range: `13.419694-16.822633 ms`
- Residuals stayed `1.64845914696343243e-12` at 128 and `7.73070496506989002e-12` at 256
- Values digests stayed `0xd6dbb9200f65bd92` and `0x09ed4d367faab431`
- Vector digests stayed `0x6cf3573b5b50c275` and `0xc32797c0d224a75a`

## Stage Split

Command:

```bash
env RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-psn7x17-target \
  cargo test -j 1 -p fsci-linalg --lib symmetric_eigh_native_stage_breakdown_probe --release --locked -- --ignored --nocapture
```

Transcript: `profile_symmetric_eigh_stage_split_local.txt`

Result:

- 400x400: reduction `13.865564 ms`, tridiagonal eigen `10.344970 ms`, backtransform `6.895181 ms`, sort `0.917507 ms`
- 800x800: reduction `107.966982 ms`, tridiagonal eigen `40.287203 ms`, backtransform `40.239793 ms`, sort `3.232749 ms`
- 1200x1200: reduction `365.980855 ms`, tridiagonal eigen `90.414421 ms`, backtransform `119.430150 ms`, sort `6.562060 ms`
- Digests stayed `0x0dbbde75b75c8612`, `0x4461962827bdb038`, `0x2fc45e1f18ceb0ab`

## Next Lever Boundary

Allowed direction:

- implement a true production band-to-tridiagonal/eigenvector route for `eig_banded(lower=true, eigvals_only=false)` using compact lower-band storage and Q replay
- prove public values/vectors/residuals/orthogonality/order against the current dense oracle
- rebench with the same public `eig_banded_eigenvectors_perf_probe`

Do not retry:

- benchmark-only transformed dense materialization cleanup
- direct-index rotation packet wrappers
- full active lower-envelope storage
- fixed envelope width guesses
- full-reorthogonalized Lanczos eigenvectors
- shifted inverse iteration over band solves
- worker-count retuning
- raw/stale compact-WY panels
- scalar spelling or SIMD rank-2 vector spelling

## Pass 2 Reject: Lower-Storage Native Entrypoint

Candidate:

- Split `symmetric_eigh_native` into an owned lower-storage entrypoint.
- Changed public `eig_banded(lower=true, eigvals_only=false)` to materialize only the lower half of the banded matrix and pass that owned matrix directly.
- Arithmetic inside Householder reduction, tridiagonal eigensolve, eigenvalue ordering, backtransform, tie behavior, and RNG behavior were unchanged.

Proof:

- `after_lower_storage_entrypoint_isomorphism_test.txt`: temporary bit-equivalence test passed, proving full symmetric storage and lower-only storage produced bit-identical sorted eigenvalues and eigenvectors for the native backend.
- `after_lower_storage_entrypoint_probe.txt`: public `eig_banded_eigenvectors_perf_probe` passed with unchanged residuals and unchanged public digests:
  - 128x128 values `0xd6dbb9200f65bd92`, vectors `0x6cf3573b5b50c275`, residual `1.64845914696343243e-12`
  - 256x256 values `0x09ed4d367faab431`, vectors `0xc32797c0d224a75a`, residual `7.73070496506989002e-12`

Rebench:

- Current-head local baseline: `205.5 ms +/- 9.6 ms` (`local_baseline_hyperfine_current_head.txt`)
- Candidate local rebench: `206.0 ms +/- 10.3 ms` (`after_lower_storage_entrypoint_hyperfine.txt`)
- Wall ratio: `0.998x`; public per-shape timings were mixed/noisy (`128` often faster, `256` mixed), and the overall gate did not improve.

Score:

- `Impact 0.0 * Confidence 4.0 / Effort 1.0 = 0.0`
- Source restored; `git diff -- crates/fsci-linalg/src/lib.rs` is empty after restore.

Route:

- Do not spend another pass on lower-storage/mirror/clone materialization cleanup.
- Next primitive must be a true production DSBTRD-style diagonal-band bulge chase with accumulated Q metadata replay, or a different algorithmic primitive selected from the current perf tracker.

## Pass 3 Reject: Dense Adjacent-Givens DSBTRD Route

Candidate:

- Added a production-facing moderate-bandwidth route for `eig_banded(lower=true, eigvals_only=false)`.
- The route used adjacent Givens similarities to reduce the lower-band matrix to tridiagonal form, then replayed the accumulated rotations onto the tridiagonal eigenvectors.
- This was an algorithmic DSBTRD-style route, not a materialization cleanup.

Proof:

- `after_givens_dsbtrd_probe.txt`: public `eig_banded_eigenvectors_perf_probe` passed.
- Dense-oracle eigenvalue drift and eigenvector residuals improved versus the tolerance gate:
  - 128x128 drift `2.67164068645797670e-12`, residual `9.66338120633736253e-13`
  - 256x256 drift `7.95807864051312208e-12`, residual `2.38742359215393662e-12`
- Output digests changed because the orthogonal reduction path changed:
  - 128x128 values `0x76dffb7ede1ec53c`, vectors `0x735d7877d616df3a`
  - 256x256 values `0xab524ebcc8117369`, vectors `0x09f006947d8f5a22`

Rebench:

- Current-head local baseline: `205.5 ms +/- 9.6 ms`
- Candidate local rebench: `267.6 ms +/- 32.0 ms` (`after_givens_dsbtrd_hyperfine.txt`)
- Candidate per-shape timings regressed badly:
  - 128x128: about `5.1-6.3 ms`, versus baseline about `3.6-4.3 ms`
  - 256x256: about `53.3-60.0 ms`, versus baseline about `13.3-15.8 ms`

Score:

- `Impact 0.0 * Confidence 5.0 / Effort 2.0 = 0.0`
- Source restored; `git diff -- crates/fsci-linalg/src/lib.rs` is empty after restore.

Route:

- Do not retry dense adjacent-rotation tridiagonalization for this bead.
- The next DSBTRD pass must keep the bulge chase in compact diagonal/band lanes and replay Q metadata directly; dense full-row/full-column Givens updates lose to the native Householder backend.
