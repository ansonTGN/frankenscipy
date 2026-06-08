# fsci-linalg eigh_tridiagonal vector residual fix

Bead: `frankenscipy-me4pf`

## Target

`eigh_tridiagonal(d, e, eigvals_only=false)` returned correct eigenvalues but wrong eigenvectors for zero-diagonal tridiagonal matrices with +/- paired spectra. This blocked the O(n^2) Golub-Welsch route for orthogonal-polynomial roots and carried an explicit profile note: the old public tridiagonal vector path measured about 1.66 s at `n=1000`, slower than dense eigensolve.

Lever: route only the eigenvector-producing path through the existing scaled symmetric-tridiagonal QR helper used by the private bidiagonal SVD backend. The `eigvals_only=true` path remains on the previous implementation.

## Baseline and After

RCH baseline repro, remote `vmi1293453`, `baseline_repro_rch.txt`:

- generic-spectrum worst residual: `3.553e-15`
- zero-diagonal +/- pair-spectrum worst residual: `1.500e0`
- status: bug present

RCH after repro, remote `vmi1149989`, `after_repro_rch.txt`:

- generic-spectrum worst residual: `1.776e-15`
- zero-diagonal +/- pair-spectrum worst residual: `7.772e-16`
- pair-spectrum `n=1000`: `486.830 ms`, worst residual `1.226e-13`
- `eigvals_only n=1000` digest: `0xdb6d3dfa8e0ca509`
- status: fixed

Perf note: the after timing is directional against the bead's recorded old `n=1000` timing of about `1.66s`; the primary keep gate for this bead is correctness of the vector path plus no regression to the unchanged eigenvalue-only path.

## Isomorphism / Migration Proof

- API and validation: input length checks, finite checks, dimension checks, trace operation name, and error type remain the same.
- Ordering: output eigenpairs are still sorted ascending with the same `partial_cmp(...).unwrap_or(Equal)` comparator.
- Tie-breaking: equal-key sorting still follows the existing stable index sort behavior.
- Floating point: `eigvals_only=true` is byte-route unchanged; the vector-producing path intentionally migrates from the defective inline QR/QL rotations to the scaled tridiagonal QR helper.
- RNG: none.
- Golden evidence: `eigvals_only n=1000` digest is pinned at `0xdb6d3dfa8e0ca509`; vector-path golden behavior is the residual oracle, now below tolerance for the pair-spectrum reproducer.

## Validation

- `rch exec -- cargo run -p fsci-linalg --release --bin check_eigh_tridiag_vectors --locked --quiet`
- `rch exec -- cargo test -p fsci-linalg --lib eigh_tridiagonal --locked -- --nocapture`
- `rch exec -- cargo check -p fsci-linalg --all-targets --locked`
- `rch exec -- cargo clippy -p fsci-linalg --all-targets --no-deps --locked -- -D warnings`
- `ubs crates/fsci-linalg/src/lib.rs crates/fsci-linalg/src/bin/check_eigh_tridiag_vectors.rs`: exit 0, critical issues 0
