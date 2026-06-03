# frankenscipy-e3713 conclusion

## Verdict

Kept. The one production lever changes the large flat-workspace GEMM traversal
from row-tile outer to B-panel outer.

## Baseline and after

Fresh pre-edit RCH Criterion baseline on `vmi1227854`:

| row | before median |
| --- | ---: |
| `matmul/512x512` | `38.543 ms` |
| `matmul/768x768` | `129.09 ms` |
| `matmul/1024x1024` | `421.95 ms` |

Focused after RCH Criterion on `vmi1293453`:

| row | after median |
| --- | ---: |
| `matmul/512x512` | `39.942 ms` |
| `matmul/768x768` | `139.92 ms` |
| `matmul/1024x1024` | `372.14 ms` |

The production lever is gated to `m >= 1024`, `k >= 1024`, and `n >= 1024`, so
the `512` and `768` rows remain below the optimized path. The keep row is
`1024x1024`.

Comparable same-worker signal: the pre-edit broad reprofile on `vmi1293453`
measured `matmul/1024x1024` at `504.32 ms`; the focused after run on the same
worker measured `372.14 ms` (`1.36x`). The fresh baseline comparison is
`421.95 ms -> 372.14 ms` (`1.13x`) across workers.

Score: `2.4 = impact 2 * confidence 3 / effort 2.5`.

## Isomorphism proof

- Ordering preserved: yes. Public output remains row-major `Vec<Vec<f64>>`.
- Tie-breaking unchanged: yes. GEMM has no tie-breaking surface.
- Floating-point preserved: yes. Each output cell still accumulates `k = 0..ka`
  monotonically with separate `acc += a * b` updates.
- RNG preserved: N/A. No RNG surface exists.
- Golden tests: before and after sorted test-line SHA-256 both
  `61e12eb58f34ccba1dcedd29425ff3292fd7df5769f7411352cd2a617a58d6c7`.

## Gates

- `cargo fmt -p fsci-linalg --check`: pass.
- `ubs crates/fsci-linalg/src/lib.rs`: critical `0`.
- RCH `cargo test -p fsci-linalg --release --locked matmul -- --nocapture`:
  pass on `vmi1149989`.
- RCH `cargo check -p fsci-linalg --all-targets --locked`: pass on
  `vmi1149989`.
- RCH `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`:
  pass on `vmi1153651`.

## Reprofile

RCH linalg reprofile on `vmi1149989` was noisy across unrelated rows but still
ranked `matmul/1024x1024` first at median `571.38 ms`, followed by
`baseline_solve/1000x1000` at `380.01 ms`, `matmul/768x768` at `226.34 ms`,
`lstsq/512x256` at `125.76 ms`, and `pinv/512x256` at `118.70 ms`.

Next target: a deeper GEMM primitive again, likely recursive/cache-oblivious
blocking or a packed-panel kernel with same-worker comparator discipline.
