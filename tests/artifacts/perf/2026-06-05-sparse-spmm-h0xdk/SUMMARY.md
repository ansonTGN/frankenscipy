# frankenscipy-h0xdk Slice-Iterator Trial

## Target

- Bead: `frankenscipy-h0xdk`
- Hotspot: `sparse_spmm/2000x2000_d1/2000`
- Profile evidence: post-`32f03911` sparse reprofile still ranked SpMM first at `12.090 ms` median on `ts2`.
- Candidate family: micro slice-iterator rewrite inside the existing CSR Gustavson traversal.

## Result

Rejected. Source was restored.

## Benchmark Evidence

- Baseline: RCH `ts1`, `9.7823 ms` median, interval `[9.6796 ms, 9.8911 ms]`.
- After: RCH `ts1`, `9.8091 ms` median, interval `[9.4174 ms, 10.505 ms]`.
- Decision: no real win; the after interval overlaps baseline and the median is slightly slower.
- Score: `0.0`, below the keep threshold.

## Behavior Proof

- Ordering preserved: yes. The attempted rewrite only changed local index-loop mechanics and kept A row order, A nonzero order, B row encounter order, and reverse first-seen output emission.
- Tie-breaking unchanged: yes. SpMM has no tie-breaking surface beyond deterministic encounter order, which was preserved.
- Floating-point order: identical. Multiplications and additions occurred in the same row-local order.
- RNG: none on the runtime path.
- Zero elision: unchanged, still `v.abs() > 0.0`.
- Metadata: unchanged.
- Golden SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2` before and after.

## Follow-Up

Do not repeat this micro-lever. The next `h0xdk` attempt must use a deeper GraphBLAS-style SpGEMM primitive, such as CSC/column-panel traversal or a semiring-style symbolic structure cache, while preserving the strict SpMM golden SHA.

## Branch-Guard Trial

Rejected. Source was restored.

- Candidate family: const-generic specialization to elide `k < b_rows` checks when `A.cols <= B.rows`.
- Baseline: RCH `ts1`, `9.7823 ms` median, interval `[9.6796 ms, 9.8911 ms]`.
- After: RCH `ts1`, `9.8231 ms` median, interval `[9.7015 ms, 9.9474 ms]`.
- Decision: no real win; the after interval overlaps baseline and the median is slower.
- Golden SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`.
- Strict payload comparison: `cmp` exit `0`.
- Score: `0.0`, below the keep threshold.
- Restore proof: `git diff -- crates/fsci-sparse/src/linalg.rs` is empty; `cargo fmt -p fsci-sparse --check` passed.

## Bound-Guard Rerun Addendum

Rejected again. Source was restored.

- Candidate family: same shape-proven bound-check specialization as the branch-guard trial.
- Behavior proof: strict filtered payload SHA stayed `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`; strict payload `cmp` exited `0`.
- Focused proof test: RCH `cargo test -p fsci-sparse --locked spmm_parallel_matches_serial_byte_for_byte -- --nocapture` passed on `ts1`.
- After evidence: RCH selected `vmi1149989`, so the `22.474 ms` median is cross-worker and not usable as keep proof against the `ts1` baseline. The earlier same-worker branch-guard run already showed no win (`9.7823 ms` -> `9.8231 ms`).
- Score: `0.0`; no source change kept.
- Restore proof: `git diff -- crates/fsci-sparse/src/linalg.rs` is empty; `cargo fmt -p fsci-sparse --check` passed after restore.

## Dense Outer-Product CSC Trial

Rejected. Source was restored.

- Candidate family: GraphBLAS-style outer-product SpGEMM over a transient CSC view of canonical A, with dense `m*n` accumulators and per-row first-seen replay.
- Baseline: RCH `ts1`, `9.7823 ms` median, interval `[9.6796 ms, 9.8911 ms]`. The earlier post-127pc sparse reprofile also ranked this row first on RCH `ts2` at `12.090 ms` `[11.965 ms, 12.230 ms]`.
- After: RCH `vmi1153651`, `93.216 ms` median, interval `[87.387 ms, 99.939 ms]`.
- Decision: clear regression; the dense global workspace overwhelms the intended traversal benefit. Cross-worker evidence is not used as a keep proof, and this result is already too slow to pursue.
- Behavior proof: guarded canonical A path preserved A `k` order, B row encounter order, reverse first-seen row emission, floating-point accumulation order, explicit zero elision, sorted metadata, tie behavior, and RNG absence.
- Focused proof: RCH byte-for-byte outer-product-vs-serial test passed before benchmarking.
- Golden SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`; strict payload comparison exited `0`.
- Score: `0.0`, below the keep threshold.
- Restore proof: `git diff -- crates/fsci-sparse/src/linalg.rs` is empty; `cargo fmt -p fsci-sparse --check` passed after restore.

## Next Primitive

Do not retry dense global outer-product traversal. The next `h0xdk` attempt should use a bounded sparse row-panel primitive: cache row-local symbolic structure and replay numeric values without `m*n` workspace, while preserving exact per-row encounter order and the strict SpMM golden SHA.

## Bounded Row-Plan Symbolic Cache Trial

Rejected. Source was restored.

- Candidate family: semiring-style symbolic row plan. The trial recorded each row's first-seen column structure without floating-point accumulation, then replayed the numeric products into compact row-local slots in the same encounter order.
- Fresh baseline: RCH `ts2`, `12.877 ms` median, interval `[12.742 ms, 13.023 ms]`.
- Same-worker comparison baseline: RCH `ts1`, `9.7823 ms` median, interval `[9.6796 ms, 9.8911 ms]`.
- After: RCH `ts1`, `10.325 ms` median, interval `[9.9190 ms, 10.754 ms]`.
- Decision: no real win; the same-worker after median regressed and the lower confidence bound is above the baseline median.
- Behavior proof: A row order, A nonzero order, B row encounter order, reverse first-seen output emission, floating-point accumulation order, explicit zero elision, metadata, tie behavior, and RNG absence were preserved.
- Focused proof: RCH `spmm_parallel_matches_serial_byte_for_byte` passed.
- Golden SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`; strict payload comparison exited `0`.
- Score: `0.0`, below the keep threshold.
- Restore proof: `git diff -- crates/fsci-sparse/src/linalg.rs` is empty; `cargo fmt -p fsci-sparse --check` passed after restore.

## Handoff

Close `frankenscipy-h0xdk` as rejected. Do not iterate further on SpMM row-plan/counting/replay variants in this bead; pivot to the next ready `[perf]` bead or to a fresh profile-backed primitive in a different algorithmic family.
