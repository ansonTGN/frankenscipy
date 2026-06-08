# perf: ndimage axes-subset rank/median/percentile filters — parallel, 22–25x byte-identical

## Lever (ONE)
The full-image rank/median/percentile filters already parallelized via
`fill_rank_filter`, but the **axes-subset** core
`rank_filter_index_usize_axes_with_origins` (used by `rank_filter_axes`,
`median_filter_axes`, `percentile_filter_axes`, and the `minimum`/`maximum`
filter axes paths) still ran a serial per-output-pixel loop:

```rust
for flat_out in 0..input.size() {
    let out_idx = input.unravel(flat_out);
    let mut neighborhood = ...; // gather footprint over the selected axes
    output.data[flat_out] = select_total_rank(&mut neighborhood, rank);
}
```

Each output pixel depends only on a read-only neighbourhood, so route the loop
through the existing `fill_pixels_parallel` helper (the same one `generic_filter`
uses) — distributing the disjoint output pixels across threads, each with a reused
scratch neighbourhood buffer. Handles arbitrary axes subsets (unlike the
full-ndim `rank_filter_pixel` path).

## Parity — BYTE-IDENTICAL
- `fill_pixels_parallel` computes each `output.data[flat_out]` with the identical
  gather + `select_total_rank`, written to its own slot ⇒ bit-identical to the
  serial loop. The `perf_rank_filter` golden generator's digests match EXACTLY
  between the serial and parallel builds for all 6 cases — including the
  axis-subset cases (`median_axis_last_wrap_size9`,
  `rank_axis_first_nearest_size9_rank3`) and NaN/±Inf/±0 edge values. See
  `golden_payload.txt`.
- All 24 rank/median/percentile + 6 min/max `fsci-ndimage` filter tests pass.

## Timing — rch remote, 64 cores, `--profile release-perf`
`rank_filter_axes` (axis-0 median):

| image       | size | serial   | parallel  | speedup |
|-------------|------|----------|-----------|---------|
| 1500 × 1500 | 31   | 2.246 s  | 101.281 ms| 22.2x   |
| 2000 × 2000 | 41   | 5.115 s  | 222.338 ms| 23.0x   |
| 3000 × 3000 | 21   | 6.557 s  | 266.674 ms| 24.6x   |

Score ≥ 2.0 cleared with a large margin. (The full-image rank/median/percentile
paths were already parallel via `fill_rank_filter`; this completes the axes-subset
variants, which were the remaining serial gap.)

Harness: `crates/fsci-ndimage/src/bin/perf_rank_filter_timing.rs` (timing),
`crates/fsci-ndimage/src/bin/perf_rank_filter.rs` (golden byte-identity).
Run: `cargo run --profile release-perf -p fsci-ndimage --bin perf_rank_filter_timing`
