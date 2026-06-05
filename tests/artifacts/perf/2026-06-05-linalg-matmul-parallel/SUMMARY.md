# fsci-linalg matmul: single-threaded flat-workspace GEMM -> multithreaded row split

## Target (bead frankenscipy-9s2mz, GEMM swing)

matmul's large-matrix path (matmul_flat_workspace, m,ka,n >= 1024) was already
SIMD + cache-blocked + B-panel-packed, but SINGLE-THREADED (~11 GFLOP/s on a
64-core worker). The directive's "different parallelization model" — a bigger AND
byte-identical lever than Strassen on this already-blocked kernel.

## Lever (one)

Extract the kernel body into `matmul_flat_compute_rows(out, row_start, row_end, ...)`
and distribute disjoint output-row ranges across threads via `std::thread::scope` +
`c_flat.chunks_mut(chunk_rows*n)` (the project's established no-tokio threading
pattern, cf. sparse spmm). Thread count = cores capped so each thread owns >= 64
rows; sequential for matmuls below 64M MACs.

## Isomorphism / proof (BYTE-IDENTICAL)

Every c[i][j] accumulates k in 0..ka monotonic order via the identical scalar
mul+add sequence regardless of the RB/MR block grouping or which thread owns the
row, so the result is bit-identical to the sequential kernel — only *which* core
writes each row changes. Proven by:
  - new test matmul_flat_compute_rows_row_split_is_bit_identical: full range vs
    {2,3,5,8,m}-way splits, f64::to_bits equal, n=28 (non-multiple of NR) to cover
    both SIMD-packed and scalar-tail paths under ragged splits.
  - unchanged matmul_microkernel_golden_digest + matmul_flat_workspace_is_bit_
    identical_to_naive_ijk (flat == naive ijk). => parallel == naive, bit-for-bit.
fsci-linalg matmul suite 5 passed; lib clippy + fmt clean.

## Rebench (perf_matmul bin, square, 64-core worker)

| n | before (1 thread) | after (parallel) | speedup | GFLOP/s after |
| ---: | ---: | ---: | ---: | ---: |
| 1024 | 207.8 ms | 40.33 ms | 5.15x | 53.2 |
| 2048 | 1538.6 ms | 159.84 ms | 9.63x | 107.5 |
| 4096 | 12058 ms | 925.75 ms | 13.03x | 148.5 |

Speedup grows with n (thread-spawn amortizes, more row-parallelism): 11 -> 148
GFLOP/s at 4096. Score >> 2.0, byte-identical. Strassen (O(n^2.807)) remains an
orthogonal future lever that composes on top of this now-parallel base.
