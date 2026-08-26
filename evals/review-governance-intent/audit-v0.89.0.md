# Author≠grader audit — `review-governance-intent` v0.89.0 true-deletion body cut

**Verdict: PASS** (round 1, no fix list) · Grader: independent validator, authored none of the
graded material · Date: 2026-08-26 · Ceremony: `.claude/rules/mochiko/primitive-edits.md`

All 70 inventory rules survive, both budgets pass, all three shared reference pointers resolve,
and no citation anywhere in the plugin depends on a section this cut removed. Four non-blocking
advisories are recorded at the foot, one of them a pre-existing description/body mismatch this
cut did not create.

## 1. Deterministic char-budget pre-assert (D7)

| Class | Measured | Budget | Delivery cap | Result |
|---|---|---|---|---|
| Skill body | **5,562** | 6,953 | — | PASS |
| `description:` | **483** | 604 | 1,536 | PASS |

Baseline re-measured independently from `git show HEAD:` at **8,150** body / **483**
description, so 8,150 → 5,562 = **−31.75%**, matching the −31.8% recorded everywhere. The
description is byte-identical to baseline and sits exactly at its recorded winner figure. The
budget arithmetic checks: 5,562 × 1.25 = 6,952.5, rounded up to 6,953.

## 2. Preserved responsibilities — all 70 rules walked

**LOST: none. 70/70 have a home.** This skill has no `references/` directory of its own; every
rule lands in the body or behind one of the three shared pointers.

- **Survives in the body: 66** — R-001 through R-030, R-033 through R-070, spread across the
  five paragraphs: identity and contract (L8), lens and jurisdiction (L10), the four-leg protocol
  (L12), the survivor report (L14), and Floors (L16).
- **Survives behind an explicit shared-source pointer: 4** — R-017
  (`../authoring-constitution/references/INTERROGATION-AGENDA.md`, the ten-dimension coverage
  yardstick), R-026 and R-031 (`../review-brainstorm/references/CROSS-EXAM.md` for the pair
  protocol, and its external-claim carve-out for R-031's source-re-read clause), R-032
  (`../review-brainstorm/references/EXTERNAL-CLAIMS.md`).

The three restorations the strip and ADR claim are all present and verified: R-005
(traceable-contract), R-025 (lead-introduces plus the one-shot four-message exchange), R-031 (the
floor-class qualifier on the external-sourced fact).

Two rules earned closer scrutiny, and both hold:

- **R-008** requires both that out-of-lens findings are reported and that the reviewer never
  merges its set with the counterpart's. The body carries the first limb directly ("depth, never
  jurisdiction: report real out-of-lens trips"). The second is carried twice over: L14 requires
  "your own survivors … flagged counterpart duplicates", and the pointed-at `CROSS-EXAM.md`
  states "the cross-set merge is the lead's" as its own single source. Fully homed.
- **R-031**'s source-re-read clause is not restated in the body, correctly — `CROSS-EXAM.md`'s
  External-claim carve-out owns it ("both reviewers re-read the cited sources before the finding
  survives"), and the body's "runs per EXTERNAL-CLAIMS.md, never argued" reaches it.

## 3. Internal coherence

All three relative links resolve from the skill directory:
`../authoring-constitution/references/INTERROGATION-AGENDA.md`,
`../review-brainstorm/references/CROSS-EXAM.md`, and
`../review-brainstorm/references/EXTERNAL-CLAIMS.md`.

The FAIL posture is present ("**Never default to `ready`** — earned by a completed hunt, never by
looking reasonable", plus the description's "defaults to a FAIL posture"), and the
never-a-participant fence appears in both the opening line and Floors.

`CROSS-EXAM.md`'s framing of this skill still fits the compressed body exactly. That file states
both skills "run it by reference — each skill supplies its own substrate bindings (what 'the
artifact' and 'the fact substrate' are, and where fact disputes route)". The new body supplies
precisely those three, and does so under the file's own term: `bindings — *artifact*: … *fact
substrate*: … *fact authority*: …`. It also discharges the file's rule that "the invoking skill
names" the fact authority, with all three fact types and their three routes intact.

## 4. Stale pointers, both directions

No dead pointers, verified rather than assumed. Every citation of this skill in the plugin is
skill-level, and **none names a section or anchor of it**: `agents/devils-advocate.md:7,22`,
`skills/mochiko/SKILL.md:44,141`, `review-brainstorm/references/CROSS-EXAM.md:4`, and
`review-brainstorm/references/EXTERNAL-CLAIMS.md:94`. The router row at :44 is the densest
citation and every term it promises survives — cold intent reviewer, frozen confidence-marked
synthesis, before ratification, the coverage/coherence pair-or-solo shape, survivors plus tally
plus recommended status, never a session participant, clearing the lead's and ratification the
user's.

A sweep for this skill's deleted headings ("Independent cold read", "Survivor report",
"Cross-examination", "Independence", "Common Mistakes") returns only other skills' own headings.
Nothing points into a heading this cut removed.

`commands/setup.md`'s four dependent clauses all still hold: the pre-ratification cold seat and
the lead's pen (R-042, R-043), the blind-map dispatch staying command-side with the body merely
compatible and not re-homing it (R-068), coverage-survivor routing having a named coverage lens
and the agenda yardstick to consume (R-069), and the Goal's default-FAIL list resting on
before-ratification timing plus author≠grader independence (R-070).

## 5. Record-layer consistency

Strip `[v0.89.0]`, the ADR, the `DECISIONS.md` row and the cost ledger all carry 8,150 → 5,562,
−31.8%, budget 5,562/6,953, shipped v0.89.0. Every disposition-map claim checks out against the
body, including the MANDATORY KEPT reconciliation: the v0.26.0 elements that survived v0.63.0
(pair-protocol bindings, the D1 exclusion), the v0.63.0 guardrails keep-set, the v0.65.0
declared-level machinery with its critical-gaps arm, and the v0.46.0 its-command-states-them
clause. The strip's "No dead pointers created" is true, verified against the sweep in section 4.

`plugin.json` is at 0.88.0 — correct pre-bump ordering.

## Advisories — none blocking

1. **The cost-asymmetry rationale is deleted** (a gap here costs one interrogation follow-up
   versus re-ratification plus a produce→validate round). R-001's behavioral core — the
   before-ratification timing — survives in the opening line; only the reason prose is gone, and
   the strip discloses this deletion by name.
2. **R-043 thinning.** "You never write the file" is no longer explicit. It is entailed three
   ways: findings enter through the lead's pen, the report is a message with no report files, and
   the reviewer never authors, revises, or ratifies.
3. **Pre-existing description/body mismatch, not this cut's doing.** The `description:` advertises
   "the bounded delta-pass on a material post-review edit", and no body text describes that mode.
   I checked the baseline: "delta-pass" appears only in the description there too, and per the ADR
   the v0.63.0 benchmark wave stripped the verify/delta-pass prose. So the cut neither created nor
   worsened this. Worth a future ruling on whether the description should keep advertising it.
4. **Still owed at the bump:** the `CHANGELOG.md` 0.89.0 entry (release gate 4) and the
   `marketplace.json` sync.

No fix list — nothing blocks this cut from shipping at v0.89.0.
