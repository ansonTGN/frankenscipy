# Gauntlet Release Scorecard

Last updated: 2026-06-19 by cod-a / MistyBirch.

This scorecard tracks code-first performance work that has been converted into
measured head-to-head evidence against the SciPy original. The detailed
win/loss/neutral ledger lives in `docs/progress/perf-negative-results.md`.

## Measured Keeps

| Bead | Cluster | Realistic workload | Rust result | SciPy result | Ratio | Decision |
| --- | --- | --- | ---: | ---: | ---: | --- |
| `frankenscipy-u0ucw` | Wide `pinv` Cholesky TRSM + diagonal rcond gate | 500x1000 full-row-rank dense `scipy.linalg.pinv` equivalent | 183.699926 ms | 7.257573 s | 39.51x faster | keep |
| `frankenscipy-u0ucw` | Wide `lstsq` current materialized normal equations after row-stream revert | 500x1000 full-row-rank dense `scipy.linalg.lstsq` equivalent | 109.369915 ms | 1.253347 s | 11.46x faster | keep current, reject row-stream lever |

## Internal Regression Gates

| Bead | Current route | Superseded route | Mean delta | Decision |
| --- | --- | --- | ---: | --- |
| `frankenscipy-u0ucw` | Cholesky + diagonal rcond gate | Cholesky + eigenspectrum rcond gate | 1.40x faster | keep current |
| `frankenscipy-u0ucw` | Cholesky + diagonal rcond gate | SVD fallback | 2.82x faster | keep current |
| `frankenscipy-u0ucw` | Wide `lstsq` materialized `A^T` | Row-streamed `A A^T` + `A^T y` | 1.035x faster | revert row-streaming |

## Current Readiness

| Area | Status | Evidence |
| --- | --- | --- |
| Wide `pinv` performance | measured keep | Criterion mean point estimate vs SciPy 1.17.1 oracle, 39.51x faster |
| Wide `pinv` correctness | guarded | targeted `fsci-linalg` tests cover the diagonal gate, Cholesky route, helper products, and SciPy reference values |
| Wide `lstsq` performance | measured keep plus internal reject | current materialized path is 11.46x faster than SciPy; row-streamed lever was 0.966x vs materialized and was reverted |
| Wide `lstsq` correctness | guarded | `public_wide_min_norm_lstsq_route_perf_probe` passed in release with max abs diff `3.38840067115597776e-13` |
| rch SciPy oracle parity | blocked on worker image | `vmi1152480` and `vmi1227854` lacked `scipy`; local same-host oracle supplied the head-to-head ratios |
| Release readiness | partial | two linalg perf clusters verified; other code-first perf ledger entries still need gauntlet conversion |

## Pending Gauntlet Backlog

Continue converting `pending batch-test` entries in the negative-evidence ledger
one cluster at a time. Each conversion must record the SciPy ratio, internal
A/B route deltas, conformance status, and keep/revert decision before release
readiness can be raised.
