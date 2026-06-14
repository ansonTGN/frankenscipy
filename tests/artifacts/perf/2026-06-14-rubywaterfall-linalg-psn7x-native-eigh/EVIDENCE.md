# frankenscipy-psn7x native symmetric eigh rank-2 update

## Change

Replaced the staged `symmetric_eigh_native` two-sided Householder similarity
update with the equivalent symmetric rank-2 trailing-block update:

`p = tau * A * v`, `w = p - (tau * (v^T p) / 2) * v`,
`A = A - v * w^T - w * v^T`.

The native route remains staged and is not wired into public `eigh`.

## Benchmark Gate

Same worker: `vmi1227854`.

| n | Before native | After native | Speedup |
|---|---:|---:|---:|
| 400 | 102.1 ms | 87.7 ms | 1.16x |
| 800 | 758.9 ms | 617.6 ms | 1.23x |
| 1200 | 3368.5 ms | 2277.7 ms | 1.48x |

Baseline artifact: `baseline_native_rank2_candidate_rch.txt`.
Final after artifact: `after_rank2_native_timing_final_vmi1227854_rch.txt`.

Score: `Impact 3.0 * Confidence 4.0 / Effort 2.0 = 6.0`.

## Isomorphism Proof

- Ordering preserved: yes. `symmetric_eigh_native` still sorts eigenvalues with
  `total_cmp` and reorders eigenvector columns by the same order.
- Tie-breaking unchanged: yes. No public route or sorting policy changed.
- Floating-point scope: internal staged native routine changes arithmetic from
  two full Householder passes to the standard symmetric rank-2 formulation; the
  proof gate verifies residuals, orthonormality, and nalgebra eigenvalue
  agreement under existing tolerances.
- RNG: unchanged. The deterministic test seed is unchanged.
- Public golden output: unchanged; `eigh_index_sort_public_golden_digest` stayed
  `0x287a5d3679a8bc6a`.

## Gates

- `cargo test -j 1 -p fsci-linalg symmetric_eigh_native_matches_nalgebra_and_timing --lib --release --locked -- --nocapture`
  passed on RCH `vmi1153651`.
- `cargo test -j 1 -p fsci-linalg eigh_index_sort_matches_materialized_pair_sort_bits --lib --release --locked -- --nocapture`
  passed on RCH `vmi1153651`.
- `cargo fmt --check -p fsci-linalg` passed locally.
- `cargo check -j 1 -p fsci-linalg --all-targets --locked` passed on RCH
  `vmi1227854`, with a pre-existing warning in `crates/fsci-linalg/src/bin/perf_cwt.rs`.
- `ubs crates/fsci-linalg/src/lib.rs` reported `Critical issues: 0`.

Clippy note: `cargo clippy -j 1 -p fsci-linalg --lib --locked --no-deps -- -D warnings`
is still blocked by three pre-existing `clippy::needless_range_loop` findings at
`crates/fsci-linalg/src/lib.rs:3709`, `3720`, and `4170`. The new rank-2 helper
lint was fixed and no longer appears in `clippy_fsci_linalg_lib_no_deps_after_newfix_rch.txt`.

## Next Route

The staged native path is faster but still slower than nalgebra
(`0.59x`, `0.63x`, `0.56x` vs nalgebra on the final run). The next
profile-backed lever is parallelizing the rank-2 trailing update and then the
eigenvector back-transform, before any public `eigh` routing.
