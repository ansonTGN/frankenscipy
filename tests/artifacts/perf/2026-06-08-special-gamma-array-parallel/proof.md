# perf: parallelize gamma-family real/complex-array dispatch

Bead: frankenscipy-lyg9p

## Lever
The four core gamma dispatchers in crates/fsci-special/src/gamma.rs —
`gamma_dispatch`, `gammaln_dispatch`, `loggamma_dispatch`, `digamma_dispatch` (public
`gamma`/`gammaln`/`loggamma`/`digamma`/`psi`) — mapped their per-element kernels (Lanczos
gamma, log-gamma, digamma series; complex variants) serially over the `RealVec` and
`ComplexVec` arms. Added one generic index-based helper `par_map_indices<T>(n, f)`
(T = f64 or Complex64; infallible complex kernels wrap their result in `Ok`) and routed all
8 vec arms through it. gamma/gammaln/digamma are among the most ubiquitous special functions
in stats/Bayesian/combinatorics.

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
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_gamma_array`
```
n=100   gamma=f360fee0b21b3129 gammaln=a527edeb0272cad8 digamma=3f5fb951f90af0d9
n=5000  gamma=4a29c703711d4481 gammaln=f0fa3910602d607c digamma=f74c96336f68c873
n=50000 gamma=36b12fcd487bf449 gammaln=0c647528ab6e610a digamma=f13c00a608c13176
timing acc (gammaln): 2M=2d8b62775409c310  4M=708f23a725ac0400
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = 8db0953b5ce3821d49f1a9062691cba09e1a1a9d61e2ddda2b3b5cfa8123e8f5

## Timing (rch remote, release-perf, 3 back-to-back runs each) — gammaln (x∈(0,50])
| array | serial (3x)          | new (3x)            | speedup |
|-------|----------------------|---------------------|---------|
| 2M    | 41.68/45.05/41.05 ms | 13.22/12.81/14.02 ms| ~3.2x   |
| 4M    | 91.31/95.43/98.37 ms | 15.75/17.10/16.67 ms| ~5.7x   |

(gammaln/gamma are cheaper per element than erf, so the ratio is moderate — partly
bandwidth-bound — but these are the most ubiquitous special functions.)

## Validation
133 gamma unit tests pass; clippy: no warning in gamma.rs (helper + arms clean).
