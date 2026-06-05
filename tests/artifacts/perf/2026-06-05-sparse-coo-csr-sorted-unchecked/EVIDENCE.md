# COO Sorted-Unique CSR Unchecked Build Evidence

Bead: `frankenscipy-8l8r1.34`

## Profile-Backed Target

- Hot benchmark: `sparse_csr_construction/10000x10000_d0/10000`
- Prior reprofile in bead: 621.14 us median on `ts2`
- Target function: `sorted_unique_coo_to_csr` in `crates/fsci-sparse/src/ops.rs`

## One Lever

When a COO matrix is already strictly sorted and unique by `(row, col)`, build the CSR components directly with `CsrMatrix::from_components_unchecked` and set canonical metadata. The generic sort/dedup fallback is unchanged.

## Isomorphism Proof

- Ordering preserved: yes. The fast path only accepts strict `(row, col)` increasing order and writes CSR rows in that same encounter order.
- Tie-breaking unchanged: yes. Strict uniqueness means there are no duplicate-coordinate ties in this fast path; duplicate or unsorted inputs still use the existing canonical fallback.
- Floating-point unchanged: yes. Values are copied without arithmetic, reordering, rounding, or zero filtering.
- RNG unchanged: yes. No RNG is used in conversion; benchmark inputs keep the same deterministic seed.
- Error behavior unchanged: yes. `CooMatrix::from_triplets` already validated lengths and coordinate bounds before this conversion path.
- Golden output: `943927e5ee49288577e3ed37e13b8f38c76aec8d0b71ac159b4895905afd6df1`

## RCH Proof

- `sha256sum -c golden_after.sha256`: OK
- `golden_after.cmp`: `coo_csr_golden_cmp=0`
- Current RCH payload: `golden_current.txt`, same sha256 `943927e5ee49288577e3ed37e13b8f38c76aec8d0b71ac159b4895905afd6df1`
- `rch exec -- cargo test -p fsci-sparse coo_to_csr --lib --locked`: 2 passed
- `rch exec -- cargo check -p fsci-sparse --all-targets --locked`: pass
- `rch exec -- cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`: pass
- `cargo fmt --check -p fsci-sparse`: pass
- `ubs crates/fsci-sparse/src/ops.rs`: no critical findings; broad pre-existing warning inventory remains outside this hunk

## Benchmark Delta

Criterion command:

```text
rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- sparse_csr_construction/10000x10000_d0/10000 --warm-up-time 1 --measurement-time 5 --sample-size 30 --noplot
```

| Run | Worker | Median |
| --- | --- | ---: |
| Baseline | `ts2` | 623.92 us |
| Restored baseline | `ts1` | 527.53 us |
| Candidate | `ts1` | 256.98 us |
| Current after | `ts1` | 257.48 us |

Same-worker delta: `527.53 us -> 257.48 us`, 2.05x faster.

Score: Impact 4 x Confidence 5 / Effort 2 = 10.0, keep.
