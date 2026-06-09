# frankenscipy-8l8r1.76 rank-2 dense-eigh candidate rejection

Date: 2026-06-09
Agent: BlackThrush
Worktree: `/data/projects/.scratch/frankenscipy-codex-opt-20260609-2148`

## Target

`frankenscipy-8l8r1.76` followed the scalar SBR rejection and tested a
different dense symmetric `eigh` primitive as a private ignored probe. The
candidate used direct symmetric Householder tridiagonalization with the standard
rank-2 trailing update `A := A - v w^T - w v^T`, accumulated Q, solved the
tridiagonal problem with the existing safe-Rust helper, then backtransformed
eigenvectors. Public `eigh` was not changed.

## Baseline

RCH Criterion selected `ovh-a`.

```text
eigh_dense/256x256      time:   [12.524 ms 12.600 ms 12.811 ms]
eigh_dense/512x512      time:   [99.671 ms 100.35 ms 100.99 ms]
```

Focused public golden proof selected `ovh-a` and preserved:

```text
eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a
```

## Candidate Evidence

The first pinned and unpinned candidate attempts correctly refused local fallback
while RCH had no admissible worker. The successful candidate probe used Cargo
`-j 1` and selected `vmi1227854`.

```text
256x256:
public_eigh_ms=12.055060
rank2_candidate_ms=82.760864
speedup=0.145661
tridiagonal_residual=0.00000000000000000e0
eigenvalue_max_abs_diff=1.04591890703886747e-11
reconstruction_max_abs=5.57065504835918546e-12
orthogonality_max_abs=6.66133814775093924e-15
public_digest=0xb2c0047b36b19aa2
candidate_digest=0x27a078d6e712080e
candidate_raw_public_bits_equal=false

512x512:
public_eigh_ms=98.515578
rank2_candidate_ms=1183.960386
speedup=0.083209
tridiagonal_residual=0.00000000000000000e0
eigenvalue_max_abs_diff=2.68300937023013830e-11
reconstruction_max_abs=2.00088834390044212e-11
orthogonality_max_abs=1.03250741290139558e-14
public_digest=0x4729412f23d9f9e4
candidate_digest=0x9420f0cec14e2cd0
candidate_raw_public_bits_equal=false
```

## Isomorphism Decision

The candidate preserved validation inputs, used the existing tridiagonal solver's
ascending eigenvalue ordering, introduced no RNG, no unsafe code, and no external
BLAS/LAPACK/MKL/XLA dependency. Numerical invariants passed: tridiagonal
residual, eigenvalue drift, reconstruction, and orthogonality stayed within the
current tolerance contract.

It failed both retention gates:

- Same-process speed was much worse than public `eigh`: `0.145661x` at `256x256`
  and `0.083209x` at `512x512`.
- Raw public output bits differed, so it cannot replace public `eigh` under the
  current digest contract.

## Verdict

Rejected and source restored.

Do not repeat unblocked direct rank-2 Householder tridiagonalization. The next
dense-`eigh` primitive must be structurally different: true compact-WY blocked
panels with a BLAS-3-style symmetric far update and a direct band-to-tridiagonal
bulge-chasing stage, with stage timings before any public route attempt.
