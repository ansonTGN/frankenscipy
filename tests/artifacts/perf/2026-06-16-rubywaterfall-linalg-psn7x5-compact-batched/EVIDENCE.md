# frankenscipy-psn7x.5 rejection: column-pair backtransform replay

Agent: RubyWaterfall
Crate: `fsci-linalg`
Base commit: `57a7eb06`
Bead: `frankenscipy-psn7x.5`

## Scope

Lever tested: process two eigenvector columns per reflector inside
`apply_left_reflectors_to_column_chunk`, while preserving each column's
ascending dot-product offset order and update offset order.

No public dispatch, sorting, fallback, RNG, worker-count, or unsafe-code policy
changed. The source lever was restored after the failed score gate.

## Baseline

RCH worker: `vmi1149989`

Post-`9lb2l` stage baseline:

| n | reduction | tridiagonal_eigen | backtransform | sort | values digest |
|---:|---:|---:|---:|---:|---|
| 400 | 10.841124 ms | 9.405592 ms | 16.684691 ms | 0.670820 ms | `0x0dbbde75b75c8612` |
| 800 | 115.734092 ms | 37.822541 ms | 50.280387 ms | 1.903727 ms | `0x4461962827bdb038` |
| 1200 | 327.912149 ms | 85.739157 ms | 154.307902 ms | 25.004178 ms | `0x2fc45e1f18ceb0ab` |

Transcript: `baseline_stage_profile_rch.txt`

## Proof

RCH `symmetric_eigh_backtransform_parallel_matches_serial_bits` passed on
`vmi1149989`.

The proof compares the parallel column-chunk replay against scalar reflector
replay by `f64::to_bits()`, so ordering/tie behavior, floating-point update
order, and RNG absence stayed within the existing bitwise contract.

Transcript: `proof_column_pair_replay_bits_rch.txt`

## Rebench

RCH worker: `vmi1149989`

| n | before backtransform | after backtransform | speedup | values digest |
|---:|---:|---:|---:|---|
| 400 | 16.684691 ms | 16.877705 ms | 0.988564x | `0x0dbbde75b75c8612` |
| 800 | 50.280387 ms | 204.621209 ms | 0.245724x | `0x4461962827bdb038` |
| 1200 | 154.307902 ms | 491.193883 ms | 0.314157x | `0x2fc45e1f18ceb0ab` |

Transcript: `after_column_pair_stage_profile_rch.txt`

## Verdict

Rejected/no-ship. Values digests stayed unchanged and the bitwise replay proof
passed, but the timing regressed materially on the target sizes.

Score: Impact `0.0` x Confidence `4.0` / Effort `1.0` = `0.0`.

Source status after restoration: `git diff -- crates/fsci-linalg/src/lib.rs`
is empty.

## Next Route

The post-`9lb2l` profile now ranks lower-storage tridiagonal reduction first:
n=1200 reduction `327.912149 ms`, backtransform `154.307902 ms`,
tridiagonal solve `85.739157 ms`. Route to an algorithmically different
blocked/communication-avoiding reduction primitive rather than another
backtransform loop-order tweak.
