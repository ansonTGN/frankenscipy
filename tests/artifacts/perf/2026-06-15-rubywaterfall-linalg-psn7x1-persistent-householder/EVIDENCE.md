# frankenscipy-psn7x.1 Evidence

## Target

- Bead: `frankenscipy-psn7x.1`
- Kernel: native symmetric-eigh Householder trailing rank-2 update
- Worker: RCH `ovh-a`
- Lever: replace iterator-adapter hot loops with explicit index loops and fuse the `p *= tau` pass with `v dot p`, preserving the column-major accumulation order and the scalar lower-triangle update formula.

## Baseline

- `baseline_public_eigh_route_rch.txt`
  - 400x400 public `eigh`: `44.115212 ms`, digest `0x4b8334c92ce624eb`
  - 800x800 public `eigh`: `231.176244 ms`, digest `0xad8a7e5fa1980bfb`
  - 1200x1200 public `eigh`: `694.001290 ms`, digest `0x181b3486089d0e4a`
- `baseline_native_vs_nalgebra_rch.txt`
  - 400x400 native: `35.1 ms`
  - 800x800 native: `231.3 ms`
  - 1200x1200 native: `722.4 ms`

## Proof

- `after_rank2_column_update_bits_rch.txt`: `symmetric_rank2_column_update_matches_rowwise_bits` passed, proving the touched kernel remains bit-identical to the rowwise reference fixture.
- `after_eigh_behavior_rch.txt`: 16 `eigh` behavior tests passed, including native correctness and `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`.
- Public route digests stayed fixed for all measured shapes after the lever:
  - 400x400: `0x4b8334c92ce624eb`
  - 800x800: `0xad8a7e5fa1980bfb`
  - 1200x1200: `0x181b3486089d0e4a`

## Rebench

- `after_public_eigh_route_rch.txt`
  - 400x400 public `eigh`: `46.615584 ms` (below the native threshold; small nalgebra-route noise)
  - 800x800 public `eigh`: `197.849506 ms` (`1.168x` faster than baseline)
  - 1200x1200 public `eigh`: `596.825623 ms` (`1.163x` faster than baseline)
- `after_native_vs_nalgebra_rch.txt`
  - 400x400 native: `33.2 ms` (`1.057x` faster than baseline)
  - 800x800 native: `192.5 ms` (`1.202x` faster than baseline)
  - 1200x1200 native: `592.5 ms` (`1.219x` faster than baseline)

## Gates

- `cargo fmt -p fsci-linalg -- --check`: passed.
- `check_fsci_linalg_lib_rch.txt`: `cargo check -j 1 -p fsci-linalg --lib --locked` passed.
- `clippy_fsci_linalg_lib_rch.txt`: first clippy run failed on the deliberate index-loop spelling.
- `clippy_fsci_linalg_lib_rch_retry.txt`: `cargo clippy -j 1 -p fsci-linalg --lib --no-deps --locked -- -D warnings` passed after the allow was scoped to the hot kernel.
- `final_integrated_check_fsci_linalg_lib_rch.txt`: after cherry-picking onto current `origin/main`, `cargo check -j 1 -p fsci-linalg --lib --locked` passed.
- `ubs crates/fsci-linalg/src/lib.rs`: completed with `Critical issues: 0`; warnings were broad pre-existing file-wide inventory.

## Score

- Impact: `2.0` (large native `eigh` route improved by ~16-22% on same-worker probes)
- Confidence: `4.0` (bitwise kernel proof, public golden digest, public route digests, and same-worker timing agree)
- Effort: `1.0`
- Score: `8.0`
- Verdict: KEEP

## Rejected Lower-Storage Side Probe

Agent: `RubyWaterfall`
Date: 2026-06-15
Crate: `fsci-linalg`
Worker: `ovh-a`

## Target

The profile-backed target was the native symmetric `eigh` Householder reduction
stage. The current n=1200 split still leaves reduction as the dominant stage:
`reduction 406.741 ms`, `tridiagonal_eigen 75.736 ms`, `backtransform
103.695 ms`, `sort 5.408 ms`.

## Baseline

Criterion baseline (`baseline_eigh_dense_criterion_rch.txt`):

| row | mean range |
| --- | ---: |
| `eigh_dense/256x256` | `12.084 ms` - `12.158 ms` |
| `eigh_dense/512x512` | `99.007 ms` - `99.926 ms` |

Public native route baseline (`baseline_public_native_route_ovh_a_rch.txt`):

| n | routed native | nalgebra | speedup |
| ---: | ---: | ---: | ---: |
| 400 | `45.311907 ms` | `41.983716 ms` | `0.926549x` |
| 800 | `234.585242 ms` | `314.798586 ms` | `1.341937x` |
| 1200 | `727.670071 ms` | `1033.365120 ms` | `1.420101x` |

Native direct baseline (`baseline_native_vs_nalgebra_ovh_a_rch.txt`):

| n | native | nalgebra | ratio |
| ---: | ---: | ---: | ---: |
| 400 | `37.6 ms` | `41.6 ms` | `1.11x` |
| 800 | `228.0 ms` | `318.8 ms` | `1.40x` |
| 1200 | `728.4 ms` | `1040.1 ms` | `1.43x` |

## Candidate

Single source lever: keep the active trailing symmetric matrix in lower-triangle
storage during native Householder reduction. The `p = tau * A * v` matvec read
the lower storage symmetrically in the same per-row column accumulation order,
and the rank-2 update wrote only the lower triangle. This removed the strided
mirror write performed by the full-storage rank-2 update.

No production source was retained after the failed performance gate.

## Proof

- `proof_lower_storage_bits_rch.txt`: lower-storage native solver matched the
  full-storage native reference bit-for-bit on deterministic symmetric fixtures.
- `proof_lower_storage_native_rch.txt`: existing native-vs-nalgebra correctness
  proof passed.
- `proof_public_eigh_golden_rch.txt`: public `eigh` materialized-pair golden
  guard passed with digest `0x287a5d3679a8bc6a`.
- Ordering/tie behavior: unchanged; the public sort/golden guard stayed bitwise.
- Floating point: the proof-only candidate preserved staged native FP bits
  against the full-storage reference, but was not shipped.
- RNG: deterministic fixtures only; no randomness added.

## Rebench

Candidate public route (`after_lower_storage_public_native_route_ovh_a_rch.txt`):

| n | baseline routed | candidate routed | ratio |
| ---: | ---: | ---: | ---: |
| 400 | `45.311907 ms` | `45.316479 ms` | `0.999899x` |
| 800 | `234.585242 ms` | `309.275687 ms` | `0.758504x` |
| 1200 | `727.670071 ms` | `1024.273802 ms` | `0.710418x` |

Score: `Impact 0.0 * Confidence 4.0 / Effort 2.0 = 0.0`.

Verdict: REJECT. The mirror-write removal preserved behavior but slowed the
large public route, likely because symmetric lower-storage reads added branch
and transposed-load pressure to the `p` matvec.

## Next Primitive

Do not retry lower-storage-only mirror suppression, per-step scoped `p`
parallelism, worker-count retuning, or scalar/slice spelling variants.

The next admissible Householder route should change the memory model more
deeply: blocked active-suffix tiles or packed lower panels with exact scalar
`p/w` proof, so the reduction works on contiguous panel data instead of paying
branchy symmetric reads against the live `DMatrix`.
