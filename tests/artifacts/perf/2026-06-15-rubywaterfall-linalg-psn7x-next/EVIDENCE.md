# frankenscipy-grljp Householder p-vector subprobe

Bead: `frankenscipy-grljp`
Agent: `RubyWaterfall`
Date: 2026-06-15
Base: `cc62e5a5` after fast-forwarding the detached clean worktree to `origin/main`

## Profile-backed target

The current native symmetric-eigh route remains profile-backed by the prior stage split at n=1200:

- Householder reduction: `406.741 ms`
- Tridiagonal eigen: `75.736 ms`
- Eigenvector back-transform: `103.695 ms`
- Sort/copy: `5.408 ms`

The follow-up target is the Householder reduction stage. The rejected candidate below was intentionally limited to the rank-2 update `p = tau * A_active * v` vector so public ordering, tie-breaking, floating-point formulas, and fallback behavior stayed unchanged.

## Baseline

RCH selected remote worker `ovh-a` for the usable baseline.

Criterion public `eigh_dense`:

- `eigh_dense/256x256`: mean `16.499 ms`
- `eigh_dense/512x512`: mean `237.73 ms`

Native public route probe:

- n=400 routed native `56.673791 ms` vs nalgebra `75.638153 ms`, speedup `1.334623x`, digest `0x0dbbde75b75c8612`
- n=800 routed native `253.243413 ms` vs nalgebra `380.984386 ms`, speedup `1.504420x`, digest `0xad8a7e5fa1980bfb`
- n=1200 routed native `1038.249562 ms` vs nalgebra `1131.976401 ms`, speedup `1.090274x`, digest `0x181b3486089d0e4a`

## Candidate

Probe-only source edit: parallelize only the Householder rank-2 update's `p` vector computation for large active trailing blocks. Each row retained the serial column accumulation order and wrote exactly one `p[row]` slot, so the candidate had a narrow bitwise proof target.

This was not kept.

## Isomorphism proof

- RCH `symmetric_rank2_parallel_p_matches_reference_bits` passed.
- RCH `symmetric_eigh_native_matches_nalgebra_and_timing` passed.
- Public route, eigenpair ordering, tie policy, deterministic inputs, and fallback safety were unchanged.

## Rebench

Same usable worker family: `ovh-a`.

Baseline routed native:

- n=400 `56.673791 ms`
- n=800 `253.243413 ms`
- n=1200 `1038.249562 ms`

Candidate routed native:

- n=400 `95.454219 ms`
- n=800 `546.686933 ms`
- n=1200 `1816.161388 ms`

Verdict: reject. Score `Impact 0.0 * Confidence 4.0 / Effort 2.0 = 0.0`.

`crates/fsci-linalg/src/lib.rs` was restored to zero diff after the failed gate. The next route must avoid per-step scoped thread spawning and instead target a structurally different Householder primitive: panel/block accumulation, persistent scoped reduction work, or another communication-avoiding reduction formulation.
