# CSR Sorted-Unique COO Fast Path

Bead: `frankenscipy-8l8r1.34`
Agent: `OliveSnow`
Date: `2026-06-05`

## Target

Fresh `fsci-sparse` RCH profiling ranked
`sparse_csr_construction/10000x10000_d0/10000` as the next distinct sparse
hotspot after SpMM, CSR add, and format-conversion trials.

## Lever

`sorted_unique_coo_to_csr` already scans the COO coordinates and only takes the
fast path when every consecutive `(row, col)` pair is strictly increasing. That
certificate implies row-local sorted indices and deduplicated entries. The
`CooMatrix` constructor already enforces equal lengths and in-bounds row/column
indices, so the fast path now builds the compressed CSR components with
`CsrMatrix::from_components_unchecked` and marks canonical metadata directly.

The generic unsorted or duplicate COO path still falls back to the sort/dedup
construction path.

## Isomorphism Proof

- Ordering preserved: yes. The fast path copies `cols` and `data` in original
  strictly row-major COO order and builds `indptr` from row counts.
- Tie-breaking unchanged: yes. Duplicate coordinates never enter this path
  because `prev >= current` falls back to generic canonicalization.
- Floating-point preserved: yes. Values are copied without arithmetic.
- RNG preserved: yes. The random benchmark input is created before the timed
  `to_csr` conversion; this lever does not touch sampling.
- Golden output: `coo-csr-golden` payload SHA stayed
  `943927e5ee49288577e3ed37e13b8f38c76aec8d0b71ac159b4895905afd6df1`, and
  `cmp` against `golden_after.txt` returned `0`.

## Performance

- Baseline on `ts2`: `623.92 us` median `[621.61 us, 626.16 us]`.
- After on `ts1`: `256.98 us` median `[251.24 us, 263.78 us]`.
- Restored baseline on `ts1`: `527.53 us` median `[499.27 us, 553.28 us]`.
- Reapplied confirmation on `ts2`: `394.47 us` median
  `[393.07 us, 395.87 us]`.

Primary same-worker pair: `527.53 us -> 256.98 us`, `2.05x`.
Reapplied `ts2` pair: `623.92 us -> 394.47 us`, `1.58x`.

Score: `8.0 = impact 4 * confidence 4 / effort 2`.

## Validation

- `cargo fmt -p fsci-sparse --check`
- `rch exec -- cargo check -p fsci-sparse --all-targets --locked`
- `rch exec -- cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`
- `rch exec -- cargo test -p fsci-sparse --locked -- --nocapture`
- `ubs crates/fsci-sparse/src/ops.rs`

All validation commands passed. UBS exit code was `0` with zero critical issues;
reported warnings were the existing broad `ops.rs` warning inventory.
