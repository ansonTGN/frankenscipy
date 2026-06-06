# Stage 2 Keep: Deterministic Bidiagonal SVD Solver

Bead: `frankenscipy-ffmf8`

## Target

Continue the profile-backed `fsci-linalg` full-rank rectangular SVD-family chain
from `frankenscipy-vgs9h` by adding the private deterministic solver for the
upper-bidiagonal matrix produced by the Golub-Kahan reduction.

Public `svd`, `svdvals`, `lstsq`, and `pinv` remain on the existing `safe_svd`
route in this stage.

## Baseline

RCH Criterion, `ts1`:

- `lstsq/512x256`: `[85.447 ms 86.395 ms 87.297 ms]`
- `pinv/512x256`: `[88.135 ms 88.661 ms 89.145 ms]`

Artifact:
`baseline_bidiag_svd_stage2_lstsq_pinv_512x256_criterion_rch.txt`

## Lever

Added a private safe-Rust deterministic bidiagonal SVD primitive:

- builds the symmetric tridiagonal `B^T B` from diagonal/superdiagonal inputs
- solves it with a bounded cyclic Jacobi eigensolver
- sorts singular values descending with deterministic tie fallback
- canonicalizes right-singular-vector signs deterministically
- reconstructs compact left singular vectors from `B * v / sigma`
- fills zero-singular left vectors by deterministic Gram-Schmidt basis selection
- rejects invalid shapes and non-finite inputs

No unsafe code and no C BLAS/LAPACK linkage.

## Alien And Math Lineage

- Alien graveyard `Communication-Avoiding Algorithms`: this stage continues the
  blocked Householder bidiagonal SVD route, with explicit reconstruction and
  orthogonality proof gates before any public route can use it.
- Alien artifact numerical linear algebra contract: preserve factorization
  accuracy, singular-vector orthogonality, convergence bounds, condition/fallback
  behavior, and byte-level public golden outputs.

## Proof

RCH focused tests passed on `ts1`:

```text
cargo test -p fsci-linalg --lib bidiag_svd --locked -- --nocapture
```

Result: `5 passed; 0 failed`.

Golden payload:

- `golden_bidiag_svd_stage2_payload.txt`
- SHA-256: `b5515c3aeed29cb28cd478db936b6e0bd62cd90ef4edf06cc0aaa207345c1a1c`

Golden values:

- Jacobi sweeps: `4`
- reconstruction error: `7.10542735760100186e-15`
- compact `U` column orthogonality error: `4.44089209850062616e-16`
- `V` orthogonality error: `6.66133814775093924e-16`
- singular values:
  - `1.42729956856374152e1`
  - `8.54591244670228178e0`
  - `7.42642247299642033e0`
  - `6.32220315986258630e0`

Public behavior proof:

- RCH `pinv_full_rank_rectangular_golden_payload` passed on `ts1`
- public payload SHA-256 stayed
  `bb603e9c2452a8562c6f399ff2bce5a21b481e93080ff4ca9685e4c2e9bfe185`
- no public route changes, no RNG use, no public singular-value ordering or
  rank-threshold policy changes

## Isomorphism

- Ordering: public singular values and vectors still come from the existing
  `safe_svd` path; private Stage 2 values are sorted descending with a stable
  source-index tie fallback.
- Tie-breaking: public tie behavior is unchanged; private equal-eigenvalue ties
  keep deterministic index order and deterministic sign canonicalization.
- Floating point: public `pinv_full_rank_rectangular_golden_payload` remained
  byte-identical to Stage 1, with `cmp=0` and SHA-256
  `bb603e9c2452a8562c6f399ff2bce5a21b481e93080ff4ca9685e4c2e9bfe185`.
- RNG: none.
- Certificates and errors: public rank thresholds, fallback certificates, and
  error classes remain on the current `safe_svd` route.

## Post-Change Guard

RCH Criterion, `ts1`:

- `lstsq/512x256`: `[87.672 ms 88.588 ms 89.291 ms]`
- `pinv/512x256`: `[90.169 ms 90.633 ms 91.402 ms]`

Artifact:
`after_bidiag_svd_stage2_lstsq_pinv_512x256_criterion_rch.txt`

The solver is not wired to public APIs in this stage, so no public speedup is
claimed here. The small public benchmark shift is treated as guard variance for
the unchanged route; the keep is for the proven private solver primitive needed
by the subsequent reconstruction and wiring stages.

## Validation

- RCH `cargo check -p fsci-linalg --all-targets --locked`: passed
- RCH `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`:
  passed
- `cargo fmt -p fsci-linalg --check`: passed
- `ubs crates/fsci-linalg/src/lib.rs`: exit `0`, zero critical issues

## Risk Boundary

The private helper obtains singular values through the symmetric Gram matrix of
an already-bidiagonal factor. That is acceptable only while this helper remains
unwired and covered by reconstruction, orthogonality, convergence, and golden
tests. Public wiring must fallback for ill-conditioned inputs, ambiguous rank
thresholds, clustered ties, convergence limit hits, and any reconstruction or
orthogonality proof breach.

## Verdict

Keep as proof-stage primitive.

Score: `3.0 = Impact 5 * Confidence 3 / Effort 5`.

Next deeper primitive: thin singular-vector reconstruction from the
Golub-Kahan reflector products and deterministic bidiagonal SVD factors.
