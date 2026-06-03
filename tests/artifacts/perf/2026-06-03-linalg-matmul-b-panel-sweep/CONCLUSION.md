# frankenscipy-e3713 conclusion

## Lever kept

Changed exactly one production lever in `matmul_flat_workspace`: the large
rectangular flat-workspace path now sweeps `j0` B-panels outside `i0` row tiles.
The dispatch gate is unchanged (`m`, `ka`, and `n` all at least 1024,
rectangular inputs only).

## Behavior proof

- Ordering preserved: public output is still materialized row-major from
  `c_flat`.
- Tie-breaking unchanged: GEMM has no tie-breaking surface.
- Floating-point preserved: each `c[i][j]` still accumulates `k = 0..ka`
  monotonically with the same separate `acc += a * b` updates.
- RNG unchanged: no RNG surface exists.
- Golden output: before/after sorted test-result SHA-256 stayed
  `61e12eb58f34ccba1dcedd29425ff3292fd7df5769f7411352cd2a617a58d6c7`;
  `cmp` of the sorted test-result goldens exited `0`.

The raw sorted RCH-log normalization files differ only in volatile timestamp and
test scheduling order. The stable sorted test-result golden is identical.

## Performance

Focused RCH Criterion baseline:

| row | before | after | ratio |
| --- | ---: | ---: | ---: |
| `matmul/1024x1024` | `421.95 ms` | `375.15 ms` | `1.12x` |

Supporting same-worker profile row on `vmi1293453`:

| row | before profile | after focused | ratio |
| --- | ---: | ---: | ---: |
| `matmul/1024x1024` | `504.32 ms` | `375.15 ms` | `1.34x` |

Keep score: `2.5 = impact 2 * confidence 2.5 / effort 2`.

## Validation

- `cargo fmt -p fsci-linalg --check`: passed.
- `ubs crates/fsci-linalg/src/lib.rs`: exit `0`, critical count `0`.
- RCH `cargo check -p fsci-linalg --all-targets --locked`: passed.
- RCH `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`: passed.
- RCH release matmul golden/isomorphism tests: passed.

## Reprofile

RCH linalg reprofile on `vmi1149989` still ranks deep GEMM first:

| rank | row | median |
| ---: | --- | ---: |
| 1 | `matmul/1024x1024` | `571.38 ms` |
| 2 | `matmul/768x768` | `226.34 ms` |
| 3 | `baseline_solve/1000x1000` | `171.38 ms` |
| 4 | `lstsq/512x256` | `125.76 ms` |
| 5 | `pinv/512x256` | `118.70 ms` |

Next target should remain a deeper dense-GEMM primitive, not a solve/lstsq
handoff.
