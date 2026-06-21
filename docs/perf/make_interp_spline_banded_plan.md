# Plan: compact-banded `make_interp_spline` (close the remaining ~120x scipy loss)

Status: DESIGN (authored under a disk-low / no-build window — implement + verify
when benches resume). Owner: cc / MistyBirch.

## Why
`make_interp_spline(x, y, k)` (crates/fsci-interpolate/src/lib.rs, ~line 1766) is
**29-175x slower than scipy** and the gap grows O(n²) (measured k=3: n=1000 6.81ms
vs scipy 0.23ms; n=3000 84.26ms vs 0.48ms). The byte-identical `solve_banded`
switch (commit 318898bb) took it to 1.45x (n=3000 → 58ms); the **build** is now the
O(n²) bottleneck. scipy is O(n·k) (banded throughout). Three O(n²) sources remain:

1. `let mut a_mat = vec![vec![0.0; n]; n];` — dense n×n alloc/zero (~72MB at n=3000).
2. `eval_basis_all(&t, x[i], k, n)` per row does a **degree-0 interval LINEAR SCAN**
   `for i in 0..n { if t[i] <= x < t[i+1] ... }` (O(n)) AND allocates a length-n
   `Vec` (O(n)); called n times → O(n²).
3. `a_mat[i][..n].copy_from_slice(&basis[..n])` — O(n) per row.

## Target
Match scipy's O(n·k): no n×n storage, no per-row O(n) scan/alloc.

## Implementation

### 1. Byte-exact binary-search interval finder
Replace the degree-0 linear scan with a binary search that returns the SAME index
`μ` the scan finds (the half-open interval `t[μ] <= x < t[μ+1]`, with the existing
right-endpoint special case `x == t[μ+1] && μ+1 == t.len()-k-1`). Use
`t.partition_point(|&ti| ti <= x) - 1` then walk back over any repeated knots so the
chosen μ is the FIRST index satisfying the scan's predicate (the scan sets the
lowest such i as `lo`; de-Boor uses `lo`). VERIFY byte-exactly with a test that, for
random sorted x and knot vectors (incl. repeated interior knots + both endpoints),
the binary-search μ equals the linear-scan `lo` for every site. This alone removes
source (2)'s O(n) scan (byte-exact) but not the allocs.

### 2. Compact basis eval (k+1 values + offset)
Factor the de-Boor recursion in `eval_basis_all` into `eval_basis_compact(t, x, k, μ)
-> [f64; K1]` (K1 = k+1) operating on a small stack/`Vec` of length k+1 (indices
μ-k..μ), returning the k+1 nonzero B-spline values + the column offset `μ-k`. The
arithmetic/op-order must match the current in-place ascending sweep so values are
bit-identical. No length-n alloc.

### 3. Compact banded storage + LU solve (the real win)
Collocation row i has its k+1 nonzeros at columns `[off_i, off_i+k]` (off_i from the
interval finder). Build LAPACK-style band storage `ab` of shape `(2*kl+ku+1) × n`
with `kl = ku = k` (partial pivoting fills the upper band to `2k`, so allocate
`ku_eff = 2k`; total bands `kl + ku_eff + 1 = 3k+1`). Store `A[i][j]` at
`ab[kl + ku_eff + i - j][j]` (column-major band, the dgbsv convention) or a simpler
row-band `band[i][j-off_i]` + per-row offset — pick whichever makes the solver
indexing cleanest.

Port a banded LU with partial pivoting (dgbtrf/dgbtrs-style) operating ONLY within
the band. It must reproduce `solve_banded`'s elimination/pivot ORDER so the result
is byte-identical to today's (already scipy-parity) output:
- pivot search over rows `[col, col+kl]`,
- row swap within the band window,
- elimination of rows `[col+1, col+kl]`, updating columns `[col, col+ku_eff]`,
- back-substitution over the band.
`solve_banded` (crates/fsci-interpolate/src/lib.rs) is the byte-identical reference
to diff against on dense-expanded small systems.

## Verification (when disk recovers)
- New unit test: `make_interp_spline` compact-band output == the current dense path
  (`solve_banded` on the dense build) to_bits / ≤1e-12, for k∈{1,2,3,5}, n∈{8,50,200},
  random + clustered sites, repeated interior knots.
- Existing fsci-interpolate suite must stay 172/0 (scipy-parity tests included).
- Bench `make_interp_spline/k3` (already added, commit d049502d): expect n=3000
  58ms → ~1-2ms (O(n·k)), i.e. parity-ish with scipy 0.48ms (closes the loss).

## Gotchas
- The flat-dense lever (used for RBF, commit 17e29927) does NOT apply here: dropping
  the zero-skip on a banded matrix is O(n³). Banded storage is mandatory.
- Keep `solve_banded` / `solve_dense_system` for the other callers (smoothing/lsq
  splines use solve_banded on dense storage; RBF uses solve_dense_system_flat).
- Pivoting can be skipped IF the collocation is provably totally-positive
  (Schoenberg-Whitney), but scipy uses general gbsv — keep pivoting to stay
  byte-identical to the current (parity-verified) result.
