# LSAP SAP gauntlet - frankenscipy-zl4m5

- Agent: cod-a / BlackThrush
- Date: 2026-06-20
- Crate: `fsci-opt`
- Routine: `scipy.optimize.linear_sum_assignment` equivalent
- Decision: KEEP the SciPy-style shortest augmenting path core with owned
  reusable scratch; REJECT the row-major flat-cost scratch variant.

## Lever

The previous public `linear_sum_assignment` path used the e-maxx-style
rectangular Hungarian implementation. This patch ports the modified
Jonker-Volgenant shortest augmenting path route used by SciPy's
`rectangular_lsap` core, preserving finite-input validation and row-sorted
output.

The final kept variant uses one owned scratch workspace for path,
shortest-path costs, selected rows/columns, and remaining columns. That keeps
Clippy clean without the borrowed-slice aliasing regression observed in the
intermediate helper shape.

Reference material:

- SciPy 1.17.1 `scipy/optimize/rectangular_lsap/rectangular_lsap.cpp`
- Crouse, "On implementing 2D rectangular assignment algorithms", 2016

## Benchmark evidence

Commands used the requested per-crate target directory:

`CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a`

| Workload / route | Median | Interval / note | Verdict |
| --- | ---: | --- | --- |
| Baseline Rust current, `vmi1152480`, n=500 | 43.798 ms | [41.727, 45.836] ms | prior current |
| Final Rust SAP owned scratch, `vmi1152480`, n=500 | 28.681 ms | [26.828, 30.162] ms | 1.53x faster than current |
| Local SciPy 1.17.1 oracle, n=500 | 18.578689 ms | p50, min 18.456107 ms | Rust remains 1.54x slower |
| Baseline Rust current, `vmi1152480`, n=1000 | 349.80 ms | [332.54, 368.28] ms | prior current |
| Final Rust SAP owned scratch, `vmi1152480`, n=1000 | 199.52 ms | [182.05, 217.44] ms | 1.75x faster than current |
| Local SciPy 1.17.1 oracle, n=1000 | 122.932709 ms | p50, min 117.874174 ms | Rust remains 1.62x slower |

Win/loss/neutral:

- Same-worker final Rust versus current Rust: `2/0/0`.
- Strict final Rust versus local SciPy oracle: `0/2/0`.
- Rejected flat-cost sub-variant versus the first SAP candidate: `0/1/1`
  because n=500 regressed 1.27x and n=1000 was not a significant win.

Additional routing samples:

| Route | n=500 median | n=1000 median | Note |
| --- | ---: | ---: | --- |
| First SAP candidate, `vmi1152480` | 28.955 ms | 265.05 ms | Initial algorithmic win |
| Flat-cost sub-variant, `vmi1152480` | 36.674 ms | 243.46 ms | Reverted: n=500 regression |
| Final owned-scratch SAP, unpinned `vmi1227854` | 22.971 ms | 164.15 ms | Routing only; not used for same-worker ratio |

## Gates

- PASS: `rch exec -- cargo test -p fsci-opt linear_sum_assignment --lib -- --nocapture`
  = 8 passed / 0 failed.
- PASS: `rch exec -- cargo check -p fsci-opt --all-targets`.
- PASS: `rch exec -- cargo clippy -p fsci-opt --all-targets --no-deps -- -D warnings`.
- PASS: `rch exec -- cargo build --release -p fsci-opt`.
- PASS: local live SciPy conformance
  `cargo test -p fsci-conformance --test diff_opt_linear_sum_assignment -- --nocapture`
  = 1 passed / 0 failed.
- BLOCKED/ENV: the same conformance harness on rch failed before comparison
  because worker `hz2` had no SciPy module installed.
- BLOCKED/EXISTING: changed-file UBS scan exited nonzero on the existing
  broad `crates/fsci-opt/src/lib.rs` inventory (test-only panic callbacks,
  pre-existing unwrap/assert/indexing findings). No new unsafe code was
  introduced and no Clippy warnings remain.

`rustfmt --edition 2024 --check crates/fsci-opt/src/lib.rs` was clean after the
patch. Full workspace `cargo fmt --check` remains blocked by pre-existing
formatting drift in unrelated files/crates; output is captured in
`cargo_fmt_check.txt`.

## Negative evidence

Do not retry a naive row-major flat-cost copy inside this SAP path without a
new way to remove or amortize the n=500 penalty. The profitable lever was the
complexity-class/algorithm swap from Hungarian-style rectangular assignment to
SciPy's shortest augmenting path core plus owned reusable scratch. The next
credible route for strict SciPy parity is deeper memory layout work that avoids
both `Vec<Vec<f64>>` row indirection and per-call copies, or a specialized
public dense matrix representation/API.
