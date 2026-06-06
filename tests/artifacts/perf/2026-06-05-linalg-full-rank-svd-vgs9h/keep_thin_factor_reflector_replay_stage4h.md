# Keep: Thin-SVD Factor Reflector Replay Stage 4h

Bead: `frankenscipy-z65tz`

## Profile Target

The current private `1024x512` Golub-Kahan reducer remained the active
profile-backed linalg target after the Stage 4g parallel-left trial rejection.
Stage 4h attacks the next data-movement-heavy part of the private SVD pipeline:
thin factor assembly after the bidiagonal SVD.

Fresh baseline context, RCH `ts1`:

- reducer elapsed: `198.205464 ms`
- reducer digest: `0x90cdd3f8f71ed2c1`
- first diagonal: `-1.00455335940616146e3`
- last diagonal: `-6.45492359226604862e1`

## Lever

`deterministic_thin_svd_from_reduction` now replays stored reflectors directly
over the thin factors:

- left reflectors apply in reverse order to the bidiagonal SVD's thin `U`
- right reflectors apply in reverse order to `Vt`

The old dense path materialized full `Q^T` and `V`, then multiplied:
`Q * U_bidiag` and `Vt_bidiag * V^T`. The dense path remains only as a private
reference/probe helper so the proof and benchmark measure one lever.

Public `svd`, `svdvals`, `lstsq`, `pinv`, rank/rcond thresholds, certificates,
error classes, ordering, tie-breaking, and RNG behavior are unchanged because
the deterministic bidiagonal route is still private.

## Behavior Proof

RCH `ts1`:

```text
cargo test -p fsci-linalg --release --lib thin_bidiag_reflector_replay_matches_dense_product_reference --locked -- --nocapture
```

Result: passed. Shapes `9x5`, `17x8`, and `64x32` compare direct replay against
the dense-product reference.

Proof obligations:

- singular values: bit-identical
- `U` max drift: bounded by `1e-11`
- `Vt` max drift: bounded by `1e-11`
- reconstruction: below `1e-9`
- `U` and `Vt` orthogonality: below `1e-12`

RCH focused thin-SVD tests also passed:

```text
cargo test -p fsci-linalg --release --lib thin_bidiag_svd --locked -- --nocapture
```

Public golden-output SHA-256 stayed unchanged:

```text
1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
```

## Rebench

RCH `ts1`:

```text
cargo test -p fsci-linalg --release --lib thin_bidiag_factor_replay_perf_probe --locked -- --ignored --nocapture
```

Same precomputed bidiagonal SVD for both paths:

- dense factor products: `612.515072 ms`
- direct reflector replay: `254.719775 ms`
- speedup: `2.404662x`
- reduction digest: `0x90cdd3f8f71ed2c1`
- `U` max abs diff: `5.10702591327572009e-15`
- `Vt` max abs diff: `2.33146835171282873e-15`
- dense digest: `0x6e44a30879443520`
- replay digest: `0x412adbf362e4362b`

The thin-factor digest changes because floating-point operation ordering differs
inside private unwired factor assembly, but the mathematical contract and public
golden payload are unchanged.

## Validation

- `cargo fmt -p fsci-linalg --check`
- `git diff --check -- crates/fsci-linalg/src/lib.rs`
- RCH `cargo check -p fsci-linalg --lib --locked`
- RCH focused proof and thin-SVD tests above
- RCH `cargo clippy -p fsci-linalg --all-targets --no-deps --locked -- -D warnings`
- `ubs crates/fsci-linalg/src/lib.rs`: zero criticals

Full dependency clippy is currently blocked by peer-owned `fsci-fft`
mixed-radix `manual_is_multiple_of` lints. The linalg no-deps gate is clean.

## Decision

Keep.

Score: `3.6 = Impact 2.4 * Confidence 4.5 / Effort 3.0`.

Next primitive: persistent packed panel buffers or a true two-stage
communication-avoiding bidiagonal reducer before public wiring.
