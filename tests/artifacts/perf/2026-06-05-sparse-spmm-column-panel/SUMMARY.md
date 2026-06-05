# SpMM B-Row Replay Trial

Bead: `frankenscipy-8l8r1.31`

Decision: rejected, source restored.

Profile-backed target:
- `sparse_spmm/2000x2000_d1/2000`
- Fresh RCH baseline on `ts1`: `10.038 ms` median `[9.9218, 10.182]`
- Same-source historical RCH baseline on `ts2`: `12.761 ms` median `[12.592, 13.025]`

Lever tried:
- Precompute borrowed B-row replay slices for the CSR SpMM kernel.
- Route symbolic row counts and numeric row chunks through the replay table.
- This preserved row traversal and floating-point accumulation order, but did not provide a stable enough win.

Behavior proof:
- Strict golden SHA before: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Strict golden SHA after: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`
- Normalized strict payload diff: empty.

Performance:
- First after run on `ts2`: `4.8195 ms` median `[4.6963, 4.9838]`
- Confirmation on `ts2`: `12.174 ms` median `[12.053, 12.296]`
- Longer locked confirmation on `ts2`: `12.904 ms` median `[12.793, 13.018]`

Reason rejected:
- Stable confirmation did not clearly beat the same-source `ts2` baseline and was slower than the fresh `ts1` baseline.
- Score: `0.0`; no source change kept.

Next target:
- Avoid replay/mark/epoch/capacity cleanup variants.
- Reprofile and pivot to a deeper SpGEMM shape: a true column-panel/CSC traversal with a proof-preserving row-order replay strategy, or a different sparse primitive if SpMM is no longer the dominant row.
