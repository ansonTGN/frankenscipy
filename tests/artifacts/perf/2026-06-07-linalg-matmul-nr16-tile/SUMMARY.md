# GEMM NR=16 Two-Panel Tile Rejection

Bead: `frankenscipy-8l8r1.42`

## Profile Target

The target remained the linalg no-gaps dense GEMM row after row-split
granularity and plain MR widening both failed.

Fresh pre-edit RCH Criterion baseline:

```text
command: RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- --warm-up-time 1 --measurement-time 3 'matmul/1024x1024' --noplot
worker: ts1
matmul/1024x1024 [43.408 ms 44.036 ms 45.106 ms]
```

## Lever Tested

Changed only the private flat-workspace GEMM full-tile column width from `NR=8`
to `NR=16`. The candidate accumulated two 8-lane SIMD B vectors per `k` while
keeping `MR=4`, row partitioning, output ordering, and each output cell's
monotonic `k` accumulation unchanged.

## Proof

Golden SHA proof:

```text
pre:  a759ebc9f8ae390a6c1c0247cb4abd85538355789b90330c41ce68d7779ad902
post: a759ebc9f8ae390a6c1c0247cb4abd85538355789b90330c41ce68d7779ad902
```

The SHA was computed from a temporary ignored in-module flat-workspace payload
test, filtered to the deterministic bit payload and piped to `sha256sum`.
The proof helper was removed after rejecting the lever so source returned to the
pre-pass state.

Focused RCH proof:

```text
command: RCH_FORCE_REMOTE=1 rch exec -- cargo test -p fsci-linalg --release --lib matmul --locked -- --nocapture
worker: vmi1149989
result: ok
passed: matmul_ikj_is_bit_identical_to_naive_ijk
passed: matmul_flat_compute_rows_row_split_is_bit_identical
passed: matmul_flat_workspace_is_bit_identical_to_naive_ijk
passed: matmul_microkernel_is_bit_identical_to_flat_ikj
passed: matmul_microkernel_golden_digest
```

Isomorphism:

- Ordering: row order, column order, and row partitioning unchanged.
- Tie-breaking: not applicable.
- Floating point: each cell kept the same monotonic `k` accumulation; tests compare `f64::to_bits`.
- RNG: none.
- Golden output: flat-workspace SHA unchanged.

## Rebench

Usable after-run:

```text
command: RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- --warm-up-time 1 --measurement-time 3 'matmul/1024x1024' --noplot
worker: vmi1149989
matmul/1024x1024 [208.10 ms 216.76 ms 225.49 ms]
```

This also regressed versus the recent same-production `vmi1149989` baseline from
the preceding MR=6 pass:

```text
matmul/1024x1024 [145.60 ms 155.63 ms 165.84 ms]
```

## Decision

Rejected. Score `0.0`.

Source restored to the pre-pass state. Avoid wider column-tile microkernels for
this exact path unless paired with a fundamentally different packing/traversal
model and a same-worker A/B plan.
