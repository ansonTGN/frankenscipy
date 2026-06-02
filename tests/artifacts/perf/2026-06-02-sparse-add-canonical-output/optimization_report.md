# fsci-sparse CSR Add Canonical Output Optimization

Bead: `frankenscipy-llls2`

## Profile Target

Post-profile after `frankenscipy-572zu` shifted the top sparse row to:

| Benchmark | RCH worker | Mean |
| --- | --- | --- |
| `sparse_arithmetic/10000x10000_d0_add/10000` | `vmi1293453` | `1.6954 ms` |

Focused baseline for this bead:

| Benchmark | RCH worker | Time |
| --- | --- | --- |
| `sparse_arithmetic/10000x10000_d0_add/10000` | `vmi1227854` | `[1.8421 ms 1.9356 ms 2.0463 ms]` |

## Opportunity Matrix

| Hotspot | Impact | Confidence | Effort | Score |
| --- | ---: | ---: | ---: | ---: |
| `combine_csr_rows_directly` output canonicality rescan | 3 | 4 | 2 | 6.0 |

## Change

`combine_csr_rows_directly` now skips the output `CsrMatrix::from_components(..., false)`
canonicality rescan only when both input CSR row segments are verified structurally canonical.
Inputs that merely claim canonical metadata still use the old validating constructor path.

## Benchmark Delta

| Benchmark | Before | After | Delta |
| --- | ---: | ---: | ---: |
| `sparse_arithmetic/10000x10000_d0_add/10000` | `1.9356 ms` | `1.6640 ms` | `-14.0%` |

## Isomorphism Proof

- Ordering preserved: yes. The two-pointer merge still emits row entries in the same order; verified inputs skip only a post-merge metadata scan.
- Tie-breaking unchanged: yes. Equal columns are still merged by adding lhs first, then rhs scaled value.
- Floating-point preserved: yes. Existing `0.0 + lhs`, `0.0 + rhs_scale * rhs`, and lhs-then-rhs addition order is unchanged.
- RNG unchanged: yes. The optimization does not touch `random` or benchmark seed generation.
- Mislabelled canonical metadata preserved: yes. A regression test pins the old validating path for falsely marked canonical input rows.
- Golden outputs: `golden_before.txt` and `golden_after.txt` match byte-for-byte; SHA `8391613d472fce793d2f8142226f247d710ff10b7085c3d2efb159c5f177a7e0`.

## Validation

- RCH focused baseline: `baseline_add_10000_rch.txt`, exit `RCH_SPARSE_ADD_BASELINE_EXIT:0`.
- RCH golden before/after: `golden_before_rch_raw.txt`, `golden_after_rch_raw.txt`, `GOLDEN_ADD_CMP:ok`.
- RCH focused tests: `cargo_test_fsci_sparse_add_rch.txt` and post-format rerun `cargo_test_fsci_sparse_add_postfmt_rch.txt`, `3 passed`.
- `rustfmt --check crates/fsci-sparse/src/ops.rs`: final exit `0`.
- `cargo fmt -p fsci-sparse --check`: exit `1` due pre-existing `crates/fsci-sparse/src/lib.rs` formatting drift; `ops.rs` is clean after final fix.
- UBS `crates/fsci-sparse/src/ops.rs`: exit `0`; no critical findings.
- RCH exact clippy: exit `101` on pre-existing lowercase SciPy alias types in `crates/fsci-sparse/src/lib.rs`.
- RCH clippy with `-A non_camel_case_types -D warnings`: exit `0`.

## Reprofile Note

After this lever, the next profile pass should rerun the sparse criterion matrix. The likely
shifted rows from the prior post-profile were CSR construction and `diags/tridiag/10000`.
