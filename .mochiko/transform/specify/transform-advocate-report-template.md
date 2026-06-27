# Transform — `templates/advocate-report-template.md` (P13)

Run: `specify` cluster transform · Phase 3 (transform) · Producer: `transform-producer` · Skill: `mochiko:transform-recipes`
Applied: 2026-06-27 · Inputs: reconcile §D row P13 + §E P13 trace + §G template-wrapper / Agenda 6c/6e · assess-advocate-report-template.md · HIL source `human-in-loop/plugins/humaninloop/templates/advocate-report-template.md`
Disposition (finalized, human-gate-accepted): **keep-verbatim × standalone** (departed from the port-with-edits hypothesis — body is deny-list-CLEAN; only the `iteration→round` grounding rebind + the mochiko wrapper apply).

> **Transform applies a decision; it does not make one.** The disposition was decided in assess (Step 4 departure) and ratified in reconcile (§D P13). Not graded here — `verify-output` (Phase 4, a different agent) owns the grade.

---

## Applied

**`keep-verbatim` × `standalone` + convention-wiring pass.**

- **Body (`keep-verbatim`)**: the HIL body copied unchanged except the single grounding rename. Verified **byte-faithful** to HIL + that one rename (full-body `diff` clean; verdict block `diff` identical). The 3-state verdict (`ready | needs-revision | critical-gaps` + `{{verdict_rationale}}`), the Gaps table + Severity/Gap-type taxonomies, the Clarifications shape, "What's Strong," and all `{{mustache}}` placeholders are **kept verbatim, load-bearing**.
- **The one body-level rename**: `> Iteration: {{iteration}}` → `> Round: {{round}}` (grounding rebind to the lead's bounded-loop round counter, cap 3; the HIL DAG `pass_number` provenance dies). **Identical to P12's rebind** per reconcile §6c ("same vocabulary on both report templates").
- **Structural (`standalone`)**: one home — `plugins/mochiko/templates/advocate-report-template.md`. No merge with P12 (complementary: producer report vs critic-review-with-verdict). Cross-cluster reuse by plan/tasks/techspec is correct single-source reuse, not a split/dedupe. No new partner primitive.

## Artifacts

- **Created**: `plugins/mochiko/templates/advocate-report-template.md`
- **New partners**: none (standalone; the producer↔validator pair already exists in the cluster).

## Convention-wiring pass (all five ran)

| # | Pass | Result |
|---|------|--------|
| 1 | **Classification** | Inert template — no classification frontmatter (per §G: P11/P12/P13 are inert templates, no `disable-model-invocation`, no `skills:`). Nothing added. |
| 2 | **Router registration** | **Owed, not performed here.** Registration of P13 (hinted template entry) into `skills/mochiko/SKILL.md` is a single batched cluster router-wiring edit (§G lists all specify primitives together); per Phase-3 per-primitive scoping the shared router is **not** edited in this transform. Recorded as a pending cluster-wiring item for the router pass / `verify-output` to confirm. |
| 3 | **Trigger phrasing** | N/A — inert template has no `description`/triggers. |
| 4 | **Path rebinding** | **No path in the body to rebind** (assess Check 6: the output location `specs/{feature-id}/.workflow/advocate-report.md` lives in the *consumers*, not this template). The `specs/{feature-id}/.workflow/` → `.mochiko/specs/<feature>/` rebind is owned by P1 (command) + P14 (context absorb), tagged `kept-but-rebind` there. Body unchanged. |
| 5 | **Decouple persona/skill** | Body deny-list scan **CLEAN** — `grep -inE "dispatch\|workflow-agnostic\|state-analyst\|devils-advocate\|requirements-analyst\|DAG\|catalog\|hil-dag\|brain\|.humaninloop\|verdict_field\|gate-node\|pass_number"` returns **0 hits**. Added wrapper text (title/intro/Usage Notes) is **role-based** — names the filler by role ("a reviewer"), never the agent (`devils-advocate`); no dispatch, no injected workflow modes/paths/phases. Nothing to scrub. |

**Wrapper convention applied** (reconcile §G "optional fence on a verbatim body + `## Usage Notes`; codebase-analysis-template precedent): `# Advocate Report Template` title + one intro line + ` ```markdown ` fence around the verbatim body + `## Usage Notes`. The body keeps its own `# Advocate Report` heading inside the fence (mirrors `codebase-analysis-template`'s wrapper-title / inner-title split). The fence is unambiguous — the body contains no nested code fence.

**Usage Notes are decoupled by construction**: they document only field vocabularies (placeholder convention, severity levels, gap types) and a content-level gloss of the three verdict values (a judgment about the spec under review). **No loop-consumption prose** — they do not describe who reads the verdict, the round loop, convergence, escalation, or the human gate. Adding that would re-couple the template; it stays out (the consumption is a LEAD responsibility, see R5).

## Trace (realized — every responsibility carries a final tag)

| # | Responsibility | Realized tag | Realized in artifact |
|---|----------------|--------------|----------------------|
| R1a | Header `{{feature_id}}` | **kept-but-rebind** | Field text kept verbatim (`> Feature: {{feature_id}}`); grounding rehomed from EXCLUDED `create-new-feature.sh`/branch naming → the **lead's** mochiko feature/workspace naming. **No body edit** (rebind is the lead's, executed at fill time). |
| R1b | Header `{{iteration}}` | **kept-but-rebind** | Realized as **`> Round: {{round}}`** — the single body-level rename; grounding from DAG `pass_number` → lead's bounded-loop round counter (cap 3). Identical to P12. |
| R1c | Header `{{timestamp}}` | **kept** | `> Generated: {{timestamp}}` verbatim. |
| R2 | Gaps Found shape (ID/Type/Description/Severity table) + Severity (`Critical\|Important\|Minor`) + Gap-type (`Missing\|Ambiguous\|Edge Case\|Assumption\|Contradiction`) taxonomies | **kept** (content) | Verbatim. The *consumption* — gaps read on a `needs-revision` round to drive targeted revision/routing — is **moved-to-lead** (§B.1; not template prose). |
| R3 | Clarifications Needed shape (C#: title, related gap, question, 3 options, impact) | **kept** (content) | Verbatim. The *consumption* — clarifications fed to the human gate (`AskUserQuestion`) — is **moved-to-lead** (§B.6; not template prose). |
| R4 | Verdict slot: `Status {{verdict}}` + 3-state vocabulary `ready\|needs-revision\|critical-gaps` + `{{verdict_rationale}}` — the loop-driver **field** | **kept** (load-bearing; verified byte-identical) | Verbatim verdict block; `diff` vs HIL = identical. The verdict CONTENT survives unchanged. |
| R5 | Verdict→loop-driver **consumption** (`ready`→converge / `needs-revision`→new bounded round / `critical-gaps`→escalate) | **moved-to-lead** | **NOT added to the template** (decoupling preserved). The DAG gate-node + state-analyst parse machinery dissolve; the lead reads the verdict directly to drive the bounded loop + human gate. Rehomed onto P1 (`specify` command) + via P4 (`state-analyst`) dissolution — reconcile §B.1/§B.6; **F-P13-1 lands**. Template is the origin surface only. |
| R6 | "What's Strong" / `{{strengths}}` slot | **kept** | Verbatim (positive-feedback content; no loop role). |
| R7 | `{{mustache}}` placeholder convention | **kept** | Verbatim; matches `codebase-analysis-template`. Documented in Usage Notes (vocabulary only). |
| R8 | Discoverability / placement (router template-registration; output-path audit) | **kept-but-rebind** | Router registration **owed** (batched cluster edit, not performed here — wiring pass #2). Output-path rebind (`specs/.../.workflow/` → `.mochiko/specs/<feature>/`) lives in consumers P1/P14, not this body. |
| R9 | Cross-cluster availability (also filled in plan/techspec/tasks) | **kept** | Single-source artifact; later clusters **reuse**, do not re-port (reconcile §A 7c). |

**Every responsibility accounted for. No silent loss. No `dropped` on this primitive** (the only drops in P13's chain — the DAG gate/parse machinery — are consumption mechanics that rehome to the lead as R5, not template responsibilities).

## Deviations / notes for `verify-output`

1. **Disposition departed from the run hypothesis** (`port-with-edits` → `keep-verbatim`), per assess Step 4 and ratified in reconcile §D/§E. Not gated by contract §4 (reconcile §F: P12/P13 keep-verbatim/port-with-edits "applied without explicit accept"). No new decision made at transform time.
2. **No loop-consumption prose added** — the verdict→loop-driver, clarification→human-gate, and gap→revision consumptions all stay off the template (R5/R3/R2); they are lead responsibilities (P1). The template remains the origin surface only, fully decoupled. This is the central correctness property of a `keep-verbatim` port of a verdict-carrying template: keep the verdict CONTENT, never the consumption.
3. **Router registration is owed, not done** (wiring pass #2) — flagged so the cluster router-wiring pass / `verify-output` registers P13 in `skills/mochiko/SKILL.md` as a hinted template entry.
4. **P12 cross-check**: `analyst-report-template.md` (P12) is **not yet written** at the time of this transform. The `iteration→round` rename here is fixed by the shared reconcile spec (§6c: same vocabulary on both report templates), so P12 must use the **identical** `> Round: {{round}}` token when authored. Surfaced so the pair stays consistent.

## Hand-off

Artifact + this realized trace → `verify-output` (Phase 4), run by a **different agent**. Not graded here.

---

```
TRANSFORM: advocate-report-template.md (P13)
Applied:   keep-verbatim × standalone + wiring-pass
Artifacts: plugins/mochiko/templates/advocate-report-template.md
New partners: none
Wiring:    classification=none(inert template) router=owed(batched cluster edit, not done here)
           triggers=n/a(inert) rebinds=[iteration→round (body); feature_id grounding→lead (no body edit);
           output-path specs/{feature-id}/.workflow/→.mochiko/specs/<feature>/ owned by consumers P1/P14]
           decouple=body CLEAN (0 deny-list hits); wrapper role-based; no loop-consumption prose added
Trace (realized):
  - R1a {{feature_id}}                 → kept-but-rebind (grounding→lead; no body edit)
  - R1b {{iteration}}                  → kept-but-rebind (realized as {{round}}; = P12)
  - R1c {{timestamp}}                  → kept
  - R2  Gaps Found shape + taxonomies  → kept (content); gap→revision consumption → moved-to-lead
  - R3  Clarifications Needed shape     → kept (content); →human-gate consumption → moved-to-lead
  - R4  Verdict slot (3-state+rationale)→ kept (verbatim, byte-identical — load-bearing)
  - R5  verdict→loop-driver CONSUMPTION → moved-to-lead (P1/P4; F-P13-1; NOT in template)
  - R6  What's Strong/strengths        → kept
  - R7  {{mustache}} convention         → kept
  - R8  discoverability/placement       → kept-but-rebind (router owed; output-path in consumers)
  - R9  cross-cluster availability      → kept (reuse later, no re-port)
```
