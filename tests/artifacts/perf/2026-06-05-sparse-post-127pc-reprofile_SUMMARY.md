# Sparse Reprofile After frankenscipy-127pc

Date: 2026-06-05
Crate: `fsci-sparse`
Command:

```bash
RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- --warm-up-time 1 --measurement-time 3 --sample-size 20 --noplot
```

Worker: `ts2`

## Top Results

Absolute-time ranking from the post-landing run:

1. `sparse_spmm/2000x2000_d1/2000`: `12.090 ms` median `[11.965, 12.230]`.
2. `sparse_arithmetic/10000x10000_d0_add/10000`: `1.5352 ms` median `[1.4042, 1.6406]`.
3. `sparse_spmm/1000x1000_d1/1000`: `1.1000 ms` median `[1.0942, 1.1049]`.
4. `sparse_format_conversion/10000x10000_d0_csc_to_csr/10000`: `829.98 us` median `[821.07, 843.18]`.
5. `sparse_format_conversion/10000x10000_d0_csr_to_csc/10000`: `814.45 us` median `[812.10, 816.49]`.

## Pivot

SpMM remains the top sparse hotspot after the fused exact-worker phase. The next
profile-backed bead is `frankenscipy-0uon5`.

Required next primitive: true CSC/column-panel traversal, semiring-style
symbolic structure cache, or another memory-layout/algorithmic SpGEMM change.
Avoid replay, mark/epoch, capacity/final-fill, row-partition, structural-count,
and scheduling-only variants.
