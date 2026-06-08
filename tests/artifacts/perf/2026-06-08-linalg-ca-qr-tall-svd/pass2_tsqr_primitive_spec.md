# Pass 2 TSQR Primitive Specification

Bead: `frankenscipy-8l8r1.54`

## Baseline Context

This pass targets the profile-backed tall public SVD lane:

- RCH `vmi1153651` criterion baseline:
  - `lstsq/512x256=[174.74 ms 191.04 ms 209.93 ms]`
  - `pinv/512x256=[233.44 ms 267.15 ms 307.79 ms]`
- RCH `vmi1167313` public route probe:
  - `lstsq reference 127.906224 ms -> routed 117.018190 ms`, `1.093046x`
  - `pinv reference 140.152702 ms -> routed 123.589790 ms`, `1.134015x`
  - rank `256`, max diffs `<= 1.0765e-12` for `lstsq`, `<= 2.2843e-14` for `pinv`
- RCH `vmi1167313` 1024x512 reducer probe:
  - `348.394210 ms`
  - digest `0x90cdd3f8f71ed2c1`
- Public golden SHA:
  - `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`

The bottleneck is the full tall Golub-Kahan path feeding `public_bidiag_thin_svd_candidate`.
For tall full-rank matrices, the algebraic route can shrink the problem first:

```text
A in R^(m x n), m >= 2n
A = Q R                 thin QR
R = U_r Sigma V^T       small n x n SVD
A = (Q U_r) Sigma V^T   thin SVD
```

## Chosen Primitive

Chosen primitive: sequential communication-avoiding QR, specifically TSQR for tall-skinny matrices,
used as a guarded candidate source for the public tall SVD route.

This is fundamentally different from compact-WY and packed-panel attempts:

- Compact-WY/packed-panel work still attacks the same Golub-Kahan bidiagonalization family. It tries
  to update the tall trailing matrix faster while preserving the same reduction shape.
- TSQR changes the decomposition graph. It first reduces `m x n` to a cache-resident stack of `n x n`
  `R` factors, then computes SVD on the final `n x n` core. The tall dimension participates mostly in
  independent block QR steps and one deterministic replay pass for `Q U_r`.
- The communication-avoiding idea from the graveyard is the core lever: local QR per row block, stack
  the `R` factors up a fixed tree, then reconstruct by replaying reflectors down the tree. This reduces
  data movement through the tall matrix and moves expensive work to BLAS-3-shaped small dense kernels.
- The implementation remains safe Rust only: no C BLAS, MKL, XLA, unsafe code, or thread fanout.

Alien-artifact proof lens: this is a numerically orthodox factorization identity with explicit
orthogonality, reconstruction, rank, spectrum, and public golden-output obligations. It is not a
heuristic approximation and must reject on ambiguous spectra or unstable factors.

## Exact Pass 3 One-Lever Boundary

Implement exactly one lever in `crates/fsci-linalg/src/lib.rs`:

```text
Add a private TSQR-backed tall thin-SVD candidate source and route
`public_bidiag_thin_svd_candidate` to try it before falling back to the current
Golub-Kahan candidate, under strict shape/rank/spectrum/acceptance guards.
```

Allowed Pass 3 implementation surface:

- Add private structs/functions only, for example:
  - `TsqrBlockFactor`
  - `TsqrTreeNode`
  - `tsqr_tall_thin_qr(matrix: &DMatrix<f64>, block_rows: usize) -> Option<TsqrQr>`
  - `tsqr_thin_svd_candidate(matrix: &DMatrix<f64>) -> Option<DeterministicThinSvd>`
- Reuse existing deterministic scalar Householder construction where possible.
- Compute SVD only on the final `n x n` `R` core using the existing safe deterministic SVD machinery.
- Form public candidate factors:
  - `singular_values = svd(R).singular_values`
  - `v_t = svd(R).v_t`
  - `u = apply_tsqr_q_to_dense(&tsqr, svd(R).u)`
- Change `public_bidiag_thin_svd_candidate` only to:
  1. check current public route shape gate,
  2. try the TSQR candidate,
  3. return it only if TSQR-specific preguards pass,
  4. otherwise run the existing `deterministic_thin_svd(matrix).ok()` fallback.
- Add focused tests and ignored probes in the same file.

Forbidden in Pass 3:

- No changes to public API shapes or result structs.
- No changes to `public_bidiag_svd_accepts` except possibly adding a private wrapper that calls it.
- No randomized SVD, power iteration, mixed precision, pivoted QR, or low-rank truncation in this lever.
- No parallelism, rayon, unsafe, C/Fortran BLAS/LAPACK, MKL, XLA, or external kernel dependencies.
- No public route retention without public golden SHA equality and same-worker RCH win.

## Storage, Layout, And Deterministic Loop Shapes

Use a fixed sequential TSQR tree to make the route deterministic and auditable.

### Blocking

- Shape gate: `rows >= 2 * cols`, `cols >= PUBLIC_BIDIAG_SVD_MIN_COLS`.
- Preferred block height: deterministic function of `cols`, for example `block_rows = max(2 * cols, 128)`
  rounded to a whole row count without hardware introspection.
- Each leaf owns a contiguous row range `[row_start, row_end)`.
- Leaf matrix is copied into a compact column-major `DMatrix<f64>` block with stable row-major outer loops:

```text
for local_row in 0..leaf_rows:
  for col in 0..cols:
    block[(local_row, col)] = matrix[(row_start + local_row, col)]
```

### Leaf QR

- Factor each leaf with deterministic unpivoted Householder QR.
- Store:
  - `row_start`, `row_end`
  - local reflector metadata needed to apply `Q_leaf` to dense right-hand blocks
  - leaf `R` as an `n x n` dense matrix
- Normalize `R` diagonal signs with a deterministic policy:
  - if `R[(j,j)] < 0`, multiply row `j` of `R` and the corresponding implicit `Q` column by `-1`;
  - if `R[(j,j)] == 0` or non-finite, reject.

### Tree Reduction

Use a fixed left-to-right binary tree over leaf `R` factors:

```text
level = leaves
while level.len() > 1:
  next = []
  for pair_start in (0..level.len()).step_by(2):
    if pair exists:
      stacked = [R_left; R_right]      // 2n x n
      factor stacked with the same unpivoted Householder QR
      store internal reflectors and child indices
      next.push(R_parent)
    else:
      next.push(singleton)
  level = next
```

No balancing based on worker count, cache size, timings, or input values. The same input shape must always
produce the same tree.

### Small Core SVD

- Compute SVD of root `R`.
- Reject if the small SVD returns non-finite values, wrong dimensions, unordered singular values, tied
  spectrum by the public gap floor, or rank below `cols`.
- Do not reinterpret singular values or reorder manually unless using the same deterministic order/sign
  policy as the existing deterministic SVD path.

### Reconstructing `U = Q U_r`

Apply TSQR `Q` to the small `U_r` by replaying the tree downwards:

```text
input at root: U_r, shape n x n
for each internal node, top-down:
  apply stored node Householder Q to the node input, producing 2n x n
  split rows to left and right children
for each leaf:
  apply stored leaf Householder Q to the leaf input, producing leaf_rows x n
  scatter into final U rows [row_start, row_end)
```

Loop order must be deterministic:

- tree traversal: root to leaves, left child before right child;
- reflector replay: same order as mathematical `Q` application requires, no data-dependent branching except
  documented reject paths;
- dense updates: fixed `row`, `col`, `k` loop order, no associative regrouping that depends on shape beyond
  the fixed block/tree rules.

## Guard Conditions For Public Routing

TSQR may become a public candidate source only under all of these guards.

### Shape And Input Guards

- `rows >= 2 * cols`.
- `cols >= PUBLIC_BIDIAG_SVD_MIN_COLS`.
- `cols <=` a conservative first-pass cap if needed for implementation risk, but the cap must be explicit
  and recorded in the perf artifact.
- All inputs finite.
- `rows`, `cols`, and block count fit allocation limits without integer overflow.

### Rank And Spectrum Guards

- Root `R` diagonal entries finite and nonzero after sign normalization.
- `public_bidiag_svd_stats` succeeds.
- `min_s > max(threshold, max_s * PUBLIC_BIDIAG_RANK_GAP_REL_TOL)`.
- Every adjacent singular-value gap satisfies the existing public tie floor:

```text
s[i] >= s[i + 1]
s[i] - s[i + 1] > max_s * PUBLIC_BIDIAG_RANK_GAP_REL_TOL
```

- Full rank for the public route: computed rank equals `cols` for the route to return. Rank-deficient and
  clustered-spectrum matrices must fall back to the current path.

### Reconstruction And Orthogonality Guards

The existing `public_bidiag_svd_accepts(matrix, thin, threshold)` remains the final gate. TSQR-specific
proof tests must also measure and report:

- `max_abs(A - U Sigma V^T) <= PUBLIC_BIDIAG_RECON_REL_TOL * max_abs(A).max(1.0) * sqrt(cols)`.
- `||U^T U - I||_max <= 1e-9` for proof probes.
- `||V V^T - I||_max <= 1e-9` for proof probes.
- `U.nrows() == rows`, `U.ncols() == cols`, `Vt.nrows() == cols`, `Vt.ncols() == cols`.

### Ordering, Sign, And Tie Handling

- Singular values must be descending with no ambiguous ties under the public tie floor.
- If the small-core SVD sign choices differ internally, public observables may still pass only through
  reconstruction, least-squares solution, pseudo-inverse, and golden SHA checks.
- For deterministic proof probes, compute `thin_svd_bits_digest` twice on fixed input and require equality.
- Do not add ad hoc sign flips to chase a golden hash. Sign normalization is allowed only where it follows
  from QR diagonal normalization and is applied consistently to both `Q` and `R`.

### Golden SHA Obligations

Before retaining the route:

- Run the existing public golden test under RCH:

```text
cargo test -p fsci-linalg --release --lib --locked public_svd_lstsq_pinv_golden_payload -- --nocapture
```

- Extract the payload and verify SHA exactly:

```text
1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
```

- Run the public route perf probe and criterion benchmark on the same workers as the baseline where possible.
- If the golden SHA changes, reject even if numerical tolerances pass.

## Proof Obligations For Pass 3

### Isomorphism Proof

- Ordering preserved: singular values are accepted only when strictly descending under the existing tie floor.
- Tie-breaking unchanged: clustered spectra, rank-boundary cases, and rank-deficient inputs reject to fallback.
- Floating point: TSQR changes internal summation and factorization order, so bit identity is not promised for
  private factors. Public behavior is guarded by reconstruction, orthogonality, route parity, and exact golden SHA.
- RNG: unchanged; no randomized algorithm is allowed.
- Public output: `svd`, `svdvals`, `lstsq`, and `pinv` must preserve the golden payload SHA.

### Focused Tests

Pass 3 should add tests equivalent to:

- TSQR candidate reconstructs deterministic 128x64 tall matrix and passes `public_bidiag_svd_accepts`.
- TSQR candidate is deterministic for fixed 512x256 or 1024x512 input via `thin_svd_bits_digest`.
- TSQR route matches `safe_svd` reference for public `svd`, `lstsq`, and `pinv` within the existing route
  tolerances.
- Rank-deficient and clustered-spectrum matrices reject to fallback.
- Non-finite input rejects.

### RCH Measurement

Keep/reject must be decided by same-worker RCH evidence:

- Baseline before source change, then after source change, for `lstsq/512x256|pinv/512x256`.
- Public route probe before and after.
- Golden SHA after.
- Crate-scoped build/test only.

## Rejection Criteria

Reject and restore source by manual patch if any condition holds:

- The implementation changes public API, public result shapes, thresholds, or CASP certificate semantics.
- The route returns for rank-deficient, clustered-spectrum, non-finite, or below-shape-gate matrices.
- `public_bidiag_svd_accepts` fails for any routed candidate.
- Public golden SHA differs from `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
- Same-worker RCH speedup is below the keep target or cannot be measured.
- Criterion or route probes regress either `lstsq` or `pinv` enough to make the combined Score below `2.0`.
- The code introduces unsafe blocks, thread fanout, nondeterministic scheduling, BLAS/LAPACK linkage, or data-dependent
  tree shapes.
- Memory growth is unbounded relative to the deterministic tree storage. A first-pass implementation may allocate
  factor records, but peak storage must be explainable as `O(mn)` for final `U` plus `O(blocks * n^2)` tree/core data.

## Opportunity Score And Target Ratio

Candidate: CA-QR/TSQR-first tall full-rank SVD route.

| Factor | Score | Reason |
|---|---:|---|
| Impact | 5 | Replaces tall Golub-Kahan reduction with a communication-avoiding QR shrink plus small-core SVD for the profiled tall route. |
| Confidence | 3 | Algebra is standard and proofable, but strict golden behavior and factor replay are nontrivial. |
| Effort | 5 | Requires a deterministic TSQR tree, factor replay, guarded public route, and same-worker proof bundle. |
| Opportunity | 3.0 | `(5 * 3) / 5`, above the `>=2.0` gate. |

Target ratio for retaining Pass 3:

- Minimum public keep target: `>=1.25x` on both profile-backed public lanes where measured on the same worker, or
  a combined criterion improvement that keeps Score `>=2.0` with no golden or parity regression.
- Stretch target: `>=1.35x` on the 1024x512 reducer-equivalent path by avoiding the full tall Golub-Kahan reduction.

If Pass 3 fails the keep target, the next deeper primitive should remain algorithmic rather than a TSQR micro-tune:
blocked rank-revealing QR for guarded ill-conditioned tall matrices, or a communication-avoiding bidiagonalization
route that forms blocked reflectors and applies BLAS-3 safe-Rust kernels with explicit stability certificates.
