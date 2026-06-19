#!/usr/bin/env python3
"""Oracle: scipy.stats.gaussian_kde evaluated at many points, matching the fsci-stats
bench_kde workload (n=1000/5000 1-D dataset, evaluate at n points). Mirrors the data
generator in crates/fsci-stats/benches/stats_bench.rs::bench_kde.
"""
import time
import numpy as np
from scipy.stats import gaussian_kde


def med(fn, r=7):
    ts = []
    for _ in range(r):
        t0 = time.perf_counter(); fn(); ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    for n in (1000, 5000):
        data = np.array([np.sin(i * 0.017) * 3.0 + np.cos(i * 0.0031) for i in range(n)])
        kde = gaussian_kde(data)  # scott bw, same default as fsci
        pts = np.array([-5.0 + i * 10.0 / n for i in range(n)])
        print(f"scipy gaussian_kde eval n={n} at {n} pts: {med(lambda: kde(pts))*1e6:.2f} us")
