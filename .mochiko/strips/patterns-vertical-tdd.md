# Strip notes — `skills/patterns-vertical-tdd/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.22.0–v0.23.0] entries — workflow-token-reduction waves 1–2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6a + D4/wave-2 rulings).

## [v0.27.0] Third-copy formats, TDD-sequence snippets, case subsections, and Common Mistakes stripped (body 368 → 204, −45%, in-band)
- **Disposition:** relocated/deduped → verified homes, each Read before landing: the Standard
  Cycle Format block (verbatim in `references/CYCLE-STRUCTURE.md` Cycle Anatomy; conforms-to
  `templates/tasks-template.md`, the declared single source — the in-file third copy was the
  drift risk its own intro warned about) · Task Numbering (CYCLE-STRUCTURE's Task ID Format,
  richer — adds the >4-tasks rule) · Common Mistakes, all six rows homed: tests-after +
  missing-TEST-task (cycle-format/template ordering), horizontal-disguised (in-file Core
  Principle 1 + SLICE-IDENTIFICATION Anti-Patterns), foundation-without-TDD (the identical
  in-file rationalization row), cycles-too-large (SLICE-IDENTIFICATION Size Calibration),
  missing-file-paths (CYCLE-STRUCTURE File Path Conventions + the in-file Quality Checklist row)
  · densified (form + in-file dedup): TDD Task Sequence 50 → 9 lines (four re-shown task-line
  snippets cut; behavioral glosses kept — red must FAIL and express acceptance criteria, green
  minimal across all layers, refactor no behavior change, TEST = real-infra gate checking
  **spec acceptance criteria**), Principle-2 tree + Principle-3 diagram → one-line principles
  pointing at the sections that carry them, Foundation-vs-Feature 24 → 6 lines (both
  identification questions + IP-XXX pointer kept), Simple/Split/Merge 30 → 8 (Case-column form,
  micro-examples one line each, split/merge calibration pointed at SLICE-IDENTIFICATION),
  When-to-Use 6 → 4, No-exceptions 5 bullets → 1 line (refusal clause kept), duplicate
  TEST-GRAMMAR pointer collapsed to one
- **Tier failed:** 1 (every stripped block had ≥1 verified home, most in the skill's own tree) ·
  n/a for the form-only densifications
- **Content:** the format block, numbering list, six mistake subsections, four snippet blocks,
  two ASCII diagrams, three case subsections; nothing was written to `templates/` this wave —
  dedups run against pre-existing template content, so D4's destination ban is not engaged
  (R4a credit)
- **Consumers assessed:** wave-open enumeration — 13 citing files; none links a section anchor
  of this SKILL; `task-architect`'s "cycle structure lives in patterns-vertical-tdd" claim still
  holds via CYCLE-STRUCTURE.md (skill tree). Shared-home audit: `references/TEST-GRAMMAR.md`
  (4 external consumers) is a clean single source — ownership header, CYCLE-STRUCTURE holds
  pointer-only, no dead pointers, untouched

## [v0.27.0] KEPT: Markers table (ownership ruling), rationalizations, checklist, mapping shape, epigraph
- **Tier-2 evidence:** the Markers table was contested (identical table in `tasks-template.md` —
  a legal R4a dedup) and **kept on an ownership ruling**: `task-architect` and
  `brownfield-integration` both name this skill as where the marker vocabulary is *defined*, and
  the skill tree's own CYCLE-STRUCTURE covers only `[EXTEND]`/`[MODIFY]` — stripping would leave
  the claimed owner defining 2 of 4 markers, inverting ownership toward a fill-target. Common
  Rationalizations (8 rows, excuse + reality each) kept as the discipline core — distinct from
  `executing-tdd-cycle`'s runtime red flags (checked: no cross-skill duplicate). Quality
  Checklist kept (aggregation function). The task-mapping.md shape block untouched — it IS the
  v0.23.0 canonical home (below). The letter/spirit epigraph kept as-is under R4b: its
  consequence anchor is the Overview's discipline paragraph eight lines down. Session ruling:
  wave-3 batch-1 ratified 2026-07-25.

## [v0.23.0] task-mapping.md gains its canonical compact shape (wave 2)
- **Disposition:** addition, not a strip — recorded here because it materially changes the authored artifact: `task-mapping.md` had **no prescribed structure anywhere** (the commands called it "the freehand story→cycle mapping"), and kinako's freehand form ran to 45.9k B for one slice
- **Content:** SKILL.md's Mapping-Stories-to-Cycles section now carries the canonical shape — the Story→Cycle table (Case column for Simple/Split/Merge with one-line why) + a per-cycle table (≤ 2-line rationale cells) + an optional Slicing-notes section (≤ 3 lines each, omit when empty), under the `artifact-format.md` envelope. `commands/tasks.md`'s two "freehand" mentions updated to "compact".
- **Consumers assessed:** tasks producer (task-architect — authors it) + review-task-artifacts (mapping checklist grades coverage/slice-quality, shape-agnostic; density note added this wave) + implement's producer (reads it as a design input — a table serves that read better than prose).

## [v0.22.0] TEST: grammar split out of CYCLE-STRUCTURE.md
- **Disposition:** relocated → `references/TEST-GRAMMAR.md` (new; ownership stays with this skill) — the runtime verifier (`testing-end-user`) now loads only the grammar (~5k B) instead of the full CYCLE-STRUCTURE.md (18.6k B)
- **Tier failed:** pure waste (D6a): the verifier's mandated cross-skill read pulled 13k B of cycle-structuring content it never uses
- **Content:** the `## Verification Task Requirements` block (what verification MUST include · Unified TEST: format + runtime-classification table · field definitions · action modifiers · assert patterns · four worked examples · bad-verification examples · why-this-matters · legacy format support) moved verbatim; CYCLE-STRUCTURE.md keeps a pointer section. Retargeted refs: testing-end-user SKILL.md (5) + TASK-PARSING.md (3), patterns-vertical-tdd SKILL.md (2), tasks-template.md (1).
