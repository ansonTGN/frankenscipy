# perf: parallelize Bessel real-array evaluation (bessel::map_real_input)

Bead: frankenscipy-nq400

## Lever
fsci-special was entirely serial (0 `thread::scope` across the crate). The Bessel
real-array dispatcher `map_real_input` (crates/fsci-special/src/bessel.rs) evaluated the
`RealVec` arm with a serial `values.iter().map(kernel).collect()`. Each element is an
independent — and for Bessel kernels (continued fractions / series) expensive — scalar
evaluation written to its own output slot. Chunk the array across cores; each chunk maps
the kernel into its own buffer; concatenate the buffers in element order.

This dispatcher backs **12 functions**: j0, j1, y0, y1, i0, i1, k0, k1, i0e, i1e, k0e, k1e.

## Isomorphism / byte-identity argument
- Each element is `kernel(values[i])`, written to position `i`; chunks are concatenated in
  element order ⇒ identical output `Vec`. No reduction.
- Error path: chunk results are folded in order with `?`, so the first failing chunk's
  error (in element order) is returned — the same `SpecialError` the serial map would yield.
  (On error inputs the parallel version may evaluate elements past the first failure in
  other chunks; that only affects Hardened-mode diagnostic trace emission, never the
  returned value/error.)
- Gate: serial for `< 512` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_bessel_array`
```
n=100   j0 xor=ee31de14d4581733  k0 xor=76a5e5b3aed7b214
n=5000  j0 xor=04d99a116d8831d5  k0 xor=bee3afaba078fffe
n=50000 j0 xor=075b241a624c0288  k0 xor=985bd4ec07ecfbde
timing acc (k0): 300k=70789cfb60243cbe  600k=b6b485edbbc61854
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = 2701d36e5cec6af5c2659c10dbd9e020f48c3685abd877ef99fc3afa27d45d1b

## Timing (rch remote, release-perf) — k0 (modified Bessel 2nd kind, ~45µs/elt serial)
| array     | serial      | new (3x)              | speedup |
|-----------|-------------|-----------------------|---------|
| 300k      | 13.530 s    | 447.8/452.6/462.7 ms  | ~30x    |
| 600k      | 27.238 s    | 880.7/890.2/895.2 ms  | ~30x    |

## Validation
103 bessel unit tests pass; clippy: no new warning from map_real_input (the 2 pre-existing
fsci-special warnings are in beta.rs:1181/1183).
