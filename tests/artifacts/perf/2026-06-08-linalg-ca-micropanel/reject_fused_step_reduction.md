# Reject: fused single-step Golub-Kahan pass

Bead: `frankenscipy-8l8r1.51`

## Target

Profile-backed target after `frankenscipy-8l8r1.50`: the 1024x512
Golub-Kahan bidiagonal reduction remained the dominant public SVD-family phase.

Fresh RCH baseline:

- Worker: `vmi1153651`
- Probe: `bidiag_large_reduction_perf_probe`
- Time: `523.978408 ms`
- Digest: `0x90cdd3f8f71ed2c1`

Public behavior anchor:

- Probe: `public_svd_lstsq_pinv_golden_payload`
- Payload SHA-256: `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`

## Lever Tried

Trial source fused the left-reflector trailing updates that feed the next
right-reflector dot pass. The reflector construction order stayed serial:
left reflector, row values for the right reflector, right reflector, then
right update. Per-cell dot and update order were preserved.

## Proof

RCH worker `vmi1167313`:

- `bidiag_fused_step_matches_workspace_reference_bits` passed.
- The proof compared diagonal, superdiagonal, bidiagonal matrix entries,
  reflector starts, reflector `tau`, and reflector values by exact `f64` bits.

Same-binary A/B probe on `vmi1167313`:

- Workspace-reference digest: `0x90cdd3f8f71ed2c1`
- Fused-step digest: `0x90cdd3f8f71ed2c1`

Public routes were not kept on the trial source because the performance gate
failed before commit.

## Rebench

RCH worker `vmi1167313`, same binary:

- Workspace reference: `430.168251 ms`
- Fused step: `404.088169 ms`
- Speedup: `1.064541x`

## Decision

Reject. The trial preserved exact reduction bits, but it missed the bead target
of at least `1.15x` and does not clear the keep discipline for this campaign.

Score: `1.6 = Impact 1.064541 * Confidence 4.5 / Effort 3.0`.

The source was restored before staging. The next primitive is a true
two-stage/packed-panel bidiagonalization route: reduce dense `A` to band form
with reusable packed panels and safe-Rust GEMM-shaped updates, then chase the
band to bidiagonal form under an explicit digest/public-golden migration
contract. Do not retry single-step pass fusion, replay ordering, dense
compact-WY composition, or thread fanout micro-levers for this hotspot.
