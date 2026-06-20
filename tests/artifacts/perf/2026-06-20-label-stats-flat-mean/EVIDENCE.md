# Label-Stats Flat Mean Accumulator Evidence

- Date: 2026-06-20
- Agent: cod-a / MistyBirch
- Bead: `frankenscipy-8l8r1.125`
- Decision: KEEP as an internal win; residual SciPy loss remains routed deeper.

## Lever

`ndimage.mean(input, labels, index)` no longer materializes one `Vec<f64>` per
requested label before computing the mean. It streams the input once into flat
`sum` and `count` arrays keyed by the same first-wins label map used by the
previous grouped route.

This keeps the already-landed O(N+K) lookup while removing per-label bucket
allocation and every value push for the `mean` reduction. Other reductions that
need grouped values, such as median and labeled comprehension, still use the
shared materialized grouping route.

## Same-Host A/B

Command:

```text
/data/projects/.rch-targets/frankenscipy-cod-a/release/perf_label_stats
```

The binary compares the old O(N*K) linear scan, the previous shipped O(N+K)
bucketed route, and the current flat accumulator in one optimized executable.
`mism=0/0` means bit-identical output against both old and bucketed routes.

| N | K | old O(N*K) | bucketed O(N+K) | flat O(N+K) | old/flat | bucketed/flat | mismatches |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 65536 | 512 | 13.129 ms | 1.047 ms | 590.978 us | 22.2x | 1.77x | 0/0 |
| 262144 | 1024 | 105.475 ms | 3.695 ms | 2.568 ms | 41.1x | 1.44x | 0/0 |
| 262144 | 2048 | 210.724 ms | 4.140 ms | 2.713 ms | 77.7x | 1.53x | 0/0 |
| 589824 | 4096 | 883.314 ms | 11.760 ms | 6.951 ms | 127.1x | 1.69x | 0/0 |

Internal win/loss/neutral versus the prior bucketed route: `4/0/0`.

## SciPy Oracle

Command:

```text
python3 docs/perf_oracle_label_stats.py
```

Local SciPy oracle output:

```text
scipy.ndimage.mean(labels, index) over K labels, reps=15
  N=  65536 K=  512  p50 =      0.159 ms
  N= 262144 K= 1024  p50 =      0.622 ms
  N= 262144 K= 2048  p50 =      0.581 ms
  N= 589824 K= 4096  p50 =      1.688 ms
```

| N | K | Rust flat | SciPy p50 | Rust vs SciPy | Verdict |
| ---: | ---: | ---: | ---: | ---: | --- |
| 65536 | 512 | 590.978 us | 159 us | 3.72x slower | loss |
| 262144 | 1024 | 2.568 ms | 622 us | 4.13x slower | loss |
| 262144 | 2048 | 2.713 ms | 581 us | 4.67x slower | loss |
| 589824 | 4096 | 6.951 ms | 1.688 ms | 4.12x slower | loss |

SciPy win/loss/neutral for final source: `0/4/0`.

## Guards

- PASS: `RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo test -p fsci-ndimage measurement_reduction_wrappers -- --nocapture`
  - `2 passed; 0 failed`.
- PASS: `RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo test -p fsci-ndimage --lib -- --nocapture`
  - `240 passed; 0 failed`.
- PASS: `RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo check -p fsci-ndimage --all-targets`
  - existing warnings remain in `fsci-interpolate` and `diff_geom`; no new error.
- PASS: `rustfmt --edition 2024 --check crates/fsci-ndimage/src/lib.rs crates/fsci-ndimage/src/bin/perf_label_stats.rs`.
- PASS: `git diff --check -- crates/fsci-ndimage/src/lib.rs crates/fsci-ndimage/src/bin/perf_label_stats.rs`.
- PASS: `ubs crates/fsci-ndimage/src/lib.rs crates/fsci-ndimage/src/bin/perf_label_stats.rs`
  - exit 0; no critical issues; broad existing warning inventory left untouched.
- BLOCKED: `RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo clippy -p fsci-ndimage --lib -- -D warnings`
  - stopped before this crate on existing `fsci-linalg` dependency lints
    (`needless_range_loop`, `needless_borrow`) outside this patch.

## Negative Evidence

The flat mean path removes group materialization for this one reduction, but
Rust remains 3.7-4.7x slower than SciPy's compiled C implementation on the
measured label-indexed `mean` rows.

Do not retry another `Vec<Vec<f64>>` grouping variant for `mean`. The next
route needs to attack the remaining constant factor: dense label lookup when
labels are small/contiguous, sorted-label remapping, specialized integer-label
paths, or SIMD/cache-tiled accumulation while preserving exact label equality
and first-position duplicate-index semantics.
