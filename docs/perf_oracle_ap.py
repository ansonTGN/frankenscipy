#!/usr/bin/env python3
"""Oracle: sklearn AffinityPropagation on the SAME deterministic blobs/similarity
the fsci-cluster bench (bench_affinity_propagation) uses. Mirrors blobs() and the
negative-squared-euclidean affinity from cluster_bench.rs.
"""
import time
import numpy as np
from sklearn.cluster import AffinityPropagation


def blobs(n, d):
    out = np.empty((n, d))
    for i in range(n):
        centre = float(i % 4)
        for j in range(d):
            t = float(i * (j + 1))
            out[i, j] = centre * 5.0 + np.sin(t * 0.013) * 0.5 + float((i + j) % 7) * 0.05
    return out


def neg_sq_euclid(X):
    # S[i,j] = -||x_i - x_j||^2
    sq = np.sum(X * X, axis=1)
    return -(sq[:, None] + sq[None, :] - 2.0 * X @ X.T)


def time_ap(n, d, max_iter, reps=5):
    X = blobs(n, d)
    S = neg_sq_euclid(X)
    pref = -50.0
    ts = []
    for _ in range(reps):
        t0 = time.perf_counter()
        ap = AffinityPropagation(
            affinity="precomputed", preference=pref, damping=0.9,
            max_iter=max_iter, convergence_iter=15, random_state=0,
        )
        ap.fit(S)
        ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    for (n, d, mi) in [(300, 4, 80), (1000, 4, 80), (2000, 4, 80)]:
        med = time_ap(n, d, mi)
        print(f"sklearn AffinityPropagation n={n} d={d} iter<={mi}: {med*1e3:.3f} ms (median of 5)")
