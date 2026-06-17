# Rejected: packed `u32` B column indices for SpMM

## Target

- Crate: `fsci-sparse`
- Bench: `sparse_spmm/2000x2000_d1/2000`
- Worker: `vmi1227854`
- Lever: build a guarded `Vec<u32>` copy of `B.indices()` for high-work SpMM and dispatch row chunks through a generic column-index accessor.

## Result

Baseline:

```text
tests/artifacts/perf/2026-06-11-sparse-spmm-symbolic-cache/baseline_spmm_2000_rch_retry4.txt
time: [14.675 ms 16.811 ms 19.579 ms]
```

Candidate:

```text
tests/artifacts/perf/2026-06-11-sparse-spmm-symbolic-cache/after_spmm_2000_rch_retry_loop3.txt
time: [15.088 ms 16.150 ms 17.379 ms]
```

Median moved from 16.811 ms to 16.150 ms, but the intervals overlap heavily.
This is not a credible Score >= 2.0 win, so the source change was reverted.

## Parity Evidence

- Parallel/serial byte-for-byte proof: `proof_parallel_byte_for_byte_rch_retry1.txt`
- Public ignored golden payload SHA-256:

```text
0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2
```

## Follow-Up

Do not repeat packed-index or accessor-generic micro-levers for this SpMM path.
The next sparse SpMM attempt should be a deeper symbolic/numeric split, persistent
row-plan cache, or layout-level primitive with a direct profile-backed baseline.
