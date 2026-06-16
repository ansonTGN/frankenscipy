# frankenscipy-psn7x.11 Rejection - Compact-WY Threshold for Banded Eigenvectors

## Target

- Bead: `frankenscipy-psn7x.11`
- Crate: `fsci-linalg`
- Profile-backed public route: `eig_banded(..., lower=true, eigvals_only=false)`
- Environment: local `cargo` + `hyperfine` with `RCH_REQUIRE_REMOTE=0` because the `ts1` RCH path was offline.

## Baseline

Command:

```text
RCH_REQUIRE_REMOTE=0 CARGO_TARGET_DIR=/data/projects/.scratch/frankenscipy-rubywaterfall-opt-20260616-1955/.local-target \
hyperfine --warmup 1 --runs 3 --show-output \
'cargo test -j 1 -p fsci-linalg --lib eig_banded_eigenvectors_perf_probe --release --locked -- --ignored --nocapture'
```

Current-head internal probe times:

| shape | bandwidth | internal times | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | `3.455166`, `4.097713`, `3.225180 ms` | `7.56017470848746598e-12` | `1.64845914696343243e-12` | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | `17.330612`, `15.976787`, `15.967399 ms` | `4.16093826061114669e-11` | `7.73070496506989002e-12` | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

Hyperfine wall time: `225.7 ms +/- 24.6 ms`.

Tighter restored-baseline run was also sampled with `--runs 7`. One Cargo/cache outlier made the wall statistic unusable (`301.6 ms +/- 248.5 ms`), but the internal probe band stayed comparable:

- 128x128 bw32: `3.738083`, `3.787337`, `4.100249`, `3.432726`, `4.373406`, `3.874792`, `3.127298`, `3.147326 ms`
- 256x256 bw32: `15.602155`, `15.726790`, `14.657057`, `16.904499`, `16.251514`, `14.481083`, `13.559389`, `12.798479 ms`

## Candidate

Temporary one-line source lever, then restored:

```text
SYMMETRIC_EIGH_COMPACT_BACKTRANSFORM_MIN_DIM: 512 -> 128
```

This enabled the existing compact-WY left-reflector backtransform for the 128x128 and 256x256 `eig_banded` eigenvector probe sizes. It was intentionally a narrow check of whether the already-proven compact backtransform should participate in the current dense/native fallback path for banded eigenvectors while the deeper dsbtrd-class compact-band primitive is still pending.

## Proof

Existing compact-WY scalar-vs-compact contract passed locally:

```text
compact_wy_backtransform_contract n=48 panel_width=4 max_abs_diff=1.11022302462515654e-15 scalar_digest=0x1db8d23e5f1bfdf5 compact_digest=0x49be718d438225e1
compact_wy_backtransform_contract n=96 panel_width=8 max_abs_diff=1.49880108324396133e-15 scalar_digest=0x9f470b07af18922e compact_digest=0x4df0ef983ffb6a82
```

Public `eig_banded` proof passed under the candidate: eigenvalue digests, max-diff tolerance, and residual tolerance were unchanged. Eigenvector bit digests changed as expected because compact-WY changes floating-point association inside the backtransform; the acceptance contract is the existing scalar-vs-compact drift proof plus public residual/orthogonality tolerance.

Candidate pre-run:

| shape | bandwidth | candidate time | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | `3.596707 ms` | `7.56017470848746598e-12` | `1.64845914696343243e-12` | `0xd6dbb9200f65bd92` | `0x75cf10c84740728e` |
| 256x256 | 32 | `16.147370 ms` | `4.16093826061114669e-11` | `7.73070496506989002e-12` | `0x09ed4d367faab431` | `0x12f20d1a7c08a1ea` |

Candidate `--runs 7` internal times:

- 128x128 bw32: `4.509214`, `3.238288`, `3.678441`, `3.844606`, `3.334832`, `4.539141`, `3.819629`, `3.795694 ms`
- 256x256 bw32: `16.833938`, `16.761341`, `14.947359`, `15.932153`, `14.229911`, `15.724379`, `13.618083`, `14.139630 ms`

Candidate hyperfine wall time: `205.0 ms +/- 8.1 ms`.

## Post-Restore Golden

The threshold was restored to `512`. Post-restore public proof:

| shape | bandwidth | post-restore time | max abs diff | residual | values digest | vectors digest |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| 128x128 | 32 | `4.009499 ms` | `7.56017470848746598e-12` | `1.64845914696343243e-12` | `0xd6dbb9200f65bd92` | `0x6cf3573b5b50c275` |
| 256x256 | 32 | `15.663367 ms` | `4.16093826061114669e-11` | `7.73070496506989002e-12` | `0x09ed4d367faab431` | `0xc32797c0d224a75a` |

Golden output artifact:

```text
3232002b0050e51cadb52cf1e728293d3e4cc5990f61cca9635e94306609fdbf  tests/artifacts/perf/2026-06-16-rubywaterfall-linalg-psn7x11-dsbtrd-queue/compact-wy-threshold-golden-output.txt
```

Ordering/tie behavior: unchanged after restore; the existing public `eig_banded` eigenvalue ordering path is active.

Floating-point contract: unchanged after restore; value/vector digests and residuals match the current route.

RNG: unchanged; this probe has no public RNG and uses deterministic fixtures.

Safety: no `unsafe`, no C BLAS/LAPACK/MKL/XLA linkage. No source retained.

## Decision

Rejected/no-ship. Score `Impact 0.0 * Confidence 4.0 / Effort 1.0 = 0.0`.

The wall-time sample looked superficially better, but the warmed internal probe bands did not establish a reliable win: 128x128 remained neutral/regressive, and 256x256 was mixed against the restored baseline. This is a threshold/replay micro-lever, not the dsbtrd-class primitive required by the bead.

Next route: do not retry compact-WY threshold changes, worker-count retuning, output materialization loop order, shifted band inverse iteration, fixed-envelope adjacent Givens, sparse maps, dense re-expansion, direct public DSBTRD fallback wiring, or full-reorthogonalized Lanczos eigenvectors. Continue in successor `frankenscipy-psn7x.12` with an isolated LAPACK-`dsbtrd`-class lower-band scalar-oracle harness: compact lower-band/envelope storage, explicit active bulge frontier, chase-order local rotations/reflectors, scalar dense-similarity oracle for `Q^T A Q = T`, Q orthogonality, D/E extraction, golden sha256, and only then public wiring.
