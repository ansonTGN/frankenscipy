# ndimage gaussian_filter line-walk reject

Bead: `frankenscipy-acdq2`
Agent: cod-a / MistyBirch
Date: 2026-06-19

## Lever

Candidate forced `gaussian_filter1d_axis` onto `convolve1d_along_axis` for all
axes, then split the outermost-axis slab by rows and used direct indexing for
interior taps whose stencil stayed in bounds.

The idea came from the cache/SIMD branch of the BOLD-VERIFY search: remove
generic N-D fallback overhead, make the hot interior branchless, and expose
row-level parallelism when the outer slab count is too small.

## Bench Evidence

Command:

```bash
CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-a rch exec -- cargo bench -p fsci-ndimage --bench ndimage_bench -- correlate_gaussian/gaussian_sigma2/256 --noplot
```

Candidate, uncommitted scratch based on `e7b70c70`, rch worker `vmi1152480`:

```text
correlate_gaussian/gaussian_sigma2/256
time: [4.0337 ms 4.2236 ms 4.4246 ms]
```

Clean current `origin/main` at `96a37a83`, rch worker `ovh-a`:

```text
correlate_gaussian/gaussian_sigma2/256
time: [2.4545 ms 2.4792 ms 2.5044 ms]
```

Prior committed ledger current row: 3.238 ms. The candidate is slower than both
the clean current rerun and the prior committed current row.

Local SciPy oracle:

```bash
python3 docs/perf_oracle_ndimage.py
```

```text
scipy ndimage correlate 5x5 256x256: 1078.75 us
scipy ndimage gaussian sigma2 256x256: 1471.07 us
```

Candidate vs local SciPy oracle: 4.2236 ms / 1.47107 ms = 2.87x slower.
Clean current rch row vs local SciPy oracle: 2.4792 ms / 1.47107 ms = 1.69x
slower. These are routing ratios because the SciPy oracle is local and the Rust
timings are rch worker timings.

## Decision

Reject and revert. The source tree is restored to `origin/main` behavior; this
commit records only the negative evidence.

Do not retry this family as fallback removal, outermost-axis row-splitting, or
scalar direct-index boundary peeling. Future gaussian work should target the
inner contiguous dot kernel with SIMD/tiled spans, or a cache-tiled separable
scratch/transpose route, while preserving the gaussian tolerance contract.
