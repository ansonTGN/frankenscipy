# frankenscipy-8l8r1.71 eigvalsh values-only route

## Lever

Route public `eigvalsh` through nalgebra's values-only symmetric eigensolver instead of calling full `eigh` and discarding eigenvectors.

Preserved behavior surface:

- Dimension and finite-input validation match `eigh`.
- Empty input returns an empty vector.
- Eigenvalues are sorted with the same `f64::total_cmp` policy as public `eigh`.
- Trace operation remains `"eigh"` to preserve the previous observable trace emitted by the old `eigvalsh -> eigh` route.
- No RNG, unsafe code, or external BLAS/LAPACK/MKL/XLA linkage was introduced.

## Baseline

Artifact: `baseline_public_eigvalsh_values_only_clean_rch.txt`

- RCH worker: `ovh-a`
- `256x256`: `eigvalsh_ms=12.805711`, full `eigh_ms=11.408911`
- `512x512`: `eigvalsh_ms=88.086961`, full `eigh_ms=84.793075`
- Values digests: `0x28ef4e22353a7958` (`256x256`), `0x72cfa53bb61e39ef` (`512x512`)
- Artifact SHA256: `cfe70e2b87bd036730f330b4897daa0e427ba793299914c12a31f2167b9fc374`

## After

Artifact: `after_public_eigvalsh_values_only_clean_rch.txt`

- RCH worker: `ovh-a`
- `256x256`: `eigvalsh_ms=7.082600`, full `eigh_ms=20.683052`, same-run speedup vs full `eigh=2.920263x`
- `512x512`: `eigvalsh_ms=48.600340`, full `eigh_ms=146.702189`, same-run speedup vs full `eigh=3.018542x`
- Baseline-to-after same-worker speedups: `1.807303x` (`256x256`), `1.812489x` (`512x512`)
- Max eigenvalue drift vs full `eigh`: `0.0` for both probe shapes
- Values digests unchanged: `0x28ef4e22353a7958`, `0x72cfa53bb61e39ef`
- Artifact SHA256: `abd431b10a49e36c79f2a61c73a2c0838f4b8de9c61bc1ee522ef9afe2802b39`

## Score

`Impact 1.812489 * Confidence 5 / Effort 1 = 9.062445`; keep.

## Validation

- RCH focused perf/proof probe passed: `after_public_eigvalsh_values_only_clean_rch.txt`
- RCH focused tests passed: `proof_eigvalsh_tests_clean_rch.txt`
- RCH `cargo check -p fsci-linalg --lib --locked` passed: `check_fsci_linalg_clean_rch.txt`
- RCH `cargo clippy -p fsci-linalg --lib --locked --no-deps -- -D warnings` passed: `clippy_fsci_linalg_clean_no_deps_rch.txt`
- `cargo fmt -p fsci-linalg --check` passed: `fmt_fsci_linalg_clean_check.txt`
- `ubs crates/fsci-linalg/src/lib.rs` reported zero critical issues: `ubs_fsci_linalg_clean_lib.txt`
- RCH `fsci-conformance` `diff_linalg_eigvalsh` could not run the oracle because the RCH Python environment lacked `numpy`: `conformance_diff_linalg_eigvalsh_clean_rch.txt`
- Local `fsci-conformance` `diff_linalg_eigvalsh` passed with local NumPy/SciPy: `conformance_diff_linalg_eigvalsh_clean_local.txt`
