# ADR — `validation-constitution` user-ruled true-deletion body cut

**Status:** ruled (user), built same day · **Date:** 2026-08-26

## Context

Sixth pass of the 2026-08 compression series, scouted and gated alongside
`review-governance-intent`. A `compressing-skills` pass was opened; a non-compressor seat
authored the 69-entry rule inventory (`evals/validation-constitution/rules.json`) before
the gate.

## Decision

1. **True deletion, single file.** Body rewritten as five paragraphs (identity+scope ·
   inputs · protocol · VALIDATION RESULT · floors); every behavioral rule survives as a
   compressed clause; the VALIDATION RESULT fenced block compresses to a field-complete
   enumeration (every field and sub-item kept). `description:` and both `references/`
   files untouched.
2. **Rule-complete over deeper.** Landed body: **5,103 chars, −33.1%** (after one
   inventory-driven restoration: the R-047 red-flag STOP-and-restart meta-rule with its
   named rationalization family). At the gate the user ruled: ship the rule-complete cut;
   ~−55% declined with the deaths named (the 10 VALIDATION-RESULT field rules (R-028–R-037), the
   missing-input FAIL trio, the rationalization-family floors).
3. **Why the floor is shallow:** the v0.63.0 benchmark wave already cut this skill −44%
   (Steps 2–7 superseded into `references/QUALITY-CHECKLIST.md`); the remaining body was
   the keep-set plus the v0.65.0 adaptive-depth extensions. This pass's real yield is the
   three anti-rationalization table forms (~2,300 chars) compressed into Floors clauses.
4. **Supersessions carried by this ruling:** the [v0.25.0] KEPT Red Flags + Common
   Rationalizations *table forms* (every distinct rule survives as a Floors clause); the
   v0.63.0 keep-set's section and fenced-block forms and the floor line's verbatim wording
   (substance intact). The v0.65.0 extensions and v0.76.0 schema re-key survive untouched
   in substance. Disposition map: `.mochiko/strips/validation-constitution.md` [v0.90.0]
   and `evals/validation-constitution/pass-report.md`.
5. **Eval deferred** per the standing pattern; post-cut regression check; strips re-add
   path for any lost load-bearing rule.
6. **Budget re-seeded** 6,734/8,418 → 5,103 (cap 6,379) per R11.

## Rationale

Same bounded-yield finding as the sibling pass: a previously-benchmark-cut keep-set body
compresses ~a third, not nine-tenths. With these two landed, every `review-*`/
`validation-*` skill in the library has taken a ruled cut; the compression series' cheap
targets are exhausted — further reduction work belongs to the eval-graded pilot path
(`patterns-entity-modeling`), not ruled cuts.

## Alternatives considered

- **Force ~−55%** — offered with the rule deaths named; declined.
- **Breakup into references** — banned (precedent + D5).
- **Eval-before-cut** — superseded by the standing pattern.
