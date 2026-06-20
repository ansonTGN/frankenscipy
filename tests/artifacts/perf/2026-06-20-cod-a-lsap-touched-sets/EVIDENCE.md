# LSAP touched-set dual update rejection

- Date: 2026-06-20
- Agent: cod-a / BlackThrush
- Bead: `frankenscipy-8l8r1.136`
- Crate: `fsci-opt`
- Workload: `linear_sum_assignment/dense/{500,1000}`
- Decision: REJECT / REVERT

## Lever

The tested lever replaced the full row/column scans in the modified
Jonker-Volgenant shortest augmenting path dual update with explicit
`touched_rows` and `touched_cols` vectors. The idea was a sparse frontier:
only rows and columns actually reached during the augmenting path search would
be updated after each augmentation.

That looked plausible because the current LSAP implementation still trails
SciPy in the tracked dense rows, and the full `sr`/`sc` scans are branch-heavy.
Measured results showed the opposite: the extra push/indirection overhead made
the dense case slower.

The source edit was reverted before commit. No `crates/fsci-opt/src/lib.rs`
diff remains in this closeout.

## Commands

Baseline, current main source:

```text
AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1152480 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a \
  rch exec -- cargo bench -p fsci-opt --bench optimize_bench -- linear_sum_assignment --noplot --sample-size 10 --warm-up-time 1 --measurement-time 2
```

rch selected worker `hz2`.

Candidate, touched-set source:

```text
AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 RCH_WORKER=hz2 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a \
  rch exec -- cargo bench -p fsci-opt --bench optimize_bench -- linear_sum_assignment --noplot --sample-size 10 --warm-up-time 1 --measurement-time 2
```

SciPy oracle:

```text
python3 - <<'PY'
import statistics
import time
import numpy as np
from scipy.optimize import linear_sum_assignment

U64_MAX = np.float64(np.iinfo(np.uint64).max)

def make_cost(n: int) -> np.ndarray:
    i = np.arange(n, dtype=np.uint64)[:, None]
    j = np.arange(n, dtype=np.uint64)[None, :]
    s = ((i * np.uint64(1103515245) + j) * np.uint64(12345)) ^ (
        (i * np.uint64(2654435761)) >> np.uint64(7)
    )
    scale = (np.arange(n, dtype=np.float64) % 9.0 + 1.0)[:, None]
    return (s.astype(np.float64) / U64_MAX) * scale

for n, reps in [(500, 9), (1000, 7)]:
    cost = make_cost(n)
    linear_sum_assignment(cost)
    samples = []
    for _ in range(reps):
        start = time.perf_counter()
        linear_sum_assignment(cost)
        samples.append((time.perf_counter() - start) * 1000.0)
    print(n, statistics.median(samples), sorted(samples)[-1])
PY
```

## Benchmark Evidence

| Workload | Current Rust median | Touched-set median | SciPy 1.17.1 median | Verdict |
| --- | ---: | ---: | ---: | --- |
| `linear_sum_assignment/dense/500` | 21.121 ms | 26.212 ms | 19.101180 ms | reject: touched-set is 1.24x slower than current; current Rust 1.11x slower than SciPy; touched-set 1.37x slower than SciPy |
| `linear_sum_assignment/dense/1000` | 135.72 ms | 167.30 ms | 127.840366 ms | reject: touched-set is 1.23x slower than current; current Rust 1.06x slower than SciPy; touched-set 1.31x slower than SciPy |

Criterion details:

- Baseline n=500: `[20.818 ms 21.121 ms 21.891 ms]`.
- Baseline n=1000: `[131.24 ms 135.72 ms 141.09 ms]`.
- Candidate n=500: `[25.820 ms 26.212 ms 26.799 ms]`, Criterion
  reported no statistically significant change, but the point estimate was
  worse and there was no win.
- Candidate n=1000: `[166.58 ms 167.30 ms 168.11 ms]`,
  `+23.271%` with `p = 0.00 < 0.05`, reported as a regression.
- SciPy n=500 samples: `19.101180,18.688017,18.742561,18.828934,23.375306,23.257352,19.723970,20.041712,18.755215`.
- SciPy n=1000 samples: `121.159332,126.543961,131.985428,122.795601,127.840366,134.422725,132.301376`.

Scores:

- Touched-set candidate versus current Rust: `0/1/1` (one significant
  regression, one no-win practical slowdown).
- Touched-set candidate versus SciPy oracle: `0/2/0`.
- Current main source versus this SciPy oracle snapshot: `0/2/0`, but the
  residual gap is now narrow at 1.11x and 1.06x.

## Verification

- PASS: source revert exactness:
  `git diff --exit-code -- crates/fsci-opt/src/lib.rs`.
- PASS: rch focused assignment tests:
  `AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo test -p fsci-opt linear_sum_assignment --lib -- --nocapture`
  = 9 passed / 0 failed on worker `vmi1264463`.
- PASS: rch release build:
  `AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1264463 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo build --release -p fsci-opt`.
- PASS: local live SciPy conformance:
  `AGENT_NAME=BlackThrush FSCI_REQUIRE_SCIPY_ORACLE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a cargo test -p fsci-conformance --test diff_opt_linear_sum_assignment -- --nocapture`
  = 1 passed / 0 failed.
- PASS: `git diff --check`.
- PASS: changed-file UBS on the docs/artifact/beads-only closeout exited 0
  with no recognizable code-language files to scan.

## Retry Guidance

Do not retry touched-row/touched-column dual updates for dense LSAP. The branch
scan is not the bottleneck at these sizes; the sparse frontier bookkeeping
worsens dense locality. The remaining credible route is a true dense layout
primitive that removes row-vector indirection without per-call copying, or a
lower-level LAP kernel that changes the memory traffic pattern more deeply.
