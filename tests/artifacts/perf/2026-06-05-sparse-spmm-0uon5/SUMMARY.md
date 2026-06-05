# frankenscipy-0uon5 - SpMM Bounded Outer-Panel Trial

## Target

- Bead: `frankenscipy-0uon5`
- Profile-backed hotspot: `sparse_spmm/2000x2000_d1/2000`
- Baseline command: `RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- sparse_spmm/2000x2000_d1/2000 --warm-up-time 1 --measurement-time 5 --sample-size 30 --noplot`

## Lever Tried

Tried a bounded row-panel outer-product SpGEMM path for the parallel CSR SpMM case. The candidate bucketed panel-local A entries by `k`, streamed each B row once per worker panel, accumulated into a bounded dense row-panel workspace, and emitted each row by reverse first-seen column order.

The source change was rejected and manually restored. `crates/fsci-sparse/src/linalg.rs` has no remaining diff for this trial.

## Behavior Proof

- Row order: preserved by contiguous worker ranges and row-order chunk concatenation.
- A traversal: guarded to rows with nondecreasing A column indices so `k`-ordered buckets preserve existing per-row A encounter order.
- B traversal: each B row is traversed in its existing index order.
- Output order: row-local reverse first-seen column emission preserved.
- Floating point: product accumulation order preserved for the guarded canonical path; fallback retained for unsupported row order.
- Tie-breaking/RNG: no RNG or tie-breaking participates in this path.
- Golden output: strict SpMM payload SHA-256 stayed `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`.
- Focused proof: RCH `cargo test -p fsci-sparse --locked spmm_parallel_matches_serial_byte_for_byte -- --nocapture` passed.

## Benchmarks

- Baseline on RCH `ts2`: `12.439 ms` median `[12.303, 12.566]`.
- After on RCH `ts2`: `14.036 ms` median `[13.896, 14.189]`.
- Ratio: `0.886x`; regression of about `12.8%` by median.

## Verdict

Rejected. Score `0.0`, below the keep threshold. The next pass should pivot to a different profile-backed sparse primitive rather than another SpMM scheduling, row-plan, finalization, capacity, mark/epoch, or panel-accumulator variant.
