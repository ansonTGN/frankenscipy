# frankenscipy-8l8r1.102 evidence

Target: `[perf][sparse] extend sorted small-set MMD adjacency to medium Laplacians`.

Profile-backed source:
- Post-`.100` sparse reprofile:
  `tests/artifacts/perf/2026-06-13-sparse-post-100-reprofile/mmd_ordering_perf_probe_rch.txt`.
- That routing run selected `vmi1153651` and showed `lap2d_k20 n=400` still on the
  old HashSet MMD path at `11.134419 ms`, while `lap2d_k32 n=1024` used the
  sorted small-set path and preserved digest `0xffdd6ca421f7bd89`.
- A repeat pinned to `vmi1153651` failed before execution with RCH dependency
  preflight `RCH-E326`; it is recorded as tooling context, not timing evidence.

One lever:
- Lower the sorted small-set MMD dispatch floor from `rows >= 1024` to
  `rows >= 256`.
- The density guard `nnz <= rows * 8` and raw-row-width guard `<= 64` remain
  unchanged, so dense hubs and general irregular high-degree graphs still use
  the existing HashSet path.
- No numeric factorization, pivoting, residual, tolerance, public API, RNG, or
  solve-output code changed.

Same-worker benchmark:
- Worker: `vmi1152480`.
- Baseline command transcript: `baseline_mmd_medium_vmi1152480_rch.txt`.
- Baseline target row: `lap2d_k20 1.316439 ms`, digest `0x0217ba46fa6e6a05`.
- First after transcript: `after_mmd_medium_sorted_floor_vmi1152480_rch.txt`.
- First after target row: `lap2d_k20 0.707151 ms`, digest unchanged.
- Repeat after transcript: `after_mmd_medium_sorted_floor_repeat_vmi1152480_rch.txt`.
- Repeat after target row: `lap2d_k20 0.469381 ms`, digest unchanged.
- Conservative delta uses the slower after point:
  `1.316439 ms -> 0.707151 ms`, 0.609288 ms faster, 46.28% reduction.

Behavior proof:
- Ordering/tie-breaking: unchanged. Heap keys remain `(degree, node_index)`, so
  equal-degree ties still pick the lowest index.
- Neighbor order: sorted `Vec<usize>` preserves ascending neighbor iteration,
  matching the old ordered-set reference.
- Fill graph: `sorted_insert_unique` and `sorted_remove` preserve set semantics.
- Floating point: unchanged. This lever changes symbolic ordering storage only;
  numeric sparse LU factorization and solve arithmetic are untouched.
- RNG: none on this path or in the deterministic probe fixtures.
- BTree reference proof:
  `proof_mmd_btree_reference_vmi1152480_rch.txt` passes
  `minimum_degree_ordering_matches_btree_reference_bit_for_bit`.
- Golden output proof:
  `proof_mmd_laplacian_golden_payload_vmi1152480_rch.txt` passes and records the
  deterministic `MMD_LAPLACIAN_GOLDEN` payload.

Quality gates:
- `rch exec -- cargo check -p fsci-sparse --all-targets --locked`: pass.
- `rch exec -- cargo clippy -p fsci-sparse --all-targets --locked --no-deps -- -D warnings`: pass.
- `git diff --check -- crates/fsci-sparse/src/linalg.rs`: pass.
- `ubs crates/fsci-sparse/src/linalg.rs`: exit 0; zero critical findings.
- `rustfmt --edition 2024 --check crates/fsci-sparse/src/linalg.rs`: still fails
  on pre-existing unrelated full-file formatting drift outside this one-line
  lever; transcript recorded in `rustfmt_sparse_linalg_check.txt`.

Score:
- Impact: 1.30.
- Confidence: 0.90.
- Effort: 0.35.
- Impact x Confidence / Effort = 3.34.
- Keep threshold met.
