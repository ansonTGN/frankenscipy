# fsci-linalg lstsq/pinv Gram-SVD full-rank gate trial

## Decision

Reject. The trial computed an SVD surrogate from `A^T A` for tall matrices, then
used it only when the matrix was full rank and sufficiently well conditioned.
The measured `perf_lstsq_probe` matrices are structurally low-rank, so the trial
paid the Gram/eigendecomposition cost and still fell back to the full SVD path.

## Baseline

Artifact: `baseline_perf_lstsq_probe_rch.txt`

Worker: `vmi1227854`

Rows:

- `lstsq m=2000 n=1000`: `3388.7 ms`
- `pinv  m=2000 n=1000`: `3350.2 ms`
- `lstsq m=3000 n=1500`: `20309.9 ms`
- `pinv  m=3000 n=1500`: `20739.8 ms`
- `spd-solve n=1024`: `67.0 ms`
- `spd-solve n=2048`: `271.8 ms`

Baseline SHA-256:

```text
3efd966dab504ccafc9783f5523f5a83d5d771850f7ff357925e75be699d0693
```

## After Trial

Command:

```text
RCH_FORCE_REMOTE=1 rch exec -- cargo run --release -p fsci-linalg --bin perf_lstsq_probe --locked
```

Same campaign run output showed immediate rejection rows:

- `lstsq m=2000 n=1000`: `5343.9 ms`
- `pinv  m=2000 n=1000`: `5163.8 ms`
- `lstsq m=3000 n=1500`: `20454.1 ms`
- `pinv  m=3000 n=1500`: `20696.4 ms`
- `spd-solve n=1024`: `66.7 ms`
- `spd-solve n=2048`: `436.2 ms`

A concurrent raw capture in `after_gram_svd_perf_lstsq_probe_rch.txt` also
showed regression/flat rows on another worker:

- `lstsq m=2000 n=1000`: `5948.7 ms`
- `pinv  m=2000 n=1000`: `6177.9 ms`
- `lstsq m=3000 n=1500`: `24657.3 ms`
- `pinv  m=3000 n=1500`: `24948.1 ms`

## Isomorphism

Production source was restored to the pre-trial SVD path, so ordering,
tie-breaking, floating-point operation order, RNG behavior, rank reporting,
singular-value ordering, error classes, and certificate semantics are unchanged.
No golden-output hash is accepted for the rejected source because the benchmark
failed the keep gate before behavior proof could justify shipping it.

## Next Primitive

Continue `frankenscipy-jvcdf` with a real SVD-class primitive:
blocked Householder bidiagonalization with compact reflectors and GEMM-backed
trailing updates, followed by bidiagonal QR or divide-and-conquer singular-vector
reconstruction. Target remains `>=8x` on the `m=3000 n=1500` `lstsq` and `pinv`
rows while preserving rank, singular-value ordering, tolerances, errors, and
certificate semantics.
