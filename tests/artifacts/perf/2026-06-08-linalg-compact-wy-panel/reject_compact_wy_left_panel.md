# Compact-WY Left-Panel Rejection

Bead: `frankenscipy-8l8r1.53`

## Lever

Private compact-WY left Householder panel helper:

```text
A22 -= V T (V^T A22)
```

No public `svd`, `svdvals`, `lstsq`, `pinv`, CASP, or
`deterministic_thin_svd` route switch was made.

## Baseline / Target

- Current-route 1024x512 reduction anchor from `.52`: `431.652279 ms`,
  digest `0x90cdd3f8f71ed2c1`.
- Restored-source reprofile after this rejection on RCH worker `vmi1167313`:
  `415.060721 ms`, digest `0x90cdd3f8f71ed2c1`.
- Required public golden SHA:
  `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.

## Proof

- RCH focused proof artifact: `proof_compact_wy_left_panel_rch.txt`.
- Worker: `vmi1153651`.
- Result: `2 passed; 0 failed; 1 ignored`.
- Tests:
  - `compact_wy_left_panel_matches_sequential_left_reflectors`
  - `compact_wy_left_panel_is_deterministic_for_fixed_input`
- Public golden payload SHA was rechecked locally and remained:
  `1cdd3658c6caef8dec9fc58fa7e12b8d5c90151e2f93df91ffe2fcf862c16225`.

## Isomorphism

- Ordering preserved: yes for the retained tree, because the helper was
  restored before commit. During the trial, public route ordering was untouched.
- Tie-breaking unchanged: yes; singular-value order, rank thresholds, route
  gates, and sign canonicalization were untouched.
- Floating point: trial helper was tolerance-equivalent to sequential
  reflector application (`max_abs_diff=3.21634274769166950e-10`,
  `tolerance=1.00634097256314577e-5`) but not bit-identical due changed
  accumulation grouping. Retained tree is bit-identical to pre-trial source.
- RNG: unchanged; no RNG was used.
- Golden output: public golden SHA unchanged as listed above.

## Benchmark

RCH private-kernel perf probe:

```text
worker=vmi1153651
reference_ms=31.722281
compact_wy_ms=35.640262
speedup=0.890069
reference_digest=0xebfdc2c85f97efc2
compact_wy_digest=0x8ac6a0f447f9367f
```

An additional RCH stdout run on `vmi1156319` also showed a regression:

```text
reference_ms=11.884251
compact_wy_ms=14.747280
speedup=0.805861
```

## Score Gate

- Impact: `5`
- Confidence after measurement: `0`
- Effort: `5`
- Score: `0.0`

Decision: reject. The helper passed the behavior proof but failed the
performance gate on remote release runs. The source was restored by manual
patch before closeout; `crates/fsci-linalg/src/lib.rs` has no retained diff.

## Next Primitive

The next bead should stop isolating a left-only panel and instead attack the
actual two-sided Golub-Kahan hot path: a communication-avoiding bidiagonal
panel that accumulates left and right reflectors together, keeps the coupled
small panel state resident, and applies one cache-blocked safe-Rust
GEMM-shaped trailing update to the real far rectangle.
