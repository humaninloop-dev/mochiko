# ADR — `review-plan-artifacts` user-ruled true-deletion body cut

**Status:** ruled (user), built same day · **Date:** 2026-08-26

## Context

The user directed a 90% verbosity reduction of
`plugins/mochiko/skills/review-plan-artifacts/` (body 13,521 chars), explicitly on the
`review-brainstorm` pass precedent (v0.83.0, same day, parallel session). A
`compressing-skills` pass was opened. The precedent's lesson was inherited rather than
relearned: the route is true deletion in a single file — no breakup into a new reference
(rejected there as verbosity-shifting), no relocation. A non-compressor seat authored the
113-entry rule inventory (`evals/review-plan-artifacts/rules.json`) before the gate.

## Decision

1. **True deletion, single file.** The body is rewritten as six paragraphs —
   identity + sibling boundary, the four lenses, the protocol, the cycle-card check set,
   incremental mode, floors. Every behavioral rule of the baseline body survives as a
   compressed clause, or is deleted only where its single source already lives in an
   untouched `references/` file (the mirror-checklist tables). Both `references/` files,
   the Tier-1 checker script, and the `description:` are untouched.
2. **Rule-complete over strict-90.** Landed body: **4,901 chars, −63.8%** (after three
   inventory-driven clause restorations: R-070 flag-only-between-artifacts, R-093
   good-enough-is-never-ready, R-098 obvious-never-exempts-documentation). At the
   ratification gate the user ruled: ship the rule-complete cut. The deeper options were
   declined with the deaths named: ~−82% deletes Incremental Review Mode (doubly-KEPT
   v0.15.0/v0.26.0), the cycle-card qualifiers, and the Floors' severity-discipline rules
   (~25 rules); strict −90% (≤1,352 chars) additionally deletes the cycle-card check set —
   leaving a dead pointer in `ARTIFACT-CHECKLISTS.md`, which explicitly punts to that body
   row — and the adopt-first lens, which `patterns-adopt-first` cites as BLOCKING here.
3. **Why this skill's rule-complete floor sits higher than `review-brainstorm`'s
   (−78.8%):** the cycle-card check set is body-only (no reference home; relocation fenced
   D5); Incremental Review Mode is doubly protected and plan-only unique; and three of the
   four lenses are consumer-cited by six other primitives (R10 — paraphrase that breaks a
   consumer is a failed rule).
4. **Supersessions carried by this ruling:** the [v0.15.0] KEPT report-shape block's yaml
   form (fields survive as clauses); the [v0.26.0] KEPT long forms of Red Flags / Common
   Rationalizations / Incremental Review Mode (rules survive compressed); the [v0.64.0]
   guardrails keep-set's section forms and the floor line's verbatim wording (substance
   intact); the Review Focus table's analysis / store-delta / design / cross-artifact rows
   (superseded to their single-source checklists in the untouched reference). The
   [v0.67.0] lenses, [v0.53.0] carve-out, [v0.75.0] oracle semantics, [v0.76.0] `--check`
   citation, and [v0.81.0] store-delta grade all survive compressed, no rule deleted.
   Full disposition map: `.mochiko/strips/review-plan-artifacts.md` [v0.87.0] entry and
   `evals/review-plan-artifacts/pass-report.md`.
5. **Eval deferred** ("cut now, eval validates later", the v0.82.0/v0.83.0 pattern): the
   compression-eval slot for this skill runs as a post-cut regression check — `rules.json`
   is already non-compressor-authored from the pre-cut baseline; 3 goldens, probe, and
   pre-registration follow when that check runs. A lost load-bearing rule re-adds via the
   strips re-add path.
6. **Budget re-seeded** 10,855/13,569 → 4,901 (cap 6,127, +25% rounded up) per R11.

## Rationale

Same as the precedent: the skill's value on invoke is its rules, not its prose. This
skill's prose was thinner to begin with (two prior editorial waves) and its remaining mass
is rule-dense and heavily consumer-cited, so the honest floor lands at −63.8% rather than
−78.8%. Deletion beyond that point removes rules, not verbosity.

## Alternatives considered

- **Force −82% or −90%** — offered at the gate with the rule deaths named; declined.
- **Breakup into a new reference** — not drafted; rejected by the user in the
  `review-brainstorm` pass as verbosity-shifting, and the objection transfers unchanged.
- **Eval-before-cut (full `compressing-skills` grid)** — superseded by the standing
  "cut now, eval validates later" pattern (v0.82.0, v0.83.0).
