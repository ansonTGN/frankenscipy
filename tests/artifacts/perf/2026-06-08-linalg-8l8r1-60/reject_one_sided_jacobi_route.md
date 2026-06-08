# Rejected: one-sided Jacobi public thin-SVD route

Bead: `frankenscipy-8l8r1.60`

## Candidate

Guarded one-sided Jacobi thin-SVD candidate over `A` directly:

- rotate column pairs of `A`
- accumulate deterministic right singular vectors
- sort singular values descending with stable tie policy
- accept only through the existing public reconstruction/rank/tie/sign gate
- fall back to the bidiagonal route on non-convergence

The source was restored after measurement; no code change is kept.

## Proof

- `proof_public_route_one_sided_jacobi_rch.txt`: public route proof passed on `vmi1167313`.
- `public_svd_lstsq_pinv_golden_payload.sha256`: `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
- `after_one_sided_jacobi_public_route_perf_probe_rch.txt`: the 512x256 route test passed with ranks `256/256` and diffs within tolerance while timing the candidate.

## Performance

Fresh `.60` unchanged-source baseline:

- `baseline_public_bidiag_svd_route_perf_probe_rch.txt`
- worker: `vmi1167313`
- `routed_lstsq_ms=114.909589`
- `routed_pinv_ms=119.145625`

Same-worker public-route baseline already available from the current lineage:

- `tests/artifacts/perf/2026-06-08-linalg-block-tsqr-tree/baseline_public_bidiag_svd_route_perf_probe_rch.txt`
- worker: `vmi1153651`
- `routed_lstsq_ms=131.225285`
- `routed_pinv_ms=118.656896`

One-sided Jacobi after run:

- `after_one_sided_jacobi_public_route_perf_probe_rch.txt`
- worker: `vmi1153651`
- `routed_lstsq_ms=373.403597`
- `routed_pinv_ms=400.993708`
- ranks remained `256/256`
- `lstsq_max_abs_diff=8.44378789111033257e-10`
- `pinv_max_abs_diff=1.90249231299399746e-11`

Same-worker ratios on `vmi1153651`:

- lstsq: `0.351430x`
- pinv: `0.295905x`

## Verdict

Rejected. The candidate is correct under the route gates but too slow. Do not repeat plain one-sided Jacobi sweeps for this public route.

Next primitive should avoid full SVD replacement for `lstsq`/`pinv` and attack the public route split directly, for example a guarded solve-only QR/Cholesky path for full-rank `lstsq`/`pinv` while preserving fail-closed rank and residual checks.
