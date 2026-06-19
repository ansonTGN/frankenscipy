#!/usr/bin/env python3
"""Oracle: scipy.cluster.hierarchy linkage + cophenet on the SAME deterministic
blobs the fsci-cluster bench (bench_hierarchical) uses (n=400, 4-D, average linkage).
Mirrors crates/fsci-cluster/benches/cluster_bench.rs::blobs.
"""
import time
import numpy as np
from scipy.cluster.hierarchy import linkage, cophenet
from scipy.spatial.distance import pdist


def blobs(n, d):
    out = np.empty((n, d))
    for i in range(n):
        centre = float(i % 4)
        for j in range(d):
            t = float(i * (j + 1))
            out[i, j] = centre * 5.0 + np.sin(t * 0.013) * 0.5 + float((i + j) % 7) * 0.05
    return out


def med(fn, reps=7):
    ts = []
    for _ in range(reps):
        t0 = time.perf_counter()
        fn()
        ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    n = 400
    X = blobs(n, 4)
    # scipy linkage takes observation matrix; computes pdist internally (euclidean).
    tl = med(lambda: linkage(X, method="average"))
    Z = linkage(X, method="average")
    # fsci cophenet(Z) returns distances only — use cophenet(Z) (no Y) for a fair
    # comparison; cophenet(Z, Y) would ALSO compute the correlation coefficient.
    tc = med(lambda: cophenet(Z))
    print(f"scipy linkage average n={n}: {tl*1e6:.2f} us")
    print(f"scipy cophenet(Z)   n={n}: {tc*1e6:.2f} us  (distances-only, fair)")
