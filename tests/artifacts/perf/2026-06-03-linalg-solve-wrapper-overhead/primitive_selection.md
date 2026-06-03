# Primitive selection: DMatrix build + 1-norm fusion

Bead: `frankenscipy-8l8r1.15`

## Profile-backed target

RCH Criterion pass 1 measured `baseline_solve/1000x1000` at
`[559.20 ms, 577.33 ms, 599.73 ms]`. The solve diagnostics path builds a
`DMatrix` for the general square case and then separately scans that same matrix
with `matrix_norm1(&matrix)` before the LU-based reciprocal condition estimate.

## One lever

Add `dmatrix_from_rows_with_norm1(rows)` beside `dmatrix_from_rows`, and use it
only in `condition_diagnostics_with_assumption_mode`'s general matrix branch.
The helper:

- preserves `matrix_shape(rows)?` validation;
- pushes row-major input values into the exact same `Vec<f64>` used by
  `DMatrix::from_row_slice`;
- accumulates per-column absolute sums while copying those values;
- returns `NaN` for the norm if any absolute value is non-finite, matching
  `matrix_norm1`'s non-finite behavior;
- leaves all other `dmatrix_from_rows` call sites unchanged.

## Isomorphism obligations

- Ordering preserved: yes. Validation order and row-major DMatrix data order are
  unchanged.
- Floating-point: each column sum sees row values in the same increasing row
  order as `matrix_norm1`; independent column accumulators are merely
  interleaved with the row-major copy.
- Tie-breaking: N/A; diagnostics have no tie surface before portfolio expected
  losses.
- RNG: N/A.
- Golden proof: before SHA256 is
  `d74fb19f841425c0ec647658e54c0b23a7e503a9d07487563ccbdfb33c24e89f`; after
  must match or source is restored.

## Score target

`2.0 = impact 1 * confidence 4 / effort 2`. This is a small allocation/scanning
lever; keep only if the focused RCH benchmark shows a real win and golden output
is unchanged.
