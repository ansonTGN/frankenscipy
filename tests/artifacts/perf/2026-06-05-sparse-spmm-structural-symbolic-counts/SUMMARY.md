# Sparse SpMM Structural-Symbolic Count Trial

Bead: `frankenscipy-8l8r1.37`

## Target

Fresh profile target remained `sparse_spmm/2000x2000_d1/2000`.

## Lever

Trialed a GraphBLAS-style boolean structural symbolic pass for allocation capacity only: count unique touched output columns per row without floating-point accumulation, then keep the existing numeric Gustavson row chunk authoritative for columns, values, numeric row counts, indptr, sorted metadata, and explicit-zero elision.

## Behavior Proof

Strict SpMM golden payload stayed byte-identical:

`0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`

`golden_cmp.txt` records `strict_cmp=0`.

Ordering/FP contract: row ranges, row order, A traversal, B encounter order, reverse first-seen output emission, numeric floating-point accumulation order, explicit zero elision, metadata, and RNG absence were unchanged because the existing numeric row kernel stayed authoritative.

## Benchmarks

- Baseline: RCH `ts2`, median `11.923 ms`, interval `[11.770,12.095]`.
- After signal run: RCH `vmi1149989`, median `9.8901 ms`, interval `[9.7196,10.041]`; cross-worker only, not used as keep proof.
- Same-worker after confirmation: RCH `ts2`, median `12.288 ms`, interval `[12.215,12.381]`.

## Verdict

Rejected/no-ship. Same-worker `ts2` regressed from `11.923 ms` to `12.288 ms`, so Score is `0.0`, below the keep threshold. Source restored; `git diff -- crates/fsci-sparse/src/linalg.rs` is empty and `cargo fmt -p fsci-sparse --check` passed after restoration.
