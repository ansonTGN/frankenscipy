# perf: csgraph shortest_path — naive O(V²) Dijkstra → heap Dijkstra O((V+E)log V)

## Lever (ONE)
`fsci_sparse::shortest_path(graph, source, target)` ran Dijkstra with a **linear
scan** to select the minimum-distance unvisited node each step:

```rust
for _ in 0..n {
    // O(V) scan for the unvisited node with minimum dist
    let mut u = usize::MAX; let mut min_d = f64::INFINITY;
    for (i, (&d, &v)) in dist.iter().zip(visited.iter()).enumerate() {
        if !v && d < min_d { min_d = d; u = i; }
    }
    ...
}
```

That is O(V²) regardless of edge count. Replace the selection with a binary
min-heap keyed on `(cost, position)` plus a `visited` finalization guard →
O((V+E)·log V). (The sibling `dijkstra` already used a heap; this point-to-point
variant had been left naive.)

## Parity — BYTE-IDENTICAL
The heap pops by `(cost asc, position asc)` and finalizes each node once, so the
**sequence of selected nodes** — global-minimum unvisited distance, lowest index
on ties — is identical to the linear scan's. With the same CSR neighbour order
and the same strict `alt < dist[v]` relaxation, every `prev` assignment is the
same; therefore each distance's exact floating-point sum (accumulated along the
identically-chosen predecessor chain) and the reconstructed path are byte-
identical, for any edge-weight signs (both are visited-once Dijkstra).

- Same-process A/B (verbatim naive vs the library) over **2400 queries** across
  random graphs — half UNIT-weight (tie-heavy, the adversarial case for path
  tie-breaking), half random-weight: **0 mismatches** on `(dist.to_bits, path)`.
  See `golden_payload.txt` (digest 0dcfc008ef958bfd, sha256 4e88a955…).
- Conformance `diff_sparse_shortest_path_properties` passes.

## Timing — rch remote, 64 cores, `--profile release-perf`, same-process A/B
Random sparse directed graphs, 20 random source/target pairs each.

| n      | deg | naive O(V²) | heap      | speedup |
|--------|-----|-------------|-----------|---------|
| 2000   | 4   | 1.144 ms    | 93.1 µs   | 12.3x   |
| 8000   | 4   | 19.986 ms   | 496.2 µs  | 40.3x   |
| 20000  | 5   | 118.436 ms  | 1.693 ms  | 70.0x   |

Score ≥ 2.0 cleared with large margin; the win grows with V (O(V²) → O((V+E)logV)).

Harness: `crates/fsci-sparse/src/bin/perf_shortest_path.rs`
Run: `cargo run --profile release-perf -p fsci-sparse --bin perf_shortest_path`
