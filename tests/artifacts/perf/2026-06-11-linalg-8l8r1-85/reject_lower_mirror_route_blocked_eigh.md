# frankenscipy-8l8r1.85 lower-mirror `eigh` packing rejection

## Target

- Bead: `frankenscipy-8l8r1.85`
- Crate: `fsci-linalg`
- Profile-backed target: `eigh_dense`
- Worker used for comparable baseline and candidate: `vmi1227854`

The fresh baseline was captured through RCH from a clean detached worktree after the shared checkout's untracked `src/bin` probes made dirty-worktree benchmark routing unusable.

## Baseline

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench --locked -- eigh_dense
```

Artifact: `baseline_eigh_dense_clean_worktree_rch.txt`

```text
eigh_dense/256x256 time: [11.435 ms 11.891 ms 12.702 ms]
eigh_dense/512x512 time: [91.291 ms 93.570 ms 96.444 ms]
```

Artifact sha256:

```text
b171127e86ef3e281f78df5f216c43294629a7c9315826ef110754d9e9daec6c
```

## Candidate Lever

One source lever was tested in a clean candidate worktree:

- Add an `eigh`-local large-matrix builder that mirrors the lower triangle into nalgebra's column-major `DMatrix`.
- Gate at `rows >= 128`.
- Leave validation, trace emission, eigenvalue ordering via `total_cmp`, RNG absence, and public output sorting unchanged.

This was selected as a bounded affine-layout / data-movement probe after prior direct scalar tridiagonalization and output materialization attempts had already been rejected.

## Isomorphism Proof

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo test -j 1 -p fsci-linalg --lib --locked -- eigh_ --nocapture
```

Artifact: `proof_eigh_lower_mirror_tests_rch.txt`

Result:

- 13 focused `eigh_` tests passed.
- Existing public digest preserved: `eigh_index_sort_public_golden_digest=0x287a5d3679a8bc6a`.
- Candidate large symmetric bit-proof digest: `eigh_lower_mirror_public_golden_digest=0x95e78bd6cd8be67e`.

Proof policy:

- Ordering/tie-breaking: unchanged `total_cmp` sort over nalgebra eigenvalue indices.
- Floating point: for symmetric in-contract inputs, the mirrored lower builder produces the same dense matrix bits as the old full packer; the proof compared eigenvalues and eigenvectors bit-for-bit against the old materialized-pair reference at `128x128` and `256x256`.
- RNG: none.
- Shape/error behavior: unchanged before matrix construction; finite validation still runs on the full public input.

Proof artifact sha256:

```text
fed1a2ad78b4f3dd267731c0f81c747e225cda6648239121947f87e2e4d28681
```

## Same-Worker Rebench

Command:

```bash
RCH_REQUIRE_REMOTE=1 RCH_TEST_SLOTS=1 CARGO_BUILD_JOBS=1 \
  rch exec -- cargo bench -j 1 -p fsci-linalg --bench linalg_bench --locked -- eigh_dense
```

Artifact: `after_lower_mirror_eigh_dense_rch.txt`

```text
eigh_dense/256x256 time: [11.212 ms 11.779 ms 12.333 ms]
eigh_dense/512x512 time: [89.710 ms 92.780 ms 95.502 ms]
```

Artifact sha256:

```text
fefed290c7f70c7f14ed2cf68c89e559db89465f1a2d4a21eba9ab406130c6ec
```

## Score And Verdict

Mean ratios:

- `256x256`: `11.891 / 11.779 = 1.0095x`
- `512x512`: `93.570 / 92.780 = 1.0085x`

Criterion intervals overlap on both sizes and the mean movement is below the 2% fallback trigger. The lever is proof-clean but not a real win.

Score:

```text
(Impact 0.5 * Confidence 4) / Effort 1.5 = 1.333 < 2.0
```

Verdict: rejected, source restored to zero diff, no production code kept.

## Next Primitive

The next `eigh_dense` route should skip packing micro-levers and attack the measured eigensolver core with a fundamentally different dense-linalg primitive:

- First implementation slice: blocked full-to-band symmetric reduction with accumulated orthogonal transforms, behind a private proof/perf probe before public dispatch.
- Follow-on slice: band-to-tridiagonal bulge chasing and/or blocked backtransform.
- Target ratio for a keepable first public dispatch: at least `1.25x` on `eigh_dense/512x512` same-worker RCH while preserving ascending eigenvalues, residual `||AV - VLambda||`, orthogonality `||V^T V - I||`, and sign/subspace golden policy.

Alien lineage: communication-avoiding dense linear algebra / blocked eigensolver reductions, not scalar tridiagonalization or output materialization.
