# Reject sparse band bulge chase and route deeper

Bead: `frankenscipy-8l8r1.90`

Candidate: private lower-symmetric-band to tridiagonal eigvals-only slice for
`eig_banded(..., eigvals_only=true)` using sparse `BTreeMap` row storage and
adjacent Givens rotations.

Verdict: rejected. The proof was clean, but the release perf probe showed the
map-backed chase is substantially slower than the dense-expanded scalar
reference. The source lever was restored to zero diff.

## Evidence

- Proof: `proof_band_to_tridiagonal_lower_rch.txt`
  - Worker: `vmi1152480`
  - Result: 2 passed, 1 ignored
  - Eigenvalue agreement: proof fixtures passed against dense symmetric
    reference values
  - sha256: `539e7174035d8034b999dbd498cb4d7e5482d9c433cd0cabb4a6ae021a5e965f`
- Public golden: `proof_public_eigh_golden_after_band_tridiagonal_rch.txt`
  - Worker: `ovh-a`
  - Digest: `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`
  - sha256: `ad0454c2649903ed9bdeb26308dc529da5c64b4ac9a18cde169d2699c16f2aa1`
- Release perf probe: `after_band_to_tridiagonal_perf_probe_rch.txt`
  - Worker: `ovh-a`
  - `256x256`: dense-expanded reference `66.890671 ms`, candidate
    `211.760049 ms`, speedup `0.315880x`
  - `512x512`: dense-expanded reference `512.075240 ms`, candidate
    `1562.249593 ms`, speedup `0.327781x`
  - Max absolute eigenvalue drift: `1.79625203600153327e-11`
  - sha256: `7477b565814151216220cd4dbd983c4a25454943b53c53d6133b5a43cb04ca3e`

## Score

`(Impact 0.5 * Confidence 4.0) / Effort 4.0 = 0.5 < 2.0`

## Next primitive

Do not repeat sparse maps, scalar replay, direct public dense re-expansion,
lower packing, direct scalar tridiagonalization, output materialization, or
old GEMM retunes.

Next attack: a packed-band implicit bulge-chase kernel with contiguous
`(2 * bandwidth + 1) * n` storage, fixed-window rotations, and no dynamic map
lookups. The target is at least `1.25x` over the dense-expanded reference at
`512x512` before public wiring.
