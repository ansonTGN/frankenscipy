# frankenscipy-8l8r1.65 rejection: square pinv certificate matmul

## Verdict

Rejected. Replacing the square `pinv_full_rank_square_lu` right-inverse
certificate product from nalgebra `DMatrix` multiplication to the crate
row-major `matmul` path was correct but too small to keep.

## Baseline

- Artifact: `baseline_public_square_pinv_route_perf_probe_rch.txt`
- Worker: `ovh-a`
- Probe: `public_square_pinv_route_perf_probe`
- `reference_pinv_ms=1039.307357`
- `routed_pinv_ms=76.403694`
- `pinv_rank=512`
- `pinv_max_abs_diff=1.54659271500712237e-14`

## Candidate evidence

- Artifact: `after_square_pinv_certificate_matmul_retry1_rch.txt`
- Worker: `vmi1227854`
- Probe passed with `routed_pinv_ms=58.606626`, `pinv_rank=512`,
  `pinv_max_abs_diff=1.54659271500712237e-14`.

The cross-worker route number looked faster, so the decision used the direct
same-worker A/B artifact below instead of accepting a mixed-worker comparison.

## Same-run A/B

- Artifact: `same_run_square_pinv_certificate_ab_rch.txt`
- Worker: `vmi1227854`
- Previous nalgebra-certificate route: `previous_route_pinv_ms=48.863010`
- Candidate matmul-certificate route: `candidate_direct_pinv_ms=47.160872`
- Direct speedup: `1.036092x`
- Public routed after: `routed_pinv_ms=47.813599`
- Rank preserved: `512`
- Max diff vs SVD reference unchanged:
  `1.54659271500712237e-14`
- Routed output matched candidate direct output exactly:
  `routed_vs_candidate_direct_pinv_max_abs_diff=0.0`

## Score

`Impact 1.036092 * Confidence 0.95 / Effort 1.0 = 0.984287`

This is below the required Score >= 2.0 keep gate. Source was restored.
