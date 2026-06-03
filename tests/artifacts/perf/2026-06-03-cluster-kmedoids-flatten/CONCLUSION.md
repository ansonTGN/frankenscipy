# kmedoids — contiguous-flatten + prefilter/abandonment lever

**Bead:** frankenscipy-8iq4k
**Crate:** fsci-cluster
**Date:** 2026-06-03
**Result:** POSITIVE — up to **3.09× / 2.03×** in the assignment-dominant regime, bit-identical.

## Hotspot

`kmedoids()` re-walked `Vec<Vec<f64>>` rows on every iteration in two inner loops:

1. **Nearest-medoid assignment** — `sq_dist(&data[i], &data[med])` over `k` medoids,
   `n` times per iteration. Each `data[i]` / `data[med]` is a scattered heap pointer
   (load + cache miss), and the scan is a full argmin with no early exit.
2. **M×M intra-cluster distance matrix** — `sq_dist(&data[members[i]], &data[members[j]])`
   indexes scattered member rows back into `data`.

This is the same scattered-row pointer-chase the vq/kmeans/DBSCAN paths already
retired (`0380e77e`, `64a1f94c`).

## Lever (the canonical contiguous-NN recipe)

- Pack the loop-invariant observation set into one contiguous `n × d` buffer once
  (`flatten_points`); every inner distance now streams a contiguous slice.
- Assignment: pack the current medoid rows into a reused `k × d` buffer per
  iteration and route the argmin through `nearest_centroid` — a `PREFILTER_DIMS`
  probe seeds a tight incumbent bound so partial-distance abandonment
  (`sq_dist_within`, strict-`>`) rejects most medoids after a few dimensions.
- dmat: gather each cluster's member rows into a contiguous `m × d` buffer so the
  `M(M−1)/2` evaluations stream instead of chasing `members[i]` scattered indices.

## Isomorphism proof

`nearest_centroid` is a proven bit-identical replacement for the strict-`<` argmin
(same lowest-index tie-break; the winning medoid is never abandoned, so its stored
minimum is a fully-summed distance). Flattening relocates data but `sq_dist` sums
the same terms in the same index order.

Golden output (labels + inertia bits + n_iter over a fixed n/d/k sweep), library
**before** vs **after** the change:

```
golden_before.txt  sha256 = d9bc8e10b75204da60bd45ee8bb3d3def7a8705876a2056e3c029c6c42ea625f
golden_after.txt   sha256 = d9bc8e10b75204da60bd45ee8bb3d3def7a8705876a2056e3c029c6c42ea625f   (IDENTICAL)
```

`cargo test -p fsci-cluster --lib`: 99 passed / 0 failed.

## Witness (in-process A/B, same binary — worker-invariant)

`base` = scattered-row pre-optimization mirror (`kmedoids-base` mode);
`opt` = shipped library (`kmedoids` mode). See `ab_bench.txt`.

| n | d | k | base ms | opt ms | speedup |
|----|----|-----|--------|--------|---------|
| 1500 | 32 | 8 | 13.31 | 11.78 | 1.13× |
| 2500 | 64 | 16 | 39.10 | 29.51 | 1.32× |
| 4000 | 32 | 24 | 65.86 | 48.49 | 1.36× |
| 6000 | 16 | 128 | 61.12 | 34.43 | 1.78× |
| 8000 | 16 | 256 | 185.12 | 96.78 | 1.91× |
| 6000 | 32 | 256 | 260.66 | 84.27 | **3.09×** |
| 10000 | 16 | 384 | 279.93 | 138.45 | 2.02× |
| 8000 | 16 | 512 | 197.67 | 97.17 | 2.03× |

The win grows with `k` (assignment-dominant: `n·k` re-walked medoid rows where
abandonment bites) and with `d≥16` (more dimensions for the prefilter to seed a
tight bound). At small `k` the `Σ m²·d` sqrt-bound dmat build dominates and the
locality gain is the modest ~1.1–1.4× the flatten-alone lever delivers; the
prefilter/abandonment is what carries the high-`k` regime past Score ≥ 2.0.
