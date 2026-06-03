# fsci-linalg matmul store-unroll trial - ABANDONED

Bead: `frankenscipy-8l8r1.12`
Target: `fsci_linalg::matmul`, full 4x4 register-tile path.

## Lever

Trial lever: replace only the full-tile `for di in 0..MR` writeback loop with
four explicit row writeback blocks. The `k` loop, accumulator shape, A loads, B
row loads, scalar ragged path, API, and error surfaces were unchanged.

## Baseline and after

Fresh RCH Criterion baseline on `vmi1156319`:

| row | baseline median |
| --- | ---: |
| `matmul/256x256` | 10.928 ms |
| `matmul/512x512` | 107.00 ms |
| `matmul/768x768` | 814.91 ms |
| `matmul/1024x1024` | 1.6474 s |

RCH Criterion after the store-unroll candidate:

| row | after median | result |
| --- | ---: | ---: |
| `matmul/256x256` | 10.224 ms | 1.07x |
| `matmul/512x512` | 89.825 ms | 1.19x |
| `matmul/768x768` | 931.45 ms | 0.87x |
| `matmul/1024x1024` | 1.7149 s | 0.96x |

Result: rejected. The two larger rows regressed, including the profile-driving
1024x1024 row.

Exact paired RCH Criterion on `vmi1149989` compared the prior store loop against
the store-unroll candidate in one run:

| row | prior median | candidate median | result |
| --- | ---: | ---: | ---: |
| `matmul_store_unroll_pair/256x256` | 2.8550 ms | 3.9108 ms | 0.73x |
| `matmul_store_unroll_pair/512x512` | 20.428 ms | 44.556 ms | 0.46x |
| `matmul_store_unroll_pair/768x768` | 72.244 ms | 103.28 ms | 0.70x |
| `matmul_store_unroll_pair/1024x1024` | 178.97 ms | 358.43 ms | 0.50x |

The paired run confirms the lever was negative rather than worker noise.

## Isomorphism proof

- Ordering preserved: yes. The same completed accumulator value was written to
  the same `c[i][j]` coordinate after the unchanged `k=0..ka` accumulation.
- Tie-breaking unchanged: not applicable.
- Floating-point behavior: unchanged. The candidate did not alter any multiply,
  add, or per-cell accumulation order.
- RNG: not applicable.
- Golden output: RCH `matmul_microkernel` tests passed before, during the
  candidate, and after restore. Sorted normalized golden sha256 stayed
  `ee5c848e69cc7ef4c22d0312f61633ade9fc88aca66ae2343fd6a0b6403c4b4b`.

## Closeout

Score: `0.0`; performance impact was negative in the exact paired run and on
the largest rows in the focused before/after run, below the required `>=2.0`
keep gate.

The production source was restored. `source_restored_diff.txt` is zero bytes.
Post-restore validation passed:

- `cargo fmt -p fsci-linalg --check`
- RCH `cargo test -p fsci-linalg --release matmul_microkernel --locked -- --nocapture`
