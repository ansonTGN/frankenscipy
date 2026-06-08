# frankenscipy-8l8r1.50 - column-slice Householder keep

## Lever

`apply_householder_left` and `apply_householder_right_with_workspace` now use direct `DMatrix::as_mut_slice()` column-major offsets. The scalar algorithm, loop nesting, dot accumulation order, update order, zeroing order, reflector construction, singular ordering, tie breaking, and SVD sign canonicalization are unchanged.

## Baseline and profile

- Worker: `fmd`.
- Public Criterion baseline:
  - `lstsq/512x256`: `[56.341 ms 59.522 ms 62.798 ms]`
  - `pinv/512x256`: `[67.778 ms 71.677 ms 75.977 ms]`
- Phase split:
  - `golub_kahan_bidiagonal_reduction` 1024x512: `185.095579 ms`, digest `0x90cdd3f8f71ed2c1`
  - bidiagonal SVD backend: `52.554489 ms`, `backend_sweeps=875`, digest `0x7c2787acb98e625f`

## Proof

- `bidiag_right_workspace_matches_rowwise_reference_bits`: passed on RCH worker `fmd`.
- Reduction digest before and after: `0x90cdd3f8f71ed2c1`.
- Public golden SHA-256 before and after: `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
- Isomorphism: the lever changes only address calculation. Dot products still visit reflector entries in ascending offset order for each column/row, updates still apply in the same order, no RNG is involved, and singular ordering/sign canonicalization code is untouched.

## Result

- Reduction phase: `185.095579 ms -> 178.308945 ms`, `1.038061x`.
- Public `lstsq/512x256`: `59.522 ms -> 48.483 ms`, `1.227688x`.
- Public `pinv/512x256`: `71.677 ms -> 69.498 ms`, `1.031353x`.
- Score: `Impact 3 * Confidence 4 / Effort 2 = 6.0`; keep.

## Validation

- `cargo fmt -p fsci-linalg --check`: passed.
- `git diff --check -- crates/fsci-linalg/src/lib.rs`: passed.
- `RCH_REQUIRE_REMOTE=1 RCH_FORCE_REMOTE=1 RCH_WORKER=fmd rch exec -- cargo check -p fsci-linalg --all-targets --locked`: passed.
- `RCH_REQUIRE_REMOTE=1 RCH_FORCE_REMOTE=1 RCH_WORKER=fmd rch exec -- cargo clippy -p fsci-linalg --all-targets --no-deps --locked -- -D warnings`: passed.
- `ubs crates/fsci-linalg/src/lib.rs`: zero critical findings.

## Next primitive

Profile still points at the Golub-Kahan reduction. Next target: communication-avoiding micro-panel trailing update with exact scalar-equivalence digest preservation, not final materialization or reflector replay.
