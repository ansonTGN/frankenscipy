# fsci-fft twiddle cache closeout

Date: 2026-06-02
Agent: OliveSnow
Bead: `frankenscipy-odvfk`

## Profile

Fresh fsci-fft profiling was selected because ready perf beads were drained and recent
memory covered fsci-linalg/fsci-stats work. The broad Criterion matrix identified the
largest non-linalg fsci-fft targets as:

| target | mean |
| --- | ---: |
| `baseline_fft/fft/262144` | 10.342 ms |
| `baseline_fft2/fft2/512x512` | 6.906 ms |
| `baseline_rfft/rfft/262144` | 5.434 ms |

The code hotspot was `get_or_compute_twiddles` in `crates/fsci-fft/src/transforms.rs`.
The cache stored `Vec<Complex64>` and returned a cloned full vector on every cache hit.
`cooley_tukey_radix2_inplace` reads the table immutably, so the clone copied data
without contributing to semantics.

## Lever

One lever was kept:

```text
TwiddleTable: Vec<Complex64> -> Arc<[Complex64]>
cache insert: full Vec clone -> Arc::clone handle
cache hit: full Vec clone -> Arc::clone handle
```

No FFT arithmetic, twiddle computation, butterfly loop order, normalization,
tie-breaking, RNG state, or trace ordering was changed.

## Benchmarks

Accepted target:

| state | target | mean | delta |
| --- | --- | ---: | ---: |
| pre-lever `Vec` | `baseline_fft/fft/262144` | 11.285 ms | baseline |
| after `Arc<[Complex64]>` | `baseline_fft/fft/262144` | 6.2833 ms | -44.3% |

Rejected probe:

| state | target | mean | verdict |
| --- | --- | ---: | --- |
| pre-lever `Vec` | `fft_real/rfft/1024` | 14.537 us | baseline |
| after `Arc<[Complex64]>` | `fft_real/rfft/1024` | 14.774 us | rejected as proof target |

Raw excerpts:

- `baseline_pre_arc_criterion.txt`
- `after_arc_criterion.txt`
- `rejected_rfft1024_probe.txt`

Score: impact 8 x confidence 4 / effort 2 = 16.0.

## Isomorphism

The cached value is still produced by the same loop:

```text
angle = sign * 2.0 * PI * k as f64 / n as f64
table.push((angle.cos(), angle.sin()))
```

Only the owner type changed. `Arc<[Complex64]>` dereferences to the same immutable
slice shape consumed by the FFT butterfly code. Cache miss values, cache hit values,
forward/inverse keying, loop order, floating-point operation order, RNG-free inputs,
and operation-id tie-breaking remain unchanged.

Golden/reference proof:

- `RCH_FORCE_REMOTE=1 ... rch exec -- cargo test -p fsci-fft --lib --locked -- --nocapture`
- Worker: `vmi1156319`
- Result: 144 fsci-fft library tests passed.
- Normalized reference artifact: `golden_twiddle_cache_normalized.txt`
- Sha256: recorded in `golden_twiddle_cache_normalized.sha256`

## Validation

Passed:

- `RCH_FORCE_REMOTE=1 rch exec -- cargo check -p fsci-fft --all-targets --locked`
- `RCH_FORCE_REMOTE=1 CARGO_TARGET_DIR=/data/tmp/cargo-target-frankenscipy-olivesnow-fft rch exec -- cargo test -p fsci-fft --lib --locked -- --nocapture`
- `RCH_FORCE_REMOTE=1 CARGO_TARGET_DIR=/data/tmp/cargo-target-frankenscipy-olivesnow-fft rch exec -- cargo clippy -p fsci-fft --all-targets --locked -- -D warnings`
- `cargo fmt -p fsci-fft --check`
