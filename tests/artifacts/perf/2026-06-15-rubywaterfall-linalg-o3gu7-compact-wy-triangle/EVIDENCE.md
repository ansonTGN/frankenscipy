# frankenscipy-o3gu7 compact-WY triangle update

Agent: RubyWaterfall
Date: 2026-06-15
Target: `crates/fsci-linalg/src/lib.rs`

## Lever

`apply_compact_wy_symmetric_update` now computes the active symmetric block's lower triangle and mirrors each updated value to the upper triangle.

The arithmetic for retained cells is unchanged:

- same `V`, `T`, `A*V`, `W`, and correction construction,
- same per-cell `delta` expression for the computed triangle,
- no unsafe code and no external BLAS/LAPACK linkage.

The only removed work is recomputing the symmetric counterpart.

## Baseline

Routing baseline is in `../2026-06-15-rubywaterfall-linalg-post-grljp-reprofile/ROUTING.md`.

Same-worker RCH `vmi1152480` full-to-band replay baseline:

| Shape | scalar full replay | compact-WY panel replay | compact speedup | compact digest |
| ---: | ---: | ---: | ---: | --- |
| 256x256 | 95.825050 ms | 11.595805 ms | 8.263769x | `0x7962eb5b4444cf7a` |
| 512x512 | 239.935447 ms | 123.880705 ms | 1.936827x | `0x5eefbc9a0e7c1275` |

Transcript: `../2026-06-15-rubywaterfall-linalg-post-grljp-reprofile/baseline_compact_wy_full_to_band_replay_rch.txt`.

## After

Same-worker RCH `vmi1152480` full-to-band replay after the triangle update:

| Shape | scalar full replay | compact-WY panel replay | compact speedup | compact digest |
| ---: | ---: | ---: | ---: | --- |
| 256x256 | 13.795225 ms | 8.288877 ms | 1.664306x | `0xb609dd1775cf3068` |
| 512x512 | 128.558455 ms | 63.365260 ms | 2.028848x | `0x5d108f82fded2c63` |

Transcript: `after_compact_wy_triangle_replay_rch.txt`.

The scalar side-probe also moved materially between runs, so the conservative same-worker normalized full-replay comparison is the compact/scalar ratio: 512x512 improved from `1.936827x` to `2.028848x`. The direct compact replay time also dropped from `123.880705 ms` to `63.365260 ms`.

Narrow kernel probe after the edit on `vmi1152480`:

| Shape | scalar replay | compact-WY update | speedup | symmetry drift |
| ---: | ---: | ---: | ---: | ---: |
| 256x256 | 0.757288 ms | 0.291885 ms | 2.594474x | 0.0 |
| 512x512 | 3.398457 ms | 1.541589 ms | 2.204516x | 0.0 |

Transcript: `after_compact_wy_panel_kernel_rch.txt`.

For context, the prior kept compact-WY kernel evidence had 512x512 speedup `1.528817x` on `vmi1227854`; this pass raises the scalar-normalized kernel speedup to `2.204516x` on `vmi1152480`.

## Behavior proof

- `proof_compact_wy_triangle_rch.txt`: RCH `cargo test -j 1 -p fsci-linalg --lib compact_wy --locked -- --nocapture` passed.
- `proof_public_eigh_golden_rch.txt`: public golden digest remained `0x287a5d3679a8bc6a`.
- Full-to-band proof drift stayed within the existing tolerance contract:
  - 256x256 max abs diff `1.59161572810262442e-12`
  - 512x512 max abs diff `6.13908923696726561e-12`
- Ordering/tie behavior: unchanged public `eigh` materialized-pair golden digest.
- Floating-point scope: private compact-WY primitive changes the mirrored half to reuse the computed symmetric value; public route is not yet wired to this full-to-band replay.
- RNG: no production RNG added.

## Gates

- PASS: RCH `cargo check -j 1 -p fsci-linalg --lib --locked`.
- PASS: `ubs crates/fsci-linalg/src/lib.rs` with zero critical findings.
- BLOCKED: `cargo fmt -p fsci-linalg -- --check` reports pre-existing upstream `expm_frechet` formatting hunks in `src/bin/diff_expm_frechet.rs` and `src/lib.rs`.
- BLOCKED: RCH `cargo clippy -j 1 -p fsci-linalg --lib --locked --no-deps -- -D warnings` reports pre-existing lints at `lib.rs:3709`, `3720`, `4170`, plus upstream `expm` type complexity at `lib.rs:6271`; none are introduced by this loop.

## Score

Impact 3.0 x Confidence 3.0 / Effort 1.5 = 6.0.

Verdict: KEEP. This is a small but real safe-Rust kernel improvement inside the compact-WY full-to-band path, backed by proof and rerun timing.

## Next route

Reprofile after this keep. The next non-repeating primitive should attack the remaining full-to-band replay/reduction cost or a true band-to-tridiagonal backend; do not repeat lower-triangle matvec, SIMD spelling, worker-count retuning, or Givens replay variants.
