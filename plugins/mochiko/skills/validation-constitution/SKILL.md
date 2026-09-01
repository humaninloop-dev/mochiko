---
name: validation-constitution
description: This skill MUST be invoked to grade a DRAFTED governance surface set against the quality checklist — there is NO constitution.md; the graded set is the CLAUDE.md governance region, the `.claude/rules/mochiko/` files, and the governance ledger. SHOULD also invoke for the setup loop's validate step, or when re-validating after a FAIL-loop revision. Validator-side skill of the governance producer↔validator pair; defaults to FAIL; run by an independent validator, never the author.
---

# Validating Constitution

Independent binary grade of a drafted governance surface set — enforceable, testable,
trace-closed, anti-pattern-free before finalization. There is no constitution.md: the
deliverable under grade is the surface set itself, and the grade is earned by walking the
assembled checklist against the files, never by trusting the author's account of them.
Producer side: `mochiko:authoring-constitution` (never co-mounted; the validator is a
different agent).

## Rules — load the schema first

Your first action at invoke, before any grading step: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
action.** The schema is the source of truth for this skill's binding rules; this body carries
identity and procedure only. Its rules are nested in six sections, each addressable by its
section ID: `validation-constitution.sec.independence` (author/grader separation) ·
`validation-constitution.sec.scope` (jurisdiction and the excess-governance line) ·
`validation-constitution.sec.inputs` (the from-file input set and checklist assembly) ·
`validation-constitution.sec.verdict` (the binary posture, the bump grammar, the grading
floors) · `validation-constitution.sec.output` (the VALIDATION RESULT contract) ·
`validation-constitution.sec.reserved` (the skip path only the user rules).

Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an absent
`kind:` reads `constraint`. Where a rule carries `extends: review-common.<slug>`, the stub
inherits `text` / `labels` / `pointer` only from `skill-review-common.yaml` — `class` and
`kind` are always this schema's own, and the stub's `validation-constitution.*` ID stays the
citable ID; `${verdict}` in inherited text substitutes from this schema's `vars:`. Labels
come from `../../schemas/skill-labels.yaml`. A `pointer:` rule binds you to that file's
content, referenced never restated.

The schema carries **the 14 rules of `class: floor`**. State the floor count back before the
first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
continue.

## Procedure

Read [references/QUALITY-CHECKLIST.md](references/QUALITY-CHECKLIST.md) and assemble the
working checklist from the universal core plus each selected module's embedded fragment,
routed to where that module's content actually lives. Then walk it: every item against the
files, vague language against the patterns of
[references/ANTI-PATTERNS.md](references/ANTI-PATTERNS.md), excess governance hunted with the
same seriousness as missing governance, and a version-bump determination for every change.
Close by emitting the VALIDATION RESULT block in full — the verdict, the accounting, the
issues, and the advisory line.
