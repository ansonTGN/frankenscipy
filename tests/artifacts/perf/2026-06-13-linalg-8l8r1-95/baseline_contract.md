# frankenscipy-8l8r1.95 Baseline Contract

## Hotspot and Comparator

- Target: `fsci-linalg::det` on `det/256x256`.
- Baseline artifact: `tests/artifacts/perf/2026-06-13-linalg-det-lu-route/baseline_det_256_rch.txt`
- Baseline artifact sha256: `810e540821136044643d90d3ed4fa1e601f430e03a24f2c2e38d9d91b0b4d951`
- Baseline worker: `vmi1149989`
- Baseline command: `cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- det/256x256 --sample-size 20`
- Baseline Criterion interval: `det/256x256 time [1.0959 ms 1.1294 ms 1.1721 ms]`
- Current implementation: `crates/fsci-linalg/src/lib.rs::det` validates shape/finite policy, materializes `DMatrix`, then calls `matrix.lu().determinant()`.
- Candidate single lever: for eligible dense square finite matrices, compute determinant as permutation sign times the product of the diagonal of existing safe-Rust `lu_factor_blocked` factors; fall back to the current `DMatrix` + nalgebra LU determinant path when not eligible.
- Alien mapping: requested CA-LU graveyard mapping; the local graveyard copy labels the relevant CA-LU notes under `9.6 -- Communication-Avoiding Algorithms`. Numerical-linear-algebra family 34 applies for decomposition record, factorization accuracy, condition/failure handling, and golden cross-check obligations.

## EV Gate

Keep gate: only implement/keep a source change if `Score >= 2.0` and same-worker after evidence on `vmi1149989` improves the baseline without weakening determinant behavior.

| Candidate | Impact | Confidence | Effort | Score | Decision |
|---|---:|---:|---:|---:|---|
| In-house LU determinant product using `lu_factor_blocked` | 3 | 3 | 2 | 4.5 | Candidate for implementation |
| Add small-matrix closed forms | 1 | 4 | 1 | 4.0 | Reject for this bead: does not target `256x256` hotspot |
| Retune unrelated LU/GEMM internals | 4 | 2 | 5 | 1.6 | Reject: not a single proofable determinant lever |

## Isomorphism Proof Plan

- Ordering: determinant returns one scalar; no output sequence exists. Pivot scan order must be deterministic and fixture-covered because row swaps determine determinant sign.
- Tie-breaking: current nalgebra tie behavior is the reference. Before implementation, capture a current-path golden over pivot-tie fixtures, singular fixtures, well-conditioned dense fixtures, and sign-sensitive row-swap fixtures. If the candidate's pivot tie policy produces a different sign or tolerance failure, route that case to the current path.
- RNG: none. Golden fixtures must be deterministic literal matrices or seeded generators with the seed embedded in the golden payload.
- Floating-point: do not claim bit identity. The candidate changes LU update/product order, so acceptance is tolerance-based against the current path and SciPy-scoped determinant expectations. Track raw `f64::to_bits()` in the golden payload to expose drift.
- Error behavior: preserve `ExpectedSquareMatrix`, `hardened_dimension_check`, `validate_finite_matrix`, empty-matrix `Ok(1.0)`, audit recording through `det_with_audit`, and singular determinant `Ok(0.0)` behavior.
- Golden sha256: pre-implementation blocker. Add a focused crate test that emits a SHA-256 over labels, result status, and determinant result bits from the current path; freeze that expected SHA before editing the determinant route. The implementation pass may not start until this SHA is recorded in the test/artifact.

## Validation Plan

All validation is crate-scoped and RCH-mediated; do not run full-workspace builds.

1. Current-path golden capture before source edits:
   `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1149989 CARGO_BUILD_JOBS=1 rch exec -- cargo test -j 1 -p fsci-linalg det_lu_route_golden_sha256 -- --nocapture`
2. Focused determinant tests after implementation:
   `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1149989 CARGO_BUILD_JOBS=1 rch exec -- cargo test -j 1 -p fsci-linalg det -- --nocapture`
3. Crate check:
   `RCH_REQUIRE_REMOTE=1 CARGO_BUILD_JOBS=1 rch exec -- cargo check -j 1 -p fsci-linalg --all-targets`
4. Crate clippy:
   `RCH_REQUIRE_REMOTE=1 CARGO_BUILD_JOBS=1 rch exec -- cargo clippy -j 1 -p fsci-linalg --all-targets -- -D warnings`
5. Crate formatting:
   `RCH_REQUIRE_REMOTE=1 CARGO_BUILD_JOBS=1 rch exec -- cargo fmt --package fsci-linalg -- --check`
6. Same-worker after benchmark:
   `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1149989 CARGO_BUILD_JOBS=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- det/256x256 --sample-size 20`

## Fallback and Rollback

- Runtime fallback trigger: use the current `DMatrix` + nalgebra LU determinant path when shape/finite validation fails, the matrix is empty, `lu_factor_blocked` returns `None`, the candidate determinant is non-finite where the current path is finite, pivot-tie fixtures drift beyond tolerance, or any golden/audit/error behavior test fails.
- Keep/reject trigger: reject and restore the implementation if same-worker `vmi1149989` Criterion evidence does not clear `Score >= 2.0` or if Criterion intervals overlap without a defensible midpoint/tail win.
- Rollback: after a future implementation commit, use `git revert <future_sha>` for source rollback. Do not delete this contract or the baseline evidence; they remain the audit trail for the rejected or accepted lever.
