# ADR — `validator` persona re-indexed to the router (general-purpose grader made explicit)

- **Date:** 2026-08-26
- **Status:** ruled (user, in-session) + built
- **Ships:** v0.84.0
- **Primitive:** `plugins/mochiko/agents/validator.md`
- **Strip entry:** `.mochiko/strips/validator.md` [v0.84.0]

## Context

The user read the validator persona and asked whether it is a general-purpose grader or a
constitution-only one — the `## Skills you lean on` section carried a single inline bullet
(`mochiko:validation-constitution`) whose presence read as scope. The persona was always
intended as the library's one generic grader (router agents-table row: "one generic
independent grader for any cluster"), and the generic fallback paragraph said so, but the
section's shape argued against it. Three defects converged:

1. **Misleading shape.** A one-item "Skills you lean on" list reads as a scope declaration,
   not a bundled-checklist note. It confused the persona's own maintainer.
2. **Stale + duplicated content.** The bullet described grading "a drafted constitution" —
   there is no constitution.md (the graded set is the governance surface set) — and restated
   `validation-constitution`'s checklist internals (Three-Part Rule, trace-ID cross-check,
   waiver/floor accounting, scans, version-bump), which that skill's own `description:`
   single-sources. Altitude violation on top of staleness.
3. **Scaling defect.** An inline per-skill list obliges a persona edit (a full landing) every
   time a checklist skill is authored. The router (`skills/mochiko/SKILL.md`) already indexes
   every skill with when-to-reach-each guidance and is the library's discoverability surface
   ("a primitive that is not in this router fails discoverability").

## Decision

The persona indexes its checklists **via the router**, not inline. The section is rewritten as
a three-step selection order:

1. the dispatch brief's named checklist or explicit bar wins;
2. otherwise Read the router and reach for the domain-matching checklist — the `validation-*`
   family is natively the validator's (authoritative binary grade); a `review-*` checklist
   reached this way lends its checklist only, the verdict staying binary and the clearing
   staying with the dispatching contract (consistent with the router's two-family split and
   with the primitive-edit audit's "matching `validation-*`/`review-*` skill otherwise"
   clause);
3. no fit — fall back to the generic method against the handed bar (the v0.45.0 kept fallback,
   substance preserved near-verbatim).

The frontmatter `skills: validation-constitution` mount is **kept** and explicitly framed as
delivery for the common case (the setup run's grading seat), never scope. New checklist skills
register in the router only; no per-checklist persona edit.

## What leaves / what stays

- **Leaves (superseded):** the inline `validation-constitution` bullet with its checklist
  internals and the "drafted constitution" wording; the framing sentence "When the artifact
  fits one cleanly, that checklist is your strongest asset — use it."
- **Stays:** the frontmatter `description:` (unchanged, 269 chars — budget 337); the
  `skills: validation-constitution` mount (v0.45.0 protected set); the generic-grader
  fallback (v0.45.0 protected set, substance near-verbatim in step 3); Core Identity, What
  You Produce, Iron Law, What You Reject, Your Judgment, Delegating Cheap Reads —
  byte-for-byte.

## Consumers assessed

- Router `skills/mochiko/SKILL.md`: agents-table validator row ("one generic independent
  grader for any cluster … (skills: validation-constitution)") and the two-family paragraph
  ("`validation-*` … on the `validator` persona (today: `validation-constitution`)") — both
  remain true; no router edit needed.
- `.claude/rules/mochiko/primitive-edits.md` (audit dispatch: "`mochiko:validator` grading a
  command against the command's own text … the matching `validation-*`/`review-*` skill
  otherwise") — the rewrite aligns the persona with this clause; no rule edit.
- `plugins/mochiko/skills/review-brainstorm/references/EXTERNAL-CLAIMS.md` (cited from the
  persona's Your Judgment section, untouched).
- Setup run's grading seat: unaffected — the mount survives.
- No command references the agent by name (grep of `plugins/mochiko/commands/`).

## Addendum — 2026-08-26, v0.85.0 follow-up cut

The closing paragraph's first sentence ("New checklist skills register in the router when they
are authored; the router read — not this file — is what keeps you current.") was cut the same
day on the user's challenge: it restated the router's own "Adding to the library" rule inside
a runtime persona (altitude), and its audience was the future maintainer, whose home is this
ADR and the strip layer — not the prompt. The second sentence ("The frontmatter `skills:`
mount is delivery for the common case, never your scope.") survives as the standalone closing
line: it is the load-bearing fix for the mount-read-as-scope misread that motivated this ADR.
Strip: `.mochiko/strips/validator.md` [v0.85.0]. Ships v0.85.0.

## Protected-content reconciliation

The v0.45.0 strip entry's `Kept deliberately` set — the `validation-constitution` mount and
the generic-grader method (Iron Law, checklist-fallback paragraph) — survives this edit: the
mount is kept in frontmatter, the Iron Law is untouched, and the fallback paragraph's
substance lands as selection-order step 3. The v0.63.0 entry protects the `description:`
value's prose framing — untouched. No `KEPT:` line or `DECISIONS.md`-traceable line leaves
without this ruling.
