# `frankenscipy-8l8r1.90` baseline and proof contract

## Scope

- Bead: `frankenscipy-8l8r1.90`
- Target: `fsci-linalg` public dense `eigh` residual plus the private compact-WY full-to-band route.
- Source edit at baseline: none.
- Clean worktree: `/data/projects/.scratch/frankenscipy-8l8r1-90-baseline-20260611T2132Z`
- Baseline commit: `7c74e665`
- RCH worker: `vmi1152480`

## Baseline

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 RCH_WORKER=vmi1152480 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench --locked -- eigh_dense
```

Criterion means:

- `eigh_dense/256x256`: `16.181 ms`
- `eigh_dense/512x512`: `109.87 ms`

Artifact:

- `baseline_eigh_dense_criterion_rch.txt`
- SHA-256: `4783bba47a5dd4c1fad7cab0a842a7f6f20ddc5c289e86288113bfc5cd6a7832`

## Public Golden Proof

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 RCH_WORKER=vmi1152480 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo test -j 1 -p fsci-linalg --lib --locked -- \
  eigh_index_sort_matches_materialized_pair_sort_bits --nocapture
```

Result:

- `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`
- `1 passed; 0 failed`
- Artifact SHA-256: `3fdbd698b60bce05128dbecc9c3215e4b3199326dec2b43444ec029a149cd3cc`

## One-Lever Contract

The next source edit must be one of:

1. a fused compact-WY full-to-band generator that builds panel state while reducing, or
2. a first real band-to-tridiagonal/bulge-chasing slice that avoids dense re-expansion.

Excluded repeats:

- scalar reflector replay,
- direct public wiring through `eig_banded` dense expansion,
- lower-triangle packing,
- direct scalar tridiagonalization,
- output materialization,
- old GEMM micro-levers.

## Isomorphism Proof Plan

- Ordering and tie-breaking: preserve public ascending `f64::total_cmp` order or keep route private.
- Floating point: public golden digest must stay `0x287a5d3679a8bc6a`; private route must prove similarity, orthogonality, outside-band zeros, and bounded residual against scalar reference.
- RNG: no RNG surface.
- Safety: no `unsafe`.
- External kernels: no C BLAS/LAPACK/MKL/XLA linkage.
- Keep gate: same-worker RCH rebench on `vmi1152480`; Score must be `>= 2.0`.
