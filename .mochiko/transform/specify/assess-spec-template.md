# Assessment — `templates/spec-template.md` (P11)

Run: `specify` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/spec-template.md`
Filled by (HIL): `requirements-analyst` (P2) body sections, via `authoring-requirements` (P7) + `authoring-user-stories` (P8); header block populated from the **excluded** `create-new-feature.sh` brain script + DAG/workflow state.
Read by (HIL): `devils-advocate` (P3) gate — consumes `spec.md` (+ `analyst-report.md`) and validates against it; the supervisor reads/owns `status` and places the file.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** What carries weight for a template: **placeholders, what consumes/fills it, and path coupling** (not loop-ownership, not persona/procedure).

This is the **canonical `spec.md` artifact shape** — a thin fill-in-the-blanks skeleton: an H1 title + a 3-line metadata blockquote (Feature ID / Created / Status) + eight `## section` headers each holding a single whole-section `{{mustache}}` slot (Overview, User Stories, Edge Cases, Functional Requirements, Key Entities, Success Criteria, Assumptions, Open Questions). It is **passive content** — copied into the workspace, filled by the producing analyst, read by the validating advocate; never executed. It is the **primary artifact the entire adversarial loop converges on** (CLUSTER THESIS).

Unlike the setup `constitution-template.md` (rich `[BRACKET]` + `<!-- INSTRUCTION -->` guidance + worked examples), this template carries **no embedded authoring guidance** — bare section slots only. Fill-guidance lives in the analyst's skills, not the template.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = NO` — the template is consumed-as-content; as a static shape it does **not** depend on a kernel, markdown supervisor, command, or DAG to *be a valid shape*. Body grep for `CLAUDE_PLUGIN_ROOT | humaninloop | hil-dag | catalog | mcp | dag | scripts/ | .sh | agent | skill` returns **zero hits** — the body is reference/path/kernel-clean. (Its HIL *placement* and *header population* were brain/DAG-driven — that is the script's/command's coupling, re-homed in Checks 1/6, not body coupling. Consistent with the `constitution-template` precedent's gate1=NO.)
2. **Multi-responsibility / fans out?** `gate2 = YES` — it has **two fillers** (the analyst fills the eight body sections; the lead fills the header metadata block) and is **validated by a third party** (the advocate reads it as the gap-analysis surface). Produced/filled by ≥2, consumed by ≥2.
3. **Emits a non-machine-checkable artifact?** `gate3 = YES` — the `spec.md` this shapes is judgment-laden (requirement quality, edge-case completeness, story testability, measurable success criteria). Correctness is graded by the `devils-advocate`'s **adversarial gap analysis** (`analysis-specifications`), not by a schema or version-equality assert. The producer↔validator pairing is real, not degenerate.

gate2 + gate3 trip → **full lens** (no fast-path).

## Step 3 — The lens (weighted for artifact branch)

**Check 1 — Orchestration test.** Orchestrated as **content**, not driven. *Content-coupling to kernel/DAG* = **NONE in the body** (zero kernel/catalog/MCP/DAG/path strings). *Orchestration-coupling owned by others* (re-home, do not edit the body):
- **Placement** — HIL's `create-new-feature.sh` (EXCLUDED, kernel-free) did `cp $TEMPLATE $SPEC_FILE` (script L294) using `${CLAUDE_PLUGIN_ROOT}/templates/spec-template.md` (L287). With the script shed, **placing the template as the analyst's starting `spec.md` re-homes to the lead** (SCOPE note: "the mochiko lead must place the template itself"). → `moved-to-lead` (R6).
- **Header population** — `{{feature_id}}` came from the script's `BRANCH_NAME`; `{{status}}` from the DAG/workflow lifecycle (specify.md Completion: "Update context status to `completed`"); `{{created}}` from session/context state. With the brain/DAG shed, **header population re-homes to the kernel-free lead**. → `kept-but-rebind` (R3/R4/R5).

No body content to re-home; the only mechanical exposure is the header **placeholder population source** (Check 6) + placement (above).

**Check 2 — Role (two altitudes).** Artifact-altitude skill-role = **emits-the-shape-a-producer-fills**. Team-role it confers = it makes its filler a **producer** (author of `spec.md`). It confers **no validator role** — grading the filled spec is the advocate's job. So this template *participates in a pairing that already exists*: the analyst (P2) produces, the advocate (P3) validates. → no new validator partner owed.

**Check 3 — Independence.** No produce+grade leak in the template (inert content). The independence boundary it sits inside is **structurally sound**: filler = `requirements-analyst` (producer agent), grader = `devils-advocate` (separate validator agent). The template neither holds nor threatens that boundary. Neutral.

**Check 4 — Verdict-sink / loop-driver.** Consumers: the analyst fills it → `spec.md`; the advocate grades the filled result (verdict `ready` / `needs-revision` / `critical-gaps`); the **lead** loops on the advocate's verdict and owns the `status` transition + the human gate. The template owns **no loop and no verdict** — but it **carries the loop's `status` field**, whose authority shifts from the HIL DAG/gate-verdict machinery to the mochiko lead's done-condition (R5, `kept-but-rebind`).

**Check 5 — Sibling / overlap ("look sideways").**
- **One filler skill-set, one shape:** the body sections map 1:1 onto the analyst's ported skills (FR/SC/edge → `authoring-requirements`; stories/P#/GWT → `authoring-user-stories`; overview/assumptions/key-entities/open-questions → analyst persona). Correct reuse of one substrate, **not** a dedupe.
- **Sibling templates** `analyst-report-template.md` (P12) + `advocate-report-template.md` (P13) are *different artifacts* (the agents' report shapes), not variants of this spec shape — **no merge/split owed**.
- **No shared validator, no trigger phrases** (templates carry no `description`/triggers). **No relational move** → no structural `flag-for-reconcile`.

**Check 6 — Coupling audit.**
- **Path coupling — IN BODY: NONE.** No `.humaninloop/`, no `${CLAUDE_PLUGIN_ROOT}`, no catalog/MCP/DAG/script path anywhere in the template. (Contrast `constitution-template`'s lone L225 path ref — this template has none.) The output location (`spec.md` in the feature workspace) is bound by the *consumer/lead*, not the template.
- **Path coupling — EXTERNAL (re-home, not a body edit):** the references that *point at* this template live in the EXCLUDED `create-new-feature.sh` (`${CLAUDE_PLUGIN_ROOT}/templates/spec-template.md`, L287) and the redesigned `specify.md` command (resource table L50–52; Initial Setup L94–96). These die with the script/DAG; placement re-homes to the lead.
- **Brain-script / DAG-state placeholders (the headline finding):** the **header metadata block** is the only brain/DAG-coupled content:
  - `{{feature_id}}` — HIL = `BRANCH_NAME` from the excluded feature-numbering/branch script. Presumes a **feature-id/branch/numbering kernel mochiko does not have**. → `kept-but-rebind` (lead-stamped) **OR** drop if workspace-as-state supersedes feature-numbering (soft lead flag F-1).
  - `{{status}}` — HIL = DAG lifecycle / gate-verdict state. → `kept-but-rebind`: authority moves to the **lead's loop done-condition**, not a DAG verdict.
  - `{{created}}` — HIL = session/context stamp. → `kept-but-rebind` (lead stamps at placement).
  - `{{feature_title}}` — model-derived from the feature description; kernel-neutral. → `kept`.
- **Body section placeholders** (`{{overview}}`, `{{user_stories}}`, `{{edge_cases}}`, `{{functional_requirements}}`, `{{key_entities}}`, `{{success_criteria}}`, `{{assumptions}}`, `{{open_questions}}`) — filled by model judgment (analyst + skills). No brain/DAG/path dependency. → `kept`.
- **Dangling references to excluded/deferred primitives — NONE in the body.** The template names no skill, agent, catalog, or script. The only dangling link is *external* (the excluded placer script), handled by the placement re-home.
- **Determinism boundary:** none in the template — pure document shape, all model-judgment to fill; the one deterministic check it enables ("no `{{placeholder}}` left unfilled") is a Tier-1 assert the validator can layer, but the artifact's substance is model-graded.
- **Placeholder-style convention:** `{{mustache}}` (matches `context-template.md`; differs from the setup `constitution-template`'s `[BRACKET]`+INSTRUCTION style — a known cross-template convention inconsistency, noted at cluster level, **not** a blocker and **not** in this template's scope to resolve).

**Check 7 — Conventions + loop placement.**
- **Classification:** templates are inert artifacts — no `disable-model-invocation`. The convention-wiring floor still applies: **router registration** (hinted template entry) + **path binding** to the mochiko templates dir on placement. No trigger phrasing (not a skill); no `skills:` (not an agent).
- **Decoupling scan:** clean — no sibling-agent names, no "dispatch," no injected workflow modes/paths/phases, no "workflow-agnostic" meta-label (templates carry no persona). Nothing for the deny-list `grep` to catch.
- **Producer↔validator pairing:** **already exists structurally** — analyst (producer agent) fills, advocate (separate validator agent) grades. Guaranteed by the cluster, not declared in the template.
- **Loop placement:** the template **supplies the done-condition surface** the advocate validates against (the eight mandatory sections + "no `{{placeholder}}` left" + presence of FR-/SC-/stories). It does **not** itself supply independent validation or a human gate — those live on the advocate / the lead. **No loop gap introduced by the template;** the gaps the lead must own (loop-driving on `needs-revision`, status authority, human gate, placement) are surfaced in Checks 1/4/6 for `reconcile-cluster`'s rehome map.

## Step 4 — Disposition

**Body × Structural = `port-with-edits` × `standalone`.**
*(Defensible fallback: `keep-verbatim` × `standalone` — see note; the choice is a lead/identity-model decision, flag F-1.)*

- **Body = `port-with-edits`.** The eight body section-slots are mochiko-clean and stay **verbatim**. The **header metadata block** is the one locus of genuine in-body brain/DAG coupling: `Feature ID` presumes the excluded feature-numbering/branch kernel, and `Status` presumes DAG/gate-verdict lifecycle. The run's headline goal is the **decoupling-doctrine test** — carrying `Feature ID`/`Status` framing forward unexamined would leave brain/DAG presumptions baked into the primary artifact the whole loop converges on. So the header earns a **localized edit** (4-line block only): redirect population to the lead, reframe `Status` around the loop's done-condition rather than a DAG verdict, and reconcile `Feature ID` with the kernel-free identity model. This is "mostly good, localized fixes; edit the specific lines; preserve structure and voice" — squarely `port-with-edits`, not `redesign`.
  - **Why not `keep-verbatim`:** strictly, the mustache slots are *textually* neutral (no literal kernel/path string to rebind), so keep-verbatim + `kept-but-rebind` trace notes is technically defensible — and is the right call **if the lead elects to retain lead-stamped header slots unchanged**. I lean `port-with-edits` because the run is explicitly the empirical decoupling test and the header's *semantics* (not just its filler) shifted off the kernel; honest porting softens that in-body rather than only annotating it. Final header treatment is gated on lead flag F-1.
  - **Why not `redesign`:** nothing in the body assumes a kernel/DAG/catalog *mechanism* to redesign around; the section shape is sound and the analyst's skills already produce exactly these sections. Minimalism governor: an edit fixes it.
- **Structural = `standalone`.** One home (`plugins/mochiko/templates/spec-template.md`), filled by one producer, validated by one **already-existing** validator partner (the advocate). No split (the producer↔validator pairing exists), no merge (siblings P12/P13 are different artifacts), no promote, no absorb. **No structural `flag-for-reconcile`.**

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | Define the `spec.md` artifact shape — eight body section slots (Overview, User Stories, Edge Cases, Functional Requirements, Key Entities, Success Criteria, Assumptions, Open Questions) the analyst fills | **kept** (verbatim) |
| R1a | `{{user_stories}}` slot (filled via `authoring-user-stories` — P#, Given/When/Then) | **kept** |
| R1b | `{{functional_requirements}}` + `{{edge_cases}}` slots (filled via `authoring-requirements` — FR-XXX, edge cases) | **kept** |
| R1c | `{{success_criteria}}` slot (filled via `authoring-requirements` — SC-XXX, measurable) | **kept** |
| R1d | `{{overview}}` / `{{assumptions}}` / `{{key_entities}}` / `{{open_questions}}` slots (filled by analyst persona; key-entities light at spec altitude — heavy entity-modeling is plan-cluster, out of scope) | **kept** |
| R2 | `{{feature_title}}` header title (model-derived from feature description; kernel-neutral) | **kept** |
| R3 | `{{feature_id}}` header slot — HIL-populated from the **excluded** `create-new-feature.sh` `BRANCH_NAME` (feature-numbering/branch/DAG-id state) | **kept-but-rebind** → lead-stamped; **OR dropped if workspace-as-state removes feature-numbering** (soft lead flag F-1) |
| R4 | `{{created}}` header slot — HIL session/context stamp | **kept-but-rebind** → lead stamps at placement |
| R5 | `{{status}}` header slot — HIL DAG/workflow-lifecycle + gate-verdict state | **kept-but-rebind** → re-homes to the lead's **loop done-condition / status ownership** (semantics shift: DAG-verdict → loop-state) |
| R6 | **Placement** — copying the template into the workspace as `spec.md` (HIL: excluded `create-new-feature.sh`, `cp $TEMPLATE $SPEC_FILE`) | **moved-to-lead** (mochiko lead places the template; soft flag F-2) |
| R7 | Be the **done-condition surface** the advocate validates against (section completeness, no `{{placeholder}}` left, FR-/SC-/story presence) | **kept** (surface stays in the template; the validation *action* already lives on the advocate/loop — pairing exists) |
| R8 | Discoverability / placement convention (router registration as hinted template entry; bind to mochiko templates dir) | **kept-but-rebind** (convention-wiring floor) |

No responsibility left untagged; **no silent drop** (R6 is a re-home to the lead, not a drop; the only conditional drop — R3's `feature_id` — is surfaced for lead acceptance via F-1, not silently removed).

## Reconcile / coupling flags

Both flags are **soft (lead/coupling decisions)** — neither requires a relational structural move; the disposition (`port-with-edits × standalone`) stands regardless. They mirror the `constitution-template` precedent's F-P7-1 style.

- **F-1 (soft, lead / identity + status model).** The header block (`{{feature_id}}` / `{{created}}` / `{{status}}`) was brain/DAG-populated. The lead must confirm the **kernel-free identity & status model** (run goal #2 — workspace-as-state) and thereby pick the header edit:
  - (a) retain lead-stamped slots **verbatim** → collapses the body treatment to `keep-verbatim`;
  - (b) **relabel/soften** (status driven by the loop done-condition, not a DAG verdict) → `port-with-edits` as proposed;
  - (c) **drop `Feature ID`** if workspace-as-state supersedes feature-numbering → `port-with-edits` (header trimmed), and R3's drop reason must be accepted at the human gate.
- **F-2 (soft, lead / placement re-home).** The EXCLUDED `create-new-feature.sh` placed this template as `spec.md`. Placement is orphaned and **re-homes to the lead** (R6 `moved-to-lead`). Confirm in `reconcile-cluster`'s rehome map that the lead (not a script) seeds the analyst's starting `spec.md` from this template, binding it to the mochiko feature workspace.

**No blocking structural flag.** The producer↔validator pairing this template participates in already exists in the cluster (analyst P2 produces ↔ advocate P3 validates). Disposition is decided; only the header-edit *variant* (F-1) and the placement re-home confirmation (F-2) await the lead.

---

### Summary block

```
ASSESSMENT: spec-template.md (P11)
Class:        template → branch artifact (canonical spec.md shape; filled by analyst P2, read by advocate P3 + lead)
Triage:       gate1=n gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone   (fallback keep-verbatim × standalone if lead retains header slots verbatim — F-1)
Trace:
  - R1  spec.md shape: 8 body section slots (overview/stories/edges/FR/entities/SC/assumptions/open-qs) → kept
  -   R1a user_stories slot (authoring-user-stories: P#, G/W/T)                                          → kept
  -   R1b functional_requirements + edge_cases slots (authoring-requirements: FR-XXX, edges)            → kept
  -   R1c success_criteria slot (authoring-requirements: SC-XXX)                                         → kept
  -   R1d overview/assumptions/key_entities/open_questions slots (analyst persona)                       → kept
  - R2  feature_title header (model-derived; kernel-neutral)                                             → kept
  - R3  feature_id header (HIL: excluded create-new-feature.sh BRANCH_NAME / feature-numbering)          → kept-but-rebind (lead-stamped) | conditional drop under workspace-as-state (F-1)
  - R4  created header (HIL session/context stamp)                                                       → kept-but-rebind (lead stamps)
  - R5  status header (HIL DAG lifecycle / gate-verdict)                                                 → kept-but-rebind (re-homes to lead's loop done-condition)
  - R6  placement / copy template→spec.md (HIL: excluded create-new-feature.sh)                          → moved-to-lead (F-2)
  - R7  done-condition surface the advocate validates against (sections, no placeholder left, FR/SC/story) → kept
  - R8  discoverability/placement convention (router reg, bind to mochiko templates dir)                  → kept-but-rebind (wiring floor)
Path coupling: IN-BODY = none (body is reference/path/kernel-clean). EXTERNAL = excluded create-new-feature.sh + redesigned specify.md referenced it → placement re-homes to lead.
Brain/DAG coupling: header block only — feature_id (brain BRANCH_NAME), status (DAG/gate-verdict), created (session) → all kept-but-rebind to the kernel-free lead.
Dangling refs to excluded/deferred: none in body (template names no skill/agent/catalog/script).
Reconcile flags:
  - F-1 (soft) header identity+status model — lead confirms workspace-as-state, picks header-edit variant (retain verbatim / soften / drop feature_id)
  - F-2 (soft) placement re-home — lead seeds analyst's starting spec.md from this template (excluded script no longer does)
  - No blocking structural flag — producer↔validator pairing already exists (analyst ↔ advocate)
```
