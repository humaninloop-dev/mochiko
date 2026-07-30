# Shape-home revision becomes an encoded keeper-skill mode

**Status:** ruled · **Date:** 2026-07-30

## Context

The Layer-2 mesh rewrite (team-method D1–D3) is a standalone, rulings-driven edit to
`templates/command-shape.md` — the first shape revision riding neither a conversion
checkpoint (v2: specify's S8, 2026-07-19) nor a strip wave (v3: token-reduction wave 1,
2026-07-23). A pre-dispatch assessment found the keeper skills had no procedure for that
mode: `authoring-commands` covered author/convert/strip only (template edits appear as
"flag it, never slip it in" plus the first-conversion checkpoint), and
`validation-command-shape` graded commands and strip waves, not the home's own content —
leaving a ruled rewrite's fidelity ungraded, an author≠grader gap.

## Decision

Encode the mode in both keeper skills at v0.30.0:

- `authoring-commands` **Job 4 — shape-home revision**: rulings are the requirements
  (gaps route back to the user, never invented); shape altitude; rewrites/removals follow
  Job 3's strip-note logging while pure additions land in the revision's decision row
  (the v3 run-cost precedent); footer version stamp; named conformant-command re-audit
  set; independent handoff.
- `validation-command-shape` **checks 11–14 (shape-revision runs)**: floor — footer
  stamped, rewrites logged; ceiling — ruling fidelity by diff against the prior version,
  altitude + re-audit-set coverage.
- `command-architect` description, fourth example, and skill bullet carry the new mode.

## Rationale

The alternative — compensating in each dispatch brief — leaves the procedure in
conversation context and the grading unowned. Encoding it once makes every future shape
revision dispatchable on procedure; standing-seat-lifecycle's v4+ Layer-2 rewrite is
already queued behind the same surface.

## Alternatives considered

- **Brief-only compensation** — carry the rulings, logging expectations, and grader
  assignment in every dispatch prompt. Rejected: unrepeatable, and the fidelity-grading
  gap survives.
- **A separate revision skill** — rejected: the shape's keeper pair already owns this
  surface; a third home splits it.
