# Conclusion: Rejected

The profile-backed target was `baseline_pinv/1000x500`, ranked first by the post-direct-lstsq RCH reprofile.

The one-lever trial built row-major `V * Sigma^-1` and `U^T` panels from the same full SVD and used the project safe-Rust GEMM kernel to produce the returned pseudoinverse layout. Golden behavior was unchanged: the sorted stable RCH release `pinv` test hash stayed `cc7b25e24e092e68b31a93abe71436bf881cd54a431087779a8603fb68c42e9d`.

The focused RCH Criterion benchmark failed the keep gate:

| benchmark | baseline median | after median |
| --- | ---: | ---: |
| `baseline_pinv/1000x500` | `437.29 ms` | `588.02 ms` |

Decision: reject and restore source. The source tree has no remaining diff for `crates/fsci-linalg/src/lib.rs`; only the rejection evidence and post-direct-lstsq reprofile are retained.
