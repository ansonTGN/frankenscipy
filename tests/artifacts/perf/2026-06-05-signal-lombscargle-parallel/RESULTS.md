# perf(fsci-signal): parallelize lombscargle across frequencies (byte-identical)

## Lever
lombscargle is O(m*n): each of m frequencies computes its periodogram value as an
independent O(n) reduction over the samples (two fixed-order passes over x/y). The
frequency loop was serial. Hoist the per-frequency body into a pure closure and split
the m frequencies across threads into disjoint output slices (thread::scope + chunks_mut,
work-gated by m*n).

## Byte-identity
Each power[k] = power_at(freqs[k]), identical arithmetic, fixed sample-summation order,
written to its own index. perf_lombscargle FNV digests OLD(serial)==NEW(parallel):
  n=2000 m=2000  raw=77f7db2ed252780f norm=6393bc43b7e02d76
  n=4000 m=4000  raw=9cee541383c80148 norm=9b951093c3c233c7
  n=8000 m=6000  raw=7bd46eed8f3e610f norm=a472e7b0e39d7afa

## Bench (perf_lombscargle, release-perf, min of 3, 64 cores)
| n    | m    | serial   | parallel | speedup |
|------|------|---------:|---------:|--------:|
| 2000 | 2000 | 109.4 ms | 11.68 ms |  9.4x   |
| 4000 | 4000 | 434.8 ms | 27.83 ms | 15.6x   |
| 8000 | 6000 | 1309  ms | 67.37 ms | 19.5x   |
6 lombscargle tests pass; clippy clean.
