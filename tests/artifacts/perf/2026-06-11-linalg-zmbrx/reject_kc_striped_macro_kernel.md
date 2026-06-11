# Rejected: GEMM KC-Striped Macro-Kernel Probe

Bead: `frankenscipy-zmbrx`

## Lever

Tested one exact-order safe-Rust KC-striped flat-workspace GEMM path for 768+ dimensions. The candidate kept the existing public API, B packing format, row ownership, `MR=4`, `NR=8`, `NC=16`, and monotonic per-output `k` accumulation. Completed KC prefixes were written back to `out` before the next stripe resumed the same scalar addition chain.

The source lever was rejected and fully restored; no source change is retained.

## Baseline

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- matmul
```

Worker: `vmi1227854`

Artifact: `baseline_matmul_criterion_rch_retry2.txt`

SHA256: `7d0517cd2b4d022303bee2375a849d84f4d1e1113daf71eaf1da35ca86cbd414`

Means:

| size | baseline |
| --- | ---: |
| 256 | 5.3357 ms |
| 512 | 50.110 ms |
| 768 | 134.94 ms |
| 1024 | 230.74 ms |

## Behavior Proof

Focused KC-striped bit-identity proof:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo test -j 1 -p fsci-linalg --lib --locked matmul_flat_compute_rows_kc_striped_is_bit_identical -- --nocapture
```

Artifact: `proof_kc_striped_bit_identity_rch_retry2.txt`

SHA256: `cdba56b819f1bc6006f0645b3ddb3dbbd8ea2f642b1481c2705c84d6aa1d72ed`

Result: passed on `vmi1227854`.

Matmul proof group:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo test -j 1 -p fsci-linalg --lib --locked matmul_ -- --nocapture
```

Artifact: `proof_matmul_group_rch.txt`

SHA256: `ad03d5a0180341301d1fb4875e22d57b8d5a477db277d6df4033fc985ebec7dd`

Result: 6 passed, 0 failed, 3 ignored on `vmi1227854`.

Release golden route proof:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo test -j 1 -p fsci-linalg --release --lib --locked matmul_medium_flat_workspace_route_golden_digest -- --ignored --nocapture --test-threads=1
```

Artifact: `proof_medium_route_golden_rch.txt`

SHA256: `a201dbfd948ce3031a9079ede719e6c3584d66a25099cc690956658e87098f92`

Result: passed on `vmi1227854`; digest `0x5fd37bf053d54fb0`.

Formatting note: `cargo fmt -p fsci-linalg --check` still reports the pre-existing unrelated `sqrt_t` and `is_symmetric` formatting hunks; the candidate hunk did not add new rustfmt drift.

## After Run

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- matmul
```

Worker: `vmi1227854`

Artifact: `after_kc_striped_matmul_criterion_rch.txt`

SHA256: `6da7113eddd0113b93f0e13f140bea45b20220eb8d9a2f162889d28268f5004b`

Means:

| size | baseline | candidate | ratio |
| --- | ---: | ---: | ---: |
| 256 | 5.3357 ms | 5.0985 ms | 1.046522x |
| 512 | 50.110 ms | 61.460 ms | 0.815326x |
| 768 | 134.94 ms | 159.09 ms | 0.848199x |
| 1024 | 230.74 ms | 321.63 ms | 0.717408x |

Affected-size geomean for the 768+ candidate route: `0.780067x`.

## Decision

Rejected below Score >= 2.0. The KC-striped path preserved behavior but paid too much C read/write traffic and lost badly at the affected sizes.

Do not repeat this KC-prefix writeback family. The next route should avoid B staging/direct-pack, panel-load spelling, scalar-splat spelling, MR/NR widening, worker-count row scheduling, 8-row row-panel accumulators, K-major A row-slab packing, RB geometry, and KC-striped C writeback.
