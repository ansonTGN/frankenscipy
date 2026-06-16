# frankenscipy-8ty4p production-aligned stage profile

Agent: RubyWaterfall
Crate: `fsci-linalg`
Base commit: `253dbb71`
Bead: `frankenscipy-8ty4p`

## Scope

`frankenscipy-8ty4p` was opened from `psn7x.4` to attack a two-stage
dense-to-band symmetric `eigh` route. The first pass found that the committed
stage-breakdown probe was not measuring the production reduction kernel: it used
the full-mirror rank-2 update, while production `symmetric_eigh_native` uses the
lower-storage rank-2 update.

The retained source change is test/probe-only: `symmetric_eigh_native_stage_breakdown_probe`
now calls `apply_symmetric_householder_trailing_rank2_lower_storage` and prints
`reduction_storage=lower`. Public `eigh`, `eigvalsh`, ordering, fallback,
floating-point production arithmetic, and RNG behavior are unchanged.

## Baseline

RCH public-route baseline on `vmi1227854`:

| n | routed native | nalgebra | speedup | digest |
|---:|---:|---:|---:|---|
| 400 | 51.532226 ms | 48.490839 ms | 0.940981x | `0x4b8334c92ce624eb` |
| 800 | 205.601725 ms | 350.079370 ms | 1.702706x | `0xad8a7e5fa1980bfb` |
| 1200 | 584.771339 ms | 1413.666684 ms | 2.417469x | `0x181b3486089d0e4a` |

Transcript: `baseline_public_native_route_rch.txt`

## Profile Correction

Original stale-profile path on `vmi1227854` used the full-mirror rank-2 update:

| n | reduction | tridiagonal_eigen | backtransform | sort |
|---:|---:|---:|---:|---:|
| 400 | 13.327532 ms | 14.074130 ms | 15.162321 ms | 0.480820 ms |
| 800 | 168.629024 ms | 39.353902 ms | 117.948428 ms | 1.584804 ms |
| 1200 | 647.430235 ms | 93.045951 ms | 292.782078 ms | 5.568104 ms |

Transcript: `baseline_stage_profile_rch.txt`

Production-aligned lower-storage profile on `vmi1227854`:

| n | reduction | tridiagonal_eigen | backtransform | sort |
|---:|---:|---:|---:|---:|
| 400 | 13.646727 ms | 10.030238 ms | 7.849740 ms | 0.507991 ms |
| 800 | 110.608626 ms | 41.520178 ms | 66.449026 ms | 2.090112 ms |
| 1200 | 357.218932 ms | 92.225694 ms | 351.013345 ms | 6.462192 ms |

Transcript: `aligned_lower_storage_stage_profile_rch.txt`

The profile still has reduction as the largest single n=1200 stage, but only by
1.0177x over backtransform. Existing two-stage public-route evidence shows direct
full-to-band wiring would repeat the known rejected path: scalar generator plus
compact replay double-reduces, and `eig_banded` currently expands back to dense
native `eigh` instead of supplying a real band-to-tridiagonal eigenvector path.

## Behavior Proof

- Public route digests stayed at `0x4b8334c92ce624eb`, `0xad8a7e5fa1980bfb`,
  and `0x181b3486089d0e4a` for n=`400/800/1200`.
- Production arithmetic is unchanged; the source edit affects only an ignored
  release profiling test.
- Ordering/tie-breaking and public golden behavior are unchanged by construction.
- RNG behavior is unchanged; the probe uses its existing deterministic generator.
- No `unsafe` and no external BLAS/LAPACK/MKL/XLA linkage were added.

## Gates

- `cargo fmt -p fsci-linalg -- --check`: passed.
- `ubs crates/fsci-linalg/src/lib.rs`: exit 0, critical issues 0.
- RCH `cargo check -j 1 -p fsci-linalg --lib --locked`: passed.
- RCH `cargo clippy -j 1 -p fsci-linalg --lib --no-deps --locked -- -D warnings`: passed.

Both RCH compile gates emitted the known dependency warning in
`fsci-fft/src/helpers.rs:58`; no new linalg warning was introduced.

## Verdict

Close `frankenscipy-8ty4p` as a completed profile/routing correction, not a
production speed keep. The immediate profile-backed successor is native symmetric
`eigh` backtransform: it is effectively tied with reduction on the corrected
n=1200 split, has no known direct-public two-stage blocker, and can be attacked
with a one-lever isomorphic reflector-replay change.
