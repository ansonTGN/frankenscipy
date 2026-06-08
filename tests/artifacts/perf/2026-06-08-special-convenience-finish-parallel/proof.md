# perf: finish convenience-module array parallelization (wofz + map_real_or_complex + map_real_ternary)

Bead: frankenscipy-u9mv1

## Lever
Completed the convenience-module array parallelization (crates/fsci-special/src/convenience.rs):
made the shared `par_map_indices` helper **generic over the output type T** (f64 or Complex64),
then routed the three remaining serial array paths through it:
- `wofz` (Faddeeva w(z), inline dispatch) — RealVec + ComplexVec arms (~1.2µs/elt; Voigt-profile
  backbone in spectroscopy);
- `map_real_or_complex` (erfi, erfcx, dawsn, wrightomega) — RealVec + ComplexVec arms;
- `map_real_ternary` (betaincinv = inverse regularized incomplete beta = the Student-t / F / beta
  distribution quantiles, ~1.6µs/elt; + threshold/hard_tanh) — the broadcast output loop.

## Isomorphism / byte-identity argument
- Each output index `i` is `kernel(...values[i]...)` written to slot `i`; chunks cover the index
  range contiguously and concatenate in index order ⇒ identical output `Vec`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.map(kernel).collect()`.
- Broadcast-compatibility / length / unsupported-type fail-closed checks all untouched (they run
  before the parallel map). Scalar arms untouched. Gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_conv_complex_ternary_array`
```
n=100   wofz=f22bd2203ceb737c erfi=f3e3d738cf69607c betaincinv=5bc49169ccc8eccc
n=5000  wofz=2b2b28144cd62827 erfi=83e2f27fd0759be5 betaincinv=efa7f9d587e2df5f
n=50000 wofz=f3bd90a151e27607 erfi=56f7415d83f2bef1 betaincinv=3b79e5d6334fc8f6
timing acc: wofz 1M=6c75ea3ef4cef851  betaincinv 500k=65da0bceb5884971
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = 8b8ec6e7c6de97a9b96bb7e58a371e1f03ae72093cf9628a0a2b5bc8d0b7b818

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| function / array  | serial (3x)             | new (3x)             | speedup |
|-------------------|-------------------------|----------------------|---------|
| wofz,       1M    | 1.255/1.232/1.207 s     | 52.83/56.08/52.93 ms | ~23.2x  |
| betaincinv, 500k  | 826.5/814.0/787.6 ms    | 31.59/35.11/33.02 ms | ~24.7x  |

## Validation
9 wofz unit tests pass; clippy: no warning in convenience.rs.
With this, all four convenience array dispatchers (map_real, map_real_binary,
map_real_or_complex, map_real_ternary) + wofz inline are parallel.
