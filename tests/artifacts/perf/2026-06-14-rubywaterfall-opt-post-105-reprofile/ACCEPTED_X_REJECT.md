# Accepted-X Materialization Rejection

Follow-up from `frankenscipy-8l8r1.106`.

## Stage Route

The stage/profile pass showed that `cg/rosenbrock/10` is dominated by strong-Wolfe value and finite-difference objective traffic, not accepted-point materialization:

- `nfev=18359`
- `actual_calls=11679`
- `finite_diff_budget_calls=7380`
- `line_search_budget_calls=10978`
- `accepted_x_materializations=368`

## Benchmark Gate

The accepted-point reuse probe did not clear the keep gate.

- Local fallback artifact: `after_cg_rosenbrock10_accepted_x_rch.txt`, ignored for proof.
- Remote retry artifact: `after_cg_rosenbrock10_accepted_x_remote_retry_rch.txt`, cross-worker routing evidence only.
- Focused same-worker evidence is recorded in `../2026-06-14-rubywaterfall-opt-106/EVIDENCE.md`: `vmi1227854` baseline p50 `220.15 us`, after p50 `220.19 us`.

Decision: reject, restore source, and route next to `frankenscipy-8l8r1.107`.

## Next Route

Do not repeat accepted-point materialization, accepted-gradient carry, or scratch-only workspace reuse. Attack the finite-difference plus line-search objective-call budget with a differentiated-objective primitive while preserving the current finite-difference CG fallback bit-identically.
