# frankenscipy-8l8r1.98 sparse MMD ordering keep

## Target

- Bead: `frankenscipy-8l8r1.98`
- Crate: `fsci-sparse`
- Profile-backed target: `MmdAtPlusA` ordering/factorization tail on 2D Laplacian single-solve rows.
- Baseline profile: `current_perf_spsolve_rch.txt`
  - RCH worker `vmi1227854`
  - `lap2d k=20`: RCM `2.8220 ms`, MMD `3.60495 ms`
  - `lap2d k=32`: RCM `15.1412 ms`, MMD `18.93301 ms`
  - `lap2d k=45`: long-tail stall; routing evidence only.

## Lever

One selector lever in `minimum_degree_ordering`:

- Keep the current `HashSet` adjacency path as the default.
- Route only large, low-row-width sparse matrices (`rows >= 1024`, `nnz <= rows * 8`, max raw row width `<= 64`) through the ordered `BTreeSet` adjacency implementation.
- This preserves the exact degree/index MMD elimination contract while avoiding the HashSet path's measured tail on large low-row-width Laplacian-like matrices.

Rejected intermediate variants are retained as artifacts:

- `after_adaptive_mmd_order_perf_probe_vmi1153651_rch.txt`: full offdiag scan selector; rejected for arrowhead regression.
- `after_adaptive_rowwidth_mmd_order_perf_probe_vmi1153651_rch.txt`: row-width selector; rejected for small-grid regression.

## Same-worker timing

Worker: `vmi1153651`

| Case | Current HashSet baseline | Accepted selector | Ratio |
|---|---:|---:|---:|
| `lap2d_k20` | `2.906798 ms` | `1.905085 ms` | `1.526x` |
| `lap2d_k32` | `20.417964 ms` | `11.430370 ms` | `1.786x` |
| `arrowhead_n1000` | `0.403781 ms` | `0.402645 ms` | `1.003x` |

Artifacts:

- Baseline: `after_hashset_mmd_order_perf_probe_vmi1153651_rch.txt`
- Accepted: `after_adaptive_large_lowwidth_mmd_order_perf_probe_vmi1153651_rch.txt`
- Order digests unchanged:
  - `lap2d_k20`: `0x0217ba46fa6e6a05`
  - `lap2d_k32`: `0xffdd6ca421f7bd89`
  - `arrowhead_n1000`: `0xfdd649ab98f97f95`

Score: `(Impact 2.8 * Confidence 4.0) / Effort 1.5 = 7.47`, keep.

## Behavior proof

- Exact ordering/tie-breaking: `proof_mmd_adaptive_large_lowwidth_ordering_btree_reference_rch.txt`
  - `minimum_degree_ordering_matches_btree_reference_bit_for_bit` passed.
  - The proof compares the selected implementation against a BTree reference for empty, diagonal, 2D Laplacian, and arrowhead cases.
- Floating-point output: `proof_mmd_adaptive_large_lowwidth_laplacian_golden_raw_rch.txt`
  - Payload SHA-256: `2bf1774fb3f37ac554d9d747af10d434cbc031130f7abe70e09bd1cd52125e90`
  - Matches prior reference payload SHA.
- RNG: none used.
- Unsafe: none introduced.

## Validation

- RCH `cargo check -p fsci-sparse --lib --locked`: passed (`check_fsci_sparse_lib_rch.txt`).
- RCH `cargo clippy -p fsci-sparse --lib --locked --no-deps -- -D warnings`: passed (`clippy_fsci_sparse_lib_no_deps_rch.txt`).
- RCH clippy without `--no-deps`: blocked by pre-existing `fsci-fft` lint `manual_is_multiple_of` (`clippy_fsci_sparse_lib_rch.txt`).
- UBS on `crates/fsci-sparse/src/linalg.rs`: 0 critical; existing warning inventory retained (`ubs_fsci_sparse_linalg.txt`).
- `rustfmt --edition 2024 --check crates/fsci-sparse/src/linalg.rs`: reports unrelated existing formatting drift in this large shared file (`rustfmt_fsci_sparse_linalg_edition2024_check.txt`); not applied to preserve one perf lever per commit.

## Next profile route

The next sparse primitive should move beyond MMD adjacency selection into a symbolic factorization replacement: quotient graph / supernodal symbolic analysis with a fill-reducing ordering contract, measured against the shifted sparse LU tail.
