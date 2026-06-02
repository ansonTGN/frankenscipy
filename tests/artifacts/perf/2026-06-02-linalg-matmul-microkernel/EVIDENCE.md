# matmul register micro-kernel lever — perf evidence

Bead: frankenscipy-8l8r1 ([perf][no-gaps] safe-Rust BLAS/LAPACK-class kernels)
Crate: fsci-linalg
Function: `matmul` (dense f64 GEMM, `&[Vec<f64>]`)
Lever: register-blocked MR×NR=4×4 micro-kernel (one lever)
Agent: SapphireDove (claude-opus-4-8)
Date: 2026-06-02
Hardware: rch remote vmi1149989 (shared; per-run absolute times vary, intra-run
          flat-vs-kernel comparison is apples-to-apples)

## The lever

The previous body was a flat ikj loop (`c[i][..] += a[i][k]*b[k][..]`). Each FMA
touches both `b[k][j]` and `c[i][j]` in memory, so it is memory-bound on the C
read/modify/write and re-streams all of B once per output row.

The micro-kernel computes a 4×4 tile of C in register-resident accumulators:
across the k-loop each loaded `a[i0+di][k]` feeds NR=4 columns and each loaded
`b[k][j0+dj]` feeds MR=4 rows, so 16 FMAs ride on only 8 scalar loads, and the
NR-wide accumulator rows map onto SIMD lanes. Ragged edges (mr<4 or nr<4) fall
back to a monotonic-k scalar reduction.

## Isomorphism / parity proof

BIT-IDENTICAL to naive ijk / flat ikj: every `acc[di][dj]` accumulates k in
0..ka monotonic order — the identical sequence of separate mul+add ops as the
reference (Rust does not contract `a*b+c` into a fused FMA without fast-math),
so each `c[i][j]` keeps the same IEEE-754 bit pattern.

Proofs (all green):
- `matmul_microkernel_is_bit_identical_to_flat_ikj` — to_bits() equality across
  dims straddling the 4×4 tile boundary (17,23,19 / 25,8,31 / 33,17,8 / 8,8,8 /
  9,4,5), exercising both full-tile and ragged-edge paths.
- `matmul_microkernel_golden_digest` — frozen FNV-1a 64-bit digest over the raw
  f64 bit patterns of a fixed 80×80 product = 0xf9aa16d2dc37468f.
- Full `cargo test -p fsci-linalg --release`: 341 lib + integration tests pass,
  0 failures (matmul's downstream callers unaffected).
- `cargo clippy -p fsci-linalg --release`: 0 warnings.

## Perf witness (flat ikj baseline → micro-kernel), 3 runs

    matmul 768x768:   flat_ikj=0.3075s  microkernel=0.1461s  speedup=2.10x
    matmul 1024x1024: flat_ikj=0.7174s  microkernel=0.6186s  speedup=1.16x
    matmul 768x768:   flat_ikj=0.3449s  microkernel=0.1222s  speedup=2.82x
    matmul 1024x1024: flat_ikj=0.7558s  microkernel=0.7093s  speedup=1.07x
    matmul 768x768:   flat_ikj=0.2800s  microkernel=0.1047s  speedup=2.68x
    matmul 1024x1024: flat_ikj=0.6157s  microkernel=0.5689s  speedup=1.08x

Score at 768³: 2.10–2.82x (>= 2.0 ✓), consistent across 3 runs. Never regresses
(1024³: +7–16%). Reproduce:
    cargo test -p fsci-linalg --release matmul_microkernel_perf -- --ignored --nocapture

## Negative result recorded (saves re-attempts)

Plain row-blocking (process IB output rows per B-row load, keep an IB×N C panel
resident) was tried first and REGRESSED: IB=8 → 0.62–0.80x, IB=4 → 0.67–0.85x.
On this CPU B already fits L3, so blocking only demotes the hot C accesses from
L1 to L2 to save B traffic that was already cheap. Register-tiling wins instead
because it cuts the per-FMA memory ops (C lives in registers, not L1), not just
B streaming. Do not re-attempt naive row/cache-blocking for B-fits-L3 sizes.
