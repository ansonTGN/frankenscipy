# frankenscipy-8l8r1.54 Pass 1 - baseline and TSQR contract

Date: 2026-06-08
Skill loop: `/repeatedly-apply-skill` Pass 1 of 5 applying `/extreme-software-optimization`
Bead: `frankenscipy-8l8r1.54`
Assignee: `BlackThrush`

## Target

The profile-backed target is the public tall SVD/lstsq/pinv route, currently
driven by `public_bidiag_thin_svd_candidate` and the Golub-Kahan reducer.

Baseline command:

```text
RCH_FORCE_REMOTE=1 RCH_WORKER=<worker> rch exec -- cargo test -p fsci-linalg --release --lib --locked public_bidiag_svd_route_perf_probe -- --ignored --nocapture
```

## RCH fallback attempts

Two RCH-invoked attempts fell back local and are non-gating:

```text
artifact=tests/artifacts/perf/2026-06-08-linalg-tsqr-svd-route/baseline_public_bidiag_svd_route_perf_probe_rch.txt
worker=local fallback
reason=no admissible workers: critical_pressure=2,insufficient_slots=3,hard_preflight=2,active_project_exclusion=1
reference_lstsq_ms=99.205726
routed_lstsq_ms=55.622798
reference_pinv_ms=101.057460
routed_pinv_ms=56.189260
```

```text
artifact=tests/artifacts/perf/2026-06-08-linalg-tsqr-svd-route/baseline_public_bidiag_svd_route_perf_probe_rch_vmi1167313.txt
worker=local fallback
reason=no admissible workers: critical_pressure=2,insufficient_slots=4,hard_preflight=5
reference_lstsq_ms=96.948503
routed_lstsq_ms=55.078087
reference_pinv_ms=97.065725
routed_pinv_ms=56.812530
```

These runs are useful only as smoke tests for rank and numerical deltas. They
do not satisfy the keep/reject baseline requirement.

## Current behavior anchors

- Public rank: `lstsq_rank=256`, `pinv_rank=256`.
- Public numerical deltas in the fallback probe are small:
  `lstsq_max_abs_diff=1.07647224467655178e-12`,
  `pinv_max_abs_diff=2.28428387316625958e-14`.
- Existing public golden SHA from `.53`:
  `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
- Existing reducer digest anchor from `.53`:
  `0x90cdd3f8f71ed2c1`.

## TSQR behavior contract

Any `.54` implementation must preserve:

1. Public ordering and tie-breaking: singular values remain descending with the
   existing index tie policy, and rank thresholds are unchanged.
2. Public sign policy: canonical sign handling remains unchanged.
3. Public route guards: no wide-matrix acceptance, clustered-spectrum fallback,
   or tolerance relaxation.
4. Floating-point safety: TSQR/QR-first output must pass strict reconstruction,
   `U` column orthogonality, `Vt` orthogonality, least-squares, pinv, and public
   golden SHA checks before routing.
5. RNG: unchanged; no RNG is used.
6. Safety: no unsafe code, no C BLAS/LAPACK/MKL/XLA linkage, and no thread
   fanout for the first lever.

## Alien primitive

The selected alien-graveyard family is Communication-Avoiding Algorithms §9.6:
CA-QR/TSQR factors tall-skinny matrices by local QR on row blocks and a tree of
stacked `R` factors. For this first lever, the admissible safe-Rust subset is a
sequential TSQR/QR-first SVD candidate:

```text
A = Q R
R = U_R S Vt
U = Q U_R
```

The route is only acceptable if the proof harness shows the reconstructed `A`,
least-squares solution, pseudoinverse, singular ordering, rank, and public
golden behavior match the existing guarded public route.

## Required before editing source

Capture a real remote RCH baseline for `public_bidiag_svd_route_perf_probe` on
an admissible worker. The two local fallback attempts above are not gating.
