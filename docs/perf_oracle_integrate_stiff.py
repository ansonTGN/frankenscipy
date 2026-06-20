#!/usr/bin/env python3
"""SciPy oracle timings for stiff solve_ivp BDF/Radau benchmark rows."""

from __future__ import annotations

import json
import sys
import time
from collections.abc import Callable

import numpy as np
from scipy.integrate import solve_ivp


def stiff_decay_rhs(_t: float, y: np.ndarray) -> np.ndarray:
    denom = max(y.size - 1, 1)
    rates = 1.0 + 999.0 * (np.arange(y.size, dtype=float) / float(denom))
    return -rates * y


def case_config(mode: str) -> tuple[str, int, float]:
    if mode == "bdf-stiff64":
        return "BDF", 64, 0.5
    if mode == "bdf-stiff128":
        return "BDF", 128, 0.35
    if mode == "radau-stiff32":
        return "Radau", 32, 0.25
    if mode == "radau-stiff64":
        return "Radau", 64, 0.2
    raise SystemExit(f"unknown mode: {mode}")


def run_case(mode: str, repeats: int, rhs: Callable[[float, np.ndarray], np.ndarray]) -> dict:
    method, n, t_bound = case_config(mode)
    y0 = np.ones(n, dtype=float)

    # One warm-up keeps import and first-call setup out of the timed row.
    solve_ivp(rhs, (0.0, t_bound), y0, method=method, rtol=1e-6, atol=1e-8)

    checksum = 0.0
    nfev = 0
    njev = 0
    nlu = 0
    start = time.perf_counter()
    for _ in range(repeats):
        result = solve_ivp(rhs, (0.0, t_bound), y0, method=method, rtol=1e-6, atol=1e-8)
        checksum += float(np.sum(result.t)) + float(np.sum(result.y))
        nfev += int(result.nfev)
        njev += int(result.njev)
        nlu += int(result.nlu)
    elapsed = time.perf_counter() - start
    return {
        "mode": mode,
        "engine": "scipy",
        "repeats": repeats,
        "total_ms": elapsed * 1e3,
        "per_call_us": elapsed * 1e6 / repeats,
        "nfev": nfev,
        "njev": njev,
        "nlu": nlu,
        "checksum": checksum,
    }


def main() -> None:
    modes = ["bdf-stiff64", "bdf-stiff128", "radau-stiff32", "radau-stiff64"]
    if len(sys.argv) > 1:
        modes = sys.argv[1].split(",")
    repeats = int(sys.argv[2]) if len(sys.argv) > 2 else 10

    for mode in modes:
        print(json.dumps(run_case(mode, repeats, stiff_decay_rhs), sort_keys=True))


if __name__ == "__main__":
    main()
