# SpMM Fused Exact Worker Pass - frankenscipy-127pc

Date: 2026-06-05
Agent: OliveSnow
Crate: `fsci-sparse`
Target: `sparse_spmm/2000x2000_d1/2000`

## Profile Context

`frankenscipy-127pc` was selected from `br ready --json` as the only ready
`[perf]` bead. It was backed by the post-work-balanced sparse reprofile where
`sparse_spmm/2000x2000_d1/2000` remained the top sparse target on `ts2`:

- Post-work-balanced reprofile: `13.427 ms` median `[13.330, 13.544]`.
- Previous same-source CSC/column-panel baseline: `13.182 ms` median
  `[13.084, 13.282]`.
- Fresh pre-change RCH baseline in this pass selected `ts1`: `10.591 ms`
  median `[10.373, 10.885]`. This is retained as a fresh baseline artifact,
  but same-worker keep/reject decisions use the `ts2` current-source baselines
  above because the after runs selected `ts2`.

## Lever

Kept lever: fuse the existing symbolic row-count pass and numeric row pass into
one scoped worker phase. Each range still computes exact row counts before
running the existing numeric `spmm_row_chunk`, but the worker immediately uses
its local count sum as the capacity hint and returns both counts and numeric
output. The driver then builds the global `indptr` and concatenates chunks in
the unchanged range order.

This is not a new accumulator, replay path, row partitioner, final-fill path, or
CSC direct-fill path. It removes one thread-scope spawn/join phase while
preserving the existing per-row numeric kernel.

Rejected candidates from this pass:

- Direct structural fill: `16.819 ms` median `[16.716, 16.917]` on `ts2`.
- Structural-counts plus existing numeric row kernel: `13.300 ms` median
  `[13.185, 13.427]` on `ts2`.

## Benchmarks

Command shape:

```bash
rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- sparse_spmm/2000x2000_d1/2000 --warm-up-time 1 --measurement-time 5 --sample-size 30 --noplot
```

Kept lever results:

- First after run on `ts2`: `12.537 ms` median `[12.452, 12.630]`.
- Confirmation after run on `ts2`: `12.287 ms` median `[11.986, 12.638]`.

Against the same-worker `ts2` current-source baselines, the confirmed median is:

- `6.8%` faster than `13.182 ms`.
- `8.5%` faster than `13.427 ms`.

Score: `6.0 = impact 2.0 * confidence 3.0 / effort 1.0`.

## Isomorphism Proof

Preserved invariants:

- A row traversal order is unchanged by reusing `spmm_work_balanced_ranges`.
- B row encounter order is unchanged because every worker calls the same
  `spmm_row_counts_chunk` and `spmm_row_chunk` functions as before.
- Output column order and tie-breaking are unchanged because `spmm_row_chunk`
  still emits reverse first-seen order and the driver concatenates chunks in
  range order.
- Floating-point accumulation order is unchanged because the numeric inner loop
  was not changed.
- Explicit zero elision is unchanged because the same numeric row kernel filters
  output entries.
- Metadata propagation is unchanged; `sorted_indices` is still the conjunction
  of per-chunk sorted flags.
- No RNG is involved in SpMM execution.

Strict golden payload:

- Before SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`.
- After SHA: `0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`.
- `cmp` and `diff -u` of normalized strict payloads were empty.

## Validation

- `rch exec -- cargo test -p fsci-sparse --locked spmm -- --nocapture`: passed
  (`3 passed; 0 failed; 1 ignored`).
- `rch exec -- cargo check -p fsci-sparse --all-targets --locked`: passed.
- `rch exec -- cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`: passed.
- `cargo fmt -p fsci-sparse --check`: passed.
- `ubs crates/fsci-sparse/src/linalg.rs`: exit 0, no critical findings; broad
  pre-existing warnings remain in the large file.

## Verdict

Kept. Close `frankenscipy-127pc` with the fused exact-worker phase as the single
source lever for this commit.
