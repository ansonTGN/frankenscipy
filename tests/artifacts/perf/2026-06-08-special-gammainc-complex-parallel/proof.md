# perf: parallelize complex incomplete-gamma + multigammaln array arms

Bead: frankenscipy-q5szh

## Lever
Two dispatchers in crates/fsci-special/src/gamma.rs:

1. `gammainc_dispatch` (behind `gammainc` = P(a,x) and `gammaincc` = Q(a,x)) — its **real**
   array arms were already parallel, but its **9 complex array arms** (Real-a/Complex-x,
   Complex-a/Real-x, Complex-a/Complex-x; each as vec×scalar, scalar×vec, vec×vec) mapped the
   `complex_gammainc_scalar` / `complex_gammaincc_scalar` / `gammainc_complex_parameter_scalar`
   kernels serially with `.iter().map(...).collect()`. Routed all 9 through `par_map_indices`.
2. `map_real_input` (generic unary real dispatcher behind `gammasgn` and `multigammaln`) — its
   `RealVec` arm was serial; added `+ Sync` to the `F` bound and routed it through
   `par_map_indices`. The heavy consumer is `multigammaln` (a d-term sum of `gammaln`).

All scalar arms, vector length checks, and empty/fail-closed arms are untouched.

## Isomorphism / byte-identity argument
- Each kernel is a pure free `fn` of its per-index operands (the `lower`/`d`/`mode` captures are
  `Copy` + `Sync`; real operands lifted to `Complex64::new(x, 0.0)` at the same index). Each
  output index `i` is written from the kernel applied at index `i`; chunks cover the index range
  contiguously and concatenate in index order ⇒ identical output `Vec`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.collect::<Result<Vec<_>,_>>()`.
- vec×vec length checks preserved; gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_gammainc_complex`
(gammainc/gammaincc: a=2.5, x complex re∈(0.5,12.5) im∈(-2,2); multigammaln: d=4, x∈(5,25))
```
n=100 gammainc(len=100,xor=32afa993bc4232bb) gammaincc(len=100,xor=0e0e1be1de989180) multigammaln(len=100,xor=f6b0ed54e4b9b1e1)
n=5000 gammainc(len=5000,xor=9fd894c9702ce028) gammaincc(len=5000,xor=214cac68bb60021d) multigammaln(len=5000,xor=f10e5a159e653f37)
n=50000 gammainc(len=50000,xor=210ade10dbf450fa) gammaincc(len=50000,xor=4d841a7ee19f5d27) multigammaln(len=50000,xor=ad4374db4e604312)
timing acc: gammainc 300k=7ceae9dd6bfd22fb  gammaincc 300k=0281d6cec965e56d  multigammaln 1M=de56e4fa3ecdccda
```
Identical in the stashed serial build and the parallel build (golden xor + timing acc match).
sha256(golden_payload.txt) = e2b2990902891df37db9fe581ebf72454088247be4c8b93ec4d266180d9d20c3

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| function / array      | serial (3x)             | new (3x)               | speedup |
|-----------------------|-------------------------|------------------------|---------|
| gammainc, 300k cplx   | 164.8/166.9/157.3 ms    | 12.39/11.60/12.12 ms   | ~13.5x  |
| gammaincc, 300k cplx  | 166.0/157.8/157.7 ms    | 12.39/11.40/9.25 ms    | ~14x    |
| multigammaln, 1M      | 74.6/80.0/80.2 ms       | 9.68/11.10/10.19 ms    | ~7.8x   |

## Validation
25 gammainc + 6 multigammaln unit tests pass; clippy: no warning in gamma.rs.
