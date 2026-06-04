# fsci-linalg packed matmul closeout

Bead: `frankenscipy-jhtc6`

## Lever

One production lever shipped: pack full-width `B` panels once per call and widen the dense `matmul` register kernel from 4x4 to 4x8 for full tiles. Ragged column and row tails remain on the scalar monotonic-`k` path.

The primitive matches the no-gaps safe-Rust BLAS-class direction: improve the in-house GEMM kernel by reducing hot-loop `Vec` row-header chasing and streaming each packed `B` panel with unit stride. No C BLAS, MKL, XLA, unsafe code, or external numeric backend was introduced.

## Performance

Fresh RCH Criterion baseline before source edits:

| Row | Baseline median |
| --- | ---: |
| `matmul/256x256` | 16.317 ms |
| `matmul/512x512` | 172.64 ms |
| `matmul/768x768` | 1.3444 s |
| `matmul/1024x1024` | 2.5120 s |

RCH Criterion after the packed 4x8 lever:

| Row | After median | Speedup vs fresh baseline |
| --- | ---: | ---: |
| `matmul/256x256` | 10.451 ms | 1.56x |
| `matmul/512x512` | 81.651 ms | 2.11x |
| `matmul/768x768` | 292.84 ms | 4.59x |
| `matmul/1024x1024` | 688.48 ms | 3.65x |

Same-worker historical comparison against the prior 4x4 no-pack micro-kernel on `vmi1156319`:

| Row | No-pack 4x4 median | Packed 4x8 median | Speedup |
| --- | ---: | ---: | ---: |
| `768x768` | 775.8 ms | 292.84 ms | 2.65x |
| `1024x1024` | 1.2871 s | 688.48 ms | 1.87x |

Score: `8.0 = impact 4.0 * confidence 4.0 / effort 2.0`, above the `>=2.0` keep gate. Confidence is from RCH Criterion before/after rows, the same-worker historical comparison, and deterministic golden proof.

## Isomorphism proof

- API and error behavior are unchanged.
- Each output cell still accumulates `k = 0..ka` in monotonic order with separate `mul` then `add`; no floating-point reordering, reductions, or fused fast-math assumptions are introduced.
- Output write order changes only after each cell value is complete and is not externally observable.
- Ragged matrix panic surfaces remain direct-indexing surfaces as before.
- No ordering, tie-breaking, RNG, or global-state surface is touched.
- The deterministic matmul golden digest test stayed green on RCH.

## Evidence

- Baseline: `baseline_criterion_matmul_rch.txt` (`RCH` remote `vmi1264463`).
- Re-benchmark: `after_packed_criterion_matmul_rch.txt` (`RCH` remote `vmi1156319`).
- Golden: `after_packed_golden_tests_final2_rch.txt` (`RCH` remote `vmi1149989`, 2 passed, 0 failed, 1 perf test ignored).
- Clippy: `cargo_clippy_fsci_linalg_all_targets_final2_rch.txt` (`RCH` remote `vmi1156319`, exit 0).
- Broad crate test: `cargo_test_fsci_linalg_release_final_rch.txt` passed 343 lib tests, 34 differential tests, 59 metamorphic tests, and the focused repro/NAN tests; RCH fell open to local because another same-project remote job held the active-project lane, so this is supplemental rather than primary remote evidence.
- Broad crate remote retry: `cargo_test_fsci_linalg_release_remote_final_rch.txt` repeated the same passing broad crate test, but RCH again fell open to local (`hard_preflight=1`). The primary remote behavior proof remains the focused golden digest test plus RCH clippy and the remote Criterion baseline/rebench.
- Formatting: `cargo fmt -p fsci-linalg --check` passed.
- UBS: `ubs_linalg_changed_final2.txt` scanned the changed linalg files with 0 critical issues.
- Evidence integrity: `rch_outputs.sha256`.

## Shifted bottleneck

The post-lever top measured row is still dense `matmul/1024x1024` at 688.48 ms. A separate RCH same-project `matmul_microkernel_perf` witness also ran after this closeout started; the next profile-backed linalg GEMM lever should be a separate bead and commit, likely an A-panel/blocking lever or a larger architecture-aware register shape after another fresh RCH baseline.
