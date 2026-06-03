# frankenscipy-8l8r1.19 conclusion

## Verdict

Kept. The one production lever is a gated flat row-major workspace path for
large rectangular GEMM in `fsci_linalg::matmul`.

## Baseline and after

Focused same-worker RCH Criterion on `vmi1149989`:

| row | before median | after median | ratio |
| --- | ---: | ---: | ---: |
| `matmul/256x256` | `7.0222 ms` | `8.8050 ms` | `0.80x` |
| `matmul/512x512` | `39.686 ms` | `37.938 ms` | `1.05x` |
| `matmul/768x768` | `144.63 ms` | `138.49 ms` | `1.04x` |
| `matmul/1024x1024` | `534.93 ms` | `384.22 ms` | `1.39x` |

The optimized path is gated at `m >= 1024`, `k >= 1024`, and `n >= 1024`, so
the below-gate `256`, `512`, and `768` rows remain on the existing kernel in
production. The `1024` profile row is the keep-gate row.

Score: `2.33 = impact 2 * confidence 3.5 / effort 3`.

## Isomorphism proof

- Ordering preserved: yes. Public output remains row-major `Vec<Vec<f64>>`.
- Tie-breaking unchanged: yes. GEMM has no tie-breaking surface.
- Floating-point preserved: yes. Each output cell still accumulates `k = 0..ka`
  monotonically with separate `acc += a * b` updates.
- RNG preserved: N/A. No RNG surface exists.
- Golden outputs: before and after normalized SHA-256 both
  `22e5439c63033319e9331aa38a6068d2bfded3950eac7e9dc37e7dc103852477`.
- Risk note: the large-matrix path allocates flat `A`, `B`, and `C` slabs before
  materializing public rows. The `1024` gate confines that memory tradeoff to
  the profile-driving large rectangular case.

## Gates

- `cargo fmt -p fsci-linalg --check`: pass.
- `ubs crates/fsci-linalg/src/lib.rs`: critical `0`.
- RCH `cargo test -p fsci-linalg --release --locked matmul -- --nocapture`:
  pass on `vmi1227854`.
- RCH `cargo check -p fsci-linalg --all-targets --locked`: pass on
  `vmi1149989`.
- RCH `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`:
  pass on `vmi1149989`.

## Reprofile

RCH linalg reprofile on `vmi1293453` after the kept change still ranks
`matmul/1024x1024` first at median `504.32 ms`, followed by
`lstsq/512x256` at `139.32 ms`, `matmul/768x768` at `132.46 ms`,
`pinv/512x256` at `120.58 ms`, and `baseline_solve/1000x1000` at
`105.40 ms`.

Next target: a deeper GEMM primitive rather than another tile-width retry.
