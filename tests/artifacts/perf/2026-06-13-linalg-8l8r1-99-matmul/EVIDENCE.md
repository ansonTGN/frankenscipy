# frankenscipy-8l8r1.99 evidence

Target: dense `fsci-linalg` flat-workspace matmul, profile-routed from the
post-`frankenscipy-8l8r1.94` linalg reprofile where public matmul remained the
largest dense-linear-algebra gap.

## Lever

Replace manual `Simd::from_array` packed-B panel gathers in
`matmul_flat_compute_rows` with `Simd::from_slice` loads over pre-sliced packed
panels. No block sizes, dispatch thresholds, thread partitioning, output layout,
RNG, or numerical formulas changed.

Rejected candidate before this keep: increasing the row block (`RB`) regressed
1024 and 2048 in the focused harness, so the constant remains `64`.

## Behavior proof

Isomorphism:

- Ordering: each output cell still accumulates `k` in monotonic `0..ka` order.
- Tie-breaking: no comparisons or branch ordering changed.
- Floating point: the same packed values feed the same SIMD multiply-add sequence
  and the same accumulator slots; only the lane-load spelling changed.
- RNG: deterministic test matrices are formula-generated; no RNG state exists.
- Parallelism: row ownership and thread count logic are unchanged.

Proof commands:

- `rch exec -- cargo test -p fsci-linalg --lib --release matmul_flat -- --nocapture`
  passed, including `matmul_flat_compute_rows_row_split_is_bit_identical` and
  `matmul_flat_workspace_is_bit_identical_to_naive_ijk`.
- `rch exec -- cargo test -p fsci-linalg --lib --release matmul_medium_flat_workspace_route_golden_digest -- --ignored --nocapture`
  passed with `matmul_medium_flat_route_golden_digest=0x6e401fad043ac8fd`.
- `SHA256SUMS` records sha256 hashes for the golden/proof/benchmark gate outputs.

## Benchmark evidence

Focused large-matmul harness, same RCH worker `vmi1152480`:

| size | baseline | after | ratio |
| --- | ---: | ---: | ---: |
| 1024 | 87.339 ms | 66.048 ms | 1.32x |
| 2048 | 579.120 ms | 421.936 ms | 1.37x |
| 4096 | 7565.109 ms | 3208.678 ms | 2.36x |

Criterion matmul, same RCH worker `vmi1153651`:

| size | baseline mean | after mean | ratio |
| --- | ---: | ---: | ---: |
| 256 | 10.266 ms | 7.2937 ms | 1.41x |
| 512 | 72.475 ms | 27.018 ms | 2.68x |
| 768 | 94.003 ms | 117.22 ms | 0.80x |
| 1024 | 203.19 ms | 140.89 ms | 1.44x |

Keep decision: the bead target is the large dense-matmul path. The focused
same-worker harness improves every measured target size and the Criterion
1024 route improves on the pinned worker. Score: Impact 3.5 x Confidence 0.9 /
Effort 1.0 = 3.15, above the 2.0 keep threshold.

## Gates

- `ubs crates/fsci-linalg/src/lib.rs`: exit 0; existing warning inventory only.
- `rustfmt --edition 2024 --check crates/fsci-linalg/src/lib.rs`: pass.
- `rch exec -- cargo check -p fsci-linalg --all-targets --locked`: pass.
- `rch exec -- cargo clippy -p fsci-linalg --all-targets --locked --no-deps -- -D warnings`: pass.

Blocked but recorded:

- `cargo fmt --check -p fsci-linalg` fails on pre-existing unformatted
  `crates/fsci-linalg/src/bin/diff_*` probe binaries.
- `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings` fails
  before linalg on pre-existing `fsci-fft` `manual_is_multiple_of`.
- Follow-up bead filed: `frankenscipy-wwwml`.
