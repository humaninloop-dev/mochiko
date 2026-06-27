# Transform — `analyst-report-template.md` (P12)

**Run:** transform-cluster / `specify` · Phase 3 (transform) · **Producer:** `transform-producer`
**Skill:** `mochiko:transform-recipes` (APPLY ONLY — not graded here; `verify-output` runs independently)
**Date:** 2026-06-27 · **Disposition (finalized, Phase-2 human-gate ACCEPTED):** `port-with-edits × standalone`
**Inputs:** reconcile.md §D row P12, §E P12 trace, §G template-wrapper, Agenda 6c/6d · assess-analyst-report-template.md · HIL source `templates/analyst-report-template.md`

---

## TRANSFORM (standard format)

```
TRANSFORM: analyst-report-template.md (P12)
Applied:   port-with-edits × standalone + convention-wiring pass
Artifacts: plugins/mochiko/templates/analyst-report-template.md (CREATED)
New partners: none (standalone; producer↔validator pair already exists — analyst ↔ advocate)
Wiring:    classification=n/a (inert template) · router=deferred-to-router-edit (NOT touched here) ·
           triggers=n/a (no description) · rebinds=[ {{iteration}}→{{round}} ; output path
           specs/{feature-id}/.workflow/ → .mochiko/specs/<feature>/analyst-report.md ] ·
           decouple-scan=CLEAN (role-based wrapper; no agent names / no "dispatch" / no workflow-agnostic)
Trace (realized): see §"Realized trace" below — every responsibility carries a final tag, no silent loss
```

---

## Body edits applied (port-with-edits)

1. **`{{iteration}}` → `{{round}}`** (header). Relabeled `> Iteration:` → `> Round:` to stay coherent. The DAG `pass_number` becomes the lead's bounded-loop round counter. **This rename is the matched-pair convention — IDENTICAL to P13 (`advocate-report-template`).** P13 is not yet written; reconcile Agenda 6c / §D P13 / §E P13 fix the canonical form (`{{iteration}}`→`{{round}}`, same as P12), so whoever ports P13 MUST use exactly this text-form rename. No other body edit is owed by the rebind.
2. **Demoted the counts-first shape.** HIL opened with the `## What I Created` counts table (parser-extraction surface). Reordered so the lead reads prose first: **Summary → Assumptions Made → What Changed This Round → Notes**, with the counts table moved to the bottom as `## What I Created (optional)`.
3. **Added `## What Changed This Round` (`{{changes_this_round}}`)** as a foregrounded section. Explicitly directed by the task and reconcile Agenda 6d (lead reads "what-changed-this-round"). Realizes the producer's round-over-round delta disclosure, which feeds the lead's no-progress watch (LD-3.2, §B.1) and the targeted-revision pattern (§B.4). Round-1 vs round-N guidance lives in Usage Notes, not inline, per the wrapper convention.
4. **Counts table = kept-but-rebind (demoted), trajectory-metric feed = dropped.** The five count placeholders survive as an optional convenience snapshot. Relabeled the column header `Metric` → `Section` to retire the metric framing. The DAG **outcome-trajectory metric feed** (count-per-pass trend that the state-analyst built from these counts) is **dropped + reason: kernel-free** — no trend/trajectory field added; the lead and critic read `spec.md` directly.
5. **Aligned to the mochiko template wrapper** (`codebase-analysis-template.md` precedent): `# Analyst Report Template` + intro line + `---` + ` ```markdown ` fenced body (filled doc titled `# Analyst Report`) + `---` + `## Usage Notes` (numbered). Guidance pushed to Usage Notes; the fenced body is the pure shape with no inline HTML-comment instructions (matches the precedent). No nested code fence in the body, so the wrapper fence is unambiguous.

**Kept (unchanged content):** `## Summary` (`{{summary}}`), `## Assumptions Made` (id/assumption/rationale table), `## Notes` (`{{notes}}`), `{{feature_id}}` / `{{timestamp}}` header fields, `{{mustache}}` placeholder convention.

## Structural move applied (standalone)

One home: `plugins/mochiko/templates/analyst-report-template.md`. Filled by the producer; read directly by the lead and (as context) the critic. **No split / merge / promote / absorb** — it is a genuine producer-authored artifact (contrast P14 context-template, which was pure orchestration state → absorb-into-lead). The producer↔validator pair it participates in already exists (analyst ↔ advocate; verdict lives in P13), so this template owes no partner primitive.

## Convention-wiring pass (all five)

1. **Classification** — n/a. Inert template: no `disable-model-invocation`, no `skills:` list, no trigger phrasing.
2. **Router registration** — **NOT performed here** (task: "DO NOT edit the shared router"). Owned by the dedicated router edit (reconcile §G / §D router row: register `analyst-report-template` under `/mochiko:specify`). Flagged so it is not lost.
3. **Trigger phrasing** — n/a (no `description`).
4. **Path rebinding** — HIL body carried **no** path internally; the rebind is realized in the new Usage Notes: output location `specs/{feature-id}/.workflow/analyst-report.md` → **`.mochiko/specs/<feature>/analyst-report.md`** (mochiko workspace-as-state, §G / §B.2). Recorded `kept-but-rebind`.
5. **Decouple persona/skill** — wrapper content scanned against the deny-list (sibling-agent names `requirements-analyst`/`devils-advocate`/`state-analyst`, "dispatch", injected workflow modes/paths/phases, "workflow-agnostic"). **CLEAN** — independence stated by role only (`producer` / `critic` / `lead`); the body inside the fence names no agent (preserves the HIL body's clean scan). Audited downstream by `verify-output`.

---

## Realized trace — every responsibility flipped to its final tag

| Responsibility (from assess §Trace / reconcile §E P12) | Realized tag |
|---|---|
| Structure the producer's self-report (handoff disclosure) | **kept** — reoriented prose-first for the direct-reading lead; the structure survives |
| Capture Summary (`{{summary}}`) | **kept** — foregrounded (now first content section) |
| Capture Assumptions Made (id / assumption / rationale) | **kept** — foregrounded (second section); high value (the critic stress-tests assumptions) |
| Producer round-over-round delta (`## What Changed This Round`, `{{changes_this_round}}`) | **kept** — foregrounded NEW section realizing the self-report for a direct-reading lead; supports the lead's no-progress watch (LD-3.2 / §B.1) + targeted-revision (§B.4) |
| Capture free-form Notes (`{{notes}}`) | **kept** |
| "What I Created" counts table (5 counts) | **kept-but-rebind** — demoted to `## What I Created (optional)` at the bottom; `Metric` column → `Section`; convenience snapshot, no longer a metric feed |
| Feed the DAG outcome-trajectory metric (count-per-pass trend) | **dropped + reason: kernel-free** — DAG/parse-and-advance metric machinery excluded; lead reads report + `spec.md` directly. No trend field added. (→ human gate; accepted at Phase 2) |
| Report identity header (`{{feature_id}}` / `{{iteration}}` / `{{timestamp}}`) | `{{feature_id}}` **kept** (generic; grounded by the lead's workspace naming, no body edit) · `{{iteration}}` **kept-but-rebind → `{{round}}`** (lead's bounded-loop counter; **identical to P13**) · `{{timestamp}}` **kept** |
| `{{mustache}}` placeholder convention | **kept** (matches `codebase-analysis-template`; no normalization owed) |
| Be the "Report format: follow <template>" dispatch target | **kept-but-rebind** (template path → `plugins/mochiko/templates/...`) **+ dispatch reference moved-to-lead** (the lead points the producer at this template via `agent-dispatch.md`) |
| Be heading-parsed by the state-analyst (parse-and-advance extraction) | **moved-to-lead** — the parsing layer dissolves with the state-analyst (P4); the lead reads the report directly (stable headings still aid a human-style reader) |
| Output-location / placement | **kept-but-rebind** — `.mochiko/specs/<feature>/analyst-report.md` (workspace-as-state); seeded/collected by the lead |
| Router registration (discoverability) | **kept-but-rebind (wiring)** — deferred to the dedicated router edit; NOT done in this artifact per task scope |

No responsibility left untagged. **No silent drop:** the single `dropped` item (the DAG outcome-trajectory metric feed) carries a kernel-free reason and was accepted at the Phase-2 human gate (§E drop table / §F item 6). The counts CONTENT survives (demoted, not deleted).

---

## Deviations

- **None from the finalized disposition.** All five directed edits applied as specified (`{{iteration}}`→`{{round}}`; demote counts; drop trajectory-metric feed; keep structure/Summary/Assumptions/Notes; align to wrapper; path rebind).
- **Judgment calls within the disposition (flagged for the independent verifier):**
  1. Added `## What Changed This Round` with a new `{{changes_this_round}}` placeholder — directed by the task/Agenda 6d ("what-changed-this-round"); HIL had no such section. Framed as realizing the *kept* self-report structure, not a new responsibility.
  2. Relabeled the counts column `Metric` → `Section` and titled the section `(optional)` — a minimal edit that realizes the trajectory-metric drop and the demotion intent.
  3. Router registration intentionally **not** performed (task scope: do not edit the router); recorded as a wiring item owned by the dedicated router edit.
- **Matched-pair guard:** P13 (`advocate-report-template`) is not yet written; this artifact establishes the canonical `{{iteration}}`→`{{round}}` text-form rename that P13 must match (reconcile Agenda 6c / §E P13). If P13 is later ported with a different round token, the pair is inconsistent — verifier should cross-check.

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Next:** `verify-output` (independent agent) grades this artifact + trace against the transform done-condition.
