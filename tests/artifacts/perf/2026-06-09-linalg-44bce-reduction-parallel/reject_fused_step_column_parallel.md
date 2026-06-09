## frankenscipy-44bce: fused-step column parallelism rejected

### Target

- Profile-backed residual: deterministic thin SVD square `512x512` reduction stage remained the largest stage after V replay.
- Candidate: partition independent trailing-column writes in `apply_bidiag_fused_step` across scoped worker threads.
- Primitive class: morsel-style column parallelism over a fused two-sided Householder update.

### Isomorphism

- Ordering preserved: yes for the operations this candidate parallelizes. Each column dot product still accumulates `left_values` in the same row order, and the row-dot accumulation over columns remains serial.
- Tie-breaking unchanged: yes. Reflector construction, SVD backend, singular ordering, rank policy, and sign policy are untouched.
- Floating-point: bit-identical in the exercised proof shapes.
- RNG seeds: N/A.
- Golden/output proof: `bidiag_fused_step_matches_workspace_reference_bits` passed after adding a `320x256` case that crosses the parallel threshold.

### Same-worker evidence

- Baseline artifact: `baseline_stage_ovh_a_rch.txt`
- After artifact: `after_stage_breakdown_rch.txt`
- Worker: `ovh-a`
- Baseline `worker_count`: `16`
- After `worker_count`: `16`
- Baseline `reduction_ms`: `82.636`
- After `reduction_ms`: `106.245`
- Speed ratio: `0.777800x`
- Baseline `bidiagonal_svd_ms`: `47.182`
- After `bidiagonal_svd_ms`: `45.680`
- Baseline `back_transform_u_ms`: `12.803`
- After `back_transform_u_ms`: `11.692`
- Baseline `back_transform_v_ms`: `11.712`
- After `back_transform_v_ms`: `15.059`

### Decision

Reject. The proof is clean, but the measured reduction stage regresses on the profile target. Thread setup/chunking overhead is larger than the independent-column write savings at `512x512`.

Next route: avoid more fused-step thread fanout. Attack the reduction with a structurally different primitive: blocked/compact two-sided trailing update, cache-oblivious panel layout, or a different bidiagonalization backend whose proof can be expressed through the existing reduction digest and public golden SHA.
