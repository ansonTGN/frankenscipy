# SpMM Versioned Sparse Accumulator Trial

Bead: `frankenscipy-moufa`

## Profile-Backed Target

The post-symbolic-prepass sparse reprofile in
`tests/artifacts/perf/2026-06-04-sparse-reprofile-after-symbolic-row-count/reprofile_sparse_bench_rch.txt`
still ranked `sparse_spmm/2000x2000_d1/2000` as the dominant fsci-sparse row:
RCH `ts1` median `10.445 ms` with interval `[10.347, 10.510]`.

Focused baseline for this trial:

- Worker: RCH `ts2`
- Benchmark: `sparse_spmm/2000x2000_d1/2000`
- Median: `12.761 ms`
- Interval: `[12.592, 13.025]`

## Lever Tried

Replace the row-local `Vec<bool>` seen set in both the symbolic row-count pass
and numeric fill pass with a versioned `Vec<usize>` mark vector. This avoids
clearing touched `seen` bits and stale accumulator values after each row, while
retaining the same row traversal and first-seen list.

## Isomorphism Proof

The trial preserved:

- contiguous row ranges and concatenation order
- A-row traversal order
- B-row encounter order
- first-seen `column_order` push order
- reverse first-seen emission order
- floating-point accumulation order per output cell
- explicit zero-elision predicate
- sorted-index flag semantics
- deduplicated metadata
- RNG absence

Strict golden payload SHA-256 stayed
`0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`, and the
strict payload diff was empty.

## Performance Result

Same-worker RCH `ts2` after benchmark:

- Median: `13.460 ms`
- Interval: `[13.337, 13.655]`

This regressed from the `12.761 ms` baseline, so the lever failed the real-win
gate.

## Verdict

Rejected. Score `0.0`; source restored to the shipped symbolic row-count
implementation.

Next attack: do not continue the mark/epoch/cleanup family. Use a deeper
GraphBLAS SpGEMM primitive such as CSC or column-panel traversal with row-local
order replay, or a symbolic structure cache that changes the memory model rather
than only the accumulator clearing policy.
