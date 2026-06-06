# perf(fsci-interpolate): parallelize N-D interpolator eval_many (byte-identical)

## Lever
LinearNDInterpolator / CloughTocher2DInterpolator / NearestNDInterpolator had serial
eval_many (`queries.iter().map(|q| self.eval(q)).collect()`), each query an independent
expensive simplex / nearest-neighbour search + interpolation. Added a Result-aware
`par_query_try_map` (mirrors the existing par_query_map but for `Fn(&T)->Result<f64,E>`;
returns the FIRST error in query order, matching the serial `collect::<Result>()`), and
routed the three with per-eval work weights (linear=6 small so cheap-eval batches only
parallelize when large; clough/nearest higher). RBF already used par_query_map.

## Byte-identity
Each query is a pure independent eval written in query order; chunks concatenated in order.
perf_ndinterp FNV digests OLD(serial)==NEW(parallel) for all three x {20k,80k,200k}.

## Bench (perf_ndinterp, release-perf, min of 3, 64 cores; 2000-point 2D dataset)
| queries | linear | clough | nearest |
|---------|-------:|-------:|--------:|
| 20000   |  1.0x  |  1.0x  |  1.1x   |  (gated ~serial, no regression)
| 80000   |  2.0x  |  3.5x  |  3.5x   |
| 200000  |  2.3x  |  5.5x  |  7.0x   |
Grows with query count. 126 fsci-interpolate tests pass; clippy clean.
