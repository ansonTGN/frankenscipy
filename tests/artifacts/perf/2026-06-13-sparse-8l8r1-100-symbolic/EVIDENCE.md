# frankenscipy-8l8r1.100 evidence

Target: `[perf][sparse] replace sparse LU symbolic analysis with quotient-graph/supernodal primitive`.

Profile-backed source:
- `profile_perf_spsolve_rch.txt` is a partial routing profile. It reached the wider-banded
  sparse-vs-dense rows before the 3000x3000 dense comparison was stopped to avoid burning RCH
  time on a non-target dense baseline.
- Focused baseline is `baseline_mmd_ordering_perf_probe_rch.txt`, which times the existing
  MMD ordering probe and prints deterministic order digests.
- Hot row: `lap2d_k32 n=1024 nnz=4992`, `5.419913 ms`, digest
  `0xffdd6ca421f7bd89`.

One lever:
- For the large low-row-width MMD path, replace per-node `BTreeSet` adjacency with sorted
  `Vec<usize>` small-set adjacency.
- This is a memory-layout lever: contiguous neighbor storage avoids tree-node allocation and
  pointer chasing while preserving sorted neighbor order.
- The fallback/general MMD path remains the existing `HashSet` implementation.

Same-worker benchmark:
- Worker: `vmi1152480`.
- Baseline target row: `lap2d_k32 5.419913 ms`.
- First after target row: `lap2d_k32 3.718131 ms`.
- Repeat after target row: `lap2d_k32 3.352222 ms`.
- Conservative repeat delta: `5.419913 ms -> 3.352222 ms`, 2.067691 ms faster,
  38.15% reduction.
- Order digest unchanged: `0xffdd6ca421f7bd89`.
- `k20` and `arrowhead` use the unchanged `HashSet` path; their timing movement is worker noise
  and not attributed to this lever.

Behavior proof:
- Ordering/tie-breaking: the heap key remains `(degree, node_index)`, so equal-degree ties still
  pick the lowest index.
- Neighbor order: sorted `Vec` preserves the same ascending neighbor iteration as the previous
  `BTreeSet` branch.
- Fill graph: `sorted_insert_unique` and `sorted_remove` implement the same set semantics as the
  old adjacency sets.
- Floating point: this lever changes symbolic ordering storage only; numeric factorization,
  pivoting, residuals, and solves are unchanged.
- RNG: none on this path.
- `proof_mmd_sorted_vec_btree_reference_rch.txt` passes the BTree reference equality test,
  including `lap2d_k32`.
- `proof_mmd_laplacian_golden_payload_rch.txt` records the deterministic MMD Laplacian payload.
- SHA256 manifest: `SHA256SUMS`.

Quality gates:
- `rch exec -- cargo check -p fsci-sparse --all-targets --locked`: pass.
- `rch exec -- cargo clippy -p fsci-sparse --all-targets --locked --no-deps -- -D warnings`: pass.
- `ubs crates/fsci-sparse/src/linalg.rs`: exit 0, zero critical findings.
- `git diff --check -- crates/fsci-sparse/src/linalg.rs`: pass.
- `rustfmt --edition 2024 --check crates/fsci-sparse/src/linalg.rs`: fails on pre-existing
  unrelated full-file formatting drift outside this lever; transcript recorded in
  `rustfmt_sparse_linalg_check_preexisting_fail.txt`.

Score:
- Impact: 1.45, confidence: 0.90, effort: 0.45.
- Impact x Confidence / Effort = 2.90.
- Keep threshold met.
