# 2026-06-20 cod-b filter1d queue fusion evidence

Agent: cod-b / BlackThrush
Bead: `frankenscipy-8l8r1.134`
Decision: KEEP, internal win with remaining SciPy residual loss.

## Lever

Replace the public `maximum_filter1d` / `minimum_filter1d` HGW prefix-suffix
route with a single-pass monotonic index queue over the same boundary-resolved
line. The queue stores indices in a contiguous `Vec<usize>`, counts NaNs
out-of-band, and evicts older equal extrema so signed-zero ties match the
left-to-right `f64::max/min` fold.

## Correctness

- `filter1d_hgw_byte_identical_to_fold`: PASS via rch `vmi1149989`.
- `filter1d_queue_vs_hgw_ab_timing`: PASS via rch `hz2`; the test compares HGW
  and queue outputs bit-for-bit before timing.
- `FSCI_REQUIRE_SCIPY_ORACLE=1 cargo test -p fsci-conformance --test diff_ndimage_filter_1d -- --nocapture`:
  PASS locally, 1 passed / 0 failed. This conformance target currently exercises
  `uniform_filter1d`; max/min filter1d SciPy boundary parity remains documented
  out of scope there.

## Performance

Baseline HGW means are from the same-worker `hz2` Criterion baseline recorded
before the candidate. Candidate means are from `filter1d_criterion_after.txt`.
SciPy medians are local SciPy 1.17.1 / NumPy 2.4.3 from
`filter1d_local_scipy.txt`.

| Workload | HGW baseline | Queue final | Internal ratio | SciPy median | Final vs SciPy |
| --- | ---: | ---: | ---: | ---: | ---: |
| `maximum_filter1d`, n=65536, size=31 | 1.2413 ms | 0.56072 ms | 2.21x faster | 0.51803 ms | 1.08x slower |
| `minimum_filter1d`, n=65536, size=31 | 1.0365 ms | 0.76956 ms | 1.35x faster | 0.54051 ms | 1.42x slower |
| `maximum_filter1d`, n=65536, size=101 | 1.0385 ms | 0.82422 ms | 1.26x faster | 0.51482 ms | 1.60x slower |
| `minimum_filter1d`, n=65536, size=101 | 1.0234 ms | 0.77760 ms | 1.32x faster | 0.54355 ms | 1.43x slower |

Same-process release A/B vs HGW on rch `hz2`:

| Workload | HGW | Queue | Ratio |
| --- | ---: | ---: | ---: |
| max size=31 | 1342.8 us | 1163.6 us | 1.15x faster |
| min size=31 | 1061.4 us | 903.8 us | 1.17x faster |
| max size=101 | 1105.4 us | 915.4 us | 1.21x faster |
| min size=101 | 1096.8 us | 910.1 us | 1.21x faster |

## Gates

- `cargo check -p fsci-ndimage --all-targets`: PASS via rch `hz2`; unrelated
  existing warnings remain in `fsci-interpolate` and `diff_geom`.
- `cargo clippy -p fsci-ndimage --all-targets -- -D warnings`: BLOCKED before
  this patch in `fsci-linalg` (`needless_range_loop`, `needless_borrow`).
- `rustfmt --edition 2024 --check crates/fsci-ndimage/src/lib.rs`: PASS.
- `git diff --check`: PASS.
- `ubs crates/fsci-ndimage/src/lib.rs`: PASS exit 0, 0 critical issues; broad
  existing warning inventory remains.

## Next Route

The residual is no longer dominated by HGW's three full scan buffers, but SciPy
still wins three rows and is slightly faster on max/31. Next attempts should
target the remaining queue overhead: boundary-free interior specialization for
`Reflect` long contiguous lines, branch-reduced NaN-free fast paths with a
guarded fallback, or portable-SIMD monotonic-block merging only if it preserves
signed-zero tie semantics.
