# Rejected: band tall lstsq residual-cost pass

Bead: `frankenscipy-8l8r1.63`

Target: `PUBLIC_BAND_TALL_LSTSQ_ROUTE_PERF` at shape `320x256` after the full
public ignored-test reprofile reported a noisy residual route cost:

- `public_routes_reprofile_rch.txt`: `reference_lstsq_ms=406.503590`,
  `routed_lstsq_ms=339.176946`, speedup `1.198500`, `rank=256`,
  `lstsq_max_abs_diff=1.77635683940025046e-13`.

Fresh single-target RCH evidence did not confirm a keepable residual-cost lever:

- `baseline_band_tall_lstsq_single_rch.txt` on `ovh-a`: `reference_lstsq_ms=79.033940`,
  `routed_lstsq_ms=13.079870`, speedup `6.042410`, `rank=256`,
  `lstsq_max_abs_diff=1.92290627865077113e-13`.
- `after_eigenvalues_only_band_tall_lstsq_rch.txt` on `ovh-a`: `reference_lstsq_ms=78.085900`,
  `routed_lstsq_ms=13.178585`, speedup `5.925211`, `rank=256`,
  `lstsq_max_abs_diff=1.92290627865077113e-13`.
- `current_band_tall_lstsq_confirm_rch.txt` on `ovh-b`: `reference_lstsq_ms=112.180260`,
  `routed_lstsq_ms=48.475980`, speedup `2.314141`, `rank=256`,
  `lstsq_max_abs_diff=1.77635683940025046e-13`.

Decision: no source change kept. The attempted eigenvalues-only family did not
improve the same-worker single-target run (`13.079870 ms -> 13.178585 ms`), so
Impact is negative and the keep score is `0.0`, below the `>=2.0` gate.

Behavior proof: source is restored to a clean `crates/fsci-linalg/src/lib.rs`
diff. The observed public-rank and max-diff outputs stayed unchanged across
the single-target measurements. Ordering/tie/floating-point/RNG behavior is
unchanged because no code was retained.

Next route: re-profile and attack the next profile-backed `fsci-linalg` hotspot
instead of repeating tall-band eigenvalue extraction, Gram thin-SVD, TSQR,
one-sided Jacobi, packed/two-stage bidiag, or scalar cleanup families.
