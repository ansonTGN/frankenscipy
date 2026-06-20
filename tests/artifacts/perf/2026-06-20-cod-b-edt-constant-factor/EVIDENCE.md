# frankenscipy-8l8r1.138 EDT constant-factor gauntlet

Agent: cod-b / BlackThrush

Decision: KEEP. `distance_transform_edt(return_indices=True)` no longer builds
the full background-coordinate `Vec<Vec<usize>>` before the exact separable
fast path, and 2-D inputs now fuse the final axis pass with row/column index
materialization. The exact 1-D lower-envelope kernel, axis order, all-foreground
fallback, and non-finite sampling fallback remain unchanged.

## Technique route

- `/alien-graveyard`: data-movement and allocation removal below an already
  fixed complexity class.
- `/alien-artifact-coding`: keep the mathematical EDT invariant and move only
  scratch/layout boundaries.
- `/extreme-software-optimization`: measure one path, keep only if same-worker
  timing and conformance support it.

## Bench commands

Baseline current Rust:

```bash
AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 RCH_WORKER=hz2 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo run --release -p fsci-ndimage --bin perf_edt
```

RCH selected `vmi1293453`.

Lazy-background intermediate:

```bash
AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1293453 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo run --release -p fsci-ndimage --bin perf_edt
```

Final fused 2-D path:

```bash
AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1293453 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo run --release -p fsci-ndimage --bin perf_edt
```

RCH selected `vmi1152480` for this comparable benchmark, matching the worker
used by the prior EDT scorecard rows from `frankenscipy-8l8r1.127`.

Post-cleanup final source:

```bash
AGENT_NAME=BlackThrush RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo run --release -p fsci-ndimage --bin perf_edt
```

RCH selected `vmi1149989`.

SciPy oracle:

```bash
python3 docs/perf_oracle_edt_indices.py --reps 20
```

RCH refuses non-compilation commands, so the SciPy oracle is local as in the
existing scorecard.

## Results

Same-session lazy-background proof on `vmi1293453`:

| Workload | Baseline Rust | Lazy-background Rust | Ratio |
| --- | ---: | ---: | ---: |
| 64x64 return_indices | 440.587 us | 225.229 us | 1.96x faster |
| 128x128 return_indices | 1.981 ms | 940.800 us | 2.11x faster |
| 192x192 return_indices | 5.469 ms | 5.272 ms | 1.04x faster |
| 256x256 return_indices | 8.885 ms | 5.229 ms | 1.70x faster |

Comparable fused path versus the prior `vmi1152480` EDT scorecard rows:

| Workload | Prior Rust `.127` | Fused Rust `.138` | Internal ratio |
| --- | ---: | ---: | ---: |
| 64x64 return_indices | 216.733 us | 161.471 us | 1.34x faster |
| 128x128 return_indices | 1.207 ms | 574.614 us | 2.10x faster |
| 192x192 return_indices | 2.107 ms | 2.166 ms | 0.97x, small loss |
| 256x256 return_indices | 4.855 ms | 3.787 ms | 1.28x faster |

Post-cleanup final Rust versus local SciPy 1.17.1 oracle:

| Workload | Final Rust `.138` | SciPy oracle | Ratio |
| --- | ---: | ---: | ---: |
| 64x64 return_indices | 104.120 us | 186.092 us | 1.79x faster |
| 128x128 return_indices | 677.777 us | 769.172 us | 1.13x faster |
| 192x192 return_indices | 1.470 ms | 2.346150 ms | 1.60x faster |
| 256x256 return_indices | 3.486 ms | 4.438267 ms | 1.27x faster |

Scores:

- Same-session lazy-background versus current Rust: `4/0/0`.
- Comparable fused path versus prior `vmi1152480` Rust rows: `3/1/0`.
- Post-cleanup final source versus local SciPy oracle: `4/0/0`.

## Correctness and gates

- PASS: `perf_edt` isomorphism printed `0 mismatches / 10876 cells` on
  baseline, lazy-background, and final runs; golden digest rows are unchanged.
- PASS: focused EDT tests via rch:
  `cargo test -p fsci-ndimage distance_transform_edt --lib -- --nocapture`
  = 15 passed / 0 failed.
- PASS: full ndimage lib tests via rch:
  `cargo test -p fsci-ndimage --lib -- --nocapture`
  = 246 passed / 0 failed / 5 ignored.
- PASS: per-crate compile via rch:
  `cargo check -p fsci-ndimage --all-targets`.
- PASS: local live SciPy conformance:
  `FSCI_REQUIRE_SCIPY_ORACLE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b-local-f20a cargo test -p fsci-conformance --test diff_ndimage_distance_transform_edt -- --nocapture`
  = 1 passed / 0 failed. The isolated local target dir avoids the stale shared
  target-dir rustc mismatch when RCH cannot provide a worker.
- PASS: touched-file formatting:
  `rustfmt --edition 2024 --check crates/fsci-ndimage/src/lib.rs`.
- PASS: diff hygiene:
  `git diff --check -- crates/fsci-ndimage/src/lib.rs`.
- PASS: changed-file UBS on `crates/fsci-ndimage/src/lib.rs` exits 0 with no
  critical issues; the broad pre-existing warning inventory remains.
- BLOCKED/EXISTING: `cargo fmt -p fsci-ndimage --check` still reports
  pre-existing drift in `crates/fsci-ndimage/benches/ndimage_bench.rs` and
  `crates/fsci-ndimage/src/bin/diff_fourier.rs`.
- BLOCKED/EXISTING: `cargo clippy -p fsci-ndimage --all-targets -- -D warnings`
  stops before this patch on existing `fsci-linalg` lints.

## Negative evidence

- The comparable `vmi1152480` 192x192 row is a small internal regression versus the prior
  `vmi1152480` Rust row (`2.107 ms -> 2.166 ms`, 0.97x), although it remains
  faster than today's local SciPy oracle (`2.346150 ms`). Do not call that row
  an internal win.
- RCH rejected remote conformance once because no admissible workers were
  available; the passing live-SciPy conformance was therefore local with an
  isolated target dir.
- The next EDT retry should not rebuild full background coordinate vectors for
  fast-path eligibility. Further work belongs below this layer: lower-envelope
  SIMD/branch reduction, row/column scratch transposition, or cache-blocked
  line batches.
