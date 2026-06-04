# fsci-linalg matmul baseline/profile capture

Pass: 1 - Fresh Baseline And Profile
Bead: frankenscipy-8l8r1
Captured: 2026-06-03T01:25:00-04:00
HEAD: 3e778411811e194b5949e64a91c7204c866ab8ba

## Scope

No source edits were made in this pass. This artifact captures the current
HEAD crate-scoped Criterion baseline for `fsci_linalg::matmul` before any new
A-panel or packed-panel implementation work.

## Commands

```bash
rch status > tests/artifacts/perf/2026-06-03-linalg-matmul-a-panel/rch_status.txt 2>&1
git rev-parse HEAD > tests/artifacts/perf/2026-06-03-linalg-matmul-a-panel/head.txt
date -Is > tests/artifacts/perf/2026-06-03-linalg-matmul-a-panel/capture_started_at.txt
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench matmul > tests/artifacts/perf/2026-06-03-linalg-matmul-a-panel/criterion_matmul_rch_raw.txt 2>&1
RCH_FORCE_REMOTE=1 rch exec -- perf stat -d cargo bench -p fsci-linalg --bench linalg_bench matmul/1024x1024 > tests/artifacts/perf/2026-06-03-linalg-matmul-a-panel/profile_perf_stat_1024_rch_raw.txt 2>&1
```

## RCH

The Criterion run executed remotely on `vmi1149989`:

- selected worker: `vmi1149989 at ubuntu@212.90.121.76`
- remote command: `cargo bench -p fsci-linalg --bench linalg_bench matmul`
- remote command exit: `0`
- RCH footer: `[RCH] remote vmi1149989 (148.8s)`

`rch status` reported degraded fleet posture but usable remote capacity:
`7/10 healthy`, `20/134 slots available`.

## Criterion Baseline

Criterion reported the intervals below as `[lower median upper]`.

| row | lower | median | upper |
| --- | ---: | ---: | ---: |
| `matmul/256x256` | 3.4755 ms | 3.6440 ms | 3.7297 ms |
| `matmul/512x512` | 29.995 ms | 31.747 ms | 34.240 ms |
| `matmul/768x768` | 111.26 ms | 121.76 ms | 131.47 ms |
| `matmul/1024x1024` | 343.08 ms | 394.26 ms | 447.06 ms |

The 768 row warned that 10 samples could not complete in the default 5 second
target and Criterion extended the estimate to 6.2610 seconds.

## Profile Note

`perf stat -d` was attempted through `rch exec` for the 1024 row, but profiling
was blocked by the host kernel setting:

```text
perf_event_paranoid setting is 4
```

No perf counters or callgraph were captured. The current profile-backed target
for the next pass is therefore the Criterion `matmul` group itself, with the
hot path constrained to `fsci_linalg::matmul` called from
`crates/fsci-linalg/benches/linalg_bench.rs`.

## Isomorphism Status

No behavior-changing lever was implemented in this pass. Ordering,
tie-breaking, floating-point accumulation, RNG use, and golden outputs are
unchanged because no source files were edited.
