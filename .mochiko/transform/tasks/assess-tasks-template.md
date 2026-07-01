# ASSESSMENT — P5 `tasks-template.md`

**Primitive:** `human-in-loop/plugins/humaninloop/templates/tasks-template.md`
**Run:** `/mochiko:transform-cluster tasks`, core-only · assess-only (no transform)
**Assessed:** 2026-07-01

---

ASSESSMENT: tasks-template
Class:        template → branch **artifact** (what matters: placeholders, what consumes it, path coupling)
Triage:       gate1=n gate2=y gate3=y → **full-lens** (fans out + non-machine-checkable artifact)
Disposition:  **port-with-edits × standalone** + one reconcile flag (RQ-A)
Reconcile flags: **RQ-A** — the Story→Cycle Mapping section (one-artifact absorb vs two-artifact dedupe)

---

## Triage gate (why the lens is warranted)

1. **Orchestration-coupled? NO.** The template is a passive fill-target; no kernel/DAG/markdown-supervisor *drives* it. Its only producer reference — `/humaninloop:tasks` inside the sample-cycle HTML comment (L42) — is **content-coupling** (a name in the body, fixed by an edit), not orchestration-coupling. This is exactly how the plan-template precedent treated its own producer stamp (scrub, not dissolve).
2. **Multi-responsibility / fans out? YES.** It carries the `tasks.md` deliverable **and** embeds the Story→Cycle Mapping (L191-198) — the RQ-A second-artifact content — and it feeds three consumers: the human reader, the downstream implementation workflow, and `validation-task-artifacts` (grader).
3. **Non-machine-checkable artifact? YES.** A filled `tasks.md`'s correctness (vertical-slice integrity, TDD ordering, story coverage) is model-judged by `validation-task-artifacts`, not a schema/version assert.

Gates 2 & 3 trip → full lens (artifact branch weighting).

## Lens (artifact branch — the three checks the run flagged made explicit)

**Path-rebind (checks 1 + 6 — coupling audit).** Two HIL-path bindings, both localized edits, matching the plan-template precedent exactly:
- **Provenance line L7** `> Generated from: spec.md, plan.md, requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/` — seven upstream artifacts as bare filenames, implicitly under HIL `specs/{feature-id}/`. Rebind the binding to `.mochiko/specs/<feature>/`. (Precedent: mochiko plan-template L4 **kept** the input-provenance line but rebound `/specs/[###-feature-name]/` → `.mochiko/specs/<feature>/`.) → `kept-but-rebind`.
- **HTML-comment L42/L44** names `/humaninloop:tasks` and re-lists the same upstream artifacts. Scrub the command name → "the tasks workflow" (precedent: plan-template L57 `/humaninloop:tasks` → "the tasks workflow"); rebind the L44 artifact list same as L7. → `kept-but-rebind` (decouple).
- **No brain/DAG/`.workflow/`/MCP/catalog token survives** — decouple scan finds exactly **one** HIL token (`/humaninloop:tasks`); everything else is generic craft. Sample code paths (`tests/e2e/test_[entity]_crud.py`, `src/models/[entity].py`) are illustrative `[...]`-parameterized samples, not HIL coupling — craft, kept.

**Fill-style (placeholder convention).** The template is **already** entirely `[...]` bracketed model-fill (`[FEATURE NAME]`, `[N]`, `[Entity]`, `[P]`, `[US#]`, `[Observable outcome]`, `[EXTEND]`/`[MODIFY]`). **No `{{}}` mustache anywhere.** Matches the plan-template deliverable style exactly → **no fill-style conversion needed** (confirm, not a fix). (The mustache lives in the *context*-template P6, the DAG-node payload — not here.)

**RQ-A dependency (check 5 — sibling/overlap → flag, do not resolve).** Confirmed against HIL source: **no `task-mapping-template.md` exists.** HIL's tasks workflow runs two phases — `mapping` → `task-mapping.md` (authored **freehand** by `task-architect` from its per-cycle **Success Criteria**; agent L55-61) and `tasks` → `tasks.md` (this template). This template **already substantially contains the mapping**: the **Story → Cycle Mapping** table (L191-198) is the collision point.
- If RQ-A collapses to **ONE artifact** → this section IS the absorbed mapping; keep whole, it's authoritative here. No new template.
- If RQ-A keeps **TWO artifacts** → the mapping lives freehand in `task-mapping.md`; this section becomes a *summary* view and the dedupe question ("which is authoritative") lands on reconcile. **Still no new template forced** — HIL never had one, so this template's **file-level disposition is `standalone` either way**; RQ-A only decides whether the *section* is authoritative-here or a deduped summary.
- → **flag-for-reconcile.** Reconcile (Phase-2 human gate per `context.md`) owns it; assess does not guess it.

**Sample-cycle mechanism (craft, not coupling — confirm).** The embedded SAMPLE cycles (Foundation C1-C2, Feature C3-C5) + the HTML-comment instruction to replace them (L38-53) are **craft** — a deliberate illustrate-then-replace pattern. They carry **no HIL-specific vocabulary** once `/humaninloop:tasks` is scrubbed from the comment. The mechanism survives cleanly; only the command name inside it is rebound.

## Disposition

**Body: `port-with-edits`** — body is mochiko-clean craft; edits are localized (rebind L7 provenance path, scrub L42 command name + rebind L44 list, one minor frontmatter call). Not `keep-verbatim` (there ARE edits); not `redesign` (no kernel/DAG assumption). **Mirrors P10 plan-template's `port-with-edits`.**

**Structural: `standalone`** — self-contained deliverable template, one home (`plugins/mochiko/templates/tasks-template.md`). RQ-A does **not** force a structural split of this file (no separate mapping template exists to pair with); it is carried as a reconcile flag on the mapping *section*, not a `split`/`pair` of the file. **Mirrors P10 plan-template's `standalone`.**

## Responsibility trace (every section tagged — no silent drops)

| # | Section / responsibility (line) | Tag |
|---|---|---|
| 1 | Frontmatter `description:` (L1-3) | **dropped + reason:** redundant with the H1 title; mochiko deliverable-template precedent (plan-template) carries **no frontmatter**. Minor — a transform-time judgment; flagged, not silent. |
| 2 | H1 `# Implementation Tasks: [FEATURE NAME]` (L5) | **kept** (deliverable title, `[...]` style; = plan-template `# Implementation Plan: [FEATURE]`) |
| 3 | Provenance `> Generated from: …` (L7) | **kept-but-rebind** — 7 upstream artifacts → `.mochiko/specs/<feature>/` (= plan-template Input line) |
| 4 | `> Structure: Vertical slices … TDD cycles` (L8) | **kept** (craft descriptor, no coupling) |
| 5 | Overview metrics table (L10-17) | **kept** (Total/Foundation/Feature/Parallel `[N]` fill-targets) |
| 6 | Cycle Format — TDD discipline (L19-26) | **kept** (craft; generic TN.1 test-first … TN.X demo) |
| 7 | Markers legend `[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` (L27-34) | **kept** (generic markers; brownfield markers are craft, not the deferred roadmap track) |
| 8 | Sample-cycle HTML comment (L38-53) | **kept-but-rebind** — mechanism kept; scrub `/humaninloop:tasks` (L42) → "the tasks workflow"; rebind L44 artifact list |
| 9 | Foundation Cycles + samples C1-C2 (L55-92) | **kept** (sample craft; `[...]`-parameterized paths, replaced by the model per the comment) |
| 10 | Feature Cycles + samples C3-C5 (L96-148) | **kept** (sample craft; `[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` demonstrated) |
| 11 | Execution Strategy — MVP/Incremental/Parallel (L152-175) | **kept** (craft; generic delivery strategy, no HIL vocab) |
| 12 | Cycle Dependencies ASCII diagram (L183-189) | **kept** (craft) |
| 13 | **Story → Cycle Mapping table (L191-198)** | **kept** + **flag-for-reconcile (RQ-A)** — one-artifact = authoritative absorbed mapping; two-artifact = deduped summary vs freehand `task-mapping.md` |
| 14 | Notes — TDD/Vertical/Foundation/Parallel/Commit/Checkpoint (L202-209) | **kept** (craft; generic guidance) |

**Trace complete:** every section accounted for — 11 `kept`, 2 `kept-but-rebind`, 1 `dropped + reason` (auditable), 1 reconcile flag on a kept section. No silent loss.

## Hand-off to reconcile

- **RQ-A (artifact shape):** does the Story→Cycle Mapping section (L191-198) stay authoritative here (ONE artifact) or become a deduped summary against a freehand `task-mapping.md` (TWO artifacts)? **Confirmed:** HIL has **no** `task-mapping-template.md`; the mapping is authored freehand from `task-architect`'s Success Criteria. Either resolution leaves this template `standalone`; only the section's authority changes. Reconcile + the Phase-2 human gate own the verdict.
