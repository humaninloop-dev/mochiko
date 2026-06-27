# ASSESSMENT: authoring-user-stories

**Primitive:** `human-in-loop/plugins/humaninloop/skills/authoring-user-stories/` (SKILL.md + `references/PRIORITY-DEFINITIONS.md` + `references/EXAMPLES.md` + `scripts/validate-user-stories.py`)
**Cluster:** `specify` (P8) · **Assessor:** transform-producer · **Date:** 2026-06-27
**Role in cluster:** one of the `requirements-analyst` (producer) agent's two authoring procedures — prioritized user stories (P1/P2/P3), Given/When/Then acceptance scenarios, independent tests.

---

## Class & branch

**Class:** skill → **branch: PLAYS-a-role.** Weighted checks: consumed-procedure vs emits-artifact, trigger reliability, **sibling overlap with `authoring-requirements`**, decoupling.

## Triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled (body needs kernel/supervisor/command/DAG to function)? | **no** — body is a self-contained authoring procedure; the DAG sequences it one layer up |
| 2 | Multi-responsibility / fans out? | **yes** — bundles priority taxonomy + G/W/T + independent-test + anti-rationalization + a validation script; feeds spec.md, the advocate review, and downstream QA |
| 3 | Emits an artifact whose correctness is NOT machine-checkable? | **yes** — the script checks *format syntax*; whether a priority is right / a story is genuinely independently testable / scenarios are observable is model judgment |

Gates 2 + 3 trip → **full 7-check lens** (not fast-path).

---

## The lens (weighted PLAYS-a-role)

### 1. Orchestration test — content-coupling NONE; orchestration-coupling external
- **Orchestrating layer:** the HIL `specify` DAG (`catalogs/specify-catalog.json`, node `analyst-review`) + the `hil-dag` brain + the `state-analyst` agent, all driven by the `commands/specify.md` markdown supervisor. The skill is invoked by the `requirements-analyst` agent *inside* the `analyst-review` node.
- **Content-coupling:** **none.** Grep for `brain|hil-dag|DAG|MCP|catalog|.humaninloop|carry_forward|node_id` over the whole bundle = **zero hits.** The body never names the kernel.
- **Orchestration-coupling:** **real but lives entirely one layer up** — at the agent / command / catalog / state-analyst, never in the skill. The skill is *sequenced* (after input-enrichment, before advocate-review) and *looped on* (advocate FAIL → re-brief → analyst revises). Those responsibilities are NOT the skill's to carry; they rehome to the mochiko command supervisor (lead). **The skill's body is mochiko-clean; the coupling is in the cluster around it.**

### 2. Role at two altitudes
- **Skill-role:** a **consumed-procedure** that contributes a slice of a **reviewable artifact** (the `## User Stories` section of spec.md). The analyst runs it as a step; its output becomes part of the spec the advocate reviews.
- **Team-role conferred:** **producer** (authoring/generative) — it makes its caller a producer.
- **Needs a validator partner?** The artifact is reviewable + not machine-checkable for correctness → it needs independent validation. **That validator already exists in the cluster** — the `devils-advocate` agent (via `analysis-specifications`) grades user-story completeness/gaps. Pairing is satisfied at the **agent-team level** (analyst produces, advocate grades). **Do NOT spin out a new validator for this skill** — the adversarial pairing covers it.

### 3. Independence — CLEAN (no self-grade leak)
- `requirements-analyst` mounts `authoring-requirements` + `authoring-user-stories` — **both producer/authoring skills.** It does NOT mount the grading skill `analysis-specifications` (which lives on `devils-advocate`).
- Producer skills {authoring-requirements, authoring-user-stories} ∩ validator skill {analysis-specifications} = ∅. Producer agent ≠ validator agent. **No independence flag.**

### 4. Verdict-sink / loop-driver
- **Consumers of output:** spec.md via the spec-template `{{user_stories}}` placeholder → then (a) `devils-advocate` gap review, (b) the human-clarification gate, (c) downstream QA independent tests.
- **What loops on FAIL:** the advocate's Critical/Important gaps drive a revise loop (DAG + state-analyst + command re-brief the analyst). The skill **is loop-driven but does not own the loop.**
- **Rehome:** loop-driving → mochiko command supervisor's **bounded** produce→critique→revise loop (`moved-to-lead`). The skill stays a pure authoring procedure.

### 5. Sibling / overlap — `authoring-requirements` (KEY FINDING → flag-for-reconcile)

| | authoring-user-stories | authoring-requirements |
|---|---|---|
| Distinct artifact axis | P1/P2/P3 priority taxonomy + decision tree; user-journey narrative; **Given/When/Then** acceptance scenarios; independent-test-per-story | **FR-XXX / RFC-2119** (MUST/SHOULD/MAY); **SC-XXX** success criteria; edge-case taxonomy; key-entity modeling |
| Spec sections filled | `## User Stories` | `## Functional Requirements`, `## Success Criteria`, `## Edge Cases`, `## Key Entities` |
| References | PRIORITY-DEFINITIONS, EXAMPLES | RFC-2119-KEYWORDS, EDGE-CASES |
| Script | validate-user-stories.py | validate-requirements.py |

**Shared substrate (dedupe signal):** both are the analyst's authoring procedures at the *same* `analyst-review` node; both fill complementary sections of the *same* spec.md; both enforce the **technology-agnostic ("WHAT not HOW")** and **measurable/observable-outcome** disciplines with near-identical mistake patterns; both ship a parallel structural-validation script (same `checks`/`summary` JSON shape, same determinism boundary). They **explicitly cross-reference and partition scope** ("user-stories → don't duplicate underlying requirements"; "requirements → use authoring-user-stories for stories").

**Verdict:** **NOT a thin variant over a shared core** (which would be merge-into-sibling) — each owns a *substantial, distinct* body; neither is thin; the HIL authors deliberately kept them separate. They share **discipline conventions**, not a core. **Recommendation: keep DISTINCT / standalone**, both mounted on the analyst, with (a) a **dedupe** of the shared tech-agnostic/measurable discipline substrate (one source, both point at it) and (b) a **trigger de-collision** — both currently trigger on "acceptance criteria" (user-stories: "define acceptance criteria"; requirements: "documenting acceptance criteria for user stories"); same caller so misfire-risk is low, but phrasing should be split to each skill's work-context (convention-wiring, not structural).

Because merge-vs-distinct is a sibling-dependent relational call, this is emitted as a **flag-for-reconcile** (recommendation: standalone-distinct + dedupe-shared-discipline + de-collide-triggers).

### 6. Coupling audit
- **Hardcoded paths:** none to `.humaninloop/`. Bundle-internal relative paths only (`scripts/validate-user-stories.py`, `references/*.md`) → `kept`. The two `humaninloop:` namespace cross-links → `kept-but-rebind`.
- **Prerequisite / handoff:** assumes an enriched feature description upstream (from `analysis-iterative` Who/Problem/Value) and writes into spec-template `{{user_stories}}`. Both edges are orchestrated at the agent/command layer, not encoded in the body → handoff edges owned by lead/contract (`moved-to-lead`).
- **Determinism boundary:** `validate-user-stories.py` is a **deterministic Tier-1 format assert** (priority markers, G/W/T syntax, independent-test presence, justification ≥20 chars, header format; `checks`/`summary` JSON, exit 0/1). It imports only `json/re/sys/pathlib` — **kernel-free, NOT a brain dependency.** It catches *syntax, not judgment*, so it **does not** make the producer↔validator pairing degenerate — real correctness still needs the advocate (Tier-2). Keep the script as a producer-side self-check; it is NOT the independent validator.

### 7. Conventions + loop placement
- **Classification:** currently model-invoked via "MUST be invoked when the user says…". In mochiko it is **agent-consumed** by the analyst → stay model-invoked, but **rephrase triggers to work-context** ("transforming a feature description into prioritized user stories…"), not "when the user says". (transform)
- **Discoverability:** not in any router today → **register in the mochiko router** with when-to-reach-it guidance that distinguishes it from `authoring-requirements`. (transform)
- **Reliable model-invocation:** graded exact-phrase triggers present but user-facing; rephrase + **de-collide the "acceptance criteria" overlap** with the sibling. (transform)
- **Composition / DECOUPLING SCAN (run goal):** pure-procedure skill (persona stays on the agent) — clean split. Deny-list grep evidence:
  - **agent names:** zero hits.
  - **"dispatch" / "workflow-agnostic" / injected workflow mode/path/phase:** zero real hits. One keystone-passing **false positive** — `PRIORITY-DEFINITIONS.md:97` "Consider deferring to a future phase" = product *delivery* phasing of P3 stories (true of any spec job) → **keep**; flag so the validator's scan doesn't trip on it.
  - **kernel tokens (brain/DAG/MCP/catalog/.humaninloop):** zero hits.
  - **namespace cross-refs:** `humaninloop:patterns-technical-decisions`, `humaninloop:patterns-api-contracts` (SKILL.md:29–30, both in "When NOT to Use") — these are **deferred design-track skills not ported this run**; sibling-SKILL links (not agent names, not a doctrine violation) → genericize the guidance or drop the dead links with reason. `kept-but-rebind`.
  - **Empirical decoupling-doctrine data point:** the authoring-user-stories body was **already decoupled in HIL** — no workflow coupling at all. Validates the thesis that authoring procedures stay clean and coupling lives in the agents/commands/DAG around them.
- **Loop placement:** the loop the skill sits in already has independent validation (advocate) + human gate (human-clarification / constitution-gate) + done-condition (spec-complete) — supplied by the DAG today, **rehoming to the supervisor** in mochiko. The skill itself supplies a producer-side self-check (script + quality checklist), NOT the independent gate. Unlike the setup cluster, the independent-validation gate **already exists** here — reconcile rehomes it, it does not have to be invented.

---

## DISPOSITION

**`port-with-edits` × `flag-for-reconcile`** (sibling overlap with `authoring-requirements`).

- **Body = `port-with-edits`** (not keep-verbatim): the core (format, priority taxonomy, G/W/T, examples, anti-rationalization, validation script) is mochiko-clean and stays; localized edits required by the wiring pass — (1) rephrase description triggers to agent-consumed work-context, (2) rebind `humaninloop:` sibling links / genericize the two deferred design-track pointers, (3) rebind the script-invocation path if needed, (4) de-collide the "acceptance criteria" trigger vs the sibling. No kernel assumption in the body → not redesign; capability is needed → not drop.
- **Structural = `flag-for-reconcile`** (sibling-dependent): recommended resolution **standalone / keep-distinct** from `authoring-requirements` (both mounted on the analyst), **+ dedupe** the shared tech-agnostic/measurable discipline substrate, **+ de-collide** triggers. Reconcile confirms distinct-vs-merge with full cluster context.

---

## RESPONSIBILITY TRACE

### Owned by this skill (must all carry a tag — done-condition)
- Transform feature descriptions → prioritized user stories (P1/P2/P3) — core procedure → **kept**
- Priority taxonomy + assignment guidance (defns, decision tree, distribution; PRIORITY-DEFINITIONS.md) → **kept**
- Given/When/Then acceptance-scenario grammar → **kept**
- Independent-test-per-story requirement → **kept**
- Worked user-story examples (EXAMPLES.md) → **kept**
- Anti-rationalization discipline (Red Flags / Common Rationalizations / Common Mistakes) → **kept**
- Technology-agnostic + observable/measurable-outcome discipline (shared with sibling) → **kept**, flagged **dedupe** candidate (reconcile decides whether to factor the shared substrate into one source)
- Deterministic structural format validation (`validate-user-stories.py`, Tier-1 self-check, kernel-free) → **kept** (rebind invocation path → **kept-but-rebind**)
- Scope-partition cross-references to sibling skills (`patterns-technical-decisions`, `patterns-api-contracts`, ↔ requirements) → **kept-but-rebind** (rebind ported siblings to `mochiko:`; genericize/drop the two deferred design-track links — *reason:* target skills out of specify-core scope this run)
- Classification + trigger reliability ("when user says" model-invoked) → **kept-but-rebind** (re-tag agent-consumed model-invoked; rephrase to work-context; de-collide "acceptance criteria" with sibling)
- Router discoverability — *none in HIL* → added by the convention-wiring pass at transform (register in `mochiko` router) → **kept-but-rebind** (new convention floor)

### Orchestration the skill is SUBJECT TO but does NOT own (traced so it is not silently dropped when the DAG sheds; rehomed via the P1 `specify` command + P4 `state-analyst` assessments — NOT this skill's burden)
- Sequencing (run after input-enrichment, before advocate-review) → **moved-to-lead**
- Loop-driving on advocate FAIL (re-brief analyst to revise stories) → **moved-to-lead** (supervisor's bounded loop)
- Input handoff (enriched feature description) + output sink (`{{user_stories}}` section) → **moved-to-lead** (explicit handoff edges in the contract)
- Independent validation of story *correctness* → already provided by `devils-advocate`/`analysis-specifications` at the agent level → **kept** at cluster level (no new validator for this skill)

**No silent drops.** The only removals are the two dead cross-links to deferred design-track skills, each carrying a reason for the lead to accept.

---

## OUTPUT (per format)

```
ASSESSMENT: authoring-user-stories
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × flag-for-reconcile: sibling overlap with authoring-requirements
              (recommend standalone/keep-distinct + dedupe shared discipline + de-collide triggers)
Reconcile flags:
  - sibling overlap w/ authoring-requirements: shared discipline substrate, DISTINCT artifact axes
    → recommend keep-distinct (NOT merge); dedupe shared tech-agnostic/measurable discipline;
      de-collide "acceptance criteria" trigger (convention-wiring)
  - producer↔validator pairing already satisfied by analyst→advocate (do NOT spin a new validator)
Decoupling scan: CLEAN — 0 agent names, 0 dispatch, 0 kernel tokens.
  1 keystone-pass false positive ("future phase" = product delivery phasing, PRIORITY-DEFINITIONS.md:97);
  2 namespace rebinds to deferred design-track skills (SKILL.md:29-30) = path-rebind, not a doctrine violation.
```
