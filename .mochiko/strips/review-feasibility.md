# Strip notes — `skills/review-feasibility`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 2 (review-\* cluster,
batch-2 ratified 2026-07-25; design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`).
First strip assessment of this skill (never-stripped band 30–70): body 141 → 136 lines = **3.5%**,
deeply under-band — whole-skill survivor ruling below.

## [v0.91.0] Fix round 3 — the architecture pass's boundary watch (V2 N4)

- **Disposition:** superseded → "(a **design-phase artifact** against an input)".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D5). Raised as **V2 N4** on the
  audit's **untruncated** re-sweep — a round-1 truncation miss on the auditor's side, not a
  regression introduced here: the phrase sits deep in `FEASIBILITY-LENS.md` past the point the
  first pass read.
- **Content (superseded fragment, verbatim):**

  ```
  **Boundary watch:** you grade the *topology's conformance* to governance (a plan artifact against an input), never whether the governance itself is well-formed
  ```

- **Kept deliberately:** the boundary watch entire — you grade the topology's **conformance**,
  never whether the governance itself is well-formed, and that domain stays
  `validation-constitution`'s. The two-exits rule beside it (redesign to conform, or a user-ruled
  amendment/waiver through `governance-ledger.md`) and the resolvable-vs-fundamental split are
  untouched, as is the "cites the principle is not satisfies the principle" evidence rule.
- **Budget:** `references/` files are budget-exempt.
- **Consumers assessed:** this reference's three earlier v0.91.0 sites (hunt class 3, the
  architecture-pass trigger, the guardrails row) were re-keyed in the main pass and the first fix
  round; with this one the file carries no plan-stage vocabulary. The skill body's own G1 floor
  was re-keyed in the main pass.

## [v0.91.0] `references/FEASIBILITY-LENS.md`: hunt class 3 re-keyed off TR-XXX; plan-package wording — plan-stage retirement D3/D5

- **Disposition:** superseded → hunt class 3's seam is now requirements (FR-XXX / SC-XXX) ↔
  constraints, with a re-keyed worked example; two plan-run phrases re-keyed to the design phase.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D3 (the TR-XXX layer retires) and D5
  (this grader re-scopes to the design-phase output)). Reference-file scope was opened by the
  wave lead's extension ruling of 2026-08-26.
- **Content (superseded fragments, verbatim — three sites):**

  1. Hunt class 3, seam + worked example + evidence:

     ```
     **Seam:** technical requirements (TR-XXX) ↔ constraints.
     **Question:** does any requirement assume a capability not available under the stated constraints?

     **Worked example.** `TR-009: deliver real-time push notifications to mobile clients.` `C-007: no persistent connections permitted; polling only.` The requirement assumes a capability the constraint removes. Contradiction.

     **Evidence:** the `TR-XXX`, the `C-XXX`, and the missing capability the requirement depends on.
     ```
  2. Architecture pass: `Fires when the plan package carries a **store delta**`
  3. Guardrails table: `| Reviewing the constitution | G1: plan artifacts only. The constitution has its own validator. |`

- **Kept deliberately:** hunt class 3 **survives whole as a class** — the contradiction it hunts
  (a requirement assuming a capability its constraints remove) is unaffected by which id class
  states the requirement; only the seam's upper side moved from TR-XXX to the spec's own FR-XXX
  / SC-XXX, and the worked example was re-numbered `FR-009` rather than deleted so the class
  keeps its teaching case. The resolvable-vs-fundamental split, the architecture pass A1–A3, and
  every other guardrail row are untouched.
- **Budget:** `references/` files are budget-exempt.
- **Consumers assessed:** the skill's own `description:` and G1 body line were re-keyed in the
  same wave (entries below); `mochiko:review-plan-artifacts`'s boundary table points at this
  lens by section anchor, which is unchanged.

## [v0.91.0] `description:` re-scoped from the plan package to the design-phase package — plan-stage retirement D5

- **Disposition:** superseded → the same description grading design-phase analysis/design
  artifacts, as the adversarial half of the design-phase review pair.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D5: "`review-plan-artifacts` and
  `review-feasibility` re-scope at build time to grade the design-phase output and the
  sufficiency check's own honesty").
- **Content (superseded fragments, verbatim — three phrases in one field):**

  1. `to grade plan analysis/design artifacts for cross-artifact FEASIBILITY`
  2. `plus the architecture pass when the plan package carries an architecture-store delta`
  3. `The adversarial half of the plan pair; its sibling`

- **Budget:** description-class edit, canonical snippet: **568 → 599 chars** against the
  recorded budget of **625**. Inside budget; no justification owed.
- **Kept deliberately:** the hunt's whole subject (contradictions, impossibilities, buildability
  conflicts, unjustified structure / wrong altitude), the architecture pass and its trigger
  condition, the 3-state verdict, the sibling split with `review-plan-artifacts`, the
  never-defaults-to-`feasible` posture, and the not-the-constitution carve-out.
- **Consumers assessed:** the router row (re-keyed same wave), `mochiko:patterns-adopt-first`'s
  Who-grades-what table (names hunt class 7 as blocking-capable — class numbering untouched, so
  the pointer holds), `mochiko:review-plan-artifacts`'s boundary-table pointer (unchanged).

## [v0.91.0] G1 floor re-keyed — "plan artifacts only" → design-phase artifacts only — plan-stage retirement D5

- **Disposition:** superseded → the same G1 floor scoping the reviewer to design-phase artifacts.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D5).
- **Content (superseded text, verbatim):**

  ```
  - **Your verdict is input** — the lead owns clearing, loops, and the human gate; G1: plan
    artifacts only, never the constitution.
  ```

- **Budget:** body 1,893 → 1,901 chars against the recorded budget of **2,367**. Inside budget.
  This skill was 90%-cut at v0.82.0; nothing from that keep-set was touched.
- **Kept deliberately:** the verdict-is-input rule, the lead's ownership of clearing and the
  human gate, and the never-the-constitution carve-out — G1's substance is unchanged; only the
  name of the artifact class moved.
- **Consumers assessed:** `references/FEASIBILITY-LENS.md` still carries plan-run wording and a
  worked contradiction example built on a TR-XXX (`TR-009`), which D3 retires — **out of this
  seat's writable scope, reported to the wave lead as an open ripple**. `implement.md` (P1's
  rewrite dispatches this grade in the design phase).

## [v0.82.0] User-ruled 90% body cut with breakup into references — body 18,959 → 1,893 chars (−90.0%)

- **Disposition:** superseded → a floors-and-dispatch body plus a widened
  `references/FEASIBILITY-LENS.md`. The user ruled the cut directly ("breakup + trim ≥90%",
  2026-08-22, "cut now, eval validates later" — the skill-compression eval pilot for this skill
  is thereby superseded as a pre-cut instrument and re-purposed as a post-cut regression check).
  The reference file already twinned most body content, so the cut is predominantly honest
  dedup; genuinely unique body content was **relocated**, not deleted. `description:` untouched.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md` + the in-session user
  ruling recorded there; `DECISIONS.md` 2026-08-22 row).
- **Disposition map (per section of the old body):**
  - *Overview (4 paragraphs)* — compressed into the body's opening: intersection-hunting,
    adversarial-not-checklist, and looking-buildable≠being-buildable each survive as a clause.
  - *When NOT to Use (4 bullets)* — deleted as restatement: completeness→sibling (body line +
    lens "What this lens is NOT"), constitution G1 (body floor + lens A3 boundary watch),
    never-author-what-you-grade (body), single-artifact exclusion with the v0.81.0
    element-not-file narrowing (lens intro, now carrying the class-7 exception explicitly).
  - *What you hunt — classes 1–6 table* — deleted as restatement: the lens carries every
    class's seam + question + worked example.
  - *Class 7 full rules + calibration + interrogatory round + adopt-first limb* —
    **relocated → lens "Class 7 — unjustified structure / wrong altitude" section**,
    faithful text: necessity = ladder rung 1 (`patterns-plan-minimalism`, never restated),
    altitude test, blocking-capable / never-alone-`infeasible`, calibration clause, the
    interrogatory round, the adopt-first limb with its `patterns-adopt-first` pointer.
  - *External premises* — compressed into a body floor bullet; the
    `EXTERNAL-CLAIMS.md` binding survives by path.
  - *Architecture pass (AD-D7 PROTECTED)* — body restatement deleted; the pass survives
    whole in lens sections A1–A3 (which already carried the full text), renumbered from
    7/8/9 to clear the class-7 numbering collision; the group-B conformance surface (layer
    rules · dependency allowlist · GI-linked principles · floor-asserted obligations) maps to
    A3's question + both worked examples; the two-exit routing survives in A3 **and** as a
    body floor.
  - *The boundary section + mirror table* — deleted as restatement of the lens boundary
    watches + "What this lens is NOT" (store-delta split included). Uncarried residue: the
    sibling's verdict vocabulary (`ready / needs-revision / critical-gaps`) — lives in the
    sibling's own skill and the router; assessed as safe.
  - *Core Process steps 2–5* — deleted as restatement: hunt-each-class (body load line +
    lens), gate fuel (lens four-field contract), resolvable-vs-fundamental (lens verdict
    recap, new classification line), the 3-state verdict table (lens verdict recap + body
    verdict line); the never-default-`feasible` and `infeasible`-never-flattened floors moved
    to body floors verbatim-faithful.
  - *Independence (stated by role)* — compressed into body floors: the v0.64.0
    evidence-in-artifacts floor line survives ("review evidence living only in conversation is
    a floor violation"), verdict-is-input survives, G1 survives, author≠grader survives.
  - *Common Mistakes / Red Flags / Common Rationalizations (three tables)* — **relocated +
    merged → lens "Reviewer guardrails" table**: nine rows preserving every distinct failure
    mode; overlapping tells (trust-the-summary ×2, default-feasible ×3, flatten ×2, boundary
    creep ×2) deduped into single rows.
  - *Related* — deleted; the sibling is named in the body, the template binding is now pathed
    in a body floor.
- **Additions riding the ruling (no strip):** the hunt-coverage disclosure floor (one line per
  class in the report, never a narrative — the bounded proof-of-hunt from the same ADR); the
  **pathed** report bindings (`templates/feasibility-report-template.md` under
  `templates/report-format.md` — previously pathless, the kinako delivery failure); the lens
  gate-fuel field names repaired to the template's canonical `gap / at / impact / fix` (the
  reference still carried the pre-v0.26.0 prose names the body had already abandoned).
- **MANDATORY KEPT reconciliation:**
  - **[v0.26.0] KEPT: entire remaining body (whole-skill survivor).** Superseded by this
    user ruling as a body-residency claim; every named core survives in the skill: the
    six-class content and boundary content in the lens, the `infeasible`-never-flattened and
    never-default-`feasible` floors in the body, the Mistakes/Rationalizations failure modes
    in the lens guardrails.
  - **AD-D7 architecture pass** (`DECISIONS.md` 2026-07-30) — verified fully present after
    the edit: lens A1–A3 carry topology feasibility, governance conformance, the
    floor-asserted limb, and the two-exit routing; the body carries the
    never-silently-approved floor.
  - **[v0.64.0] floor line** (evidence in the artifacts) — survives in a body floor,
    verbatim-faithful.
  - **[v0.67.0] class 7** — survives as a lens section + the verdict-recap cells
    ("no surviving class-7 finding" / "or class-7 findings" / "never alone earns it").
  - **[v0.81.0] element-not-file narrowing + D14 floor-asserted bullet** — survive in the
    lens intro and A1/A3.
- **Consumers assessed:** `review-plan-artifacts:24`, `patterns-adopt-first:101`, and the
  router rows cite "hunt class 7" — the name survives (lens section title + body).
  `tech-lead.md:94` ("the step-by-step procedure … lives in review-feasibility") — still true;
  the skill is body + references. The router's `review-feasibility` row content claims (class
  7, calibration, interrogatory round, architecture pass, floor-asserted limb, 3-state
  verdict) — all survive in the skill. `templates/feasibility-report-template.md` gains the
  `hunt_coverage` frontmatter field at the audit fix round so the disclosure floor has a
  bounded home (audit finding 1; strip:
  `.mochiko/strips/feasibility-report-template.md` [v0.82.0]).
  Body budget re-seeded downward in `.mochiko/memory/primitive-cost-budgets.md` (R11):
  1,893 measured → budget 2,367.

## [v0.81.0] Architecture pass re-keyed to the store delta; NFR↔topology reads both sides in the store — product-architecture-schema D3/D10/D12/D14

- **Disposition:** superseded → the same architecture pass, keyed on the plan package's **drafted
  store delta** (topology + `AX-XXX` concern-row changes against the standing store at
  `.mochiko/product/architecture/`) instead of the per-feature `architecture.md`, which D3 kills.
  The `description:` re-key travels with it because the pass's trigger is a model-invocation
  routing condition.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D3 · D10 (sign-off is the write
  gate, so the pass grades a draft) · D12 (`nfrs.md` absorbed — NFR targets ride concern rows) ·
  D14 (floor precedence); `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — five fragments):**

  ```
  plus unjustified structure / wrong altitude; plus the architecture pass when `architecture.md` is in scope.
  ```

  ```
  ## The architecture pass *(when `architecture.md` is in scope)*

  When the design-time architecture artifact (`architecture.md`, owned by
  `mochiko:patterns-system-design`) is under review, the hunt gains an **architecture pass** on top of
  the six classes — two lens groups, both cross-artifact, both adversarial.
  ```

  ```
  | **NFR ↔ topology** | … | NFRs ↔ architecture |
  | **Constraint ↔ topology** | … | constraints / IP ↔ architecture |
  ```
  *(the two Artifacts-in-tension cells only; both Question cells survive verbatim)*

  ```
  **On the architecture artifact,** the same line is drawn one level up: you own **topology feasibility + governance conformance** (the architecture pass); the sibling owns **component-table↔diagram coverage, qualifying-flow sequence coverage, and whether `data-model.md` / contracts conform to the approved shape**. "Can this topology be built and does it honor governance?" is yours; "are the architecture's own pieces present and covered?" is the sibling's.
  ```

  ```
  Fires when `architecture.md` (the design-time topology, owned by `mochiko:patterns-system-design`) is
  under review.
  ```
  *(plus, in the same reference file: lens 7's seam line, lens 8's seam line and worked-example
  first clause, the "What this lens is NOT" architecture bullet, and the closing one-line scope
  statement — each re-keyed from `architecture.md` to the store delta, no check dropped)*
- **Kept deliberately:** all six contradiction classes untouched; class 7 (excess / altitude) and
  its calibration clause untouched; the three-state verdict and the never-default-to-`feasible`
  floor untouched; lens 9's **two exits** (redesign to conform, or a user-ruled amendment/waiver
  through `governance-ledger.md`) and the never-silent-approval routing untouched — the
  AD-D7-protected architecture pass survives whole, only its subject moved.
- **Additions riding the decision row (no strip):** the **floor-asserted obligations actually
  met** bullet in group B (D14 leg 1, landing on lens 9's existing verified-not-asserted
  machinery) with its worked example; the **element-not-file** clarification of the
  single-artifact exclusion (see the separate note below).
- **Consumers assessed:** `mochiko:review-plan-artifacts` mirrors this boundary in its Scope
  table and its ARTIFACT-CHECKLISTS boundary table — both re-keyed in the same edit set by this
  seat, and the floor limb is split across them deliberately (vocabulary legality = the sibling's
  mechanical check; whether the shape honors the obligation = this skill's). `plan.md` (P2)
  dispatches the pass by name. Router row re-keyed (this seat).
  **`templates/feasibility-report-template.md:15`** — its `artifacts_reviewed:` example listed
  `nfrs.md`; re-keyed at the V4 delta pass (B2) to
  `[requirements.md, constraints-and-decisions.md]` with the store delta named as the structural-run
  addition. Missed on the first pass: the sweep covered skills and the router but not the report
  template this skill fills, which is exactly the kind of downstream carrier a consumer check is
  supposed to catch.

## [v0.81.0] Single-artifact exclusion narrowed — the seam is between elements, not files

- **Disposition:** superseded → the same exclusion with its unit named. D12 puts `NFR-XXX`
  targets and the topology spine on **one** surface (the architecture store), so a literal
  reading of "feasibility lives between two artifacts" would have disqualified the NFR↔topology
  lens the same ruling keeps. The rule now keys on elements.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12 — the absorb whose stated
  consequence is "`review-feasibility` NFR↔topology lens (simpler after — both sides one
  artifact)"; `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — the bullet's closing sentence):**

  ```
  Feasibility is strictly **cross-artifact**: it lives between two artifacts.
  ```
- **Kept deliberately:** the exclusion itself and both its examples ("an NFR that is vague, a
  requirement that is incomplete *on its own* is not feasibility") verbatim — the narrowing adds
  the unit, it does not open the door to single-element findings. The Common Mistake "Reviewing
  one artifact in isolation" and the Red Flag list are untouched and stay literally true.
- **Consumers assessed:** n/a — internal to this skill's scope statement; no primitive restates
  it (grep clean).

## [v0.67.0] Class 7 (unjustified structure / wrong altitude) added — section scope + verdict cells re-keyed
- **Disposition:** superseded → the excess/altitude posture from the architect-role ruling: the hunt gains a seventh, **remove-shaped** class, so the "contradictions only" section scope and the contradiction-scoped verdict cells no longer hold and were rewritten to admit class-7 findings.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md`, D3 as amended by its F3 calibration clause; DECISIONS.md combined-wave build row [architect-role-pushback-and-abstraction + plan-structure-yagni]).
- **Content (superseded lines, verbatim):**
  - Section scope — OLD: "Cross-artifact contradictions / impossibilities / buildability only. Each class is a *lens* onto a seam between artifacts where an impossibility hides — not a checkbox. Hunting heuristics and worked examples for each are in [references/FEASIBILITY-LENS.md](references/FEASIBILITY-LENS.md)." → NEW scopes classes 1–6 to contradiction/impossibility/buildability, names class 7 as the one remove-shaped class whose rules sit under the table, and re-scopes the lens-reference pointer to "classes 1–6".
  - Verdict `feasible` when-cell — OLD: "every lens hunted, zero cross-artifact contradictions" → NEW: "every lens hunted, zero cross-artifact contradictions, no surviving class-7 finding".
  - Verdict `needs-revision` when-cell — OLD: "≥1 contradiction, all **resolvable**" → NEW: "≥1 contradiction or class-7 finding, all **resolvable**".
- **Kept deliberately:** the six cross-artifact contradiction classes (rows 1–6) untouched; the "six classes"/"six lenses" phrasing at the architecture-pass line, Core-Process Step 2, and `references/FEASIBILITY-LENS.md` all left literally true — class 7 is the seventh, single-artifact, non-lens class (non-renumber design, user-approved Q6); the `infeasible` verdict row unchanged (class 7 is never alone `infeasible`); the entire architecture pass (AD-D7 protected) untouched. Pure additions riding the decision row (no strip): the class-7 table row; the class-7 rules paragraph (necessity rung-1 pointer to `mochiko:patterns-plan-minimalism`, altitude test, calibration clause, interrogatory round); the heading "+ class 7 (excess / altitude)" append; the Step-2 class-7 hunt clause; the description "plus unjustified structure / wrong altitude" append.
- **Consumers assessed:** commands — plan orchestrates by dispatch, no class-count reference (grep clean). Sibling `review-plan-artifacts` references this skill by name in its boundary — unchanged; class 7 is an independent hunt, not the sibling's rung-honesty disclosure (architect-role consistency note 7). The new tech-lead persona (cluster-agents) declares this skill; the persona-agnostic body is unaffected (no principal-architect reference existed to re-key — grep clean).

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
