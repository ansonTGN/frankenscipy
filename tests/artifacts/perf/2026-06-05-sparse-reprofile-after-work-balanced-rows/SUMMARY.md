# Sparse Reprofile After Work-Balanced SpMM Rows

Context: post-`frankenscipy-8l8r1.32` reprofile after keeping work-balanced contiguous row partitions.

Command:

`rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- --warm-up-time 1 --measurement-time 5 --sample-size 30 --noplot`

Worker: `ts2`

Top rows by median:

- `sparse_spmm/2000x2000_d1/2000`: `13.427 ms` `[13.330, 13.544]`
- `sparse_arithmetic/10000x10000_d0_add/10000`: `1.8049 ms` `[1.7299, 1.8795]`
- `sparse_spmm/1000x1000_d1/1000`: `1.0380 ms` `[1.0326, 1.0438]`
- `sparse_format_conversion/10000x10000_d0_csc_to_csr/10000`: `837.15 us` `[833.72, 840.73]`
- `sparse_format_conversion/10000x10000_d0_csr_to_csc/10000`: `826.39 us` `[823.20, 830.00]`
- `sparse_csr_construction/10000x10000_d0/10000`: `602.59 us` `[598.80, 606.32]`

Next bead filed: `frankenscipy-127pc`, targeting a true CSC/column-panel or deeper GraphBLAS SpGEMM primitive. The next pass should not revisit replay, mark/epoch, capacity, final-fill, or row-partition bookkeeping.
