# perf: nnls (non-negative least squares) — precompute Gram matrix once

## Lever (ONE)
The Lawson-Hanson active-set loop in `fsci_opt::nnls` rebuilt the passive-set
normal equations from scratch on **every inner solve**:

```rust
for (pi, &ji) in passive_indices.iter().enumerate() {
    for (pj, &jj) in passive_indices.iter().enumerate() {
        for i in 0..m { ata[pi][pj] += a[i][ji] * a[i][jj]; }  // O(p²·m)
    }
    for i in 0..m { atb_sub[pi] += a[i][ji] * b[i]; }          // O(p·m)
}
```

The inner solve runs many times (the outer active-set loop runs up to `3n`
times, and the inner constraint loop iterates as variables are pushed back to the
active set), so this `Σ_i` is recomputed over and over for the same column pairs.

Replace with a ONE-TIME precompute of the full Gram matrix `G = AᵀA` and `Aᵀb`,
then GATHER the passive submatrix each iteration:

```rust
// once, before the active-set loop
for j1 in 0..n { for j2 in 0..n {
    let mut acc = 0.0;
    for row in a.iter() { acc += row[j1] * row[j2]; }   // same `for i in 0..m` order
    gram[j1][j2] = acc;
}}
// per inner solve
ata[pi][pj] = gram[ji][jj];
atb_sub[pi] = atb[ji];
```

## Isomorphism / parity proof — BIT-IDENTICAL
- `gram[ji][jj]` accumulates `Σ_i a[i][ji]·a[i][jj]` starting from `0.0` in the
  exact `for i in 0..m` order the original inner loop used → identical bit
  pattern to the value the original wrote into `ata[pi][pj]`.
- `atb[ji]` likewise equals the original `atb_sub[pi]`.
- The downstream `solve_small_system` therefore receives bit-identical inputs, so
  `x` and the residual are bit-identical. (Gram is symmetric, but every needed
  entry is the exact value the original would have computed, regardless.)
- Golden `(x-hash, residual bits)` are IDENTICAL between the pre-change baseline
  build and the precompute-gram build for (m,n,seed) ∈
  {(20,8,1),(60,20,2),(200,40,3)}. See `golden_payload.txt`.
  sha256 = 18fed7790d4e368bcb3b01d4f887dd9c3ac30b46e3c411af145764d09fac0e65
  (same sha for BOTH builds).

## Timing — rch remote, 64 cores, `--profile release-perf`, reps=5
Same machine, back-to-back (baseline measured by stashing the change).

| m × n      | baseline   | precompute-gram | speedup |
|------------|------------|-----------------|---------|
| 2000 × 80  | 1.111 s    | 95.29 ms        | 11.7x   |
| 4000 × 120 | 7.411 s    | 440.2 ms        | 16.8x   |
| 8000 × 160 | 49.139 s   | 1.867 s         | 26.3x   |

Score ≥ 2.0 cleared with large margin; the win grows with problem size because
the eliminated recompute cost scales with (#inner-iterations × p² × m).

Harness: `crates/fsci-opt/src/bin/perf_nnls.rs`
Run: `cargo run --profile release-perf -p fsci-opt --bin perf_nnls`

## Notes
- The one-time precompute is O(n²·m), the same order as the gradient pass already
  performed on every outer step, so it adds no new dominant term.
- Conformance (`diff_opt_nnls_isotonic_lsa`) could not run on the rch worker
  (no numpy/scipy installed there); bit-identity to the previously-conforming
  serial output is the parity guarantee. Clippy clean on `fsci-opt`.
