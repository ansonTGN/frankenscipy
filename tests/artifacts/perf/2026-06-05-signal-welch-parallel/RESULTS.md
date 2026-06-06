# perf(fsci-signal): parallelize welch segment periodograms (byte-identical)

## Lever
Welch's method averages independent per-segment periodograms (constant-detrend + window +
rfft). The segment loop was serial. Compute the per-segment PSDs in parallel into an
in-order Vec (thread::scope + chunked, reusing stft_frame_thread_count), then sum them
SEQUENTIALLY in segment order and divide. The averaging order is unchanged.

## Byte-identity
Per-segment computation is deterministic; the fold runs in segment order, so the PSD is
bit-identical to the serial loop. perf_welch FNV digests OLD(serial)==NEW(parallel):
  n=1000000 nperseg=512  psd=714822fd654cbb28 freq=75c769ecfa3c5b99
  n=2000000 nperseg=1024 psd=2084acd6272fba7b freq=1ef49732823bbf99
  n=4000000 nperseg=256  psd=9b861e63c8834bb5 freq=e2e05d527ad2a999

## Bench (perf_welch, release-perf, min of 3, 64 cores)
| n       | nperseg | serial   | parallel | speedup |
|---------|--------:|---------:|---------:|--------:|
| 1000000 |     512 | 20.54 ms |  9.19 ms |  2.23x  |
| 2000000 |    1024 | 40.16 ms | 12.15 ms |  3.32x  |
| 4000000 |     256 | 90.63 ms | 60.69 ms |  1.49x  |
Larger-FFT segments parallelize well (2-3.3x). nperseg=256 is overhead-bound: thousands of
tiny FFTs + per-segment periodogram allocation + the in-order sequential sum. 7 welch tests
pass; clippy clean.

## NEXT: inline a buffer-reusing per-segment periodogram (avoid the per-call spectrum/psd/
freq allocations) to push the small-nperseg case past 2x. Same parallel-segment lever
applies to csd/coherence (two-signal segment loops).
