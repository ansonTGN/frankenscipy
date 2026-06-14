# Evidence: frankenscipy-8l8r1.106 CG stage profile and accepted-x rejection

## Target

- Bead: `frankenscipy-8l8r1.106`
- Crate: `fsci-opt`
- Profile-backed row: `cg/rosenbrock/10`
- Loop: `repeatedly-apply-skill` -> `extreme-software-optimization`, with `alien-graveyard` and `alien-artifact-coding` routing

## Baseline

Focused RCH baseline:

```text
rch exec -- cargo bench -p fsci-opt --bench optimize_bench --locked -- cg/rosenbrock/10 --sample-size 20
```

- Worker: `vmi1227854`
- Artifact: `baseline_cg_rosenbrock10_rch.txt`
- Row: `[214.25 us 220.15 us 225.87 us]`

## Stage profile

The stage probe matched public `cg_pr_plus` bit-for-bit before reporting split costs:

- Artifact: `stage_profile_cg_rosenbrock10_rch.txt`
- Worker: `vmi1293453`
- `public_match_bits=true`
- `public.digest=b55b14f8af7e4527`
- `profiled_first.digest=b55b14f8af7e4527`

Average per profiled run:

- Total runtime: `493525.4 ns`
- Wolfe runtime: `391532.4 ns`
- Wolfe value calls: `3778`
- Wolfe finite-difference gradient calls: `360`
- Actual finite-difference objective calls: `7900`
- Reserved objective calls: `6680`
- Accepted-x materializations: `368`, `14384.1 ns`
- Direction update runtime: `28521.3 ns`
- Allocation traffic: `2741` Vec events, `219280` Vec bytes

## Rejected lever

Attempted lever: carry the accepted trial point already materialized inside the strong-Wolfe gradient-probe path into CG, avoiding the post-line-search `add_scaled` materialization.

Same-worker RCH after row:

```text
RCH_REQUIRE_REMOTE=1 RCH_FORCE_REMOTE=1 RCH_WORKER=vmi1227854 RCH_TEST_SLOTS=1 rch exec -- cargo bench -p fsci-opt --bench optimize_bench --locked -- cg/rosenbrock/10 --sample-size 20
```

- Worker: `vmi1227854`
- Artifact: `after_accepted_x_cg_rosenbrock10_vmi1227854_rch.txt`
- Row: `[212.75 us 220.19 us 229.94 us]`
- Delta: `220.15 us -> 220.19 us` p50, effectively flat and slightly slower.

Ignored timing artifacts:

- `after_accepted_x_cg_rosenbrock10_rch.txt`: local fallback, not RCH proof.
- `after_accepted_x_cg_rosenbrock10_remote_rch.txt`: remote-required refusal while project gate was occupied, no timing row.

Decision: REJECT. Score is below the required `2.0` keep threshold because same-worker impact is zero.

The accepted-x source change is not retained in the local optimizer tree after rejection.

## Next route

Do not repeat accepted-gradient carry, accepted-x materialization, or scratch-only trial-buffer reuse. The next primitive must attack the measured dominant cost: strong-Wolfe finite-difference and value-call count/order. Candidate family: a deeper, behavior-audited finite-difference/Wolfe primitive that reduces objective-call traffic while preserving Wolfe comparison order, finite-difference component order, evaluation counters, FP bits, and RNG absence.
