# perf: parallelize Bessel/Hankel derivative + spherical-complex array arms

Bead: frankenscipy-06p0s

## Lever
Three dispatchers in crates/fsci-special/src/bessel.rs, all previously serial in the relevant arms:

1. `bessel_derivative_dispatch` (behind `jvp`/`yvp`/`ivp`/`kvp` — n-th derivatives of J/Y/I/K) —
   all 6 array arms (3 real-real, 3 real-order×complex-z) mapped `bessel_derivative_*_scalar`
   serially. The derivative kernel is a finite difference sum of Bessel evaluations (expensive).
2. `hankel_derivative_dispatch` (behind `h1vp`/`h2vp` — derivatives of H^(1)/H^(2), complex output) —
   all 6 array arms serial; even the real-z arms produce ComplexVec.
3. `spherical_bessel_dispatch` (behind `spherical_jn/yn/in/kn`) — its real arms were already
   parallel, but its 3 complex-z arms (`spherical_bessel_complex_scalar`) were still serial.

For (1) and (2) I introduced two local `eval_real`/`eval_complex` closures (capturing the
Copy/Sync `function`/`derivative_order`/`mode`/`kind`/`rule`) to dedup the repeated 7-arg calls,
then routed every array arm through the file's generic `par_map_indices`. For (3) I routed the
3 complex arms through `par_map_indices`. Scalar arms, length checks, complex-ORDER
fail-closed arms, and empty arms are untouched.

## Isomorphism / byte-identity argument
- Each kernel is pure in its per-index `(order, z)` given the fixed Copy params. Each output
  index `i` is the kernel applied at index `i` written to slot `i`; chunks cover the index range
  contiguously and concatenate in index order ⇒ identical output `Vec`. No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.collect::<Result<Vec<_>,_>>()`.
- vec×vec length checks preserved; gate: serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run -p fsci-special --bin perf_bessel_deriv_spherical`
(jvp: integer-ish orders 1..9, z=6.5 real; h1vp: same orders, complex z re∈(1,19) im∈(0,3);
spherical_jn: order 3, complex z.)

Byte-identity verified TWO ways (see golden_payload.txt):
1. **dev profile** — captured the harness GOLDEN_PAYLOAD for BOTH the serial (stashed) and the
   parallel source; all 9 (function × size) hashes are bit-for-bit identical.
2. **release-perf profile** — the serial and parallel builds produced the SAME 300k timing
   accumulators (jvp=de5bc7fb125d7fe9, h1vp=3838b1851db21ada, spherical_jn=f9f764e81b273571).

(The dev vs release-perf h1vp hash differs by one ULP — a kernel fma/optimization difference
between profiles, NOT the parallelization; within each profile serial==parallel exactly.)
sha256(golden_payload.txt) = 2440c5d9e577aa2c5c5a87f4e0f9edae6df2f7a4554c5ba7b778922b5e8330fd

## Timing (rch remote, release-perf, 3 back-to-back runs each, uncontended)
| function / array      | serial (3x)             | new (3x)               | speedup |
|-----------------------|-------------------------|------------------------|---------|
| jvp, 300k             | 91.9/86.2/87.6 ms       | 8.39/6.64/8.46 ms      | ~11x    |
| h1vp, 300k cplx-z     | 1.232/1.301/1.257 s     | 53.7/48.0/47.0 ms      | ~26x    |
| spherical_jn, 300k cplx | 56.3/58.9/56.4 ms     | 7.34/6.49/7.03 ms      | ~8x     |

(NOTE: an unrelated rustc toolchain bump invalidated the release-perf dep cache mid-run; the
first timing pass read 0.4–1.5s due to a concurrent full dep rebuild contending for cores. After
the rebuild settled, the uncontended numbers above are stable. acc values were identical throughout.)

## Validation
992 fsci-special unit tests pass; clippy: no warning in bessel.rs.
