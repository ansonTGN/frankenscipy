# fsci-linalg psn7x tridiagonal QR batched Givens replay

Bead: `frankenscipy-psn7x`
Agent: `RubyWaterfall`
Date: 2026-06-15
Verdict: REJECTED - source restored

## Target

Profile-backed hotspot from the current `psn7x` thread: staged native symmetric eigensolver, post-back-transform profile on `vmi1152480` showed `symmetric_tridiagonal_qr_eigen` / tridiagonal QR eigenvector stage around `1089.798 ms` of `2317.048 ms` total at n=1200. Prior rotate-slice/index spelling was rejected, so this tested a structurally different replay order for the same fixed Givens rotations.

## One Lever

Temporary source probe:

- store each sweep's generated `(col, c, s)` Givens rotations exactly
- replay them by 64-row blocks over the column-major eigenvector matrix
- keep per-row rotation order and scalar expressions bitwise-identical to immediate `rotate_eigenvector_columns`
- do not carry rotations across sweep, deflation, or 2x2 boundaries

The source probe was restored after the benchmark gate failed; `crates/fsci-linalg/src/lib.rs` has no final diff.

## Baseline

First `rch exec -- hyperfine ...` attempt refused local fallback because `hyperfine` is a non-compilation command:

- `baseline_native_hyperfine_rch.txt`

The usable baseline reran `hyperfine` locally around `rch exec -- cargo test -j 1 -p fsci-linalg symmetric_eigh_native_vs_nalgebra_timing --release --locked -- --ignored --nocapture`, which produced four `ovh-a` samples. These samples include cold compile wall time in hyperfine, so only the printed test timings are used:

| sample | n=400 native | n=800 native | n=1200 native |
| --- | ---: | ---: | ---: |
| 1 | 49.0 ms | 343.5 ms | 1121.0 ms |
| 2 | 47.3 ms | 330.3 ms | 1082.6 ms |
| 3 | 45.0 ms | 325.2 ms | 1076.5 ms |
| 4 | 81.0 ms | 568.4 ms | 1421.6 ms |

Representative stable band before the cold/noisy sample: n=400 `45.0-49.0 ms`, n=800 `325.2-343.5 ms`, n=1200 `1076.5-1121.0 ms`.

## Proofs

- `proof_batched_givens_bits_rch.txt`: RCH `vmi1153651`, direct kernel differential test passed. It covered empty, one/two rotation, 63/64/65-row boundary, tail-block, and partial-column cases by `to_bits()`.
- `proof_native_eigh_matches_rch.txt`: RCH `ovh-a`, end-to-end native symmetric-eigh comparison passed.
- `proof_public_golden_digest_rch.txt`: RCH `ovh-a`, public sort golden remained `0x287a5d3679a8bc6a`.

Isomorphism status: fixed rotations were behavior-clean in the temporary proof. Ordering, tie-breaking, RNG, and public route stayed unchanged. Floating-point expressions matched the immediate replay for fixed rotations; the source was still rejected on performance.

## Rebench

RCH release timing on `ovh-a` with the probe:

| n | after native | after nalgebra | before stable band | result |
| --- | ---: | ---: | ---: | --- |
| 400 | 61.2 ms | 42.3 ms | 45.0-49.0 ms | regression |
| 800 | 481.5 ms | 340.6 ms | 325.2-343.5 ms | regression |
| 1200 | 1576.7 ms | 1098.9 ms | 1076.5-1121.0 ms | regression |

Score: `Impact 0.0 * Confidence 4.0 / Effort 2.0 = 0.0`

## Artifact SHA-256

```text
78167827babb895c1b7192687bc4bee4957d9fd50e885ea7979a721daaa25384  after_batched_givens_timing_rch.txt
22b6730aebddcd63739ce680ea3f7026b6bdfe2ed9fbaafedd8cb9e737fe394a  baseline_native_hyperfine_rch.txt
5f0673e51874fd66454af0ae66379280cb2fffbf9d296a64ccd6513a59537036  baseline_native_hyperfine_rch_retry.txt
6e77349fda3dc12380db2afc70d212f44a335ed7b4f46bdaaabad185289c0fa7  proof_batched_givens_bits_rch.txt
ddc3262ff4acd2e0a29b99a74ebcb49e8b22db44c74749a15d2672d71da2523c  proof_native_eigh_matches_rch.txt
5ae6842a17d95061251df4922f93afb7cb9acc1de660498028a56150cbdc567c  proof_public_golden_digest_rch.txt
```

## Next Route

Do not retry row-block replay, slice helper rewrites, or index-spelling variants for this hotspot. The next `psn7x` route should replace the tridiagonal eigensolver primitive: divide-and-conquer, MRRR-style relatively robust representations, or blocked/batched bulge chasing with a proof plan that avoids changing public eigensort/golden behavior.
