# Strip notes — `skills/analysis-iterative/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 1 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
228 → 192 lines, 36 cut = 16% — **under the 30–70 never-stripped band**; per R3 the under-band
second pass generates the survivor-provenance (KEPT) entries below rather than forcing cuts.

<!-- Wave context: wave 1 of the brainstorm target-state build (v0.117.0). Ruling: `.mochiko/brainstorms/brainstorm-target-state/record.md`
(`DECISIONS.md` 2026-10-06 row) — D21 for the two repairs; D19 as changed at review (S5) and at
verify (V4) for the scoped supersession. The skill's behaviour for setup and specify is unchanged.
Pre-edit verbatim text: `git show 2e4c57c:plugins/mochiko/skills/analysis-iterative/SKILL.md`. -->

## [v0.117.0] `## Output` — the synthesis line re-keyed to defer to the calling command's deliverable

- **Disposition:** superseded → "Conclude with the deliverable your calling command names; when it
  names none, generate the synthesis document using [SYNTHESIS.md](SYNTHESIS.md)."
- **Tier failed:** n/a — supersession by ruling (`brainstorm-target-state` D21)
- **Content:** verbatim — "Generate the synthesis document using [SYNTHESIS.md](SYNTHESIS.md)."
- **Kept deliberately:** the [v0.63.0] entry's kept-deliberately line protects "Output's
  synthesis-doc contract" (the goal/output contract, with the Overview). By this ruling it is
  narrowed, not dropped: the contract stands for any caller that names no deliverable, so a direct
  think-through keeps today's synthesis, and setup's named deliverable is itself a synthesis
  (`governance-intent.md`). "Synthesis" is not swapped for "record" (D21's rationale). The Overview
  is untouched. The rest of `## Output` stays byte for byte: the floor line and the
  confidence-indicators table.
- **Consumers assessed (shared skill):** `setup` — `setup.interrogation-inline` points here, and
  `setup.synthesis-artifact` names the deliverable (`governance-intent.md` from the
  `governance-intent` template), so setup concludes with the file its own rule names, as before ·
  `specify` — `spec.intent-stage-first` points here; its Goal names the confirmed Intent section ·
  `brainstorm` — `brainstorm.deliverable-record` names `record.md` and
  `brainstorm.synthesis-on-request` keeps a synthesis on request: the fault D21 repairs · the
  router (`SKILL.md:66`, `:124`), `review-specifications` (`SKILL.md:3`), `INTERROGATION-AGENDA.md:5`,
  `:166` and `ARCHITECTURE.md:367` name the skill and quote neither sentence (grep "synthesis
  document" over `plugins/mochiko`, `README.md`, `ARCHITECTURE.md`, `.mochiko/schema-views`).

## [v0.117.0] `## Two output shapes, one engine` — the dangling "adaptive flow above" pointer re-keyed

- **Disposition:** superseded → "The adaptive questioning the Overview describes is the single
  questioning engine."
- **Tier failed:** n/a — supersession by ruling (`brainstorm-target-state` D21)
- **Content:** verbatim — "The adaptive flow above is the single questioning engine." It pointed
  at `## Adaptive Flow`, removed whole at [v0.63.0].
- **Kept deliberately:** the rest of the section byte for byte — one engine, two output shapes,
  both subsections. This is the caller-facing dispatch seam the [v0.63.0] entry's
  kept-deliberately line protects, and only its dangling pointer is re-keyed.
- **Consumers assessed:** no consumer quotes the sentence (grep "adaptive flow" over the same
  scope: this line only).

## [v0.117.0] `## Common Mistakes` — the rows kept at v0.63.0 superseded for `/mochiko:brainstorm` only

- **Disposition:** superseded for `/mochiko:brainstorm` only → `brainstorm.own-rules-win`
  (`0044-brainstorm-target-state.yaml`, `brainstorm.sec.roles`): where this skill and brainstorm's
  own rules differ, brainstorm's rules win. No text leaves this file.
- **Tier failed:** n/a — supersession by ruling (`brainstorm-target-state` D19 as changed at review,
  S5, and at verify, V4; `DECISIONS.md` 2026-10-06 row)
- **Content (the kept table, verbatim):**

  ```
  | Always using structured options | Options force premature decisions on an unsure user | Open probes help them discover what they think |
  | Ignoring "unsure" signals | Uncertainty read as indifference → recommendations passively accepted, never owned | Probe deeper from a different angle |
  | Multiple questions per turn | Fractured attention, shallow answers across all of them | One question per turn — always |
  | Questions that don't connect | Topic-jumping breaks collaborative momentum | Show how the previous answer shapes the direction |
  | Premature synthesis | Forced wrap-up leaves core trade-offs unaddressed | Nudge, never force; the user has final say |
  | Rigid opening script | A recited introduction ignores the topic's tone and energy | Adapt the opening to both |
  | Padding the synthesis | Filler sections dilute a lean conversation's record | Output depth matches conversation depth |
  ```

  The differences the record names (D19, S5): "One question per turn — always", where D6 and D17
  batch; and always-structured options listed as a mistake, where D13 fixes the form when options
  are put (an open question to an unsure user stays allowed).
- **Kept deliberately:** every row byte for byte, for setup and every other caller — the [v0.63.0]
  entry's kept-deliberately anti-patterns stand for them. The v0.63.0 cut is not reversed (D19),
  and its finding that detail buys weight, not honesty, binds the form of brainstorm's new rules.
  Accepted by the ruling: two texts that disagree stay in the plugin.
- **Consumers assessed:** `setup` (`setup.interrogation-inline`) and `specify`
  (`spec.intent-stage-first`) read the rows as before; `brainstorm` reads them under
  `brainstorm.own-rules-win`. No other primitive quotes a row.

## [v0.63.0] Guardrails cut — procedure walkthroughs removed, guardrails kept; slim description

- **Disposition:** superseded → the benchmark-ruled guardrails body + slim description (`.mochiko/benchmarks/guardrails-vs-detail/variants/body/analysis-iterative/`, `.../descriptions/analysis-iterative/`). Shipped as one merged edit: slim frontmatter + guardrails body.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict; `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`)
- **Content (faithfully compressed — section-level inventory of the body cut, 11,438 → 3,942 chars, −66%):**
  - **Removed whole:** `## When to Use` (trigger list — carried by the description) · `## Adaptive Flow` (the Opening → Discovery → Adaptive-Questioning → Conclusion phase walkthrough) · `## Discovery` (discovery-phase procedure) · `## Question Format Adaptation` and its four format subsections `### Structured Options + Recommendation`, `### Open-Ended Probes`, `### Confirmations`, `### Recommend-then-Arbitrate` (the four question-format code-block templates) · `## Reading Confidence Signals` (the per-turn signal-recalibration table plus the asymmetry principle) · `## Smart Wrap-up` (the ratification-streak doctrine and wrap-up procedure).
  - **Shortened:** `## Output` (787 → 525 chars — the synthesis-doc pointer and the confidence-indicators table kept; extra prose trimmed).
  - **Kept intact:** `# Iterative Analysis`, `## Overview`, `## When NOT to Use`, `## Common Mistakes`, `## Two output shapes, one engine` with `### General analysis` and `### Specification-input enrichment`, `## Reference`.
  - Old description (new slim form is 476 chars; **old verbatim, 808 chars**): "This skill MUST be invoked when enriching a sparse feature description into the Who / Problem / Value triad (plus explicit out-of-scope and success criteria) before a specification is authored, or when running a collaborative think-through / brainstorm that explores a problem through adaptive, one-question-at-a-time discovery and ends in a structured synthesis. SHOULD also invoke when a feature description lacks Who/Problem/Value clarity and needs conditioning before it reaches a requirements producer, or when deliberately working through a complex, multi-decision trade-off space. A general / shared skill — useful across clusters, not tied to a single workflow. Distinct from review-specifications, which reviews an already-drafted spec for gaps: this skill conditions raw input BEFORE a spec exists."
  - Verbatim removed text survives in three homes: git history of `plugins/mochiko/skills/analysis-iterative/SKILL.md`; the before/after pair in `variants/descriptions/analysis-iterative/` (before-state) and `variants/body/analysis-iterative/` (after-state); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** the goal/output contract (Overview + Output's synthesis-doc contract), the non-waivable floor (the floor line added below), the anti-patterns (`## Common Mistakes`, `## When NOT to Use`), the caller-facing dispatch seam (`## Two output shapes, one engine`), and the `references/ADAPTIVE-EXAMPLES.md` pointer. *(Touched by ruling at [v0.117.0], `brainstorm-target-state` — see those entries: Output's synthesis-doc contract now defers to the calling command's deliverable, the synthesis kept for a caller that names none (D21); the Common Mistakes rows are superseded for `/mochiko:brainstorm` only and stand for every other caller (D19); the dispatch seam's dangling "adaptive flow above" pointer is re-keyed (D21).)*
- **Floor line added (pure addition, rides this ruling):** in `## Output` — "**Floor (non-waivable)** — surface every elicited unknown as an open question in the produced artifacts; a vague zone the principal could not resolve is never silently omitted." (guardrails-vs-detail cross-cutting finding 1; build-plan §4.4). Final shipped body is 4,120 chars, within the 4,928 budget.
- **Protected-content reconciliation (MANDATORY):** the two prior `[v0.25.0] KEPT:` survivor entries reconciled against this cut —
  - **`[v0.25.0] KEPT: the four question-format templates + ratification-streak doctrine`** — **superseded by this ruling.** The four format code blocks (`### Structured Options + Recommendation`, `### Open-Ended Probes`, `### Confirmations`, `### Recommend-then-Arbitrate`) and the ratification-streak doctrine (in the removed `## Smart Wrap-up`) are removed by the guardrails cut. Their dogfood-backed intent (adapt question format to the user's state; flag a yes-streak) is now carried as guardrails only — the "always structured options" and "ignoring unsure signals" rows of `## Common Mistakes`. The verbatim templates survive in the three homes above.
  - **`[v0.25.0] KEPT: confidence-signals table, asymmetry principle, two-output-shapes dispatch`** — **partially superseded.** The per-turn confidence-signals recalibration table and the asymmetry principle (both in the removed `## Reading Confidence Signals`) are superseded by this ruling; verbatim in the three homes. The **two-output-shapes dispatch survives intact** (`## Two output shapes, one engine`) — the caller-facing enrichment-vs-general-analysis seam is kept. (Note: the Output section's *synthesis-doc* confidence-indicators table — Confident/Assumed/Contested/Unsure/Deferred — is a different table and is kept.)
- **Consumers assessed (shared skill — 3 consuming commands):** `commands/setup.md`, `commands/specify.md`, `commands/brainstorm.md`. Each dispatches the skill by name and relies on its behavioral output (adaptive questioning → structured synthesis or enrichment artifact); none quotes a removed section anchor. The kept Overview, Output contract, and two-output-shapes dispatch preserve the invocation contract each relies on. Contracts intact.


## [v0.25.0] Adaptive Flow ASCII diagram (9 lines) → one-line flow
- **Disposition:** deleted (the one-line "Opening → Discovery → Adaptive Questioning → Conclusion" retained; the four labeled phase paragraphs directly below carry every behavior the box encoded)
- **Tier failed:** 2 (no behavior named beyond the surviving paragraphs)
- **Content:** the four-column box diagram with per-phase annotations
- **Consumers assessed:** 6 consumer files checked, none reference the diagram

## [v0.25.0] Three inline example blocks (15 lines)
- **Disposition:** relocated → `references/ADAPTIVE-EXAMPLES.md` (the skill's declared example home; the ops-team crisp-answer and API-versioning format-comparison conversations exist there in expanded, annotated form — verified before landing)
- **Tier failed:** 1 (compressed copies of the reference's annotated conversations)
- **Content:** crisp-answer branch, unsure-answer branch, same-topic-three-formats examples
- **Consumers assessed:** none reference the inline examples

## [v0.25.0] Common Mistakes densified: 7 subsections → 7-row table (net −12 lines)
- **Disposition:** compressed in place (densification, zero deletions — every mistake/failure/fix survives as a row)
- **Tier failed:** n/a — no content left the skill; form only
- **Content:** the seven mistake subsections (structured-options overuse, ignored unsure signals, multiple questions, disconnected questions, premature synthesis, rigid opening, padded synthesis)
- **Consumers assessed:** none reference the subsection headings

## [v0.25.0] KEPT: the four question-format templates + ratification-streak doctrine
- **Tier-2 evidence:** contested at the under-band pass and kept — the format code blocks ARE the
  skill's operative procedure (structured options / open probe / confirmation /
  recommend-then-arbitrate), and the streak doctrine + recommend-then-arbitrate format carry
  dogfood evidence: validated across four design sessions including the skill-succinctness
  session itself (streak flagged after 4× "yes a"; user re-engaged with genuine reads). Session
  ruling: batch-1 ratification 2026-07-25.

## [v0.25.0] KEPT: confidence-signals table, asymmetry principle, two-output-shapes dispatch
- **Tier-2 evidence:** contested at the under-band pass and kept — the signals table is the
  per-turn recalibration procedure; the asymmetry principle names the failure direction
  (early wrap-up costs rework, an extra question costs one turn); the two-shapes section is the
  live dispatch seam callers rely on (enrichment vs general analysis). Session ruling: batch-1
  ratification 2026-07-25.
