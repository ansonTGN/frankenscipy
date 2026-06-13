# frankenscipy-8l8r1.95 Reject: In-House LU Determinant Product

## Target

- Bead: `frankenscipy-8l8r1.95`
- Target benchmark: `det/256x256`
- Worker: `vmi1149989`
- Lever: route `det()` through the existing safe-Rust `lu_factor_blocked` factors and compute `sign(perm) * prod(diag(U))`, with nalgebra determinant as fallback.

## Baseline

- Artifact: `tests/artifacts/perf/2026-06-13-linalg-det-lu-route/baseline_det_256_rch.txt`
- SHA-256: `810e540821136044643d90d3ed4fa1e601f430e03a24f2c2e38d9d91b0b4d951`
- Command: `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1149989 CARGO_BUILD_JOBS=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- det/256x256 --sample-size 20`
- Criterion: `[1.0959 ms 1.1294 ms 1.1721 ms]`

## Proof Run

- Artifact: `tests/artifacts/perf/2026-06-13-linalg-8l8r1-95/proof_det_route_tests_rch.txt`
- Result: 2 focused determinant-route tests passed on `vmi1149989`.
- Golden digest during probe: `det_blocked_lu_route_golden_digest=0xc723e1905cb23bbb`
- Scope: finite 130x130 deterministic fixtures, forced row swap, pivot tie, and singular fallback.

## After Benchmark

- Artifact: `tests/artifacts/perf/2026-06-13-linalg-8l8r1-95/after_det_256_lu_route_vmi1149989_rch.txt`
- SHA-256: `f0b8a1d79a8f74d6fdd0f565dfb39f4d7982b472f483d0af70a806b05f61bdd3`
- Command: `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1149989 CARGO_BUILD_JOBS=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- det/256x256 --sample-size 20`
- Criterion: `[4.0636 ms 4.2118 ms 4.4006 ms]`

## Decision

- Baseline midpoint: `1.1294 ms`
- After midpoint: `4.2118 ms`
- Ratio: `0.268x` versus baseline, a `3.73x` regression.
- Score: `(Impact 0.0 * Confidence 4.0) / Effort 2.0 = 0.0`
- Verdict: reject. The source route was reverted; no `fsci-linalg` source diff remains for this bead.

## Next Route

Do not repeat a determinant-only product over the current blocked LU factors. The failure says the existing in-house LU factorization is slower than nalgebra's determinant path at this size. The next profile-backed route should replace the LU factorization primitive itself: flat contiguous panel storage, cache-blocked/recursive trailing updates, and communication-avoiding or tournament pivoting candidates from the alien-graveyard numerical-linear-algebra lane, proved first against `solve`, `inv`, and `det` shared LU consumers.
