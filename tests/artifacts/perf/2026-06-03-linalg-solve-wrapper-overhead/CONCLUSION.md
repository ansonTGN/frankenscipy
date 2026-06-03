# Solve diagnostics norm-fusion conclusion

Bead: `frankenscipy-8l8r1.15`

## Lever

One lever was kept: fuse `DMatrix` construction with induced 1-norm
calculation in the general solve diagnostics branch. The new helper preserves
the row-major `DMatrix::from_row_slice` data and computes each column sum in
increasing row order while the row data is copied.

## Baseline

RCH Criterion baseline on `vmi1153651`:

- `baseline_solve/1000x1000`: `[559.20 ms, 577.33 ms, 599.73 ms]`

## After

RCH Criterion after on `vmi1293453`:

- `baseline_solve/1000x1000`: `[105.08 ms, 106.08 ms, 107.15 ms]`

Median speedup: `577.33 / 106.08 = 5.45x`.

## Isomorphism proof

- Ordering preserved: validation order, row-major matrix data order, LU
  factorization input, solve output order, warnings, certificates, and
  backward-error surfaces are unchanged.
- Tie-breaking unchanged: no new tie surface.
- Floating-point: each column sum follows the same increasing-row accumulation
  order as `matrix_norm1`; non-finite inputs still produce `NaN`.
- RNG: N/A.
- Golden output: sorted solve-case SHA256 matched before/after at
  `004ce6132222a051085017a29591c730f42b16e1400a746ae653210145cbe228`.

## Validation

- `cargo fmt -p fsci-linalg --check`: passed.
- RCH `cargo test -p fsci-linalg --release solve --locked -- --nocapture`:
  passed.
- RCH `cargo check -p fsci-linalg --all-targets --locked`: passed.
- RCH `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings`:
  passed.

## Score

`12.5 = impact 5 * confidence 5 / effort 2`.

Verdict: kept.
