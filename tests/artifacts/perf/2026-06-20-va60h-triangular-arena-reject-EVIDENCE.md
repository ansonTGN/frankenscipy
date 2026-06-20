# `frankenscipy-va60h` Triangular Linkage Arena Reject

Date: 2026-06-20

Agent: cod-a / MistyBirch

## Lever

Replace the retained flat full `(2n - 1) x (2n - 1)` inter-cluster linkage
arena with a compact upper-triangular arena.

Goal: reduce memory footprint and mirrored writes while preserving the same
ascending successor scan, strict `<` tie behavior, and Lance-Williams operation
order.

## Decision

Rejected and reverted. The candidate preserved exact outputs but regressed both
measured linkage rows.

## Commands

Baseline and after gauntlet:

```bash
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a-local cargo bench -p fsci-cluster --bench cluster_bench -- va60h_gauntlet_linkage --noplot
```

Correctness harness:

```bash
RCH_REQUIRE_REMOTE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo run -p fsci-cluster --release --bin perf_linkage
```

The remote harness printed `isomorphism: 0 mismatches / 7200 linkage matrices`;
`rch` then returned exit 102 because finished build artifacts timed out during
retrieval. The correctness output was emitted before that transfer failure.

Final restored-route guards:

```bash
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a-local cargo test -p fsci-cluster linkage_from_distances --lib -- --nocapture
FSCI_REQUIRE_SCIPY_ORACLE=1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a-local cargo test -p fsci-conformance --test diff_cluster_linkage_from_distances -- --nocapture
```

Both passed: the focused unit filter ran 2 tests, and the live SciPy
conformance test ran 1 test.

## Results

| Workload | Restored current | Triangular candidate | SciPy oracle | Verdict |
| --- | ---: | ---: | ---: | --- |
| `linkage(Average)`, n=800 d=4 | 7.5772 ms | 8.8260 ms | 4.2755 ms | candidate 1.165x slower than current; 2.064x slower than SciPy |
| `linkage(Ward)`, n=800 d=4 | 7.4597 ms | 9.9240 ms | 5.4866 ms | candidate 1.330x slower than current; 1.809x slower than SciPy |

Criterion reported the candidate `rust_current_flat` rows as statistically
regressed: `+19.020%` for Average and `+40.138%` for Ward.

## Follow-Up Route

Do not retry full-square-to-triangular arena layout for this NN-array linkage
path without a new profile showing the triangular index arithmetic and
merged-cluster scatter are no longer the dominant cost.

Next linkage work should target the algorithmic gap with SciPy's compiled
implementation: method-specific NN-chain or MST specializations, lower-constant
nearest-neighbour maintenance, or compiled-kernel-style branch reduction.
