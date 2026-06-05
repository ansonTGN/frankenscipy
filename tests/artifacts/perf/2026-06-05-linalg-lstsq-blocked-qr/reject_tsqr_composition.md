# Rejected lever: nalgebra-QR TSQR composition

Bead: `frankenscipy-275gu`

Lever tried: route large tall full-rank `lstsq` through a TSQR-style composition: local nalgebra QR per row block, stack local R factors, solve the reduced least-squares problem by QR, and compute singular values on the reduced R stack for rank/certificate compatibility.

Decision: reject. The same-worker RCH probe regressed the first comparable `lstsq` row.

Baseline, RCH `vmi1227854`:
- `lstsq m=2000 n=1000`: 3388.7 ms
- `lstsq m=3000 n=1500`: 20309.9 ms

After TSQR composition, RCH `vmi1227854`:
- `lstsq m=2000 n=1000`: 4180.8 ms

Proof checks run before rejection:
- `cargo check -p fsci-linalg --all-targets --locked`: pass, with pre-existing warning in `perf_solve_probe`.
- `cargo test -p fsci-linalg --lib lstsq --locked -- --nocapture`: pass, 6/6.
- `cargo fmt -p fsci-linalg --check`: pass after removing the rejected source lever.

Why it failed: the composition still pays nalgebra QR overhead for large dense
blocks and does not create a BLAS-3 trailing-update kernel. It is
algorithmically TSQR-shaped, but it is not a viable production fast path for the
current `lstsq`/`pinv` contract.

Current next primitive: `frankenscipy-jvcdf`, an in-house blocked Householder
bidiagonalization path with compact reflectors, GEMM-backed trailing updates, and
a bidiagonal SVD solver/reconstruction path. Target: `lstsq m=3000 n=1500` below
2.5s while preserving the observable SVD contract: solution, rank, singular value
ordering, tolerances, error classes, and certificates.
