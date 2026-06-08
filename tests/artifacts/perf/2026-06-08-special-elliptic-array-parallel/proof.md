# perf: parallelize elliptic real/complex-array evaluation (elliptic::map_real_or_complex)

Bead: frankenscipy-8hv6i

## Lever
The elliptic unary dispatcher `map_real_or_complex` (crates/fsci-special/src/elliptic.rs)
— behind ellipk, ellipkm1, ellipe and their complex paths (6 callers) — mapped the AGM /
Carlson per-element kernel serially over the `RealVec` and `ComplexVec` arms. Added one
generic index-based helper `par_map_indices<T>(n, f)` (T = f64 or Complex64) that evaluates
`f(0..n)` in parallel index chunks, and routed both vec arms through it.

## Isomorphism / byte-identity argument
- Each output index `i` is `kernel(values[i])` written to slot `i`; chunks cover `0..n`
  contiguously and concatenate in index order ⇒ identical output `Vec`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError` returned, exactly as the serial `values.iter().map(kernel).collect()`.
  (On error inputs the parallel version may evaluate elements past the first error in other
  chunks; that only affects Hardened-mode diagnostic trace emission, never the value/error.)
- Scalar / Empty arms untouched. Gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_elliptic_array`
```
n=100   ellipk xor=1962ea31009d71d5  ellipe xor=f068ac6c6f0eba7c
n=5000  ellipk xor=ea5d3cf3c7b96b07  ellipe xor=7a6f3022413883fe
n=50000 ellipk xor=627cb3fb58e655dd  ellipe xor=e13d8dfecc8e7244
timing acc (ellipk): 1M=07ac5f3fc59675f4  2M=b49658fe68341cf0
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = 621e62ca8a5f7c41b26b7a89e4c88c14aedacfca54d812a22a4a2fa6a1dbd14b

## Timing (rch remote, release-perf, 3 back-to-back runs each) — ellipk (m∈[0,0.999))
| array | serial (3x)            | new (3x)            | speedup |
|-------|------------------------|---------------------|---------|
| 1M    | 24.86/25.77/25.19 ms   | 8.82/8.32/9.00 ms   | ~2.9x   |
| 2M    | 48.88/50.58/48.85 ms   | 12.63/12.60/12.04 ms| ~3.9x   |

(Lower than bessel/beta: the AGM kernel is cheaper per element — partly bandwidth-bound.)

## Validation
102 elliptic unit tests pass; clippy: no warning in elliptic.rs (helper + arms clean).
