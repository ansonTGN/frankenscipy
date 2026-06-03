# packed matmul 4x8 B-panel lever - ABANDONED

Bead: `frankenscipy-jhtc6`
Crate: `fsci-linalg`
Function: `matmul`
Candidate: pack 8-column B panels and widen the register kernel from 4x4 to 4x8.
Verdict: ABANDONED. The production source was restored to the shipped no-pack 4x4
micro-kernel.

## Baseline

RCH Criterion baseline on `vmi1293453` for the shipped no-pack 4x4 kernel:

- `matmul/256x256`: median `4.1627 ms`
- `matmul/512x512`: median `36.453 ms`
- `matmul/768x768`: median `129.72 ms`
- `matmul/1024x1024`: median `475.05 ms`

Golden proof before edit: RCH `cargo test -p fsci-linalg --release matmul_microkernel`
passed `matmul_microkernel_is_bit_identical_to_flat_ikj` and
`matmul_microkernel_golden_digest`.

## Candidate result

The ungated packed 4x8 candidate preserved output bits but had mixed perf:

- Cross-worker Criterion showed a 1024 win (`475.05 ms` baseline to `353.59 ms`)
  but a 256 regression (`4.1627 ms` to `5.2579 ms`).
- Same-run witness on `vmi1149989` showed:
  - `768x768`: no-pack `0.1156s`, packed `0.1883s` (`0.61x`, regression)
  - `1024x1024`: no-pack `0.8103s`, packed `0.4733s` (`1.71x`, win)

A size-gated variant tried to keep the old no-pack path below 1024 while using
packed 4x8 at 1024+, but the final same-run witness still failed to protect the
768 row on `vmi1156319`:

- `768x768`: no-pack `0.7153s`, gated current `0.8871s` (`0.81x`, regression)
- `1024x1024`: no-pack `0.9789s`, gated current `0.6989s` (`1.40x`, win)

The lever therefore does not clear the "real win without smaller-shape regression"
bar and was not kept.

## Isomorphism

All candidate runs preserved observable behavior:

- API and error behavior unchanged.
- Ordering/tie/RNG surfaces: none.
- Floating point: each output cell kept monotonic `k=0..ka` accumulation order.
- Golden proof after candidate and after gated candidate both passed the existing
  digest and bit-identical tests.

## Artifact SHA-256

The RCH proof and benchmark transcripts are frozen by SHA-256:

- `baseline_criterion_matmul_rch.txt`: `620e333670fc7b1c2f6912cbc039ecc9328f47490f1356441a7fcb723e1cc751`
- `baseline_golden_matmul_rch.txt`: `fcf22ab7e1d00e725de290579cd2ccc94b18392b1c32745a89c16c2973fe8767`
- `after_criterion_matmul_rch.txt`: `3a8b94d14eb7ffe25f788b0d2a752d4a8bc4685ff9d833916973ae576ada5fa4`
- `after_golden_matmul_rch.txt`: `f468e80a009d096807d0d39587f2548c290727b19edd25af2fbcae9a99d6cab8`
- `same_run_packed_vs_no_pack_rch.txt`: `8513789003676b17a463ede03b613dde2924922908e9f7e7f249f535e474da7f`
- `after_gated_criterion_matmul_rch.txt`: `908335cdb38c269d37f999ee926def930a56b634d61c94743b0196cac9373f33`
- `after_golden_gated_matmul_rch.txt`: `28701a423f095256ae37579a27ea0af403cbc62c28e3daf9a45e444cb915e9d8`
- `same_run_gated_packed_vs_no_pack_rch.txt`: `f90982f10b492fc112ea2a42242ae254ae0d679fd506045f83c348a7da9b6151`

## Follow-up

Do not retry simple B-panel packing plus a 4x8 kernel in the current `Vec<Vec<f64>>`
layout. The next GEMM lever needs either:

- a real contiguous matrix representation to remove row-`Vec` indirection before
  packing, or
- a shape-dispatched large-GEMM path with stronger same-worker Criterion proof and
  no regression on 768 and below.
