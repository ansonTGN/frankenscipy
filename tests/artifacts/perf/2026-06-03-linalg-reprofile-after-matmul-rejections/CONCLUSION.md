# fsci-linalg reprofile after matmul rejected levers

Parent bead: `frankenscipy-8l8r1`
HEAD: `9ef69118f644ce108994d9400bf0f8387fec53f0`

## Scope

This was a no-source-edit reprofile after the B-flat and A-row-ref matmul trials
were rejected. The crate-scoped RCH Criterion run was stopped after the main
linalg groups completed and after `baseline_solve/1000x1000`; the later
100-sample 2000x2000 baseline rows were not needed for selecting the next
one-lever matmul child and would have tied up the RCH lane for several more
minutes.

## Completed top rows

| row | median |
| --- | ---: |
| `matmul/1024x1024` | 650.36 ms |
| `baseline_solve/1000x1000` | 214.69 ms |
| `matmul/768x768` | 145.70 ms |
| `lstsq/512x256` | 135.90 ms |
| `pinv/512x256` | 123.60 ms |

Verdict: among completed rows, dense `matmul/1024x1024` remained the top
profile-backed linalg target, so the next child bead stayed on the GEMM lane.

