# frankenscipy-8l8r1.123 - jnjnp_zeros cutoff-driven generator

Date: 2026-06-19
Agent: cod-b / MistyBirch
Decision: KEEP
Rust worker: `ovh-b` via `rch`
Local SciPy oracle host: `thinkstation1`

## Lever

`jnjnp_zeros(nt)` now tries a cutoff-driven triangular generator before the
existing rectangular frontier fallback. The generator estimates a global zero
cutoff, emits only function and derivative roots below that cutoff, and accepts
the result only when both the first omitted serial root and the first omitted
order root exceed the retained `nt`th zero. This keeps the existing
frontier-proof fallback for misses.

I restored the public `jn_zeros` loop body after an intermediate helper
refactor, so this lever only changes the `jnjnp_zeros(nt >= 16)` path.

## Commands

Fresh pre-edit baseline and final candidate bench:

```bash
RCH_REQUIRE_REMOTE=1 RCH_WORKER=ovh-b CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo bench -p fsci-special --bench special_bench -- acoco_gauntlet_jnjnp_zeros --noplot
```

Focused correctness:

```bash
RCH_REQUIRE_REMOTE=1 RCH_WORKER=ovh-b CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo test -p fsci-special jnjnp -- --nocapture
```

Build gate:

```bash
RCH_REQUIRE_REMOTE=1 RCH_WORKER=ovh-b CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo check -p fsci-special --all-targets
```

Live SciPy conformance:

```bash
FSCI_REQUIRE_SCIPY_ORACLE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  cargo test -p fsci-conformance --test diff_special_bessel_zeros -- --nocapture
```

SciPy timing oracle:

```bash
python3 - <<'PY'
import statistics, time
import numpy as np
import scipy
from scipy import special
print(f"python scipy={scipy.__version__} numpy={np.__version__}")
for nt in (64, 128):
    for _ in range(10):
        special.jnjnp_zeros(nt)
    samples = []
    for _ in range(31):
        t0 = time.perf_counter()
        special.jnjnp_zeros(nt)
        samples.append(time.perf_counter() - t0)
    med = statistics.median(samples)
    mean = statistics.mean(samples)
    p95 = statistics.quantiles(samples, n=20)[18]
    zo, n, m, t = special.jnjnp_zeros(nt)
    print(nt, med, mean, p95, int(max(n)), int(max(m)), float(zo[-1]))
PY
```

## Same-worker Rust A/B

| Workload | Pre-edit current mean | Candidate current mean | Candidate/baseline | Verdict |
| --- | ---: | ---: | ---: | --- |
| `jnjnp_zeros(nt=64)` | 3.6486 ms | 1.5856 ms | 0.435x time, 2.30x faster | keep |
| `jnjnp_zeros(nt=128)` | 6.6226 ms | 2.9035 ms | 0.438x time, 2.28x faster | keep |

The final bench ran after restoring the public `jn_zeros` loop body. The
non-shipping `rust_legacy_duplicate` comparator stayed in Criterion's
no-change/noise band in the final run.

## SciPy Head-to-head

Local oracle versions: Python 3.13.7, NumPy 2.4.3, SciPy 1.17.1.

| Workload | Candidate Rust mean | SciPy median | Candidate/SciPy | Verdict |
| --- | ---: | ---: | ---: | --- |
| `jnjnp_zeros(nt=64)` | 1.5856 ms | 427.47 us | 3.71x slower | residual loss |
| `jnjnp_zeros(nt=128)` | 2.9035 ms | 789.23 us | 3.68x slower | residual loss |

SciPy win/loss/neutral: `0/2/0`.
Same-worker internal keep/loss/neutral: `2/0/0`.

## Correctness And Gates

- PASS: `jnyn_and_jnjnp_zeros_match_scipy`.
- PASS: `jnjnp_adaptive_envelope_matches_oversized_reference`.
- PASS: `jnjnp_frontier_matches_scipy_bench_cutoffs`.
- PASS: live SciPy `diff_special_bessel_zeros` conformance (`1 passed`).
- PASS: `cargo check -p fsci-special --all-targets` via `rch` on `ovh-b`.
- BLOCKED: `cargo clippy -p fsci-special --all-targets -- -D warnings`
  stops before this patch on existing `fsci-integrate` `too_many_arguments`
  and `fsci-linalg` `needless_range_loop` / `needless_borrow` lints.
- BLOCKED: broad `cargo fmt --check` is blocked by pre-existing workspace
  rustfmt drift; touched-file `rustfmt --edition 2024 --check
  crates/fsci-special/src/bessel.rs` is blocked by older drift in the same
  file outside the edited block.

## Decision

KEEP. The cutoff-driven generator is a real same-worker speedup and preserves
the SciPy output contract in focused and live-oracle tests. It still loses to
SciPy by about 3.7x, so the next credible route is not another envelope tweak:
profile the per-root scalar kernel, cross-order recurrence reuse, or a
Specfun-style global enumerator/codegen path.
