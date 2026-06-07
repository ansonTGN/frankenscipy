# Compact-WY Replay Pass: Column-Panel Rejection

Bead: `frankenscipy-z41sx`

## Target

Continue the profile-backed bidiagonal SVD factor replay campaign after
`frankenscipy-8l8r1.46` rejected index/slice cleanup. The target lineage was
Alien Graveyard `9.6 Communication-Avoiding Algorithms`: reduce data movement in
Householder replay with a cache-aware/tree-like application schedule while
preserving the public SVD-family contract.

Fresh RCH baseline:

- Worker: `ts1`
- Probe: `thin_bidiag_factor_replay_perf_probe`
- Shape: `1024x512`
- Dense-product reference: `470.583819 ms`
- Current serial reflector replay: `236.800143 ms`
- Replay digest: `0x8f521a39638fb520`

## Lever Tried

Column-panel left reflector replay for the thin `U` factor. Instead of replaying
each left reflector across every column before moving to the next reflector, the
candidate replayed all left reflectors over `64`-column panels.

This is a cache/data-movement schedule change, not a floating-point rewrite:
each output column still sees the same reflector order, dot-product summation
order, update order, sign canonicalization, singular-value ordering, rank/rcond
thresholds, errors, and RNG absence.

## Proof

RCH `ts1` bit proof passed:

- `thin_bidiag_column_panel_replay_matches_serial_bits`: passed
- Shapes included `96x80`, crossing the `64`-column panel boundary
- Equality policy: every `U` and `Vt` entry compared by `f64::to_bits`
- Digest: unchanged at `0x8f521a39638fb520`

## Rebench

Same-binary A/B on RCH `ts1`:

- Serial left replay: `257.106428 ms`
- Column-panel left replay: `257.439329 ms`
- Speedup: `0.998707x`
- Digest: `0x8f521a39638fb520`

## Decision

Rejected. The candidate preserved bits but was effectively flat with a slight
regression, so it does not clear the Score `>= 2.0` keep gate. Source was
restored; no production code from this trial remains.

Score: `0.0`.

Next primitive: do not retry panel ordering or indexing cleanup. Move to true
compact-WY/block reflector composition or a two-stage communication-avoiding
bidiagonal reducer where multiple reflectors are mathematically composed and
applied with a GEMM-shaped safe-Rust kernel. That path may require an explicit
golden migration if it changes floating-point bits.
