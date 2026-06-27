# Transform — `spec-template.md` (P11)

**Run:** transform-cluster / `specify` · Phase 3 (transform) · **Producer:** `transform-producer`
**Skill:** `mochiko:transform-recipes` (APPLY ONLY — not graded here; `verify-output` runs independently)
**Date:** 2026-06-27 · **Disposition (finalized, Phase-2 human-gate ACCEPTED):** `port-with-edits × standalone` (header variant **b — soften**; `feature_id` rebind accepted; feature-numbering soft-drop accepted)
**Inputs:** reconcile.md §D row P11, §E P11 trace, §G template-wrapper + path rebind, Agenda 6a/6b · assess-spec-template.md · HIL source `templates/spec-template.md` · `plugins/mochiko/templates/constitution-template.md` (deliverable-template shape precedent) · `analyst-report-template.md` / `advocate-report-template.md` (cluster header convention `> Feature: {{feature_id}}`)

---

## TRANSFORM (standard format)

```
TRANSFORM: spec-template.md (P11)
Applied:   port-with-edits × standalone + convention-wiring pass
Artifacts: plugins/mochiko/templates/spec-template.md (CREATED)
New partners: none (standalone; producer↔validator pair already exists — analyst ↔ advocate)
Wiring:    classification=n/a (inert template) · router=deferred-to-router-edit (NOT touched here) ·
           triggers=n/a (no description) ·
           rebinds=[ header "Feature ID"→"Feature" (feature_id = lead-derived workspace id; create-new-feature.sh
           numbering scheme dropped) ; status semantics DAG-verdict → lead's loop done-condition ;
           created → lead-stamped ; workspace path → .mochiko/specs/<feature>/ (lead-bound at placement;
           NO path string present in the body to rewrite) ] ·
           decouple-scan=CLEAN (bare deliverable skeleton; no agent names / no "dispatch" /
           no workflow modes-paths-phases / no "workflow-agnostic" / no kernel/DAG/catalog/MCP/.humaninloop strings)
Trace (realized): see §"Realized trace" below — every responsibility carries a final tag, no silent loss
```

---

## Body edits applied (port-with-edits — header is the ONLY edit locus)

**One visible in-body edit; everything else byte-identical to the HIL source.**

1. **Header line 3: `> Feature ID: {{feature_id}}` → `> Feature: {{feature_id}}`** (variant **b — soften**). Relabel "Feature ID" → "Feature":
   - Drops the **branch-numbering / `create-new-feature.sh` `BRANCH_NAME`** identity model that "Feature ID" connoted; the value is now a **lead-derived workspace identifier** — the `<feature>` segment of `.mochiko/specs/<feature>/` (workspace-as-state, run goal #2).
   - Placeholder token `{{feature_id}}` **kept** (kept-but-rebind: slot kept, meaning rebound; numbering scheme dropped).
   - **Cluster-consistent:** identical to the realized header line in `analyst-report-template.md` and `advocate-report-template.md` (`> Feature: {{feature_id}}`). P11 follows the matched cluster convention rather than inventing a new label.
2. **`> Created: {{created}}` — kept (bare slot); rebind is population-authority only.** Semantics rebind from HIL session/context stamp → **lead-stamped at seed time** (R4). No label edit owed: "Created" is mochiko-appropriate for a deliverable, and there is no kernel string to excise. The rebind is recorded in the trace (a textually-neutral deliverable slot cannot encode "who populates it" in-body without adding guidance, which the bare-template convention forbids).
3. **`> Status: {{status}}` — kept (bare slot); semantics rebind only.** Authority moves from the **HIL DAG/workflow lifecycle + gate-verdict** → the **lead's loop done-condition / status ownership** (R5 — the headline of variant b). The DAG-verdict presumption lived in the *command* (`specify.md` "update context status to completed"), **not** in this template, so there is no DAG string in the body to remove; the soften is the rebind of authority to the lead, recorded in the trace. No inline status enum/guidance added (bare-deliverable precedent — see §"Structural / shape decision").

**Kept verbatim (unchanged):**
- **The H1 `# {{feature_title}}`** (R2 — model-derived feature name, kernel-neutral).
- **All 8 body section-slots, in order, with their `---` separators:** `## Overview` (`{{overview}}`), `## User Stories` (`{{user_stories}}`), `## Edge Cases` (`{{edge_cases}}`), `## Functional Requirements` (`{{functional_requirements}}`), `## Key Entities` (`{{key_entities}}`), `## Success Criteria` (`{{success_criteria}}`), `## Assumptions` (`{{assumptions}}`), `## Open Questions` (`{{open_questions}}`). These map 1:1 onto the analyst's ported skills (FR/edge/SC → `authoring-requirements`; stories → `authoring-user-stories`; overview/key-entities/assumptions/open-questions → analyst persona).
- **`{{mustache}}` placeholder convention** (matches the HIL source + the cluster report templates; the bracket-vs-mustache cross-template inconsistency is out of this primitive's scope per assess Check 6).

## Structural / shape decision (standalone — and why the template stays BARE)

- **Structural move = `standalone`.** One home: `plugins/mochiko/templates/spec-template.md`. Filled by one producer (`requirements-analyst`), graded by one **already-existing** validator partner (`devils-advocate`). **No split / merge / promote / absorb** — the producer↔validator pairing this template participates in already exists in the cluster, so it owes no partner primitive (contrast setup, which had to construct `constitution-validator`).
- **Kept as a BARE fill-in skeleton — NO mochiko fence-wrapper, NO `## Usage Notes`, NO inline guidance.** This is a deliberate, directed choice, not an omission:
  - §G template-wrapper convention gives **three distinct instructions**: P12 "align to the mochiko template shape," P13 "optional fence on a verbatim body," **P11 "keep bare section slots."** P11 is explicitly excluded from the wrapper P12/P13 received.
  - The task names **`constitution-template.md` as the mochiko template-shape precedent for P11** — and that deliverable template is **not** fence-wrapped; it is the document shape directly. P11 is a **deliverable document** (the actual `spec.md`), the same category as the constitution, **not** a report (analyst/advocate reports are the wrapped category).
  - assess-spec-template §Step-1: "carries **no embedded authoring guidance** — bare section slots only. Fill-guidance lives in the analyst's skills, not the template." Adding a Usage Notes block would re-introduce embedded guidance the HIL design (and the assessment) deliberately keep out.
  - Consequence for the status/created rebinds: their semantics are documented **in this trace**, not in template prose — a Usage Notes block would be the natural home, but the bare-deliverable convention forecloses it. The artifact + this trace are checkable together.

## Convention-wiring pass (all five)

1. **Classification** — **n/a.** Inert template: no `disable-model-invocation`, no `skills:` list, no trigger phrasing. (assess Check 7: templates are inert artifacts.)
2. **Router registration** — **NOT performed here** (task: "DO NOT edit the shared router"). Owned by the dedicated router edit (reconcile §G / §D router row: register `spec-template` under `/mochiko:specify`). Recorded so it is not lost. (R8.)
3. **Trigger phrasing** — **n/a** (no `description`; not a skill).
4. **Path rebinding** — the HIL body carries **no path string internally** (assess Check 6: "Path coupling — IN BODY: NONE"; grep for `.humaninloop` / `${CLAUDE_PLUGIN_ROOT}` / catalog / MCP / DAG / `.sh` returns zero). So the `specs/{feature-id}/.workflow/` → **`.mochiko/specs/<feature>/`** rebind is a **no-op in this artifact** — there was nothing to rewrite. The workspace binding (`.mochiko/specs/<feature>/spec.md`) is owned by the **lead** at placement (R6), not encoded in the bare template. Recorded `kept-but-rebind` at the convention level.
5. **Decouple persona/skill** — **CLEAN.** The template carries no persona; the body names no sibling agent, no "dispatch," no injected workflow modes/paths/phases, no "workflow-agnostic" meta-label, and (post-edit) no kernel/DAG/catalog/branch-numbering string. The lone semantic kernel-presumption ("Feature ID" = branch number) is softened by the relabel. Audited downstream by `verify-output`'s decoupling scan.

---

## Realized trace — every responsibility flipped to its final tag

| # | Responsibility (from assess §Trace / reconcile §E P11) | Realized tag |
|---|---|---|
| R1 | Define the `spec.md` shape — 8 body section slots (overview / user stories / edge cases / functional requirements / key entities / success criteria / assumptions / open questions) | **kept** (verbatim, in order, with `---` separators) |
| R1a | `{{user_stories}}` slot (filled via `authoring-user-stories` — P#, Given/When/Then) | **kept** |
| R1b | `{{functional_requirements}}` + `{{edge_cases}}` slots (filled via `authoring-requirements` — FR-XXX, edges) | **kept** |
| R1c | `{{success_criteria}}` slot (filled via `authoring-requirements` — SC-XXX, measurable) | **kept** |
| R1d | `{{overview}}` / `{{assumptions}}` / `{{key_entities}}` / `{{open_questions}}` slots (analyst persona) | **kept** |
| R2 | `# {{feature_title}}` header title (model-derived; kernel-neutral) | **kept** (verbatim H1) |
| R3 | `{{feature_id}}` header slot — HIL `create-new-feature.sh` `BRANCH_NAME` (feature-numbering) | **kept-but-rebind** — label "Feature ID" → "Feature"; value = lead-derived workspace id (`.mochiko/specs/<feature>/`); **numbering scheme dropped + reason** (accepted at soft gate, §F item 7) — capability NOT lost: workspace identity survives, only the branch-number kernel drops |
| R4 | `{{created}}` header slot — HIL session/context stamp | **kept-but-rebind** → lead-stamped at seed time (bare slot; population-authority rebind, recorded here) |
| R5 | `{{status}}` header slot — HIL DAG lifecycle / gate-verdict | **kept-but-rebind** → lead's **loop done-condition** / status ownership (semantics rebind; DAG presumption was external to the template — no body string to remove) |
| R6 | Placement — copy template into the workspace as `spec.md` (HIL: excluded `create-new-feature.sh` `cp $TEMPLATE $SPEC_FILE`) | **moved-to-lead** — the lead seeds the analyst's starting `spec.md` from this template (kernel-free `mkdir -p` + seed; reconcile §B.5 / Agenda 6b; F-2 confirmed). NOT a body change. |
| R7 | Be the **done-condition surface** the advocate validates against (8 sections present, no `{{placeholder}}` left, FR-/SC-/story presence) | **kept** — surface stays in the template; the validation *action* already lives on the advocate / the lead (pairing exists) |
| R8 | Discoverability / placement convention (router registration as a hinted template entry; bind to mochiko templates dir) | **kept-but-rebind (wiring)** — deferred to the dedicated router edit; NOT done in this artifact per task scope |

**No responsibility left untagged.** **No silent drop.** The only `dropped` element is the **feature-numbering scheme** (the `create-new-feature.sh` `BRANCH_NAME` numbering), folded into R3 with reason "workspace-as-state supersedes feature-numbering; `feature_id` rebinds to a lead-derived workspace identifier" — **accepted at the Phase-2 soft gate** (reconcile §F item 7 / §E drop table P11 R3). The workspace-identity *capability* survives (the `<feature>` slug); only the numbering kernel drops.

---

## Deviations

- **None from the finalized disposition.** Applied exactly as directed: 8 body section-slots verbatim; header variant **b — soften**; `feature_id` kept-but-rebind to a lead-derived workspace id with numbering dropped; `created` lead-stamped; `status` rebound to the lead's loop done-condition; path rebind → `.mochiko/specs/<feature>/`; placement re-homed to the lead (note-only, no body change); no structural change (standalone).
- **Judgment calls within the disposition (flagged for the independent verifier):**
  1. **Realized the variant-b soften as the single visible edit `> Feature ID:` → `> Feature:`** (placeholder `{{feature_id}}` kept). Chosen because (a) variant b requires a visible in-body soften distinguishing it from variant a (keep-verbatim), and (b) "Feature ID" was the one header label carrying the dropped branch-numbering kernel. The label matches the cluster house style (`> Feature: {{feature_id}}` in P12/P13). The `status` and `created` rebinds are population/authority shifts on textually-neutral deliverable slots, recorded in this trace rather than as label edits (no kernel string to excise; bare-deliverable convention forbids inline guidance).
  2. **Kept the template BARE — no mochiko fence-wrapper / no `## Usage Notes`** (unlike P12/P13). Directed by §G ("P11 keep bare section slots") + the task's named precedent (`constitution-template`, an unwrapped deliverable) + assess ("bare section slots only"). P11 is a deliverable-document template, not a report template. An independent verifier should confirm this is the intended class distinction and not an accidental omission of the wrapper.
  3. **Path-rebind is a no-op in the body** (the HIL spec-template contains no path string); the `.mochiko/specs/<feature>/` workspace binding is owned by the lead at placement (R6). Recorded so the absence of a path edit is not read as a missed rebind.
  4. **Router registration intentionally not performed** (task scope: do not edit the router); recorded as a wiring item owned by the dedicated router edit.

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Next:** `verify-output` (independent agent) grades this artifact + trace against the transform done-condition.
