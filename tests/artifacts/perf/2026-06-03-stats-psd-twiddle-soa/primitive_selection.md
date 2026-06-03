# PSD Welch Twiddle SoA Primitive Selection

Bead: `frankenscipy-8l8r1.9`

Target: `time_series/psd_welch/4096_w128_o64`

Profile evidence:
- Latest broad stats reprofile after Sobol incremental closeout ranked `psd_welch` as the top remaining stats row: `464.40 us` median in `tests/artifacts/perf/2026-06-03-stats-sobol-incremental/reprofile_stats_after_sobol_incremental_rch.txt`.
- Fresh focused RCH Criterion baseline on this bead: `[716.01 us, 727.76 us, 740.68 us]` in `baseline_psd_welch_rch.txt`.

Candidate primitive:
- Split cached DFT twiddle storage from `Vec<(f64, f64)>` to separate row-major cosine and sine vectors.
- The alien-graveyard match is a layout primitive: use SoA/contiguous arrays to reduce tuple destructuring and make sequential numeric streams explicit for cache/locality and autovectorization.

Behavior contract:
- Preserve Hann window generation order and bits.
- Preserve twiddle angle generation order: frequency-major, then sample-major.
- Preserve per-sample operation sequence: `re += s * cos`, then `im -= s * sin`.
- Preserve segment order, frequency order, sample order, output order, normalization, and allocation-visible API.
- No RNG, tie-breaking, or global mutable state changes beyond the existing `OnceLock` plan cache shape.
- Golden PSD output must remain byte-identical: `85048a3c06ab045815cbeb238fee9e1e07a05c27ceed3c3782ec0fd5ea97c6b1`.

Score target:
- Impact: 2.0. The target is top-ranked in the stats profile, but this lever only improves cached table layout.
- Confidence: 3.0. Arithmetic order is preservable exactly, but memory-layout-only wins can be machine-noisy.
- Effort: 2.0. One local struct/layout change plus proof.
- Score: `3.0 = 2.0 * 3.0 / 2.0`.

Decision:
- Trial this lever only if the source diff stays layout-only.
- Reject and restore source if golden changes, validation fails, or focused RCH after does not show a real win.
