# frankenscipy-psn7x.16 Evidence - Rejected Progressive Bulge Chase

## Target

- Bead: `frankenscipy-psn7x.16`
- Crate: `fsci-linalg`
- Profile-backed route: successor to `frankenscipy-psn7x.15`, which kept the fused compact-band frontier chase harness with local dense-vs-compact speedups up to `2.77x`.
- Environment: local cargo + hyperfine with `RCH_REQUIRE_REMOTE=0`; `ts1` RCH path remains offline.

## Baseline

Baseline before source edits reused the committed fused frontier probe:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target hyperfine --warmup 1 --runs 3 --show-output 'cargo test -j 1 -p fsci-linalg --lib lower_band_fused_frontier_chase_perf_probe --release --locked -- --ignored --nocapture'
```

Baseline ranges:

| shape | bandwidth | dense ms | compact ms | speedup range |
| --- | ---: | ---: | ---: | ---: |
| 128x128 | 32 | `12.060643`-`13.441729` | `4.728797`-`4.836040` | `2.499849x`-`2.842526x` |
| 256x256 | 32 | `23.919073`-`32.567403` | `14.636352`-`15.031179` | `1.620745x`-`2.166657x` |
| 512x512 | 32 | `46.375556`-`57.367937` | `27.181342`-`28.833531` | `1.706154x`-`1.989626x` |

## Candidate

Candidate source was tested and restored before commit.

The candidate chose adjacent Givens rotations column-by-column, attempted to annihilate all entries below the first subdiagonal, emitted D/E from the active lower-envelope storage, and replayed Q metadata against the dense oracle.

Two bounded-envelope attempts failed behavior before timing:

- `bandwidth + 2` overflowed the active envelope at `(8,1)` for the 18x18 fixture.
- `2*bandwidth + 2` overflowed at `(16,5)`.
- `4*bandwidth + 4` passed 18x18 and 37x37, then overflowed at `(60,7)` on the 64x64 fixture.

The full active lower-envelope version passed behavior but regressed performance.

## Proof

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target cargo test -j 1 -p fsci-linalg --lib lower_band_progressive_bulge_chase_matches_dense_oracle --release --locked -- --nocapture
```

Passing proof output for the full active lower-envelope candidate:

```text
lower_band_progressive_bulge n=18 bandwidth=4 cols=7 rotations=108 transformed_drift=3.55271367880050093e-14 tail_max=5.56253812826202583e-17 eigenvector_drift=2.32452945780892151e-16 compact_digest=0xff38cfc5a4a36144
lower_band_progressive_bulge n=37 bandwidth=8 cols=13 rotations=566 transformed_drift=1.27897692436818033e-13 tail_max=6.50284855896919986e-17 eigenvector_drift=3.60822483003175876e-16 compact_digest=0x89c0b1fb89f86f5a
lower_band_progressive_bulge n=64 bandwidth=12 cols=17 rotations=1818 transformed_drift=1.76257452867983101e-13 tail_max=5.37226967554113574e-17 eigenvector_drift=4.44089209850062616e-16 compact_digest=0x0a455627d3c98852
```

No golden output was kept because the lever failed the performance gate and the source was restored.

## Rebench

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target hyperfine --warmup 1 --runs 3 --show-output 'cargo test -j 1 -p fsci-linalg --lib lower_band_progressive_bulge_chase_perf_probe --release --locked -- --ignored --nocapture'
```

Hyperfine wall time: `201.5 ms +/- 26.2 ms` over 3 runs.

Internal candidate ranges:

| shape | bandwidth | rotations | dense ms | compact ms | speedup range | tail max | compact digest |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 64x64 | 12 | 1818 | `2.136828`-`2.441153` | `4.268725`-`4.702777` | `0.459932x`-`0.571869x` | `5.37226967554113574e-17` | `0xb9be7421c7c60556` |
| 96x96 | 16 | 4230 | `2.698712`-`3.396833` | `5.297915`-`6.679100` | `0.404053x`-`0.579251x` | `1.06825659154602612e-16` | `0xf341fad64d8e1c72` |
| 128x128 | 32 | 7812 | `4.429159`-`6.678499` | `7.863173`-`10.811959` | `0.485465x`-`0.617696x` | `1.02195115377834354e-16` | `0xc3d5d42c1529e8d1` |

## Score

- Impact: `0.0` (behavior proof passed, but the full lower-envelope candidate regressed all measured sizes).
- Confidence: `4.0` (proof passed and local hyperfine consistently showed regression).
- Effort: `1.0`.
- Score: `0.0`.

Verdict: REJECTED / NO-SHIP. Source restored before commit.

## Next Primitive

Do not retry full active lower-envelope storage for progressive chasing. The next route should use an explicit bounded bulge queue/ring-buffer representation that stores only the moving fill frontier, or a proper LAPACK-`DSBTRD`-style rotation packet schedule that applies `DLAR2V/DLARTV`-class batched rotations to compact diagonals without full-envelope indexing overhead.
