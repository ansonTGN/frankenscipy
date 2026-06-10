# frankenscipy-8l8r1.77 keep: direct column-major DMatrix import

## Target

- Bead: `frankenscipy-8l8r1.77`
- Crate: `fsci-linalg`
- Hot path: public dense `eigh` row input import into nalgebra `DMatrix`
- Worker: RCH `vmi1227854`
- Lever: build `DMatrix` storage directly in nalgebra column-major order instead of creating a row-major temporary and calling `DMatrix::from_row_slice`.

## Baseline

Command:

```bash
RCH_REQUIRE_REMOTE=1 rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench -- eigh_dense --noplot --sample-size 10 --warm-up-time 1 --measurement-time 2
```

Artifact: `baseline_eigh_dense_criterion_rch.txt`

Criterion means:

- `eigh_dense/256x256`: 12.965 ms
- `eigh_dense/512x512`: 95.523 ms

Stage probe artifact: `public_eigh_stage_timing_probe_rch.txt`

- `256x256`: import 0.616354 ms, symmetric_eigen 11.892012 ms, sort/export 0.212168 ms, total 12.720534 ms
- `512x512`: import 2.562899 ms, symmetric_eigen 86.563297 ms, sort/export 0.873979 ms, total 90.000175 ms
- Stage digests:
  - `256x256` values `0x28ef4e22353a7958`, vectors `0xe08e4a3aa69b2a51`
  - `512x512` values `0x72cfa53bb61e39ef`, vectors `0xc3ab30bc96010f42`

Golden artifact: `baseline_eigh_public_golden_rch.txt`

- `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`

## After

Artifact: `after_colmajor_import_eigh_dense_criterion_rch.txt`

Criterion means:

- `eigh_dense/256x256`: 12.267 ms, 5.38% faster than baseline mean
- `eigh_dense/512x512`: 94.994 ms, 0.55% faster than baseline mean

Stage probe artifact: `after_colmajor_import_stage_timing_probe_rch.txt`

- `256x256`: import 0.202394 ms, symmetric_eigen 11.319616 ms, sort/export 0.233701 ms, total 11.755711 ms
- `512x512`: import 0.660660 ms, symmetric_eigen 99.188153 ms, sort/export 0.958366 ms, total 100.807179 ms
- Stage digests unchanged:
  - `256x256` values `0x28ef4e22353a7958`, vectors `0xe08e4a3aa69b2a51`
  - `512x512` values `0x72cfa53bb61e39ef`, vectors `0xc3ab30bc96010f42`

Golden artifact: `after_colmajor_import_eigh_public_golden_rch.txt`

- `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`

## Isomorphism proof

- Ordering preserved: public `eigh` still sorts source indices with stable `sort_by` and `f64::total_cmp`; the sort/export logic is unchanged.
- Tie-breaking unchanged: equal eigenvalues keep source order through the unchanged stable index sort.
- Floating point preserved: `dmatrix_from_rows` writes each `rows[row][col]` to the same logical matrix coordinate as `DMatrix::from_row_slice`; the eigensolver sees coordinate-identical input values. Stage digests and the public golden digest are unchanged.
- RNG unchanged: no RNG is used in this path.
- Safety and dependency boundary unchanged: no unsafe code, no external BLAS/LAPACK/MKL/XLA linkage.
- Validation and zero-size/ragged validation preserved: `matrix_shape(rows)?` is unchanged before storage construction.

## Validation

- `cargo fmt -p fsci-linalg --check`: pass
- `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1227854 rch exec -- cargo check -j 1 -p fsci-linalg --all-targets --locked`: pass
- `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1227854 rch exec -- cargo clippy -j 1 -p fsci-linalg --all-targets --no-deps --locked -- -D warnings`: pass after narrow fixture-only `needless_range_loop` annotations
- `RCH_REQUIRE_REMOTE=1 RCH_WORKER=vmi1227854 rch exec -- cargo test -j 1 -p fsci-linalg --lib --locked -- --test-threads=1`: pass, 391 passed, 26 ignored
- Focused public golden proof: pass, digest unchanged
- Full dependency clippy note: `cargo clippy -p fsci-linalg --all-targets --locked -- -D warnings` is blocked by a pre-existing `fsci-fft` dependency lint (`manual_is_multiple_of`) outside this bead; the linalg no-deps clippy gate is clean.

## Score

- Impact: 2
- Confidence: 3
- Effort: 1
- Score: `2 * 3 / 1 = 6.0`
- Verdict: keep

## Reprofile / next route

The import path is no longer the main dense-eigh residual. After this keep, `symmetric_eigen` remains dominant in the stage probe. The next profile-backed primitive is `frankenscipy-8l8r1.78`: true compact-WY blocked symmetric reduction with a direct band-to-tridiagonal bulge-chasing backend and stage timings before any public route.
