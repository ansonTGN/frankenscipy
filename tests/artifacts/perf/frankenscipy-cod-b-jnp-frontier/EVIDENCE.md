# frankenscipy-01lxz jnjnp_zeros frontier evidence

Date: 2026-06-19
Agent: cod-b / MistyBirch
Crate: `fsci-special`
Lever: replace the fixed `nt + 2` by `nt + 2` candidate rectangle in
`jnjnp_zeros` with an output-sensitive frontier that starts near `sqrt(nt)` and
expands until the retained cutoff is below both the serial-tail and omitted-order
frontiers.

## Commands

Baseline on `origin/main` before the patch:

```bash
env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo bench -p fsci-special --bench special_bench -- \
  acoco_gauntlet_jnjnp_zeros --noplot
```

Candidate after the patch, pinned to the same rch worker:

```bash
env RCH_WORKER=hz1 CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo bench -p fsci-special --bench special_bench -- \
  acoco_gauntlet_jnjnp_zeros --noplot
```

SciPy-inclusive local oracle because rch worker `hz1` could not import
`scipy.special`:

```bash
env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  cargo bench -p fsci-special --bench special_bench -- \
  acoco_gauntlet_jnjnp_zeros --noplot
```

Correctness/build gates:

```bash
env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo test -p fsci-special \
  jnjnp_adaptive_envelope_matches_oversized_reference --lib -- --nocapture

env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo test -p fsci-special \
  jnyn_and_jnjnp_zeros_match_scipy --lib -- --nocapture

env CARGO_TARGET_DIR=/data/projects/.rch-targets/frankenscipy-cod-b \
  rch exec -- cargo check -p fsci-special --all-targets
```

## Same-worker Rust A/B

Worker: `hz1`.

| Workload | Baseline current mean | Candidate current mean | Candidate/baseline | Verdict |
| --- | ---: | ---: | ---: | --- |
| `jnjnp_zeros(nt=64)` | 97.861 ms | 5.4922 ms | 0.0561x time, 17.82x faster | keep |
| `jnjnp_zeros(nt=128)` | 513.89 ms | 10.121 ms | 0.0197x time, 50.77x faster | keep |

The benchmark's legacy duplicate route stayed essentially unchanged on the same
worker: `nt=64` was 133.87 ms after the patch and `nt=128` was 693.52 ms after
the patch. This confirms the improvement is from the new current frontier, not
from worker drift.

## SciPy Head-to-head

Local SciPy oracle rows from the same Criterion group:

| Workload | Candidate Rust mean | SciPy mean | Rust/SciPy | Verdict |
| --- | ---: | ---: | ---: | --- |
| `jnjnp_zeros(nt=64)` | 4.3372 ms | 486.57 us | 8.91x slower | residual loss |
| `jnjnp_zeros(nt=128)` | 7.5415 ms | 792.81 us | 9.51x slower | residual loss |

SciPy win/loss/neutral: `0/2/0`.
Same-worker internal keep/loss/neutral: `2/0/0`.

## Correctness

- PASS: `jnjnp_adaptive_envelope_matches_oversized_reference` on rch worker
  `ovh-a`.
- PASS: `jnyn_and_jnjnp_zeros_match_scipy` on rch worker `hz1`.
- PASS: `cargo check -p fsci-special --all-targets` on rch worker `ovh-b`.
- BLOCKED: `cargo clippy -p fsci-special --all-targets -- -D warnings` stopped
  in dependency crates `fsci-integrate` (`too_many_arguments`) and
  `fsci-linalg` (`needless_range_loop`, `needless_borrow`) before reaching this
  patch.
- PARTIAL: broad `cargo fmt --check` and `rustfmt --check
  crates/fsci-special/src/bessel.rs` still report pre-existing rustfmt drift
  outside this patch; the file was not reformatted wholesale to avoid unrelated
  churn.

## Decision

KEEP. The fixed quadratic candidate rectangle was the measured gap; replacing
it with a monotone output frontier reduces current Rust time by 17.82x to
50.77x on same-worker evidence. This does not dominate SciPy yet, so the
residual gap is routed to a deeper root-finder/evaluator bead instead of
retrying the envelope family.

Retry condition: do not retry larger fixed rectangles, duplicate
`jnp_zeros` calls, or envelope-only tweaks for this workload. The next credible
route must attack per-root evaluation/bracketing constants or use a SciPy-style
global zero enumeration strategy.
