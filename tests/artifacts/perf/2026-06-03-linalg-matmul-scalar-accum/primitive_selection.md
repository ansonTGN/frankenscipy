# fsci-linalg matmul scalar-accumulator primitive

Bead: `frankenscipy-8l8r1.13`

## Profile target

The committed linalg reprofile after prior matmul rejected levers keeps dense
`matmul/1024x1024` as the completed top linalg hotspot at `650.36 ms`. Fresh
RCH baseline for this pass measured `matmul/1024x1024` at `608.85 ms` median on
`vmi1227854`.

## Selected primitive

Alien-graveyard dense-kernel guidance points to BLAS-like tiled kernels whose
hot path keeps the micro-kernel state in registers and uses cache-conscious,
SIMD-friendly scalar lanes. The current safe-Rust implementation already has a
4x4 register tile but stores the tile state as `[[f64; 4]; 4]`.

One lever: replace only that aggregate with sixteen named scalar accumulators
`c00..c33`.

## Required invariants

- Loop nesting and monotonic `k=0..ka` traversal stay unchanged.
- Each output cell keeps the same sequence of separate `a * b` then `+=`
  operations.
- A/B load order, output store order, validation/error behavior, and the ragged
  scalar path stay unchanged.
- RNG, tie-breaking, and global state are not part of the surface.

## Score target

Target Score: `3.0 = impact 2 * confidence 3 / effort 2`.

Reject if golden proof changes, if any large row materially regresses, or if
focused/paired RCH timing does not show a real win. Keep only if Score is at
least `2.0`.
