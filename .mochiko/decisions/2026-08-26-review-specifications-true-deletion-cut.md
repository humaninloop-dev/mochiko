# ADR — `review-specifications` user-ruled true-deletion body cut

**Status:** ruled (user), built same day · **Date:** 2026-08-26

## Context

Fourth pass in the 2026-08 compression series (v0.82.0 `review-feasibility` · v0.83.0
`review-brainstorm` · v0.87.0 `review-plan-artifacts`). The user directed the same cut for
the next candidate; `review-specifications` was selected at the candidate survey as the
fattest remaining `review-*` body (12,184 chars, zero reference files). A
`compressing-skills` pass was opened; a non-compressor seat authored the 81-entry rule
inventory (`evals/review-specifications/rules.json`) before the gate.

## Decision

1. **True deletion, single file.** The body is rewritten as six paragraphs —
   identity+routing, method (question craft + the six-class taxonomy), feature layer
   (baseline rule + 10 checks), Screens & Flows (walk rules + 8 checks), severity+output,
   floors. Every behavioral rule survives as a compressed clause; all worked-example
   tables, section forms, and rationale prose are deleted. The `description:` is untouched.
2. **Rule-complete over deeper.** Landed body: **6,187 chars, −49.2%** (after five
   inventory-driven clause restorations: R-012, R-015, R-064, R-065, R-066). At the
   ratification gate the user ruled: ship the rule-complete cut. Declined with deaths
   named: ~−70% degrades the 18 ruled check clauses to bare labels and deletes the
   coverage clause + 4 Floors rules (~30 rules); −90% (≤1,218) deletes the feature-layer
   and S&F check sets wholesale — breaking the router row's "including the feature-layer
   grade" citation and `devils-advocate`'s canonical-home anchor for the gap taxonomy,
   severity rubric, and output format.
3. **Why this floor is the series' shallowest** (−49.2% vs −90.0%/−78.8%/−63.8%): zero
   reference files — the earlier cuts leaned on 28k/17k/10k chars of untouched
   single-source checklist references, here all 18 ruled checks and the six-class taxonomy
   are body-only (relocation fenced D5); and the body was already twice-compressed
   (v0.26.0 editorial wave, v0.63.0 benchmark-ruled guardrails cut). What remains is rule
   mass: deletion beyond this point removes rules, not verbosity.
4. **Supersessions carried by this ruling:** the [v0.26.0] KEPT severity and Core
   Principle *table forms* (substance survives compressed); the [v0.63.0] guardrails
   keep-set *section forms* and the floor line's verbatim wording (substance intact); the
   worked-example rows throughout. The [v0.25.0] canonical-home taxonomy, [v0.67.0]
   class-6 calibration, [v0.58.0] feature-layer checks + R13 baseline rule, [v0.50.0] S&F
   checks + walk rules, [v0.53.0] carve-out, and [v0.82.0] envelope wording all survive
   compressed, no rule deleted. Full disposition map:
   `.mochiko/strips/review-specifications.md` [v0.88.0] and
   `evals/review-specifications/pass-report.md`.
5. **Eval deferred** ("cut now, eval validates later"): the compression-eval slot runs as
   a post-cut regression check; `rules.json` is already non-compressor-authored from the
   pre-cut baseline. A lost load-bearing rule re-adds via the strips re-add path.
6. **Budget re-seeded** 11,271/14,089 → 6,187 (cap 7,734, +25% rounded up) per R11.

## Rationale

The series' premise holds — the skill's invoke-time value is its rules — but this skill is
the boundary case that shows where the premise stops paying: with no reference tree and 18
ruled checks, half the body IS rules, and the honest floor lands at half rather than a
tenth. Recording that gradient is itself useful: future candidates should be sized by
prose-over-rules ratio, not raw body chars.

## Alternatives considered

- **Force −70% or −90%** — offered at the gate with the rule deaths named; declined.
- **Breakup into a new references/ file** — banned twice over: the review-brainstorm
  verbosity-shifting rejection transfers, and D5 fences topology (no new files).
- **Eval-before-cut** — superseded by the standing pattern (v0.82.0/v0.83.0/v0.87.0).
