# fsci-linalg matmul scalar-accumulator trial

Bead: `frankenscipy-8l8r1.13`

Verdict: rejected and restored. Score `0.0`, below the required keep gate of `2.0`.

## Profile-backed target

The target remained `fsci_linalg::matmul`, selected from the linalg perf backlog after the prior committed linalg profiles continued to rank 1024x1024 matmul as the dominant completed linalg hotspot.

## Baseline

RCH Criterion `matmul` baseline on `vmi1227854`:

- 256x256 median `4.6168 ms`
- 512x512 median `35.109 ms`
- 768x768 median `119.92 ms`
- 1024x1024 median `608.85 ms`

## Candidate

Single lever: replace only the full-tile `acc[[f64; 4]; 4]` accumulator with named scalar locals `c00..c33`, preserving loop order, input validation, output order, RNG absence, tie-breaking absence, global-state absence, and the ragged scalar path.

The first direct after-run exposed an intermediate variant that repacked scalar locals into a temporary `acc` array before writeback. That artifact is retained as evidence but was not used for the keep/reject decision. The corrected candidate used direct scalar writeback and was re-proven before final timing.

Corrected candidate direct RCH Criterion `matmul` on `vmi1153651`:

- 256x256 median `11.182 ms`
- 512x512 median `177.54 ms`
- 768x768 median `984.18 ms`
- 1024x1024 median `2.3127 s`

## Exact paired gate

Same-worker RCH Criterion on `vmi1227854` compared the prior array accumulator against the corrected scalar-accumulator candidate:

- 256x256: `3.0599 ms -> 4.5333 ms` (`0.68x`)
- 512x512: `25.525 ms -> 35.015 ms` (`0.73x`)
- 768x768: `89.355 ms -> 129.97 ms` (`0.69x`)
- 1024x1024: `673.78 ms -> 678.86 ms` (`0.99x`)

This is negative on the first three sizes and only noisy parity on the largest size, so the lever fails the keep gate.

## Behavior proof

RCH `cargo test -p fsci-linalg --release matmul_microkernel --locked -- --nocapture` passed before the candidate, after the corrected candidate, and after restore.

Stable sorted normalized golden sha256:

`85e17039e61d4a5e59aaa6f2ad70a28bb7e24a95b07d621ca6dbc392f652d9a1`

The same sha appears in:

- `golden_before_stable_sorted_normalized.txt`
- `golden_after_direct_store_stable_sorted_normalized.txt`
- `golden_after_restore_stable_sorted_normalized.txt`

## Restore

Production source and the temporary paired benchmark helper were restored. `source_restored_diff.txt` is empty. `cargo fmt -p fsci-linalg --check` passed after restore.
