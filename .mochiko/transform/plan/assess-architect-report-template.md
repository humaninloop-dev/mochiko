# ASSESSMENT: architect-report-template.md (P13)

**Source:** `human-in-loop/plugins/humaninloop/templates/architect-report-template.md`
**Run:** transform-cluster / `plan` (plan-core scope) · **Assessed:** 2026-06-30
**Assessor:** `mochiko:transform-producer` (assess-only; no edit applied, no grade)
**Class:** template → **artifact branch**

---

## Header

```
Class:        template → branch ARTIFACT
Triage:       gate1=y gate2=y gate3=y  → full-lens
Disposition:  port-with-edits × flag-for-reconcile: RQ1 (reviewer architecture)
              — body treatment is itself contingent on RQ1 (see §Disposition)
Reconcile flags: RQ1 (primary — decides P13's HOST/fate)
              + cross-refs: verdict-routing → moved-to-lead (P1, regardless of RQ1);
                P15 advocate-report (RQ4) entangled under RQ1-ii
```

This is the **feasibility reviewer's report format** — the structured artifact a feasibility/cross-artifact reviewer writes (HIL: the Principal Architect, to `.workflow/architect-report.md`) to record cross-artifact contradictions, a **3-state feasibility verdict** (`feasible` / `needs-revision` / `infeasible`), per-issue evidence/impact/resolution, and the strengths noted. It is **not passive documentation**: its per-issue fields are the structured input contract for the **Feasibility Rejection Loop's `AskUserQuestion` human gate** (`commands/plan.md:626`), and its verdict is the loop-driver. Its whole fate rides on **RQ1** — does plan keep a distinct feasibility reviewer, fold feasibility into the advocate, or rehome it onto the generic binary `validator`? Flagged, not decided.

---

## Triage gate (artifact branch)

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — the report's content is consumed by the **plan supervisor's** Feasibility Rejection Loop (its per-issue fields feed `AskUserQuestion`; its verdict routes the loop), and the template literally **embeds verdict-routing** ("Next Steps", lines 75–82). It does not stand alone — a supervisor drives the review that fills it and loops on what it says. |
| 2 | Multi-responsibility / fans out? | **YES** — it carries a verdict + **three** distinct cross-artifact issue taxonomies + strengths + routing, and it fans to **two** consumers: the human gate (`AskUserQuestion`) and the lead's verdict-router. |
| 3 | Shapes a non-machine-checkable artifact? | **YES** — the feasibility report is **pure cross-artifact model judgment** (are these requirements/constraints/NFRs/decisions mutually buildable?), not a schema/version/equality check. This is *exactly* why rehoming it onto the binary `validator` (RQ1-iii) has friction — the validator degenerates to PASS/FAIL, this artifact does not. |

All three trip → **full 7-check lens** (run with artifact-branch weighting: placeholders, consumers, path coupling).

---

## Placeholder catalog (artifact-branch core)

HIL placeholder style = **single-brace** `{...}` (the plan-template family). The mochiko ported sibling reports use **double-brace** `{{mustache}}` (see `advocate-report-template.md`, `analyst-report-template.md`) — normalizing the style is a convention-wiring/transform note, not a responsibility change.

**Frontmatter (YAML):**
| Token | Role | Port note |
|-------|------|-----------|
| `type: plan-feasibility-review` | literal — DAG node-type discriminator | **drop** candidate (kernel/catalog-shed); no `type:` field on the ported advocate-report |
| `artifact: "{artifact_path}"` | placeholder — the reviewed artifact | `kept-but-rebind` (workspace-as-state path) |
| `iteration: 1` | literal — DAG pass/iteration counter | **iteration→round** (specify precedent: analyst-report "iteration→round"); demote/rebind |
| `created` / `updated: "{timestamp}"` | placeholders — provenance | `kept` (benign; matches advocate `Generated:`) |

**Body placeholders:**
- `{artifact_paths}` — Review Summary "Artifacts Reviewed" (rebind to workspace-as-state)
- `{timestamp}` — "Reviewed"
- **`{feasible | infeasible | needs-revision}`** — the **3-state verdict enum** (Review Summary)
- `{verdict}` + `{One-sentence explanation of the verdict}` — Verdict heading
- **Cross-Artifact Contradictions** row: `{artifact + ID}` ×2 · `{description of conflict}` · `{Critical / Important}` · `{specific action}`
- **Constraint-Decision Conflicts** row: `{C-XXX}: {name}` · `{D-XXX}: {name}` · `{how they conflict}` · `{action to resolve}`
- **NFR-Constraint Impossibilities** row: `{NFR-XXX}: {target}` · `{C-XXX or D-XXX}: {name}` · `{why target is unachievable}` · `{relax NFR / change decision / escalate}`
- **Strengths Noted**: `{Strength 1 …}` `{Strength 2 …}`
- Conditional empty-states: "No cross-artifact contradictions found." / "No constraint-decision conflicts found." / "No NFR-constraint impossibilities found."
- **Next Steps** (lines 75–82): verdict-routing prose — *not* a placeholder, it is **orchestration baked into the artifact** (see lens 4).

---

## Verdict-shape + per-issue structure (the load-bearing finding)

**1. The 3-state verdict is the loop-driver.** `feasible` / `needs-revision` / `infeasible` — and the **third state is the distinctive one**: `infeasible` = "a fundamental conflict requiring a business-level decision" → escalate, *not* a routine revision. This is a **different third state** from the advocate-report's `critical-gaps` (`ready`/`needs-revision`/`critical-gaps`). Any merge (RQ1-ii) or rehome (RQ1-iii) must preserve the `infeasible`→escalate semantics or it is silently flattened (silent-drop #1).

**2. The per-issue structure IS the human-gate input contract.** The Feasibility Rejection Loop reads each issue straight into `AskUserQuestion` (`commands/plan.md:626`):

```
question: "Feasibility concern: {issue.description}\n\nEvidence: {issue.evidence}\n\n
           Impact on design: {issue.impact}\n\nSuggested resolution: {issue.suggested_resolution}",
header: issue.id || "Feasibility",
```

So the conceptual per-issue tuple **{description / evidence / impact / suggested_resolution}** the brief names is sourced from the template's table columns:

| AskUserQuestion field | Sourced from (template column) |
|-----------------------|--------------------------------|
| `issue.description` | **Contradiction** ("description of conflict") |
| `issue.evidence` | **Artifact A + Artifact B + their IDs** (`{artifact + ID}` pair) |
| `issue.impact` | **Severity** (`Critical`/`Important`) — *closest field*; the literal template has no separate "impact" column, so impact is read off severity / inferred (a minor shape-gap worth tightening on port) |
| `issue.suggested_resolution` | **Suggested Resolution** ("specific action") |

**Consequence for the port:** these four per-issue fields are **load-bearing**, not cosmetic. Dropping the artifact-pair (evidence), the severity (impact), or the suggested-resolution column starves the human gate of a field it renders. The reviewer-report format must keep all four populated regardless of which RQ1 host wins. Note also the literal **column/field-name mismatch** (template: Artifact A/B + Severity; loop: evidence + impact) — on port, tighten the report shape so the four AskUserQuestion fields are first-class.

---

## 7-check lens (weighted for artifact)

**1 — Orchestration test.** *Content-coupling:* light — the body names artifact **types** generically (requirements / constraints / NFRs / decisions, `C-XXX`/`D-XXX`/`NFR-XXX` IDs), no kernel/catalog tokens **except** the frontmatter `type`/`iteration` DAG fields (drop/rebind). *Orchestration-coupling:* **strong** — the artifact is the structured fuel for the supervisor-driven Feasibility Rejection Loop, and the **"Next Steps" block embeds the loop's routing** inside the artifact. When the HIL markdown supervisor dissolves, the routing re-homes to the **plan lead (P1)** (`moved-to-lead`); the template keeps only the *format*, not the *what-happens-next*. (Consistent with the ported advocate-report having **no** Next-Steps routing section.)

**2 — Role at two altitudes.** *Artifact-role:* it is the **verdict-emitting reviewer's artifact** — a validator-side output (verdict + cited issues + resolutions). It does **not** need its own downstream validator (a reviewer's verdict is terminal; its sink is the lead's loop — no regress). *Conferred role:* whoever produces it plays a **feasibility-reviewer / validator** team-role. This is the load-bearing fact for RQ1: a 3-state, resolution-bearing reviewer artifact is *pair/merge* material, not *binary-grader* material.

**3 — Independence.** A template cannot itself leak self-grade, but it **encodes the producer↔validator boundary**: it is the *reviewer's* artifact over the **technical-analyst's** output (different agent). The independence question is *which agent* authors it — RQ1. **Guardrail:** whichever host wins, the agent that fills this report must not also be the producer of the artifacts it reviews (it never is — feasibility grades the analyst's work). No independence defect in the template; the host decision is reconcile's.

**4 — Verdict-sink / loop-driver.** The verdict is consumed by the **plan supervisor**, which drives the Feasibility Rejection Loop (present each concern via `AskUserQuestion`, route `feasible`→proceed / `needs-revision`→revise-analyst / `infeasible`→escalate-business-decision). **All of this is `moved-to-lead` (P1)** — including the "Next Steps" block currently baked in the template, and the **"review feasibility ONCE before completeness"** ordering (`plan.md:908`). The template retains the *format of the fuel*; the lead owns *what loops on it*. (Matches P3 trace #8, P4 R17.)

**5 — Sibling / overlap ("look sideways") — this is where RQ1 lives.** Three sibling relations, none resolvable from P13 alone:
- vs **advocate-report-template (P15, mochiko-ported)** — same artifact *genre* (reviewer report: issues table + 3-state verdict + strengths + clarifications), **different scope**: P13 = *cross-artifact* feasibility (Contradictions / Constraint-Decision / NFR-Constraint), P15 = *intra-artifact* completeness (Missing / Ambiguous / Edge Case / Assumption / Contradiction gaps). **Merge candidate** (RQ1-ii) → `moved-to-sibling-template`.
- vs the **generic `validator`'s** output format — the validator emits **binary PASS/FAIL + fix-list** (`validator.md:73–80`), P13 is **3-state + suggested-resolutions + AskUserQuestion-feeding**. **Contract mismatch** (RQ1-iii), detailed below.
- vs **techanalyst-report-template (P12)** — the *producer's* report (what was produced), not a reviewer artifact. Not an overlap; the designed producer→reviewer handoff.
→ **All `flag-for-reconcile`.**

**6 — Coupling audit.**
- *Paths:* HIL write path `specs/{feature-id}/.workflow/architect-report.md` (`plan.md:76, 261, 360`). Rebind: drop `.workflow/`, `{feature-id}`→`<feature>`, land under workspace-as-state `.mochiko/specs/<feature>/` — **but the concrete write location is the lead's**, not the template body's (the ported advocate-report carries no path; only `Feature:`/`Round:`). `kept-but-rebind` for the reviewed-artifact placeholders; the write-location ownership is `moved-to-lead`.
- *Prerequisite:* assumes the Phase-1 analysis artifacts exist (technical-analyst ran first) → an explicit handoff edge the lead owns.
- *Determinism boundary:* **entirely model judgment** (cross-artifact contradiction hunting). No deterministic slice → the validator that fills this is **real, not degenerate** — reinforcing the RQ1-iii friction.

**7 — Conventions + loop placement + DECOUPLING SCAN.**
- *Classification:* artifact (no user/model classification; it is a template). Placeholder-style normalization (`{}`→`{{}}`) owed at transform.
- *Discoverability:* templates are not router entries; the *consumer* (the feasibility reviewer + lead) is registered. n/a.
- *Decoupling — three hits (all `port-with-edits` fixes):*
  - **Line 11** "**Principal Architect** review of cross-artifact contradictions" → decouple to **role** ("feasibility reviewer" / "the reviewer"). Sibling/self-agent name in the artifact.
  - **Line 79** "Proceed to **Devil's Advocate** completeness review" → **sibling-agent name** (inside Next Steps; whole block is `moved-to-lead` anyway).
  - **Line 80** "Re-invoke **Technical Analyst** with contradictions above" → **sibling-agent name** (same).
  - Line 81 "Escalate to **supervisor**" — *role*, acceptable, but moves to the lead with the block.
  - Line 12 "Generated **during plan workflow**" — light workflow-name provenance; genericize/keep.
  - **Filename** `architect-report` is agent-named → rename candidate to a **work/role-named** file (`feasibility-report` / `feasibility-intersection-review`); the template's own *title* ("# Feasibility Intersection Review") is already work-named and clean.
- *Sound-loop:* the loop this artifact sits in gets its done-condition / FAIL-routing / human-gate from the **plan lead**, not the template.

---

## RQ1 — reviewer architecture: P13's responsibilities traced to each landing (FLAG-FOR-RECONCILE — analysis only)

P13's **fate** is a direct function of RQ1. Tracing the report's load-bearing responsibilities to each of the three candidate landings (the brief's core deliverable):

| Report responsibility | **(i)** keep distinct feasibility reviewer | **(ii)** fold feasibility into advocate | **(iii)** rehome onto generic `validator` |
|-----------------------|--------------------------------------------|-----------------------------------------|-------------------------------------------|
| **3-state verdict** (`feasible`/`needs-revision`/**`infeasible`**) | `kept` in P13 — ports as the feasibility-report format | `moved-to-sibling-template` → advocate-report must **reconcile `infeasible` vs its `critical-gaps`** (two different third states; preserve the `infeasible`→business-decision escalation) | **CONFLICT** — validator is binary PASS/FAIL; 3-state can't be expressed → flatten to PASS/FAIL (lossy, drops `infeasible` escalation) **or** break the validator's binary invariant |
| **Per-issue {desc/evidence/impact/resolution}** (AskUserQuestion fuel) | `kept-but-rebind` in P13 | `moved-to-sibling-template` → advocate-report's Clarifications block (Question/Options/Why-it-matters) must **gain evidence + suggested_resolution** to keep the 4 gate fields | validator fix-list ("name the item, the missing thing, the fix") has **no evidence/impact/resolution-per-issue gate shape** → human-gate fuel lost unless rebuilt |
| **Three cross-artifact taxonomies** (Contradictions / Constraint-Decision / NFR-Constraint) — the feasibility≠completeness substance | `kept` in P13 | `moved-to-sibling-template` — these tables are the **variant-unique slice** the advocate-report gains (its gap-types are intra-artifact) | `moved-to-validator` behind a **new feasibility skill** (against the binary-contract friction) or dropped |
| **Strengths Noted** | `kept` | `dedupe` with advocate's "What's Strong" (same responsibility) | n/a (validator emits no strengths) |
| **Reviewed-artifacts ref + provenance** | `kept-but-rebind` | `kept-but-rebind` (merged) | folded into validator's "Evidence read" |
| **Verdict-routing / Next-Steps** | `moved-to-lead` (P1) | `moved-to-lead` (P1) | `moved-to-lead` (P1) |

**Landing summary:**
- **RQ1-i** → **P13 ports as the feasibility-report format** (`port-with-edits × standalone`). The convention-5 two-form case: advocate-report = *checklist/completeness* artifact, this = *adversarial-critique/feasibility* artifact. plan = first cluster to ship both reviewer-report forms.
- **RQ1-ii** → **P13 merges into the advocate-report (P15)** (`merge-into-sibling`). Entangles with **RQ4** (P15 reuse-vs-extend): "extend" becomes mandatory — P15 must gain the 3 cross-artifact tables + the `infeasible` state. The deliberate feasibility≠completeness split blurs (silent-drop #2).
- **RQ1-iii** → **P13 is dropped or forces a non-binary validator** (`dropped + reason` or contract-break). See the tension below.

---

## The binary-vs-3-state tension (RQ1-iii friction — the brief's flagged conflict)

> **Real tension, noted not resolved.** The generic `validator`'s output is rigidly **binary**: "A **binary verdict** over one artifact — nothing else… `VERDICT: PASS | FAIL`… Issues requiring fix" (`validator.md:70–80`), and both its skills (`verify-output`, `validation-constitution`) emit PASS/FAIL. The architect-report is **3-state** (`feasible`/`needs-revision`/`infeasible`) **+ per-issue suggested-resolutions + an AskUserQuestion-feeding structure**.

Rehoming feasibility onto the generic validator (RQ1-iii) cannot host P13's shape without one of two lossy moves:
1. **Collapse 3→2** (`feasible`→PASS, {`needs-revision`,`infeasible`}→FAIL) — flattens the distinct **`infeasible`→business-decision escalation** into an ordinary FAIL; the human gate loses the "is this fundamentally infeasible?" branch. **Capability loss.**
2. **Add a non-binary report mode to the validator** — breaks its **binary-by-construction** invariant (its Iron Law, its "What You Produce" contract) and bleeds an adversarial-reviewer shape into a binary grader.

**Grounded data point for reconcile:** *both* plan reviewer artifacts are **3-state adversarial-reviewer reports** (architect: feasible/needs-revision/infeasible; advocate: ready/needs-revision/critical-gaps), **not** binary grades — which is structurally why RQ1-iii has friction and RQ1-i / RQ1-ii sit more naturally. (Consistent with the P3 and P4 assessments.) **Reconcile decides; I do not.**

---

## Disposition

```
port-with-edits × flag-for-reconcile   (RQ1 — reviewer architecture)
```

**Body treatment is contingent on RQ1** (stated explicitly so reconcile inherits a clean choice — mirrors the P3 pattern):
- **If RQ1-i (distinct feasibility reviewer):** body = **`port-with-edits`** — rebind paths (`.workflow/` drop, `{feature-id}`→`<feature>`, workspace-as-state), drop/demote DAG frontmatter (`type` drop, `iteration`→`round`), normalize placeholder style (`{}`→`{{}}`), decouple the agent names (line 11 → role; rename file to `feasibility-report`), and **lift the "Next Steps" routing to the lead** (P1). Structural = `standalone`.
- **If RQ1-ii (fold into advocate):** body collapses; P13's unique slice (3 cross-artifact tables + `infeasible` state + the 4-field gate fuel) is **`moved-to-sibling-template`** into the advocate-report (P15) → structural = `merge-into-sibling`. Entangled with RQ4.
- **If RQ1-iii (generic validator):** P13's shape **conflicts** with binary PASS/FAIL → either `dropped + reason` (lossy, see tension) or a new feasibility skill on the validator carrying a non-binary report (contract-break). Structural = `rewire-cluster`/`drop`.

Either way the structural move is **`flag-for-reconcile`**. The body recipe cannot be finalized until RQ1 picks the host.

---

## Responsibility trace (complete — every responsibility tagged)

| # | Responsibility (HIL template holds) | Tag | Where / note |
|---|--------------------------------------|-----|--------------|
| T1 | **3-state feasibility verdict** (`feasible`/`needs-revision`/`infeasible`) — the loop-driver value | **`flag-for-reconcile`** | host per RQ1 (see landing table). Preserve all 3, esp. `infeasible`→escalate (silent-drop #1). |
| T2 | **Per-issue {description/evidence/impact/suggested_resolution}** — `AskUserQuestion` fuel (`plan.md:626`) | **`flag-for-reconcile`** (travels w/ T1) | **load-bearing human-gate input contract**; keep all 4 fields; tighten the column↔field mapping on port. |
| T3 | **Three cross-artifact taxonomies** (Contradictions / Constraint-Decision / NFR-Constraint) — feasibility≠completeness substance | **`flag-for-reconcile`** | the variant-unique slice; `kept`(i) / `moved-to-sibling-template`(ii) / new-validator-skill(iii). |
| T4 | **Strengths Noted** | `kept` / `dedupe` | same responsibility as advocate-report "What's Strong"; dedupe on merge (RQ1-ii). |
| T5 | **Reviewed-artifacts reference** (`{artifact_path}` / `{artifact_paths}`) | `kept-but-rebind` | rebind to workspace-as-state. |
| T6 | **Write-location coupling** (`specs/{feature-id}/.workflow/architect-report.md`) | `kept-but-rebind` **+ `moved-to-lead`** | drop `.workflow/`, `{feature-id}`→`<feature>`; the concrete write *location* is the lead's (template stays path-agnostic, like the ported advocate-report). |
| T7 | **DAG/state frontmatter** (`type: plan-feasibility-review`, `iteration: 1`) | `iteration`→`kept-but-rebind` (iteration→round); `type`→**`dropped + reason`** | `type` = DAG node-type discriminator (kernel/catalog-shed); `iteration` follows the specify analyst-report "iteration→round" precedent. |
| T8 | **Provenance timestamps** (`created`/`updated`/`Reviewed`) | `kept` | benign metadata; matches advocate-report `Generated:`. |
| T9 | **Verdict-routing "Next Steps"** (feasible→advocate / needs-revision→re-invoke analyst / infeasible→escalate) | **`moved-to-lead`** (P1) | the Feasibility Rejection Loop routing + "review-once-before-completeness" ordering belong to the supervisor, not the artifact. Carries the line 79/80 sibling-name decouples with it. |
| T10 | **"Principal Architect" self-reference** (line 11) | `kept-but-rebind` (decouple) | → role ("feasibility reviewer"); filename `architect-report`→`feasibility-report`. |
| T11 | **"Generated during plan workflow" provenance** (line 12) | `kept-but-rebind` (light decouple) | genericize or keep as benign provenance. |

**No untagged responsibility remains → trace is complete (assessment done-condition met). No bare `dropped` — T7's `type` carries its reason (DAG node-type discriminator).**

---

## Silent-drop risks (the assess role's primary alarm)

1. **#1 — the `infeasible` verdict state (CRITICAL).** It is the one state with a *different consequence* (business-level decision / escalate, not a routine revision). RQ1-ii (fold into advocate, whose third state is `critical-gaps`) or RQ1-iii (binary collapse) can flatten it silently. **Preserve all three states and the `infeasible`→escalate route.**
2. **#2 — the feasibility≠completeness division of labor.** P13's three *cross-artifact* taxonomies are deliberately distinct from the advocate's *intra-artifact* gap types. A careless RQ1-ii merge blurs two sharp adversarial lenses into one weaker review. The cross-artifact tables are the variant-unique slice — keep them as a branch, don't dissolve them.
3. **#3 — the per-issue 4-field gate fuel.** {description/evidence/impact/suggested_resolution} is the literal `AskUserQuestion` payload (`plan.md:626`). Dropping the artifact-pair (evidence), severity (impact), or suggested-resolution column starves the human gate. Must survive every RQ1 host.
4. **#4 — the verdict-routing must land on the lead, not vanish.** "Next Steps" (T9) is orchestration baked in the artifact; it is `moved-to-lead` (P1), **not** dropped with the template — the Feasibility Rejection Loop + the "once-before-completeness" ordering (`plan.md:908`) are the supervisor's to own (cross-ref P3 #8, P4 R17).

---

## Reconcile flags handed to `reconcile-cluster`

1. **RQ1 — reviewer architecture (PRIMARY).** Decides P13's **host/fate**: (i) **port as the feasibility-report format** (`port-with-edits × standalone`; convention-5 two-form, alongside the advocate's checklist report) · (ii) **merge into the advocate-report (P15)** (`merge-into-sibling`; the cross-artifact tables + `infeasible` state become P15's variant slice — entangles RQ4) · (iii) **rehome onto the generic `validator`** (binary-vs-3-state **contract mismatch** — `dropped + reason` or validator contract-break). Needs P3 (PA reviewer role / RQ2), P4 (advocate), P9 (`validation-plan-artifacts`), P15 (advocate-report / RQ4) in view. Human-gated (contract §4a).
2. **Cross-ref — verdict-routing is `moved-to-lead` (P1) regardless of RQ1.** The "Next Steps" routing, the Feasibility Rejection Loop, and the "review-feasibility-once-before-completeness" ordering re-home to the plan supervisor. Recorded so they are not dropped with the HIL markdown supervisor.
3. **Cross-ref — RQ4 (P15 advocate-report reuse-vs-extend) is downstream of RQ1.** If RQ1-ii, P15's "reuse as-is" is off the table — it must **extend** to host feasibility's tables + `infeasible` state. Resolve RQ1 first.
4. **Grounded data point:** both plan reviewer artifacts are **3-state adversarial-reviewer reports**, not binary grades → RQ1-i / RQ1-ii sit naturally, RQ1-iii has structural friction.

**Open flags after this assessment: RQ1 (intentionally unresolved — reconcile owns it).**

---

**Assessment version:** v1 · **Governed by:** `loop-discipline` · **Disposition:** port-with-edits × flag-for-reconcile (RQ1) · **Role:** assess/diagnose only — no edit, no grade applied.
