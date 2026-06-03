# Conclusion: frankenscipy-3q382 — FFT 1D radix-2 cache-blocking (NEGATIVE RESULT)

## Target

- Bead: `frankenscipy-3q382`
- Crate: `fsci-fft`
- Hot path: `baseline_fft/fft/262144` — the top remaining 1D row from the prior
  FFT closeout (`tests/artifacts/perf/2026-06-02-fft-no-gaps-profile`), n = 2^18
  complex = a 4 MiB buffer.
- One lever tested: **L2 cache-blocked stage schedule** for
  `cooley_tukey_radix2_inplace_with_twiddles`.

## Lever

After the global bit-reversal, run the early Cooley-Tukey stages
(`stage_len <= BLOCK`, `BLOCK = 4096` complex = 64 KiB) block-by-block while each
window stays cache-resident, then run the remaining stages over the whole array.
Those early stages only ever pair indices within the same contiguous `BLOCK`
span, so the schedule is **bit-identical** to the flat schedule: each butterfly
keeps the same operands (post previous stage), the same `twiddles[k * stride]`
(the twiddle index depends only on `k` and `stage_len`, never on `base`), and the
same add-then-sub arithmetic. Only the visit order of mutually independent
butterflies changes, which cannot affect IEEE-754 results.

## Isomorphism / parity proof

Bit-identity was confirmed two ways before measuring perf:

1. A differential unit test compared the production `fft()` output against an
   independent flat-schedule reference, **byte-for-byte (`to_bits`)**, at sizes
   `[1, 2, 4, 8, 16, 64, 256, 1024, 2048, 4096, 8192, 16384, 65536, 262144]`
   (covering both the per-window phase and the whole-array phase). PASS.
2. The same-process A/B harness (`perf_fft ab`) asserts
   `bit_identical=true` for the flat vs blocked kernels every run (see
   `ab_flat_vs_blocked_rch.txt`).
3. Golden artifacts for the shared core (`golden_rfft_after.txt`,
   `golden_irfft_after.txt`, `golden_fft2_after.txt`, `golden_polymul_after.txt`)
   were captured; `golden_fft2_after.txt` sha256
   `79b17591371e8b8472ce6e9a89264628b960ae9fcde9f0e5c6b0c6de57bba5d8` **matches**
   the fft2 validation SHA from the prior committed FFT closeout — independent
   confirmation the change preserved output bits.

## Measurement — same-process A/B (worker-invariant)

`rch exec` cannot pin a worker, and an initial criterion baseline (19.18 ms on
`vmi1149989`) vs after-run landed on different workers (`vmi1156319`), so they
were not comparable. The decisive measurement runs BOTH kernels in one process
against identical data + twiddles (`perf_fft ab 262144 51`, 3 runs):

| run | flat median | blocked median | flat / blocked |
|-----|-------------|----------------|----------------|
| 1   | 6.0819 ms   | 6.0403 ms      | 1.0069         |
| 2   | 6.0718 ms   | 6.0298 ms      | 1.0070         |
| 3   | 6.0695 ms   | 5.9649 ms      | 1.0175         |

**Score ≈ 1.007–1.018 — NO win (gate is Score ≥ 2.0).**

## Root cause

At n = 2^18 the 4 MiB complex buffer **fits L3** on the bench workers (Contabo
"xl" VPS). Every `log2(n) = 18` butterfly pass is therefore already
cache-resident, so L2-blocking demotes nothing and removes no DRAM traffic — the
working set was never DRAM-bound. This is the same lesson recorded for the GEMM
NC-panel / row-blocking negative results ("B fits L3 so blocking only demotes hot
data from a faster to a slower cache").

A second observation: the bare kernel is ~6.0 ms while the criterion `fft()`
median was 17–21 ms. The remaining gap vs SciPy (~2.5 ms) therefore lives in (a)
the kernel FLOP count — a real 2× needs split-radix / radix-4 or SIMD, **both of
which change the butterfly association and break bit-parity** — and (b) the
`fft()` wrapper (input `to_vec`, finiteness validation over all n elements,
normalization pass, trace/audit), **not** in DRAM bandwidth.

## Disposition

- **Concurrent collision (resolved):** agent OliveSnow independently reserved all
  of `crates/fsci-fft/**` and ran the identical lever at the same time (their
  evidence dir: `tests/artifacts/perf/2026-06-03-fft-cache-block-stages/`).
  OliveSnow owns the crate, so I reverted my source edits (the kernel NOTE and the
  `perf_fft ab` harness) back to HEAD to avoid contaminating their reserved files,
  and bead `frankenscipy-3q382` was closed by OliveSnow as a negative result
  ("Criterion 5.4865 -> 5.4768 ms is noise across workers, Score 0.0; source
  restored"). No source change landed from this investigation.
- **Why this bundle is retained:** OliveSnow's close attributes the non-result to
  cross-worker criterion *noise*. This bundle adds the stronger, worker-invariant
  proof — a same-process A/B (flat vs blocked in one process) measuring a flat 1.007
  ratio — plus the L3-residency root cause. Together they show the lever is not
  merely unmeasurable here but genuinely worthless at these sizes, so it should not
  be re-attempted "with a cleaner bench."
- The `perf_fft ab` harness was used to produce `ab_flat_vs_blocked_rch.txt` but is
  NOT committed (it lived in OliveSnow's reserved file). To reproduce, re-add a
  two-kernel A/B harness; the kernels are the flat schedule and the
  `BLOCK = 1<<12` blocked schedule described above.
- `fsci-fft` source is HEAD-equivalent (144 lib tests green, clippy clean).

## Do-not-retry

- Do **not** re-attempt L2 cache-blocking of the radix-2 stages at these sizes —
  the array fits L3, there is no bandwidth to recover.
- A bit-parity-preserving win on this kernel must come from reducing work per
  element (hard without breaking FP bits) or from wrapper overhead, not from
  cache locality.
