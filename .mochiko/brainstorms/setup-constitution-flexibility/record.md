# Brainstorm Record — Setup Rewrite: Constitution Flexibility & Deep Elicitation

**Date:** 2026-07-05
**Status:** ACCEPTED (2026-07-05) — review verified 11/11, all dispositions logged, user accepted. This record is the session deliverable.
**Slug:** `setup-constitution-flexibility`

## Topic

A fundamental rewrite of mochiko's setup workflow (`/mochiko:setup`). Two stated drives:

1. **More flexibility in constitution principles** — the principle set should not be decided for the user.
2. **A deep brainstorming session with the user** — going into all aspects of constitution setup *before* the constitution is authored for them.

## Problem statement (user's diagnosis, Q1)

The current workflow is too rigid on three axes:

- **Project-type bias:** the fixed greenfield set (Hexagonal, SRP, Dependency Discipline) assumes backend projects; frontend (and other) project types get governance that doesn't fit.
- **Project-scope insensitivity:** a quick POC and a production codebase receive the same constitution; rigor doesn't scale with intent.
- **Baseline mistaken for canon:** the current seven-principle scope was a baseline the user personally drafted — the workflow calcified it as the only constitution it can create. The rewrite must let the output adapt, not reproduce the baseline.

## Session constraints

- **Cold session by user instruction:** no prior brainstorm records on this topic were read (a prior `setup-workflow-rewrite/` directory exists and was deliberately not opened). All context comes from the shipped files and this conversation.
- **Fact-checker seated** — reality surface is the shipped setup cluster (`plugins/mochiko/commands/setup.md` + authoring/validation skills, agents, templates). Its factual map lands early in the session.
- No `.mochiko/memory/constitution.md` exists in this repo — no governing constitution to carry as context.

## Baseline facts (lead's own read, pre-map)

From `plugins/mochiko/commands/setup.md`:

- The user speaks at four points today: G1 mode-select (greenfield/brownfield/amend), G2 brownfield-analysis checkpoint, a *conditional* in-loop clarification sub-gate (only if the producer raises questions it cannot resolve), and G3 acceptance (accept / amend / reject) after authoring + validation are complete (`setup.md:24-53`).
- Greenfield principle content is a fixed opinionated set: Essential Floor I–IV + architectural principles V–VII (Hexagonal Architecture, Single Responsibility, Dependency Discipline) (`setup.md:45`).
- Nothing in the current flow elicits what the user wants governed before the producer authors. The user's first contact with principle content is the finished, validated artifact.
- Loop shape: producer `mochiko:principal-architect` (authoring-constitution) → validator `mochiko:validator` (validation-constitution), ≤3 rounds, default-FAIL, human acceptance at G3 (`setup.md:39-53`).

## Fact-checker map — load-bearing facts (with citations)

- **No elicitation upstream of authoring exists — confirmed.** Grep across the whole setup cluster for elicitation machinery: no matches. `analysis-iterative` (a full adaptive questioning engine with recommend-then-arbitrate and convergence handling) ships in the repo but is mounted only in `brainstorm`/`specify` — never in setup.
- **The floor's non-negotiability is authored doctrine, not deep structure.** "Every project, in either mode, MUST cover the four Essential Floor categories" lives at `authoring-constitution/SKILL.md:229` + `ESSENTIAL-FLOOR.md:5`. The validator checks floor coverage **only in its brownfield-specific block** (`QUALITY-CHECKLIST.md:41-42`); greenfield floor coverage is author-mandated, not validator-checked.
- **The validator grades structure, never selection.** Three-part rule per principle, quantified thresholds, placeholder scan, mandatory sections, semver — but no check constrains *which* principles or *how many* (the "5-10" figure is template-guidance only). Flexibility in content selection would pass today's validator untouched.
- **Rigidity extends into the template** *(corrected during review — M1; the original bullet over-claimed)*: the backend-flavored sections ("Project Structure + Layer Import Rules") are **conditional** in `constitution-template.md` and never validator-checked. What `QUALITY-CHECKLIST.md:15-23` genuinely mandates is **stack-general** — the Quality Gates table (with pre-seeded coverage thresholds) and the Technology Stack table. The backend bias actually lives in un-mandated places: the greenfield SHOULD-defaults (`RECOMMENDED-PATTERNS.md`) and the conditional template scaffolding. *(The over-claim — "the validator enforces the backend skeleton" — was the lead's summary error, fact-checker-confirmed false during review.)*
- **Doctrine already sanctions elicitation as a human gate.** `loop-discipline:92-95`: a preference gap "only the human can make" routes to the human gate; gate *placement* is per-workflow, presence non-negotiable. Desired principles are definitionally a preference gap.
- **Mode selects which phases run** (`setup.md:78`) — inserting an elicitation stage forces a decision about whether it runs in all three modes or only some.

## Decisions

**D1 — The floor survives, scope-tiered.** The Essential Floor *concept* stays: the tool will not emit a floor-less constitution. But floor content and strictness are parameterized by a **declared project scope/tier** (e.g. POC vs production) chosen during the pre-authoring session — rigor becomes a function of declared intent, and the declared scope itself becomes elicited content.
*Rejected:* invariant floor (reproduces the one-size rigidity diagnosed in Q1); fully open (a constitution with no floor degrades into a preferences document — the floor is what makes the document a constitution rather than a preferences list; its value is the governance contract with the user, not a downstream mechanical dependency — none exists today. *Rationale softened during review, A4.*).
*Grounding:* the current invariant is authored doctrine (`authoring-constitution:229`, `ESSENTIAL-FLOOR.md:5`), not infrastructure — retiering is a doctrine edit.
*Confidence:* Confident *(adopted from lead's recommendation without modification)*.

**D2 — Explicit waiver at low tiers.** A low tier may waive a floor category, but the waiver is recorded in the constitution itself — which tier waived it and the revisit trigger (tier graduation). No pretend-principles; absence is always deliberate and auditable; accumulated waivers become the governance re-entry checklist when the project graduates tiers.
*Rejected:* soften-only (residual ceremony — four authored principles regardless of fit); silent drop (absence is ambiguous between deliberate and forgotten).
*Confidence:* Confident *(adopted from lead's recommendation without modification)*.

**D3 — Content source: catalog-seeded, generation-open, interrogation-led.** Principle content comes from a tier/type-tagged catalog deck plus session-minted principles; minted principles are graduation candidates for the catalog (the deck grows from real sessions, not one author's baseline). **User amendment:** the seed is expanded only after *rigorous interrogation of the user's intent* — the interrogation leads, the deck follows. Minted content must trace to elicited intent, never to shallow prompting.
*Rejected:* pure catalog (repeats the calcification failure at larger scale); pure generation (discards battle-tested floor content; every session starts from zero).
*Confidence:* Confident *(recommendation adopted with a substantive user amendment — sequencing is user-authored)*.

**D4 — The interrogation agenda (nine dimensions).** The pre-authoring session works through these adaptively (not as a fixed script), in this order, before any catalog deck appears:

1. **Project identity & intent** — what's being built, for whom, expected lifespan.
2. **Scope tier** — POC / internal tool / production / regulated + expected graduation path. *(feeds D1, D2 defaults)*
3. **Project type & shape** — frontend / backend / fullstack / CLI / library / service. *(selects the catalog shelf, D3)*
4. **Risk surface** — what failure costs: data loss, money, reputation, compliance, user trust. *(drives floor strictness honestly)*
5. **Team reality** — solo vs team, experience mix, review culture. *(enforcement must fit who's enforcing)*
6. **Existing practices & tools** — detected stack, CI, linters, tests; brownfield analysis feeds in here. *(source of the real commands the validator requires)*
7. **Deployment & release reality** — target, environments, cadence, what blocks a release, rollback expectations. *(feeds Quality Gates; calibrates Observability/Error-Handling strictness; sharpens the tier declaration)* — **user-added dimension**
8. **Values & non-negotiables** — what the user insists on enforcing and explicitly refuses. *(the preference gap only they can fill; primary source for minted principles)*
9. **Deliberate exclusions** — what governance will not cover at this tier. *(feeds D2's recorded waivers)*

*Provenance:* lead-drafted eight dimensions; user added deployment (7) and confirmed the whole. *Confidence:* Confident.

**D5 — The session runs in all three modes, depth-proportional.** The interrogation covers only what the mode leaves unknown. Greenfield: full nine-dimension agenda *(subject to low-tier agenda pruning — an F3 clarification recorded in Workflow consequences)*. Brownfield: the code analysis pre-fills dimension 6; the session focuses on what code cannot say (intent, tier, risk, values); detected-reality-vs-declared-intent conflicts are **confronted in the open** ("you declared production tier; the codebase has no tests"), never silently resolved. Amend: micro-session scoped to the delta — an amendment that bumps the tier or un-waives a floor category is a governance event and gets the relevant agenda slice.
*Rejected:* greenfield-only (scope insensitivity bites brownfield hardest — code says what IS, never what's INTENDED); sparing amend entirely (tier-bump / un-waive amendments are exactly where intent needs re-eliciting).
*Confidence:* Confident *(adopted from lead's recommendation without modification)*.

**D6 — Tier axis: named taxonomy as preset, elicited overrides on top.** Fixed named tiers are the spine — catalog-deck filtering (D3) and waiver defaults (D2) hang off stable tier names — but the session's risk / values / exclusions dimensions (D4: 4, 8, 9) tune the preset per project. Tier sets the starting position, never the final one: the same preset→arbitration relationship the deck has. Strawman labels `poc` / `internal` / `production` / `regulated`; the structure is the decision, labels stay soft and may be renamed by usage.
*Rejected:* pure fixed taxonomy (reproduces the calcification pattern one level up); pure freeform scope description (mechanically breaks D3 — an untagged catalog can't be filtered — and leaves waiver defaults undefinable).
*Confidence:* Confident *(adopted from lead's recommendation without modification)*.

**D7 — The session synthesis is a traceable contract on the producer, not a brief.** Every principle in the authored constitution must trace to a synthesis element: deck-kept, minted-from-elicited-intent, or floor-preset-with-override. Producer latitude is confined to *formulation* (wording, enforcement mechanics, structuring intent into three-part principles with real commands) — never *selection*. Producer-believed deviations or additions surface as flagged proposals ruled on at the acceptance gate, never folded in silently. The validator gains the matching selection-level cross-check of constitution↔synthesis (precedent: brownfield's tools/versions cross-check against `codebase-analysis.md`, `setup.md:18` — *analogy qualified during review: the ID-match is deterministic, semantic fidelity judgment-grade; see F7*).
*Framing components (presented before the fork, answered within, unobjected — marked Assumed):* the lead conducts the interrogation inline via `analysis-iterative` (dispatched agents can't hold an interactive session; brainstorm precedent); the session emits a synthesis artifact — tier declaration, arbitrated deck rulings, waivers, minted-principle intents, real commands — as the producer's brief-source.
*Rejected:* brief-with-latitude (reopens the original silent-override hole one step later — the rewrite exists because producer defaults overrode user intent).
*Confidence:* fork Confident *(adopted from lead's recommendation without modification)*; framing components Assumed.

**D8 — Template layer: universal core + attachable modules; the skills change accordingly.** A slim core every constitution shares (Core Principles, Governance, versioning, CLAUDE.md sync mapping) plus type/tier modules selected by the synthesis (e.g. layer-rules for backend, release-gates, accessibility for frontend). The quality checklist checks core + selected modules — modularization kills the backend bias where it actually lives: the template's backend-flavored *conditional* sections and the greenfield architectural defaults become type-selected modules instead of universal baseline, and the one genuinely mandated pre-seed (coverage thresholds in the Quality Gates table) becomes tier-parameterized rather than fixed. *(Problem statement corrected during review — M1: the original cited a validator mandate that doesn't exist; ruling unchanged, user-ratified.)* **User addendum:** the skill layer changes accordingly — `authoring-constitution` learns module assembly from synthesis selections; `validation-constitution` grades core + selected modules; the template file becomes a core + modules set.
*Rejected:* one parameterized template (conditional branches rot into a template-program); per-type variants (N×N drift; new type = whole new pair — the calcification barrier again).
*Confidence:* Confident *(user-chosen at an unrecommended fork, post streak-flag, with own reasoning)*.

**D9 — Catalog graduation: deferred, with a named seam.** The rewrite ships with the seeded catalog only. The graduation mechanism (curation authority, admission bar, catalog home and versioning) is designed only after real sessions have minted real principles — designing a curation pipeline for content that doesn't exist yet is speculative machinery. The seam that makes deferral safe: D7 traceability already stamps every minted principle's provenance in the synthesis and the constitution, so a future graduation pass harvests candidates from real artifacts; nothing is lost by waiting.
*Confidence:* **Deferred** *(explicitly postponed pending real minted principles; adopted from lead's recommendation)*.

## Workflow consequences (lead's formulation — assembled from D1–D9, not separately user-ruled)

*Load-bearing calls in this block are provisional until record acceptance; the end-stage review stress-tested them at full strength — its folds are marked inline (M2, F2, F5, F7, F3, F4, A7, A1). (F8)*

The rewritten setup flow: **Phase 0 detect + mode-select (G1, unchanged)** → **brownfield analysis + checkpoint (G2, unchanged; feeds agenda dimension 6)** → **NEW: interrogation session** — lead-conducted inline via `analysis-iterative`; depth-proportional per mode (D5); nine-dimension agenda (D4), where an early low-tier declaration **licenses explicit agenda pruning** — foreclosed dimensions skipped and named as skipped, adaptive convergence handling the rest *(clarifies D5's proportionality — F3)*; then deck arbitration (D3) and waiver rulings (D2); the session closes with a **synthesis-confirmation checkpoint** — confirm / edit / reject on the assembled synthesis, mirroring G2, the discrete gate that ratifies synthesis↔intent before any authoring *(M2)* → **constitution loop** — producer authors from the synthesis as traceable contract (D7); validator grades → **acceptance (G3)** — presentation includes a **trace summary** (which principles trace where; flagged proposals listed) alongside rulings on any producer flagged-proposals (D7) *(M2)* → finalize.

Contract mapping (per `loop-discipline`):

- **Done-condition unchanged in kind:** default-FAIL; clears on validator PASS + human acceptance **+ synthesis-confirmation cleared (all modes)** (+ brownfield analysis checkpoint in brownfield mode) *(enumeration completed per verify pass — M2 follow-through)*. The session's questioning itself is a *named human gate*, never the done-condition — consistent with the existing rationale at `setup.md:22`.
- **Synthesis ratification** *(revised during review — M2)*: the synthesis gets a **discrete human checkpoint at session close** (confirm / edit / reject, mirroring G2). Both reviewers independently showed the original "arbitration IS ratification" stance leaned on an imperfect precedent — `setup.md:37`'s no-analysis-validator call presumed a deterministic baseline *plus* a checkpoint, and the synthesis had neither. No machine validator is added for the synthesis; the checkpoint is the fidelity gate, and the constitution remains the sole machine-graded deliverable.
- **Validator additions** *(deterministic-where-possible; the original "all structural" claim is retracted — F7)*: constitution↔synthesis traceability via **deterministic trace-IDs** — synthesis elements carry IDs, the producer stamps each principle with its source ID, the validator string-matches ID presence/validity (D7). **Semantic fidelity of a stamped trace remains judgment-grade — recorded residual risk** (a fabricated-but-plausible trace passes the ID check; mitigated by the M2 checkpoint and the G3 trace summary). Also: tier declaration present (D6); waiver format — waiving tier + revisit trigger (D2); section checklist parameterized by selected modules (D8); and the **brownfield floor-coverage check becomes waiver-aware** — "a principle *or a recorded waiver* per floor category" — without which a brownfield low-tier constitution with a D2 waiver fails its own validator as designed *(F5)*.
- **Producer escape valve** *(A7)*: the flagged-proposals channel explicitly covers *elicited intent that resists enforceable formulation* — the producer flags it rather than authoring vagueness or silently dropping it; the user rules at G3. Dimension 8's phrasing pre-filters for enforceability.
- **Out of scope, explicitly** *(F4)*: drift detection between invocations. Setup is on-demand tooling; D2 revisit triggers fire on re-invocation only — a project that never re-runs setup is never re-checked, by design.
- **Bounds and independence unchanged:** ≤3 produce↔validate rounds, no-progress exit, kill-switch; principal-architect authors / validator grades, disjoint. The session itself is bounded by user-driven convergence, not an agent loop.

Artifact inventory after the rewrite: the **catalog** — tier/type-tagged deck seeded by restructuring `ESSENTIAL-FLOOR.md` + `RECOMMENDED-PATTERNS.md` content **plus a net-new frontend shelf authored in-scope** (type principles and frontend-appropriate floor examples; **CLI/library shelves ship empty by accepted decision** and grow mint-driven via the D9 seam — *A1, user-ruled*); the **template set** (universal core + type/tier modules + per-module checklist fragments, D8); the **persisted synthesis artifact** (durable alongside `constitution.md`, name soft — e.g. `.mochiko/memory/governance-intent.md`; **amend runs update it delta-wise** — untouched principles keep their existing trace elements, and the traceability cross-check always runs against the persisted, updated synthesis — *F2*); **constitution.md** gains a tier declaration, waiver records, and trace stamps.

Implementation-level notes (soft, for the spec stage): the agenda likely lives with setup's own materials rather than inside generic `analysis-iterative` — precedent: `SPECIFICATION-INPUT.md` is a shape-reference under `analysis-iterative`; a `CONSTITUTION-INPUT.md`-style shape would fit. The synthesis-confirmation checkpoint needs a gate designation in the G-sequence (insert or renumber) at spec stage. Skills touched per D8: `authoring-constitution`, `validation-constitution`, the setup command, template + checklist files — plus, per F5, `analysis-codebase` (it consumes `ESSENTIAL-FLOOR.md` for present/partial/absent floor assessment and must become tier/waiver-aware) and the brownfield block of `QUALITY-CHECKLIST.md`.

## Corrections & reversals

*(logged where they happen)*

## Review

**Pair:** reviewer-a and reviewer-b (`mochiko:devils-advocate` running `mochiko:validation-brainstorm`, end-stage role) — independent cold reads formed blind to each other, then one cross-examination round; fact disputes settled by the fact-checker from files, never argued. A session-limit pause (09:21–11:51 UTC) interrupted between cold reads and cross-exam; both reviewer contexts survived intact.

**Tallies:** reviewer-a 7 raised → 6 survived (A5 withdrawn on a shown misread; A6 subsumed into F7). reviewer-b 8 raised → 8 survived (F3, F4 severity-downgraded under accepted attack). Cross-reviewer merges: A3≡F1, A2≡F6. **Combined unique survivors: 11 — 6 Important, 5 Minor.** Both reviewers recommend **needs-revision**; lead verdict: **needs-revision** (folds below, verify pass to follow, then acceptance).

### Survivors and dispositions

**Important**

- **M1 (A3≡F1) — Fact-map over-claim, fact-checker-CONFIRMED:** line 42's claim that the checklist makes "Project Structure + Layer Import Rules" mandatory is false — both are conditional in the template and absent from the checklist's mandatory block; the genuinely mandated skeleton (Quality Gates table with pre-seeded coverage, Technology Stack table) is stack-general. D8's *decision* survives on corrected facts; its stated problem was inflated — the real backend flavor lives entirely in **un-mandated** places (greenfield Hexagonal V–VII SHOULD-defaults in `RECOMMENDED-PATTERNS.md`; the two conditional template sections), which remains real material for D8 to modularize. → **Disposition: resolved; D8-rationale correction user-ratified.** The corrected problem statement: modularization kills the backend bias where it actually lives (conditional template sections + greenfield architectural defaults become type-selected modules; the genuinely mandated coverage pre-seed becomes tier-parameterized) — not a validator mandate, which never existed.
- **M2 (A2≡F6) — Synthesis-fidelity gap** (found independently by both, cold): the traceability check guards constitution↔synthesis, nothing ratifies synthesis↔intent; the no-analysis-validator precedent is imperfect (codebase-analysis has detect-stack + G2; the synthesis had neither). → **Disposition: resolved (lead accepts, no argument)** — fold: a discrete **synthesis-confirmation checkpoint** ends the session (mirror of G2: confirm / edit / reject on the assembled synthesis before any producer dispatch), and G3's presentation gains a trace summary (what traces where, incl. flagged proposals).
- **F2 — D5×D7 amend inconsistency:** delta-scoped amend synthesis vs every-principle cross-check; synthesis persistence undefined ("per-run" vs durable trace stamps). → **Disposition: resolved (lead accepts)** — fold: the synthesis is a **persisted durable artifact** updated delta-wise on amend (untouched principles keep their existing trace elements); the cross-check always runs against the persisted, updated synthesis. Amend stays delta-scoped; traceability stays total.
- **F5 — Floor-retiering blast radius (hardened by reviewer-a):** (a) `analysis-codebase` consumes `ESSENTIAL-FLOOR.md` for present/partial/absent assessment — unaddressed; (b) **direct contradiction:** brownfield checklist line "all four essential floor categories have principles" fails any brownfield low-tier constitution with a D2 waiver, as designed. → **Disposition: resolved (lead accepts)** — fold: brownfield floor-coverage check becomes waiver-aware ("principle **or recorded waiver** per category"); change-set inventory gains `analysis-codebase` + the brownfield checklist block.
- **F7 — Traceability check is judgment-grade, not "structural" (subsumes A6):** minted-principle↔intent matching is semantic; a plausible fabricated trace could pass, undercutting D7's anti-silent-override core. → **Disposition: resolved (lead accepts)** — fold: **deterministic trace-IDs** (synthesis elements carry IDs; producer stamps each principle with its source ID; validator string-matches ID presence/validity — deterministic), while **semantic fidelity is explicitly recorded as judgment-grade residual risk**, mitigated by the new M2 checkpoint + G3 trace summary. The "all structural" wording is retracted.
- **A1 — Non-backend catalog seed under-serves the motivating goal:** seeding only from `ESSENTIAL-FLOOR.md` + `RECOMMENDED-PATTERNS.md` (backend/service content, backend-flavored floor examples) leaves frontend/CLI/library shelves empty at launch — ≈ the pure-generation mode D3 rejected, for the flagship frontend use case. (reviewer-b calibration attached: benefit-deferred, not failure — still better than forced backend defaults. Sharpened in close-out: even the floor's worked examples are backend-flavored — RFC 7807 error bodies, `/health` endpoints — so an SPA gets misfitting content even from the "universal" shelf.) → **Disposition: user-ruled — resolved.** Ruling: seed a **frontend shelf in-scope** (type principles — accessibility, bundle/performance budgets, component/state discipline, release gates — plus frontend-appropriate floor *examples*); **CLI/library shelves stay mint-driven** with the launch asymmetry recorded as accepted — those shelves grow organically from real sessions via the D9 seam, rather than from another speculative author-baseline.

**Minor**

- **A4 — D1 rationale leg unsupported:** no downstream workflow reads the floor (grep-verified); "what downstream workflows lean on" is hollow, the preferences-document leg still holds. → **Disposition: resolved; D1-rationale softening user-ratified** — the rejection rests on the governance-value leg alone.
- **A7 — Can't-formulate edge (D7 lock × enforceability bar), heavily mitigated:** → **Disposition: resolved (lead accepts)** — fold: flagged-proposals channel explicitly covers "elicited intent that resists enforceable formulation" (producer flags it; user rules at G3); dimension 8 phrasing pre-filters for enforceability.
- **F3 (downgraded) — Tier-proportional interrogation depth unspecified:** → **Disposition: resolved (lead accepts)** — fold, as clarification of user-ruled D5 proportionality: an early low-tier declaration **licenses explicit agenda pruning** (foreclosed dimensions skipped, named as skipped), with adaptive convergence handling the rest.
- **F4 (downgraded) — Silent tier-graduation residual:** → **Disposition: resolved (lead accepts)** — fold: explicit out-of-scope note — setup is on-demand; drift between invocations is unmonitored by design; D2 revisit triggers fire on re-invocation only.
- **F8 (refined) — Consequences block absorbs un-ruled load-bearing claims as settled prose:** → **Disposition: resolved (lead accepts)** — fold: the block's load-bearing calls get explicit provisional markers; its biggest ("synthesis is not independently validated") is superseded by M2's checkpoint fold.

**Fallen (retrievable on ask):** A5 (withdrawn — misread shown by counterpart); A6 (subsumed into F7); reviewer-b reserves (waiver blast-radius duplicate; line-32 "four points" imprecision).

### Verify pass

Both reviewers independently confirmed **11/11 folds landed**, quoting the record's new text per fold; no fold contradicts a ruled decision; Review-section tallies confirmed accurate by both. The pass surfaced four post-fold one-liners, all applied: **(i)** D5's "full nine-dimension agenda" now points at the F3 pruning license (fold-introduced inconsistency, reviewer-b); **(ii)** the done-condition enumeration gained the synthesis-confirmation checkpoint alongside the brownfield checkpoint (fold-introduced asymmetry, reviewer-b); **(iii)** D7's precedent analogy carries the F7 qualifier (soft note, reviewer-a); **(iv)** the spec-stage notes name the checkpoint's missing gate designation (soft note, reviewer-a). Final reviewer statuses: reviewer-a "verified clean — cleared to proceed to user acceptance"; reviewer-b "acceptance-ready once (i)+(ii) reconciled" — both reconciled above. Clearing verdict is lead-owned; acceptance is the user's.
