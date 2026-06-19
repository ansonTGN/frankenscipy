# frankenscipy-u0ucw Wide Lstsq Gauntlet

Agent: cod-a / MistyBirch
Date: 2026-06-19

## Decision

Revert the row-streamed wide `lstsq` micro-optimization. Keep the current
materialized `A^T` normal-equation route.

## Measurements

| Route | Environment | Criterion mean | Ratio | Verdict |
| --- | --- | ---: | ---: | --- |
| Rust row-streamed `A A^T` + `A^T y` | `rch` worker `vmi1227854` | 139.965 ms | 0.966x vs materialized | loss |
| Rust materialized `A^T` pre-revert | `rch` worker `vmi1227854` | 135.206 ms | 1.00x internal reference | keep |
| Rust current materialized `A^T` after revert | local SciPy host | 109.370 ms | 11.46x faster than SciPy | keep |
| SciPy `scipy.linalg.lstsq(check_finite=False)` | Python 3.13.7 / NumPy 2.4.3 / SciPy 1.17.1 | 1.253347 s | 1.00x oracle | reference |

## Raw Artifacts

- `bench_u0ucw_wide_lstsq_rch_hz1.txt`: remote same-worker A/B. The worker
  lacked SciPy, so this file proves the row-streamed Rust candidate lost to the
  materialized Rust route.
- `bench_u0ucw_wide_lstsq_local_scipy_after_revert.txt`: local head-to-head
  Criterion run against original SciPy.

## Gates

- PASS: `rch exec -- env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a cargo check -p fsci-linalg --benches`
- PASS: `rch exec -- env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a cargo test -p fsci-linalg wide_pinv -- --nocapture`
- PASS: `rch exec -- env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a cargo test -p fsci-linalg public_wide_min_norm_lstsq_route_perf_probe --release -- --ignored --nocapture`
- PASS: `rustfmt --edition 2024 --check crates/fsci-linalg/benches/linalg_bench.rs`
- BLOCKED: `rustfmt --edition 2024 --check crates/fsci-linalg/src/lib.rs crates/fsci-linalg/benches/linalg_bench.rs`
  reports file-wide formatting drift in `src/lib.rs` outside this revert.
- BLOCKED: `rch exec -- env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a cargo clippy -p fsci-linalg --benches -- -D warnings`
  fails on existing `src/lib.rs` lints and concurrently modified
  `src/cossin.rs` excessive-precision literals.

The release probe reported `shape=256x512`, `lstsq_speedup=15.571283`, and
`lstsq_max_abs_diff=3.38840067115597776e-13`.
