# frankenscipy-8l8r1.108 - in-place exact-gradient CG callback subprobe rejection

## Target

Profile-backed subprobe after `frankenscipy-8l8r1.107`.

The kept exact-gradient route removed the finite-difference objective-call budget
from `cg/rosenbrock/10`; the next candidate tested whether avoiding
`GradientFunc = fn(&[f64]) -> Vec<f64>` materialization would improve the new
exact-gradient hot row.

Integration note: a concurrent remote bead with the same ID,
`frankenscipy-8l8r1.108`, owns the broader derivative-interface primitive. This
artifact records only the rejected in-place callback subprobe; the shared bead
remains open for the fused value+gradient or AD-tape route.

## Baseline

Fresh RCH baseline:

- Artifact: `baseline_exact_gradient_vmi1167313_rch.txt`
- Selected worker: `vmi1293453`
- Row: `cg/rosenbrock_exact_gradient/10`
- Time: `[143.93 us, 147.48 us, 151.37 us]`

Existing same-worker comparator from `frankenscipy-8l8r1.107`:

- Artifact: `../2026-06-14-rubywaterfall-opt-107/after_cg_rosenbrock10_exact_gradient_vmi1167313_rch.txt`
- Worker: `vmi1149989`
- Row: `cg/rosenbrock_exact_gradient/10`
- Time: `[121.55 us, 126.41 us, 131.35 us]`

## Candidate

One probe lever:

- Add an opt-in in-place exact-gradient callback API.
- Reuse line-search gradient storage for callback output.
- Add Criterion row `cg/rosenbrock_inplace_gradient/10`.
- Preserve `gradient: None` and existing Vec-returning `gradient` behavior.

The source probe was restored after the benchmark gate failed.

## Behavior Proof

- Default finite-difference CG payload was byte-identical to the previous golden.
- Golden SHA-256: `92830169fd8409f2c5b7566f2378c0c79b93135ae3bda05e6d7c0a4f655e8887`
- Artifact: `golden_cg_current_payload.txt`
- Focused in-place tests passed during the probe:
  - `cg_rosenbrock_in_place_gradient_matches_vec_callback`
  - `cg_in_place_gradient_rejects_nonfinite_output`
- Existing exact-gradient tests passed after the probe.

Ordering/tie/floating-point/RNG contract:

- `gradient: None` stayed on the existing finite-difference component order.
- Existing Vec callback branch stayed higher precedence than the new probe field.
- Wolfe alpha/tie logic and PR+ recurrence were unchanged.
- No RNG or unsafe code was introduced.

## Benchmark Gate

Same-worker RCH comparison on `vmi1149989`:

| Row | Artifact | p50 |
| --- | --- | ---: |
| `cg/rosenbrock_exact_gradient/10` | `../2026-06-14-rubywaterfall-opt-107/after_cg_rosenbrock10_exact_gradient_vmi1167313_rch.txt` | `126.41 us` |
| `cg/rosenbrock_inplace_gradient/10` | `after_in_place_gradient_vmi1149989_rch.txt` | `130.98 us` |

Speedup: `126.41 / 130.98 = 0.965x`.

Score: `(Impact 0.0 * Confidence 4.0) / Effort 1.0 = 0.0`.

Verdict: REJECT. Source restored to zero `fsci-opt` diff.

## Next Route

Do not repeat in-place callback or accepted-point materialization. The next
algorithmically different route should target a fused value+gradient callback or
AD-tape primitive so Wolfe can evaluate shared objective residuals and gradients
without duplicating work. Target ratio: at least `1.20x` over
`cg/rosenbrock_exact_gradient/10`.
