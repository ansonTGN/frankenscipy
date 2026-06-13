# Cholesky SYRK Trailing Update Evidence

Bead: `frankenscipy-e30by`
Source commit: `f8dcfb2c` (`perf(linalg): Cholesky trailing update as lower-triangle SYRK (~2x)`)
Follow-up evidence commit: pending in this change

## Target

Profile-backed hotspot: `solve(..., assume_a = PositiveDefinite)` at `1000x1000`, where
`cholesky_solve_blocked` spent avoidable work materializing `L21^T` and computing the full
`L21 * L21^T` product even though downstream Cholesky only reads the lower triangle.

One lever: replace the full trailing GEMM update with a safe-Rust lower-triangle SYRK update
over packed `L21` rows and 8-wide SIMD dot products. The committed source lever is already in
`f8dcfb2c`; this follow-up records the proof and benchmark artifacts.

## Same-worker benchmark

Command:

```text
RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1149989 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- baseline_solve_pos/1000x1000
```

Worker: `vmi1149989`

| Case | Artifact | Criterion time |
| --- | --- | --- |
| Baseline clean HEAD (`c520dd39` + benchmark harness only) | `baseline_solve_pos_1000_clean_head_rch.txt` | `[64.845 ms 66.475 ms 68.126 ms]` |
| After SYRK source lever (`f8dcfb2c` + same harness) | `after_solve_pos_1000_syrk_fresh_worktree_rch.txt` | `[40.447 ms 41.134 ms 41.822 ms]` |

Mean speedup: `66.475 / 41.134 = 1.616x`.

Score: `(Impact 4.0 * Confidence 4.5) / Effort 2.0 = 9.0 >= 2.0`.

## Isomorphism proof

- Ordering preserved: yes. The solve output is a vector with fixed indices; no sorting, map
  iteration, or tie-breaking is introduced.
- Tie-breaking unchanged: not applicable. The route performs deterministic dense arithmetic only.
- Floating point preserved at the contract level: the update computes the same mathematical
  symmetric rank update for the only triangle read by downstream Cholesky. Public behavior is
  locked by the large-route golden proof below plus residual/known-solution checks.
- RNG preserved: no RNG is introduced or consumed.
- External kernels: none. The implementation remains safe Rust and uses no C BLAS/LAPACK/MKL/XLA.

## Golden behavior

The final release proof exercises the actual public fast-path threshold (`n = 1000`) through
`solve(..., MatrixAssumption::PositiveDefinite)`, checks a deterministic known solution and
residual, and locks the output digest:

```text
solve_spd_fast_path_1000_golden_digest=0x11a9fcf14e2c99f7
```

Artifact:

```text
proof_solve_spd_1000_fast_path_locked_fresh_worktree_rch.txt
```

Supporting focused proofs:

- `proof_cholesky_blocked_reference_fresh_worktree_rch.txt`: private blocked Cholesky reference/residual test passed.
- `proof_solve_spd_assumption_fresh_worktree_rch.txt`: public SPD assumption smoke test passed.

## Gates

- RCH `cargo check -j 1 -p fsci-linalg --lib`: passed (`check_fsci_linalg_lib_after_golden_fresh_worktree_rch.txt`).
- RCH `cargo clippy -j 1 -p fsci-linalg --lib --no-deps -- -D warnings`: passed (`clippy_fsci_linalg_lib_no_deps_after_golden_fresh_worktree_rch.txt`).
- `rustfmt --edition 2024 --check crates/fsci-linalg/src/lib.rs crates/fsci-linalg/benches/linalg_bench.rs`: passed (`rustfmt_touched_files_check_after_golden.txt`).
- `git diff --check` on touched files: passed (`git_diff_check_touched_files_after_golden.txt`).
- `ubs crates/fsci-linalg/src/lib.rs crates/fsci-linalg/benches/linalg_bench.rs`: exit 0 with 0 critical issues (`ubs_linalg_cholesky_syrk_after_golden.txt`).

Note: `cargo fmt --check -p fsci-linalg` currently reports unrelated pre-existing formatting
diffs in `crates/fsci-linalg/src/bin/diff_*.rs`; the touched files are rustfmt-clean.

## RCH note

Two early RCH attempts from the shared project root failed because the worker retained a stale
deleted helper body in the remote project directory. The kept benchmark/proof artifacts use fresh
detached scratch worktree roots so the remote source matched the local diff.
