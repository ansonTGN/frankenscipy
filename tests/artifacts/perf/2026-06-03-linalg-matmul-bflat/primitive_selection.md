# B-Flat Matmul Primitive Selection

Bead: `frankenscipy-8l8r1.10`

## Profile-Backed Target

Fresh current-HEAD RCH Criterion baseline on `vmi1156319`:

- `matmul/256x256`: `[10.323 ms, 10.866 ms, 11.181 ms]`
- `matmul/512x512`: `[84.486 ms, 90.769 ms, 93.893 ms]`
- `matmul/768x768`: `[847.18 ms, 884.41 ms, 929.71 ms]`
- `matmul/1024x1024`: `[1.6526 s, 1.6964 s, 1.7432 s]`

The older committed profile at
`tests/artifacts/perf/2026-06-03-linalg-matmul-a-panel/criterion_matmul_rch_raw.txt`
also identifies `fsci_linalg::matmul` as the dominant linalg kernel, with a
1024x1024 median of `394.26 ms`.

## Alien Primitive

Matched guidance:

- Alien graveyard 6.5: affine loop nests can use tiling/interchange/locality
  optimization when dependence order is provably preserved.
- Alien graveyard 9.6: dense BLAS-class kernels should reduce data movement and
  use tiled matrix multiply as the inner primitive.

Selected one lever: flatten the read-only `B` matrix once into contiguous
row-major storage, then keep the existing 4x4 register tile shape and scalar
ragged path. This avoids `Vec` row-header chasing in the hot `k` loop without
changing each output cell's monotonic `k` accumulation sequence.

Rejected non-levers:

- Do not retry NC column-panel traversal: closed negative in
  `frankenscipy-8l8r1.1`.
- Do not retry B-panel plus 4x8 packing: closed negative in
  `frankenscipy-jhtc6`.
- Do not link external BLAS/LAPACK/MKL/XLA or add unsafe code.

## Proof Contract

Preserve:

- API and error behavior.
- Validation order and direct indexing surfaces.
- Output shape and row/column order.
- Per-cell floating-point sequence: for each `c[i][j]`, accumulate
  `k = 0..ka` in monotonic order with separate multiply then add.
- Tie-breaking absence, RNG absence, and global-state absence.

Golden before/after normalized sha256:

`0def10fbd95d1bf20c417af563de181eeab314cae762cc82fd67c1ebac6f406c`

## Score Target

Target score: `4.5 = impact 3 * confidence 3 / effort 2`.

Keep only if the focused RCH re-benchmark shows a real win and no material
size-row regression. Otherwise restore source and close as a rejected trial.
