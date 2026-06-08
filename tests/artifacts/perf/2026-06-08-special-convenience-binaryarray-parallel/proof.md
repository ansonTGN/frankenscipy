# perf: parallelize convenience-module binary real-array dispatch (convenience::map_real_binary)

Bead: frankenscipy-kf30s

## Lever
`map_real_binary` (crates/fsci-special/src/convenience.rs) is the shared binary real-array
dispatcher behind **18 functions** — including `gammaincinv`/`gammainccinv` (inverse
regularized incomplete gamma — the chi²/gamma distribution quantiles, iterative/expensive),
`owens_t` (Owen's T, used in skew-normal / bivariate-normal probabilities), `boxcox`/
`boxcox1p`/`inv_boxcox`, `powm1`, `stirling2`, and the info-theory / ML terms
`xlogy`/`xlog1py`/`rel_entr`/`binary_cross_entropy`/`huber`/`pseudo_huber`. It mapped its
per-element kernel serially over the three real-vec broadcast arms (`vec×scalar`,
`scalar×vec`, `vec×vec` zip). Routed all three through the `par_map_indices` helper already
in this file (added with the convenience map_real commit).

## Isomorphism / byte-identity argument
- Each output index `i` is `kernel(...values[i]...)` written to slot `i`; chunks cover the
  index range contiguously and concatenate in index order ⇒ identical output `Vec`. No
  reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.iter().map(kernel).collect()`.
- `vec×vec` length-mismatch check + unsupported-type fail-closed arm untouched. Gate: serial
  for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_conv_binary_array`
```
n=100   gammaincinv=abebab28ceaf3557 owens_t=6ee89ce4f34176ba
n=5000  gammaincinv=225afa0bf9b94990 owens_t=3b42d950108bdab3
n=50000 gammaincinv=8f38b0a20998d2fd owens_t=78172e6ac7e079c8
timing acc: gammaincinv 500k=ab06af22c212e000  owens_t 1M=7b4c1f7662e171a7
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = e8f820db15d55b63e2d31a008f73417e54b778040f5bdf6db3380bcd6cc18470

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| function / array     | serial (3x)             | new (3x)             | speedup |
|----------------------|-------------------------|----------------------|---------|
| gammaincinv, 500k    | 392.6/385.7/388.8 ms    | 23.77/18.02/21.02 ms | ~18.5x  |
| owens_t,     1M      | 57.28/52.94/53.09 ms    | 8.52/10.44/8.89 ms   | ~6.0x   |

## Validation
fsci-special unit tests pass (owens/gammaincinv); clippy: no warning in convenience.rs.
