# Backend Feature Probe - LU scalar hotspot

**Bead:** `frankenscipy-perf-linalg-lu-scalar-2y3wp`
**Skill loop:** `extreme-software-optimization`, pass 1 of 5
**Verdict:** rejected backend/config-only change; no Cargo/code change kept.

## Current Remote Baseline

Command:

```bash
rch exec -- hyperfine --setup 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_lu_probe RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve' --warmup 3 --runs 10 --export-json tests/artifacts/perf/2026-06-01-linalg-solve/baseline_backend_probe_rch.json '/tmp/rch_target_fsci_linalg_lu_probe/release-perf/perf_solve solve 1000 1 42'
```

Result for `perf_solve solve 1000 1 42`:

| metric | value |
|--------|-------|
| mean +- sigma | 130.7 ms +- 7.1 ms |
| median | 130.9 ms |
| min / max | 122.3 / 143.1 ms |
| user / system | 99.5 / 30.7 ms |
| runs | 10 |

Stage reconfirmation:

```bash
rch exec -- hyperfine --setup 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_lu_probe RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve' --warmup 2 --runs 10 --export-json tests/artifacts/perf/2026-06-01-linalg-solve/stage_backend_probe_rch.json '/tmp/rch_target_fsci_linalg_lu_probe/release-perf/perf_solve lu_factor 1000 1 42' '/tmp/rch_target_fsci_linalg_lu_probe/release-perf/perf_solve lu_solve 1000 1 42'
```

| mode | mean +- sigma | median | user / system | share |
|------|---------------|--------|---------------|-------|
| `lu_factor` | 98.5 ms +- 7.4 ms | 95.5 ms | 83.5 / 14.7 ms | 78.7% of `lu_solve`; 75.4% of `solve` |
| `lu_solve` | 125.2 ms +- 8.6 ms | 125.6 ms | 98.8 / 25.6 ms | baseline for factor+solve path |

This reconfirms the previous `hotspot_table.md` ranking: LU factorization remains the dominant measured cost.

## Backend/Feature Feasibility

- `matrixmultiply`: already active. `fsci-linalg` depends on `nalgebra = "0.34.2"` with default features, and `nalgebra/default -> std -> matrixmultiply`. Adding a Cargo feature would be a no-op.
- `matrixmultiply` effect on this hotspot: not applicable. The profiled LU path calls `gauss_step -> axpy -> axcpy_uninit`; nalgebra's `matrixmultiply::dgemm` dispatch is inside `gemm_uninit`, not the LU AXPY rank-1 update.
- `simba wide`: already active through `nalgebra/std -> simba/std -> simba/wide -> wide/std`. The active scalar type is still `f64`, so this does not vectorize `DMatrix<f64>::lu()`.
- `simba portable_simd`: not exposed by `nalgebra 0.34.2` as a nalgebra feature. For this code path, enabling it as a direct `simba` feature would not change `T = f64` scalar LU.
- `nalgebra-lapack`: available only as a separate crate/backend, not as a `nalgebra` feature that changes `DMatrix::lu()`. A dependency-only change would not affect the current call sites; a real test would require replacing the LU type/path with `nalgebra_lapack::LU` plus choosing a native LAPACK provider. That is a larger code/backend replacement, not this pass's config-only probe, and it has non-bit-exact floating-point/pivoting risk to prove separately.

## Isomorphism Proof

No code or dependency configuration was changed. Ordering, tie-breaking, floating-point operation order, and RNG are unchanged by construction.

Existing golden references remain bit-identical:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  golden/golden_before.txt
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  golden/golden_after.txt
```

## Opportunity Score

Config-only backend feature pass:

| lever | impact | confidence | effort | score | decision |
|-------|--------|------------|--------|-------|----------|
| Add/enable `matrixmultiply` | 0 | 5 | 1 | 0.0 | reject; already enabled and not on LU AXPY path |
| Add/enable `simba wide` | 0 | 5 | 1 | 0.0 | reject; already enabled and scalar LU remains scalar |
| Add `nalgebra-lapack` without call-site replacement | 0 | 5 | 2 | 0.0 | reject; dependency alone does not affect `DMatrix::lu()` |
| Replace LU with LAPACK backend | 4 | 1 | 5 | 0.8 | defer; below 2.0 for this pass and requires a separate behavior-proofed backend swap |

No dependency/config change meets the `Impact x Confidence / Effort >= 2.0` bar.
