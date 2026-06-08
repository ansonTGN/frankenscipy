# perf: correlate2d direct 2D convolution — parallel over output rows, 4.9–9.1x byte-identical

## Lever (ONE)
`correlate2d`'s direct path (taken for large images with small kernels, where the
FFT crossover keeps direct — its own comment notes "no regression for big-image/
small-kernel shapes, where direct stays ahead") was a serial scatter:

```rust
for i in 0..ar { for j in 0..ac {
    let aval = a[i*ac+j];
    for ki in 0..vr { for kj in 0..vc {
        full[(i+ki)*full_c + (j+kj)] += aval * v_rev[ki*vc+kj];
    }}
}}
```

O(ar·ac·vr·vc). Output row `o` receives contributions only from input rows `i`
with `i ≤ o ≤ i+vr-1`, so the output rows are independent. Partition the output
rows across threads; each thread iterates the SAME (i,j,ki,kj) nesting restricted
to the input rows feeding its rows (`i ∈ [or0-vr+1, or1)`, `ki ∈ [or0-i, or1-i)`),
writing only its disjoint output-row slice.

## Parity — BYTE-IDENTICAL
- For each output cell `full[o][oj]`, the contributing `(i, ki=o-i)` pairs are
  visited in the same `i`-ascending order as the serial loop, with the same inner
  `j`/`kj` order, so the floating-point accumulation is identical. Threads write
  disjoint output rows (no contention).
- Full-output FNV hash matches the serial baseline exactly (parallel vs stashed
  serial): `fa275c81…`, `a4a5ed73…`. See `golden_payload.txt`.
- All 26 convolve + 6 correlate `fsci-signal` tests pass. A work gate keeps small
  inputs (and every conformance case) on the serial loop.

## Timing — rch remote, 64 cores, `--profile release-perf`
`correlate2d` (Same mode), large image × small kernel (direct path):

| image       | kernel | serial    | parallel  | speedup |
|-------------|--------|-----------|-----------|---------|
| 1500 × 1500 | 11×11  | 166.499 ms| 20.990 ms | 7.93x   |
| 2000 × 2000 | 15×15  | 456.856 ms| 50.094 ms | 9.12x   |
| 3000 × 3000 | 9×9    | 534.774 ms| 108.705 ms| 4.92x   |

Score ≥ 2.0 cleared. (3000² is lower — output write bandwidth bound at 9M cells.)
Each thread's inner kernel also benefits from contiguous row writes.

Harness: `crates/fsci-signal/src/bin/perf_correlate2d_par.rs`
Run: `cargo run --profile release-perf -p fsci-signal --bin perf_correlate2d_par`

## Notes
- Complements the existing direct↔FFT crossover (large kernels still take FFT);
  this only accelerates the direct branch (small/medium kernels on big images),
  which is the common image-processing regime (blur, edge filters).
