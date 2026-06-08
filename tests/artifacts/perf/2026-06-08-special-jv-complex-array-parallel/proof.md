# perf: parallelize complex-z arbitrary-order Bessel arms (jv/yv/iv/kv complex)

Bead: frankenscipy-vn2j7

## Lever
`bessel_dispatch` (crates/fsci-special/src/bessel.rs) handled the real-order × complex-z
array arms (`order_scalar × z_complexvec`, `order_vec × z_complexscalar`, `order_vec ×
z_complexvec` zip) of jv/yv/jve/yve/iv/kv/ive/kve via a serial map over
`bessel_complex_scalar` (complex Bessel series — EM/dielectric/waveguide use, expensive).
Made bessel.rs's `par_map_indices` **generic over T** (f64 or Complex64) and routed all three
complex-z arms through it. This completes bessel_dispatch (real arms parallelized earlier in
8634e76a; complex-z arms now too). Complex-ORDER remains fail-closed `not_yet_implemented`,
unchanged.

## Isomorphism / byte-identity argument
- Each output index `i` is `bessel_complex_scalar(...broadcast at i...)` written to slot `i`;
  chunks cover the index range contiguously and concatenate in index order ⇒ identical output
  `Vec<Complex64>`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.iter().map(...).collect()`.
- The `order_vec × z_vec` length check + scalar/complex-order arms untouched. Gate: serial for
  `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_jv_complex_array`
(order=2.5, z = re∈(0.5,20.5) + i·im∈(-4,4))
```
n=100   jv_c=d1e483bbd01ceb9c kv_c=8eedfe25adfbeddb
n=5000  jv_c=fc54206a2ab97071 kv_c=ca59a2734560b275
n=50000 jv_c=b1246e645494c512 kv_c=61e6d3e996c7a741
timing acc: jv_c 200k=7328ca9ddc7f0227  kv_c 100k=47cd41383c3f35e5
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = e54db9090f0210db40dde40137c18ac023b9846a190a13f520ac9927caf7b573

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| function / array | serial (3x)             | new (3x)            | speedup |
|------------------|-------------------------|---------------------|---------|
| jv (complex z), 200k | 129.4/126.1/122.5 ms | 9.79/9.55/8.92 ms   | ~13.3x  |
| kv (complex z), 100k | 87.67/88.73/84.41 ms | 6.40/6.45/6.66 ms   | ~13.5x  |

## Validation
103 bessel unit tests pass; clippy: no warning in bessel.rs.
(Note: a transient `cargo test -p fsci-special` failure referencing a `apply_bidiag_fused_step`
linalg symbol was a concurrent linalg edit on the shared rch worker — unrelated to this change;
re-run passed clean. This change touches only bessel.rs.)
