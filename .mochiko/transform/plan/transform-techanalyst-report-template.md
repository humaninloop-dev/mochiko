# Transform — `techanalyst-report-template.md` (P12)

**Run:** transform-cluster / `plan` · **Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` · **Date:** 2026-06-30
**Role:** APPLY + WIRE only — NOT grade (independent grade is `mochiko:verify-output`, run by a different agent).
**Finalized disposition (from reconcile.md, P12 row):** `port-with-edits × standalone`
**Inputs consumed:** `assess-techanalyst-report-template.md` · `reconcile.md` (P12 rows + Job 2 rehome) · HIL source `human-in-loop/plugins/humaninloop/templates/techanalyst-report-template.md` · precedent `plugins/mochiko/templates/analyst-report-template.md` (+ sibling `advocate-report-template.md` for family round-vocab).
**Artifact written:** `plugins/mochiko/templates/techanalyst-report-template.md`

> What this report is: the PRODUCER's (technical-analyst) self-disclosure report. It carries **NO verdict** — the
> lead owns the clearing verdict, informed by the reviewer report(s). Every edit below serves that one fact.

---

## TRANSFORM (transform-recipes output format)

```
TRANSFORM: techanalyst-report-template.md (P12)
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/templates/techanalyst-report-template.md (created)
New partners: none (standalone — no split/promote; producer↔reviewer pairing already exists at the agent layer, RQ1)
Wiring:    classification=template/inert (wiring-floor; no disable-model-invocation field, not model-invoked)
           router=NOT registered here — entry NOTED below (shared router/plugin.json NOT edited, per task)
           triggers=N/A (inert template — no description/trigger surface)
           rebinds=[ specs/{feature-id}/.workflow/techanalyst-report.md → .mochiko/specs/<feature>/techanalyst-report.md ]
                   [ ${CLAUDE_PLUGIN_ROOT}/templates/techanalyst-report-template.md → mochiko plugin path — lives in lead/agent-dispatch, NOT this file ]
           single-source=independence doctrine referenced lightly (Usage Note 1: verdict lives in reviewer report(s),
                   lead owns the loop decision); loop-discipline NOT restated
Trace (realized): all 19 responsibilities tagged below; zero silent loss
```

---

## Body recipe applied (`port-with-edits`) — the five edits the body earned

The body was clean (zero kernel/DAG/catalog/MCP token, zero hardcoded path, no parser dependency — `plan` is a
prose supervisor with no state-analyst, so unlike specify there was **no parser to dissolve**). The edits are
localized; structure and producer-voice preserved.

| # | Edit | Realization |
|---|------|-------------|
| (a) | **`{{iteration}}` → `{{round}}`** | Header line `> Iteration: {{iteration}}` → `> Round: {{round}}`. Rebinds the brain-era pass counter to the lead's bounded round loop (contract round-cap = 3). Identical to the precedent; aligns round vocabulary across the plan report family (analyst-report, advocate-report both use `{{round}}`). |
| (b) | **Demote the brain-era counts** | HIL's count-heavy "Key Outputs" (Summary `{{artifacts_produced}}`; Analysis-phase TR/constraint/decision/NFR/FR-coverage table; the TR traceability snapshot; Design-phase entity/relationship/endpoint/classification/integration table) **collapsed to one optional `## Artifacts Produced` pointer table** (artifact + one-line "what's in it"). Retires the "Metric" framing; points the reviewer(s) at the real artifacts instead of duplicating them as counts. "thin disclosure, not a metrics dump." |
| (c) | **Drop `{{completion_status}}`; reframe `{{ready_for_review}}`** | The HIL `## Summary` "Completion" row is **removed entirely** (see "The completion_status drop" below). `{{ready_for_review}}` token **kept** but the section retitled **`## Handoff to Review`** + Usage Note 6 reframes it as a *pointer* to what is handed for grading — never a "ready/done" assertion. |
| (d) | **Align to the mochiko template wrapper** | `# Title Template` + one-line intro + fenced ` ```markdown ` body + `## Usage Notes`. The two `<!-- If phase == … -->` HTML-comment directives are **removed from the fenced body** and re-expressed as phase-gating guidance in Usage Note 8. Path rebind realized in Usage Note 9. |
| (e) | **Add `## What Changed This Round`** (round-delta) | New `{{changes_this_round}}` section (recommended; reconcile P12 row confirmed the add). Supports the lead's no-progress / bounded-loop watch (contract §3) — mirrors the precedent's same-named section. |

**Kept as-is (earned no edit):** `{{phase}}` disclosure (lead owns the 2-phase sequence, report only discloses
it); `{{production_summary}}` ("What Was Produced" — the primary decision prose); `{{constitution_alignment}}`
(references the real `.mochiko/memory/constitution.md` artifact); `{{open_questions}}` (feeds the clarification
loop); `{{feature_id}}`/`{{timestamp}}` (generic identity); title "Technical Analyst Report" (role-based
self-reference). **Optional constitution-label reword DECLINED** — no ported `technical-analyst` agent exists
yet to align the label to, and the constitution is a real mochiko artifact, so "Constitution Alignment" stays
accurate; minimalism governs (re-open if the P2 port adopts a different label).

`redesign` was rejected (body portable, no mechanism to rewrite); `keep-verbatim` was rejected (real edits a–e
earned). Structural `standalone` confirmed — one real producer-authored artifact in `templates/`, not in-session
orchestration state (contrast P11 plan-context-template = `absorb-into-lead`).

---

## The `completion_status` drop (independence — called out per task)

- **`{{completion_status}}`** (the HIL `## Summary` "Completion" row) → **`dropped + reason`**.
- **Reason:** independence. The done-condition / clearing verdict is **LEAD-owned**, informed by the reviewer
  report(s); a PRODUCER self-report that declares its own "Completion" leaks the verdict the independence rule
  reserves for the lead. A producer must not self-assert the lead-owned done-state.
- **Realization:** the entire HIL `## Summary` metric table (`Phase` / `Artifacts Produced` / `Completion`) is
  removed. `Phase` survives in the header; `Artifacts Produced` is demoted to the optional pointer table;
  **`Completion` has no successor field by design.** Usage Note 1 makes the absence explicit and load-bearing
  ("There is deliberately no 'Completion' / done field…").
- **Acceptance:** human-gated (reconcile §E + Human-gate bundle list "P12 `completion_status` drop"). Not silent.

---

## Edits vs the analyst-report precedent (called out per task)

The precedent is specify's ported producer-report `analyst-report-template.md`. P12 mirrors it where it applies
and diverges only on P12-specific surface.

| Dimension | analyst-report precedent (specify) | techanalyst-report (plan, P12) |
|-----------|-----------------------------------|--------------------------------|
| **Wrapper** | `# Title Template` + intro + fenced body + Usage Notes | **MIRRORED exactly** |
| **Round token** | `{{round}}`; `> Round:` blockquote header | **MIRRORED** (`{{iteration}}`→`{{round}}`); same blockquote-header shape |
| **Primary prose** | `## Summary` → `{{summary}}` | `## What Was Produced` → `{{production_summary}}` (P12's native heading for the same role — kept, not renamed to "Summary") |
| **Round-delta** | `## What Changed This Round` (for no-progress watch) | **MIRRORED** — added `## What Changed This Round` → `{{changes_this_round}}` |
| **Demoted counts** | one optional `## What I Created` count table ("convenience, not a tracked metric") | one optional `## Artifacts Produced` table — **heavier demotion** (P12 had 2 count tables + a TR snapshot vs the precedent's inline counts); realized as a per-**artifact** pointer (thinner than counts; points at the real files) rather than a per-section count |
| **Self-disclosure-not-a-verdict note** | Usage Note 1 (no PASS/FAIL; verdict is the critic's; lead owns the loop) | **MIRRORED + strengthened** — names the deliberate absence of a Completion/done field |
| **Output-location rebind** | `.mochiko/specs/<feature>/analyst-report.md` | **MIRRORED** → `.mochiko/specs/<feature>/techanalyst-report.md` |
| **`{{phase}}` disclosure** | none (specify has no phase concept) | **P12-NEW** — `> Phase:` header + Usage Note 4 (lead owns the 2-phase sequence; report discloses it) |
| **`{{constitution_alignment}}`** | none | **P12-NEW** — kept; references `.mochiko/memory/constitution.md`; optional label reword declined |
| **`{{open_questions}}`** | analog: Assumptions/Notes | **P12-NEW** kept — feeds the clarification loop / gap routing |
| **`{{ready_for_review}}` reframe** | (no such field) | **P12-NEW** — token kept, retitled `## Handoff to Review`, reframed to a handoff pointer (not a ready/verdict claim) |
| **`{{completion_status}}` drop** | (no such field) | **P12-NEW** — dropped for independence (above) |

---

## Realized responsibility trace (every responsibility → final tag; zero silent loss)

| # | Responsibility | Final tag |
|---|----------------|-----------|
| 1 | Producer self-report structure (handoff disclosure) | **kept** |
| 2 | "What Was Produced" production summary (primary prose) | **kept** (`{{production_summary}}`) |
| 3 | Identity header `{{feature_id}}` / `{{timestamp}}` | **kept** (generic identity) |
| 4 | `{{iteration}}` loop counter | **kept-but-rebind** → `{{round}}` (`> Iteration:`→`> Round:`; brain pass-counter → bounded round) |
| 5 | `{{phase}}` disclosure (analysis\|design) | **kept** — lead owns the 2-phase sequence (`moved-to-lead`, reconcile Job 2A #1); the report only discloses it (Usage Note 4) |
| 6 | Summary `{{artifacts_produced}}` count | **kept-but-rebind** (demoted into the optional `## Artifacts Produced` pointer table) |
| 7 | Summary `{{completion_status}}` (self-asserted done) | **dropped + reason** — independence: the done-state/verdict is LEAD-owned, never producer-self-asserted (human-gated; not silent) |
| 8 | Analysis-phase count table (TR/constraint/decision/NFR/FR-coverage) | **kept-but-rebind** (demoted/retired; duplicates `requirements.md` / `constraints-and-decisions.md` / `nfrs.md`) |
| 9 | TR traceability snapshot table (TR id/source FR/priority/desc) | **kept-but-rebind** (demoted; duplicated `requirements.md`) |
| 10 | Design-phase count table (entity/rel/endpoint/classification/integration) | **kept-but-rebind** (demoted/retired; duplicates `data-model.md` / `contracts/api.yaml`) |
| 11 | Brain-era count/metric/parse-surface framing | **dropped + reason** — kernel-free; lead + reviewer(s) read the artifacts directly; no metric/trajectory feed (and plan had no parser to feed) |
| 12 | `{{constitution_alignment}}` (governance disclosure) | **kept** (references `.mochiko/memory/constitution.md`; optional label reword declined — minimalism) |
| 13 | `{{open_questions}}` | **kept** — feeds the clarification loop / gap routing (reviewer stress-tests gaps) |
| 14 | `{{ready_for_review}}` (verdict-ish) | **kept-but-reframe** — token kept; retitled `## Handoff to Review`; Usage Note 6 reframes to a handoff *pointer*, not a ready/done assertion |
| 15 | `{{mustache}}` + section-heading convention | **kept** (matches the mochiko template wrapper) |
| 16 | Round-delta visibility for the bounded loop | **kept (added)** — new `## What Changed This Round` → `{{changes_this_round}}` (precedent-mirrored; supports no-progress watch) |
| 17 | Read by Supervisor + reviewers as producer-disclosure context | **kept-but-rebind** — consumers → plan lead + reviewer(s) (RQ1; role-generic in the body); direct prose read, no parser |
| 18 | "Report format: follow <template>" dispatch reference | **kept-but-rebind** (template path → mochiko) + **moved-to-lead** — the lead points the producer at this template via `agent-dispatch` (NOT in this file) |
| 19 | Output location / placement | **kept-but-rebind** → `.mochiko/specs/<feature>/techanalyst-report.md` (workspace-as-state; Usage Note 9) |
| 20 | "Technical Analyst" self-reference (title) | **kept** — role-based self-reference (parallels precedent "Analyst Report"); no sibling-agent name |

**Homeless-responsibility check:** none. Every responsibility carries a tag. Two `dropped + reason` (#7, #11) are
human-gate accepted (reconcile §E + bundle); both `moved-to-lead` slices (#5 sequence ownership, #18 dispatch
reference) land on the thinned `plan` lead per the reconcile rehome map, not in this artifact.

---

## Convention-wiring pass (all 6 — the floor every port pays)

1. **Classification** — inert **template**; no callable surface, so no `disable-model-invocation` field and not
   model-invoked. Wiring-floor satisfied (it is a passive fill-target). Matches the analyst-report / advocate-report
   sibling templates.
2. **Router registration** — **NOT edited here** (task: do NOT touch the shared router / `plugin.json`).
   **Router entry needed (for whoever owns the router edit):**
   > `techanalyst-report-template` (template, plan cluster) — the technical-analyst producer's self-disclosure
   > report; filled alongside the analysis/design artifacts each round and read directly by the lead + reviewer(s).
   > Hinted (template, not fired). Sits in the plan report family with `feasibility-report` (P13) and
   > `advocate-report` (P15).
3. **Trigger phrasing** — N/A (inert template; no `description`/trigger surface to phrase).
4. **Path rebinding** — `specs/{feature-id}/.workflow/techanalyst-report.md` → **`.mochiko/specs/<feature>/techanalyst-report.md`**
   (realized in Usage Note 9; matches the analyst-report precedent and the specify command's workspace-as-state).
   The external template-reference path (`${CLAUDE_PLUGIN_ROOT}/templates/…`) rebinds to the mochiko plugin path
   but lives in the lead / `agent-dispatch`, not in this file — recorded, not edited here.
5. **Decouple persona/skill** — **scan CLEAN.** Body contains: 0 sibling-agent names (no principal-architect /
   devils-advocate / supervisor / state-analyst), 0 "dispatch", 0 "workflow-agnostic" label, 0 injected
   workflow modes/paths/phases inside the fenced body (phase-gating moved to Usage Note 8; paths to Usage Note 9).
   Title "Technical Analyst Report" = **role-based self-reference**, KEEP (exactly parallel to the precedent
   keeping "Analyst Report"). Consumer references in Usage Notes are **role words** ("lead", "reviewer(s)"),
   never proper agent names — independence stated by role.
6. **Single-source / de-duplicate** — independence doctrine **referenced**, not restated: Usage Note 1 states the
   verdict lives in the reviewer report(s) and the lead owns the loop decision (a light pointer, the same shape as
   the precedent's Note 1). `loop-discipline` / `workflow-contract` are **not** inlined. The demoted counts now
   single-source onto the real artifacts (`requirements.md`, `constraints-and-decisions.md`, `nfrs.md`,
   `data-model.md`, `contracts/api.yaml`) rather than duplicating them as metrics.

---

## Handoff

Artifact + this realized trace are ready for **independent** grading by `mochiko:verify-output`, run by a
**different agent** (per the independence rule — this producer does not grade its own output). Open items for the
verifier / lead, none blocking this artifact:
- Router entry (item 2 above) — owned by whoever edits the shared router.
- `{{completion_status}}` drop (#7) and brain-era metric-framing drop (#11) — human-gate acceptance (already in
  the reconcile bundle).
- Cross-report-family coherence — round vocabulary aligned (`{{round}}`); header **shape** intentionally differs
  across the family (P12/P15 blockquote vs P13 `feasibility-report` YAML frontmatter) — a cross-template call, not
  this artifact's to force.
