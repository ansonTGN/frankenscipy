# perf(fsci-signal): parallelize csd segment cross-periodograms (byte-identical)

## Lever
csd_with_scaling averages independent per-segment cross-periodograms (constant-detrend +
window + TWO rffts + conj(X)*Y). The segment loop was serial. Compute the per-segment
per-bin contributions (with the one-sided factor applied) in parallel into an in-order Vec
(thread::scope + chunked, reusing stft_frame_thread_count), then sum SEQUENTIALLY in
segment order and apply the final scale. Two FFTs per segment => more parallel-worthy than
welch. coherence (welch + csd) inherits the win.

## Byte-identity
Per-segment computation is deterministic; the fold runs in segment order. perf_csd FNV
digest of the complex CSD OLD(serial)==NEW(parallel):
  n=1000000 nperseg=512  csd=b4f32dfb77890453
  n=2000000 nperseg=1024 csd=9ec4323d444d8c31
  n=4000000 nperseg=256  csd=63844ff53b1954c8

## Bench (perf_csd, release-perf, min of 3, 64 cores)
| n       | nperseg | serial   | parallel | speedup |
|---------|--------:|---------:|---------:|--------:|
| 1000000 |     512 | 35.78 ms | 15.20 ms |  2.35x  |
| 2000000 |    1024 | 66.25 ms | 18.52 ms |  3.58x  |
| 4000000 |     256 | 154.2 ms | 117.6 ms |  1.31x  |
Larger-FFT segments parallelize well (2.35-3.58x). nperseg=256 overhead-bound (tiny FFTs +
per-segment alloc + in-order sum). 3 csd + 5 coherence tests pass; clippy clean.
