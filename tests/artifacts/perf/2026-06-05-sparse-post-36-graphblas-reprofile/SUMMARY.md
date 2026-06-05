# Sparse Post-.36 GraphBLAS Reprofile

Bead: `frankenscipy-8l8r1.38`

## Command

`RCH_FORCE_REMOTE=1 rch exec -- cargo bench -p fsci-sparse --bench sparse_bench --locked -- --warm-up-time 1 --measurement-time 3 --sample-size 20 --noplot`

## Result

- Worker: `ts1`.
- Top row: `sparse_spmm/2000x2000_d1/2000`, median `10.060 ms`, interval `[9.8328,10.234]`.
- Next row: `sparse_arithmetic/10000x10000_d0_add/10000`, median `1.6808 ms`, interval `[1.5762,1.8290]`.
- Other contenders: `sparse_spmm/1000x1000_d1/1000` at `731.51 us`, CSR/CSC conversion 10000 at about `563-564 us`, CSR construction 10000 at `260.73 us`.

## Decision

SpMM remained the current sparse hotspot after `frankenscipy-8l8r1.36` rejected/no-shipped, so the next implementation candidate stayed in SpMM. The prior rejection streak ruled out marker/epoch/count/capacity/replay/row-plan/dense-outer-product bookkeeping families.
