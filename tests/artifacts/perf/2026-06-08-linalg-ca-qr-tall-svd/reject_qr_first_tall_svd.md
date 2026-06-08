# Rejected Lever - Sequential QR-First Tall SVD

Bead: `frankenscipy-8l8r1.54`

## Lever

Private guarded QR-first candidate:

- `A = Q R`
- deterministic thin SVD on square `R`
- `U = Q * U_R`
- existing public reconstruction/rank/tie/sign guards

The source was restored after measurement; no `fsci-linalg` code is kept.

## Proof

Focused proof passed remotely on `vmi1167313`:

- artifact: `proof_qr_first_tall_svd_rch.txt`
- result: `1 passed; 0 failed`
- behavior checked: deterministic digest, nonincreasing singular order, public reconstruction guard, agreement with safe SVD reference, U/V orthogonality

Local fallback public-route sanity also passed:

- artifact: `after_qr_first_public_bidiag_svd_route_perf_probe_rch.txt`
- note: non-gating because RCH fell back local

## Remote Score Gate

Remote-required public-route probe selected `vmi1156319`:

- artifact: `after_qr_first_public_bidiag_svd_route_perf_probe_rch_remote_required3.txt`
- `reference_lstsq_ms=135.930455`
- `routed_lstsq_ms=165.088561`
- `lstsq_speedup=0.823379`
- `reference_pinv_ms=147.313174`
- `routed_pinv_ms=287.796269`
- `pinv_speedup=0.511866`
- `lstsq_rank=256`
- `pinv_rank=256`
- `lstsq_max_abs_diff=2.07434069920964248e-12`
- `pinv_max_abs_diff=1.95329863394988479e-13`

Same-run conclusion: QR-first was slower than the reference path for both public lanes on remote RCH. It does not clear the keep gate.

## Isomorphism

- Ordering preserved in the proof candidate by descending singular checks.
- Tie-breaking preserved by rejecting clustered/tied spectra through the public guard.
- Floating-point behavior was within tolerance in proof and remote route probes, but no source is kept.
- RNG unchanged/not used.
- Golden output unchanged by source restoration; public golden anchor remains `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.

## Decision

Rejected.

Score: Impact `4` x Confidence `1` / Effort `3` = `1.33`, below the `2.0` keep gate.

Next primitive: true communication-avoiding TSQR with block-local QR, stacked-`R` tree reduction, and reverse replay. The sequential QR stepping stone proved the public guard shape but not the performance model.
