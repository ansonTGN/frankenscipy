# frankenscipy-mifdz: `eigh` index-sort output gather keep

## Lever

Public `eigh` no longer materializes `(eigenvalue, Vec<column>)` pairs before
sorting. It sorts source column indices by the same eigenvalue comparator, then
copies the selected eigenvector columns directly into the row-major public
output.

No eigensolver, eigenvalue comparator, validation, trace, public API, or
fallback behavior changed.

## Baseline

Command:

```text
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- eigh_dense
```

Worker: `vmi1227854`

Artifact: `baseline_criterion_eigh_dense_rch.txt`

```text
eigh_dense/256x256      time: [13.303 ms 13.677 ms 14.019 ms]
eigh_dense/512x512      time: [103.50 ms 105.45 ms 107.58 ms]
```

## After

Command:

```text
RCH_WORKER=vmi1227854 RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- eigh_dense
```

Worker: `vmi1227854`

Artifact: `after_eigh_index_sort_criterion_eigh_dense_rch.txt`

```text
eigh_dense/256x256      time: [12.240 ms 12.793 ms 14.082 ms]
eigh_dense/512x512      time: [89.998 ms 93.092 ms 97.074 ms]
```

Same-worker median deltas:

```text
256x256: 13.677 ms -> 12.793 ms = 1.069100x
512x512: 105.45 ms -> 93.092 ms = 1.132749x
```

Score: `Impact 2 * Confidence 5 / Effort 1 = 10.0`; keep.

## Isomorphism Proof

Proof artifact: `proof_eigh_index_sort_bits_rch.txt`

```text
eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a
test result: ok. 1 passed; 0 failed
```

- Ordering preserved: yes. Both paths use stable `sort_by` over the original
  nalgebra eigenvalue sequence with `f64::total_cmp`.
- Tie-breaking unchanged: yes. Equal-comparator rows keep original source order
  under stable sort; the proof includes an identity matrix fixture.
- Floating-point preserved: yes. The optimized path copies the same nalgebra
  eigenvalues and eigenvector elements; the proof compares all public output
  bits against the prior materialized-pair construction.
- RNG preserved: yes. No RNG surface exists in this path.
- Golden output: digest `0x287a5d3679a8bc6a`; artifact SHA verification passed
  via `evidence_checksums_verify.txt`.

## Validation

```text
RCH cargo test -p fsci-linalg --lib --locked eigh_index_sort_matches_materialized_pair_sort_bits -- --nocapture --test-threads=1
RCH cargo check -p fsci-linalg --lib --locked
RCH cargo clippy -p fsci-linalg --lib --locked --no-deps -- -D warnings
cargo fmt -p fsci-linalg --check
ubs crates/fsci-linalg/src/lib.rs
sha256sum -c evidence_checksums.sha256
```

All passed. UBS reported zero critical findings; broad warnings are pre-existing
file-wide inventory.
