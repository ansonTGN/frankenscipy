#!/usr/bin/env python3
"""Oracle: scipy.stats.kendalltau on the SAME x,y the fsci-stats bench_rank_correlation
uses (n=2048/4096). Mirrors deterministic_data + the y formula in stats_bench.rs.
"""
import time
import numpy as np
from scipy.stats import kendalltau


def med(fn, r=9):
    ts = []
    for _ in range(r):
        t0 = time.perf_counter(); fn(); ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    for n in (2048, 4096):
        x = np.array([np.sin(i * 0.017) + np.cos(i * 0.031) * 0.25 + (i % 17) * 0.001 for i in range(n)])
        y = np.array([round(np.cos(i * 0.013) + (i % 11) * 0.1) for i in range(n)])
        print(f"scipy kendalltau n={n}: {med(lambda: kendalltau(x, y))*1e6:.2f} us")
