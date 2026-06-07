# fsci-spatial halfspace N-D candidate parallelism

Bead: `frankenscipy-hv4do`

## Target

Profile-backed hotspot: `HalfspaceIntersection::from_nd` spends dominant time enumerating independent `C(m, ndim)` halfspace combinations, solving each tiny equality system, and scanning feasibility before a serial duplicate merge.

Lever: compute independent candidate solve+feasibility work in deterministic scoped worker chunks, then replay the existing duplicate/facet merge serially in the original lexicographic combo order.

## Baseline and After

RCH baseline, remote `vmi1156319`, `baseline_perf_halfspace_nd_rch.txt`:

- `m=120 ndim=3`: 171.026 ms/build
- `m=60 ndim=4`: 439.163 ms/build

RCH after, same remote `vmi1156319`, `after_parallel_candidates_rch.txt`:

- `m=120 ndim=3`: 67.085 ms/build, 2.55x faster
- `m=60 ndim=4`: 142.378 ms/build, 3.08x faster

Same-binary RCH serial-vs-parallel probe, remote `vmi1149989`, `ab_parallel_candidates_release_rch.txt`:

- `m=120 ndim=3`: serial 170.940076 ms, parallel 72.485893 ms, 2.358253x faster
- `m=60 ndim=4`: serial 401.420467 ms, parallel 164.642689 ms, 2.438131x faster

Score: `2.36 impact * 5 confidence / 2 effort = 5.9`, keep.

## Golden Parity

Golden payload SHA-256 is identical for baseline, after, and directional after-run:

`0ff8b232ce76580dadb2bff1f4f6bebc3d75837dc66454a873af6ea2c62ada49`

Payload:

```text
m=10 ndim=3 seed=1 nverts=16 chk=8b5b9c9d36c31dc9
m=20 ndim=3 seed=2 nverts=36 chk=912b75a80142fbf5
m=16 ndim=4 seed=3 nverts=58 chk=6a353199eb77efe3
```

## Isomorphism Proof

- Ordering: combinations are generated once by the existing recursive lexicographic enumerator; worker chunks preserve chunk order; the flattened candidate stream is replayed by the original serial duplicate/facet merge loop.
- Tie-breaking: approximate duplicate detection and facet extension stay serial, with the same first-accepted vertex winning every tie.
- Floating point: each candidate builds the same dense matrix and RHS, calls the same `solve_linear_system(..., 1e-10)`, and scans halfspaces in the same row order with tolerance `1e-8`. Cross-candidate scheduling cannot affect a candidate's arithmetic.
- RNG: production path has no RNG. Benchmark fixture RNG is deterministic and outside the production routine.
- Bit proof: `proof_parallel_candidates_bits_rch.txt` compares forced serial and forced parallel candidate streams for equal combo order and equal coordinate bits.

## Validation

- `rch exec -- cargo check -p fsci-spatial --all-targets --locked`
- `rch exec -- cargo clippy -p fsci-spatial --all-targets --no-deps --locked -- -D warnings`
- `rch exec -- cargo test -p fsci-spatial --lib halfspace_intersection -- --nocapture`
- `rch exec -- cargo test -p fsci-spatial --lib halfspace_vertex_candidates_parallel_matches_serial_bits_and_order -- --nocapture`
- `ubs crates/fsci-spatial/src/lib.rs`: exit 0, critical issues 0
