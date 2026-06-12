# `frankenscipy-8l8r1.91` baseline contract

## Target

`eig_banded(..., lower=true, eigvals_only=true)` for general lower symmetric
band storage.

The current implementation expands band storage to a full dense symmetric
matrix, applies scalar Householder tridiagonalization, and then solves the
tridiagonal eigenproblem. The new candidate must avoid dense expansion and
avoid the rejected sparse `BTreeMap` chase from `frankenscipy-8l8r1.90`.

## Baseline Evidence

The immediately preceding `.90` RCH release probe measured the same dense
expanded scalar reference in the same executable as the rejected sparse-band
candidate:

- Artifact: `tests/artifacts/perf/2026-06-11-linalg-8l8r1-90/after_band_to_tridiagonal_perf_probe_rch.txt`
- Worker: `ovh-a`
- `256x256`, bandwidth `32`: dense-expanded reference `66.890671 ms`
- `512x512`, bandwidth `32`: dense-expanded reference `512.075240 ms`
- Artifact sha256: `7477b565814151216220cd4dbd983c4a25454943b53c53d6133b5a43cb04ca3e`

Public `eigh` ordering/golden baseline:

- Digest: `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`
- Artifact: `tests/artifacts/perf/2026-06-11-linalg-8l8r1-90/proof_public_eigh_golden_after_band_tridiagonal_rch.txt`
- Artifact sha256: `ad0454c2649903ed9bdeb26308dc529da5c64b4ac9a18cde169d2699c16f2aa1`

## Candidate Contract

One lever only: packed contiguous band-to-tridiagonal values-only slice for
lower symmetric band storage.

Acceptable directions:

- fixed-width packed band workspace with a bounded multiple of `bandwidth * n`
  storage, currently `6 * bandwidth + 7` diagonals including the main diagonal,
- local-window implicit Givens/bulge chase that updates only active bands,
- values-only path first; eigenvector backtransform and public dense `eigh`
  dispatch remain out of scope.

Rejected repeats:

- sparse maps or hash maps,
- scalar reflector replay,
- direct dense re-expansion,
- lower-triangle packing,
- direct scalar dense tridiagonalization,
- output materialization,
- old GEMM retunes,
- external BLAS/LAPACK/MKL/XLA or `unsafe`.

## Proof Requirements

- Shape/error/finite-check behavior remains compatible with `eig_banded`.
- Eigenvalues agree with the dense symmetric reference within the existing
  tolerance budget.
- Public `eigh` golden digest remains `0x287a5d3679a8bc6a`.
- No RNG path is introduced.
- No `unsafe` code is introduced.

## Keep Gate

Target: at least `1.25x` over the dense-expanded reference at `512x512` on the
release perf probe, with Score `>= 2.0`.

Reject immediately if the candidate is not faster than the dense-expanded
reference, fails eigenvalue agreement, changes public golden digest, or sneaks
in a rejected route family.
