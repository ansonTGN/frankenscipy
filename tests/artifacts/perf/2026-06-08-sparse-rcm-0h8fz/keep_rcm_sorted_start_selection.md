# Keep: RCM start-node selection O(C·V) → O(V log V)

Bead: `frankenscipy-0h8fz`
Worker fleet: rch

## Lever

`csgraph::reverse_cuthill_mckee` selected each connected component's BFS start
node with `(0..n).filter(|i| !visited[i]).min_by_key(|i| degree[i])` — an O(V)
scan **per component**, so O(C·V) = O(V²) for fragmented / block-diagonal graphs
(many components), versus scipy's O(V+E).

Fix: pre-sort node indices by `(degree, index)` once with a **stable** sort
(equal degrees keep ascending index), then walk a cursor past visited nodes.
The first unvisited entry in that order is exactly the minimum-degree unvisited
node with the lowest index — identical to what `min_by_key` returned — but the
whole per-component start search is now O(V log V + V) total.

## Isomorphism / behavior parity

- The start selection is provably identical (min degree, ties → lowest index)
  and the BFS is unchanged, so the RCM ordering is **bit-for-bit identical**.
- New test `reverse_cuthill_mckee_matches_min_scan_reference_bit_for_bit`
  asserts the production output equals an inline copy of the previous
  min-scan-per-component implementation on a fragmented graph and a
  mixed-component graph: `assert_eq!` on the full permutation vectors.
- Perf probe confirms `orderings_match=true`.
- Existing `reverse_cuthill_mckee_matches_scipy_reference_values` still passes.
- Full `fsci-sparse` release suite: 311 passed, 0 failed.
- Connected graphs (C = 1) are unaffected — the single O(V) scan was already
  negligible next to the BFS; the win is for fragmented patterns.

## Performance (rch, 8000-node graph = 4000 disjoint 2-node components)

The probe's `min_scan_ref_ms` runs the previous O(C·V) selection on the same
graph (same-process A/B):

- run 1: `min-scan 19.18 ms → sorted-order 0.31 ms` = **61.4×**
- run 2: `min-scan 25.28 ms → sorted-order 0.36 ms` = **70.9×**
- `orderings_match = true`

## Score

`≈ 61–71×` on fragmented graphs; clears Score ≥ 2.0 by a wide margin.
**Keep.** Closes a genuine vs-scipy complexity gap (scipy RCM is O(V+E); we were
O(C·V) in the start search).
