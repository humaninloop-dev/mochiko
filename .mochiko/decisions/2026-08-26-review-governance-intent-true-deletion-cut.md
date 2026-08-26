# ADR — `review-governance-intent` user-ruled true-deletion body cut

**Status:** ruled (user), built same day · **Date:** 2026-08-26

## Context

Fifth pass of the 2026-08 compression series (v0.82.0 review-feasibility · v0.83.0
review-brainstorm · v0.87.0 review-plan-artifacts · v0.88.0 review-specifications). The
user directed the series onward to the next candidates; this skill and
`validation-constitution` were scouted together. A `compressing-skills` pass was opened; a
non-compressor seat authored the 70-entry rule inventory
(`evals/review-governance-intent/rules.json`) before the gate.

## Decision

1. **True deletion, single file.** Body rewritten as five paragraphs (identity+contract ·
   lens+jurisdiction · protocol · survivor report · floors); every behavioral rule
   survives as a compressed clause. `description:` and the three shared reference
   pointers untouched.
2. **Rule-complete over deeper.** Landed body: **5,562 chars, −31.8%** (after three
   inventory-driven restorations: R-005, R-025, R-031). At the gate the user ruled: ship
   the rule-complete cut; ~−55% declined with the deaths named (verdict-criteria,
   fact-route, and declared-level rules degrade to labels; four mistake-row floors die).
3. **Why the floor is shallow:** the v0.63.0 benchmark wave already strip-cut this skill
   −46% (blind map, five-hunt-class table, coverage findings, verify/delta passes) — the
   baseline was already its keep-set plus the v0.65.0 adaptive-depth ruling. This pass
   removes the last rationale prose and the section/table forms only.
4. **Supersessions carried by this ruling:** the v0.63.0 keep-set's section/table forms
   and the review-evidence floor line's verbatim wording (substance intact); the v0.26.0
   KEPT elements surviving v0.63.0 (pair-protocol bindings, D1 exclusion), the v0.65.0
   declared-level machinery, and the v0.46.0 its-command-states-them clause all survive
   compressed, no rule deleted. Disposition map:
   `.mochiko/strips/review-governance-intent.md` [v0.89.0] and
   `evals/review-governance-intent/pass-report.md`.
5. **Eval deferred** per the standing pattern; the slot runs as a post-cut regression
   check; a lost load-bearing rule re-adds via the strips re-add path.
6. **Budget re-seeded** 7,089/8,862 → 5,562 (cap 6,953) per R11.

## Rationale

The series premise (invoke-time value = rules, not prose) holds, but the yield here is
bounded by the v0.63.0 benchmark cut having already taken the deep strip — confirming the
v0.88.0 ADR's sizing lesson: candidates are sized by prose-over-rules ratio, and a
previously-cut keep-set body carries mostly rules.

## Alternatives considered

- **Force ~−55%** — offered with the rule deaths named; declined.
- **Breakup into references** — banned (verbosity-shifting precedent + D5 topology fence).
- **Eval-before-cut** — superseded by the standing pattern.
