# frankenscipy-mifdz rejected sub-levers

## Baseline

- Current fresh Criterion baseline artifact: `baseline_criterion_eigh_dense_current_rch.txt`
- RCH worker: `ovh-b`
- `eigh_dense/256x256`: `[23.665 ms, 24.037 ms, 24.543 ms]`
- `eigh_dense/512x512`: `[188.66 ms, 189.41 ms, 190.16 ms]`
- Artifact SHA256: `b8fd8a158543df3261ce1cf40642a4bb5489df6fba5933024cc288a13a0d11e6`
- Inherited bead baseline on `vmi1227854`: `256x256` median `13.677 ms`, `512x512` median `105.45 ms`.

## Rejected sub-lever 1: direct scalar full-to-tridiagonal candidate

- Source retained: no.
- Proof: `proof_tridiagonal_candidate_rch.txt`, RCH worker `vmi1227854`, focused reconstruction/eigenvalue proof passed.
- Proof artifact SHA256: `9d4928e017837a42bb4dd0277e47caab1a44fdd528dc4994b12c38e4f6865a8e`
- Same-process perf/proof: `after_tridiagonal_candidate_perf_probe_rch.txt`, RCH worker `ovh-a`.
- `256x256`: current public `eigh` `13.133373 ms`, candidate `47.397769 ms`, speedup `0.277088x`.
- `512x512`: current public `eigh` `89.025452 ms`, candidate `503.499158 ms`, speedup `0.176814x`.
- Behavior: eigenvalue drift within tolerance, reconstruction/orthogonality proof passed. Canonical digests differ because eigenvector bases are allowed to differ up to the reconstruction/sign policy, but the route was rejected on performance before public dispatch.
- Verdict: rejected. This is a direct scalar tridiagonalization, not the requested blocked two-stage full-to-band primitive, and it fails the keep gate decisively.

## Rejected sub-lever 2: sorted-index output materialization

- Source retained: no.
- Change tested: replace per-column `Vec` allocation before sorting with stable sorted indices and direct eigenvector column copy.
- Golden before: `baseline_public_eigh_golden_payload_rch.txt`, payload SHA256 `fe9a1c3e401593d2640905730aee3a5baaa8e452de78651bd52fa4bfc0dcae84`.
- Proof after: `proof_eigh_index_materialization_rch.txt`, RCH worker `ovh-a`, 12 focused `eigh_*` tests passed.
- Golden after: payload SHA256 `fe9a1c3e401593d2640905730aee3a5baaa8e452de78651bd52fa4bfc0dcae84`.
- After Criterion: `after_criterion_eigh_index_materialization_rch.txt`, RCH worker `ovh-a`.
- `256x256`: `[11.719 ms, 12.234 ms, 12.505 ms]`.
- `512x512`: `[103.60 ms, 118.24 ms, 127.00 ms]`.
- Verdict: rejected. Behavior was unchanged, but the timing evidence is mixed and not a defensible full-route win: the 512x512 after median is slower than the inherited `vmi1227854` baseline and slower than the earlier same-worker `ovh-a` current-route side-probe (`89.025452 ms`).

## Next route

Continue `frankenscipy-mifdz` with a genuinely different primitive: safe-Rust blocked two-stage symmetric eigensolver work. The next attempt should target a cache-blocked full-to-band reduction or a band-to-tridiagonal bulge-chasing stage with a same-process A/B proof, not direct scalar tridiagonalization or output materialization.
