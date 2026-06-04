# Alien primitive selection

Bead: `frankenscipy-8l8r1.30`

## Recommendation Contract

Change: collapse `silhouette_samples` per-cluster rescans into exact
segment/bucket accumulation, with a memory-capped symmetric pairwise bucket path
and an exact one-pass-per-anchor bucket fallback.

Hotspot evidence: RCH hyperfine on `n=2000 d=2 k=256` measured pre-edit current
`silhouette-samples` at `656.1 ms`; the cost driver is the repeated `k * n`
cluster-label scan around the same per-anchor distance set. Final exact bucket
implementation measured `14.6 ms` on the same shape.

Mapped graveyard sections:

- `alien_cs_graveyard.md` Opportunity Matrix Gate: only keep measured
  optimizations with score >= 2.0.
- `alien_cs_graveyard.md` A1 Numeric kernels: cache locality and SIMD-friendly
  data movement matter for numerical kernels, with correctness certificates.
- `alien_cs_graveyard.md` Data Parallel Haskell / segmented-array primitive:
  segment-aware reductions over flat data avoid nested rescans while preserving
  per-segment semantics.

Alien-artifact family: numerical/segmented reduction artifact with explicit
memory budget and exact fallback.

EV score: `8.0 = impact 5 * confidence 4 / effort 2`.

Priority tier: A. The shape is directly profile-backed and has a simple proof.

Adoption wedge: one function body, no API or validation changes.

Budgeted mode: no extra asymptotic memory beyond `O(k)` per anchor; exact same
error behavior on validation failure.

Expected-loss model:

- state: high-`k` repeated label scan dominates; action: segment buckets; loss low.
- state: distance kernel dominates; action: symmetric pairwise buckets; loss low.
- state: golden changes; action: reject and restore; loss avoided.

Fallback trigger: reject if `perf_cluster golden` SHA-256 changes, sorted
`silhouette_samples` test-output SHA changes, validation fails, or focused RCH
hyperfine does not beat the `656.1 ms` keep gate with Score >= 2.0.

## Exclusions

- No distance-kernel changes.
- No approximate clustering or pruning.
- No validation, API, error-message, label-density, output-order, RNG, or
  parallelism changes.
- No unsafe code and no external BLAS/LAPACK linkage.
