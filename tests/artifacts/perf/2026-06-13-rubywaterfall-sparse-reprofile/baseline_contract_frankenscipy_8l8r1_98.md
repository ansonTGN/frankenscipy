# frankenscipy-8l8r1.98 Baseline Contract

Target: `fsci-sparse` native sparse LU ordering/factorization for explicit
`PermutationOrdering::MmdAtPlusA`.

Profile evidence:
- Artifact: `current_perf_spsolve_rch.txt`
- Artifact sha256: `d867b7242c4be7f53368765e4a0582410b3d00a4111b9201b4ff3b63bd90ada9`
- RCH worker: `vmi1227854`
- Command: `RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo run --profile release-perf -p fsci-sparse --bin perf_spsolve --locked`

Measured current rows:
- `lap2d k=20 n=400`: RCM `2.8220 ms`, MMD `3.60495 ms`, ratio `0.78x`, `max|dx|=5.26e-13`
- `lap2d k=32 n=1024`: RCM `15.1412 ms`, MMD `18.93301 ms`, ratio `0.80x`, `max|dx|=4.89e-12`
- `lap2d k=45 n=2025`: long-tail row did not complete after several minutes; stale RCH process was terminated. This is routing evidence only, not a keep/reject measurement.

One lever:
- Preserve the MMD degree/index elimination order, but replace the internal adjacency storage used by `minimum_degree_ordering` with a cheaper membership representation.
- No default ordering change. `Colamd` remains the RCM-backed default path.

Behavior proof required:
- The `minimum_degree_ordering` permutation must be byte-for-byte identical to the current implementation on grid, arrowhead, and fragmented tie-heavy graphs.
- `spsolve(..., MmdAtPlusA)` must keep the same solution golden digest for a deterministic 2D Laplacian probe.
- Ordering/tie-breaking remains degree then lowest original index. No RNG, no unsafe, no external BLAS/LAPACK.

Keep gate:
- Same-worker RCH rebench must show a real win on the focused MMD rows.
- Score must be at least `2.0 = Impact x Confidence / Effort`.
