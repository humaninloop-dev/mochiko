# Strip notes — `skills/patterns-technical-decisions/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 3 (patterns-\* cluster,
batch-3 ratified 2026-07-25; design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`).
First strip assessment of this skill (never-stripped band 30–70): body 161 → 89 lines = **45%**,
in-band.

## [v0.64.0] Guardrails cut — When-to-Use removed (supersedes v0.27.0 KEPT); slim description

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 5,025 → 4,626 chars, −399, −8%; description 757 → 469 chars):**
  - **Removed whole:** `## When to Use` — the six-bullet list ("Choosing between technology options (libraries, frameworks, services)" · "When a decision is blocked on missing information and must be flagged **NEEDS CLARIFICATION**" · "Documenting architectural decisions for the team" · "When a technology choice needs a documented justification" · "Evaluating existing stack vs new dependencies" · "Any decision with long-term maintenance implications"). Restates the description; the NEEDS CLARIFICATION obligation survives in `### Phase 2: Decide`, brownfield/existing-stack in `## Brownfield Alignment`.
  - Old description verbatim: "This skill MUST be invoked when making and documenting a technology or architecture decision — evaluating two or more alternatives against weighted criteria, capturing the trade-offs and consequences of the choice, scoring brownfield alignment with the existing stack, and recording the rationale as a decision record (ADR). SHOULD also invoke when the work involves evaluating alternatives, weighing trade-offs and consequences, a decision record or ADR, decision rationale (\"why we chose\"), brownfield-alignment scoring, or marking a decision NEEDS CLARIFICATION. Owns the decision-making technique and ADR record depth; the decisions it produces are recorded in the constraints-and-decisions.md artifact owned by mochiko:authoring-technical-requirements."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** `## Overview`, `## When NOT to Use`, `## Decision Workflow` (the EVALUATE→DECIDE→DOCUMENT spine + the three phases + the NEEDS CLARIFICATION paragraph — v0.27.0-KEPT unique boundary + the EVALUATION-MATRIX / DECISION-RECORD pointers), `## Where decisions are recorded` + the ownership boundary table (v0.27.0-KEPT, SKILL-side statement fuller than DECISION-RECORD's), `## Brownfield Alignment`, `## Quality Checklist`. The description keeps the MUST trigger, the evaluate/decide/document gist, the top trigger phrases (incl. NEEDS CLARIFICATION), and the ownership boundary.
- **Protected-content reconciliation (MANDATORY):** the v0.27.0 strip listed "Kept: … Quality Checklist, **both When-to sections**." The `## When to Use` half of that KEPT pair is **superseded by this ruling** (recorded above; it restated the description, exactly the D4 cut target) — an explicit recorded supersession, not a silent drop. The other v0.27.0-KEPT elements (Decision Workflow spine, NEEDS CLARIFICATION paragraph, "Where decisions are recorded" ownership section + boundary table, Quality Checklist, and `## When NOT to Use`) all survive intact.
- **Consumers assessed:** `agents/system-architect.md`, `agents/technical-analyst.md`, `skills/authoring-technical-requirements/SKILL.md` (+ `references/ARTIFACT-TEMPLATES.md`, `references/TRACEABILITY-PATTERNS.md`), `skills/patterns-api-contracts/SKILL.md`, `skills/patterns-entity-modeling/SKILL.md`, `skills/patterns-system-design/SKILL.md`, `skills/review-brainstorm/references/EXTERNAL-CLAIMS.md`, router `skills/mochiko/SKILL.md` — all reference the skill by name; none links a removed section anchor. The technique↔artifact ownership boundary with `authoring-technical-requirements` is intact.

## [v0.46.0] loop-discipline pointer out
- **Disposition:** superseded → the sentence stands without the citation
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "...not to this skill (see `mochiko:loop-discipline`)." → "...not to this skill."
- **Consumers assessed:** plan command briefs unchanged.

## [v0.27.0] Quick-layer restatements and fully-homed Common Mistakes stripped (body 161 → 89, −45%)
- **Disposition:** relocated → verified reference homes, each Read before landing: the
  Quick-Criteria table → `references/EVALUATION-MATRIX.md` Evaluation Criteria (same 8
  criteria, richer — Description + Questions-to-Ask columns; the criterion names stay in-body
  as a one-line enumeration) · the Quick-Comparison table → the reference's two canonical
  shapes (weighted decision matrix + side-by-side options comparison) — the in-body table was
  a divergent *third* shape for the same function, and the pointer now says "don't invent a
  third format" (drift repaired) · the Quick-Decision-Record block →
  `references/DECISION-RECORD.md` Decision Record Format (fuller — adds Alternatives
  Considered + structured Consequences; the field enumeration stays in-body as one line) ·
  the Brownfield-Alignment table → EVALUATION-MATRIX Alignment Scoring (verbatim identical,
  all 4 rows; the check-existing-stack-first sentence stays with the pointer) · deleted
  (Tier 1): all seven Common-Mistakes rows, each homed — single-option-evaluation (Phase-1
  "2-3 alternatives minimum" + checklist), shiny-object (Alignment Scoring strong-justification
  row + Brownfield Questions "prefer extension over addition"), vague-rationale
  (DECISION-RECORD Good/Bad Rationale — near-verbatim JWT material), ignoring-team-skills
  (Team Familiarity criterion + the Criteria-Weights worked example), missing-trade-offs
  (checklist + Phase-2 + Consequence Documentation), orphan-decisions (checklist + Dependency
  Documentation, D2-depends-on-D1 example verbatim), governance-blindness (checklist +
  Governance Alignment section)
- **Tier failed:** 1 throughout — every stripped block restated a verified reference home or
  in-file statement; no Tier-2 deletions
- **Content:** four quick-layer blocks (criteria table, comparison table, mini-ADR, alignment
  table) and the seven mistake subsections; nothing written to `templates/` — D4's destination
  ban not engaged
- **Consumers assessed:** wave-open enumeration — 8 citing files, none references the stripped
  blocks or a section anchor. Kept: the Decision Workflow spine, the NEEDS CLARIFICATION
  paragraph (unique boundary with `mochiko:loop-discipline` — marking is this skill's job,
  driving resolution is the supervisor's), the "Where decisions are recorded" ownership section
  + boundary table (the SKILL-side statement is fuller than DECISION-RECORD's point-of-use
  paragraph — each sits at its own point of use, not duplication-only), Quality Checklist,
  both When-to sections. Reference audit: EVALUATION-MATRIX's category tables carry an
  explicit illustrative-only disclaimer; DECISION-RECORD's artifact-boundary paragraph is
  correctly scoped — both clean, untouched. Session ruling: wave-3 batch-3 ratified 2026-07-25.
