# frankenscipy-8l8r1.53 Pass 1 - baseline and compact-WY contract

Date: 2026-06-08
Skill loop: `/repeatedly-apply-skill` Pass 1 of 5 applying `/extreme-software-optimization`
Bead: `frankenscipy-8l8r1.53`
Assignee confirmed: `BlackThrush`

## Scope

This pass captured the current behavior anchors and wrote the contract for a
future true compact-WY packed-panel bidiagonalization far-update lever. No source
files, `.beads`, or `.skill-loop-progress.md` were edited.

Allowed artifact directory:

```text
tests/artifacts/perf/2026-06-08-linalg-compact-wy-panel/
```

## Bead State

Command:

```text
br show frankenscipy-8l8r1.53 --json
```

Result: `status=in_progress`, `assignee=BlackThrush`, `priority=1`,
`labels=["linalg","no-gaps","perf"]`.

Raw output:

```text
tests/artifacts/perf/2026-06-08-linalg-compact-wy-panel/br_show_frankenscipy-8l8r1.53.json
```

## Commands

Reduction probe:

```text
RCH_FORCE_REMOTE=1 RCH_WORKER=vmi1153651 rch exec -- cargo test -p fsci-linalg --release --lib --locked bidiag_large_reduction_perf_probe -- --ignored --nocapture
```

Public golden probe:

```text
RCH_FORCE_REMOTE=1 RCH_WORKER=vmi1153651 rch exec -- cargo test -p fsci-linalg --release --lib --locked public_svd_lstsq_pinv_golden_payload -- --nocapture
```

Payload extraction:

```text
awk '/PUBLIC_SVD_LSTSQ_PINV_GOLDEN_BEGIN/{keep=1} keep{print} /PUBLIC_SVD_LSTSQ_PINV_GOLDEN_END/{keep=0}' public_svd_lstsq_pinv_golden_payload_rch.raw.txt > public_svd_lstsq_pinv_golden_payload.txt
sha256sum public_svd_lstsq_pinv_golden_payload.txt > public_svd_lstsq_pinv_golden_payload.sha256
```

## RCH Caveat

Both requested commands were invoked through `rch exec` with
`RCH_FORCE_REMOTE=1` and `RCH_WORKER=vmi1153651`, but RCH reported local fallback:

```text
[RCH] local (no admissible workers: critical_pressure=2,insufficient_slots=3,hard_preflight=4,active_project_exclusion=1)
[RCH] local (no admissible workers: critical_pressure=2,insufficient_slots=3,hard_preflight=1,active_project_exclusion=1)
```

Therefore the elapsed time below is a current-source, RCH-invoked fallback timing
and is non-gating for keep/reject decisions. It proves the current digest and
public golden payload on the local fallback path, but it is not comparable to the
prior same-worker `vmi1153651` baseline.

## Remote Baseline Addendum

Later in the same `.53` run, RCH accepted remote jobs and produced gating
current-source anchors:

```text
worker=vmi1156319
shape=1024x512
elapsed_ms=494.483766
digest=0x90cdd3f8f71ed2c1
first_diagonal=-1.00455335940616146e3
last_diagonal=-6.45492359226604862e1
artifact=tests/artifacts/perf/2026-06-08-linalg-compact-wy/baseline_bidiag_large_reduction_perf_probe_rch_attempt5_vmi1153651.txt
```

```text
worker=vmi1156319
public_golden_sha256=1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
artifact=tests/artifacts/perf/2026-06-08-linalg-compact-wy/baseline_public_svd_lstsq_pinv_golden_payload_rch.txt
```

The baseline artifact name includes the requested `RCH_WORKER=vmi1153651`, but
the RCH log selected `vmi1156319`. The worker named in the log is the
comparison anchor for these two remote baseline probes.

## Baseline Numbers

Reduction probe raw output:

```text
tests/artifacts/perf/2026-06-08-linalg-compact-wy-panel/bidiag_large_reduction_perf_probe_rch.raw.txt
```

Extracted current-source fallback result:

```text
shape=1024x512
elapsed_ms=245.760722
digest=0x90cdd3f8f71ed2c1
first_diagonal=-1.00455335940616146e3
last_diagonal=-6.45492359226604862e1
exit_status=0
worker=local fallback, no admissible RCH worker
```

Comparison anchor supplied by the bead context:

```text
same_worker=vmi1153651
current_golub_kahan_baseline_ms=431.652279
current_golub_kahan_digest=0x90cdd3f8f71ed2c1
rejected_stage1_ms=10628.935808
rejected_stage1_speedup=0.040611x
```

The digest matches the prior current Golub-Kahan digest, so the fallback run is
on the expected reducer output state. The timing is still non-gating because the
worker did not match.

Public golden raw output:

```text
tests/artifacts/perf/2026-06-08-linalg-compact-wy-panel/public_svd_lstsq_pinv_golden_payload_rch.raw.txt
```

Extracted payload and SHA:

```text
tests/artifacts/perf/2026-06-08-linalg-compact-wy-panel/public_svd_lstsq_pinv_golden_payload.txt
tests/artifacts/perf/2026-06-08-linalg-compact-wy-panel/public_svd_lstsq_pinv_golden_payload.sha256
```

```text
public_golden_sha256=1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
expected_public_golden_sha256=1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
exit_status=0
worker=local fallback, no admissible RCH worker
```

## Current Reducer Shape

Source references read:

- `crates/fsci-linalg/src/lib.rs:6352` builds scalar Householder reflectors.
- `crates/fsci-linalg/src/lib.rs:6383` applies a left reflector column-by-column.
- `crates/fsci-linalg/src/lib.rs:6477` applies a right reflector with row dot workspace.
- `crates/fsci-linalg/src/lib.rs:6526` contains the existing fused rank-k update helper, currently a scalar-equivalent two-term update primitive.
- `crates/fsci-linalg/src/lib.rs:7335` is the active Golub-Kahan bidiagonal reduction loop.
- `crates/fsci-linalg/src/lib.rs:13428` computes the reduction digest over diagonal, superdiagonal, and reflector taus.
- `crates/fsci-linalg/src/lib.rs:13834` is the 1024x512 reduction perf probe.
- `crates/fsci-linalg/src/lib.rs:14800` is the public SVD/lstsq/pinv golden payload probe.

The active reducer performs one left Householder and one right Householder per
step. It applies the left update over columns `step..cols`, zeros the strict
subdiagonal in the active column, applies the right update over rows `step..rows`
with reused dot workspace, zeros row entries beyond the superdiagonal, and stores
all left/right reflectors for later SVD materialization.

## Hotspot Statement

The current bottleneck remains the 1024x512 Golub-Kahan reduction path, not the
public golden surface. The reducer repeatedly streams full trailing rectangles
through scalar Householder applications. A true compact-WY panel lever should
attack the far trailing update by accumulating a narrow panel state and applying
cache-blocked matrix updates to `A22`, rather than replaying full-rectangle
deltas or changing public route guards.

Prior rejected family: verified-delta packed-panel Stage 1 preserved proof but
ran `10628.935808 ms` on `vmi1153651` against `431.652279 ms`
(`0.040611x`). Do not repeat full-rectangle delta replay.

## Compact-WY Behavior Contract

Any future implementation must preserve these observable contracts:

1. Input rejection is unchanged: wide matrices still return
   `UnsupportedAssumption`; non-finite values still return `NonFiniteInput`.
2. Reflector construction is unchanged: same `start`, vector values, beta sign
   rule, `tau`, and zero-reflector handling for every generated left and right
   reflector.
3. Reduction outputs are bit-identical for the deterministic probes unless an
   explicitly approved non-bitwise floating-point contract is written first:
   diagonal bits, superdiagonal bits, upper-bidiagonal structure, reflector taus,
   and `bidiag_reduction_digest` must remain `0x90cdd3f8f71ed2c1` for the
   1024x512 probe.
4. Public API behavior is unchanged: `svd`, `svdvals`, `lstsq`, and `pinv`
   must keep payload SHA
   `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.
5. Ordering and tie-breaking are unchanged: singular values remain in the same
   descending order, rank decisions and clustered-spectrum fallback behavior are
   not loosened, and public route acceptance thresholds are not touched.
6. Floating-point stability is not weakened: no tolerance relaxation, no change
   to public reconstruction thresholds, no change to canonical sign handling,
   and no unsafe code.
7. The panel update must be isomorphic to sequential Householder application at
   the contract level. If exact bit identity is not achievable, the candidate
   must first add a stricter proof harness defining the new tolerated operation
   order, reconstruction bound, orthogonality bound, and public golden invariants.

## Initial Opportunity Score

| Hotspot | Impact | Confidence | Effort | Score |
| --- | ---: | ---: | ---: | ---: |
| `golub_kahan_bidiagonal_reduction` far trailing update | 5 | 3 | 5 | 3.0 |

Score formula: `Impact * Confidence / Effort`.

Rationale: impact is high because the reduction phase is still the dominant
private SVD core target; confidence is medium because prior dense/verified-delta
compact-WY-like attempts were rejected, but true narrow panel accumulation is a
different primitive; effort is high because preserving reflector and floating
point contracts in safe Rust is nontrivial.

## Target

Gating target from bead context:

```text
same-worker vmi1153651 target >= 1.35x
baseline_ms=431.652279
target_ms <= 319.742429
digest must remain 0x90cdd3f8f71ed2c1
public_golden_sha256 must remain 1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225
```

Non-gating local fallback target, included only for repeatability if RCH falls
back to the same machine again:

```text
fallback_baseline_ms=245.760722
fallback_target_ms <= 182.045
```

Before Pass 2 implementation, reattempt the reduction baseline through an
admissible RCH worker, preferably `vmi1153651`, or explicitly accept the supplied
`vmi1153651` context baseline as the comparison gate.
