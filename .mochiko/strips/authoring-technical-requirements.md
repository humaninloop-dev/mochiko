# Strip notes — `skills/authoring-technical-requirements/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T2;
ratified 2026-07-24).

## [v0.81.0] `nfrs.md` dies as a file; the Structural Decisions subsection dies whole — product-architecture-schema D12

- **Disposition:** superseded → the architecture store. Two independent absorbs land in one edit:
  (a) **NFR-XXX** loses its `nfrs.md` document home and becomes fields on the store's concern
  rows — the **ids and the grammar stay this skill's**, the row shape is the store schema's;
  (b) the **Structural Decisions** subsection dies whole — structural-origin D-XXX are replaced by
  the store delta, whose ruling **is** the decision record, so the `Origin` axis has nothing left
  to distinguish and goes with it.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12, `Contested` — user ruled
  absorb against the lead's coexist recommendation, blast radius priced and accepted;
  `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — SKILL.md):**

  ```
  **Structural Decisions section (architecture-time D-XXX rows).** `constraints-and-decisions.md`
  carries a designated **Structural Decisions** subsection inside Section 2, grouping the D-XXX rows
  that record *topology* choices — component boundaries, interaction style, responsibility placement —
  decided during the architecture stage. These rows are authored by the **architecture seat**
  (`mochiko:patterns-system-design`), **not** this skill's analysis-time author, who keeps its own
  technology-decision D-XXX rows and **preserves** this section rather than filling it. Both origins
  share one D-XXX field schema, the same ADR discipline (`mochiko:patterns-technical-decisions`), and
  one continuous D-XXX sequence; the architecture delta summary links each structural change to its row
  here. (Template + the `Origin` marker: ARTIFACT-TEMPLATES.md.)
  ```

  ```
  ### 3. Non-Functional Requirements (nfrs.md) -- NFR-XXX

  Define measurable quality attributes. Every NFR has a numeric target. Field schema in ARTIFACT-TEMPLATES.md.
  ```

  ```
  - [ ] Architecture-time topology decisions live as D-XXX rows in the **Structural Decisions** section (authored by the architecture seat; the analysis-time author preserves it, never fills it)
  ```

  Plus the Overview's "three traceable analysis artifacts" line and the layer-enumeration line
  naming `nfrs.md (NFR-XXX)`.
- **Content (superseded, verbatim — references/ARTIFACT-TEMPLATES.md):** the whole
  `### Structural Decisions` block (its explanatory blockquote + the `### D-004: [Structural
  Decision Title]` record template with Context / Options / Choice / Consequences / Governance
  alignment); the `Origin` column and its `| D-004 | Avatar processing placement | async worker
  off a queue | NFR-002 | structural |` example row in the Decision Summary index; the `Origin`
  field-definition row and the "one shared sequence across both origins" clause on the `ID` row;
  and the `nfrs.md` **document template** (the `# Non-Functional Requirements: {feature_id}`
  header, the NFR Summary ID-index table, and the `## NFR-001` record skeleton).
- **Kept deliberately:** the **entire NFR grammar** — the field definitions table (ID · Title ·
  Category · Source · Requirement · Target · Measured · Applies to), the six categories with
  their bad-vs-measurable examples, the "Writing Measurement Methods" guidance and its worked
  example (reframed from a document section to a row's fields), and the `"Fast" is not a
  requirement` floor with its no-exceptions clause. **Trace chains preserved**: `TR-XXX →
  NFR-XXX` resolves unchanged, `Applies to:` still cites TR-XXX — D12 moves the path, never the
  ids. **C-XXX hard constraints and IP-XXX rows stay** in `constraints-and-decisions.md` (D12
  reduces that artifact, it does not kill it), as does the whole technology-decision D-XXX record
  format, the ADR-technique pointer to `mochiko:patterns-technical-decisions`, and the C↔D / IP
  traceability rules. `quickstart.md` is untouched — user-ruled keep.
- **Replacement guidance added (rides the decision row):** a short "structural decisions are not
  this artifact's" paragraph replacing the deleted subsection, naming where topology rulings go
  and how to handle a technology decision entangled with one (record the technology decision
  here, let the delta carry the shape, cross-cite by ID) — the deletion would otherwise leave the
  author with no route for the entangled case.
- **Consumers assessed:** `mochiko:review-plan-artifacts` graded `nfrs.md` with a standalone
  checklist and graded the structural-D-XXX links in its Architecture checklist — both re-keyed
  in the same edit set by this seat. `mochiko:review-feasibility`'s classes 1–5 cite C-XXX /
  D-XXX / NFR-XXX by id only, and stay correct (its architecture-pass seam re-keyed by this
  seat). `mochiko:testing-gap-finding` runtime-NFR probes re-pointed by this seat.
  `mochiko:patterns-technical-decisions` owns the decision *technique* and names no origin axis —
  verified unedited. `mochiko:patterns-system-design` (P3) no longer authors rows into this
  artifact. `plan.md` / `implement.md` baseline lists and the `feature-sizing` D9/D15
  baseline-set clauses are P2's this wave.
- **Consumers assessed — second pass (V4 delta, B2 + B2-extension).** The first pass swept skills
  and the router but missed the artifact-chain carriers that name this skill's outputs by
  filename. Five further `nfrs.md` sites re-keyed, all listing-class (an enumeration of produced
  or reviewed artifacts, no behavior stated):
  `templates/artifact-format.md:5` (the deliverable envelope's pipeline artifact chain) ·
  `templates/feasibility-report-template.md:15` (also logged under `strips/review-feasibility.md`) ·
  `templates/techanalyst-report-template.md:16` (`produced:` example, gains "+ store-delta NFR rows
  when touched") · `agents/technical-analyst.md:23` (the skill's own pointer, which now states the
  store home explicitly rather than merely dropping the filename) ·
  `schemas/tasks.yaml:23,103` (the generated-from provenance pointer, stated twice — the `check:`
  string and the rendered example; logged in `strips/tasks-template.md`, that schema's strip
  lineage). Router `technical-analyst` row re-keyed the same pass (drops the NFRs limb, "approved
  architecture" → signed store delta). **Post-fix sweep:** no `nfrs.md` reference survives in
  `plugins/` except deliberate negative statements ("there is no `nfrs.md`") and the D16
  migration-source lists in `commands/architecture.md` and `authoring-architecture-store`, which
  must name the absorbed file to reconstruct from it.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 11,255 → 10,628 chars (−6%); description 1,001 → 496
  chars (−50%). Body cut: the **When to Use** section deleted whole (six bullets restating the
  description's invocation conditions — TR-XXX / C-XXX / NFR-XXX / IP-XXX authoring plus the
  INT-XXX and DS-XXX analysis-declaration bullets; each obligation survives in §4 System
  Integrations and §5 Data Sensitivity respectively, and in the three-artifact spine). Description
  cut: the trigger-phrase enumeration trimmed and the trailing "Produces requirements.md,
  constraints-and-decisions.md, and nfrs.md from a business specification" sentence dropped; the
  MUST clause, core triggers, and the constraints-and-decisions.md-ownership +
  `patterns-technical-decisions` sibling distinction kept. Verbatim homes: git history of this
  file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when authoring the technical-requirements layer of a feature specification — decomposing business functional requirements into technical requirements (TR-XXX), documenting hard constraints (C-XXX), defining measurable non-functional requirements (NFR-XXX) with numeric targets, and specifying infrastructure-provisioning requirements (IP-XXX), each traced to a business source. SHOULD also invoke when the work involves "TR-", "C-", "NFR-", "IP-", "technical requirements", "hard constraints", "non-functional requirements", "infrastructure provisioning", or authoring the constraints-and-decisions.md artifact and its C↔D / IP traceability. This skill owns the constraints-and-decisions.md artifact structure (the D-XXX field schema) and traceability — NOT the decision-evaluation technique (use mochiko:patterns-technical-decisions to evaluate alternatives and write ADRs). Produces requirements.md, constraints-and-decisions.md, and nfrs.md from a business specification.
- **Kept deliberately:** the guardrails keep-set — the three-artifact spine (§§1–5 incl. the
  Structural Decisions subsection), the analysis-vs-downstream router blockquote, Traceability
  Rules mandatory links, Technology-Agnostic Writing, the "'Fast' is not a requirement" /
  "constraints are facts" / no-orphan / IP-coverage behavioral lines, the three no-exceptions
  lines, the Common Rationalizations table, the Quality Checklist, the Red Flags STOP paragraph,
  the letter/spirit epigraph, and all `references/` pointers (ARTIFACT-TEMPLATES,
  TRACEABILITY-PATTERNS).
- **KEPT reconciliation:** the [v0.28.0] and [v0.23.0] kept-sets below survive this cut in full —
  the When-to-Use bullets are not in any prior KEPT/protected set, and no `DECISIONS.md`-traceable
  line was removed. No prior KEPT or protected line is touched.
- **Consumers assessed:** technical-analyst (mounts it) · patterns-entity-modeling,
  patterns-technical-decisions (cross-reference the artifacts / D-XXX schema) · mochiko router.
  None links the removed When-to-Use bullets or a description clause. Contract intact.

## [v0.28.0] Reference-copied field tables, homed mistake rows, and excuse-column red flags stripped (body 229 → 135, −41%, in-band)
- **Disposition:** deduped → `references/ARTIFACT-TEMPLATES.md` (Read: every field appears in
  its Field Definitions with extra Format/Rules columns, plus document templates — and the SKILL
  already declared it the home): all five in-body field tables (TR / C / D / IP / NFR) and the
  sign-in decomposition example (richer 4-TR worked table there) · deleted (Tier 1, in-file
  homes): the §4/§5 canonical-home blockquotes (restated the top analysis-vs-downstream router
  blockquote, which stays; the x-integration field list and four-level taxonomy they enumerated
  live only in `patterns-api-contracts` / `patterns-entity-modeling` — one-line canonical-home
  clauses folded into the §4/§5 declaration paragraphs), the Completeness-check line (its six
  checks restate the kept Quality Checklist), the Red Flags trigger bullets + no-exceptions list
  (the six bullets map ~1:1 onto the Common Rationalizations table's Excuse column — five are
  semantic parallels, none verbatim; bullet 4's excuse shifted, "sensitivity is obvious"
  (authoring shortcut) vs the kept row's "classification is a security team concern"
  (delegation excuse) — its substance is homed at §5 + rationalizations row 4. STOP framing
  kept as one paragraph, the table kept whole as the discipline core, vertical-tdd precedent) · **Common Mistakes deleted whole** (all 6 rows homed: transcribing → decomposition
  rule + rationalizations row 1; unmeasurable NFRs → "'Fast' is not a requirement" + the
  reference's NFR-categories table; never-bounded → §4's kept optimistic-maps paragraph;
  preferences-disguised → the reference's Distinguishing table + violation test; unclassified →
  §5 + rationalizations row 4; orphans → Traceability Rules + checklist) · densified: the
  technology-agnostic Wrong/Right table → rule + one pair (constraints-MAY-name-tech exception
  kept), the D-technique blockquote → 2 lines (the boundary is also in the description,
  When-NOT-to-Use, ARTIFACT-TEMPLATES' Part-2 blockquote, and TRACEABILITY-PATTERNS' note —
  width-only, no line delta)
- **Tier failed:** 1 throughout (every cut had a verified richer home, most in the already-
  declared reference) · n/a for the densifications
- **Content:** five field tables, one example sentence, two blockquotes, one 4-row table, six
  mistake subsections, eleven red-flag/no-exception bullets; nothing written to `templates/` —
  dedups run against pre-existing reference content, D4's destination ban not engaged
- **Consumers assessed:** wave-open enumeration — 8 citing files (technical-analyst, plan,
  patterns-entity-modeling ×2, patterns-technical-decisions ×2, artifact-format, mochiko
  router); none links a section anchor. Kept: the three-artifact spine, Traceability Rules
  mandatory links, INT/DS declaration paragraphs, "'Fast' is not a requirement" /
  "constraints are facts" / no-orphan / IP-coverage behavioral lines, three no-exceptions
  lines, Common Rationalizations table, Quality Checklist, the letter/spirit epigraph (R4b:
  anchored to the envelope density rules directly above it). Session ruling: wave-4 batch-2
  ratified 2026-07-25.

## [v0.23.0] Description fields collapsed into the statement line across TR/C/IP/NFR blocks (T2, user-ruled)
- **Disposition:** revised per the wave-2 T2 ruling — the separate `**Description:**` paragraph field is deleted from all block templates (`references/ARTIFACT-TEMPLATES.md`) and field-definition tables (SKILL.md + reference); the ID line's statement IS the description
- **Tier failed:** artifact density (epic D4 extension): kinako's requirements.md 61k B / constraints-and-decisions.md 67k B were dominated by per-item field ceremony (label lines + a Description paragraph restating the statement), re-paid ~10× per feature
- **Content:** per-block forms compressed — TR: `**FR-XXX · MUST** — statement` + Criteria bullets + `**Deps:**` line (was Title/Source/Priority/Description/AC-checkboxes/Dependencies-list); C: type·severity·source on the statement line + one-line Impact (was 6 labeled fields); D: one-to-two-line Context + compact options table + ≤3-line Rationale + one-line Consequences (options/choice/ADR substance kept — `patterns-technical-decisions` owns the technique); IP: same collapse; NFR: statement line + Target/Measured/Applies-to lines (was 6 labeled fields + paragraph Requirement + paragraph Measurement Method). Summary tables kept and designated the **ID index** per `templates/artifact-format.md`. `references/TRACEABILITY-PATTERNS.md` pattern examples aligned to the statement-line form.
- **Consumers assessed:** plan producer (technical-analyst) + review-plan-artifacts (ARTIFACT-CHECKLISTS retargeted this wave) + review-feasibility (reads the artifacts; field-agnostic, checked — no edit needed) + downstream tasks/implement readers (consume IDs + statements, unaffected).

## [v0.23.0] Corrections landed in-wave (not strips)
- **Content:** (1) ARTIFACT-TEMPLATES' constraint Severity value set said `Hard` ("all constraints are hard boundaries by definition") while SKILL.md's field table says `blocking / significant / minor` — aligned to the SKILL (blocking/significant/minor), the pre-existing drift resolved toward the skill body; (2) `Constitution Alignment` field renamed `Governance alignment` (post-dissolution vocabulary, the plan-wave Constitution→Governance precedent), now optional-and-omitted when no principle applies.
