# fsci-integrate stiff BDF/Radau stream-norm gauntlet

- Date: 2026-06-20
- Agent: cod-a / MistyBirch
- Beads: `frankenscipy-8l8r1.119`, `frankenscipy-8l8r1.120`, `frankenscipy-8l8r1.121`
- Target crate: `fsci-integrate`
- Benchmark harness: `crates/fsci-integrate/src/bin/perf_integrate.rs`
- SciPy oracle: `docs/perf_oracle_integrate_stiff.py`
- Target dir: `CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a`

## Decision

Keep the BDF streamed scaled RMS helper from `.119/.121` and reject/revert the
Radau streamed scaled RMS helper from `.120`.

The same-worker `hz2` A/B run showed BDF64 improved and BDF128 was within noise,
while Radau32 and Radau64 regressed. The committed source restores Radau's
collected scaled-vector path and keeps the BDF streaming path.

## Internal A/B

Baseline was a detached worktree at `d502c74814c7d281b46e3784475bc413289dbf37`
with only the benchmark harness patch applied. Candidate was current source
before the Radau revert. Both rows below ran through:

```text
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a \
  rch exec -- cargo run --release -p fsci-integrate --bin perf_integrate -- stiff-suite 10
```

| Mode | Baseline on hz2 | Candidate on hz2 | Ratio | Decision |
| --- | ---: | ---: | ---: | --- |
| `bdf-stiff64` | 2390.435500 us | 2298.069000 us | 1.040x faster | keep |
| `bdf-stiff128` | 12032.349600 us | 12138.374200 us | 0.991x | neutral |
| `radau-stiff32` | 12586.934900 us | 14971.401500 us | 0.841x | reject |
| `radau-stiff64` | 78394.827700 us | 81492.956400 us | 0.962x | reject |

Step counters and checksums matched between baseline and candidate for every
row, so the Radau loss was not explained by a different step path:
`nfev/njev/nlu` were `10550/10/2920` for Radau32 and `10450/10/2800` for
Radau64 in both runs.

## Final Source vs SciPy

Final source means after the Radau revert, measured on rch worker `ovh-a`:

```text
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a \
  rch exec -- cargo run --release -p fsci-integrate --bin perf_integrate -- stiff-suite 10
```

SciPy oracle ran locally because direct SSH to the rch worker was unavailable
and the existing campaign accepts local SciPy oracle rows when worker images do
not provide a usable oracle:

```text
python3 docs/perf_oracle_integrate_stiff.py bdf-stiff64,bdf-stiff128,radau-stiff32,radau-stiff64 10
```

| Mode | Final Rust | SciPy oracle | Ratio vs SciPy | Verdict |
| --- | ---: | ---: | ---: | --- |
| `bdf-stiff64` | 1959.286800 us | 26351.239008 us | 13.45x faster | win |
| `bdf-stiff128` | 11052.293000 us | 29334.902694 us | 2.65x faster | win |
| `radau-stiff32` | 10191.487800 us | 33444.223704 us | 3.28x faster | win |
| `radau-stiff64` | 70176.946400 us | 35156.708304 us | 2.00x slower | loss |

Final win/loss/neutral vs SciPy: `3/1/0`.

## Gates

| Gate | Result | Notes |
| --- | --- | --- |
| Release build/run | PASS | `rch exec -- cargo run --release -p fsci-integrate --bin perf_integrate -- stiff-suite 10` |
| BDF focused tests | PASS | `cargo test -p fsci-integrate bdf --lib -- --nocapture` via rch: 16 passed / 0 failed |
| Radau focused tests | PASS | `cargo test -p fsci-integrate radau --lib -- --nocapture` via rch: 2 passed / 0 failed |
| IVP conformance | PASS | `cargo test -p fsci-conformance --test e2e_ivp -- --nocapture` via rch: 11 passed / 0 failed |
| Compile | PASS | `cargo check -p fsci-integrate --all-targets` via rch |
| Formatting | PASS | `rustfmt --edition 2024 --check crates/fsci-integrate/src/bin/perf_integrate.rs crates/fsci-integrate/src/radau.rs` |
| Diff hygiene | PASS | `git diff --check` |
| UBS | PASS | Exact changed-file scan: critical 0, warnings are heuristic inventory |
| Clippy | BLOCKED | Existing `fsci-integrate` lints outside touched files: `api.rs` too-many-arguments, `rk.rs` too-many-arguments/type-complexity, `quad.rs` excessive-precision/type-complexity. Follow-up: `frankenscipy-3qjah`. |

## Negative Evidence

Do not retry the Radau streamed scaled RMS formulation from `.120` unless a new
profile proves scaled-norm vector allocation is again a top integrate hotspot.
The same-worker reject was clear on the scoped Radau rows, and the remaining
measured SciPy loss is `radau-stiff64`, where the next plausible work is Radau
stage linear algebra/LU reuse, DMatrix/DVector assembly, stage-major cache
layout, or structured-Jacobian exploitation. Follow-up: `frankenscipy-zpunl`.
