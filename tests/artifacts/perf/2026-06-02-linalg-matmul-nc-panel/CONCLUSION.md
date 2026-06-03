# matmul NC=64 column-panel traversal — ABANDONED (negative result)

Bead: frankenscipy-8l8r1.1 (child of 8l8r1 no-gaps umbrella)
Crate: fsci-linalg
Function: `matmul` (dense f64 GEMM)
Candidate lever: wrap the shipped MR=NR=4 register micro-kernel in an outer
                 NC=64 column-panel traversal to keep a B column-panel resident.
Verdict: ABANDONED — does not clear Score>=2.0 vs the currently-shipped
         no-panel micro-kernel (commit 629890da). matmul reverted to HEAD.
Date: 2026-06-02

## Why abandoned

The honest comparison is column-panel vs the ALREADY-SHIPPED no-panel 4x4
micro-kernel (not vs the long-replaced flat ikj). Same-run perf witness on RCH:

    run A: 768x768  column_panel vs no_panel = 0.67x   (regression)
           1024x1024 column_panel vs no_panel = 0.74x  (regression)
    run B: 768x768  column_panel vs no_panel = 0.75x   (regression)
           1024x1024 column_panel vs no_panel = 1.63x  (win, but noisy)

768³ regresses consistently (~0.7x); 1024³ is noisy (0.74x vs 1.63x across runs
on a heavily-contended host, ~2.4x variance in the no_panel baseline itself).
Per the loop's own Pass-3 fallback ("abandon if same-run no-panel vs
column-panel does not clear Score>=2.0"), the lever is abandoned. The "2.5–3.0x
vs_flat" figures in the raw witness files are vs the obsolete flat-ikj baseline
and are NOT the decision metric.

## Root cause

For these sizes the working set is dominated by the C register-tile traffic and
A/B feeding, not by B column-panel residency. Wrapping the kernel in an NC outer
loop re-walks A across every panel, adding A re-streaming that outweighs the
marginal B-locality gain. The no-panel kernel already keeps each C tile in
registers across the full k-sweep, which is where the win was.

## What was kept

- `crates/fsci-linalg/benches/linalg_bench.rs`: added a permanent `bench_matmul`
  criterion benchmark (256/512/768/1024) so future GEMM levers have a stable,
  low-noise measurement harness instead of ad-hoc ignored tests.
- matmul itself: UNCHANGED (HEAD micro-kernel, golden digest 0xf9aa16d2dc37468f).

## Guidance for the next GEMM lever

Naive row/cache-blocking (negative, see 2026-06-02-linalg-matmul-microkernel)
AND NC column-panel traversal (this) both regress at B-fits-L3 sizes. The real
remaining gap to BLAS at 1024³+ needs explicit panel PACKING (copy A/B into
contiguous, aligned, kernel-ordered scratch buffers) combined with a wider
MRxNR register kernel — packing is what makes blocking pay by turning strided
A/B access into unit-stride streams. Benchmark with `bench_matmul`, not ignored
tests, to escape host-contention noise.
