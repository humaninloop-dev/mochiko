---
description: Create a feature specification via an independent author→critic loop (requirements-analyst writes spec.md, devils-advocate stress-tests it) with a human acceptance gate — sparse input is enriched first; default-FAIL, bounded, kernel-free.
---

# Specify — Feature Specification

You are the **lead / supervisor** for producing a feature specification. There is no kernel, no DAG, no catalog, no `state-analyst`, and no separate lead agent — **you, executing this command, are the supervisor**. You own the loop, the done-condition, the verdict, and the human gates. The specification is **authored** by a dispatched producer (`mochiko:requirements-analyst`) and **stress-tested** by a different, independent critic (`mochiko:devils-advocate`). You never let those two collapse into one agent — independence is the point. The critic *reports*; **you own the clearing verdict** — its recommended status is input, never the gate.

This workflow is a mochiko **sound loop**. Honor `mochiko:loop-discipline` throughout: a pre-declared done-condition that **defaults to FAIL**, independent validation (the author never grades its own spec), bounded iteration, and named human gates. The filled `workflow-contract` for this loop is inlined below; it is the inspectable proof that all four requirements are met.

**Argument:** `$ARGUMENTS` = the feature description (e.g. "let users export their data as CSV"). Empty or sparse is handled — Phase 0 assesses the input and enriches it before authoring.

---

## Done-condition (pre-declared, DEFAULT FAIL)

The specification **starts in the FAILING state** and flips to done **only** when **all** of these hold:

- `mochiko:devils-advocate` returns a recommended status of **`ready`** on `.mochiko/specs/<feature>/spec.md`, grounded in the spec file itself, **AND**
- **you (the lead) own and confirm the clearing verdict** — you Read `spec.md` + the advocate report and agree no unresolved blocking gap remains (the advocate's status is *input*, never the gate), **AND**
- the **human acceptance gate** (Phase 3) has cleared the validated spec.

Until all three hold, the run is **FAIL**. Running out of rounds is **FAIL-and-escalate**, never "done."

> **DROPPED (replaced):** HIL's exit — *gate verdict `ready` + milestone `achieved`, both evaluated autonomously by the State Analyst* — is removed. The verdict was **orchestrator-evaluated** (the analyst auto-completed the gate), and on the happy path there was **no human acceptance** — the loop could declare itself done on pass 1. Both violate `loop-discipline` requirement 1. The advocate's three-way status survives as **input to the lead's verdict**, not as the done-condition. Acceptance of this drop was recorded at the Phase-2 reconcile human gate.

---

## Workflow contract (this loop)

<!-- Filled instance of templates/workflow-contract.md. No open brackets = sound per loop-discipline. -->

**1. Done-condition (DEFAULTS TO FAIL)** — stated above. Measurable end state: advocate recommended status `ready` on `.mochiko/specs/<feature>/spec.md` + lead-confirmed clearing verdict + human acceptance cleared. Stated check: **you Read `spec.md` and `.mochiko/specs/<feature>/advocate-report.md` directly** (there is no parse layer and no state-analyst middle-man) and confirm no blocking gap remains. Constraints: no user story or functional requirement silently dropped; no `[NEEDS CLARIFICATION]`/`[PLACEHOLDER]`/open-bracket tokens ship in `spec.md`; kernel-free (no brain/DAG/MCP/catalog/state-analyst). Initial state: **FAIL**.

**2. Producer ↔ Validator (independence on two axes)**

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer (author)** | `mochiko:requirements-analyst` | `authoring-requirements`, `authoring-user-stories` | authors `spec.md` (+ `analyst-report.md`); **never grades** |
| **Validator (critic)** | `mochiko:devils-advocate` | `analysis-specifications` | stress-tests `spec.md`; emits a grounded gap report + recommended status; **never authors** |

- **Independence check:** producer agent ≠ validator agent ✓ **AND** producer skills `{authoring-requirements, authoring-user-stories}` ∩ validator skill `{analysis-specifications}` = ∅ ✓.
- **Validator trustworthiness tier:** **Tier 2 — separate-context grounded LLM** (highest the artifact allows; spec quality is model judgment, not a schema/version equality). Tier-1 deterministic sub-checks layered in: `spec.md` exists, and a placeholder scan (`grep` for `[NEEDS CLARIFICATION]`/`[...]` tokens) — neither is the quality gate.
- **Verdict ownership:** the advocate emits **severity-bucketed gaps + clarifying questions + a *recommended* status** (`ready` / `needs-revision` / `critical-gaps`); **the lead owns the clearing verdict and the done-condition.** The advocate's status never auto-completes the loop.
- **Tamper-proofing:** no `ready`-accept unless the advocate **Read `spec.md` this round** and cites evidence quoted from it; the lead also Reads `spec.md` + the advocate report before declaring the verdict. "Looks ready" is an automatic FAIL.

**3. Bounded iteration**

- **Hard round cap:** **3** produce↔critique rounds, counted by **you** (the lead), not by the model feeling done.
- **No-progress exit:** stop if a round's advocate gap set is **unchanged** from the previous round (the loop is stuck).
- **Budget / kill-switch:** out-of-band halt via sentinel file `.mochiko/specs/<feature>/SPECIFY_STOP` — checked **before each producer and critic dispatch**. If present, halt and escalate.
- **On hitting a guard:** **escalate** to the human gate (`AskUserQuestion`) with the advocate's last gap list and the failure context. **Never** mark the run done on cap / no-progress / kill-switch exhaustion.

**4. Human gate** — *presence non-negotiable; this loop names three (plus escalation):*

| # | Gate | Where it fires | What the human decides |
|---|------|----------------|------------------------|
| G1 | **Input recovery** | Phase 0 | re-enter lost input (the `@`-reference bug) or proceed without |
| G2 | **Clarification** (inside the loop, NOT the done-condition) | Phase 2 | answer the author's/critic's blocking questions; resolve **preference-gap** decisions |
| G3 | **Spec acceptance** (NEW) | Phase 3 | accept / amend / reject the **validated** spec |
| — | **Escalation** | any Phase-2 guard trip | resolve cap / no-progress / kill-switch / `critical-gaps` / scope-gap |

**Contract version:** v1 · **Governed by:** `loop-discipline`

---

## Phase 0 — Initialize & input assessment  *(human gate G1)*

### 0a. Capture the feature description

```text
$ARGUMENTS
```

`feature_description = $ARGUMENTS.trim()`. If `$ARGUMENTS` is empty (a known Claude Code bug drops `@`-references), recover the input (G1):

```
AskUserQuestion(
  questions: [{
    question: "Input may have been lost (known Claude Code bug with @ references). Re-enter the feature description?",
    header: "Input",
    options: [
      {label: "Re-enter input", description: "I'll type the feature description in the terminal"},
      {label: "Continue without input", description: "Proceed with no description (Phase 0 will enrich from scratch)"}
    ],
    multiSelect: false
  }]
)
```

### 0b. Constitution prerequisite (deterministic handoff edge)

The spec is governed by the project constitution. Check it is present — this replaces HIL's silent `carry_forward` auto-resolution (mochiko does **not** silently recover a missing prerequisite):

```bash
test -f .mochiko/memory/constitution.md && echo "constitution present" || echo "constitution MISSING"
```

- **Present** → Read it; carry its relevant constraints into the producer's brief as governing context.
- **Missing** → do **not** silently proceed. Surface it to the user (offer to run `/mochiko:setup` first, or to proceed without governance for this spec). Never auto-resolve.

### 0c. Input Assessment (lead-owned entry triage)

Decide whether the input is **rich** or **sparse** — you own the enrich-or-not routing:

- **Rich** — Who / Problem / Value are clear (from the description and/or the constitution's domain context) → **skip enrichment, go to Phase 2.**
- **Sparse** — missing Who / Problem / Value, or a one-line request with no context → **go to Phase 1** (enrich first).

> Project/constitution context can substitute for enrichment — do not enrich what is already clear. Enrichment is a **pre-loop** step only; do **not** re-enrich after round 1 (post-pass enrichment is wasteful — the loop's own critique drives later rounds).

---

## Phase 1 — Enrichment  *(sparse input only)*

The input is sparse (Phase 0c). Enrich it **before** authoring by invoking the enrichment engine in-session:

```
Skill(skill: "mochiko:analysis-iterative")
```

Work with the user to surface Who / Problem / Value and the core feature shape — this is interactive (the engine asks; the user answers). Carry the **enriched feature description** forward **in-session** into Phase 2's produce dispatch (optionally persist it under `.mochiko/specs/<feature>/` once the workspace exists in 2a). Enrichment runs **once**, pre-loop.

> Why the *lead* invokes this (not the producer): the enrich-or-not decision is loop-entry triage the lead owns; the skill stays a reusable conditioner that does not know which workflow called it. Independence is unaffected — enrichment conditions the *input*; it does not author or grade the spec.

---

## Phase 2 — Spec loop  *(all inputs · the sound loop)*

This is the produce → critique → repeat-on-FAIL core. **You own the round counter and the verdict.**

### 2a. Set up the feature workspace (kernel-free; no numbering script)

Derive a short kebab-case `<feature>` slug from the (now clear) feature description (e.g. "export data as CSV" → `csv-export`). This is a **lead-derived workspace identifier** — the HIL `create-new-feature.sh` numbering scheme is dropped (workspace-as-state replaces feature-numbering). Create the workspace and **seed `spec.md` from the template** (the excluded `create-new-feature.sh`'s template-copy capability, re-homed here):

```bash
mkdir -p .mochiko/specs/<feature>
cp ${CLAUDE_PLUGIN_ROOT}/templates/spec-template.md .mochiko/specs/<feature>/spec.md
```

Initialize `round = 1`; the spec is **FAIL** until proven otherwise.

### 2b. Produce — dispatch the author

Before dispatch, check the kill-switch: if `.mochiko/specs/<feature>/SPECIFY_STOP` exists, **escalate** (2e) — do not dispatch.

```
Task(
  subagent_type: "mochiko:requirements-analyst",
  description: "Author spec (round N)",
  prompt: "Author the feature specification into .mochiko/specs/<feature>/spec.md.
           This work is in the domain of `authoring-requirements` and `authoring-user-stories` — lean on them.
           Feature description (enriched in-session where applicable): <feature description / enriched input>.
           Governing context: READ .mochiko/memory/constitution.md (if present) and honor its constraints.
           The file is already seeded from the spec template — Read it first, then author into it: prioritized
           user stories (P1/P2/P3) with Given-When-Then acceptance scenarios; functional requirements (FR-XXX,
           RFC-2119 MUST/SHOULD/MAY); measurable success criteria (SC-XXX); edge cases. Technology-agnostic.
           No [NEEDS CLARIFICATION]/[PLACEHOLDER] tokens — if something is genuinely undecidable, list it as a
           clarification in your report rather than inventing it.
           ROUND > 1 (targeted revision): address EVERY item in the gap list below and do NOT rewrite or regress
           sections that already passed. GAP LIST: <paste the advocate's gaps-requiring-fix from the prior round
           + any Explore research findings + any clarification answers, or 'none — first round'>.
           WRITE the spec to .mochiko/specs/<feature>/spec.md and a short report to
           .mochiko/specs/<feature>/analyst-report.md (use templates/analyst-report-template.md; round = N).
           RETURN (in your reply) a brief summary: what you authored/changed this round, assumptions made, and
           any clarifications you genuinely need. Do NOT grade your own output."
)
```

**Clarification sub-gate (human, inside the loop — NOT the done-condition):** if the producer's report raises questions it cannot resolve, present them to the user (`AskUserQuestion`), collect answers, and feed them into the next produce dispatch as added context. This exchange **never** ends the loop on its own; the critic gate (2c) and your verdict (2d) still govern.

**Verify existence** (deterministic, not quality):
```bash
test -f .mochiko/specs/<feature>/spec.md && echo "spec present"
```

### 2c. Critique — dispatch the independent critic

Check the kill-switch again (`.mochiko/specs/<feature>/SPECIFY_STOP`). Then dispatch the **critic** — **never** the author:

```
Task(
  subagent_type: "mochiko:devils-advocate",
  description: "Stress-test spec (round N)",
  prompt: "Stress-test the specification at .mochiko/specs/<feature>/spec.md. This work is in the domain of
           `analysis-specifications` — lean on it. READ the spec file itself — never the author's report.
           Hunt for gaps, ambiguities, unstated assumptions, missing edge cases, untestable requirements, and
           vague success criteria. Cross-check against .mochiko/memory/constitution.md (if present).
           Bucket findings by severity, raise clarifying questions, note genuine strengths, and give a
           RECOMMENDED status: `ready` / `needs-revision` / `critical-gaps`. Your status is a RECOMMENDATION —
           the lead owns the clearing verdict.
           WRITE your review to .mochiko/specs/<feature>/advocate-report.md (use
           templates/advocate-report-template.md; round = N). RETURN (in your reply) a brief summary + your
           recommended status. Default to skepticism; quote evidence from the spec. Do NOT edit the spec."
)
```

### 2d. Loop control — you own the verdict (bounded iteration)

**Read the evidence yourself.** Open `.mochiko/specs/<feature>/spec.md` and `.mochiko/specs/<feature>/advocate-report.md` and Read them **directly** — there is no parse layer and no state-analyst middle-man. The advocate's recommended status is **input**; you declare the verdict.

- **You confirm `ready`** (advocate recommended `ready` **AND** you find no unresolved blocking gap in the spec) → record the verdict; proceed to **Phase 3** (acceptance).
- **Gaps remain** (advocate `needs-revision`, or you judge a blocking gap regardless of the advocate's status) → **classify each gap** per `loop-discipline`'s Gap Classification, then route it:
  - **knowledge gap** (missing factual / technical / codebase information) → dispatch the native `Explore` agent to research it; carry the findings into the next produce round's gap list. *(`Explore` is a native Claude Code read-only research agent, not a ported primitive: `Task(subagent_type: "Explore", description: "Research <gap>", prompt: "...")`.)*
  - **preference gap** (a product / scope / design decision only the user can make) → the **clarification human gate** (`AskUserQuestion`); feed the answer into the next produce round.
  - **scope gap** (the request is too large or out of bounds) → **halt / escalate** (2e) — do not silently shrink the spec.

  Having gathered any research findings and clarification answers, **increment `round`** and apply the bounds before looping:
  - If `round >= 3` (**round cap**) → **escalate** (2e). Do not loop again.
  - If this round's advocate gap set is **unchanged** from the previous round (**no-progress**) → **escalate** (2e).
  - If `.mochiko/specs/<feature>/SPECIFY_STOP` exists (**kill-switch**) → **escalate** (2e).
  - Otherwise → loop back to **2b** with the classified-gap list (targeted revision: address the flagged gaps, leave clean sections alone).
- **`critical-gaps`** (the advocate finds the spec fundamentally unsound) → **escalate** (2e).

### 2e. Escalate (never declare done on exhaustion)

```
AskUserQuestion(
  questions: [{
    question: "The specification did not reach an accepted clearing verdict within the bound.\n\nLast advocate status: [ready | needs-revision | critical-gaps]\nOutstanding gaps:\n[the advocate's gap list]\n\nReason the loop stopped: [round cap | no-progress | kill-switch | critical-gaps | scope-gap]\n\nHow should we proceed?",
    header: "Specify Escalation",
    options: [
      {label: "Give guidance and retry", description: "I'll add direction; run one more bounded round"},
      {label: "Accept with noted gaps", description: "Ship the spec as-is; I accept the outstanding items"},
      {label: "Abort", description: "Stop the run; leave the draft in .mochiko/specs/<feature>/ for later"}
    ],
    multiSelect: false
  }]
)
```
The run stays **FAIL** unless the human explicitly accepts. "Ran out of rounds" is never "done."

---

## Phase 3 — Spec acceptance  *(NEW human gate G3)*

Only reachable when you have confirmed the clearing verdict (advocate `ready` + your own read). Present the validated spec for the final acceptance the original workflow never had (G3):

```
AskUserQuestion(
  questions: [{
    question: "The spec at .mochiko/specs/<feature>/spec.md reached a clearing verdict (advocate: ready; confirmed by the lead against spec.md + the advocate report).\n\nUser stories: [count]\nFunctional requirements: [count]\nOutstanding clarifications: [none | list]\n\nAccept this specification?",
    header: "Spec Acceptance",
    options: [
      {label: "Accept", description: "Adopt the spec; proceed to finalize"},
      {label: "Amend - more changes", description: "Send specific changes back through the author→critic loop"},
      {label: "Reject", description: "Do not adopt; abort the run"}
    ],
    multiSelect: false
  }]
)
```

- **Accept** → Phase 4. The done-condition is now satisfied (advocate `ready` + lead-confirmed verdict + human acceptance).
- **Amend** → re-enter Phase 2 with the requested changes as the gap list (still bounded; the spec must reach a clearing verdict again before returning here).
- **Reject** → abort; the draft remains under `.mochiko/specs/<feature>/` for a later run.

---

## Phase 4 — Finalize

1. **Report to the user.** Read the accepted spec for the summary:

   ```markdown
   ## Specification Complete

   ### Artifacts
   - `.mochiko/specs/<feature>/spec.md` — the feature specification (deliverable)
   - `.mochiko/specs/<feature>/analyst-report.md` — author's round notes
   - `.mochiko/specs/<feature>/advocate-report.md` — critic's final review

   ### Summary
   - Feature: <feature>
   - Rounds: [N]
   - User stories: [count] · Functional requirements: [count]
   - Independent critique: clearing verdict reached (advocate: ready); accepted at the human gate

   ### Suggested commit
   `docs: specify <feature>`

   ### Next steps
   1. Review the spec at `.mochiko/specs/<feature>/spec.md`
   2. Run `/mochiko:plan` to create the implementation plan *(reference stub — `plan` not ported yet)*
   ```

2. **Cleanup.** The workspace under `.mochiko/specs/<feature>/` holds the deliverable (`spec.md`) plus the round reports — there is **no** ephemeral context-handoff file to delete (HIL's `context.md` was absorbed into this lead). Offer a lightweight retain/clean choice:
   ```
   AskUserQuestion(
     questions: [{
       question: "Specify is complete. The spec and round reports live under .mochiko/specs/<feature>/.\n\nKeep the workspace as-is, or remove the intermediate round reports?",
       header: "Cleanup",
       options: [
         {label: "Keep everything", description: "Retain spec.md + the analyst/advocate reports"},
         {label: "Remove reports only", description: "Delete analyst-report.md and advocate-report.md; keep spec.md"}
       ],
       multiSelect: false
     }]
   )
   ```
   Never offer to delete `spec.md` — it is the deliverable.

---

## State recovery

If interrupted, resume from evidence in the `.mochiko/specs/<feature>/` workspace (not from a context-file `phase` field — there is none):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/` for the feature | Phase 0 (triage; enrich if sparse) |
| workspace exists, `spec.md` is still the bare seeded template (unfilled placeholders) | Phase 2 (produce, round 1) |
| `spec.md` authored, no `advocate-report.md` for this round | Phase 2 (critique) |
| `advocate-report.md` present, status `needs-revision`/`critical-gaps`, within the round cap | Phase 2 (loop control → produce) |
| `advocate-report.md` status `ready`, not yet accepted | Phase 3 (acceptance) |
| accepted | Phase 4 |
| `.mochiko/specs/<feature>/SPECIFY_STOP` present | halt → escalate (2e) |

---

## Supervisor behaviors (what you own, not the agents)

- **Own the loop:** the round counter, the no-progress check, the round cap (3), the kill-switch, the escalation. The model's "looks done" is never the exit — your confirmed clearing verdict + human acceptance is.
- **Own the verdict:** the author proposes the spec, the critic recommends a status *from* the spec, **you** Read `spec.md` + the advocate report **directly** and declare the verdict — only against the pre-declared, default-FAIL done-condition. The advocate's status is input, never the gate. *(This is the deliberate reversal of HIL's "never read agent reports directly": the state-analyst middle-man that required that context-isolation is gone, so you must read the artifacts to own the verdict — `loop-discipline` requirement 2.)*
- **Own the human gates:** input recovery (G1), in-loop clarification + preference-gap decisions (G2), spec acceptance (G3), and every escalation. Never auto-accept a spec, a dropped gap, or an exhaustion.
- **Own input triage:** assess sparse vs rich input; enrich (via `analysis-iterative`) only when sparse, only before round 1.
- **Classify gaps** (via `loop-discipline`): knowledge → research with the native `Explore` agent; preference → clarification gate; scope → halt.
- **Never let producer and critic collapse into one agent.** `requirements-analyst` authors; the independent `devils-advocate` critiques; they share no skills. Independence is the point.
- **Stay kernel-free:** no brain, DAG, MCP, catalog, or state-analyst. All orchestration is this command + the two dispatched agents (+ `analysis-iterative` / `Explore` as needed) + the `.mochiko/specs/<feature>/` workspace + `.mochiko/memory/constitution.md`.
- **Domain agents carry no workflow knowledge** — brief them per `agent-dispatch.md`; all run context goes in the dispatch prompt and the workspace artifacts, never baked into their personas.
- **Do NOT modify git config or push to remote.** Always use the Task tool to invoke agents — never inline agent behavior.
