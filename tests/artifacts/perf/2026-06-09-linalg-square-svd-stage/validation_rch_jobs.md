# RCH validation jobs

Source: `rch status --jobs --json` on 2026-06-09 after the keep.

- `29879662679163289`: `cargo check -p fsci-linalg --all-targets --locked`, worker `vmi1227854`, exit `0`, remote, duration `107308 ms`.
- `29879662679163294`: `cargo clippy -p fsci-linalg --all-targets --locked --no-deps -- -D warnings`, worker `vmi1227854`, exit `0`, remote, duration `92104 ms`.

Additional focused proof jobs captured in this artifact directory:

- `proof_parallel_right_bits_rch.txt`: `thin_bidiag_parallel_right_replay_matches_serial_bits`, exit `0`.
- `public_golden_rch.txt`: `public_svd_lstsq_pinv_golden_payload`, exit `0`.
- `after_public_square_svd_route_rch.txt`: `public_square_svd_route_perf_probe`, exit `0`.
