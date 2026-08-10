# CP2 checkpoint report — specify stage, single replicate (PRELIMINARY: n=1, no noise guard)

Judged blind (sets E–H, fresh letters; key untouched by the judge). De-anonymized against
`report/cp2-anon-key.txt`.

## Scores vs baseline (D6 rule, preliminary)

| Arm | Run | Score | Δ vs baseline | 10% rule (prelim) | Floors | Routing/staffing |
|---|---|---|---|---|---|---|
| baseline | specify-baseline-r1 | **98.2** (G) | — | — | PASS | clean |
| **body** | specify-body-r1 | **94.6** (H) | **−3.7%** | within | PASS | 0 fire misses |
| **descriptions** | specify-descriptions-r1 | **92.9** (F) | **−5.4%** | within | PASS | 0 misses, both sibling traps resolved |
| **agents** | specify-agents-r1 | **92.9** (E) | **−5.4%** | within | **F-X1 VIOLATION** | 4/4 varied personas staffed, 0 route misses |

Cost (lead skill-bytes): baseline 78,315 · body 52,362 (−33%) · descriptions/agents ≈ original bodies (not those arms' measurement).

## Headline

**All three cut arms within the 10% threshold on score at n=1.** Baseline won this stage —
the inverse of CP1 (where baseline lost on over-ceremony). Spread rides two rubric rows only:
planted-vague-zone honesty (SP7) and edge-case depth (SP12); baseline was the only set
honoring every vague zone.

## The F-X1 floor violation (agents arm) — attribution caveat

The judge (artifacts-only, correctly) found no evidence of an independent grade in the agents
set. The run's own capture claims the review ran as an independent subagent but the lead
cleared the fix round itself and the artifacts carry no verdict/disposition trail. Two readings:

- **Strict D6:** floor violation in a cut-arm replicate = automatic "detail pays" for that
  floor's section — i.e. agent example blocks return.
- **Attribution doubt (lead's flag):** the violated floor is review-evidence discipline — a
  run-lead process behavior. The causal chain from "example blocks removed from 6 agent
  descriptions" to "run-lead didn't leave review evidence in artifacts" is weak; staffing
  (the arm's actual mechanism) was 4/4 correct. A same-arm r2 would test whether the
  violation reproduces; if it does not, it was run noise, not the variant.

Per D6 the floor rule is absolute; per the noise guard, n=1 spread cannot be assessed. The
honest disposition is CP3 for this arm at minimum.

## Fire-rate / route summary across CP1+CP2 (both routing arms)

- descriptions: 0 misses over 12+ moments across 2 commands, incl. deliberate sibling traps.
- agents: 0 route misses over 9+ staffing decisions; 4/4 varied personas correctly staffed at
  specify; correct non-staffing (validator, principal-architect) with recorded basis.

## Two-stage picture (n=1 each)

| Arm | Setup Δ | Specify Δ | Combined mean Δ |
|---|---|---|---|
| body | +3.7% | −3.7% | ≈ 0 |
| descriptions | −1.9% | −5.4% | −3.6% |
| agents | +3.7% | −5.4% (+floor) | −0.9% (+floor) |

## Caveats (binding)

- n=1 per cell; D6's formal verdict requires ≥2 replicates; noise guard unfired.
- CP1/CP2 baseline rank flipped (4th → 1st) — direct evidence single-run variance is real
  and material at these margins.

## Decision asked of the user

CP3 options: (a) full r2 wave (8 runs); (b) targeted r2 — agents arm both commands (floor
reproduction) + body arm both commands (the ship-candidate closest to verdict); (c) stop and
rule on n=1 evidence (recorded as a user ruling over the D6 replicate requirement).
