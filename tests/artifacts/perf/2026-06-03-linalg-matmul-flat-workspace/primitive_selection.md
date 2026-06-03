# frankenscipy-8l8r1.19 primitive selection

## Profile target

Fresh RCH Criterion baseline on `vmi1149989`:

- `matmul/256x256`: median `7.0222 ms`
- `matmul/512x512`: median `39.686 ms`
- `matmul/768x768`: median `144.63 ms`
- `matmul/1024x1024`: median `534.93 ms`

The broader post-`.15` reprofile ranked `matmul/1024x1024` as the current
top linalg hotspot at median `436.43 ms`, ahead of `matmul/768x768`,
`lstsq/512x256`, and `pinv/512x256`.

## Alien primitive

Safe-Rust BLAS-class contiguous workspace for large GEMM:

- copy rectangular `A` and `B` into row-major contiguous slabs;
- compute the existing 4x8 register micro-kernel into a row-major contiguous
  `C` slab;
- materialize the public `Vec<Vec<f64>>` output in row order at the end.

This is a memory-layout replacement rather than another accumulator or tile-size
micro-lever. It targets the graveyard/no-gaps numeric-kernel lane: cache-local,
BLAS-class dense kernels in pure safe Rust, without C BLAS/LAPACK/MKL/XLA.

## Scope and proof obligations

The optimized path is gated to rectangular matrices whose `m`, `k`, and `n` are
all at least `1024`; all smaller or ragged cases use the existing kernel. This
keeps previously rejected small-row regressions out of scope while attacking the
profile-driving large GEMM row.

Behavior invariants:

- public API and shape-error behavior are unchanged;
- output remains row-major `Vec<Vec<f64>>`;
- each output cell accumulates `k = 0..ka` in the same monotonic order;
- each update remains a separate `acc += a * b` operation;
- ragged and sub-threshold inputs use the existing code path;
- no RNG, tie-breaking, global state, or ordering surface is introduced.

Keep gate:

- remote RCH matmul golden/isomorphism proof must stay stable;
- sorted normalized golden sha256 must match before/after;
- focused RCH re-benchmark must show a real `matmul/1024x1024` win with
  `Score >= 2.0`; otherwise restore source and close rejected.
