# Stage 4i rejection: tridiagonal-eigh backend for private bidiagonal SVD

## Profile-backed target

Current private `1024x512` bidiagonal reduction remains stable:

- RCH worker: `vmi1227854`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib bidiag_large_reduction_perf_probe --locked -- --ignored --nocapture`
- Baseline: `elapsed_ms=170.413997`
- Digest: `0x90cdd3f8f71ed2c1`
- First diagonal: `-1.00455335940616146e3`
- Last diagonal: `-6.45492359226604862e1`

The post-Stage-4h thin-factor probe shows the bottleneck has shifted beyond
factor assembly:

- RCH worker: `ts1`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib thin_bidiag_factor_replay_perf_probe --locked -- --ignored --nocapture`
- Test wall time: `4.22s`
- Dense factor product timing: `535.722598 ms`
- Reflector replay timing: `286.873171 ms`
- Replay speedup: `1.867455x`
- Reduction digest: `0x90cdd3f8f71ed2c1`
- `U` max drift vs dense reference: `5.10702591327572009e-15`
- `Vt` max drift vs dense reference: `2.33146835171282873e-15`

The unprinted stage cost is therefore dominated by the private bidiagonal
SVD/eigensolver path, not by the already-shipped factor replay.

## Candidate lever

Replace `deterministic_bidiagonal_svd`'s dense cyclic Jacobi solve over the
explicit Gram matrix with the existing in-repo `eigh_tridiagonal` helper over
the Gram diagonal/offdiagonal:

- `gram_diagonal[i] = d[i]^2 + e[i-1]^2`
- `gram_offdiagonal[i] = d[i] * e[i]`
- keep descending singular-value ordering with deterministic index tie-breaks
- keep sign canonicalization, zero-singular-vector fill, RNG absence, and public
  SVD/lstsq/pinv routes unchanged

## Proof result

Rejected before benchmarking.

- RCH worker: `vmi1156319`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib bidiag_svd_tridiagonal_eigh_matches_jacobi_reference --locked -- --nocapture`
- Result: failed focused proof
- Failure: `tridiagonal route U orthogonality 2.05564191299575011e-1`

The existing public tridiagonal QL helper is not a valid drop-in backend for
the private bidiagonal SVD vector contract. It may produce eigenvalues useful
for public `eigh_tridiagonal` coverage, but its vectors do not satisfy the
orthogonality obligation needed by `B = U Sigma Vt`.

After source restoration, the focused public SVD/lstsq/pinv golden payload was
rerun:

- RCH worker: `vmi1227854`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib public_svd_lstsq_pinv_golden_payload --locked -- --nocapture`
- Result: passed
- Public golden SHA-256: `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`

## Isomorphism

- Ordering preserved: not accepted; proof failed before benchmark.
- Tie-breaking unchanged: not accepted; proof failed before benchmark.
- Floating-point behavior: not accepted; private SVD vectors violated
  orthogonality.
- RNG seeds: unchanged / not used.
- Public golden outputs: unchanged after source restoration; SHA-256
  `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.

## Decision

Reject and restore source. Score `0.0`.

Next primitive: implement a real bidiagonal-specialized SVD backend with proof
obligations, not a blind adapter around the current public tridiagonal helper.
Candidate families:

- implicit-shift bidiagonal QR with stable vector accumulation,
- dqds for singular values plus inverse iteration / MRRR-style vector recovery,
- divide-and-conquer bidiagonal SVD for larger panels.

Target ratio: `>=2.5x` on the private `1024x512` SVD stage with reconstruction,
orthogonality, ordering/tie-break, and public golden SHA evidence before any
public wiring.
