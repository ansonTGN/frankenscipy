#!/usr/bin/env python3
"""Oracle: scipy.stats.multiscale_graphcorr on the SAME deterministic x,y the
fsci-stats bench_mgc uses (n=80, 1-D, reps=100). Mirrors the data generator in
crates/fsci-stats/benches/stats_bench.rs::bench_mgc.
"""
import time
import numpy as np
from scipy.stats import multiscale_graphcorr


def med(fn, r=5):
    ts = []
    for _ in range(r):
        t0 = time.perf_counter(); fn(); ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    n = 80
    x = np.array([np.sin(i * 0.1) for i in range(n)])
    y = np.array([np.sin(i * 0.1) + np.cos(i * 0.37) * 0.3 for i in range(n)])
    t = med(lambda: multiscale_graphcorr(x, y, reps=100, random_state=0))
    print(f"scipy multiscale_graphcorr n={n} reps=100: {t*1e6:.2f} us")
