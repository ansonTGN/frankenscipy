# Signal remez flat normal-matrix rejection

Bead: `frankenscipy-g6x1v`

Target:
- `design/remez/257_two_band`
- Profile source: `tests/artifacts/perf/2026-06-02-signal-profile/reprofile_after_remez_broad_rch.txt`
- Reprofile row: `[35.062 ms 35.795 ms 36.553 ms]`

Proposed lever:
- Assemble the dense Remez normal-equation matrix in a flat contiguous `Vec<f64>`.
- Preserve grid order, coefficient order, cosine basis values, per-cell update order, symmetric mirrored writes, solver input values, tap conversion order, and RNG-free behavior.
- Convert back to the existing `Vec<Vec<f64>>` solver boundary after assembly so the solver path is unchanged.

Graveyard primitive:
- Packed/SoA-style contiguous storage for dense update kernels, selected because the hot loop is a dense normal-equation update over a 129 by 129 matrix for each frequency-grid row.

Behavior proof:
- Golden before SHA256: `55cd534a7c608b0d07be5ada52a24d38ee5512c57718ae32439028f44b4df45a`
- Golden after SHA256: `55cd534a7c608b0d07be5ada52a24d38ee5512c57718ae32439028f44b4df45a`
- `cmp golden_before.txt golden_after.txt`: passed

Baseline:
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-signal --bench signal_bench --locked -- design/remez/257_two_band --warm-up-time 1 --measurement-time 2 --sample-size 10 --noplot`
- Worker: `vmi1156319`
- Result: `[72.480 ms 74.450 ms 76.242 ms]`

Rebench:
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-signal --bench signal_bench --locked -- design/remez/257_two_band --warm-up-time 1 --measurement-time 2 --sample-size 10 --noplot`
- Worker: `vmi1153651`
- Result: `[95.226 ms 115.06 ms 139.19 ms]`

Decision:
- Rejected. The lever preserved golden output but did not demonstrate a real RCH win and failed the Score >= 2.0 keep gate.
- Production code and helper edits were reverted; no `fsci-signal` source change is kept.
- Next remez direction should be algorithmic, for example eliminating the frequency grid and computing normal-equation band moments analytically, with a tolerance-backed oracle plan because bit-for-bit equality is unlikely under a reordered summation.
