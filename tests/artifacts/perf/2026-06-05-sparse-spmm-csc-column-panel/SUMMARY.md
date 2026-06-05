# Sparse SpMM CSC Column-Panel Trial

Bead: `frankenscipy-qk91c`

Verdict: rejected; source restored. No Rust source change is kept in this pass.

## Baseline

- Command: `rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- sparse_spmm/2000x2000_d1/2000`
- Worker: `ts2`
- Criterion row: `sparse_spmm/2000x2000_d1/2000`
- Time: `13.182 ms` median, CI `[13.084, 13.282]`
- Log: `baseline_rch.txt`

## Behavior Proof

- Strict golden payload before: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Strict golden payload after restore: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- `cmp` before/restored strict payloads: `0`
- Preserved contract: A row order, B row encounter order, reverse first-seen output order, floating-point accumulation order, explicit zero elision, metadata, and RNG absence.

## Trial Result

The direct final-fill shape was rejected and the source hunk was restored. A row-slice replay measurement preserved the strict golden payload but did not produce a same-worker win.

- Same-worker replay confirmation: `13.280 ms` median, CI `[13.181, 13.382]`
- Baseline: `13.182 ms` median, CI `[13.084, 13.282]`
- Score: `0.0`

Next attack: fresh sparse reprofile, then a true CSC/column-panel or deeper GraphBLAS SpGEMM primitive rather than finalization/replay bookkeeping.
