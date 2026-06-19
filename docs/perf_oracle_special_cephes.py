#!/usr/bin/env python3
"""SciPy single-core oracle for the Cephes-rational special kernels (erf, j0)
and the bessel-zero family. Times scipy.special on the SAME 65536-element grid
the fsci `special_array_65536` Criterion bench uses, plus jnjnp_zeros.

Usage: python3 docs/perf_oracle_special_cephes.py [--reps N] [--warmups W]
"""
import argparse
import statistics
import time

import numpy as np
from scipy import special


def bench(fn, reps, warmups):
    for _ in range(warmups):
        fn()
    samples = []
    for _ in range(reps):
        t0 = time.perf_counter()
        fn()
        samples.append(time.perf_counter() - t0)
    samples.sort()
    return statistics.median(samples)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--reps", type=int, default=200)
    ap.add_argument("--warmups", type=int, default=10)
    args = ap.parse_args()

    # Mirror the Rust bench grid: 0.5 + i*0.0001 for i in 0..65536.
    xs = 0.5 + np.arange(65536, dtype=np.float64) * 0.0001

    print(f"scipy {special.__name__} oracle, n={xs.size}, reps={args.reps}")
    for name, fn in [
        ("erf", lambda: special.erf(xs)),
        ("j0", lambda: special.j0(xs)),
        ("gamma", lambda: special.gamma(xs)),
    ]:
        p50 = bench(fn, args.reps, args.warmups)
        print(f"  {name:8s} p50 = {p50*1e6:12.3f} us")

    # jnjnp_zeros equivalent: scipy.special.jnjnp_zeros(nt)
    for nt in (64, 128):
        p50 = bench(lambda: special.jnjnp_zeros(nt), max(20, args.reps // 4), 3)
        print(f"  jnjnp_zeros(nt={nt}) p50 = {p50*1e6:12.3f} us")


if __name__ == "__main__":
    main()
