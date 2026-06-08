# Pass 1 Baseline And Contract

Bead: `frankenscipy-8l8r1.54`

## Claim

`frankenscipy-8l8r1.54` is in progress and assigned to `BlackThrush`.
The target is a communication-avoiding QR/TSQR-first tall SVD route after the
compact-WY left-panel helper was rejected in `.53`.

## Baselines

Initial RCH attempts for criterion, route timing, and reducer timing fell back
local because no admissible workers were available. Those local results are
kept as non-gating diagnostics only:

- local criterion `lstsq/512x256`: `[53.506 ms 54.330 ms 55.271 ms]`
- local criterion `pinv/512x256`: `[57.707 ms 58.939 ms 60.323 ms]`
- local public route `lstsq`: `94.662316 ms -> 100.638298 ms`
- local public route `pinv`: `107.791168 ms -> 94.716448 ms`
- local reducer: `225.686852 ms`, digest `0x90cdd3f8f71ed2c1`

Gating remote baselines:

```text
worker=vmi1153651
command=cargo bench -p fsci-linalg --bench linalg_bench --locked -- --warm-up-time 1 --measurement-time 2 'lstsq/512x256|pinv/512x256' --noplot
lstsq/512x256=[174.74 ms 191.04 ms 209.93 ms]
pinv/512x256=[233.44 ms 267.15 ms 307.79 ms]
```

```text
worker=vmi1167313
command=cargo test -p fsci-linalg --release --lib --locked public_bidiag_svd_route_perf_probe -- --ignored --nocapture
shape=512x256
reference_lstsq_ms=127.906224
routed_lstsq_ms=117.018190
lstsq_speedup=1.093046
reference_pinv_ms=140.152702
routed_pinv_ms=123.589790
pinv_speedup=1.134015
lstsq_rank=256
pinv_rank=256
lstsq_max_abs_diff=1.07647224467655178e-12
pinv_max_abs_diff=2.28428387316625958e-14
```

```text
worker=vmi1167313
command=cargo test -p fsci-linalg --release --lib --locked bidiag_large_reduction_perf_probe -- --ignored --nocapture
shape=1024x512
elapsed_ms=348.394210
digest=0x90cdd3f8f71ed2c1
```

## Golden Output

RCH worker `vmi1167313` passed `public_svd_lstsq_pinv_golden_payload`.
The extracted payload SHA is:

```text
1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
```

This remains the public golden-output gate for any `.54` source change.

## Opportunity Matrix

Candidate: CA-QR/TSQR-first tall full-rank SVD route.

- Impact: `5` because the route can bypass the full Golub-Kahan reducer for
  tall full-rank matrices and work on the small `R` factor.
- Confidence: `3` before implementation because prior public-route probes show
  the current bidiagonal route is only `1.09x` to `1.13x` faster than nalgebra
  reference on 512x256, while the 1024x512 reducer alone still costs
  `348.394210 ms`.
- Effort: `5` because rank/sign/order/reconstruction guards are strict.
- Score: `3.0`.

Keep gate: Score must stay `>=2.0` after same-worker RCH measurement. Target
is at least `1.25x` on the profile-backed tall public lane before public
routing is retained.

## Behavior Contract

- Ordering preserved: singular values must remain descending. Equal/tied values
  must use the existing deterministic order/sign policy or reject the route.
- Tie-breaking unchanged: rank thresholds, boundary gaps, and clustered-spectrum
  rejection must remain at least as strict as `public_bidiag_svd_accepts`.
- Floating point: the CA-QR route may have different internal summation order,
  so public acceptance must require strict reconstruction/orthogonality/rank
  checks before returning. Public golden SHA must remain unchanged.
- RNG: unchanged; no randomization is allowed.
- Public route isolation: no public route switch until the private/guarded
  helper proves reconstruction, orthogonality, rank, sign, and golden output.
- Dependencies: no unsafe, no C BLAS/LAPACK, no MKL/XLA, and no thread fanout
  for this lever.

## Artifact Index

- `br_show_frankenscipy-8l8r1.54.json`
- `rch_cargo_bench_lstsq_pinv_512x256_retry.raw.txt`
- `rch_public_bidiag_svd_route_perf_probe_retry.raw.txt`
- `rch_bidiag_large_reduction_perf_probe_retry.raw.txt`
- `rch_public_svd_lstsq_pinv_golden_payload.raw.txt`
- `public_svd_lstsq_pinv_golden_payload.txt`
- `public_svd_lstsq_pinv_golden_payload.sha256`
