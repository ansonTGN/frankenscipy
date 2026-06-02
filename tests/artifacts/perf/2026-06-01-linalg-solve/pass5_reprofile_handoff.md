# Pass 5 - Re-profile And Handoff

Date: 2026-06-01T21:38:35-0400
Target bead: `frankenscipy-perf-linalg-lu-scalar-2y3wp`
Git before pass: `b04b0709`

## Mission

Re-run the current remote benchmark/profile for the dense LU scalar hotspot,
update the evidence bundle, close or reject the active bead with numbers, and
report the next ready perf bead state.

No Rust source, Cargo configuration, or production behavior was changed in this
pass.

## Fresh Remote Benchmark

Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- hyperfine --setup 'env CARGO_TARGET_DIR=/tmp/rch_target_fsci_linalg_pass5_reprofile RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p fsci-linalg --profile release-perf --bin perf_solve' --warmup 3 --runs 10 --export-json tests/artifacts/perf/2026-06-01-linalg-solve/pass5_reprofile_benchmark_rch.json '/tmp/rch_target_fsci_linalg_pass5_reprofile/release-perf/perf_solve solve 1000 1 42' '/tmp/rch_target_fsci_linalg_pass5_reprofile/release-perf/perf_solve lu_factor 1000 1 42' '/tmp/rch_target_fsci_linalg_pass5_reprofile/release-perf/perf_solve lu_solve 1000 1 42'
```

Results:

| mode | mean | stddev | median | min | max | user | system |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `solve 1000 1 42` | 118.8 ms | 4.7 ms | 118.4 ms | 114.4 ms | 129.3 ms | 91.7 ms | 26.9 ms |
| `lu_factor 1000 1 42` | 95.4 ms | 5.6 ms | 93.4 ms | 88.4 ms | 106.1 ms | 81.3 ms | 13.7 ms |
| `lu_solve 1000 1 42` | 114.0 ms | 6.3 ms | 111.3 ms | 107.5 ms | 124.9 ms | 88.8 ms | 24.8 ms |

`lu_factor` remains 80.3% of end-to-end `solve` by mean wall time in this run.

## Fresh Profile Check

Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- bash -lc 'set -o noclobber; out=tests/artifacts/perf/2026-06-01-linalg-solve/pass5_reprofile_gdb_samples.txt; bin=/tmp/rch_target_fsci_linalg_pass5_reprofile/release-perf/perf_solve; tmp=/tmp/pass5_reprofile_run_$$.out; "$bin" solve 1000 80 42 > "$tmp" 2>&1 & pid=$!; { printf "command: %s solve 1000 80 42\n" "$bin"; printf "sampler: gdb -batch -p <pid> -ex bt, 20 samples\n"; for i in $(seq 1 20); do printf "===SAMPLE %s===\n" "$i"; gdb -batch -p "$pid" -ex bt 2>&1 | sed -n "1,24p"; sleep 0.25; done; wait "$pid"; printf "===RUN OUTPUT===\n"; cat "$tmp"; } > "$out"'
```

Results:

- 20 samples captured.
- `gauss_step|array_axcpy|axcpy_uninit|blas_uninit` appeared in 18/20 samples.
- `condition_diagnostics_with_assumption` appeared in 15/20 samples.
- `fast_rcond_from_lu` appeared in 1/20 samples.
- Text match count for `gauss_step|array_axcpy|axcpy_uninit|blas_uninit`: 53.
- Run output: `{"mode":"solve","n":1000,"repeats":80,"total_ms":11552.155,"per_call_ms":144.4019,"checksum":-7.875862e1}`.

Conclusion: the remaining hotspot is still nalgebra 0.34.2 scalar LU work,
especially `gauss_step -> axpy/axcpy_uninit -> array_axcpy` with stack tops in
`core::clone`, floating-point `add`, and index arithmetic.

## Golden Proof

Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- bash -lc 'set -o noclobber; /tmp/rch_target_fsci_linalg_pass5_reprofile/release-perf/perf_solve golden > tests/artifacts/perf/2026-06-01-linalg-solve/golden/golden_pass5_reprofile.txt; sha256sum tests/artifacts/perf/2026-06-01-linalg-solve/golden/golden_pass5_reprofile.txt tests/artifacts/perf/2026-06-01-linalg-solve/golden/golden_blas_lapack_pass3_before.txt'
```

Result:

```text
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-01-linalg-solve/golden/golden_pass5_reprofile.txt
5809995418488c93cc66dc6f2dc01a0d5fd8e2d8faab6f9a7c44241e99025bdd  tests/artifacts/perf/2026-06-01-linalg-solve/golden/golden_blas_lapack_pass3_before.txt
```

Because no production code or Cargo configuration changed, ordering,
tie-breaking, floating-point grouping, RNG seeds, certificates, warnings, and
backward-error bits are unchanged by construction. The fresh golden output also
matches the retained production checksum.

## Five-pass Summary

| pass | lever evaluated | decision | score / proof |
| --- | --- | --- | --- |
| 1 | dependency/config backend probe | rejected | config-only score 0.0; `matrixmultiply`, `simba/wide`, and `wide/std` were already active and did not alter nalgebra LU |
| 2 | residual clone split | rejected | candidate changed observable `backward_error` golden bits; confidence 0 |
| 3 | LAPACK backend swap | rejected | isolated subset was faster but production replacement scored 0.6 due link portability, provider complexity, and unproven full CASP/rcond/backward-error contract |
| 4 | safe Rust blocked/panel LU replacement | rejected | production replacement scored 0.8; harness-only probe changed golden bits and did not prove production integration |
| 5 | re-profile and closeout | rejected/closed | final profile still points to scalar nalgebra LU; no safe one-lever production candidate remains above score 2.0 |

## Final Decision

Close `frankenscipy-perf-linalg-lu-scalar-2y3wp` as a completed rejection for
this campaign. The hotspot is real and still present, but all evaluated
production-safe one-lever paths either failed the bit-identical behavior proof
or scored below the required `2.0` threshold.

Close command:

```bash
br close frankenscipy-perf-linalg-lu-scalar-2y3wp --reason "5-pass profile-driven campaign complete: final rch hyperfine solve 118.8±4.7 ms, lu_factor 95.4±5.6 ms, lu_solve 114.0±6.3 ms; gdb samples still show nalgebra gauss_step/array_axcpy in 18/20 samples; all production levers scored <2.0 or failed golden proof, so no safe code/config change was kept."
```

Sync command:

```bash
br sync --flush-only
```

Sync result: `Nothing to export (no dirty issues)`.

## Recommended Next Work

Do not keep pushing on this same bead without relaxing one of the constraints.
The next useful optimization work should be a new, separately scoped bead with
one of these explicit targets:

- LAPACK provider integration bead: establish portable provider selection and a
  full production proof for `solve`, `lu_factor`, `lu_solve`, `rcond`, warnings,
  certificates, and `backward_error`.
- Floating-point-contract bead: decide whether a non-bit-identical blocked LU is
  allowed under tolerance-based parity for this exact scenario; if yes, define
  the oracle and golden policy before implementation.
- Adjacent hotspot bead: profile a different `[perf]` target rather than the
  same scalar LU path.

After closeout, `br ready --json` returned:

```json
[]
```

No next ready perf bead is currently available from `br ready --json`.
