# frankenscipy-8l8r1.73 split tridiagonal-QR backend rejection

Date: 2026-06-09
Agent: BlackThrush
Worktree: `/data/projects/.scratch/frankenscipy-mifdz-codex-20260609-2059`

## Target

`frankenscipy-8l8r1.73` targets dense symmetric `eigh` core replacement after the
post-GEMM reprofile put `eigh_dense/512x512` back at the top of the public linalg
rows.

The attempted lever was deliberately narrow: use nalgebra's safe-Rust symmetric
tridiagonal reduction, then feed the resulting `(d, e)` into the existing
`symmetric_tridiagonal_qr_eigen` helper. The public `eigh` route was not retained.

## Baseline

RCH Criterion baseline requested `vmi1227854`; RCH selected `ovh-a`.

Command:

```text
RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1227854 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench -- eigh_dense --noplot --sample-size 10 --warm-up-time 1 --measurement-time 2
```

Measured rows:

```text
eigh_dense/256x256      time:   [11.946 ms 12.027 ms 12.162 ms]
eigh_dense/512x512      time:   [113.22 ms 143.20 ms 161.97 ms]
```

The 512x512 row was noisy, so the candidate also used a same-process public-vs-
candidate probe on `ovh-a`.

Focused golden proof on current public output selected `vmi1227854` and passed:

```text
eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a
test result: ok. 1 passed; 0 failed
```

## Candidate Evidence

RCH candidate probe requested and selected `ovh-a`.

Command:

```text
RCH_REQUIRE_REMOTE=1 RCH_WORKER=ovh-a rch exec -- cargo test -p fsci-linalg --release --lib --locked public_eigh_tridiagonal_qr_candidate_perf_probe -- --ignored --nocapture --test-threads=1
```

Same-process rows:

```text
256x256:
public_eigh_ms=13.976638
tridiagonal_qr_candidate_ms=14.566977
speedup=0.959474
candidate_sweeps=451
eigenvalue_max_abs_diff=1.12549969344399869e-11
reconstruction_max_abs=3.18323145620524883e-12
orthogonality_max_abs=2.72975761512061135e-15
public_digest=0xb2c0047b36b19aa2
candidate_digest=0x92ec16069a9906d0
candidate_raw_public_bits_equal=false

512x512:
public_eigh_ms=96.052450
tridiagonal_qr_candidate_ms=97.245944
speedup=0.987727
candidate_sweeps=923
eigenvalue_max_abs_diff=2.81943357549607754e-11
reconstruction_max_abs=1.11413100967183709e-11
orthogonality_max_abs=4.44089209850062616e-15
public_digest=0x4729412f23d9f9e4
candidate_digest=0x4f9c72e2e8565635
candidate_raw_public_bits_equal=false
```

## Isomorphism Decision

The candidate keeps the same observable sorting policy by applying the existing
`f64::total_cmp` index order after the candidate backend. There is no RNG,
unsafe code, or external BLAS/LAPACK surface.

The numerical invariant checks pass within the current tolerance contract:
eigenvalue drift, reconstruction error, and orthogonality error are all small.
However, the candidate does not preserve raw public eigenvector bits or the
public digest, and it is slower in the same-process comparison at both measured
sizes. That fails the campaign keep gate.

## Verdict

Rejected and source restored.

Do not repeat this split route: nalgebra full-to-tridiagonal plus repo QR does
not clear the speed gate and does not preserve the raw public output contract.
The next dense-`eigh` primitive should be a fundamentally different backend,
such as true blocked full-to-band plus band-to-tridiagonal SBR, or a compact-WY
full-to-tridiagonal implementation with a raw-public-digest fallback gate.
