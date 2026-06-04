# Uniform-axis O(1) interval fast path for 3D nearest regular-grid eval

Bead: frankenscipy-5os31
Bench: `regular_grid/nearest_eval_many/32x32x16_4096` (fsci-interpolate)
Host: rch ts2

## Lever
Replace per-axis binary-search `find_interval` (O(log n)) in the 3D nearest
hot path with direct-address interval inversion for evenly-spaced axes
(`i = floor((x-x0)/dx)` + short correction against stored coordinates), and
fuse the previously-separate NaN / bounds / nearest passes into one per-query
loop with hoisted axis refs. Uniform-axis metadata `(x0, 1/dx)` is detected
once at construction; irregular axes keep the binary-search path.

## Isomorphism proof
1. `find_interval_uniform_matches_binary_search` asserts the O(1) lookup is
   bit-identical to `find_interval` across interior points, exact coordinates,
   midpoints, and out-of-range probes on four uniform axes (incl. the two bench
   axes). Since the only change to the eval path is swapping these functions on
   uniform axes, eval output is provably unchanged.
2. Golden SHA over the nearest-eval payload (bench grid + edge/NaN/OOB queries),
   `dump_nearest_payload_for_golden_sha`:
   - optimized build: `68d5b3b474fd1e60cc453ac780226983500cbce6af48258fc1bd2902da790f8e`
   - slow-path forced (detect_uniform_axis -> None): identical sha.

## Same-worker A/B (back-to-back, same session)
- SLOW (binary search, original algo): 277.23 us median
- FAST (uniform O(1) + fused):          131.40 us median
- **Score = 277.23 / 131.40 = 2.11x**

For reference, the linear path on the same grid is ~272 us, so 3D nearest now
runs ~2.07x faster than linear (it was previously slower).

## Validation
- `cargo test -p fsci-interpolate --lib`: 122 passed, 0 failed.
- `cargo clippy -p fsci-interpolate --lib --tests`: clean (the 2 sparse warnings
  are an unrelated in-progress crate).
