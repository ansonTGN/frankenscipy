## frankenscipy-44bce: slice rotate backend micro-lever rejected

### Target

- Profile-backed residual: square public `svd()` still routes through deterministic thin SVD; post-keep stage split left bidiagonal SVD backend at ~47-52 ms and reduction at ~82 ms.
- Candidate: change `rotate_eigenvector_columns` from indexed `DMatrix[(row, col)]` reads/writes to two mutable column slices.
- Primitive class: cache/indexing overhead removal inside the symmetric tridiagonal QR eigenvector replay backend.

### Isomorphism

- Ordering preserved: yes. Rows are visited in ascending order exactly as before.
- Tie-breaking unchanged: yes. The tridiagonal QR deflation/eigenvalue ordering code is untouched.
- Floating-point: identical operations for each row: `c * left - s * right`, then `s * left + c * right`.
- RNG seeds: N/A.
- Golden/output proof: backend probe digest unchanged.

### Same-worker evidence

- Baseline artifact: `baseline_backend_vmi1227854_rch.txt`
- After artifact: `after_slice_rotate_backend_ovh_a_rch.txt` (RCH selected `vmi1227854` despite the requested worker)
- Worker: `vmi1227854`
- Baseline `symmetric_eigen_route_ms`: `52.432819`
- After `symmetric_eigen_route_ms`: `51.132521`
- Speedup: `1.025430x`
- `backend_sweeps`: `875 -> 875`
- `svd_digest`: `0x7c2787acb98e625f -> 0x7c2787acb98e625f`
- `jacobi_reference_digest`: `0xd485296937b9f15f -> 0xd485296937b9f15f`
- `reconstruction_error`: `2.32830643653869629e-10 -> 2.32830643653869629e-10`
- `u_column_orthogonality_error`: `3.42614825399323308e-13 -> 3.42614825399323308e-13`
- `vt_orthogonality_error`: `1.37667655053519411e-14 -> 1.37667655053519411e-14`

### Decision

Reject. The behavior proof is clean, but the speedup is a small backend-only indexing win and does not clear the campaign score gate for a retained source change.

Next route: attack the reduction residual directly with a column/row partitioned fused-step update that preserves each dot-product accumulation order, or move to a materially different communication-avoiding reduction primitive if that fails.
