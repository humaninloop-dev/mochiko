# ADR — `review-brainstorm` user-ruled true-deletion body cut

**Status:** ruled (user), built same day · **Date:** 2026-08-26

## Context

The user directed a 90% verbosity reduction of `plugins/mochiko/skills/review-brainstorm/`
(body 11,754 chars; the fattest remaining review-skill body after the v0.82.0
`review-feasibility` cut). A `compressing-skills` pass was opened. The first draft followed
the `review-feasibility` v0.82.0 precedent — a breakup into a floors-and-dispatch body
(−87.0%) plus a new `references/REVIEW-PROTOCOL.md` carrying the relocated protocol. The user
rejected it mid-pass: "all the verbosity is shifted to review protocol." The objection holds —
unlike `review-feasibility`, whose reference file pre-existed and twinned its body, here the
reference would be new, the corpus would grow, and a real review run reads the protocol file
anyway, so the per-invoke saving is mostly accounting.

## Decision

1. **True deletion, single file.** The body is rewritten as a compressed
   identity + protocol chain + verify pass + floors — no new reference file, no relocation.
   Every behavioral rule of the old body survives as a compressed clause; all rationale
   prose, worked framing, and long-form tables are deleted outright. The three existing
   `references/` files and the `description:` are untouched.
2. **Rule-complete over strict-90.** Rule-retention floor as first measured (~1,965 chars)
   proved low by six rules at audit; the true rule-complete floor is the landed 2,497. Reaching
   −90.0% (≤1,175) required deleting six ruled/load-bearing rules outright (reopen-born
   verify grading, synthesis fidelity sample, class-6 calibration clause, coverage
   materiality gate, blind-map grounding fence, cross-exam substrate binding). At the
   ratification gate the user ruled: ship the rule-complete cut. Landed body: **2,497 chars,
   −78.8%** — the ratified 1,965-char draft gained post-gate repairs enforcing its own
   rule-complete claim: the integrity-lens sample-audit clause (+71, lead-caught), then six
   audit-forced restorations (lens taxonomy, coverage-severity test, full cross-exam
   substrate binding, decision(s)-touched field, fidelity-sample criteria, the v0.46.0
   pointer clause; audit FAIL round 1, 7 blocking → fix round → delta PASS).
3. **Supersessions carried by this ruling:** the [v0.26.0] whole-body `KEPT:` survivor
   ruling ends; the [v0.64.0] floor line's verbatim wording is superseded by its compressed
   form (substance intact); the [v0.60.0] and [v0.67.0] protected machinery survives
   compressed, no rule deleted. Full disposition map: `.mochiko/strips/review-brainstorm.md`
   [v0.83.0] entry.
4. **Eval deferred** ("cut now, eval validates later", the v0.82.0 pattern): the
   compression-eval slot for this skill runs as a post-cut regression check — non-compressor
   seat authors `rules.json` from the v0.82.0 baseline + 3 goldens; a lost load-bearing rule
   re-adds via the strips re-add path. Pass artifacts: `evals/review-brainstorm/`.
5. **Budget re-seeded** 11,508 → 2,497 (cap 3,122, +25% rounded up) per R11.

## Rationale

The skill's value on invoke is its rules, not its prose: the reviewer needs the floors, the
hunt-class list, and the chain order — the anti-anchoring essays and worked framing teach the
author, not the seat. Relocation would have preserved the reading cost while claiming the
reduction; deletion makes the claim true. Stopping at the rule-retention floor keeps the cut
lossless at the rule level, which is what the deferred regression check will grade.

## Alternatives considered

- **Breakup into a new reference (−87.0% body)** — drafted, rejected by the user as
  verbosity-shifting; corpus grew and per-invoke delivery was unchanged on real runs.
- **Force −90.0% (≤1,175 chars)** — offered at the gate with the six rule deaths named;
  declined in favor of rule-complete.
- **Eval-before-cut (full `compressing-skills` grid)** — offered; the user took the ruled-cut
  path, precedented at v0.82.0.
