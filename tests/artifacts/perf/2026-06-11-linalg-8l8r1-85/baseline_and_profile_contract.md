# frankenscipy-8l8r1.85 baseline and profile contract

Bead: `frankenscipy-8l8r1.85`

## Coordination

- Claimed `frankenscipy-8l8r1.85` as `in_progress`.
- Reserved:
  - `crates/fsci-linalg/src/lib.rs`
  - `crates/fsci-linalg/benches/linalg_bench.rs`
  - `tests/artifacts/perf/2026-06-11-linalg-8l8r1-85`
  - `.skill-loop-progress.md`
  - `.beads/issues.jsonl`
- Agent Mail message delivery was unavailable because the HTTP endpoint refused
  the send request; the file reservation and bead claim are the coordination
  record for this pass.

## Matmul baseline status

Fresh accepted baseline inherited from `.84`:

```text
tests/artifacts/perf/2026-06-11-linalg-8l8r1-84/baseline_matmul_criterion_rch.txt
sha256: 1358109b5d2cc23731862ae9f7419ffe617203b3e8074eee3e6e21deded8da0e
worker: vmi1227854
```

| shape | mean |
| --- | ---: |
| `matmul/256x256` | `5.0186 ms` |
| `matmul/512x512` | `18.232 ms` |
| `matmul/768x768` | `104.81 ms` |
| `matmul/1024x1024` | `169.27 ms` |

`.85` retry artifact:

```text
tests/artifacts/perf/2026-06-11-linalg-8l8r1-85/baseline_matmul_criterion_rch.txt
sha256: 56c01fa0e88b07525727b26ef3de59efe189ec9645b3b53c7372a70cef3a9742
```

The `.85` retry is a remote-required refusal, not a benchmark:

```text
[RCH] local (no admissible workers: critical_pressure=1,insufficient_slots=1)
[RCH] remote required; refusing local fallback (no worker assigned)
```

## Eigh baseline status

Dirty-worktree attempts under `tests/artifacts/perf/2026-06-11-linalg-eigh-next`
were not usable as perf evidence:

- two remote-required refusals,
- one remote failure caused by an untracked local `src/bin` target that RCH did
  not sync to the worker.

Clean detached worktree used only for benchmark admission:

```text
/data/projects/.scratch/frankenscipy-8l8r1-85-clean-20260611T043812Z
```

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench --locked -- eigh_dense
```

Artifact:

```text
tests/artifacts/perf/2026-06-11-linalg-8l8r1-85/baseline_eigh_dense_clean_worktree_rch.txt
sha256: b171127e86ef3e281f78df5f216c43294629a7c9315826ef110754d9e9daec6c
worker: vmi1227854
```

Criterion means:

| shape | mean |
| --- | ---: |
| `eigh_dense/256x256` | `11.891 ms` |
| `eigh_dense/512x512` | `93.570 ms` |

## Target classification

`eigh_dense` is the current profile-backed target for `.85`.

GEMM remains baseline-backed, but the obvious remaining source levers overlap
prior rejected or kept families. The next `eigh_dense` primitive must be a
blocked/two-stage symmetric eigensolver route, not:

- direct scalar tridiagonalization,
- output materialization/index sorting,
- GEMM B staging/direct-pack,
- panel-load spelling,
- scalar-splat spelling,
- MR/NR widening,
- worker-count row scheduling,
- 8-row row-panel accumulators,
- K-major A row-slab packing,
- RB geometry,
- KC-striped C writeback,
- row-owned output materialization,
- or 4x24 tile-width-only repetition.

## Isomorphism contract for next source lever

- Public validation, finite checks, shape errors, and trace behavior must remain
  unchanged.
- Eigenvalues remain sorted ascending with the same comparator policy.
- Eigenvectors may differ only within an explicitly documented orthogonal
  similarity/sign/subspace policy, with residual and orthogonality proofs.
- Golden output must be captured as a sha256 or stable digest over the accepted
  public-output policy before and after the lever.
- Same-worker RCH rebench is required before keep/reject.
