# perf: parallelize incomplete elliptic integrals ellipkinc/ellipeinc real arms

Bead: frankenscipy-hgflr

## Lever
`map_real_or_complex_binary` (crates/fsci-special/src/elliptic.rs) — behind `ellipkinc` and
`ellipeinc` (incomplete elliptic integrals F(φ|m), E(φ|m), via Carlson symmetric forms
RF/RD; used in pendulum periods, arc length, EM fields) — mapped its per-element kernel
serially over the three real-real vector broadcast arms (`vec×scalar`, `scalar×vec`,
`vec×vec` zip). Routed all three through the `par_map_indices` helper already in elliptic.rs.
The complex broadcast arms are left serial (niche).

## Isomorphism / byte-identity argument
- Each output index `i` is `real_kernel(...values[i]...)` written to slot `i`; chunks cover
  the index range contiguously and concatenate in index order ⇒ identical output `Vec`. No
  reduction.
- Error path: chunk results folded with `?` in chunk (=index) order ⇒ first failing index's
  `SpecialError`, exactly as the serial `.iter().map(kernel).collect()`.
- The `vec×vec` length check and the empty/complex fail-closed arms are untouched. Gate:
  serial for `< 256` elements.

⇒ The returned value (and first error) is bit-identical to the serial implementation.

## Proof (golden — serial vs parallel, identical)
Harness: `cargo run --profile release-perf -p fsci-special --bin perf_ellipinc_array`
(m=0.5, φ∈(-1.5,1.5))
```
n=100   ellipkinc=889e5010624e624c ellipeinc=59cb4790182f8674
n=5000  ellipkinc=4a3196ce93b179dc ellipeinc=9d9408bb29ca96cd
n=50000 ellipkinc=cc7675d7873516de ellipeinc=abcba6e865ff4346
timing acc: ellipkinc 1M=0a1bda24a9efc28e  2M=568c5451f3c6a688
```
Identical in the stashed serial build and the parallel build.
sha256(golden payload file) = b13c018a32da0a94bc14f42f134bcd03ddc9593e37f34f49bfe58ebbee7b9602

## Timing (rch remote, release-perf, 3 back-to-back runs each)
| array | serial (3x)             | new (3x)            | speedup |
|-------|-------------------------|---------------------|---------|
| 1M    | 147.9/148.1/151.0 ms    | 14.37/12.26/12.98 ms| ~11.4x  |
| 2M    | 309.1/302.7/312.8 ms    | 20.13/23.81/20.82 ms| ~14.7x  |

## Validation
10 ellipkinc/ellipeinc unit tests pass; clippy: no warning in elliptic.rs.
