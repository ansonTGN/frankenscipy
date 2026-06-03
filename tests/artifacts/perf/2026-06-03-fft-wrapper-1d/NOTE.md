# fsci-fft large-1D wrapper: dead finiteness-scan removal + no-2x-headroom finding

## Context

Follow-up to the `fft/262144` cache-blocking negative result
(`2026-06-03-fft-radix4-1d/`). That investigation noted the bare radix-2 kernel
was ~6 ms while a criterion `fft()` median read 17–21 ms, and flagged the fft()
**wrapper** as a possible untapped bit-identical lever.

## Finding: the wrapper gap was worker noise — no 2× lever there

Same-process decomposition (`perf_fft fftprobe 262144 41`, default `FftOptions`):

| | fft() | ifft() | isolated dead scan |
|---|-------|--------|--------------------|
| measured | **5.1–5.7 ms** | 5.0–5.4 ms | 0.123 ms |

`fft()` is ~5.5 ms — essentially equal to the bare kernel. The earlier 17–21 ms
criterion figure was a loaded/!pinned worker, **not** wrapper overhead. There is
no 2× to recover in the wrapper; the remaining gap vs SciPy (~2.5 ms) is the
kernel algorithm (split-radix / SIMD, which break bit-parity).

## Shipped: remove a provably-dead full-array scan from the default path

`FftOptions::default()` has `check_finite = false` and `mode = Strict`, so
`should_check = false`. But `validate_finite_complex_with_audit` (and the real
variant) ran `input.iter().any(!is_finite)` **before** testing the policy flag —
a full O(n) read pass over the 4 MiB buffer whose result is then ANDed with
`should_check` (false) and discarded (the gated `record_fail_closed` is also a
no-op without an audit ledger). The scan is reordered to short-circuit on the
cheap policy flag first, so the default `fft/ifft/rfft/irfft` path skips it.

- **Behavior-identical:** the `record_fail_closed` still fires exactly when
  `should_check && non-finite`; `validate_finite_complex` is unchanged. Full
  `fsci-fft` suite green (144 lib + 54), including `check_finite_rejects_nan_complex`
  and `check_finite_rejects_inf_real`.
- **Golden-identical:** rfft/irfft/fft2/polymul sha256 all match the values
  committed in `2026-06-03-fft-radix4-1d/` (e.g. fft2
  `79b17591…`, rfft `d3e41795…`).
- **Effect:** removes the 0.123 ms dead scan (~2.2% of a large fft()) per
  default-path call across all four 1D entry points. Small but free and correct;
  this is a dead-work removal, not a Score≥2.0 cache/algorithmic lever (none
  exists here — see above).

Kept the `perf_fft fftprobe` mode as the reusable wrapper-decomposition witness.
