# Strip notes — `skills/authoring-user-stories/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T1;
ratified 2026-07-24).

## [v0.28.0] Reference restatements, excuse-column red flags, and homed mistake rows stripped (body 179 → 116, −35%, in-band)
- **Disposition:** deduped → verified pre-existing homes, each Read before landing (nothing
  written to any reference this wave): the P1/P2/P3
  table → one-line enumeration + `references/PRIORITY-DEFINITIONS.md` pointer (richer:
  definitions, criteria, business signals, decision tree, distribution guidelines, its own
  mistakes table), the good/bad scenario example pair → `references/EXAMPLES.md` pointer (its
  Good-vs-Bad Comparisons section holds the same pair richer, plus journey / justification /
  independent-test pairs; pointer moved onto the scenario-rules section), the Given/When/Then
  definition bullets → folded into the pattern intro line, the script-checks bullets → a
  parenthetical on the intro line (command kept) · Red Flags trigger bullets + no-exceptions
  list → one STOP paragraph (the bullets map ~1:1 onto the kept Common Rationalizations Excuse
  column — "the user just wants quick stories" verbatim; batch-2 precedent) · **Common Mistakes
  deleted whole** (all 6 rows homed: technical-stories → When-NOT-to-Use + the format block's
  journey field; missing-justification → the format's one-line field + rationalizations rows
  1/6 + the script's justification check; implementation-details → scenario rule 4 + EXAMPLES'
  bad pairs; vague-outcomes → rule 4 + checklist; compound-stories → rule 1 ("more than 3 means
  the story is compound"); non-testable → rule 3 + checklist)
- **Tier failed:** 1 throughout (verified homes) · n/a for the fold-ins
- **Content:** one table, two fenced examples, three bullet lists, ten red-flag/no-exception
  bullets, six mistake subsections
- **Consumers assessed:** wave-open enumeration — 7 citing files (requirements-analyst, specify,
  authoring-requirements, authoring-slices, spec-template, artifact-format, mochiko router);
  none links a section anchor; `scripts/validate-user-stories.py` checks the authored artifact,
  not these teaching sections. Kept: the story format block + scenario rules ([v0.23.0] T1-ruled
  form, untouchable), Independent Test bullets (only home), Quality Checklist, Common
  Rationalizations table, the letter/spirit epigraph (R4b: anchored by the discipline paragraph
  below it). Session ruling: wave-4 batch-3 ratified 2026-07-25.

## [v0.23.0] Acceptance scenarios compressed to one line each, cap 2-4 → 2-3 (T1, user-ruled)
- **Disposition:** revised per the wave-2 T1 ruling (deleted prose replaced by the dense form; nothing relocated)
- **Tier failed:** artifact density (epic D4 extension): multi-line Given/When/Then prose re-paid ~10× per feature via mandated reads; the G/W/T grammar carries the testability, the line breaks carried nothing
- **Content:** the story-format block's multi-line scenario shape; the "2-4 scenarios" rule (now 2-3 with the compound-story rationale); the multi-line good example (now one line); quality-checklist counts. `references/EXAMPLES.md` rewritten to the dense form (3 examples: journeys ≤ 2 lines, one-line why/test/scenarios — same substance, same story content).
- **Consumers assessed:** spec.md producers (specify) + review-specifications (retargeted this wave: density-is-not-a-gap note) + `scripts/validate-user-stories.py` (checked: numbered-scenario + G/W/T keyword regexes match the one-line form — no script change needed).
