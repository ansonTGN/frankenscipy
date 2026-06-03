# PSD Welch Twiddle SoA Conclusion

## Target

- Bead: `frankenscipy-8l8r1.9`
- Profile-backed row: `time_series/psd_welch/4096_w128_o64`
- Lever: split cached Welch DFT twiddle storage from `Vec<(f64, f64)>` into separate row-major cosine and sine vectors.

## Performance

- Baseline RCH Criterion: `[716.01 us, 727.76 us, 740.68 us]`
- After RCH Criterion: `[526.77 us, 533.82 us, 541.80 us]`
- Conservative median speedup: `1.36x`
- Repeat after-run: `[260.18 us, 264.21 us, 268.68 us]`

## Behavior Proof

- Golden before SHA: `85048a3c06ab045815cbeb238fee9e1e07a05c27ceed3c3782ec0fd5ea97c6b1`
- Golden after SHA: `85048a3c06ab045815cbeb238fee9e1e07a05c27ceed3c3782ec0fd5ea97c6b1`
- Byte compare: identical

Isomorphism obligations preserved:

- Segment traversal order unchanged.
- Frequency traversal order unchanged.
- Sample traversal order unchanged.
- Output PSD bin order unchanged.
- Twiddle angle generation order unchanged.
- Floating-point operation sequence remains `re += s * cos` then `im -= s * sin` per sample.
- RNG and tie-breaking surfaces are absent.
- The 128-point `OnceLock` plan cache semantics are unchanged; only twiddle field layout changed.

## Validation

- `rch exec -- cargo test -p fsci-stats psd_welch -- --nocapture`
- `rch exec -- cargo check -p fsci-stats --all-targets`
- `rch exec -- cargo clippy -p fsci-stats --all-targets -- -D warnings`
- `cargo fmt -p fsci-stats --check`
- `ubs crates/fsci-stats/src/lib.rs`

## Post-Change Reprofile

- Command: `rch exec -- cargo bench -p fsci-stats --bench stats_bench --locked --`
- PSD row after broad reprofile: `[255.46 us, 257.41 us, 259.60 us]`
- Next visible stats hotspots:
  - `time_series/psd_welch/4096_w128_o64`: `257.41 us` median
  - `qmc_discrepancy/mixture/512x2`: `243.16 us` median
  - `qmc_discrepancy/l2_star/512x2`: `203.31 us` median

## Score

`3.0 = impact 2 * confidence 3 / effort 2`

Verdict: keep. The lever clears Score>=2.0 with unchanged golden output and a conservative focused RCH win.
