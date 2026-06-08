# Pass 2 Compact-WY Primitive Spec

Bead: `frankenscipy-8l8r1.53`

## Selected Primitive

Use a private compact-WY Householder-panel primitive for the far trailing
update. Keep deterministic scalar reflector construction from
`make_householder_reflector`, but accumulate a left-reflector panel as:

```text
Q_panel = I - V T V^T
```

Then apply the far trailing update as:

```text
W = V^T A22
Z = T W
A22 -= V Z
```

This is the compact-WY class of update the `.53` bead asks for. It replaces
the rejected `.52` full-rectangle verified-delta replay with narrow panel state
and cache-blocked safe-Rust matrix updates.

## Mapping To Current Code

- `make_householder_reflector`: remains the only way to construct reflectors.
- `apply_householder_left`: remains the scalar reference for proof.
- `apply_householder_right_with_workspace`: remains the right-reflector path for
  this pass.
- `golub_kahan_bidiagonal_reduction`: must not be routed through this primitive
  until Stage 1 plus Stage 2 proof clears.
- `apply_bidiag_fused_rank_k_update`: remains a loop-shape reference only; do
  not repeat full-delta or scalar DLABRD replay.

## Pass 3 Boundary

Implement exactly one private helper family in `crates/fsci-linalg/src/lib.rs`:

- A private compact-WY panel struct storing `row_start`, `row_count`,
  `panel_width`, packed `V` in column-major `k x row_count` layout, and dense
  triangular `T`.
- A private builder that takes existing `HouseholderReflector` values and forms
  `V` and `T` for one left-reflector panel.
- A private cache-blocked updater that applies `A22 -= V T (V^T A22)` to a
  far trailing matrix region using deterministic column-major loops.
- Focused tests that compare the compact-WY helper against sequential
  `apply_householder_left` on deterministic inputs within a strict tolerance and
  prove determinism for fixed input.
- One ignored perf probe comparing sequential panel application versus compact
  WY on a 1024x512-style far rectangle.

Out of scope:

- No public `svd`, `svdvals`, `lstsq`, `pinv`, CASP, or
  `deterministic_thin_svd` route switch.
- No Stage 2 band-to-bidiagonal bulge chase.
- No full-rectangle delta replay.
- No dense compact-WY public route composition.
- No thread fanout, rayon, async, unsafe, C BLAS, MKL, or XLA.

## Storage And Loop Shape

Let:

- `m = row_count`
- `n = col_count`
- `k = panel_width`
- `V` stored as `k` contiguous reflector columns, each length `m`:
  `v_by_k_row[k_idx * m + row_rel]`
- `T` stored row-major `k x k`: `t[row * k + col]`
- `A22` is the region `matrix[row_start.., col_start..]`, with `DMatrix`
  column-major backing.

For a column block of `A22`:

1. Compute `W = V^T A_block` with loops `col -> k -> row` so each dot
   accumulates rows in increasing order.
2. Compute `Z = T W` with loops `col -> t_row -> t_col`.
3. Update `A_block -= V Z` with loops `col -> row -> k`.

The loops are fixed and single-threaded. The first proof target is tolerance
equivalence to sequential reflector application, not bit identity.

## Behavior Contract

- Ordering: public ordering is unchanged because the helper remains private.
- Tie-breaking: singular-value order, rank thresholds, route gates, and sign
  canonicalization are untouched.
- Floating point: private helper may use a different summation order; tests must
  prove strict reconstruction/tolerance equivalence against sequential
  `apply_householder_left`. Public golden SHA remains unchanged because no
  public route uses the helper.
- RNG: unchanged; no RNG is used.
- Public golden SHA:
  `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.

## Opportunity Score

Candidate: true compact-WY left-panel far update.

- Impact: `5`
- Confidence: `3`
- Effort: `5`
- Score: `3.0`

Target for eventual route: at least `1.35x` on the 1024x512 reduction probe
against the same-worker `vmi1153651` baseline `431.652279 ms`.

Pass 3 private-kernel target: compact-WY panel application must beat sequential
left-reflector panel application on the same RCH worker, or it is not a credible
Stage 1 primitive.

## Rejection Criteria

Reject and restore source if:

- Any public route changes.
- The helper repeats full-rectangle verified-delta replay.
- The helper fails deterministic/tolerance proof versus sequential reference.
- Public golden SHA changes.
- Same-worker RCH shows no credible speedup for the private panel kernel.
- The code introduces unsafe, external BLAS, or thread fanout.
