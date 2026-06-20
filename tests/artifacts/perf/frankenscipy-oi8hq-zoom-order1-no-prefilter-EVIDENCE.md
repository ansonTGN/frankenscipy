# frankenscipy-oi8hq - ndimage zoom order=1 no-prefilter fast path

Date: 2026-06-20
Agent: cod-b / MistyBirch

## Decision

KEEP. The 2-D `BoundaryMode::Reflect`, `order=1` `zoom` path now skips
`prefilter_spline_coefficients` entirely and interpolates directly from the
original image with precomputed row/column linear supports.

This closes the previous `frankenscipy-wm14d` residual SciPy loss for
`scipy.ndimage.zoom(256x256, 2x, order=1)`.

## Lever

The prior fast path still paid an order-1 spline setup cost: for reflect/mirror
boundary modes the spline builder padded/copied the image even though order-1
has no recursive prefilter. For zoom coordinates in the original image domain,
the padded coefficient grid is only an integer-offset view of the same samples.
The replacement computes the same linear basis weights in the original
coordinate domain and performs the same four-load bilinear sum over
`input.data`.

## Performance Evidence

Baseline and final-source Rust command:

```text
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo bench -p fsci-ndimage --bench ndimage_bench -- zoom/2x_256/1 --noplot --sample-size 10 --measurement-time 1 --warm-up-time 1
```

SciPy oracle command:

```text
python3 docs/perf_oracle_zoom.py
```

| Workload | Result | Worker / host | Mean / median | Verdict |
| --- | ---: | --- | ---: | --- |
| Rust prepatch residual current | `[7.5432, 8.8419, 9.7257] ms` | rch `hz2` | 8.8419 ms | baseline |
| Rust final source | `[1.0624, 1.2219, 1.5189] ms` | rch `vmi1149989` | 1.2219 ms | keep |
| SciPy oracle, order=1 | `4861.71 us` | local Python 3.13.7 / SciPy 1.17.1 | 4.86171 ms | oracle |
| SciPy oracle, order=3 | `13397.15 us` | local Python 3.13.7 / SciPy 1.17.1 | 13.39715 ms | reference |

Ratios:

- Final Rust vs SciPy order=1: `4.86171 / 1.2219 = 3.98x faster`.
- Final Rust vs prepatch residual current: `8.8419 / 1.2219 = 7.24x faster`
  across rch workers. This is routing-strength internal evidence, not a
  same-worker A/B claim.
- Previous recorded `frankenscipy-wm14d` residual current was `7.9684 ms`, so
  final source is `6.52x` faster than that recorded residual row.

SciPy win/loss/neutral for final source: `1/0/0`.

## Correctness / Conformance Gates

- PASS: `rustfmt --edition 2024 --check crates/fsci-ndimage/src/lib.rs`.
- PASS: `CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo check -p fsci-ndimage --all-targets`
  on rch `hz1` with existing unrelated dependency/bin warnings.
- PASS: `CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo test -p fsci-ndimage zoom_ --lib -- --nocapture`
  on rch `ovh-a`: `6 passed; 0 failed; 235 filtered out`.
- PASS: focused bit-equivalence guard
  `zoom_order_one_reflect_fast_path_matches_generic_sampler_bits`.
- PASS: `FSCI_REQUIRE_SCIPY_ORACLE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b cargo test -p fsci-conformance --test diff_ndimage_zoom -- --nocapture`:
  `1 passed; 0 failed`.
- PASS: `ubs crates/fsci-ndimage/src/lib.rs docs/GAUNTLET_RELEASE_SCORECARD.md docs/progress/perf-negative-results.md docs/progress/perf-release-readiness-scorecard.md tests/artifacts/perf/frankenscipy-oi8hq-zoom-order1-no-prefilter-EVIDENCE.md`
  exited 0; it reported broad pre-existing `fsci-ndimage` warning inventory
  but no critical issues.
- BLOCKED: `CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo clippy -p fsci-ndimage --all-targets -- -D warnings`
  failed before this patch on existing `fsci-linalg` dependency lints
  (`needless_range_loop`, `needless_borrow`).

## Negative Evidence / Stop Rule

No revert. The no-prefilter/order-1 route is now a measured SciPy win.

Do not retry padding, spline-coefficient materialization, or scheduler-only
retuning for this 2-D reflect/order-1 row without a fresh profile showing they
beat the current direct-original path. Future work should move to order-3
zoom, SIMD/tiled row interpolation, or different ndimage residual losses.
