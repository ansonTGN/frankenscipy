# frankenscipy-5mk8b rejection evidence

## Target

- Bead: `frankenscipy-5mk8b`
- Lane: `fsci-opt` nonlinear CG, `cg/rosenbrock/10`
- Candidate lever: carry the accepted strong-Wolfe gradient vector out of line search and reuse it for the next PR+ gradient instead of recomputing.

## Baseline

- Command: `rch exec -- cargo bench -p fsci-opt --bench optimize_bench --locked -- cg/rosenbrock/10 --sample-size 20`
- Artifact: `baseline_cg_rosenbrock10_current_rch.txt`
- Worker: `vmi1167313`
- Criterion: `[696.31 us, 745.53 us, 789.88 us]`

## Candidate

- Command: `rch exec -- cargo bench -p fsci-opt --bench optimize_bench --locked -- cg/rosenbrock/10 --sample-size 20`
- Artifact: `after_cg_rosenbrock10_gradient_carry_rch.txt`
- Worker: `vmi1156319`
- Criterion: `[792.97 us, 843.92 us, 904.08 us]`
- Midpoint ratio: `0.883x` (`745.53 / 843.92`)
- Score: `(Impact 0.0 * Confidence 3.0) / Effort 1.0 = 0.0`
- Verdict: reject and restore source.

## Behavior proof

- No source change was retained.
- Default finite-difference CG golden payload SHA-256 stayed `92830169fd8409f2c5b7566f2378c0c79b93135ae3bda05e6d7c0a4f655e8887`.
- Payload comparison: `golden_cg_after_payload.txt` matched both prior `golden_cg_before_payload.txt` and `golden_cg_after_payload.txt` from `2026-06-13-rubywaterfall-opt-reprofile`.
- Ordering/tie/FP/RNG: no retained algorithmic change; the rejected probe preserved the default finite-difference path before it was removed.

## Next primitive

The accepted-gradient cache is the wrong lever. The next route is a fused value-gradient CG line-search evaluator: compute Wolfe trial value and derivative evidence through one certified evaluator with reusable point/gradient storage, preserving finite-difference component order, Wolfe alpha/tie decisions, PR+ beta sequence, `nfev`/`njev`, and all floating-point bits. Target ratio remains at least `1.35x` on a fresh RCH `cg/rosenbrock/10` baseline.
