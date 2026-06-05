# Sparse SpMM Lazy Sorted-Certificate Setup

Bead: `frankenscipy-8l8r1.39`

## Target

The `.38` reprofile kept `sparse_spmm/2000x2000_d1/2000` first. A focused RCH baseline captured `12.699 ms` median `[12.531,12.930]` on `ts2`, and the before strict golden SHA was:

`0728e7d2e4072bf721c19f6b8d0a85a1e064bad0a69d4f10382efbc8ab4c5af2`

## Verdict

Superseded/no-ship. The shared tree already had the `frankenscipy-8l8r1.37` structural-symbolic source trial in `crates/fsci-sparse/src/linalg.rs`, so benchmarking a lazy sorted-certificate edit would have mixed two source levers. The lazy-certificate hunk was removed, no source change was kept for this bead, and `frankenscipy-8l8r1.37` was finished instead.
