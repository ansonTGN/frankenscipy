# frankenscipy-psn7x.10 Local Rejection - Fixed-Envelope Band Givens

## Target

- Bead: `frankenscipy-psn7x.10`
- Crate: `fsci-linalg`
- Route: guarded lower-band eigenvector primitive for `eig_banded(..., eigvals_only=false)`
- Environment: local cargo/hyperfine, because `ts1` RCH worker was offline per 2026-06-16 override.

## Baseline

Command:

```text
cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture --test-threads=1
```

Local probe baseline:

| shape | bandwidth | baseline time | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | 4.114137 ms | 7.56017470848746598e-12 | 1.64845914696343243e-12 | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | 16.233105 ms | 4.16093826061114669e-11 | 7.73070496506989002e-12 | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

Hyperfine command wall-time baseline:

```text
hyperfine --warmup 1 --runs 5 'cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture --test-threads=1'
```

Mean command time: `202.8 ms +/- 11.0 ms`.

The native symmetric-eigh stage split remained reduction-dominant on the same local checkout:

| shape | reduction | tridiagonal | backtransform | sort | values digest |
| --- | ---: | ---: | ---: | ---: | --- |
| 400x400 | 16.735668 ms | 10.242352 ms | 8.695700 ms | 0.901729 ms | `0x0dbbde75b75c8612` |
| 800x800 | 109.000647 ms | 51.053917 ms | 43.076458 ms | 3.572421 ms | `0x4461962827bdb038` |
| 1200x1200 | 350.238879 ms | 98.214365 ms | 125.402010 ms | 8.226481 ms | `0x2fc45e1f18ceb0ab` |

## Candidate

Single source lever tested locally, then restored:

- Add a guarded fixed-envelope contiguous band storage helper for `eig_banded` eigenvectors.
- Apply adjacent symmetric Givens similarities over the envelope.
- Accumulate rotations and replay them into the tridiagonal eigenvectors.
- Return only if the band residual and eigenvector orthogonality guards pass; otherwise fall back to the existing dense/native route.

This avoided sparse maps and full-reorthogonalized Lanczos, but the fixed-envelope adjacent-rotation schedule did not meet the performance gate. The focused public probe was interrupted after more than 60 seconds without producing the first proof block; the accepted baseline for the same probe is sub-20 ms internally and about 203 ms end-to-end via hyperfine.

## Isomorphism / Golden Proof

The rejected source was restored. Post-restore proof:

| shape | bandwidth | post-restore time | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | 3.353603 ms | 7.56017470848746598e-12 | 1.64845914696343243e-12 | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | 16.292007 ms | 4.16093826061114669e-11 | 7.73070496506989002e-12 | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

- Ordering/tie-breaking: unchanged after restore; existing `eig_banded` sorting path is active.
- Floating-point contract: unchanged after restore; values/vector digests, residuals, and max-diff match the baseline.
- RNG: unchanged; no public RNG is used by this probe.
- Safety: no source change retained; candidate used only safe Rust while tested.

## Decision

Reject/no-ship. Score `0.0`: the fixed-envelope adjacent-Givens formulation exceeded the time gate before a proof block and cannot meet `Impact x Confidence / Effort >= 2.0`.

Next primitive: do not retry fixed-envelope adjacent Givens, sparse maps, dense re-expansion, full-reorthogonalized Lanczos eigenvectors, raw/stale compact-WY panels, row-wise lower-storage traversal, or scalar/SIMD spelling. Attack a LAPACK-`dsbtrd`-class safe-Rust primitive: explicit active bulge queue over compact symmetric-band storage, chase-order local rotations/reflectors, and blocked/row-tiled dense Q accumulation with residual-gated fallback.
