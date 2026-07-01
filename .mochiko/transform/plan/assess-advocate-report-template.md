# ASSESSMENT: advocate-report-template.md (P15)

**Source (read-only):** `human-in-loop/plugins/humaninloop/templates/advocate-report-template.md`
**Current port:** `plugins/mochiko/templates/advocate-report-template.md` (ported for `specify`, 2026-06-28 — was P13 that run; `keep-verbatim × standalone`)
**Run:** transform-cluster / `plan` (plan-core scope) · **Assessed:** 2026-06-30
**Assessor:** `mochiko:transform-producer` (assess-only; no edit applied, no grade)
**Mode:** **DELTA assessment** — the template is ALREADY PORTED; this assesses only whether the `plan` cluster requires any change to it (reuse-as-is vs extend), against plan's actual completeness-review usage.

---

## Header

```
Class:        template → branch artifact
Triage:       gate1=n gate2=y gate3=y  → full-lens (run light — delta on an already-ported artifact)
Disposition:  keep-verbatim × standalone   (REUSE the existing specify port unchanged; plan run emits NO new file)
              + flag-for-reconcile: RQ4 (reuse-as-is vs extend — relational, human-gated; my input: REUSE)
Reconcile flags: RQ4 (primary) + 2 coordination edges (RQ1-coupling; P9/RQ3 home for the consistency-matrix)
```

The template is the **completeness reviewer's report shape** for `plan` — the same role it plays for `specify`, with a different review *procedure* feeding it. In `plan` the `devils-advocate` fills it twice (Phase 2 analysis-completeness; Phase 3 design-review-incremental) via `validation-plan-artifacts` (P9); in `specify` the same agent fills it via `analysis-specifications`. One report shape, two review skills, two workflows — the verdict/gap/clarification contract is identical in both.

---

## The delta in one line

> **REUSE the specify port AS-IS — no body change, no new file.** The plan completeness review emits the **same** 3-state verdict (`ready`/`needs-revision`/`critical-gaps`), the **same** Critical/Important/Minor severity model, and gap categories that **fold cleanly into the existing taxonomy** (Missing/Ambiguous/Edge Case/Assumption/Contradiction). HIL itself proves the reuse: `commands/plan.md` binds **both** plan review sites (L435 analysis, L586 design-incremental) to `templates/advocate-report-template.md` — the identical template specify uses. The only plan-only candidate (the incremental-mode Cross-Artifact Consistency Pass/Fail matrix) is **not a template field** — HIL homes it in the **P9 skill** (`validation-plan-artifacts` report-format guidance), and its substantive failures already route into **Gaps Found** as Contradiction-type gaps.

What is **unchanged** (carried verbatim from the specify port, re-confirmed against plan usage):
- The five-block body — header / Gaps Found (+ severity & gap-type taxonomies) / Clarifications Needed / Verdict (3-state) / What's Strong.
- The verdict vocabulary `ready` / `needs-revision` / `critical-gaps` — **identical** to `validation-plan-artifacts`'s Verdict Criteria table (P9 SKILL.md L147-153).
- The Critical / Important / Minor severity model — **identical** to P9's Issue Classification table (P9 SKILL.md L44-52).
- The `{{mustache}}` placeholder convention; the title + ```markdown fence + Usage Notes packaging the specify port added.
- Body decoupling: zero deny-list tokens (re-confirmed below) — names no filler agent, no DAG, no path.

What **changes** (the plan delta — none of it a body edit):
- **Rehome target flips to the plan lead (P1)** — the verdict-consumption / clarification-feed / gap-routing that drive the loop now land on the **plan** supervisor, not the specify lead. The template is the *origin surface* only; carries none of it.
- **Cross-cluster reuse is now REALIZED** — the specify assessment tagged this "reuse later, no re-port"; `plan` is the first later cluster to actually reuse it. Confirmed: reuse, not re-port → **no new artifact**.
- **REGISTRY re-file** — the row should broaden `specify` → **shared (specify + plan)**; a wiring/reconcile step, not a body edit.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight on placeholders, what consumes it, and path coupling (NOT loop-ownership, NOT persona/procedure). Unchanged from the specify assessment; class does not change on re-use. As in specify, this artifact is unusual in that its **verdict field is the loop's input surface**, so Check 4 (verdict-sink) carries weight an inert template normally wouldn't — but the *consumption* sits on the lead, not the template body.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict (plan delta) | Same as specify? |
|------|---|----------------------|------------------|
| 1 | Orchestration-coupled? | **NO** — inert artifact-shape; functions standalone (an agent/human fills it without a kernel). The dependency is *inverted*: the lead reads the verdict FROM the report; the report does not depend on the lead. | yes (was gate1=n) |
| 2 | Multi-responsibility / fans out? | **YES** — five slot-responsibilities; fans to three consumers: the **plan lead** (reads verdict to converge/revise/escalate), the **human gate** (reads clarifications), the **producer** `technical-analyst` (reads gaps to revise on `needs-revision`). | yes (consumers re-targeted to the plan cluster) |
| 3 | Non-machine-checkable artifact? | **YES** — whether the gaps are real, the verdict justified, the clarifications well-posed is all model judgment, not a schema/equality assert. | yes |

gate2 + gate3 trip → **full lens**, run *light* (delta on an artifact already full-lensed in specify; checks unchanged are confirmed against plan usage, not re-derived).

## Step 3 — The lens (weighted for artifact branch; delta-focused)

**Check 1 — Orchestration test.** Orchestrated as **content**, not driven — re-confirmed for plan.
- **Content-coupling = NONE.** Deny-list scan of the current mochiko port (`dispatch | workflow-agnostic | state-analyst | devils-advocate | technical-analyst | principal-architect | requirements-analyst | validator | DAG | catalog | hil-dag | brain | .humaninloop | .workflow | pass_number | node`) → **CLEAN, zero hits.** The template names no filler agent and no DAG consumer; it is already fully decoupled. Reusable across clusters precisely because it is content-clean.
- **Orchestration-coupling = the consumers, which now rehome to the PLAN lead (P1).** The verdict-extraction / gate-routing that HIL's `state-analyst` + DAG owned re-home onto the plan supervisor (P1, `redesign`) — exactly as they did onto the specify lead, only the target changes. None of that lives in the body → **nothing in the body to soften.** No body work owed on coupling grounds.

**Check 2 — Role (two altitudes).**
- *Artifact-shape role:* **emits-the-shape-the-critic-fills** — the validator-side report shape (the analog of `techanalyst-report-template`'s producer-side shape, P12).
- *Team-role conferred:* makes its filler a **VALIDATOR / completeness reviewer**, and uniquely carries the **loop-driver field** (the verdict) the lead-referee consumes. *Delta:* in plan the filler invokes a **second** procedure skill (`validation-plan-artifacts`) instead of `analysis-specifications`, but the *report shape it produces is the same* — the skill changes the *thinking*, not the *artifact contract*. The producer↔validator pairing this shape participates in already exists structurally in the plan cluster (analyst produces / advocate critiques / lead referees) → no pairing owed by this template.

**Check 3 — Independence.** No produce+grade leak in the template (neutral content). Independence is enforced one level up — the `devils-advocate` that fills this is a different agent from the `technical-analyst` producer, with disjoint skills (confirmed in `assess-devils-advocate.md` R15). Template is neutral; carries no independence risk.

**Check 4 — Verdict-sink / loop-driver (load-bearing — and the crux of "does plan need more?").**
- **The verdict is THE loop driver, and plan's verdict vocabulary is IDENTICAL to specify's.** P9 `validation-plan-artifacts` SKILL.md L147-153 defines exactly `ready` / `needs-revision` / `critical-gaps` with the same converge/revise/escalate semantics. So the loop-fuel shape the plan lead consumes is **the same contract** the specify lead consumed — no new verdict states, no new field for P1 to parse.
- **Mochiko rehome (1:1, now onto P1):** `ready` → plan lead converges the phase → next phase / human acceptance gate; `needs-revision`/`critical-gaps` → plan lead opens a bounded round (the Clarification Loop, plan L461-469) capped per contract. The DAG-gate + state-analyst parse machinery dissolve; the **plan lead reads the verdict directly** from the filled report → `moved-to-lead` (now P1). The template owns the verdict *field*, not the loop that consumes it.
- **Two fill-events, one shape.** Plan fills this template **twice** in one run (Phase 2.7 analysis-completeness; Phase 3.5 design-review-incremental), overwriting `.../advocate-report.md` each time. HIL drives BOTH through `advocate-report-template.md` (L435, L586) with no phase field — the lead disambiguates by dispatch context, not by reading a phase from the report. The template demonstrably already serves multi-phase plan usage in HIL; the mochiko port inherits that fitness. Strong evidence for reuse-as-is.

**Check 5 — Sibling / overlap ("look sideways") — where the plan-specific candidates surface.**
- **Does plan's completeness review emit anything this shape doesn't carry?** Three candidates examined:
  1. **Plan gap categories** (FR-coverage, orphan-TR, untestable-criteria, entity-coverage, schema-match, data-sensitivity, endpoint-coverage, NFR-measurability, integration-boundary) — these all **fold into the existing gap-type taxonomy**: "FR not mapped to a TR" = `Missing`; "schema doesn't match the data model" = `Contradiction`; "NFR not measurable" = `Ambiguous`; "orphan TR" = `Assumption`/`Missing`. **No new gap category needed.** The plan-specific *checks* live in the P9 skill (its phase checklists); their *findings* land in the existing Gaps Found table. → carried.
  2. **Incremental-mode Cross-Artifact Consistency Pass/Fail matrix** + "New-vs-Previous artifacts" Review Summary (P9 SKILL.md L109-136). This is the one genuine shape the template lacks — BUT it is **not a template responsibility**: (a) HIL homes it in the **P9 skill's** report-format block, not the template, and the plan command still binds the artifact to `advocate-report-template.md` *even in incremental mode* (L586); (b) the substantive consistency **failures** already route into Gaps Found as `Contradiction`/`Missing` gaps — no capability lost; (c) only the *affirmative* "I checked X and it passed" matrix is unrepresented, which specify's template also omits (affirmatives go in prose under What's Strong). If mochiko wants the affirmative matrix recorded, its home is **P9** (the skill) and/or **P14** (`cross-artifact-checklist`, RQ3) — **not** an extension of P15. → flagged, not owed.
  3. **Phase disambiguation** (analysis vs design report) — handled by the lead's dispatch context; HIL carries no `{{phase}}` field either, so it is not a capability the port would *lose*. An informational header line is at most a reconcile nicety. → not owed.
- **vs the feasibility reviewer's report (architect-report-template, P13 this run):** complementary, not duplicate — P15 = completeness reviewer's shape (gaps + 3-state verdict); P13 = feasibility reviewer's shape. Different fillers, different findings. The fate of P13 is tied to RQ1; the fate of P15 is **largely independent of RQ1** (see Reconcile flags) → no merge owed.
- **Cross-cluster reuse:** the same template is referenced by `plan` / `tasks` / `techspec` in HIL — a **shared substrate, ported once (specify), reused in multiple workflows.** Plan is the first realized reuse. → correct single-source reuse, not a split/dedupe, not a re-port.

**Check 6 — Coupling audit.**
- *Hardcoded paths in the body:* **NONE** — confirmed. The output location `.../advocate-report.md` and the `${CLAUDE_PLUGIN_ROOT}/templates/...` reference live in the **consumers** (the plan command, the plan-context-template P11), not in this body. The `specs/.../.workflow/` → `.mochiko/` workspace rebind is owed by **P1/P11**, surfaced here so it is not lost — but no path in *this* body to rebind.
- *Grounding rebinds (who populates fields, not the body):* `{{feature_id}}` → from the **plan lead's** workspace/feature naming (HIL's excluded `create-new-feature.sh` is gone); `{{round}}` → the **plan lead's** bounded-loop round counter (the specify port already renamed `iteration`→`round`, which generalizes cleanly to plan's clarification/revision rounds). Both `kept-but-rebind`, both executed by the lead/wiring pass, **neither a body edit**.
- *Determinism boundary:* none in the template — pure document shape; all model-judgment to fill.
- *Placeholder convention:* `{{mustache}}` — unchanged; no normalization owed.

**Check 7 — Conventions + loop placement + decoupling scan.**
- *Classification:* inert artifact — no `disable-model-invocation`, no trigger phrasing. Convention-wiring floor = **router/REGISTRY registration** (broaden the entry to shared specify+plan) + path-rebind audit (**none owed in body**). The specify port already carries the title + ```markdown fence + Usage Notes packaging; no re-packaging owed.
- *Decoupling scan:* **CLEAN** (Check 1) — zero deny-list tokens; nothing to scrub. Re-confirmed against the plan deny-list (added `technical-analyst`, `principal-architect`, `validation-plan-artifacts` to the scan) → still zero hits.
- *Loop placement:* the template supplies the **verdict surface** (the loop's input) and the **clarification surface** (the human gate's input). It introduces no loop gap; the done-condition / independent-validation / human-gate live on the **plan lead** + the contract. The verdict-consumption rehome (R5, now onto P1) is what makes the plan sound-loop work — owned by reconcile/P1, not the template.

## Step 4 — Disposition

**Body × Structural = `keep-verbatim` × `standalone`** (REUSE the existing specify port; the plan run produces **NO new file** for P15)
**+ `flag-for-reconcile: RQ4`** (reuse-as-is vs extend — human-gated per contract §4a; my input: **REUSE AS-IS**).

- *Body = `keep-verbatim`.* Re-stated as a **reuse** for the delta: I cannot name a single body line that *must* change for plan correctness. The verdict vocabulary, severity model, gap taxonomy, clarification shape, and strengths slot all carry plan's completeness-review output unchanged — and HIL itself binds plan's two review sites to this exact template. Adding plan-only prose (a consistency matrix, a phase field) would either (a) re-couple the template to a specific workflow, rejected by decoupling-doctrine, or (b) duplicate a shape that belongs in P9 — so the minimalism governor (`keep-verbatim` > `port-with-edits`) holds firmly. **No edit owed; reuse the artifact as it stands.**
- *Structural = `standalone`.* One shared home (`plugins/mochiko/templates/advocate-report-template.md`), filled by the advocate, read by the plan lead. No split/merge/promote/absorb. Cross-cluster reuse (specify now + plan + later tasks) is correct single-source reuse, not a structural move. `standalone` holds in **all three RQ1 branches** (the advocate is the completeness reviewer regardless of how feasibility is homed).
- *Why a `flag-for-reconcile` rather than a clean self-decided reuse:* the reuse-vs-extend call is in the contract's **human-gated bundle** (§4a: "P15 `advocate-report-template` reuse") and touches two sibling relations — whether the consistency-matrix homes in **P9** vs **P14** (RQ3), and the mild **RQ1** coupling (if feasibility folds INTO the advocate, the shape gains feasibility findings). Those are cluster-scoped, not decidable from this primitive alone → flag with my recommendation, let reconcile + the human gate ratify.

## Step 5 — Responsibility trace (complete — every responsibility tagged; DELTA flips vs the specify port marked ★)

| # | Responsibility | Tag (plan run) | Delta vs. specify port |
|---|----------------|----------------|------------------------|
| R1a | Header `{{feature_id}}` | ★ **kept-but-rebind** | grounding source flips to the **plan lead's** workspace/feature naming (was specify lead); no body edit |
| R1b | Header `{{round}}` | ★ **kept-but-rebind** | grounding flips to the **plan lead's** bounded-loop round counter (was specify lead); `iteration→round` rename already applied in specify, generalizes to plan; no body edit |
| R1c | Header `{{timestamp}}` | **kept** | unchanged (framework-agnostic) |
| R2 | Gaps Found shape: table (ID/type/description/severity) + `Severity` (Critical\|Important\|Minor) + `Gap type` (Missing\|Ambiguous\|Edge Case\|Assumption\|Contradiction) | **kept** (content) | unchanged — plan's findings fold into this taxonomy (see R10); severity model IDENTICAL to P9's Issue Classification |
| R3 | Clarifications Needed shape (C#: title, related gap, question, 3 options, impact) | ★ **kept** (content); consumption **moved-to-lead (P1)** | shape unchanged; consumption feeds the **plan** Clarification Loop's human gate (was specify lead) |
| R4 | Verdict slot: `Status {{verdict}}` + 3-state `ready\|needs-revision\|critical-gaps` + `{{verdict_rationale}}` — the loop-driver field | **kept** (load-bearing) | **unchanged — `validation-plan-artifacts` emits the SAME 3-state vocabulary** (P9 SKILL.md L147-153); no new verdict contract |
| R5 | Verdict → loop-driver **consumption** (ready→converge / needs-revision→new round / critical-gaps→escalate) | ★ **moved-to-lead (P1)** | rehome target is the **plan** supervisor (was specify); DAG-gate + state-analyst parse machinery dissolve; the plan lead reads the verdict directly |
| R6 | "What's Strong" / `{{strengths}}` slot | **kept** | unchanged; absorbs plan's "acknowledge what was done well" (P9 Step 4) — including affirmative consistency notes in prose |
| R7 | `{{mustache}}` placeholder convention | **kept** | unchanged; no normalization owed |
| R8 | Discoverability / placement (router + REGISTRY registration; output-path audit) | ★ **kept-but-rebind** | REGISTRY row broadens `specify` → **shared (specify + plan)**; output-path rebind still lives in consumers (P1/P11), not this body; a wiring step, not a body edit |
| R9 | Cross-cluster availability (also filled in plan/tasks/techspec) | ★ **kept** — reuse **REALIZED** for plan | was "reuse later, no re-port" (potential) → **plan is the first realized reuse**; confirmed reuse, **no new file**, no re-port |
| R10 | **(plan-new)** Plan completeness-review findings (FR-coverage, orphan-TR, entity/endpoint coverage, schema-match, data-sensitivity, NFR-measurability, integration-boundary) | **kept** — folded into existing Gaps Found taxonomy | NEW with the plan skill; the *checks* ride in **P9** (`validation-plan-artifacts`); the *findings* land in R2's table as Missing/Ambiguous/Edge Case/Assumption/Contradiction — **no new gap category owed by P15** |
| R11 | **(plan-new)** Incremental-mode Cross-Artifact Consistency presentation (Pass/Fail matrix + New-vs-Previous Review Summary) | **moved-to-sibling-skill (P9)** / coordinate **RQ3 (P14)** — **NOT owed by P15** | NEW; HIL homes it in the **P9 skill** report-format block, not the template (command binds output to advocate-report-template.md even in incremental mode, L586). Substantive failures already route into R2 as gaps; only the affirmative matrix is unrepresented (specify omits it too). Flagged — see RQ4 |
| R12 | **(plan-new)** Phase disambiguation (analysis-completeness vs design-review fill events) | **kept** (no field owed) | NEW; lead disambiguates by dispatch context; HIL carries no `{{phase}}` field → not a capability the port loses; an informational header line is at most a reconcile nicety |

**Trace complete — every responsibility carries a tag. No `dropped` tags.** The plan-new responsibilities (R10–R12) are either absorbed by the existing shape (R10, R12) or homed in a sibling skill (R11) — none requires a P15 body edit. R9's reuse is *realized*, not lost.

---

## Plan-specific gap analysis (the heart of RQ4) — does the specify version miss anything plan emits?

| Plan completeness-review output | Carried by the existing template? | Where |
|---------------------------------|-----------------------------------|-------|
| 3-state verdict (`ready`/`needs-revision`/`critical-gaps`) | **YES — identical** | Verdict block (R4); P9 SKILL.md L147-153 matches exactly |
| Severity-classified issues (Critical/Important/Minor) | **YES — identical** | Gaps Found severity column (R2); P9 SKILL.md L44-52 matches exactly |
| Plan gap categories (FR-coverage, orphan-TR, entity/endpoint/schema/NFR/integration checks) | **YES — fold into existing taxonomy** | Gaps Found type column (R10): Missing/Ambiguous/Edge Case/Assumption/Contradiction covers them |
| Clarifying questions with options | **YES — identical** | Clarifications Needed block (R3); feeds plan's Clarification Loop |
| Strengths / "what was done well" | **YES** | What's Strong block (R6) |
| **Incremental Cross-Artifact Consistency Pass/Fail matrix** | **PARTIAL — failures yes, affirmative matrix no** | Failures → Gaps Found (Contradiction); affirmative matrix → home in **P9**/**P14**, not P15 (R11) |
| Phase tag (analysis vs design) | **N/A — lead tracks it; HIL has no phase field** | dispatch context (R12) |

**Net:** every verdict-contract field plan's completeness review emits is already carried. The single un-carried item (the affirmative consistency matrix) is (a) absent from HIL's template too, (b) not consumed by the loop, and (c) properly homed in the P9 skill / P14 checklist if wanted — **not a reason to extend P15.** Conclusion: **reuse as-is.**

---

## Decoupling scan result (summary)

- **Body: ZERO deny-list tokens** — grep-clean for sibling-agent names (incl. the plan additions `technical-analyst`, `principal-architect`), `dispatch`, `workflow-agnostic`/independence-by-declaration, injected workflow modes/paths/phases, and kernel/DAG/catalog/MCP/path tokens. The template never names its filler or its consumer; that content-cleanliness is *why* it reuses across clusters.
- **Net:** reusing a shared artifact in a second workflow is a registration/rehome change, not a coupling change. A clean positive continuing the run's decoupling-doctrine test.

## Reconcile flags

**PRIMARY — `flag-for-reconcile: RQ4` (advocate-report-template reuse).** Decide: reuse the specify port **as-is** (no new file; confirm fit + broaden the REGISTRY/router note specify→shared) vs **extend** it with plan-specific sections. **My assessment input (a recommendation, not a decision):**
- **REUSE AS-IS.** The verdict/severity/gap/clarification/strengths contract is identical between specify and plan; HIL binds both plan review sites to this exact template; plan's gap categories fold into the existing taxonomy with no new type; the template already serves multi-phase plan usage in HIL.
- **Do NOT extend P15 with the consistency matrix.** The one plan-only shape (incremental Cross-Artifact Consistency Pass/Fail matrix + New-vs-Previous Review Summary) belongs in **P9** (`validation-plan-artifacts` report-format guidance, where HIL keeps it) and/or coordinates with **P14** (`cross-artifact-checklist`, RQ3). Its substantive failures already land in Gaps Found; only the affirmative matrix is unrepresented, and extending the shared template to carry it would re-couple a cross-cluster artifact to plan.
- **If reuse-as-is is ratified:** the P15 line of the plan run produces **NO new file**; the only touched artifacts are (1) the REGISTRY row re-file (`specify` → **shared: specify + plan**) and (2) an optional broadening of the `mochiko` router/REGISTRY note — both **wiring/reconcile steps**, not body edits. Human-gated (contract §4a).

**COORDINATION edges (non-blocking — for the rehome map / reconcile context):**
1. **RQ1 coupling (mild).** Reuse-as-is holds cleanly under RQ1 options **(i)** two distinct reviewers and **(iii)** feasibility → generic `validator` — the advocate-report stays completeness-only. Under option **(ii)** (fold feasibility INTO the advocate), the shape would *also* carry feasibility findings — but a feasibility contradiction is still a `Contradiction`/`Critical` **gap** and `critical-gaps` already covers infeasibility, so even option (ii) most likely **reuses-as-is** (feasibility findings are just gaps). Recorded so reconcile sees the coupling; my read: reuse survives all three RQ1 branches.
2. **R5 rehome to the plan lead (P1).** The verdict-consumption / clarification-feed / gap-routing that drive the loop must land on the **plan** supervisor (P1, `redesign`) — exactly as they landed on the specify lead. The template is the origin surface only; if P1 does not pick up the verdict consumption, the plan bounded loop is silently broken. Mirrors specify's F-P13-1; record in the rehome map.

*Soft note (deferred to transform):* whether to add an informational `{{phase}}` header line (R12) — cosmetic, not owed, HIL carries none; let reconcile/transform settle if at all.

---

### Summary block

```
ASSESSMENT: advocate-report-template.md (P15)  [DELTA — already ported for specify]
Class:        template → branch artifact (completeness-reviewer report shape; verdict = loop-driver surface)
Triage:       gate1=n gate2=y gate3=y  [full-lens, light; Check 4 + Check 5 load-bearing]
Disposition:  keep-verbatim × standalone  →  REUSE the specify port AS-IS; plan run emits NO new file
              + flag-for-reconcile: RQ4 (reuse-as-is vs extend; my input: REUSE AS-IS)
Trace (DELTA — flips ★):
  - R1a header {{feature_id}}                       → ★ kept-but-rebind (grounding → plan lead)
  - R1b header {{round}}                            → ★ kept-but-rebind (grounding → plan lead round counter)
  - R1c header {{timestamp}}                        → kept
  - R2  Gaps Found shape + severity/type taxonomies → kept (content; severity model IDENTICAL to P9)
  - R3  Clarifications Needed shape                  → ★ kept (content); consumption → moved-to-lead (P1 human gate)
  - R4  Verdict slot (ready|needs-revision|critical-gaps) → kept (3-state IDENTICAL to P9 — load-bearing)
  - R5  verdict→loop-driver CONSUMPTION             → ★ moved-to-lead (P1) (DAG/state-analyst dissolve; rehome via P1)
  - R6  "What's Strong"/strengths slot              → kept
  - R7  {{mustache}} placeholder convention         → kept
  - R8  discoverability/placement (REGISTRY)        → ★ kept-but-rebind (re-file specify → shared specify+plan)
  - R9  cross-cluster availability                  → ★ kept — reuse REALIZED for plan (no re-port, no new file)
  - R10 plan gap categories (FR-coverage/schema/NFR/…) → kept (fold into existing taxonomy; checks ride in P9)
  - R11 incremental consistency Pass/Fail matrix    → moved-to-sibling-skill (P9) / RQ3 (P14) — NOT owed by P15
  - R12 phase disambiguation (analysis vs design)   → kept (no field owed; lead tracks via dispatch)
Reconcile flags:
  - RQ4 (PRIMARY, human-gated): reuse-as-is vs extend. Input: REUSE AS-IS — verdict/gap/clarification
    contract identical to specify; HIL binds both plan review sites to this template; the only plan-only
    shape (consistency matrix) homes in P9/P14, NOT P15. Reuse → no new file; only a REGISTRY re-file
    (specify → shared) + optional router-note breadth (wiring, not a body edit).
  - coordination (non-blocking): RQ1-coupling (reuse survives all 3 branches; option-ii feasibility findings
    are just gaps); R5 verdict-consumption rehome must land on the plan lead P1 (mirrors specify F-P13-1).
```

---

**Assessment version:** v1 · **Governed by:** `loop-discipline` · **Disposition:** keep-verbatim × standalone (REUSE) + flag-for-reconcile: RQ4 · **Role:** assess/diagnose only — no edit, no grade applied.
