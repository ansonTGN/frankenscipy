# frankenscipy-8l8r1.101 evidence

Target: `[perf][linalg] deepen large solve mixed-LU 2000x2000`.

Profile-backed source:
- `tests/artifacts/perf/2026-06-13-linalg-post-99-reprofile/linalg_bench_rch.txt`
- Top completed public post-99 row before stopping projected 4000x4000 collection:
  `baseline_solve/2000x2000` mean `720.64 ms`.

One lever:
- In `lu_factor_blocked_f32`, widen only the U12 triangular-solve vector loop from
  8 f32 lanes to 16 f32 lanes.
- No algorithm-selection, pivot-search, fallback, residual, tolerance, RNG, or output-order
  changes.

Same-worker benchmark:
- Worker: `vmi1152480`.
- Baseline command transcript: `baseline_solve_2000_rch.txt`.
- Baseline `baseline_solve/2000x2000`: `[420.33 ms 427.84 ms 435.64 ms]`.
- After command transcript: `after_solve_2000_u12_16_rch.txt`.
- After `baseline_solve/2000x2000`: `[381.74 ms 397.54 ms 413.37 ms]`.
- Mean delta: `427.84 ms -> 397.54 ms`, 30.30 ms faster, 7.08% reduction.
- Diagnostic exact-f64 arm was not targeted; observed `688.88 ms -> 683.31 ms`, consistent
  with noise and no exact-f64 code change.

Behavior proof:
- Pivoting: unchanged. Pivot search still scans rows in ascending order and uses the same
  strict `v > mx` tie-breaking rule.
- Floating point ordering: unchanged per output element. Each lane still executes
  `s -= L[i,p] * U[p,j]` for monotonically increasing `p`; only independent columns are
  grouped into a wider SIMD vector.
- Remainder handling: unchanged scalar loop for trailing columns not covered by the vector
  chunk.
- RNG: none on this path or in the deterministic benchmark fixture.
- Fallback and tolerance: `proof_mixed_lu_u12_16_rch.txt` passes
  `lu_solve_mixed_precision_matches_f64_and_falls_back`.
- Golden output: `proof_flat_lu_golden_digest_ignored_rch.txt` prints and checks
  `flat_lu_golden_digest=0x2fc8ed294ef0427c`.
- SHA256 manifest: `SHA256SUMS`.

Quality gates:
- `rustfmt --edition 2024 --check crates/fsci-linalg/src/lib.rs`: pass.
- `rch exec -- cargo check -p fsci-linalg --all-targets --locked`: pass.
- `rch exec -- cargo clippy -p fsci-linalg --all-targets --locked --no-deps -- -D warnings`: pass.
- `ubs crates/fsci-linalg/src/lib.rs`: exit 0; no critical findings, existing broad warning
  inventory only.

Score:
- Impact: 1.2, confidence: 0.95, effort: 0.35.
- Impact x Confidence / Effort = 3.26.
- Keep threshold met.
