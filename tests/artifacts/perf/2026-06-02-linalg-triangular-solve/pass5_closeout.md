# Pass 5 - Re-profile And Closeout

Date: 2026-06-02T03:03:00-0400
Agent: OliveSnow
Target bead: `frankenscipy-perf-linalg-directlu-triangular-solve-v82ao`

## Mission

Re-profile the kept Pass 4 cached-rcond state after the bottleneck shifted, prove
the retained behavior artifacts still verify, and close the bead without adding a
second lever.

## Re-profile

Artifacts:

- `pass5_reprofile_rch.json`
- `pass5_final_rch.json`

Command shape:

```bash
RCH_FORCE_REMOTE=1 rch exec -- hyperfine \
  --setup 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_tri_pass5_reprofile_20260602a RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve --locked' \
  --warmup 3 \
  --runs 10 \
  --export-json tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass5_reprofile_rch.json \
  '/tmp/rch_target_fsci_linalg_tri_pass5_reprofile_20260602a/release-perf/perf_solve lu_factor 1000 1 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass5_reprofile_20260602a/release-perf/perf_solve lu_solve 1000 1 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass5_reprofile_20260602a/release-perf/perf_solve lu_solve_cached 1000 200 42' \
  '/tmp/rch_target_fsci_linalg_tri_pass5_reprofile_20260602a/release-perf/perf_solve solve 1000 1 42'
```

`rch exec` warned that `hyperfine` is a non-compilation wrapper command. The
setup build is crate-scoped and the run still produced the retained JSON
artifact. A direct remote `cargo run` probe is recorded below as traceable remote
execution evidence.

| workload | mean | stddev | median | min | max |
| --- | ---: | ---: | ---: | ---: | ---: |
| `lu_factor 1000 1 42` | 115.887 ms | 5.874 ms | 114.283 ms | 108.188 ms | 125.192 ms |
| `lu_solve 1000 1 42` | 115.406 ms | 7.247 ms | 115.050 ms | 106.691 ms | 125.993 ms |
| `lu_solve_cached 1000 200 42` | 160.426 ms | 6.491 ms | 158.477 ms | 153.604 ms | 175.787 ms |
| `solve 1000 1 42` | 125.587 ms | 9.937 ms | 127.674 ms | 112.520 ms | 140.985 ms |

Cached solve closeout rate: `160.426 ms / 200 = 0.802 ms/call`, consistent with
Pass 4's `0.796 ms/call`.

The confirmatory `pass5_final_rch.json` run measured:

| workload | mean | stddev | median |
| --- | ---: | ---: | ---: |
| `solve 1000 1 42` | 120.892 ms | 5.405 ms | 121.337 ms |
| `lu_factor 1000 1 42` | 116.554 ms | 6.721 ms | 114.948 ms |
| `lu_solve 1000 1 42` | 119.556 ms | 5.528 ms | 117.340 ms |
| `lu_solve_cached 1000 200 42` | 159.060 ms | 9.863 ms | 157.807 ms |

Confirmatory cached solve rate: `159.060 ms / 200 = 0.795 ms/call`.

## Direct Remote Probe

Direct command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- cargo run --quiet -p fsci-linalg --profile release-perf --bin perf_solve --locked -- lu_solve_cached 1000 200 42
```

Worker: `vmi1149989`

Observed JSON line:

```json
{"mode":"lu_solve_cached","n":1000,"repeats":200,"total_ms":41.096,"per_call_ms":0.2055,"checksum":-1.968966e2}
```

This worker result is not compared against the hyperfine host because the
hardware differs. It confirms the kept state executes remotely through `rch` and
preserves the canonical cached checksum value.

## Golden / Isomorphism

Retained Pass 4 artifacts:

- `pass3_cached_rcond_golden_after.txt`
- `pass3_cached_rcond_golden_after.sha256`
- `pass3_cached_rcond_lu_solve_cached_after.txt`
- `pass3_cached_rcond_lu_solve_cached_after.sha256`

Checks passed:

```text
tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3_cached_rcond_golden_after.txt: OK
tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3_cached_rcond_lu_solve_cached_after.txt: OK
```

The retained `perf_solve golden` sha256 remains:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd
```

The retained cached `lu_solve_cached 1000 200 42` checksum sha256 remains:

```text
999b301ff7191ddd3eacb3e1379c981693484a32889e7566ec812b6c2c9afa35
```

Ordering, tie-breaking, solution floating-point order, and deterministic seed use
remain the Pass 4 state. Pass 4 moved an existing rcond estimate from repeated
`lu_solve` calls into the immutable LU factorization object; Pass 5 made no code
change.

## Shifted Bottleneck

Pass 4's post-edit profile remains the current evidence:

```text
total_sample_blocks=30
attached_sample_blocks=13
fast_rcond_from_lu=0
lu_solve_or_nalgebra_lu_solve=13
lower_triangular_solve_frames=8
upper_triangular_solve_frames=5
```

The next profile-backed target should start from nalgebra final `LU::solve`
triangular internals or a scoped public API that can preserve bitwise solution
and `backward_error` output.

## Validation

Passed:

- `jq empty tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass5_reprofile_rch.json`
- `jq empty tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass5_final_rch.json`
- `sha256sum -c tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3_cached_rcond_golden_after.sha256`
- `sha256sum -c tests/artifacts/perf/2026-06-02-linalg-triangular-solve/pass3_cached_rcond_lu_solve_cached_after.sha256`

No code changed in Pass 5, so no new Rust validation was required beyond the
Pass 4 rch check, focused test, and clippy evidence.

## Verdict

Close `frankenscipy-perf-linalg-directlu-triangular-solve-v82ao` as completed.
The kept cached-rcond lever scores `15.0` and remains valid after re-profile.
