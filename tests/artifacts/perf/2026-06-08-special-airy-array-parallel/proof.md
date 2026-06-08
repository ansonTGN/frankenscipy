# perf: parallelize Airy real/complex-array evaluation (airy::map_airy_component)

Bead: frankenscipy-xqyb1

## Lever
The Airy dispatcher `map_airy_component` (crates/fsci-special/src/airy.rs) — behind `ai`
and `bi` (and their complex paths) — mapped the expensive per-element `airy_scalar` /
`airy_complex_scalar` kernel (Bessel-form evaluation across the complex plane) serially
over the `RealVec` and `ComplexVec` arms. Added a generic index-based helper
`par_map_indices<T>(n, f)` (T = f64 or Complex64) that evaluates `f(0..n)` in parallel
index chunks, and routed both vec arms through it.

## Isomorphism / byte-identity argument
- Each output index `i` is `airy_scalar(values[i]).map(kernel)` written to slot `i`; chunks
  cover `0..n` contiguously and concatenate in index order ⇒ identical output `Vec`. No
  reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError` returned, exactly as the serial `values.iter().map(...).collect()`.
  (Parallel may evaluate elements past the first error in other chunks; only affects
  Hardened-mode diagnostic trace emission, never the returned value/error.)
- Scalar / Empty arms untouched. Gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_airy_array`
```
n=100   ai xor=d728b993034a2431  bi xor=cd319fb621130cc7
n=5000  ai xor=1816138c700fbaf7  bi xor=080c1215b2d97336
n=50000 ai xor=1019ea7afe698033  bi xor=12c75258d3508837
timing acc (ai): 300k=aeea8fddaf9053f5  600k=bf02cee99b8f798e
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = a91c70508a1ab4148258a097a56b53b478d831bbfa6779622552a45323067e74

## Timing (rch remote, release-perf, 3 back-to-back runs each) — ai (x∈[-15,15])
| array | serial (3x)            | new (3x)           | speedup |
|-------|------------------------|--------------------|---------|
| 300k  | 20.11/23.28/22.97 ms   | 4.94/6.00/5.71 ms  | ~4.0x   |
| 600k  | 46.24/46.87/43.68 ms   | 7.99/6.77/8.08 ms  | ~6.1x   |

## Validation
30 airy unit tests pass; clippy: no warning in airy.rs (helper + arms clean).
