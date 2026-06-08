# perf: parallelize opt::jacobian partials + per-component scratch reuse

Bead: frankenscipy-wvojq

## Lever
`jacobian()` (crates/fsci-opt/src/lib.rs) filled the rows×columns Jacobian by a serial
double loop, each entry an adaptive first-derivative (`adaptive_first_derivative`) that
perturbs one input coordinate and reads one output of `f(&shifted)`. Two coupled changes:

1. **Parallelize** the `(row, column)` grid — the partials are mutually independent
   (each perturbs only `x[column]`, reads only `f(...)[row]`, no shared mutable state) —
   in a single `std::thread::scope` batch, folding results back in row-major order.
2. **Per-component scratch reuse** — the original `component` closure did
   `let mut shifted = x.to_vec();` on *every* evaluation. Replace with one scratch vector
   per component (RefCell, thread-local): set the perturbed coordinate, evaluate, restore
   it. `f` sees a bit-identical input either way, but allocation drops from O(nfev) to
   O(rows·columns). Without this, concurrent per-eval allocation serializes on the global
   allocator and the parallel path *regresses* (measured 0.83–0.91× before the fix).

## Isomorphism / byte-identity argument
- Scratch buffer: before each `f` call the buffer equals `x` with only `x[column]`
  replaced by `value` (restored to `x[column]` after each call) — identical to the
  per-eval `x.to_vec()` + index assignment. Same inputs to `f` ⇒ same outputs.
- `df`/`error`: disjoint cells; write order irrelevant.
- `nfev`: exact integer sum. `nit`: max. `success`: AND. `status`:
  `merge_differentiate_status` precedence pick — all commutative/associative.
- No floating-point reduction reassociated; each partial is the same scalar, on another
  core. Error path folds with `?` in pair order = serial's first-failing pair.

⇒ **bit-identical** to the serial double loop.

## Proof (golden — serial baseline vs parallel NEW, identical)
Harness: `cargo run --profile release-perf -p fsci-opt --bin perf_jacobian`
Map: O(n^2)-per-eval vector field (deterministic).

```
n=4  df_xor_bits=897e4f875ffee6fb nfev=165  nit=6 success=true  status=Converged
n=12 df_xor_bits=6adc68fb85ea5b87 nfev=1437 nit=7 success=false status=ErrorIncreased
n=30 df_xor_bits=7c835fc303d742f8 nfev=8939 nit=7 success=false status=ErrorIncreased
```
Identical bits/counts in the stashed serial build and the parallel+scratch build.
sha256(golden serial baseline payload) =
a50f157f65e9ed454afd6651b85c7137fdaffe566f0504df6f4a34957ee320c7

## Timing (rch remote, release-perf) — original serial → parallel+scratch
| n   | serial    | new       | speedup |
|-----|-----------|-----------|---------|
| 40  | 774.5 ms  | 23.06 ms  | 33.6x   |
| 70  | 6.605 s   | 162.8 ms  | 40.6x   |
| 100 | 27.97 s   | 576.7 ms  | 48.5x   |

(For reference, the parallel-only version without scratch reuse measured 0.83–0.91× at
n=40/70 — allocator contention — confirming the scratch reuse is what lets it scale.)

## Gate
Serial path kept for `pairs.len() < 16`. Validated: 3 jacobian unit tests pass.
