#!/usr/bin/env python3
"""Oracle: scipy.sparse CSR SpMV (.dot) at the same n/density as the fsci-sparse
bench_spmv CONFIGS. SpMV time approx O(nnz); exact pattern differs but timing is
comparable. crates/fsci-sparse/benches/sparse_bench.rs CONFIGS = (100,5%),(1000,1%),
(10000,0.1%).
"""
import time
import numpy as np
import scipy.sparse as sp


def med(fn, r=15):
    ts = []
    for _ in range(r):
        t0 = time.perf_counter(); fn(); ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    rng = np.random.default_rng(0)
    for (n, density) in [(100, 0.05), (1000, 0.01), (10000, 0.001)]:
        A = sp.random(n, n, density=density, format="csr", random_state=0)
        x = rng.standard_normal(n)
        print(f"scipy csr spmv {n}x{n} nnz={A.nnz}: {med(lambda: A.dot(x))*1e6:.2f} us")
