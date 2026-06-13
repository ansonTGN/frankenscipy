# SIMD sqeuclidean (cdist/pdist/knn inner kernel)

## Lever
`euclidean`/`sqeuclidean` computed `Σ(aᵢ-bᵢ)²` via `.map().sum()` — the float-sum fold is
NOT auto-vectorized (no fast-math reassociation), so it ran scalar. Rewrote sqeuclidean as
two 8-wide SIMD accumulators + a single-8 step + scalar tail; `euclidean = sqeuclidean.sqrt()`.
This is the dominant inner reduction of cdist / pdist / nearest-neighbour searches.

## Isomorphism
The perf_cdist probe self-checks the parallel `cdist_metric` byte-for-byte against a
sequential reference (both call `metric_distance`→`euclidean`): **bit_identical=true** on all
5 shapes after the change (the SIMD reduction is applied to both paths, so cdist stays
bit-identical; the result differs from the old scalar order only within fp tolerance).
Parity vs scipy: `cargo test -p fsci-spatial --lib` = **184 passed, 0 failed** (incl
euclidean/sqeuclidean reference-value tests).

## Benchmark (perf_cdist, seq = single-thread distance compute)
| shape                 | before (scalar) | after (SIMD) | speedup |
|-----------------------|-----------------|--------------|---------|
| cdist 2000×2000 dim=3 | 35.6 ms         | 24.7 ms      | 1.44x   |
| cdist 4000×1000 dim=8 | 35.4 ms         | 26.8 ms      | 1.32x   |
| cdist 3000×3000 dim=16| 101.6 ms        | 76.9 ms      | 1.32x   |
| pdist 4000 dim=16     | 106.4 ms        | 87.9 ms      | 1.21x   |

Real bit-identical ~1.3x on the euclidean kernel shared by cdist/pdist/knn. Modest because
the benched dims (3–16) are small; the win grows with dimensionality. Added `#![feature(
portable_simd)]` to fsci-spatial (first SIMD use in the crate).
