# Rejected: square pinv certificate matmul

Bead: `frankenscipy-8l8r1.65`

Profile-backed target: `PUBLIC_SQUARE_PINV_ROUTE_PERF` at shape `512x512`.
The serial public-route reprofile left square `pinv` as the largest remaining
public-route absolute cost, with `routed_pinv_ms=53.020724` on `vmi1227854`.

Lever tested: keep the existing nalgebra inverse in `pinv_full_rank_square_lu`,
but replace the right-inverse acceptance product `A * A^-1` with the project
row-major/register-blocked `matmul(a, inverse_rows)` certificate path.

RCH evidence:

- Baseline on `ovh-a`: `reference_pinv_ms=1039.307357`,
  `routed_pinv_ms=76.403694`, speedup `13.602842`, `rank=512`,
  `pinv_max_abs_diff=1.54659271500712237e-14`.
- After on `ovh-a`: `reference_pinv_ms=1059.184227`,
  `routed_pinv_ms=87.435620`, speedup `12.113876`, `rank=512`,
  `pinv_max_abs_diff=1.54659271500712237e-14`.

Decision: rejected. Same-worker routed time regressed
`76.403694 ms -> 87.435620 ms` (`0.874277x`). Impact is negative, so the keep
score is `0.0`, below the `>=2.0` gate.

Behavior proof: rank stayed `512`; max absolute difference against the SVD
reference stayed `1.54659271500712237e-14`. Ordering, tie-breaking, and RNG
behavior are unchanged. The production certificate-product hunk was restored,
so no `.65` source change is retained.

Next route: the inverse kernel and certificate product are not the right square
`pinv` primitive. Continue with the next profile-backed linalg bead rather than
repeating blocked-LU cutoff or certificate-matmul families.
