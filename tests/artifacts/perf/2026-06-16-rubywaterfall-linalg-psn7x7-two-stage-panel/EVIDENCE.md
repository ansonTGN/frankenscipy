# frankenscipy-psn7x.7 Evidence - Two-Stage Panel Symmetric Reduction

## Baseline

RCH worker: `vmi1152480`.

Command: `rch exec -- cargo test -p fsci-linalg symmetric_eigh_native_stage_breakdown_probe --release -- --ignored --nocapture`

Current-head direct native `eigh` stage profile:

| n | reduction | tridiagonal | backtransform | sort | values digest |
|---:|---:|---:|---:|---:|---|
| 400 | 27.594096 ms | 17.321068 ms | 18.362451 ms | 0.571522 ms | `0x0dbbde75b75c8612` |
| 800 | 313.486667 ms | 77.344520 ms | 196.974033 ms | 2.685804 ms | `0x4461962827bdb038` |
| 1200 | 782.323182 ms | 133.213448 ms | 644.164411 ms | 7.431731 ms | `0x2fc45e1f18ceb0ab` |

Profile verdict: reduction remains a measured wall, with backtransform second. This bead targeted a true two-stage/panel reduction primitive rather than another scalar loop spelling.

## Sub-Kernel Routing Probe

Command: `rch exec -- cargo test -p fsci-linalg compact_wy_full_to_band_replay_perf_probe --release -- --ignored --nocapture`

Known-reflector compact-WY replay is a real fast sub-kernel:

| n | bandwidth | panel width | scalar replay | compact-WY replay | speedup | max abs drift |
|---:|---:|---:|---:|---:|---:|---:|
| 256 | 32 | 8 | 15.070133 ms | 8.765772 ms | 1.719202x | 1.59161572810262442e-12 |
| 512 | 32 | 8 | 127.755495 ms | 73.758112 ms | 1.732087x | 6.13908923696726561e-12 |

Routing interpretation: applying an already-known reflector panel as compact-WY is useful, but production full-to-band reduction still needs a correct way to generate later reflectors inside a panel.

## Rejected Source Lever

Temporary lever:

- Generate full-to-band reflectors panel-by-panel.
- Apply each reflector to the prefix/cross-block region needed by later panel columns.
- Apply the active suffix once through compact-WY.
- Bridge the resulting band matrix through a non-recursive band eigensolver and replay full-to-band reflectors.

RCH proof: `cargo test -p fsci-linalg compact_wy_full_to_band_generation_matches_scalar_reduction -- --nocapture` on `vmi1152480`.

Result: rejected before release rebench. The proof failed with scalar-reduction drift:

- Invalid panel geometry (`n=18`, bandwidth `1`, panel width `3`): drift `6.89925071389281275e-1`.
- Production-shaped panel geometry (`n=37`, bandwidth `8`, panel width `4`): drift `6.72055843216352450e-2`.

Isomorphism verdict: behavior was not proven. The source lever was restored before commit, so public ordering, tie behavior, floating-point behavior, RNG behavior, and golden values digests remain unchanged on the committed tree.

Score: `Impact 0.0 x Confidence 4.0 / Effort 2.0 = 0.0`.

## Next Primitive

Do not retry raw-reflector compact-WY generation with only prefix cross-block updates. The next algorithmic primitive must use a proper transformed-panel formulation: compact WY with panel vectors updated in the live panel basis, or a full two-stage route with band-to-tridiagonal bulge chasing and accumulated orthogonal transformations. Target: at least `2x` reduction-stage win on the same-worker 800/1200 native `eigh` stage profile while preserving the public tolerance contract.
