# fsci-integrate solve_ivp no-output fast path: rejected candidate

Bead: frankenscipy-c9j5d
Date: 2026-06-02
Agent: OliveSnow

## Profile-backed target

Fresh fsci-integrate RCH Criterion profile on worker vmi1149989 ranked
`solve_ivp_lorenz_rk45` as the top row:

- `solve_ivp_lorenz_rk45`: mean 27.359 us
- `solve_ivp_exponential_rk45`: mean 20.205 us
- `validate_tol_vector_100`: mean 6.6383 us
- `validate_tol_scalar`: mean 292.36 ns

The investigated lever was a no-events/no-t_eval/no-dense-output fast path in
`solve_ivp_core` to avoid dense-output and event-state cloning on plain solves.

## Behavior proof

The candidate fast path was checked with a focused exact-equality test against
the existing general path forced by `events = Some(Vec::new())`. The comparison
covered:

- `t` ordering and values
- `y` vectors and floating-point bit patterns
- `nfev`, `njev`, `nlu`
- `status`, `message`, `success`
- absent `sol`, `t_events`, and `y_events`
- no RNG or tie-breaking changes

Golden output was emitted by the temporary `perf_integrate golden` helper:

- golden file: `golden_after.txt`
- sha256: `327b936597b6df9a5eb5d181a7f545a7c946458febea97652c73343781aa1eff`

## RCH measurements

The candidate did not produce a reproducible win:

- focused baseline, worker vmi1227854: 36.376 us
- after run, worker vmi1167313: 53.903 us
- repeat after run, worker vmi1167313: 61.627 us
- scratch HEAD baseline, worker vmi1153651: 57.099 us
- repeat after run, worker vmi1167313: 62.549 us

Because RCH did not provide a same-worker winning before/after pair, and the
repeat after-runs were slower than the available baselines, the candidate score
is below the required keep threshold.

## Decision

Rejected. The code lever was backed out and must not be committed as an
optimization. The profile target remains plausible, but it needs a different
lever and a reproducible same-worker RCH comparison before shipping.
