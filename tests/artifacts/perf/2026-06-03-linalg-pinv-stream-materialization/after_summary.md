# Stream Pinv Materialization After

- Timestamp: 2026-06-03T18:01:25-04:00
- Bead: `frankenscipy-8l8r1.29`
- Profile-backed target: `baseline_pinv/1000x500`
- Worker: `ts2`
- Command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-linalg --bench linalg_bench --locked -- 'baseline_pinv/1000x500' --warm-up-time 1 --measurement-time 3 --sample-size 10 --noplot`
- Exit: `0`

## Criterion Median

| benchmark | baseline median | after median | result |
| --- | ---: | ---: | --- |
| `baseline_pinv/1000x500` | `437.29 ms` | `588.02 ms` | rejected; `1.34x` slower |

## Behavior Proof

- Before sorted stable SHA-256: `cc7b25e24e092e68b31a93abe71436bf881cd54a431087779a8603fb68c42e9d`
- After sorted stable SHA-256: `cc7b25e24e092e68b31a93abe71436bf881cd54a431087779a8603fb68c42e9d`
- Sorted stable before/after diff: empty.
- Trial contract preserved validation/error order, SVD invocation/options, singular-value ordering, threshold/rank semantics, certificate fields, output row/column order, RNG absence, tie-breaking absence, and global-state absence.

## Source Restore

- The row-major GEMM materialization source lever was restored after the benchmark failed the keep gate.
- `git diff --quiet -- crates/fsci-linalg/src/lib.rs` exit: `0`
- `cargo fmt -p fsci-linalg --check` after restore exit: `0`

## Gate

Rejected. The trial does not meet Score `>= 2.0` because it produced a measured slowdown despite unchanged behavior proof. Next pinv work should replace the full SVD/materialization algorithmic layer rather than repackage the same full-rank pseudoinverse multiply.
