# Reject: square pinv 512 blocked-LU cutoff

Bead: `frankenscipy-8l8r1.64`

## Target

Corrected serial public-route reprofile ranked square `pinv` as the largest
remaining public-route absolute cost:

- Worker: `vmi1227854`
- `PUBLIC_SQUARE_PINV_ROUTE_PERF`
- `reference_pinv_ms=1602.486446`
- `routed_pinv_ms=53.020724`
- `pinv_speedup=30.223775`
- `rank=512`
- `pinv_max_abs_diff=1.54659271500712237e-14`

Focused baseline before the lever:

- Worker: `ovh-a`
- `reference_pinv_ms=1046.562255`
- `routed_pinv_ms=53.052269`
- `pinv_speedup=19.727003`
- `rank=512`
- `pinv_max_abs_diff=1.54659271500712237e-14`

## Lever Tried

Inside `pinv_full_rank_square_lu` only, route `n >= 512` square `pinv`
through the in-house `inv_blocked` inverse while leaving the global
`BLOCKED_LU_MIN_DIM` threshold unchanged for `solve` and `inv`.

## Result

Same worker (`ovh-a`) after:

- `reference_pinv_ms=1032.992178`
- `previous_route_pinv_ms=44.175733`
- `routed_pinv_ms=49.420258`
- `pinv_speedup=20.902201`
- `pinv_speedup_vs_previous_route=0.893879`
- `pinv_rank=512`
- `pinv_max_abs_diff=1.53895993171282441e-14`
- `previous_route_pinv_max_abs_diff=1.54659271500712237e-14`
- `routed_vs_previous_route_pinv_max_abs_diff=4.53977133663130417e-15`

Behavior proof passed, but the in-house blocked LU inverse is slower than the
existing nalgebra inverse kernel at this size. The cross-run route delta
(`53.052269 ms -> 49.420258 ms`) is too small for Score >= 2.0, and the
same-run previous-kernel comparison is negative.

## Verdict

Rejected. Source restored. Do not lower the square `pinv` blocked-LU cutoff to
512; keep the in-house blocked inverse for larger matrices where the original
artifact expected it to pay off.
