# Rejected: RB=128 row-panel geometry

Date: 2026-06-11
Agent: BlackThrush
Bead: `frankenscipy-8l8r1.82`
Worker: `vmi1227854`
Verdict: rejected; no source retained.

## Lever

Change the private `RB` row-panel constant in `matmul_flat_compute_rows` from
`64` to `128`.

This was a row-panel/cache-layout probe only. It did not change thread count,
worker chunking, B packing, `MR=4`, `NR=8`, `NC=16`, public dispatch, scalar
tails, output ordering, RNG surface, tie behavior, or per-output monotonic
`k = 0..ka` accumulation.

## Behavior proof

All proof gates completed before the after benchmark:

```text
proof_matmul_group_rch.txt
sha256 cc10e34239f58e92bae70aee19cda944ab433c2f3e915e6e6a4c380ff1070eb7
worker vmi1227854
result 5 passed, 0 failed, 3 ignored
```

```text
proof_medium_route_golden_rch.txt
sha256 479636623120f1e23a38fb113394d88fa7e9ec58052a06fa6d758647c376b7a0
worker vmi1227854
digest 0x5fd37bf053d54fb0
result passed
```

Additional gates:

```text
check_fsci_linalg_rch.txt
sha256 d5d05c2f72aebdfff8555ecf4839a2ef5102ec63735dea625b209a139e3cbe8b
worker vmi1227854
result cargo check -p fsci-linalg --all-targets passed
```

```text
ubs_fsci_linalg_lib.txt
sha256 439617cefcaaeaccb049aa90f106fe11f33bdb83e2449965514674b7e9fba709
result 0 critical findings
```

`git diff --check` passed. Standalone `cargo fmt -p fsci-linalg --check` remains
blocked by pre-existing unrelated formatting hunks around `sqrt_t` and
`is_symmetric`; no formatting source changes were mixed into this one-lever
trial.

## Same-worker benchmark

Baseline:

```text
baseline_matmul_criterion_rch_retry2.txt
sha256 3061f1e044c6caf6c4944d5dbe2a69b62bce1679f2a3fdecd3fd5545531254f7
worker vmi1227854
256   5.3159 ms
512  39.0660 ms
768 104.9200 ms
1024 199.2800 ms
```

Candidate:

```text
after_rb128_matmul_criterion_rch.txt
sha256 5c9682a05ec8ec7e7db5841c4204ec7b369d5b0a6e4d87e6294ead55045e6171
worker vmi1227854
256   6.1616 ms
512  50.8340 ms
768 122.5900 ms
1024 209.9100 ms
```

Speed ratios:

| row | ratio |
| --- | ---: |
| 512 | `0.768501x` |
| 768 | `0.855861x` |
| 1024 | `0.949359x` |

Affected-size geomean: `0.854725x`.

## Score

Impact is negative, so Score is below the `>= 2.0` keep gate. Source was
restored to zero diff.

## Route

Do not repeat private row-panel geometry (`RB`) probes or worker-count row
scheduling for this target. The next primitive must be materially deeper:
exact-order K-striping / BLIS-style `KC` macro-kernel work that preserves the
monotonic per-output `k` accumulation order, or a separate bit-exact safe-Rust
microkernel family with same-worker A/B proof.

