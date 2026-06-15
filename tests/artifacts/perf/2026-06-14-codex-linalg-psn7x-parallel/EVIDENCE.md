# frankenscipy-psn7x parallel rank-2 subprobe

Status: REJECTED

## Target

- Bead: `frankenscipy-psn7x`
- Candidate: per-Householder `thread::scope` parallelization of the staged native `symmetric_eigh_native` rank-2 trailing update.
- Source result: restored to `origin/main`; no source change is kept from this subprobe.

## Baseline

Command:

```text
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 rch exec -- env CARGO_BUILD_JOBS=1 cargo test -j 1 -p fsci-linalg --release --locked --lib symmetric_eigh_native -- --include-ignored --nocapture
```

Artifact: `baseline_rank2_native_timing_rch.txt`

Worker: `vmi1152480`

Results:

- n=400: native 139.6 ms | nalgebra 99.4 ms | ratio 0.71x
- n=800: native 1098.3 ms | nalgebra 563.7 ms | ratio 0.51x
- n=1200: native 3510.1 ms | nalgebra 1867.2 ms | ratio 0.53x

## Proof

Command:

```text
RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1152480 RCH_TEST_SLOTS=1 rch exec -- env CARGO_BUILD_JOBS=1 cargo test -j 1 -p fsci-linalg --release --locked --lib symmetric_rank2_parallel_matches_serial_bits -- --nocapture
```

Artifact: `proof_rank2_parallel_bits_rch.txt`

Result: passed on `vmi1167313` after RCH selected a different worker. The forced one-worker and four-worker rank-2 update produced bit-identical `p`, `w`, and matrix entries on the deterministic 530x530 fixture.

## Benchmark

Command:

```text
RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1152480 RCH_TEST_SLOTS=1 rch exec -- env CARGO_BUILD_JOBS=1 cargo test -j 1 -p fsci-linalg --release --locked --lib symmetric_eigh_native -- --include-ignored --nocapture
```

Artifact: `after_parallel_rank2_timing_vmi1152480_rch.txt`

Worker: `vmi1152480`

Results:

- n=400: native 115.9 ms | nalgebra 65.4 ms | ratio 0.56x
- n=800: native 731.5 ms | nalgebra 542.8 ms | ratio 0.74x
- n=1200: native 6047.8 ms | nalgebra 2311.9 ms | ratio 0.38x

## Verdict

Reject. The same-worker n=1200 route regressed from 3510.1 ms to 6047.8 ms, so the per-step thread-spawn approach is not a keeper despite smaller-size noise. Score: `Impact 0.0 * Confidence 4.0 / Effort 2.0 = 0.0`.

Next route: do not tune this per-step spawning family. Attack a structurally different primitive: persistent scoped work over multiple Householder steps, parallel eigenvector back-transform, or a divide-and-conquer / MRRR-style tridiagonal eigensolver so the sequential tridiagonal stage stops dominating.
