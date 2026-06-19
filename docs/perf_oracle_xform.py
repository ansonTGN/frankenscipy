#!/usr/bin/env python3
"""Oracle: scipy.spatial.transform.Rotation.apply on the SAME 8192 points + quaternion
the fsci-spatial bench_transform_batch uses. Mirrors spatial_bench.rs (apply_many vs
map-apply; here vs scipy's vectorized apply).
"""
import time
import numpy as np
from scipy.spatial.transform import Rotation


def med(fn, r=15):
    ts = []
    for _ in range(r):
        t0 = time.perf_counter(); fn(); ts.append(time.perf_counter() - t0)
    return sorted(ts)[len(ts) // 2]


if __name__ == "__main__":
    n = 8192
    pts = np.array([[t * 0.001, np.sin(t * 0.7), np.cos(t * 0.3)] for t in range(n)])
    # scipy quat order is [x,y,z,w]; fsci from_quat([x,y,z,w]) — match.
    q = [0.022260026714733816, 0.43967973954090955, 0.3604234056503559, 0.8223631719059994]
    r = Rotation.from_quat(q)
    print(f"scipy Rotation.apply {n} pts: {med(lambda: r.apply(pts))*1e6:.2f} us")
