# frankenscipy-vifqu Evidence

Target: `sparse_arithmetic/10000x10000_d0_add/10000`

Profile context: after the CSR scale keep, the crate-scoped sparse reprofile ranked large CSR add first at about `2.1179 ms` median. The bead required a structurally different primitive instead of another CSR add bookkeeping tweak.

Lever: `combine_csr_rows_directly` now performs a GraphBLAS-style row-block symbolic/numeric split for large canonical CSR row merges. Each row is merged into chunk-local buffers; chunks are concatenated in row order. Small matrices keep the serial path.

Isomorphism proof:
- Row order: chunk ranges are built from increasing contiguous row intervals and concatenated in range order, so `indptr`, `indices`, and `data` retain row-major CSR order.
- Column order and duplicate handling: every row uses `merge_canonical_row`, shared by serial and parallel paths.
- Explicit zero elision: the same `value != 0.0` gate is used for each emitted value.
- Floating-point order: unmatched-left, unmatched-right, and matched-column branches use the original scalar expressions exactly: `0.0 + lhs`, `0.0 + rhs_scale * rhs`, and `let mut value = 0.0; value += lhs; value += rhs_scale * rhs`.
- RNG and tie-breaking: no runtime RNG or tie-breaking exists in CSR add. Test input generation remains deterministic only inside test/golden harnesses.
- Test proof: `combine_rows_parallel_matches_serial_byte_for_byte` compares serial and parallel `data`, `indices`, `indptr`, and `CanonicalMeta` for add/subtract scale modes and thread counts 2, 3, 7, and 64.

Golden output:
- RCH `perf_sparse add-csr-golden` regenerated `golden_current_payload.txt`.
- SHA-256: `a3a9d49d373b8d28f1aca881ab2b2322229b8befc0cb91c1e85a0820bc318da8`
- The regenerated payload matches `golden_final_payload.txt` with the same SHA-256.

Benchmarks:
- Baseline RCH Criterion on `ts2`: `[2.1038 ms 2.1057 ms 2.1080 ms]`.
- After RCH Criterion on `ts2`: `[1.7328 ms 1.7854 ms 1.8331 ms]`.
- Same-worker median change: `2.1057 ms -> 1.7854 ms`, `1.179x` faster, about `15.2%` lower median.
- Current-source confirmation on `ts1`: `[1.5566 ms 1.5719 ms 1.5875 ms]`.

Validation:
- `cargo fmt -p fsci-sparse --check`
- RCH `cargo check -p fsci-sparse --all-targets --locked`
- RCH `cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`
- RCH `cargo test -p fsci-sparse --locked -- --nocapture`: 308 unit tests, 56 metamorphic tests, doc-tests clean.
- `ubs crates/fsci-sparse/src/ops.rs`: exit 0.

Score: `6.0 = impact 3 * confidence 4 / effort 2`, above the keep threshold.
