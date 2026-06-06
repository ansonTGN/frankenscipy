# perf(fsci-signal): parallelize resample_poly polyphase output samples (byte-identical)

## Lever
resample_poly's polyphase implementation computes each output sample as an independent FIR
dot product at upsampled position i = j*down over the taps `(k_start..n_taps).step_by(up)`.
The serial while-loop is replaced by a parallel fill across output samples (thread::scope +
chunks_mut, gated by n_out * taps_per_out). Each sample is written to its own index with the
same tap-summation order, so the result is bit-identical to the serial loop.

## Byte-identity
perf_resample_poly FNV digests OLD(serial)==NEW(parallel):
  n=200000 up=3 down=2  713ad5897242ee05
  n=400000 up=5 down=4  bb639972825e9c5e
  n=300000 up=7 down=3  df35b56320da88f8

## Bench (perf_resample_poly, release-perf, min of 3, 64 cores)
| n      | up | down | serial   | parallel | speedup |
|--------|----|------|---------:|---------:|--------:|
| 200000 | 3  | 2    | 14.31 ms |  3.71 ms |  3.9x   |
| 400000 | 5  | 4    | 24.51 ms |  4.34 ms |  5.6x   |
| 300000 | 7  | 3    | 33.96 ms |  6.01 ms |  5.6x   |
6 resample_poly tests pass; clippy clean.
