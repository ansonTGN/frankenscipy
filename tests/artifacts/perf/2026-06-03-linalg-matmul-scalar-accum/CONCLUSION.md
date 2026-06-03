# fsci-linalg matmul scalar-accumulator trial - ABANDONED

Bead: `frankenscipy-8l8r1.13`
Target: `fsci_linalg::matmul`, full 4x4 register-tile path.

## Lever

Trial lever: replace only the full-tile `acc[[f64; 4]; 4]` aggregate with
sixteen named scalar accumulators `c00..c33`. The `k` loop, A loads, B loads,
ragged scalar path, API, and error surfaces were unchanged.

## Baseline and after

Fresh RCH Criterion baseline on `vmi1227854`:

| row | baseline median |
| --- | ---: |
| `matmul/256x256` | 4.6168 ms |
| `matmul/512x512` | 35.109 ms |
| `matmul/768x768` | 119.92 ms |
| `matmul/1024x1024` | 608.85 ms |

RCH Criterion after an initial scalar/direct-store candidate on `vmi1264463`:

| row | after median | result |
| --- | ---: | ---: |
| `matmul/256x256` | 15.203 ms | 0.30x |
| `matmul/512x512` | 212.20 ms | 0.17x |
| `matmul/768x768` | 1.2097 s | 0.10x |
| `matmul/1024x1024` | 2.4790 s | 0.25x |

Exact paired RCH Criterion on `vmi1227854` compared the prior array accumulator
against the scalar-accumulator candidate in one run:

| row | prior median | candidate median | result |
| --- | ---: | ---: | ---: |
| `matmul_scalar_accum_pair/256x256` | 3.0599 ms | 4.5333 ms | 0.68x |
| `matmul_scalar_accum_pair/512x512` | 25.525 ms | 35.015 ms | 0.73x |
| `matmul_scalar_accum_pair/768x768` | 89.355 ms | 129.97 ms | 0.69x |
| `matmul_scalar_accum_pair/1024x1024` | 673.78 ms | 678.86 ms | 0.99x |

Result: rejected. The exact paired run showed material regressions for the
smaller three rows and no meaningful 1024x1024 win.

## Isomorphism proof

- Ordering preserved: yes. The same per-cell accumulation order was used during
  the candidate.
- Tie-breaking unchanged: not applicable.
- Floating-point behavior: unchanged. The candidate kept the same separate
  multiply then add sequence for every output cell.
- RNG: not applicable.
- Golden output: RCH `matmul_microkernel` tests passed before, during the
  candidate, and after restore. Stable normalized golden sha256 stayed
  `4e96161ff0bd1aaf1a7d46d299b3b0255984350bbee513da271bb90ea1436578`.

## Closeout

Score: `0.0`; performance impact was negative in the exact paired run and far
below the required `>=2.0` keep gate.

The production source was restored. `source_restored_diff.txt` is zero bytes.
Post-restore validation passed:

- `cargo fmt -p fsci-linalg --check`
- RCH `cargo test -p fsci-linalg --release matmul_microkernel --locked -- --nocapture`
