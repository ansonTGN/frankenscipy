# perf: parallelize Hankel function array arms (hankel1/hankel2)

Bead: frankenscipy-0j2hx

## Lever
`hankel_dispatch` (crates/fsci-special/src/bessel.rs) — behind `hankel1`/`hankel2`
(H^(1)_v(z) = J_v + iY_v, H^(2)_v = J_v − iY_v; EM / wave propagation / scattering) — mapped
its per-element kernel (`hankel_real_scalar` for real z, `hankel_complex_scalar` for complex z)
serially over **6 array arms** (real-order × {real,complex} z; vec×scalar, scalar×vec, vec×vec
zip). Routed all six through the generic `par_map_indices` helper in bessel.rs (output is
`ComplexVec`). Complex-order remains fail-closed `not_yet_implemented`, unchanged.

## Isomorphism / byte-identity argument
- Each output index `i` is `hankel_*_scalar(...broadcast at i...)` written to slot `i`; chunks
  cover the index range contiguously and concatenate in index order ⇒ identical output
  `Vec<Complex64>`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.iter().map(...).collect()`.
- The vec×vec length checks + scalar / complex-order / empty fail-closed arms are untouched.
  Gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_hankel_array`
(order=2.5, real z∈(0.5,40.5))
```
n=100   hankel1=f8ec94bfb1c6fe4f hankel2=071368bfb1c6fe70
n=5000  hankel1=c8dcc766e7863a4d hankel2=c8dcc766e786058d
n=50000 hankel1=3eb698b05f5d21b1 hankel2=c149674fa09d218e
timing acc: hankel1 500k=c387d102a145d8c5  hankel2 500k=c387d13d5eba2705
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = 0204df7b829cfb62c9d3269d516c0a6b167fb77770a591e1f12bfe13bca0a8df

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| function / array | serial (3x)             | new (3x)             | speedup |
|------------------|-------------------------|----------------------|---------|
| hankel1, 500k    | 274.1/274.2/289.8 ms    | 18.16/17.76/17.68 ms | ~15.4x  |
| hankel2, 500k    | 287.7/278.6/284.7 ms    | 16.43/17.78/(–) ms   | ~16x    |

## Validation
8 hankel unit tests pass; clippy: no warning in bessel.rs.
