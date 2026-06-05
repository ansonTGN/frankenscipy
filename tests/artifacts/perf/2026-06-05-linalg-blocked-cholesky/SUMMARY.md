# fsci-linalg SPD solve: nalgebra single-threaded Cholesky -> in-house blocked Cholesky

## Target (bead frankenscipy-vycxu, Cholesky part)

solve(assume_a=PositiveDefinite) was excluded from the blocked-LU fast path, so it ran
nalgebra's single-threaded Cholesky. Probe (64-core worker): spd-solve n=2048 = 4427 ms.

## Lever (one) — our own blocked Cholesky

cholesky_solve_blocked: right-looking blocked Cholesky (A = L Lᵀ, NB=128). Each diagonal
block is factored unblocked (with sqrt), the panel below solved, then the trailing update
A22 -= L21·L21ᵀ — the O(n^3) bulk — runs on all cores via the multithreaded flat-workspace
GEMM. Wired as a solve() fast path for assume_a=PositiveDefinite (n>=1024, Strict,
untransposed, finite). A non-positive pivot (matrix not actually PD) returns None and falls
through to the portfolio solver, PRESERVING the exact `assume_a=pos` rejection semantics.

## Parity (tolerance)

Blocked Cholesky reproduces the standard Cholesky factorisation, so x matches scipy/nalgebra
to rounding. New test cholesky_solve_blocked_matches_reference: vs nalgebra SPD solve
max|dx| < 1e-7 AND residual ||Ax-b|| < 1e-9 across n=16/130/270; AND a non-PD matrix
(eigvals 3,-1) is rejected (None). fsci-linalg 350 passed / 0 failed; lib clippy + fmt clean.
Conformance cases (<1024) use the unchanged portfolio path.

## Rebench (perf_lstsq_probe, 64-core worker)

| op | before (nalgebra Cholesky) | after (blocked Cholesky) | speedup |
| --- | ---: | ---: | ---: |
| spd-solve n=1024 | 253.2 ms | 57.1 ms | 4.43x |
| spd-solve n=2048 | 4427.1 ms | 281.9 ms | 15.7x |

SPD solve at 2048: 4.4 s -> 0.28 s. Score >> 2.0.

matmul + solve + inv + SPD-solve are now all multicore safe-Rust kernels (no C BLAS).
REMAINING gap: lstsq/pinv (nalgebra QR/SVD) — m=3000 n=1500 = 16.3 s single-threaded;
next swing is blocked Householder QR with parallel trailing update.
