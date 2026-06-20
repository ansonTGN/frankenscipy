# fsci-ndimage gaussian_filter tile-local scratch proof

- Agent: cod-a / BlackThrush
- Bead: `frankenscipy-8l8r1.132`
- Date: 2026-06-20
- Decision: KEEP

## Lever

`gaussian_filter_2d_reflect_order0` already had a cache-planned reflect index
table and a folded AXPY first pass. This lever keeps the same reflected tap
plans and tolerance surface, but changes scratch layout: each worker row chunk
now computes its vertical pass into a thread-local scratch tile and immediately
runs the horizontal pass from that hot tile into the output chunk.

This removes the full-image scratch buffer and the second scoped thread barrier.
It is the cache-blocked/tile-local route recommended by the previous Gaussian
negative evidence, not another scalar tap peel.

## Benchmark Evidence

Rust commands used `RCH_REQUIRE_REMOTE=1` and
`CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a`.

| Route | Worker | Command | Mean | Interval / value | Verdict |
| --- | --- | --- | ---: | ---: | --- |
| Current `ce1857ab` | `hz2` | `cargo bench -p fsci-ndimage --bench ndimage_bench -- correlate_gaussian/gaussian_sigma2/256 --sample-size 10 --measurement-time 1 --warm-up-time 1` | 1.9819 ms | [1.8484, 2.2050] ms | baseline; Rust 1.34x slower than local SciPy |
| Tile-local scratch candidate | `hz2` | same | 1.2274 ms | [1.1564, 1.2960] ms; Criterion change -40.721% | keep: 1.61x faster than current |
| Current A/B gather arm | `hz2` | `cargo test -p fsci-ndimage gaussian_2d_axpy_ab_timing --release -- --ignored --nocapture` | 2760.0 us | same-process interleaved | profile/proven current route arm |
| Current A/B AXPY arm | `hz2` | same | 2430.3 us | same-process interleaved | existing AXPY still 1.14x faster than gather |
| SciPy `ndimage.gaussian_filter` | local | `python3 docs/perf_oracle_ndimage.py` | 1.47367 ms | p50 | candidate is 1.20x faster than SciPy |

Scores:

- Same-worker Rust current vs candidate: `1/0/0`.
- Strict SciPy score for candidate: `1/0/0`.
- Previous final-source Gaussian row was a SciPy loss on the same oracle
  family; this flips the tracked `gaussian_sigma2/256` release row to a win.

## Correctness And Gates

- PASS: rch focused Gaussian suite:
  `cargo test -p fsci-ndimage gaussian --lib -- --nocapture` =
  31 passed / 0 failed / 1 ignored.
- PASS: local live SciPy conformance:
  `FSCI_REQUIRE_SCIPY_ORACLE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a-local cargo test -p fsci-conformance --test diff_ndimage_gaussian_filter -- --nocapture`
  = 1 passed / 0 failed.
- PASS: rch per-crate compile:
  `cargo check -p fsci-ndimage --all-targets` passed on `vmi1149989`;
  unrelated existing warnings remained in `fsci-interpolate` and `diff_geom`.
- PASS: `git diff --check`.
- PASS: `ubs crates/fsci-ndimage/src/lib.rs` exited 0 with 0 critical findings;
  broad existing warnings remain inventory.
- BLOCKED: `cargo fmt -p fsci-ndimage -- --check` remains blocked by
  pre-existing formatting drift in `ndimage_bench.rs`, `diff_fourier.rs`, and
  older `src/lib.rs` hunks outside this change.
- BLOCKED: `cargo clippy -p fsci-ndimage --all-targets -- -D warnings` stopped
  before this patch on existing `fsci-linalg` dependency lints
  (`needless_range_loop` and `needless_borrow`).

## Retry Notes

Do not return to the full-image scratch plus two scoped thread barriers for this
2-D Reflect/order-0 route. The measured path forward from here is smaller:
remove per-call source-plan allocation, specialize tiny fixed-radius plans, or
try a fused/tiled source-plan cache only with same-worker proof.
