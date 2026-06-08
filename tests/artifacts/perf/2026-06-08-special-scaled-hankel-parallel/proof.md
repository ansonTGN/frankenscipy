# perf: parallelize scaled Hankel array arms (hankel1e/hankel2e)

Bead: frankenscipy-yo06z

## Lever
`scaled_hankel_dispatch` (crates/fsci-special/src/bessel.rs) — behind `hankel1e`/`hankel2e`
(exp-scaled Hankel: hankel1e = H^(1)_v(z)·exp(-iz), hankel2e = H^(2)_v(z)·exp(+iz); used to
keep magnitudes finite for large-|z| / large-imag-z wave-propagation work) — mapped its
per-element kernel (`hankel_real_scalar`/`hankel_complex_scalar` followed by `scale_hankel`)
serially over **6 array arms** (real-order × {real,complex} z; vec×scalar, scalar×vec, vec×vec
zip). Routed all six through the generic `par_map_indices` helper in bessel.rs (output is
`ComplexVec`). Complex-order remains fail-closed `not_yet_implemented`, unchanged.

## Isomorphism / byte-identity argument
- Each output index `i` is `hankel_*_scalar(...broadcast at i...).map(|v| scale_hankel(v, z_i,
  scale))` written to slot `i`; the scaling is a pure per-element function of the value and the
  per-index z. Chunks cover the index range contiguously and concatenate in index order ⇒
  identical output `Vec<Complex64>`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.iter().map(...).collect()`.
- The vec×vec length checks + scalar / complex-order / empty fail-closed arms are untouched.
  Gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_hankele_array`
(order=2.5, real z∈(0.5,40.5))
```
n=100 hankel1e(len=100,xor=e34e56cac6c7b5d1) hankel2e(len=100,xor=1cb1aacac6c7b5ee)
n=5000 hankel1e(len=5000,xor=c492a49a80450bb3) hankel2e(len=5000,xor=c492a49a80453473)
n=50000 hankel1e(len=50000,xor=a17a9567942d8c73) hankel2e(len=50000,xor=5e856a986bed8c4c)
timing acc: hankel1e 500k=1890abd05e314c30  hankel2e 500k=1890abefa1ceb3f0
```
Identical in the stashed serial build and the parallel build (golden xor + timing acc both match).
sha256(golden_payload.txt) = bd2b4372296d80a9a85d0f4dc9226119edf7294837d582e9a61606deca072ef2

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| function / array | serial (3x)             | new (3x)               | speedup |
|------------------|-------------------------|------------------------|---------|
| hankel1e, 500k   | 261.6/267.8/276.7 ms    | 21.80/20.99/24.13 ms   | ~12.1x  |
| hankel2e, 500k   | 275.9/265.3/274.0 ms    | 18.31/24.55 ms         | ~12.6x  |

## Validation
8 hankel unit tests pass; clippy: no warning in bessel.rs.
