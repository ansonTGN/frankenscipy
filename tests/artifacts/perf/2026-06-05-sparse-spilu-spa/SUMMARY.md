# fsci-sparse spilu: O(nnz) linear-scan index lookup -> O(1) dense workspace (SPA)

## Target (bead frankenscipy-ly4ks)

spilu (ILU(0), crates/fsci-sparse/src/linalg.rs) called find_index_in_row — an
O(row_nnz) linear scan — for every (i,j) fill update inside the elimination, giving
O(n·nnz^2)-class behavior.

## Lever (one)

Per active row i, scatter column -> data-index into a dense workspace
`row_lookup: Vec<usize>` (sentinel usize::MAX), reset via a `touched` list after the
row. Each (i,j) membership test/update is then O(1) instead of an O(row_nnz) scan
(the SPA / Gustavson pattern, cf. perf_sparse_spgemm_spa). find_index_in_row removed.

## Isomorphism / proof (BYTE-IDENTICAL)

Only the index *lookup* changes; the arithmetic (multiplier, subtract order, pivot
checks, L/U extraction) is untouched, so the factors are bit-identical. Proven two
ways:
  - in-process bit test `spilu_row_workspace_matches_linear_scan_factor_bits`
    compares the new spilu to a verbatim linear-scan reference via f64::to_bits()
    across (n,bw) = (16,3),(64,5),(160,7) — PASS.
  - perf_sparse spilu-golden sha256 3ed06db15de67484c3ce67e615e09f05a4936ac8479f100bbac032dd47a31484;
    A/B timing run checksums identical before/after (5.120019047832e4, 6.144020546457e4).
fsci-sparse spilu suite: 10 passed / 0 failed; lib clippy + fmt clean; benches compile.

## Rebench (perf_sparse spilu, banded, factor+solve)

| case | before (linear scan) | after (SPA) | speedup |
| --- | ---: | ---: | ---: |
| n=512  bw=16 | 1.666 ms | 0.442 ms | 3.77x |
| n=1024 bw=32 | 20.827 ms | 2.658 ms | 7.83x |

Win grows with bandwidth/fill (O(nnz^2) -> O(nnz)). Score >> 2.0, byte-identical.

## Attribution

Implemented by the CodexOpt optimizer pane (lever + bit-identity test + sparse_spilu
bench + perf_sparse spilu/spilu-golden subcommands); it stalled (credit-limited)
before committing. Verified end-to-end and shipped by frankenscipy-cc.
