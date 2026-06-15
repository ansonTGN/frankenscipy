# frankenscipy-psn7x.3 packed-panel p/w rejection

Agent: `RubyWaterfall`
Date: 2026-06-15
Crate: `fsci-linalg`
Worker: RCH `ovh-a`

## Target

The profile-backed target remained native symmetric `eigh` Householder
reduction after the kept `psn7x.1` rank-2 index/fused-dot lever and the
rejected `psn7x.2` mirror-materialization write schedule.

This pass tested a deeper packed-panel primitive: copy each active suffix into
contiguous column-major panel storage before computing `p = tau*A*v`, while
preserving exact scalar `p`, `w`, and rank-2 update order before mutating the
live `DMatrix`.

No production source was retained after the performance gate failed.

## Baseline

Baseline artifacts were carried from the immediately preceding current-source
`ovh-a` run because `psn7x.2` restored the source before closeout and no linalg
source changed between the baselines and this candidate.

Criterion (`baseline_eigh_dense_criterion_ovh_a_rch.txt`):

| bench | mean range |
| --- | ---: |
| `eigh_dense/256x256` | `10.891 ms` - `11.009 ms` |
| `eigh_dense/512x512` | `93.944 ms` - `94.729 ms` |

Public native route (`baseline_public_native_route_ovh_a_rch.txt`):

| n | routed native | nalgebra | speedup |
| ---: | ---: | ---: | ---: |
| 400 | `46.813422 ms` | `41.760757 ms` | `0.892068x` |
| 800 | `204.479441 ms` | `347.971005 ms` | `1.701741x` |
| 1200 | `603.412375 ms` | `1060.210706 ms` | `1.757025x` |

Native direct (`baseline_native_vs_nalgebra_ovh_a_rch.txt`):

| n | native | nalgebra | ratio |
| ---: | ---: | ---: | ---: |
| 400 | `30.8 ms` | `43.5 ms` | `1.41x` |
| 800 | `189.1 ms` | `324.8 ms` | `1.72x` |
| 1200 | `587.0 ms` | `1062.6 ms` | `1.81x` |

## Candidate

Single source lever:

- Added a packed-panel rank-2 path that reuses a `Vec<f64>` scratch buffer.
- For each Householder step, copied the active suffix into contiguous
  column-major panel storage.
- Computed `p` from the packed panel in the same column/row loop order as the
  direct path.
- Reused the existing scalar `w` construction and rank-2 matrix update order.

## Proof

- `proof_packed_panel_rank2_bits_ovh_a_rch.txt`: packed-panel rank-2 path
  matched the direct current rank-2 path bit-for-bit for `p`, `w`, and every
  matrix entry.
- `proof_public_eigh_behavior_golden_ovh_a_rch.txt`: 16 public `eigh` behavior
  tests passed; public materialized-pair golden digest stayed
  `0x287a5d3679a8bc6a`.
- Ordering/tie behavior: unchanged by the public materialized-pair golden proof.
- Floating point: `p/w/matrix` bits were proven against the current direct
  rank-2 path before timing.
- RNG: deterministic fixtures/probes only; no randomness added.

## Rebench

Public native route (`after_packed_panel_public_native_route_ovh_a_rch.txt`):

| n | baseline routed | candidate routed | ratio |
| ---: | ---: | ---: | ---: |
| 400 | `46.813422 ms` | `46.116640 ms` | `1.015110x` |
| 800 | `204.479441 ms` | `242.336066 ms` | `0.843783x` |
| 1200 | `603.412375 ms` | `794.295248 ms` | `0.759682x` |

Native direct (`after_packed_panel_native_vs_nalgebra_ovh_a_rch.txt`):

| n | baseline native | candidate native | ratio |
| ---: | ---: | ---: | ---: |
| 400 | `30.8 ms` | `36.2 ms` | `0.850829x` |
| 800 | `189.1 ms` | `235.2 ms` | `0.803997x` |
| 1200 | `587.0 ms` | `829.2 ms` | `0.707912x` |

Score: `Impact 0.0 * Confidence 4.0 / Effort 2.0 = 0.0`.

Verdict: REJECT. Copying the active suffix into a packed panel preserved
behavior but added too much O(n^3) memory traffic before the rank-2 update.

## Next Primitive

Do not retry lower-storage-only mirror suppression, separated mirror
materialization, direct packed full-suffix copy, scalar/slice spelling variants,
worker-count retuning, delayed compact-WY generation with stale cross-block
state, row-major/row-block Givens replay, or per-step thread spawning.

Next route: switch away from per-reflector full active-suffix materialization.
Attack an algorithmically different two-stage path: dense-to-band reduction with
strict scalar replay proof at panel boundaries, or move to the shifted
tridiagonal eigensolver/backtransform wall if fresh profile shows reduction is
no longer the dominant post-`psn7x.1` cost.
