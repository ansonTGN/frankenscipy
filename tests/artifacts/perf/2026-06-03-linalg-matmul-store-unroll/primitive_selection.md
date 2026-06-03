# frankenscipy-8l8r1.12 Primitive Selection

## Profile Target

- Target: `fsci_linalg::matmul` 4x4 full-tile micro-kernel writeback.
- Current RCH Criterion baseline on `vmi1156319`: 256 median `10.928 ms`, 512 `107.00 ms`, 768 `814.91 ms`, 1024 `1.6474 s`.
- Profile basis: the committed linalg profile and the two rejected follow-up trials keep matmul as the dominant linalg hotspot under this campaign.

## Rejected Nearby Levers

- Row/cache blocking: already too broad for a single-lever store-side trial.
- NC panel / packed B / 4x8 variants: previous matmul packing attempts did not meet the keep threshold.
- B-flat read path: exact paired Criterion regressed three of four sizes.
- A row-reference hoist: exact paired Criterion regressed all four sizes.
- Loop-order or ragged-path changes: higher behavior risk and not needed to test this writeback hypothesis.

## Selected Lever

Explicitly unroll only the full 4x4 tile store:

- Replace `for di in 0..MR` writeback with four explicit row blocks.
- Keep the accumulator layout `acc[MR][NR]`.
- Keep the monotonic `k` loop and separate multiply/add sequence unchanged.
- Keep direct A and B loads unchanged.
- Keep scalar ragged tiles unchanged.

## Isomorphism Contract

- Validation order and error behavior: unchanged; this edit is after validated dimensions.
- Output coordinates: each `acc[row][col]` is written to the same `c[i0+row][j0+col]`.
- Floating point: accumulation order and operations are unchanged; only store syntax changes.
- Ordering/tie-breaking: no ordering or tie decisions exist in this kernel.
- RNG: no RNG is used.
- Global state: no global state is read or written.
- Golden proof: sorted normalized sha256 must remain `ee5c848e69cc7ef4c22d0312f61633ade9fc88aca66ae2343fd6a0b6403c4b4b`; raw output-order hashes are recorded separately because test output order can vary.

## Score Gate

- Target score: `2.0 = impact 1 * confidence 4 / effort 2`.
- Keep only if RCH re-benchmark shows a real focused win and golden proof remains identical.
- Restore source and close as rejected if the exact paired or direct gate fails Score>=2.0.
