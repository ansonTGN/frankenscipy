# frankenscipy-3q382 - FFT cache-blocked early stages

Agent: OliveSnow
Date: 2026-06-03

## Target

Profile-backed hotspot: `fsci-fft` 1D radix-2 FFT at `baseline_fft/fft/262144`.

- Broad reprofile before this bead: `tests/artifacts/perf/2026-06-02-fft-no-gaps-profile/reprofile_fft_broad_after_axis_plan_rch.txt`
- Broad profile median: 5.7284 ms
- Focused RCH/Criterion baseline command:
  `rch exec -- cargo bench -p fsci-fft --bench fft_bench --locked -- baseline_fft/fft/262144 --warm-up-time 1 --measurement-time 2 --sample-size 10 --noplot`
- Focused baseline median: 5.4865 ms

## Lever Tested

Cache-block the first independent radix-2 FFT stages after bit reversal, running all stages up to a 4096-complex block inside each block before resuming the global stages.

The intent was to keep the early-stage butterfly windows in L2 and reduce full-array passes for `n=262144`.

## Isomorphism Proof

- Input generation unchanged.
- Bit-reversal permutation unchanged.
- Butterfly arithmetic unchanged: same twiddle index, same complex multiply, same add/sub assignment.
- The only intended ordering change was between independent butterflies whose read/write index sets are disjoint.
- No RNG, tie-breaking, tolerance, or floating-point approximation policy changed.
- Golden-output check covers the full `n=262144` complex FFT output as raw `f64::to_bits()` pairs.

Golden sha before:

```text
ba07ec573869b3da6cc62fb8a556070bf8426e4b98c2c377dbe22d1d34eaa253
```

Golden sha after:

```text
ba07ec573869b3da6cc62fb8a556070bf8426e4b98c2c377dbe22d1d34eaa253
```

Comparison: `golden_cmp=identical`.

## Benchmark Result

Focused Criterion baseline:

```text
baseline_fft/fft/262144 time:   [5.4283 ms 5.4865 ms 5.5473 ms]
```

After cache-blocking:

```text
baseline_fft/fft/262144 time:   [5.4107 ms 5.4768 ms 5.5411 ms]
```

Delta: 5.4865 ms -> 5.4768 ms median, a 1.0018x apparent speedup across different RCH workers. This is noise, not a real win.

Score: 0.0 (Impact 0 x Confidence 0 / Effort 1).

## Verdict

Rejected. Source was restored; no FFT code change was kept.
