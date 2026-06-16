# frankenscipy-psn7x.11 Baseline - DSBTRD-Style Bulge Queue

## Target

- Bead: `frankenscipy-psn7x.11`
- Crate: `fsci-linalg`
- Public route: `eig_banded(..., lower=true, eigvals_only=false)`
- Environment: local cargo/hyperfine, because `ts1` RCH worker was offline.

## Primary Algorithm Reference

The target primitive follows the LAPACK `DSBTRD` contract: reduce a real
symmetric band matrix to symmetric tridiagonal form by orthogonal similarity,
with lower-band input stored as `AB(1+i-j,j) = A(i,j)`. Netlib's lower branch
uses generated plane rotations, applies them from both sides, stores off-band
bulges in work storage, and copies the tridiagonal diagonal/subdiagonal into
`D` and `E`.

The next source lever must be a safe-Rust implementation of that class of
primitive, not a dense re-expansion, fixed-envelope adjacent-Givens sweep,
sparse map, or Lanczos eigenvector route.

## Current-Head Public Baseline

Command:

```text
cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture --test-threads=1
```

| shape | bandwidth | local time | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | 3.904898 ms | 7.56017470848746598e-12 | 1.64845914696343243e-12 | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | 15.697910 ms | 4.16093826061114669e-11 | 7.73070496506989002e-12 | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

Hyperfine command baseline:

```text
hyperfine --warmup 1 --runs 5 'cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture --test-threads=1'
```

Mean command time: `208.0 ms +/- 10.2 ms`.

## Native Symmetric-Eigh Stage Profile

Command:

```text
cargo test -j 1 -p fsci-linalg --lib symmetric_eigh_native_stage_breakdown_probe --release --locked -- --ignored --nocapture --test-threads=1
```

| shape | reduction | tridiagonal | backtransform | sort | values digest |
| --- | ---: | ---: | ---: | ---: | --- |
| 400x400 | 18.705779 ms | 11.888073 ms | 12.542502 ms | 0.863546 ms | `0x0dbbde75b75c8612` |
| 800x800 | 167.538426 ms | 46.321918 ms | 48.705884 ms | 3.488629 ms | `0x4461962827bdb038` |
| 1200x1200 | 418.214098 ms | 99.987990 ms | 135.728812 ms | 8.080809 ms | `0x2fc45e1f18ceb0ab` |

## Acceptance Gate

- Ordering/tie-breaking: preserve existing `total_cmp` sorted eigenpair order.
- Floating point: residual/max-diff guards at least as strict as the current
  public probe; golden values/vector digests must be recorded before and after.
- RNG: no public RNG in this route.
- Safety: safe Rust only; no C BLAS/LAPACK/MKL/XLA linkage.
- Score gate: keep only if same-machine public probe improves enough for
  `Impact x Confidence / Effort >= 2.0`; otherwise restore source and route
  deeper.
