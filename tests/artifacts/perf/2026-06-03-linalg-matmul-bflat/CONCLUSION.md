# B-flat matmul trial conclusion

Bead: `frankenscipy-8l8r1.10`
Target: `fsci-linalg::matmul`, existing 4x4 register micro-kernel.

## Lever

Trial lever: flatten read-only `B` once into contiguous row-major storage and use that buffer for full 4x4 tiles. Ragged/tail tiles kept the existing scalar path.

Source status: rejected and restored. `source_restored_diff.txt` is empty for `crates/fsci-linalg/src/lib.rs` and `crates/fsci-linalg/benches/linalg_bench.rs`.

## Baseline and profile

Fresh RCH Criterion baseline for `matmul`:

| Size | Baseline median |
| --- | ---: |
| 256x256 | 10.866 ms |
| 512x512 | 90.769 ms |
| 768x768 | 884.41 ms |
| 1024x1024 | 1.6964 s |

Historical committed profile context also kept `matmul` as the linalg hotspot with 1024x1024 median `394.26 ms`.

## Exact paired benchmark

The initial candidate benchmark looked faster against the fresh cross-worker baseline, but cross-worker spread was too high. A temporary bench-only paired comparator then measured the exact prior direct-`B` row kernel and the B-flat candidate in one RCH run on `vmi1149989`.

| Size | Prior direct-B median | B-flat median | Speedup |
| --- | ---: | ---: | ---: |
| 256x256 | 2.8045 ms | 3.1479 ms | 0.89x |
| 512x512 | 25.254 ms | 27.572 ms | 0.92x |
| 768x768 | 84.393 ms | 105.97 ms | 0.80x |
| 1024x1024 | 425.04 ms | 389.12 ms | 1.09x |

Result: rejected. Three of four sizes regressed, and the lone 1024x1024 win was modest and noisy.

## Behavior proof

Golden-output proof before and after the trial kept normalized sha256:

`0def10fbd95d1bf20c417af563de181eeab314cae762cc82fd67c1ebac6f406c`

The trial preserved the intended isomorphism while it was applied: API/error behavior, validation order, output order, per-cell monotonic `k` accumulation, separate multiply/add operations, RNG absence, tie-breaking absence, and global-state absence were unchanged. After rejection, the production source was restored to the prior direct-row implementation.

## Validation

- RCH `cargo test -p fsci-linalg --release matmul_microkernel --locked -- --nocapture` passed before the trial.
- RCH `cargo test -p fsci-linalg --release matmul_microkernel --locked -- --nocapture` passed after restore.
- `cargo fmt -p fsci-linalg --check` passed after restore.
- `source_restored_diff.txt` is empty.

## Score

Score: `0.0`. Performance impact was negative under the exact paired benchmark, so the lever did not meet the required `>=2.0` keep threshold.

## Next profile target

Re-profile next before choosing another lever. The failed result suggests that naively packing `B` rows adds allocation/copy cost and can inhibit the compiler's stronger direct-row optimization at smaller and mid-sized matrices. A future target should be profile-backed and likely focus on blocking/tiling that improves 1024x1024 without regressing 256-768.
