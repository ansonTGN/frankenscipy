# frankenscipy-psn7x.11 Rejection - Lower-Band DSBTRD Port

## Target

- Bead: `frankenscipy-psn7x.11`
- Crate: `fsci-linalg`
- Route: `eig_banded(..., lower=true, eigvals_only=false)`
- Environment: local cargo/hyperfine, because `ts1` RCH worker was offline.

## Baseline

Public probe:

| shape | bandwidth | baseline time | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | 3.904898 ms | 7.56017470848746598e-12 | 1.64845914696343243e-12 | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | 15.697910 ms | 4.16093826061114669e-11 | 7.73070496506989002e-12 | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

Hyperfine command mean: `208.0 ms +/- 10.2 ms`.

The native dense symmetric-eigh stage profile still shows reduction as the large
wall at `1200x1200`: reduction `418.214098 ms`, tridiagonal `99.987990 ms`,
backtransform `135.728812 ms`, sort `8.080809 ms`.

## Candidate

Single source lever tested locally, then restored:

- Added a guarded safe-Rust lower-band `DSBTRD`-style helper.
- Used compact band storage with 1-based LAPACK indexing, generated plane
  rotations, `DLARGV`/`DLARTV`/`DLAR2V`/`DROT`-style helpers, dense Q
  accumulation for eigenvectors, and residual/orthogonality fallback.
- Wired it before the current dense/native `eig_banded` fallback.

Primary reference points were Netlib LAPACK `DSBTRD` lower-band reduction and
`DLAR2V` symmetric 2x2 rotation update. The implementation did not link to
LAPACK/BLAS and used safe Rust only.

## Candidate Result

Command:

```text
cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture --test-threads=1
```

| shape | bandwidth | baseline | candidate | speedup | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | 3.904898 ms | 4.790956 ms | 0.815059x | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | 15.697910 ms | 19.490373 ms | 0.805417x | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

The unchanged public digests and residuals show the candidate's guard fell back
to the current dense/native route, but after paying extra band-reduction
overhead. It therefore failed the performance gate.

## Post-Restore Proof

The source lever was restored. Post-restore public probe:

| shape | bandwidth | post-restore time | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | 3.661038 ms | 7.56017470848746598e-12 | 1.64845914696343243e-12 | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | 16.792421 ms | 4.16093826061114669e-11 | 7.73070496506989002e-12 | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

- Ordering/tie-breaking: unchanged after restore; existing public sort remains.
- Floating point: golden values/vector digests and residuals match baseline.
- RNG: unchanged; no public RNG is used by this route.
- Safety: no source change retained; tested source was safe Rust only.

## Decision

Reject/no-ship. Score `0.0`: the lower-band `DSBTRD` port did not prove a
band-native result and regressed the public probe via fallback overhead.

Next primitive: stop wiring a full public route before the tridiagonalizer has
an isolated proof. Build a focused scalar-oracle `DSBTRD` lower-branch harness
over small deterministic lower-band fixtures, prove the produced `D/E/Q`
reconstruct `A` before any public `eig_banded` integration, then optimize the
bulge queue only after that kernel is proof-clean.
