# TRANSFORM: architect-report-template.md → feasibility-report-template.md (P13)

**Run:** `/mochiko:transform-cluster` (plan-core) · **Phase 3 (transform)** · **Date:** 2026-06-30
**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes`
**Role:** apply the finalized disposition + run the convention-wiring pass. **NOT** grade (independence → `verify-output`, different agent).
**Disposition (finalized, reconcile §RQ1 option i):** `port-with-edits × standalone`, **RENAMED → `feasibility-report-template.md`**.
**Inputs:** `assess-architect-report-template.md` · `reconcile.md` (P13 row L272–273; §RQ1; Job 2 #2/#15) · HIL `templates/architect-report-template.md` · sibling `plugins/mochiko/templates/advocate-report-template.md` (+ `analyst-report-template.md`).

---

## Output block (transform-recipes format)

```
TRANSFORM: architect-report-template → feasibility-report-template
Applied:   port-with-edits × standalone + wiring-pass  (RENAME applied)
Artifacts: plugins/mochiko/templates/feasibility-report-template.md (created)
           human-in-loop/.../architect-report-template.md (source; untouched — reference submodule)
New partners: none (standalone). The CONSUMER skill validation-feasibility (NEW, reconcile Job 2b)
              is produced by a separate transform; this template is its report FORMAT only.
Wiring:    classification=template (wiring floor — no user/model tag; consumer carries it)
           router=NOT edited (note below — entry owed by the consuming skill, not this template)
           triggers=n/a (template has no auto-trigger description)
           rebinds=[ .workflow/ path dropped; {feature-id}→<feature>; workspace-as-state;
                     write-location → lead-owned (path-agnostic body) ]
           single-source=verdict-routing referenced to the lead's loop, not restated;
                         no loop-discipline doctrine inlined
Trace (realized): see table below — all 11 responsibilities tagged, zero silent loss
```

---

## Body recipe applied (`port-with-edits`)

| # | Edit | Result |
|---|------|--------|
| B1 | **Rename** file `architect-report-template.md` → `feasibility-report-template.md`; report title `Feasibility Intersection Review` → role title **`Feasibility Review Report`**; file-doc title `# Feasibility Review Report Template` (sibling pattern). | agent-named filename decoupled to role/work name. |
| B2 | **Preserve the 3-state verdict** `feasible / needs-revision / infeasible` as a `## Verdict` section (sibling shape: Status / Verdict options / Rationale). `infeasible` semantics spelled out in Usage Note 2 with an explicit **do-not-collapse** guard. | silent-drop #1 (the `infeasible`→business-escalation) defended. |
| B3 | **Preserve the 3 cross-artifact taxonomies** (Cross-Artifact Contradictions / Constraint-Decision Conflicts / NFR-Constraint Impossibilities) as three tables + empty-state notes. | silent-drop #2 (feasibility≠completeness) defended; Usage Note 4 keeps the lens distinct from completeness by role. |
| B4 | **Tighten the column↔gate-field mismatch.** Uniform table shape across all three taxonomies: `# / Description / Evidence / Impact / Severity / Suggested Resolution`. **Impact promoted to a first-class column** (HIL inferred it off Severity — the assess shape-gap); **Evidence labeled explicitly** as the cited conflicting pair (`A ↔ B`); Severity retained as the triage signal (was only on the Contradictions table in HIL; now uniform, matching the sibling severity convention). Usage Note 5 states the 4-field gate alignment one-to-one. | silent-drop #3 (the 4-field gate fuel) defended; the `AskUserQuestion` payload (`plan.md:626`) now reads each field by its own column with no inference. |
| B5 | **Placeholder style `{ }` → `{{ }}`** throughout (mochiko sibling-report convention — like advocate-report/analyst-report; UNLIKE the `[...]` deliverable templates). | convention-aligned with the report family. |
| B6 | **Drop the "## Next Steps" verdict-routing block** (HIL L75–82) from the body. The Feasibility Rejection Loop routing + the review-feasibility-once-before-completeness ordering → the command lead. | T9 `moved-to-lead`; carries the L79 "Devil's Advocate" / L80 "Technical Analyst" sibling-name decouples out with it. |
| B7 | **Drop the YAML frontmatter** (`type` / `artifact` / `iteration` / `created` / `updated`); replace with the sibling `>` provenance lines (`Feature` / `Round` / `Generated`). `type:` dropped (DAG node-type discriminator); `iteration`→`round`; reviewed-artifact ref folded into the body `## Artifacts Reviewed` section. | matches advocate-report/analyst-report (no frontmatter); kernel/DAG shed. |

---

## Convention-wiring pass (all 6 — every port pays the floor)

1. **Classification** — *template → wiring floor.* Templates carry no `user-invoked`/`model-invoked` tag and no `disable-model-invocation`; classification rides on the **consumer**. The consumer is the NEW `validation-feasibility` skill (reconcile Job 2b), mounted on the re-broadened `principal-architect`; the lead seeds/collects the artifact. Nothing classification-bearing owed by the template itself. ✔
2. **Router registration** — **NOT edited (per instruction).** Templates are not router entries. *Router entry owed (by a later transform, not here):* the consuming skill **`validation-feasibility`** registers in the `mochiko` router as the feasibility reviewer's driver (model-invoked, agent-consumed; when-to-reach = "cross-artifact feasibility / buildability / contradiction review of plan analysis+design artifacts"). No `plugin.json` / shared-router edit made by this port. ✔ (noted)
3. **Trigger phrasing** — n/a (a template has no auto-trigger `description`). The consuming skill owns graded triggers. ✔
4. **Path rebinding** — HIL `specs/{feature-id}/.workflow/architect-report.md` → **body made path-agnostic** (no write path inside the template, like advocate-report). Reviewed-artifact placeholders rebind to **workspace-as-state**; the lead-owned write location `.mochiko/specs/<feature>/feasibility-report.md` is documented in Usage Note 6 and attributed to the lead (mirrors analyst-report Usage Note 6). `.humaninloop/`→`.mochiko/` applied to any path language. ✔
5. **Decouple persona/skill** — `Principal Architect` (HIL L11) → **role** ("the feasibility reviewer"); `Devil's Advocate` (L79) + `Technical Analyst` (L80) sibling names **lifted out** with the Next-Steps block; "Generated during plan workflow" (L12) workflow-name provenance **genericized** to `> Generated: {{timestamp}}`. No agent names, no "dispatch", no workflow modes/paths/phases, no "workflow-agnostic" meta-label remain. Independence stated by *role*. ✔
6. **Single-source / de-duplicate** — the verdict-**routing** is *referenced* to the command lead's feasibility loop (Usage Note 6), not restated in the artifact; no `loop-discipline` doctrine (the four guards, validator tiers, done-condition mechanics) inlined; the 3-state verdict + the 4-field gate shape are the report's own contract (not doctrine), so they stay. ✔

---

## Realized responsibility trace (every responsibility flipped to its final tag — assess T1–T11)

| # | Responsibility (HIL template held) | Assess tag | **Realized tag** | Where it landed in the artifact |
|---|-------------------------------------|-----------|------------------|----------------------------------|
| T1 | **3-state feasibility verdict** (`feasible`/`needs-revision`/**`infeasible`**) | flag-for-reconcile | **`kept`** | `## Verdict` section; all three states; `infeasible`→business-escalation preserved with do-not-collapse guard (Usage Note 2). |
| T2 | **Per-issue {description / evidence / impact / suggested_resolution}** (`AskUserQuestion` fuel) | flag-for-reconcile | **`kept-but-rebind`** | first-class columns in all 3 tables; alignment tightened (Impact promoted, Evidence labeled); Usage Note 5 maps each to the gate field 1:1. |
| T3 | **Three cross-artifact taxonomies** (Contradictions / Constraint-Decision / NFR-Constraint) | flag-for-reconcile | **`kept`** | three tables + empty-state notes; Usage Note 4 keeps the lens distinct from completeness. |
| T4 | **Strengths Noted** | kept / dedupe | **`kept`** | `## Strengths Noted` section (under (i) the report stands alone; no merge → no dedupe needed). |
| T5 | **Reviewed-artifacts reference** (`{artifact_paths}`) | kept-but-rebind | **`kept-but-rebind`** | `## Artifacts Reviewed` section, workspace-as-state, `{{artifacts_reviewed}}` — path-agnostic. |
| T6 | **Write-location coupling** (`specs/{feature-id}/.workflow/architect-report.md`) | kept-but-rebind + moved-to-lead | **`kept-but-rebind` + `moved-to-lead`** | body carries no path; lead-owned write location `.mochiko/specs/<feature>/feasibility-report.md` documented in Usage Note 6 (reconcile Job 2 #15). |
| T7 | **DAG/state frontmatter** (`type:`, `iteration:`) | type→dropped+reason; iteration→kept-but-rebind | **`type` → `dropped + reason`** (DAG node-type discriminator — kernel/catalog shed; reconcile Job 2E) · **`iteration` → `kept-but-rebind`** (→ `> Round: {{round}}`) | frontmatter removed; `round` provenance line. |
| T8 | **Provenance timestamps** (`created`/`updated`/`Reviewed`) | kept | **`kept`** | collapsed to `> Generated: {{timestamp}}` (sibling convention). |
| T9 | **Verdict-routing "Next Steps"** (feasible→completeness / needs-revision→re-produce / infeasible→escalate) | moved-to-lead | **`moved-to-lead`** | block removed from body; routing + feasibility-once-before-completeness ordering reference the lead (Usage Note 6). Carries the L79/L80 sibling-name decouples. |
| T10 | **"Principal Architect" self-reference** (L11) + agent-named filename | kept-but-rebind (decouple) | **`kept-but-rebind`** | → role "the feasibility reviewer"; file renamed `architect-report` → `feasibility-report`. |
| T11 | **"Generated during plan workflow" provenance** (L12) | kept-but-rebind (light decouple) | **`kept-but-rebind`** | workflow name dropped; genericized to the `Generated` timestamp provenance. |

**Homeless-responsibility check: none.** Every responsibility carries a realized tag. The only `dropped` is T7's `type:` frontmatter, which carries its reason (DAG node-type discriminator — no DAG in mochiko). No bare drop.

---

## Confirmations requested by the brief

- **`infeasible` PRESERVED** ✔ — kept as the distinct third verdict state with its **business-level-decision / escalation** consequence; Usage Note 2 carries an explicit *do-not-collapse* guard (against flattening to `needs-revision`, `feasible`, or a generic FAIL). Silent-drop #1 defended.
- **4-field gate fuel PRESERVED + TIGHTENED** ✔ — `Description / Evidence / Impact / Suggested Resolution` are first-class, uniform columns across all three taxonomy tables; **Impact promoted** from being inferred off Severity; **Evidence labeled** as the cited pair. Usage Note 5 documents the one-to-one map to the `AskUserQuestion` payload (`plan.md:626`). Silent-drop #3 defended.
- **Next-Steps LIFTED-TO-LEAD** ✔ — the `## Next Steps` verdict-routing block (HIL L75–82) is removed from the artifact; the Feasibility Rejection Loop routing + the review-once-before-completeness ordering belong to the command lead (reconcile Job 2 #2). The L79 "Devil's Advocate" / L80 "Technical Analyst" sibling-name decouples were lifted out with it.
- **RENAME applied** ✔ — file `feasibility-report-template.md`; report title `Feasibility Review Report`; the `validation-feasibility` reviewer's report format under reconcile §RQ1 option (i).

---

## Handoff to `verify-output` (run by a DIFFERENT agent — independence)

- **Artifact to grade:** `plugins/mochiko/templates/feasibility-report-template.md`
- **Trace to audit:** this file.
- **Checkpoints:** five conventions (classification floor / router-note-only / no trigger owed / path rebind / decouple) + single-source (routing referenced, not restated) + the trace audit for silent loss (esp. the three named silent-drop risks: `infeasible`, the 3 taxonomies, the 4-field gate fuel).
- **Open item for the lead (not a defect):** the router entry for the consuming `validation-feasibility` skill is **noted, not made** here (shared-router edit deliberately deferred per instruction).

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Disposition applied:** `port-with-edits × standalone` (rename) · **Role:** apply + wire only — no grade (independence reserved for `verify-output`).
