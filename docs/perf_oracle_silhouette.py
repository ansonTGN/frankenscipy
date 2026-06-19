#!/usr/bin/env python3
"""Oracle: sklearn.metrics.silhouette_score on the SAME deterministic blobs the
fsci-cluster bench_silhouette uses (n=500/2000, 4-D, labels i%4). Mirrors blobs() in
crates/fsci-cluster/benches/cluster_bench.rs.
"""
import time
import numpy as np
from sklearn.metrics import silhouette_score


def blobs(n, d):
    out = np.empty((n, d))
    for i in range(n):
        centre = float(i % 4)
        for j in range(d):
            t = float(i * (j + 1))
            out[i, j] = centre * 5.0 + np.sin(t * 0.013) * 0.5 + float((i + j) % 7) * 0.05
    return out


def med(fn, r=7):
    ts = []
    for _ in range(r):
        t0 = time.perf_counter(); fn(); ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    for n in (500, 2000):
        X = blobs(n, 4)
        labels = np.array([i % 4 for i in range(n)])
        print(f"sklearn silhouette_score n={n}: {med(lambda: silhouette_score(X, labels))*1e6:.2f} us")
