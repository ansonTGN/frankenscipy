# fsci-special incomplete elliptic powi-square feasibility

Bead: `frankenscipy-perf-special-elliptic-vector-733cd`

## Profile-backed target

Fresh broad Criterion profile, `fsci-special` on RCH worker `vmi1156319`:

- `special_incomplete_elliptic/ellipkinc_broadcast_m/scalar_phi_over_m_vec`: mean `800.69 ns`
- `special_incomplete_elliptic/ellipeinc_pairwise_vec/phi_vec_m_vec`: mean `756.99 ns`
- `special_erf/erf/3`: mean `730.80 ns`
- `special_erfc/erfc/-3`: mean `723.03 ns`

The first two rows share the incomplete elliptic scalar quadrature kernels through vector/broadcast dispatch, so the first lever tested the per-node `sin().powi(2)` work in `gauss_legendre_elliptic_f` and `gauss_legendre_elliptic_e`.

## Baseline

Focused baseline command:

```text
RCH_FORCE_REMOTE=1 CARGO_TARGET_DIR=/data/tmp/cargo-target-frankenscipy-olivesnow-special-elliptic-baseline rch exec -- cargo bench -p fsci-special --bench special_bench --locked -- special_incomplete_elliptic --warm-up-time 1 --measurement-time 3 --sample-size 15
```

Worker `vmi1153651`:

- `ellipkinc_scalar/phi0.785_m0.5`: mean `322.23 ns`
- `ellipeinc_scalar/phi0.785_m0.5`: mean `264.09 ns`
- `ellipkinc_scalar/phi1.047_m0.9`: mean `314.08 ns`
- `ellipeinc_scalar/phi1.047_m0.9`: mean `305.87 ns`
- `ellipkinc_broadcast_m/scalar_phi_over_m_vec`: mean `976.88 ns`
- `ellipeinc_pairwise_vec/phi_vec_m_vec`: mean `857.22 ns`

Golden before:

- clean output sha256 `baa88a303e1e2ff4e204a57de38ec782e002e7f1f0077a7956cd9bc5367eccb6`

## Candidate

Temporary one-lever candidate:

- replace `t_pos.sin().powi(2)` and `t_neg.sin().powi(2)` with explicit same-sine temporaries multiplied by themselves.

RCH after run, worker `vmi1156319`:

- `ellipkinc_scalar/phi0.785_m0.5`: mean `275.87 ns`
- `ellipeinc_scalar/phi0.785_m0.5`: mean `231.35 ns`
- `ellipkinc_scalar/phi1.047_m0.9`: mean `258.94 ns`
- `ellipeinc_scalar/phi1.047_m0.9`: mean `236.17 ns`
- `ellipkinc_broadcast_m/scalar_phi_over_m_vec`: mean `894.07 ns`
- `ellipeinc_pairwise_vec/phi_vec_m_vec`: mean `759.18 ns`

Golden after:

- clean output sha256 `baa88a303e1e2ff4e204a57de38ec782e002e7f1f0077a7956cd9bc5367eccb6`
- before/after golden files are byte-identical

Final tracked harness refresh after replacing panic-based exits with structured errors:

- RCH `cargo run --quiet -p fsci-special --example perf_special --locked -- golden-elliptic` on worker `vmi1227854`
- clean final output sha256 `baa88a303e1e2ff4e204a57de38ec782e002e7f1f0077a7956cd9bc5367eccb6`
- before/final golden files are byte-identical

The tracked example was rerun against the current clippy-clean `elliptic.rs` state:

- RCH `cargo run --quiet -p fsci-special --example perf_special --locked -- golden-elliptic` on worker `vmi1227854`
- clean final output sha256 `baa88a303e1e2ff4e204a57de38ec782e002e7f1f0077a7956cd9bc5367eccb6`
- before/final golden files are byte-identical

## Control

After restoring the original `powi(2)` expression, a control run was taken to test whether the apparent speedup survived worker variance.

Control command:

```text
RCH_FORCE_REMOTE=1 CARGO_TARGET_DIR=/data/tmp/cargo-target-frankenscipy-olivesnow-special-elliptic-control rch exec -- cargo bench -p fsci-special --bench special_bench --locked -- special_incomplete_elliptic --warm-up-time 1 --measurement-time 3 --sample-size 15
```

Worker `vmi1149989`:

- `ellipkinc_scalar/phi0.785_m0.5`: mean `134.50 ns`
- `ellipeinc_scalar/phi0.785_m0.5`: mean `131.79 ns`
- `ellipkinc_scalar/phi1.047_m0.9`: mean `139.26 ns`
- `ellipeinc_scalar/phi1.047_m0.9`: mean `137.01 ns`
- `ellipkinc_broadcast_m/scalar_phi_over_m_vec`: mean `450.52 ns`
- `ellipeinc_pairwise_vec/phi_vec_m_vec`: mean `440.50 ns`

The restored original is materially faster than the candidate run on this control worker. The candidate is therefore below the required keep threshold because the measured improvement is not attributable to the lever with enough confidence.

## Isomorphism proof

- Ordering: retained production code is restored to the original loop and contribution order.
- Tie-breaking: no comparisons or tie-breaking logic participate in the rejected lever.
- Floating point: the temporary candidate produced bit-identical canonical public outputs, but the retained code is the original `powi(2)` path; no floating-point operation order changed in the committed production source.
- RNG: none.
- Golden output: before and after candidate sha256 matched exactly at `baa88a303e1e2ff4e204a57de38ec782e002e7f1f0077a7956cd9bc5367eccb6`.

## Validation

- RCH `cargo check -p fsci-special --example perf_special --locked` passed on worker `vmi1149989`.
- RCH `cargo clippy -p fsci-special --example perf_special --locked -- -D warnings` passed on worker `vmi1153651`.
- `cargo fmt -p fsci-special --check` passed.
- `git diff --check` passed for the touched progress, harness, and artifact surfaces.
- `ubs crates/fsci-special/examples/perf_special.rs crates/fsci-special/src/elliptic.rs tests/artifacts/perf/2026-06-02-special-elliptic-vector/powi_square_feasibility.md .skill-loop-progress.md` exited `0`; it reported `0` critical findings and warning inventory in existing `elliptic.rs` test/assert/direct-index surfaces.

## Verdict

Rejected. Score `0.0` because confidence is zero after the restored-original control run. No production optimization is kept for this lever.
