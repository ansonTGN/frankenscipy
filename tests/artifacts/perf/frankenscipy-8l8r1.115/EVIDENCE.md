# frankenscipy-8l8r1.115 randomized_eigh gauntlet

Date: 2026-06-19
Agent: cod-b / MistyBirch
Decision: KEEP

## Lever

`randomized_eigh` uses a projected symmetric sketch with a thin modified
Gram-Schmidt basis, deterministic random seed, and a full eigensolve only on the
small projected `q^T A q` matrix. The workload is low-rank symmetric dense input
where callers request only the top `k` eigenpairs.

## Commands

Remote build/check/test:

```bash
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo check -p fsci-linalg --all-targets
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo test -p fsci-linalg randomized_eigh_matches_full_eigh_on_low_rank --lib -- --nocapture
```

Remote Rust bench:

```bash
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b rch exec -- cargo bench -p fsci-linalg --bench linalg_bench randomized_eigh_gauntlet_scipy -- --noplot
```

Local same-host SciPy oracle bench, because rch worker `ovh-a` did not have
`scipy.linalg` importable:

```bash
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b cargo bench -p fsci-linalg --bench linalg_bench randomized_eigh_gauntlet_scipy -- --noplot
```

## Results

Criterion mean point estimates from
`/data/projects/.rch-targets/frankenscipy-cod-b/criterion/randomized_eigh_gauntlet_scipy/*/new/estimates.json`:

| Workload | Rust randomized mean | Rust full `eigh` mean | SciPy subset `eigh` mean | SciPy/Rust randomized | Full/Rand internal | Verdict |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| 256x256, k=16 | 3.492738 ms | 11.509742 ms | 4.879053 ms | 1.40x faster | 3.30x faster | win |
| 512x512, k=24 | 16.774593 ms | 136.046217 ms | 198.116969 ms | 11.81x faster | 8.11x faster | win |

The 512 SciPy row was noisy (`92.954415 ms` to `321.817958 ms` mean CI), but
the Rust randomized mean remains faster even against the SciPy lower bound
(`92.954415 / 16.774593 = 5.54x`).

Remote rch Rust-only bench on `ovh-a` also showed the same shape before the
SciPy oracle skipped:

| Workload | Rust randomized | Rust full `eigh` | Internal speedup |
| --- | ---: | ---: | ---: |
| 256x256, k=16 | 2.8202 ms | 10.220 ms | 3.62x |
| 512x512, k=24 | 13.917 ms | 109.66 ms | 7.88x |

## Guardrails

- PASS: `cargo check -p fsci-linalg --all-targets` through rch worker
  `vmi1227854`.
- PASS: focused low-rank randomized-eigh correctness test through rch worker
  `vmi1227854`.
- No revert. The measured route is a head-to-head SciPy win on the scoped
  low-rank top-k symmetric workload, and the in-crate full-eigh baseline remains
  a much slower fallback for this shape.
