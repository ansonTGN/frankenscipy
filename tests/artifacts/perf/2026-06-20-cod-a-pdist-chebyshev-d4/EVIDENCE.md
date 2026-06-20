# frankenscipy-i0ghz: pdist Chebyshev d4 SoA SIMD

Agent: cod-a / BlackThrush
Date: 2026-06-20
Crate: `fsci-spatial`

## Lever

`pdist(..., DistanceMetric::Chebyshev)` now has a dim-4 fast path matching the
existing Euclidean/SqEuclidean/Cityblock/Cosine fixed-row infrastructure:

- validate and stage rows into `[f64; 4]`
- transpose to four SoA coordinate columns
- fill condensed rows with 8-wide SIMD across independent `(i, j)` pairs
- preserve the scalar helper's NaN-propagating max fold with an explicit lane
  NaN mask

No unsafe code was added. Generic dimensions still use the prior route.

## Raw Logs

- Baseline routing sweep:
  `baseline_pdist_sweep_rch.txt`
- Final Rust sweep:
  `candidate_pdist_sweep_rch.txt`
- Local SciPy oracle:
  `scipy_oracle_pdist_sweep.txt`
- Criterion bench:
  `criterion_pdist_chebyshev512_rch.txt`

## Commands

Baseline and final Rust sweep:

```bash
AGENT_NAME=BlackThrush \
RCH_REQUIRE_REMOTE=1 \
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a \
  rch exec -- cargo run --release -p fsci-spatial --bin perf_pdist_sweep
```

Criterion:

```bash
AGENT_NAME=BlackThrush \
RCH_REQUIRE_REMOTE=1 \
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a \
  rch exec -- cargo bench -p fsci-spatial --bench spatial_bench -- \
  pdist/chebyshev/512 --noplot --sample-size 10 --warm-up-time 1 \
  --measurement-time 2
```

SciPy oracle:

```bash
python3 - <<'PY'
# same deterministic inputs as perf_pdist_sweep
PY
```

## Measurements

Baseline run selected `vmi1264463`; final sweep selected `hz1`. The baseline
therefore supports route/keep evidence, but the strict decision is the final
head-to-head against the local SciPy oracle.

| Workload | Baseline Rust | Final Rust | SciPy oracle | Verdict |
| --- | ---: | ---: | ---: | --- |
| `pdist/chebyshev/n512/d4` | 2.141 ms | 0.173 ms | 0.175 ms | keep: Rust 1.01x faster than SciPy |
| Criterion `pdist/chebyshev/512` | n/a | 136.38 us median | 175 us | keep: Rust 1.28x faster than SciPy |
| `pdist/chebyshev/n512/d16` | 83.275 ms | 1.862 ms | 0.555 ms | residual loss: Rust 3.36x slower |
| `pdist/chebyshev/n512/d64` | 63.129 ms | 5.767 ms | 2.133 ms | residual loss: Rust 2.70x slower |
| `pdist/chebyshev/n2048/d64` | 464.103 ms | 71.833 ms | 39.290 ms | residual loss: Rust 1.83x slower |

Full final sweep vs SciPy:

| Workload | Final Rust | SciPy oracle | Ratio |
| --- | ---: | ---: | --- |
| `pdist/euclidean/n512/d4` | 0.234 ms | 0.306 ms | Rust 1.31x faster |
| `pdist/cityblock/n512/d4` | 0.160 ms | 0.191 ms | Rust 1.19x faster |
| `pdist/sqeuclidean/n512/d4` | 0.132 ms | 0.221 ms | Rust 1.67x faster |
| `pdist/chebyshev/n512/d4` | 0.173 ms | 0.175 ms | Rust 1.01x faster |
| `pdist/euclidean/n512/d16` | 1.140 ms | 0.756 ms | Rust 1.51x slower |
| `pdist/cityblock/n512/d16` | 1.072 ms | 0.588 ms | Rust 1.82x slower |
| `pdist/sqeuclidean/n512/d16` | 0.906 ms | 0.542 ms | Rust 1.67x slower |
| `pdist/chebyshev/n512/d16` | 1.862 ms | 0.555 ms | Rust 3.36x slower |
| `pdist/euclidean/n512/d64` | 2.223 ms | 2.180 ms | Rust 1.02x slower |
| `pdist/cityblock/n512/d64` | 1.543 ms | 2.682 ms | Rust 1.74x faster |
| `pdist/sqeuclidean/n512/d64` | 1.642 ms | 2.031 ms | Rust 1.24x faster |
| `pdist/chebyshev/n512/d64` | 5.767 ms | 2.133 ms | Rust 2.70x slower |
| `pdist/euclidean/n4096/d4` | 24.214 ms | 54.218 ms | Rust 2.24x faster |
| `pdist/cosine/n4096/d4` | 20.823 ms | 51.827 ms | Rust 2.49x faster |
| `pdist/chebyshev/n2048/d64` | 71.833 ms | 39.290 ms | Rust 1.83x slower |
| `pdist/cityblock/n2048/d64` | 19.724 ms | 44.039 ms | Rust 2.23x faster |

Win/loss/neutral vs SciPy: 8 / 6 / 0.

## Correctness And Gates

- PASS: `cargo test -p fsci-spatial pdist_dim4 --lib -- --nocapture` via rch:
  3 passed, including NaN fold preservation.
- PASS: `cargo test -p fsci-conformance --test diff_spatial_pdist_cdist --
  --nocapture`: 1 passed. RCH had no admissible worker and failed open locally.
- PASS: `cargo check -p fsci-spatial --all-targets` via rch.
- PASS: `cargo clippy -p fsci-spatial --all-targets --no-deps -- -D warnings`
  via rch.
- PASS: `cargo fmt --check -p fsci-spatial`.
- PASS: `git diff --check`.
- BLOCKED/EXISTING: changed-file `ubs` exited 1 on pre-existing broad
  `fsci-spatial` test panic / unwrap / direct-index findings in the touched
  file set. Compiler, clippy, focused tests, conformance, formatting, and diff
  hygiene are green for this patch.

## Decision

Keep. The tracked dim-4 Chebyshev SciPy gap is closed, and the implementation
is bit-identical to the public metric helper, including NaN propagation.

Do not retry dim-4 Chebyshev. The remaining spatial `pdist` losses are
Chebyshev d16/d64 and need a generic-width SIMD/blocking route.
