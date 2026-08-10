# Strip notes — `skills/review-feasibility`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 2 (review-\* cluster,
batch-2 ratified 2026-07-25; design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`).
First strip assessment of this skill (never-stripped band 30–70): body 141 → 136 lines = **3.5%**,
deeply under-band — whole-skill survivor ruling below.

## [v0.64.0] Guardrails Wave 2 — body deletions (When to Use, Steps 1 & 6) + slim description + review-evidence floor line
- **Disposition:** superseded → the guardrails-vs-detail Wave 2 editorial cut (D4 cut line): the "When to Use" list and two procedure steps whose obligations already live in the tables/floors/Common-Mistakes are deleted; description slimmed; one sanctioned floor line added.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row [its Wave 2 residual authorization] + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — guardrails held across all four skill natures).
- **Content (faithfully compressed).** Description 1,513 → 500 chars (−67%). Body 16,875 → 15,246 chars (−1,629, −10%; net of the +~176-char floor-line pure addition). Sections removed:
  - **## When to Use** (four bullets) — restated the description's invocation conditions (analysis-set review · design-set review · re-review after structural revision · architecture-pass trigger). All survive in the description and the architecture-pass section.
  - **### Step 1: Gather the cross-artifact context** — the read-the-actual-artifacts-never-a-summary obligation survives in Common Mistakes ("Grading from a summary or the producer's claim → Read the artifacts themselves") and Common Rationalizations ("I'll trust the analyst's report summary").
  - **### Step 6: Emit the review** — the feasibility-report template binding survives in `## Related` ("the **feasibility-report** … its markdown shape is owned by that template"); the do-not-invent-routing obligation survives in Common Mistakes ("Restating the loop / round cap / human-gate mechanics here") and `## Independence`; the write-verdict-into-the-report obligation is restored (stronger) by the added floor line below.
  - **Old description (verbatim):** "This skill MUST be invoked to grade plan analysis and design artifacts for cross-artifact FEASIBILITY — adversarially hunting contradictions, impossibilities, and buildability conflicts that no single artifact reveals in isolation: constraint-decision conflicts, NFR-constraint impossibilities, requirement-constraint contradictions, decision-decision conflicts, NFR-design feasibility, and constraint-design buildability — plus, when `architecture.md` is in scope, the architecture pass: topology feasibility (NFR↔topology, constraint↔topology) and governance conformance (layer rules, dependency allowlist, GI-linked principles) routed to amendment/waiver, never silently passed — emitting a 3-state `feasible / needs-revision / infeasible` verdict with per-issue evidence, impact, and suggested resolution. SHOULD also invoke whenever a producer's analysis or design artifacts (requirements, constraints-and-decisions, NFRs, architecture, data-model, contracts) need an independent buildability review, or when re-reviewing after a structural revision (new or changed constraints, expanded requirement scope, modified NFR targets). The feasibility reviewer's driver — the adversarial-critique half of the cross-artifact review pair: its sibling grades coverage / measurability / consistency / presence, this skill grades contradiction / impossibility / buildability. Never defaults to `feasible`; grades a different agent's artifacts, never the author's own; operates over plan artifacts, NOT the constitution."
  - Verbatim homes for the removed body + description text: git history of this SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Floor line added (pure addition, cross-cutting finding 1 / F-X1 mitigation):** "The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation." Placed in `## Independence (stated by role)`, as a new bullet after the "grade artifacts authored by a different agent … never review your own authoring" bullet. Rides the same decision row. Note: this restores and strengthens the deleted Step 6's "write the verdict … into the feasibility report" obligation.
- **Kept deliberately (the guardrails keep-set):** the six-class hunt table; the External-premises paragraph and its `EXTERNAL-CLAIMS.md` pointer; the entire **architecture pass** (topology-feasibility table + governance-conformance list + the never-silent-approval routing) — AD-D7 ruled content, protected; the boundary table + the architecture-boundary paragraph; Core Process **Steps 2 (Hunt each class), 3 (gate fuel — the cite-the-IDs rule and the resolution taxonomy kept inline per the v0.26.0 strip), 4 (resolvable-vs-fundamental classification), 5 (the 3-state verdict table + the `infeasible`-never-flattened and never-default-`feasible` floors)**; Common Mistakes; Red Flags; Common Rationalizations; Related.
- **MANDATORY KEPT reconciliation:**
  - **[v0.26.0] KEPT: the entire remaining body (whole-skill survivor ruling).** This cut removes three members of that KEPT body — When to Use, Step 1, Step 6 — recorded here as superseded-by-this-ruling. The v0.26.0 evidence for the KEEP named "Common Mistakes and Rationalizations are already tables … the six-class table and the boundary table are the skill's core unique content … the `infeasible`-never-flattened and never-default-`feasible` paragraphs"; every one of those named survivors is untouched. The three removed sections were procedure/when-to-use, not among the v0.26.0 evidence's named cores, and each removed obligation survives in a kept table/floor (enumerated above).
  - The AD-D7 architecture pass (DECISIONS.md 2026-07-30 architecture-design-primitive row) is RULED, protected content — verified fully present after the edit.
  - The Red Flags line "Stop and restart from Step 2" still resolves — Step 2 is retained.
- **Consumers assessed:** commands — `plugins/mochiko/commands/` grep clean (plan orchestrates by dispatch). Agents — `plugins/mochiko/agents/principal-architect.md` declares the feasibility review in its persona; the kept six-class table, architecture pass, verdict table, and boundary leave that composition intact. `review-plan-artifacts` (the sibling) references this skill by name in its boundary — unchanged. Contract intact.

## [v0.46.0] loop-discipline pointers + gap-routing taxonomy out
- **Disposition:** superseded → routing is the lead's judgment (the knowledge/preference/scope taxonomy dropped entirely, per the purge ruling — no new home)
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "routing per `loop-discipline`" → "routing"; "Loop ownership, the round bound, and the human gate are the lead's — see `loop-discipline`" → "— its command states them"; "Route findings by their kind per `loop-discipline`'s gap routing — a fundamental conflict is a preference/scope gap for the human, not a knowledge gap" → "Routing each finding is the lead's judgment — a fundamental conflict is the human's to rule, never something investigation can settle"; the Common-Mistakes row's "governed by `loop-discipline`" → "its command states them"; the Related bullet deleted.
- **Kept deliberately:** the two-exit governance routing, the per-issue gate fuel, and the never-clear-it-yourself boundary — unchanged.
- **Consumers assessed:** plan command briefs unchanged.

## [v0.26.0] Step-3 field-gloss table → prose with canonical field names (−5 lines; drift repaired)
- **Disposition:** deduped → `templates/feasibility-report-template.md` (its **pre-existing**
  Usage Notes define all four gate-fuel fields with the same glosses — Read and confirmed before
  landing; nothing was written to templates/ this wave, so D4's destination ban is not engaged —
  R4a dedup credit, not a relocation); the
  skill's unique adds kept inline: the cite-the-IDs rule and the resolution taxonomy. The strip
  also **repaired a naming drift**: Step 3 used prose names (description / evidence /
  suggested_resolution) while the template — and this skill's own Independence section — use
  `gap` / `at` / `impact` / `fix`; the compressed prose now uses the canonical names
- **Tier failed:** 1 (the field glosses restated the template's Usage-Note definitions under
  divergent names — the second-home drift symptom)
- **Content:** the 4-row Field / What-it-states table
- **Consumers assessed:** wave-open enumeration — 7 citing files, none reference the table

## [v0.26.0] KEPT: the entire remaining body (whole-skill survivor ruling, 3.5% vs 30–70)
- **Tier-2 evidence:** contested as a whole at the under-band pass and kept — authored
  post-doctrine and at altitude throughout: Common Mistakes and Rationalizations are already
  tables; every Overview paragraph names a distinct failure mode (intersection-hunting,
  looking-buildable ≠ being-buildable, adversarial-not-checklist); the six-class table and the
  boundary table are the skill's core unique content (the boundary table is this skill's own
  side of the seam contract — each end of a boundary states its contract, the R4b
  one-line-per-mount logic, not duplication of `review-plan-artifacts`' Scope table); the
  `infeasible`-never-flattened and never-default-`feasible` paragraphs each name the failure
  they block. Fourth whole-skill survivor of the pass (after testing-governance-injection,
  validation-command-shape, loop-discipline). Session ruling: batch-2 ratification 2026-07-25.
