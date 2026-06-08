# perf: parallelize error-function real/complex-array evaluation (error::map_unary_input)

Bead: frankenscipy-lxo1i

## Lever
The error-function dispatcher `map_unary_input` (crates/fsci-special/src/error.rs) — behind
**erf, erfc, erfinv, erfcinv** (and their complex paths, 4 callers) — mapped the per-element
kernel serially over the `RealVec` and `ComplexVec` arms. The kernels are non-trivial:
erf uses a Maclaurin series (and the erfc Lentz continued fraction for x≥1); erfinv/erfcinv
use Newton/Halley refinement (~250 ns/element for erf serial). Added a generic index-based
helper `par_map_indices<T>(n, f)` (T = f64 or Complex64) that evaluates `f(0..n)` in parallel
index chunks, and routed both vec arms through it.

erf/erfc are the Gaussian-CDF backbone — among the most ubiquitous special functions in
stats / ML.

## Isomorphism / byte-identity argument
- Each output index `i` is `kernel(values[i])` written to slot `i`; chunks cover `0..n`
  contiguously and concatenate in index order ⇒ identical output `Vec`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError` returned, exactly as the serial `values.iter().map(kernel).collect()`.
  (Parallel may evaluate elements past the first error in other chunks; only affects
  Hardened-mode diagnostic trace emission, never the returned value/error.)
- Scalar / Empty arms untouched. Gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_erf_array`
```
n=100   erf xor=e2ff4a2005e53032  erfinv xor=f673bdf697780454
n=5000  erf xor=5152f5b58582f943  erfinv xor=624e9ec014d2c8cb
n=50000 erf xor=75cdbff2d1474a15  erfinv xor=d38c1314a3179049
timing acc: erf 2M=35bfab90dc6265da  erfinv 1M=5c659b5de55c8b46
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = 59c74fbe61904b19a38954cd32b60e372e3ddb764f9a62e36a46de2b4814935c

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| function / array      | serial (3x)            | new (3x)             | speedup |
|-----------------------|------------------------|----------------------|---------|
| erf,    2M (x∈[-4,4]) | 497.9/504.2/512.5 ms   | 25.90/29.63/26.87 ms | ~18.6x  |
| erfinv, 1M (y∈[-.99,.99]) | 323.0/340.5/333.4 ms| 16.45/16.75/15.26 ms | ~20.2x  |

## Validation
37 erf unit tests pass; clippy: no warning in error.rs (helper + arms clean).
