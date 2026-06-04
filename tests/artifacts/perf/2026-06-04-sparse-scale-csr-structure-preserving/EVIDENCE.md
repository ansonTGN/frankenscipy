# CSR Scale Structure-Preserving Constructor

Bead: `frankenscipy-p6lb5`

## Profile Target

Fresh sparse reprofiles after the direct transpose pass still showed
`sparse_arithmetic/10000x10000_d0_scale/10000` in the remaining hot set
at about `284.60 us`. The current `scale_csr` path multiplied data,
cloned `indices` and `indptr`, then revalidated the copied CSR structure.

Alien primitive: GraphBLAS-style separation of sparse structure from
value-only maps. The selected lever is proof-carrying invariant reuse for a
compressed sparse value map.

## One Lever

`scale_csr` now constructs the scaled CSR with
`CsrMatrix::from_components_unchecked` after cloning the source structure,
then restores the source canonical metadata.

No other sparse kernel behavior was changed. `scale_csc` is intentionally
untouched for a separate target.

## Isomorphism Proof

- Shape is copied from the input matrix.
- `indices` and `indptr` are exact clones of the input matrix structure.
- The source `CsrMatrix` is already public-valid: matching data/index
  lengths, monotone/end-point-valid `indptr`, and in-bounds indices were
  checked when it was constructed.
- The old path revalidated those exact cloned structure vectors with
  `canonicalize=false`, then overwrote the detected canonical metadata with
  `matrix.canonical`.
- The new path skips only that redundant validation scan and still overwrites
  the canonical metadata with `matrix.canonical`.
- Data values are produced by the same `matrix.data().iter().map(|v| v * alpha)`
  expression, in the same order, with the same scalar floating-point operation
  per stored value.
- Explicit zeros, signed zeros, duplicate entries, unsorted rows, row order,
  output ordering, and metadata are preserved.
- No RNG, tie-breaking, iteration reordering, tolerance, or error-order surface
  is involved for public-valid CSR inputs.

## Golden Proof

Command:

```text
rch exec -- cargo run -p fsci-sparse --bin perf_sparse --locked -- scale-csr-golden
```

Payload SHA-256 before:

```text
13fd17a126ddeabdc4ce0999efb5fd76da0542f0c463a8226a724aa544b132d0
```

Payload SHA-256 after:

```text
13fd17a126ddeabdc4ce0999efb5fd76da0542f0c463a8226a724aa544b132d0
```

Normalized payload diff:

```text
empty
```

Golden coverage includes canonical CSR, zero alpha, valid unsorted CSR metadata,
and valid duplicate-entry CSR metadata.

## Performance

Same-worker RCH Criterion on `ts2`:

```text
baseline sparse_arithmetic/10000x10000_d0_scale/10000:
  [278.68 us 280.77 us 284.01 us]

after sparse_arithmetic/10000x10000_d0_scale/10000:
  [50.699 us 51.077 us 51.432 us]
```

Median speedup: `280.77 / 51.077 = 5.50x`.

Score: `16.0 = impact 4 * confidence 4 / effort 1`.

## Validation

- `cargo fmt -p fsci-sparse --check`: pass.
- RCH `cargo check -p fsci-sparse --all-targets --locked`: pass.
- RCH `cargo clippy -p fsci-sparse --all-targets --locked -- -D warnings`: pass.
- RCH `cargo test -p fsci-sparse --locked scale -- --nocapture`: pass.
- `ubs crates/fsci-sparse/src/ops.rs crates/fsci-sparse/src/bin/perf_sparse.rs`: exit 0.

UBS reported no critical findings. Its warnings are inventory over existing
panic/test/perf-harness patterns and are not introduced by this lever.

## Reprofile

Post-change RCH sparse reprofile on `ts2`:

```text
sparse_arithmetic/10000x10000_d0_add/10000        2.1179 ms
sparse_format_conversion/10000x10000_d0_csr_to_csc/10000 810.43 us
sparse_format_conversion/10000x10000_d0_csc_to_csr/10000 802.66 us
sparse_csr_construction/10000x10000_d0/10000      629.87 us
sparse_diags/tridiag/10000                        301.46 us
sparse_spmv/10000x10000_d0_nnz100000/10000        233.24 us
sparse_arithmetic/10000x10000_d0_scale/10000       52.276 us
```

Next target should avoid another add-loop bookkeeping micro-lever. The next
profile-backed primitive should be a fundamentally different sparse add or CSR
construction strategy, such as a GraphBLAS symbolic/numeric row-block
accumulator with explicit duplicate/order proof, or a row-bucketed COO-to-CSR
constructor with bit-identical duplicate handling.
