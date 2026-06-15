# psn7x back-transform column-chunk replay

Bead: `frankenscipy-psn7x`
Agent: `RubyWaterfall`
Base commit: `f6d29eea58445ff7e6149849b308a7fcf9411418`
Date: 2026-06-15

## Lever

`symmetric_eigh_native` now reuses the existing `apply_left_reflectors_column_chunks`
primitive for the eigenvector back-transform instead of applying every reflector to
all columns serially. This changes only the column scheduling of the already-built
reflector replay.

## Behavior proof

- Ordering and tie-breaking: eigenvalue sorting remains unchanged and still uses
  `total_cmp` over the tridiagonal eigenvalues after the back-transform.
- Floating-point order: each output column sees the same reflector order and the same
  dot/update order as the previous serial replay; only independent columns are split
  across scoped threads.
- RNG: no RNG exists in the changed production path.
- Unsafe: no unsafe code was added.
- Bitwise proof: `symmetric_eigh_backtransform_parallel_matches_serial_bits` compares
  the serial reflector replay against the column-chunk replay with `f64::to_bits()` for
  every element of a 160x160 matrix.
- Golden proof artifact SHA-256: `e76341aae0c7c13ddfe2b5da23f020f8770fbdb61f197d5f0ac07f917d56b33e`.

## Same-worker timing

RCH worker: `vmi1152480`

| size | baseline native | after native | speedup |
| --- | ---: | ---: | ---: |
| n=400 | 139.6 ms | 137.0 ms | 1.02x |
| n=800 | 1098.3 ms | 662.8 ms | 1.66x |
| n=1200 | 3510.1 ms | 2191.3 ms | 1.60x |

Large-case geometric speedup (`n=800`, `n=1200`): 1.63x.

Score: Impact 4 x Confidence 3 / Effort 2 = 6.0. Keep.

## Validation

- `cargo fmt --package fsci-linalg --check`: pass.
- `ubs crates/fsci-linalg/src/lib.rs`: exit 0, no critical findings; broad pre-existing
  warning inventory remains in this large file.
- RCH `cargo test -j 1 -p fsci-linalg --release --locked --lib symmetric_eigh_backtransform_parallel_matches_serial_bits -- --nocapture`: pass.
- RCH `cargo test -j 1 -p fsci-linalg --release --locked --lib symmetric_eigh_native -- --include-ignored --nocapture`: pass.
- RCH `cargo check -j 1 -p fsci-linalg --all-targets --locked`: pass.
- RCH `cargo clippy -j 1 -p fsci-linalg --all-targets --locked -- -D warnings`: blocked by pre-existing `fsci-fft/src/helpers.rs:58` unused variable.
- RCH `cargo clippy -j 1 -p fsci-linalg --lib --locked --no-deps -- -D warnings`: blocked by pre-existing `needless_range_loop` findings at `crates/fsci-linalg/src/lib.rs:3709`, `3720`, and `4170`.

## Artifacts

- `baseline_rank2_native_timing_vmi1152480_rch.txt`
- `proof_backtransform_parallel_bits_rch.txt`
- `after_backtransform_timing_vmi1152480_rch.txt`
- `check_fsci_linalg_all_targets_rch.txt`
- `clippy_fsci_linalg_all_targets_rch.txt`
- `clippy_fsci_linalg_lib_no_deps_rch.txt`
- `evidence_checksums.sha256`
