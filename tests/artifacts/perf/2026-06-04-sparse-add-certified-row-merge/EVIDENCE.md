# CSR Add Certified Row Merge Trial

Bead: `frankenscipy-vs4vz`
Verdict: rejected, source restored.

## Profile Target

Fresh RCH `fsci-sparse` reprofile after the direct-transpose pass ranked
`sparse_arithmetic/10000x10000_d0_add/10000` as the top sparse row at
`2.1737 ms` median on `ts2`.

Focused same-worker baseline:

- Command: `RCH_WORKER=ts2 rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked sparse_arithmetic/10000x10000_d0_add/10000 -- --warm-up-time 1 --measurement-time 2 --sample-size 10 --noplot`
- Artifact: `baseline_add_csr_rch.txt`
- Median: `2.1695 ms`

## Trial Lever

Split metadata-canonical CSR add into a certified-output row merge that removed
`row_last_col` and output canonical bookkeeping from emission. The trial
preserved scalar addition order in the fast path:

- singleton values used `0.0 + value`
- overlaps used `value = 0.0; value += lhs; value += rhs`
- explicit-zero elision remained `value != 0.0`

## Proof Results

The first unguarded implementation had a real speedup but failed the focused
behavior suite:

- Unguarded after median: `1.9453 ms`
- Artifact: `after_add_csr_rch.txt`
- Failed test artifact: `cargo_test_fsci_sparse_add_csr_rch.txt`
- Failure: `add_csr_mislabelled_canonical_input_keeps_validating_path`

Root cause: `CsrMatrix::from_components(..., canonicalize=true)` can force
canonical metadata on structurally unsorted rows. The old direct merge preserves
observable row ordering and discovers output metadata; the certified-output
kernel incorrectly asserted sorted/deduplicated output from metadata alone.

The guarded implementation admitted the fast kernel only after a structural
row-order proof. That restored behavior:

- Focused RCH tests passed: `cargo_test_fsci_sparse_add_csr_rch_guarded.txt`
- Golden payload SHA-256 stayed
  `a3a9d49d373b8d28f1aca881ab2b2322229b8befc0cb91c1e85a0820bc318da8`
- Golden compare artifact: `golden_cmp_guarded.txt`

## RCH Gate

The guarded implementation failed the performance gate:

- Baseline median: `2.1695 ms`
- Guarded after median: `2.2003 ms`
- Ratio: `0.986x`
- Score: `0.0`, below `Score >= 2.0`

## Restore Proof

The source lever was removed after the failed gate:

- `git diff --quiet -- crates/fsci-sparse/src/ops.rs` exited `0`
- `cargo fmt -p fsci-sparse --check` exited `0`
- Restore artifact: `cargo_fmt_fsci_sparse_check_restored.txt`

Next sparse add work should avoid another metadata/bookkeeping micro-lever and
move to a fundamentally different primitive, such as a two-pass row capacity
planner, row-blocked batch merge, or sparse accumulator layout that can preserve
the mislabelled-input observable contract.
