# LU U-Panel Trailing-Update Evidence

Target: profile-backed `fsci-linalg` LU/solve hotspot under the no-gaps linalg campaign. No ready `[perf]` bead was available; the active parent is `frankenscipy-8l8r1`. BronzeDove held `.beads/**` and `.skill-loop-progress.md` reservations during implementation, so the code/evidence reservation was limited to `fsci-linalg` and this artifact directory.

Primitive: communication-avoiding LU data movement. This applies the alien-graveyard CA-LU/CA-QR lesson to the existing blocked safe-Rust LU kernel by reusing each solved U panel across row tiles before advancing to the next panel.

## Lever

One lever only: in `lu_factor_blocked` and `lu_factor_blocked_f32`, swap the trailing-update tile traversal from row-block outer order to U-panel outer order:

- before: `i0` row tile outer, `j0` column/U panel inner
- after: `j0` column/U panel outer, `i0` row tile inner

The per-output reduction order over `p in k..kb`, pivot scan, tie behavior, row swaps, panel solve, fallback branches, and RNG-free execution are unchanged.

## Benchmarks

Criterion command:

`rch exec -- cargo bench -p fsci-linalg --bench linalg_bench solve -- --sample-size 10 --measurement-time 2`

Same-worker comparison on `vmi1152480`:

| Case | Baseline `e0dd8e75` | After U-panel traversal | Delta |
| --- | ---: | ---: | ---: |
| `solve/1000x1000 mixed` median | 65.476 ms | 45.559 ms | 1.437x faster |
| `solve/1000x1000 f64` median | 97.844 ms | 81.141 ms | 1.206x faster |

Artifacts:

- `baseline_solve_1000_vmi1152480_base_e0dd8e75_rch.txt`
- `after_solve_1000_u_panel_rch.txt`

Score: Impact 3.0 x Confidence 4.0 / Effort 2.0 = 6.0, keep.

## Isomorphism Proof

The tile visitation order changes only between independent output tiles. For every updated `a[ii, jj]`, both kernels still compute:

`a[ii, jj] -= sum_{p=k..kb-1} a[ii, p] * a[p, jj]`

The summation order for each scalar output is identical, so floating-point roundoff is preserved per output. Pivot search still uses the same strict comparison, preserving ties. Row swaps and panel divisions execute before the trailing update exactly as before. There is no RNG in this path.

Golden payload:

- before payload sha256: `5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd`
- after payload sha256: `5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd`

Unit golden:

- `flat_lu_golden_digest=0x2fc8ed294ef0427c`
- `proof_flat_lu_golden_after_rch.txt`

Mixed-precision behavior proof:

- `cargo test -j 1 -p fsci-linalg --release --lib lu_solve_mixed_precision_matches_f64_and_falls_back -- --nocapture`
- artifact: `proof_mixed_after_rch.txt`

## Quality Gates

Passed:

- `rch exec -- cargo check -j 1 -p fsci-linalg --lib`
- `git diff --check -- crates/fsci-linalg/src/lib.rs`
- `cargo test -j 1 -p fsci-linalg --release --lib lu_solve_mixed_precision_matches_f64_and_falls_back -- --nocapture`
- `cargo test -j 1 -p fsci-linalg --release --lib flat_lu_golden_digest -- --include-ignored --nocapture`

Recorded blockers outside this lever:

- `cargo clippy -j 1 -p fsci-linalg --lib -- -D warnings` fails in dependency `fsci-fft` at `crates/fsci-fft/src/transforms.rs:2780` (`manual_is_multiple_of`), covered by existing quality lint work.
- `cargo clippy -j 1 -p fsci-linalg --lib --no-deps -- -D warnings` finds pre-existing SRHT lint at `crates/fsci-linalg/src/lib.rs:3929`, authored before this LU lever.
- `cargo fmt --check -p fsci-linalg` reports unrelated pre-existing formatting drift in probe/bin files and non-LU regions.
- `ubs crates/fsci-linalg/src/lib.rs` completed with no critical findings and pre-existing warnings recorded in `ubs_fsci_linalg_lib.txt`.
