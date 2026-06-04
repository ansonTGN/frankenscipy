# fsci-cluster silhouette_samples optimization conclusion

Bead: `frankenscipy-8l8r1.30`

Result: KEPT.

## Lever

`silhouette_samples` no longer repeats a full `j = 0..n` label scan once for
same-cluster `a(i)` and once per cluster for `b(i)`.

The final implementation builds exact per-anchor/per-cluster distance buckets:

- for moderate `n * k`, a symmetric pairwise pass computes each distance once
  and updates both affected anchor rows;
- for small inputs, huge bucket matrices, or overflow, a memory-capped exact
  bucket fallback performs one `j = 0..n` pass per anchor.

Both paths preserve the public API, validation, output ordering, cluster-order
`b(i)` min behavior, and final formula.

## Before / After

Focused RCH hyperfine keep gate:

| mode | shape | mean |
| --- | --- | ---: |
| pre-edit library baseline | `n=2000 d=2 k=256 repeats=1` | `656.1 ms +/- 25.6 ms` |
| final `silhouette-samples` | `n=2000 d=2 k=256 repeats=1` | `14.6 ms +/- 1.4 ms` |

Speedup vs pre-edit library baseline: `44.9x`.

Final same-binary A/B:

| mode | shape | mean |
| --- | --- | ---: |
| original repeated-rescan mirror | `n=2000 d=2 k=256 repeats=1` | `523.3 ms +/- 40.4 ms` |
| bucket-pass mirror | `n=2000 d=2 k=256 repeats=1` | `14.5 ms +/- 0.7 ms` |
| final implementation | `n=2000 d=2 k=256 repeats=1` | `14.6 ms +/- 1.4 ms` |

The measured win is the segmented bucket reduction over repeated cluster
rescans.

## Isomorphism Proof

```text
perf_cluster golden before:
37b0fd42300f4133fed4af0625f9657a66e5ff47038e005d816a6fb98d6e955d

perf_cluster golden after:
37b0fd42300f4133fed4af0625f9657a66e5ff47038e005d816a6fb98d6e955d

sorted silhouette_samples test-output before/after:
a8b744b19577e3b60b84a3e47b2507d8613e5668c9ad91af5c6886c55b72f279
```

`cmp` exits:

- `golden_before_after_cmp.exit = 0`
- `golden_before_after_tests_stable_sorted_cmp.exit = 0`

Isomorphism details:

- Ordering preserved: samples are pushed in input sample order.
- Tie-breaking unchanged: `b(i)` only updates on strict lower cluster mean.
- Floating-point: per-cluster sums receive distances in ascending sample-index
  order for the fallback; the symmetric path computes the same pair distances and
  writes each target anchor/cluster bucket exactly once per pair.
- RNG: none.

## Validation

All final-state gates exited `0`:

- RCH `cargo test -p fsci-cluster --release --locked`
- RCH `cargo check -p fsci-cluster --all-targets --locked`
- RCH `cargo clippy -p fsci-cluster --all-targets --locked -- -D warnings`
- `ubs crates/fsci-cluster/src/lib.rs crates/fsci-cluster/src/bin/perf_cluster.rs`
- `rustfmt --edition 2024 --check crates/fsci-cluster/src/bin/perf_cluster.rs`
- `cargo fmt -p fsci-cluster --check`

Score: `8.0 = impact 5 * confidence 4 / effort 2`.
