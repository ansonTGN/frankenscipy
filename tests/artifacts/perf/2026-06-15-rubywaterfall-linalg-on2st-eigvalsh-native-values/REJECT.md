# `frankenscipy-on2st` Rejection Evidence

## Candidate

Route large public `eigvalsh` calls through the existing safe-Rust dense rank-2 tridiagonalization and eigenvalue-only tridiagonal solver, skipping eigenvector back-transform.

## Baseline

Source: `baseline_public_eigvalsh_values_rch.txt`

- Worker: RCH `ovh-a`
- 256x256 public `eigvalsh`: `19.241456 ms`
- 512x512 public `eigvalsh`: `55.314643 ms`
- 512x512 values digest: `0x72cfa53bb61e39ef`

## Candidate Results

Source: `after_public_eigvalsh_native_values_rch.txt`

- Worker: RCH `ovh-a`
- 256x256 public `eigvalsh`: `7.352330 ms` (below the route threshold; retained only as timing noise)
- 512x512 public `eigvalsh`: `82.310942 ms`
- 512x512 values digest: `0x0834e54681a0bd80`

Source: `after_public_eigvalsh_native_values_ab_rch.txt`

- Worker: RCH `vmi1227854`
- 512x512 candidate: `197.271351 ms`
- 512x512 direct nalgebra values in the same binary: `32.252697 ms`
- 800x800 candidate: `178.869061 ms`
- 800x800 direct nalgebra values in the same binary: `107.751859 ms`
- Max drift versus direct nalgebra stayed within the probe tolerance.

## Verdict

Rejected. The current dense Householder native values route is slower than direct nalgebra values at every routed size measured. Source was restored to zero diff.

Do not retry this direct route. The next eigvalsh/eigh values-only attempt should use a different primitive: divide-and-conquer tridiagonal values, MRRR-style representation work, or the compact-WY panel-generation route tracked in `frankenscipy-w2pp0`.
