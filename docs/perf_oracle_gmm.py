#!/usr/bin/env python3
"""Head-to-head oracle: time sklearn GaussianMixture on the SAME deterministic
`blobs` workload the fsci-cluster criterion bench (bench_gmm) uses, so the wall
clocks are comparable. Mirrors crates/fsci-cluster/benches/cluster_bench.rs::blobs.
"""
import time
import numpy as np
from sklearn.mixture import GaussianMixture


def blobs(n, d):
    out = np.empty((n, d))
    for i in range(n):
        centre = float(i % 4)
        for j in range(d):
            t = float(i * (j + 1))
            out[i, j] = centre * 5.0 + np.sin(t * 0.013) * 0.5 + float((i + j) % 7) * 0.05
    return out


def time_gmm(n, d, k, max_iter, reps=7):
    X = blobs(n, d)
    ts = []
    for _ in range(reps):
        t0 = time.perf_counter()
        # diagonal covariance to match fsci gaussian_mixture (per-dim variances)
        gm = GaussianMixture(
            n_components=k, covariance_type="diag", max_iter=max_iter,
            tol=1e-4, reg_covar=1e-6, n_init=1, init_params="random_from_data",
            random_state=42,
        )
        gm.fit(X)
        ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]  # median


if __name__ == "__main__":
    for (n, d, k, mi) in [(1000, 3, 3, 50), (5000, 8, 5, 50), (20000, 16, 8, 50)]:
        med = time_gmm(n, d, k, mi)
        print(f"sklearn GMM diag n={n} d={d} k={k} iter<={mi}: {med*1e3:.3f} ms (median of 7)")
