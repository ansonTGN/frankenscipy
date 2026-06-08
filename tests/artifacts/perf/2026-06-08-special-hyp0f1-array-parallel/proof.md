# perf: parallelize hyp0f1 confluent-limit real-array arms

Bead: frankenscipy-tf0wx

## Lever
`hyp0f1_dispatch` (confluent hypergeometric limit 0F1; closely related to Bessel functions)
in crates/fsci-special/src/hyper.rs mapped its per-element series kernel serially over the
three real-real broadcast arms (`b_scalar × z_vec`, `b_vec × z_scalar`, `b_vec × z_vec` zip).
Routed all three through the `par_map_indices` helper already in hyper.rs; the complex /
mixed arms are left serial (niche).

## Isomorphism / byte-identity argument
- Each output index `i` is `hyp0f1_scalar(...broadcast at i...)` written to slot `i`; chunks
  cover the index range contiguously and concatenate in index order ⇒ identical output `Vec`.
  No reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial loop's `?`.
- The `b_vec×z_vec` length check + scalar/complex/mixed arms are untouched. Gate: serial for
  `< 64` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_hyp0f1_array`
(b=1.5, z∈(-20,20))
```
n=100   hyp0f1=af46979cd98058fb
n=5000  hyp0f1=f84838d803a24e7b
n=50000 hyp0f1=92473008848f6bce
timing acc: hyp0f1 500k=2b6fd590d9099a3b  1M=c550088beabc5a06
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = 3cf6acbe10ea18e9ece07bae1214f96056ab1556750a50500a5dddd70505cc1f

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| array | serial (3x)          | new (3x)          | speedup |
|-------|----------------------|-------------------|---------|
| 500k  | 30.58/32.17/30.30 ms | 6.50/8.13/7.62 ms | ~4.0x   |
| 1M    | 57.63/58.72/65.73 ms | 9.22/9.17/9.66 ms | ~6.3x   |

## Validation
17 hyp0f1 unit tests pass; clippy: no warning in hyper.rs. With this, all hyper-module
broadcast dispatchers (hyp0f1, hyp1f1, hyp2f1, hyperu) are parallel.
