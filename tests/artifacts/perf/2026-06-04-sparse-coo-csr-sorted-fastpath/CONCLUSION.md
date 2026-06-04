# Sparse Sorted COO-to-CSR Fast Path Evidence

Bead: `frankenscipy-egndo`

Target: `sparse_csr_construction/10000x10000_d0/10000`, selected from the post-sort sparse reprofile where it remained the top non-owned sparse hotspot at median `4.7033 ms`.

## One Lever

`CooMatrix::to_csr` now detects strictly row-major unique COO input and compresses it directly into CSR by row counts. Unsorted or duplicate-containing COO falls back to the existing `canonical_triplets` path.

## Benchmarks

All benchmark commands were crate-scoped and run through RCH.

- Baseline: `cargo bench -p fsci-sparse --bench sparse_bench --locked -- sparse_csr_construction/10000x10000_d0/10000 --warm-up-time 1 --measurement-time 2 --sample-size 10 --noplot`
  - Worker: `ts2`
  - Median: `4.6723 ms`
- After: same command
  - Worker: `ts2`
  - Median: `636.52 us`
- After confirm: same command
  - Worker: `ts2`
  - Median: `637.83 us`

Conservative confirmed speedup: `4.6723 ms / 637.83 us = 7.32x`.

Score: `Impact 7.32 * Confidence 0.98 / Effort 1.5 = 4.78`, keep.

## Isomorphism Proof

- Fast path only triggers when each `(row, col)` pair is strictly greater than the previous pair, proving sorted unique coordinates.
- Duplicate coordinates fall back to `canonical_triplets`, preserving existing floating-point accumulation order for duplicate sums.
- Unsorted COO falls back to the existing canonical sort path.
- Direct compression emits the same `data`, `indices`, and row-count-derived `indptr` for sorted unique COO.
- Explicit zero entries are copied unchanged.
- No library RNG behavior changes; seeded random golden output is only a deterministic witness.
- `to_csc` is unchanged and still uses column-major canonical sorting.

Golden output before and after:

- `golden_before.txt` SHA256: `943927e5ee49288577e3ed37e13b8f38c76aec8d0b71ac159b4895905afd6df1`
- `golden_after.txt` SHA256: `943927e5ee49288577e3ed37e13b8f38c76aec8d0b71ac159b4895905afd6df1`
- `golden_before_after.diff`: empty

## Validation

- `cargo fmt -p fsci-sparse --check`: pass
- `RCH_FORCE_REMOTE=1 rch exec -- cargo check -p fsci-sparse --all-targets --locked`: pass
- `RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-sparse --lib --locked`: pass, `307 passed`
- `RCH_FORCE_REMOTE=1 rch exec -- cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`: pass
- `ubs --ci --only=rust crates/fsci-sparse/src/ops.rs`: exit 0
