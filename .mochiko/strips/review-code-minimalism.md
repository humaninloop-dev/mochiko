# Strip notes — `skills/review-code-minimalism`

Entry formats: `strips/README.md`. First strip for this skill. Wave context: guardrails-vs-detail
Wave 2 (editorial extension of the D4 cut line to the untested primitives). This skill was authored
recently and terse (the Ponytail code-minimalism build, PT-D1–D10); the honest D4 yield is small —
its body is almost entirely non-waivable floors + output contract + the `patterns-code-minimalism`
standard binding, none of which the cut line removes.

## [v0.91.0] When-NOT-to-Use carve-out re-keyed: "plan/spec reviewers", "the plan-time sibling" → design — plan-stage retirement D1/D5

- **Disposition:** superseded → "the design-phase and spec reviewers own those surfaces; the
  design-time sibling of this lens…".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 and D5; wording ruled by the wave
  lead 2026-08-26).
- **Content (superseded text, verbatim):**

  ```
  - **Design-time artifact review** — plan/spec reviewers own those surfaces; the plan-time sibling of this lens is the rung-honesty grade in `mochiko:review-plan-artifacts` against `mochiko:patterns-plan-minimalism` (same posture, design-time altitude)
  ```

- **Kept deliberately:** the carve-out's whole point — this lens does **not** review design-time
  artifacts, those surfaces belong to their own reviewers, and the sibling relationship runs to
  the rung-honesty grade in `mochiko:review-plan-artifacts`. Both skill slugs are unchanged, so
  both pointers still resolve. Note the bullet's own label was already "**Design-time artifact
  review**" and its closing parenthetical already read "same posture, design-time altitude" — the
  ruled wording brings the body of the bullet into line with a heading that was correct all along.
- **Budget:** body 3,866 → **3,884** against the 4,612 budget; description unchanged at 492
  against 615. Both inside.
- **Consumers assessed:** `mochiko:patterns-code-minimalism` carried the parallel sibling line,
  re-keyed identically in the same wave; `mochiko:review-plan-artifacts` was re-scoped to the
  design-phase package this wave, so the pointer's target now matches its description.

## [v0.64.0] Guardrails Wave 2 — slim description + "When to Use" deleted (floor line skipped — equivalent present)
- **Disposition:** superseded → the guardrails-vs-detail Wave 2 editorial cut (D4 cut line): the "When to Use" list restating the description is deleted; description slimmed. No other body change.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row [its Wave 2 residual authorization] + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — guardrails held across all four skill natures).
- **Content (faithfully compressed).** Description 1,102 → 492 chars (−55%). Body 3,879 → 3,689 chars (−190, −5%; no additions). Section removed:
  - **## When to Use** (two bullets — "Grading a cycle's diff + disclosed decomposition during per-cycle verification" · "Auditing a rung claim … against the actual codebase") — restated the description's MUST trigger and the Procedure's rung-2/3/5 codebase-read obligation. Both survive in the slim description and in Procedure steps 2–3.
  - Description cut: the rung glosses ("2 (reuse), 3 (stdlib), 5 (installed dependency)"), the explicit grep/manifest mechanics, the "over-engineering in this diff / should this have been reused / rung claim / code-shape audit / minimalism findings" SHOULD-trigger phrases, and the mounted-on-the-verification-seat-never-the-producer clause compressed. Kept in the slim description: the MUST trigger, the pre-code-ladder + code-minimalism-lens + verification-seat framing, read-diff-AND-`cycle-report.md`, grade-against-`mochiko:patterns-code-minimalism`-the-standard-never-restated-here, the rungs-2/3/5 codebase-read obligation, the ADVISORY-never-a-cycle-failing-gate posture, and the minimalism-lens-ONLY scope.
  - **Old description (verbatim):** "This skill MUST be invoked when independently grading a cycle's produced code against the pre-code ladder — the per-cycle code-minimalism lens run by the verification seat during the verification it already performs: read the cycle's git diff AND cycle-report.md (the disclosed decomposition and its rung claims), grade each claim against mochiko:patterns-code-minimalism (the standard — never restated here), and emit a minimalism: findings block in the verification report. Rungs 2 (reuse), 3 (stdlib), and 5 (installed dependency) carry a codebase-read obligation — targeted greps for existing helpers and a dependency-manifest check, never diff + disclosure alone. SHOULD also invoke when the work involves \"over-engineering in this diff\", \"should this have been reused\", \"rung claim\", \"code-shape audit\", or \"minimalism findings\". Findings are ADVISORY to the lead's checkpoint verdict — never a cycle-failing gate. Mounted on the verification seat only, never on the producer. Scope is the minimalism lens ONLY: general code review (naming, patterns, correctness beyond tests) stays out of scope."
  - Verbatim homes for the removed text: git history of this SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Floor line — SKIPPED (equivalent already present).** The Wave 2 review-evidence floor line ("The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation") was NOT added: this skill produces no verdict (its findings are advisory) and already carries the equivalent persistence obligation — Procedure step 5 ("One `minimalism:` entry per finding in the verification report … never narrate a clean grade") plus the Quality Checklist item "Findings emitted as advisory `minimalism:` entries". Review evidence living only in conversation is already precluded. Per the Wave 2 instruction, the skip is recorded here.
- **Kept deliberately (the guardrails keep-set):** the epigraph floor ("Diff shows what was written … neither alone shows what should never have been written"); When NOT to Use (scope boundary — general code review out · TEST/quality gates to `testing-end-user` · design-time to plan reviewers); Inputs (the grading substrate contract, incl. "read the report file itself, never a relay"); the full Procedure (steps 1–5 — the missing-rung-note-is-a-finding rule, the grade-against-the-standard binding, the **rung-2/3/5 codebase-read obligation** [the skill's central non-waivable floor], the floor-line/accessibility check, the `minimalism:` output contract); Verdict semantics — advisory; Quality Checklist.
- **MANDATORY KEPT reconciliation:** no prior strip file existed and no `KEPT:` / protected / `DECISIONS.md`-traceable line is in scope of this cut. The Ponytail-ruled codebase-read obligation and advisory-only posture (PT-D1–D10) are the skill's core and are fully retained.
- **Consumers assessed:** command — `plugins/mochiko/commands/implement.md` references it (the implement command dispatches the verification seat's minimalism lens); the retained MUST trigger, codebase-read obligation, and advisory posture leave that dispatch intact. Agent — `plugins/mochiko/agents/qa-engineer.md` mounts it (the verification seat); the kept Procedure, Inputs, and Verdict-semantics leave that composition intact. Contract intact.
