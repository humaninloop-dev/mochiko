# Strip notes — `agents/validator.md`

Entry formats: `strips/README.md`.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/validator.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **2 `<example>` blocks removed** from the `description:` value:
  1. Context: a finished constitution needs an independent quality grade before acceptance — commentary claimed the example demonstrated that independent grading of a finished artifact against an explicit checklist is the validator's core work.
  2. Context: an artifact is handed over with a checklist outside the validator's built-in domains — commentary claimed it demonstrated that the same skeptical, evidence-first grading applies; the artifact and checklist differ, the craft does not.

  Description parsed-value char delta: **1,448 → 268** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/validator.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/validator.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Skeptical, independent reviewer who grades a finished artifact against an explicit checklist … Defaults to FAIL. Never grades work it authored.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `validator`: `skills/*/SKILL.md` and `references/` (`ARTIFACT-CHECKLISTS.md`, `EXTERNAL-CLAIMS.md`, `INTERROGATION-AGENDA.md`, `QUALITY-CHECKLIST.md`, `backend-service.md`, `check-artifacts.py`); no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** the prior [v0.45.0] entry touches the `skills:` frontmatter list and a body "Skills you lean on" bullet (the `validation-command-shape` mount drop). It does not touch the frontmatter `description:` value or any `<example>` block. Its `Kept deliberately` set (the `validation-constitution` mount and the generic-grader method in the body) is untouched by this edit. No overlap.

## [v0.45.0] `validation-command-shape` mount dropped (fourth-consumer edit)
- **Disposition:** superseded → the generic-checklist fallback already in the persona body
  ("When it does not, do not force it: fall back on your own method and grade the artifact
  against the bar you were given")
- **Tier failed:** n/a — supersession by ruling (user ruling 2026-08-02, framework-trio
  deletion; ADR `.mochiko/decisions/2026-08-02-framework-trio-deleted.md`). This is the
  fourth-consumer edit pre-flagged in `strips/authoring-commands.md` [v0.44.0] DEFERRED —
  owed at the move, landed at the delete instead.
- **Content:** frontmatter `skills: validation-constitution, validation-command-shape` →
  drops the second name; the "Skills you lean on" bullet — *"**`mochiko:validation-command-shape`**:
  for grading an orchestration command's conformance to its codified shape — a deterministic
  grep floor (references present, no restated single-sourced prose, exceptions marked,
  frontmatter correct) run first and recorded as evidence, then the prose judgment ceiling
  (altitude, parameter completeness, contract-fill soundness), plus the strip-note audit when
  a minimalism wave is closing."* — deleted whole.
- **Kept deliberately:** the `validation-constitution` mount and the entire generic-grader
  method (Iron Law, checklist-fallback paragraph) — command audits now ride that fallback
  with `templates/command-shape.md` as the handed bar.
- **Consumers assessed:** router `validator` row (skills list edited in the same wave) ·
  setup command's validator seat unaffected (uses `validation-constitution`).
