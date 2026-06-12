# `frankenscipy-8l8r1.90` candidate contract: band-to-tridiagonal slice

## Selected primitive

Implement one private lower-symmetric-band to tridiagonal reduction slice for `eig_banded(..., eigvals_only=true)`.

The lever targets the current general `eig_banded` blocker: band storage is expanded back into a dense matrix before a scalar Householder tridiagonalization. The first source pass must keep eigenvector backtransform and public `eigh` dispatch out of scope.

## Profile-backed target

- Worker: `vmi1152480`
- Public baseline: `eigh_dense/256x256 = 16.181 ms`, `eigh_dense/512x512 = 109.87 ms`
- Public digest: `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`

## Touch surface

- `eig_banded` general lower-band path in `crates/fsci-linalg/src/lib.rs`
- New private band-reduction helper near `eig_banded`
- Focused tests near the existing `eig_banded_*` tests

## Excluded repeats

This candidate must not:

- use scalar reflector replay as the measured lever,
- wire through `eig_banded` dense expansion as a public route,
- repeat lower-triangle packing,
- repeat direct scalar dense tridiagonalization,
- tune output materialization,
- retune GEMM micro-kernels.

## Score estimate

Score formula: `(Impact * Confidence) / Effort`

- Impact: `4.0`, because the existing general band path pays dense expansion plus full scalar reduction.
- Confidence: `3.0`, because the target is exactly the profiled blocker but the first slice is private and values-only.
- Effort: `4.0`, because stable band bulge-chasing is larger than a loop reorder.

Estimated Score: `3.0`

## Fallback trigger

Reject the candidate if any of these hold:

- it expands band storage back to dense on the candidate path,
- it changes the public `eigh` golden digest,
- it fails shape/error parity,
- same-worker `512x512` stage speedup is below `1.20x`, making Score fall below `2.0`,
- proof cannot show tridiagonal reconstruction/residual within the existing tolerance budget.

## Proof plan

- Ordering/tie-breaking: unchanged for public `eigh`; private `eig_banded(..., eigvals_only=true)` returns sorted values through `eigh_tridiagonal`.
- Floating point: public digest must remain `0x287a5d3679a8bc6a`; private band route must prove eigenvalue agreement against the dense-expanded reference within documented tolerance.
- RNG: no RNG surface.
- Safety: no `unsafe`.
- External kernels: no C BLAS/LAPACK/MKL/XLA linkage.

## Commands

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 RCH_WORKER=vmi1152480 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo test -j 1 -p fsci-linalg --lib --locked -- \
  band_to_tridiagonal_lower --nocapture

RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 RCH_WORKER=vmi1152480 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo test -j 1 -p fsci-linalg --lib --locked -- \
  eigh_index_sort_matches_materialized_pair_sort_bits --nocapture

RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 RCH_WORKER=vmi1152480 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo test --release -j 1 -p fsci-linalg --lib --locked -- \
  band_to_tridiagonal_lower_perf_probe --ignored --nocapture --test-threads=1
```
