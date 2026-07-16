# Setup Rewrite — Constitution Flexibility & Deep Elicitation: Analysis Synthesis

**Date:** 2026-07-05 · **Session record (full audit trail + review):** [record.md](record.md) · **Status:** record accepted after end-stage adversarial review (15 findings raised → 11 survived → all dispositioned → 11/11 folds verified by both reviewers)

## Problem Statement

`/mochiko:setup` authors the project constitution *for* the user from a fixed, backend-flavored baseline — the user's first contact with principle content is an accept/amend gate on a finished document. The baseline was a personal first draft that the workflow calcified into the only constitution it can produce: insensitive to project type (frontend/CLI/library get backend governance) and to project scope (a weekend POC and a production service receive identical rigor). The rewrite makes declared intent the primary input: a rigorous interrogation session precedes and governs authoring.

## Context & Constraints

- **Kernel-free**: orchestration stays in native Claude Code constructs; discipline lives in skills/agents (repo non-negotiable).
- **Loop-discipline binds any redesign**: default-FAIL done-condition, independent validation, bounded iteration, named human gates.
- **The validator's structural bar survives**: three-part principles, quantified thresholds, no placeholders — flexibility is in *content selection*, never structure.
- **Cold session**: prior brainstorm notes on this topic were deliberately not consulted; all grounding came from shipped files via a seated fact-checker.

## Key Decisions

| # | Decision | Choice | Confidence | Rationale |
|---|----------|--------|------------|-----------|
| D1 | Essential Floor | Survives, **scope-tiered** — never floor-less, rigor scales with declared intent | Confident | Invariant floor reproduces one-size rigidity; no floor degrades the document to a preferences list |
| D2 | Low-tier floor categories | **Explicit recorded waivers** (waiving tier + revisit trigger), never soften-only or silent drop | Confident | No pretend-principles; absence stays deliberate and auditable; waivers form the graduation re-entry checklist |
| D3 | Principle content source | **Catalog-seeded, generation-open, interrogation-led** — deck follows interrogation; minted content traces to elicited intent | Confident *(sequencing user-authored)* | Pure catalog re-calcifies; pure generation starts every session from zero |
| D4 | Interrogation agenda | **Nine dimensions**: identity → tier → type → risk → team → tools → deployment (user-added) → values → exclusions | Confident | The definition of "go deep into all aspects"; each dimension feeds a named downstream consumer |
| D5 | Mode coverage | Session runs in **all three modes, depth-proportional**; brownfield confronts detected-reality-vs-declared-intent openly; amend gets a delta micro-session | Confident | Code says what IS, never what's intended; tier-bump amendments are governance events |
| D6 | Tier axis | **Named taxonomy as preset + elicited overrides** (`poc`/`internal`/`production`/`regulated`, labels soft) | Confident | Stable names carry deck filtering and waiver defaults; overrides prevent taxonomy calcification |
| D7 | Producer contract | Synthesis is a **traceable contract** — latitude in formulation only; deviations surface as flagged proposals; deterministic trace-IDs | Confident *(framing Assumed)* | The rewrite exists because producer defaults silently overrode intent; traceability closes that hole |
| D8 | Template layer | **Universal core + attachable type/tier modules**; checklist grades core + selected modules; skills change accordingly | Confident *(user-chosen at unrecommended fork)* | Parameterized single template rots; per-type variants drift; modules mirror the deck architecture |
| D9 | Catalog graduation | **Deferred with a named seam** — trace provenance already marks minted principles for future harvest | Deferred | Designing curation before any principle has been minted is speculative machinery |
| A1 | Catalog launch scope | **Frontend shelf authored in-scope** (principles + frontend-appropriate floor examples); CLI/library ship empty, mint-driven | Confident *(review-routed ruling)* | Frontend was the named pain; unseeded shelves for unused types would be baseline-authoring again |

New workflow shape: detect/mode-select → brownfield analysis + checkpoint → **interrogation session** (agenda → deck arbitration → waivers → **synthesis-confirmation checkpoint**) → authoring under traceable contract → validation (trace-IDs, tier, waiver-aware floor check, module-parameterized sections) → acceptance with trace summary. The synthesis persists and updates delta-wise on amend.

## Decision Trail

- **D3 — user amendment reshaped sequencing.** Recommendation was catalog-seeded-generation-open; the user added that the seed expands only after *rigorous interrogation of intent* — interrogation leads, deck follows. This became the session's spine.
- **D4 — user added the deployment dimension.** Lead drafted eight dimensions; the user asked "what about deployment?"— added as dimension 7 (release reality feeds Quality Gates and floor strictness).
- **D8 — chosen by the user at an unrecommended fork.** After three consecutive verbatim recommendation-adoptions (Q6–Q8), the streak was flagged and the template fork was posed with steelmans only. The user picked core+modules with their own reasoning and extended it ("the skills need to be changed accordingly").
- **Review-driven revisions (post-convergence).** Both reviewers independently found the synthesis-fidelity gap → a **synthesis-confirmation checkpoint** now closes the session (M2). A fact-checker-confirmed correction: the "validator-mandated backend skeleton" cited under D8 never existed — the real bias lives in un-mandated defaults and conditional template sections; ruling unchanged, rationale corrected (M1). The "all structural" claim about validator additions was retracted in favor of deterministic trace-IDs + a recorded judgment-grade residual (F7). A hard contradiction was fixed: the unchanged brownfield checklist would have failed any waiver-carrying low-tier constitution (F5).

## Risks

- **Semantic-fidelity residual (F7, recorded):** a fabricated-but-plausible trace passes the deterministic ID check; mitigated by the synthesis checkpoint and G3 trace summary, not eliminated.
- **Launch asymmetry (A1, accepted):** CLI/library projects run near-pure-generation until their shelves grow from real sessions via the D9 seam.
- **Frontend seed quality:** the new shelf is opinion-authoring; it's only as good as its author until validated by real frontend sessions.
- **Drift blindspot (F4, by design):** setup is on-demand; a project that never re-runs it is never re-checked — D2 revisit triggers fire on re-invocation only.
- **Tier-taxonomy calcification watch:** the named tiers are themselves a baseline; soft labels + per-project overrides are the designed counterweight.

## Open Questions (spec-stage)

- Final tier labels (strawman: `poc` / `internal` / `production` / `regulated`).
- Gate designation for the synthesis-confirmation checkpoint (insert vs renumber the G-sequence).
- Synthesis artifact name/home (e.g. `.mochiko/memory/governance-intent.md`).
- Module boundary drawing for the template set; where the agenda lives (`CONSTITUTION-INPUT.md`-style shape under `analysis-iterative` vs setup-local).
- Frontend shelf contents (accessibility, bundle/performance budgets, component/state discipline, release gates — to be authored).

## Recommended Next Steps

1. **Commit both artifacts** (`record.md`, `synthesis.md`) on the current branch — they're the durable design state.
2. **Run `/mochiko:specify`** with this synthesis + record as the feature description for the setup rewrite; the spec stage pins the open questions above.
3. **Author the frontend shelf during that feature** (in-scope per A1), validating it against a real frontend project soon after landing.
