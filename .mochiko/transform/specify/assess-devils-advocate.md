# ASSESSMENT: devils-advocate (P3)

**Source:** `human-in-loop/plugins/humaninloop/agents/devils-advocate.md`
**Run:** transform-cluster / `specify` (specify-core scope) · **Assessed:** 2026-06-27
**Assessor:** `mochiko:transform-producer` (assess-only; no edit applied)

---

## Header

```
Class:        agent → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  → full-lens
Disposition:  port-with-edits × standalone
Reconcile flags: 2 relational signals (record-only, non-blocking) — see §Reconcile
```

This is the adversarial **VALIDATOR** half of the specify-core producer↔validator pair
(producer = `requirements-analyst`; lead/referee = the `specify` command supervisor). It does
not author the spec; it grades it and returns a verdict (`ready` / `needs-revision` /
`critical-gaps`).

---

## Step 1 — Branch by class

Agent → **PLAYS-a-role**. Weighting per branch: persona-vs-procedure split, the team-role it
confers, `skills:` independence, and the persona decoupling scan (the run goal). Loop-ownership
checks (done-condition / who-drives) are *not* weighted on the agent — they sit on the lead.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — in HIL it only runs because the Supervisor (via `state-analyst` + `specify-catalog.json` DAG) dispatches it at the review node, feeds it the artifact, and consumes its verdict. *Orchestration*-coupling, not content-coupling (body is clean — see Check 1). |
| 2 | Multi-responsibility / fans out? | **YES** — three review domains via three skills (spec / plan / tasks), feeding three workflows. |
| 3 | Emits a non-machine-checkable artifact? | **YES** — gap report + `ready/needs-revision/critical-gaps` verdict + clarifying questions are model judgment, not a schema/equality check. So the validator is *real*, not a degenerate deterministic assert. |

All three trip → **full 7-check lens** (recorded).

## Step 3 — The lens (weighted for agent / PLAYS-a-role)

**1. Orchestration test.** Orchestrated in HIL by the Supervisor + `state-analyst` driving the
`specify-catalog.json` DAG. Splitting the two couplings:
- *Content-coupling:* **NONE.** Grep for `DAG|catalog|MCP|node|pass|brain|hil-dag|.json` in the
  file returns zero. The body never references kernel mechanics. Body is content-clean → no
  `redesign` owed on coupling grounds.
- *Orchestration-coupling:* **YES**, but it lives entirely *around* the agent, not in it. The
  responsibilities that dissolve with the DAG — dispatch/sequencing (run advocate *after* the
  analyst emits spec.md), verdict-consumption, and the FAIL-loop (needs-revision → re-author) —
  re-home to the `specify` command supervisor (lead). The agent body carries none of them, so
  this is rehome-orchestration (`moved-to-lead`), not body work.

**2. Role at two altitudes.**
- *Skill-role:* consumes review procedures (`analysis-specifications`) to **emit a reviewable
  artifact** (gap report + verdict).
- *Team-role conferred:* **VALIDATOR** (adversarial critic). It is the grading half of the pair;
  its verdict is the gate. It is the terminal grader — its verdict is arbitrated by the lead +
  human gate, so it needs no downstream validator of its own.

**3. Independence — the decisive check, and it is CLEAN.**
- `devils-advocate` mounts `analysis-specifications, validation-plan-artifacts,
  validation-task-artifacts` — **all three are review/validation skills; none author.**
- `requirements-analyst` (producer) mounts `authoring-requirements, authoring-user-stories` —
  **both author; neither grades.**
- The skill sets are **disjoint**, on **different agents**. No agent both produces and grades.
  This is the textbook-clean producer↔validator split that the doctrine demands — and it is the
  notable contrast with `setup`, where `principal-architect` had to be *split* to create an
  independent validator. **Specify already ships its independent validator: this agent.** No
  split is owed.
- *Port hazard to preserve:* the port MUST keep these sets disjoint — never mount a review skill
  on the analyst or an authoring skill on the advocate. Independence here is structural (separate
  agent + disjoint skills), not declared in prose.

**4. Verdict-sink / loop-driver.** Verdict consumed by the lead (`specify` supervisor). On
`needs-revision`/`critical-gaps` the lead's bounded loop re-dispatches the producer. The agent's
own responsibility is to *emit* the verdict (loop-fuel) — `kept`; the *consuming* and *looping*
are `moved-to-lead`.

**5. Sibling / overlap.** Pairing partner is `requirements-analyst` — a designed pair, not an
overlap to merge. No shared core, no shared validator, no merge/split signal. The only sideways
signals are relational record-keeping, not structural moves (see §Reconcile). Soft note: the
persona's "What You Hunt For" heuristics overlap the gap-finding procedure inside
`analysis-specifications` — a possible dedupe at transform time, not a structural move.

**6. Coupling audit.**
- *Hardcoded paths:* **NONE** in the body (grep clean).
- *`skills:` references:* `analysis-specifications` → resolves to the to-be-ported specify-core
  skill (P6); the two `validation-*` skills do **not** resolve this run (plan/tasks deferred) →
  must be stubbed/removed-with-note so the frontmatter never points at an unported skill.
- *Upstream handoff:* assumes an artifact (spec.md from the analyst) already exists to review →
  an explicit handoff edge the lead wires (analyst → spec.md → advocate).
- *Determinism boundary:* review is **model judgment** (gap/severity/verdict), confirming a real
  grounded validator (not a deterministic assert).

**7. Conventions + loop placement + DECOUPLING SCAN (run goal).**
- *Classification / discoverability / triggers:* dispatched sub-agent; convention-wiring pass
  (in transform) sets mochiko frontmatter, registers it in the router, and trims the description
  `<example>` set. Format (`<example>` blocks) is kept — matches `mochiko:principal-architect`.
- *Persona-vs-procedure:* persona (Core Identity, Quality Standards, **Adversarial Calibration**,
  What You Reject/Embrace) is the disposition of a trustworthy skeptic — keystone-true on any
  job. "What You Hunt For" is a review heuristic that leans procedural and echoes
  `analysis-specifications`; keep as persona, soft-dedupe candidate.
- **Decoupling scan (deny-list) — persona body is CLEAN.** Grep evidence:
  - sibling-agent names → **NONE** (the `devils-advocate` mentions in lines 8/18/26/27 are the
    agent naming *itself* in selection examples, not a sibling);
  - `dispatch` → **NONE**; `workflow-agnostic`/independence-by-declaration → **NONE**;
    injected workflow modes/paths/phases *in the persona* → **NONE**.
  - The only workflow vocabulary lives in the **`description` examples** (lines 4, 7-8, 16-18,
    25-27) and the **"Skills Available"** section (lines 44-45): out-of-scope domain nouns
    (*planning artifacts, data model, contracts, task artifacts*) + mild phase phrasing
    (*before planning, next phase, before implementation*).
- *Producer↔validator pairing:* structurally satisfied (this agent IS the validator; partner
  exists; disjoint skills). The persona does not even name its partner → independence is purely
  structural. Model case.
- *Sound-loop:* the loop it sits in has independent validation (this agent **is** it);
  done-condition (`ready`), FAIL-loop, and human gate are the lead's — re-homed there.

### Decoupling-doctrine verdict (the run's empirical question)

**The multi-workflow `description` is legitimate multi-domain craft, NOT a deny-list violation.**
The agent states its role *by role* ("adversarial reviewer"), never names a sibling agent, never
says "dispatch," never declares itself "workflow-agnostic," and keeps its persona body free of
injected phases/modes/paths. What looks like coupling is (a) breadth — it reviews specs *and*
plans *and* tasks — and (b) caller-side sequencing phrasing, and both live in the
**description/examples (caller context)**, never in the persona.

**Two consequences for the port (both `port-with-edits`, neither changes independence):**
1. **Scope trim, not a decoupling fix:** the plan/tasks examples + domain nouns are *out of
   specify-core scope* (deferred clusters), so trim the description to the spec slice. This is a
   scoping edit driven by the deferred skills, not a deny-list removal.
2. **Caller-context externalize:** the mild phase phrasing ("before planning", "ready for the
   next phase", "before implementation begins") is sequencing context that belongs in
   `agent-dispatch.md` per the doctrine, not in the agent.

**Trimming to the specify slice does NOT change independence.** Independence is conferred by the
disjoint-skill-set + separate-agent structure, not by the description's breadth or any
declaration. A narrower description still grades the same way.

*Data point for the doctrine test:* a well-written HIL agent keeps coupling in the
description-examples (caller context) and states role-by-role; the deny-list scan correctly finds
the **persona clean**, and the keystone test correctly flags only the example phase-vocabulary as
caller-context to externalize. No deny-list token slipped into a persona.

## Step 4 — Disposition

**`port-with-edits × standalone`** (matches hypothesis).

- *Body = `port-with-edits`* (not `keep-verbatim`): the persona prose (Core Identity → What You
  Embrace) is keep-verbatim-grade and keystone-clean, but three localized edits are owed — trim
  the `description` to the spec slice, prune the "Skills Available" section to
  `analysis-specifications`, and rebind the `skills:` frontmatter (drop/stub the two deferred).
  Not `redesign`: the body assumes no kernel and the approach is sound.
- *Structural = `standalone`*: self-contained agent, one home (the specify-pair validator). The
  move does not depend on a sibling decision, so this is **not** `flag-for-reconcile`. No `split`
  (it already only grades — nothing to separate), no `merge`, no `promote`, no `absorb`.

## Step 5 — Responsibility trace (complete — every responsibility tagged)

| # | Responsibility (currently held) | Tag |
|---|----------------------------------|-----|
| R1 | Adversarial review of **specifications** — gaps, ambiguities, edge cases, assumption gaps, contradictions | **kept** (specify-core validator role; via `analysis-specifications`) |
| R2 | Emit review **verdict** (`ready` / `needs-revision` / `critical-gaps`) | **kept** (validator verdict; loop-fuel for the lead) |
| R3 | Generate **clarifying questions** (product-framed, concrete options) | **kept** (validator output; feeds human gate + re-author loop) |
| R4 | **Severity calibration** (critical = "will break in production") | **kept** (persona Adversarial Calibration + `analysis-specifications`) |
| R5 | Adversarial **persona/values** — never rubber-stamp, never downgrade, never approve with zero findings, require evidence | **kept** (keystone-true; decoupled; this is what makes the validator trustworthy/skeptical-by-default) |
| R6 | **"What You Hunt For"** review heuristics (5 categories) | **kept** (persona) + *soft dedupe candidate* vs `analysis-specifications` at transform |
| R7 | Review of **planning artifacts** (research, data model, contracts) via `validation-plan-artifacts` | **moved-to-other-cluster** (plan cluster deferred) — **stub / rebind-by-reference**, re-mount when plan ports. **NOT dropped.** |
| R8 | Review of **task artifacts** (vertical slice, TDD, traceability) via `validation-task-artifacts` | **moved-to-other-cluster** (tasks cluster deferred) — **stub / rebind-by-reference**, re-mount when tasks ports. **NOT dropped.** |
| R9 | `skills:` frontmatter resolution | **kept-but-rebind** — `analysis-specifications` → ported P6 skill; remove/stub the two deferred refs so frontmatter never points at an unported skill |
| R10 | `description` breadth: spec example | **kept** (trimmed to spec slice) |
| R11 | `description` breadth: plan/tasks examples + domain nouns | **moved-to-other-cluster** (deferred with R7/R8) — removed from description this run, return when those clusters port |
| R12 | Caller-side phase phrasing in examples ("before planning", "next phase", "before implementation") | **moved-to-lead** (externalize to `agent-dispatch.md`, not the agent) |
| R13 | Being dispatched at the right point + verdict consumption + **FAIL-loop driving** (needs-revision → re-author) | **moved-to-lead** (rehome-orchestration; the `specify` supervisor's bounded loop) |
| R14 | Upstream **handoff**: assumes spec.md (from producer) exists to review | **moved-to-lead** (explicit handoff edge analyst → spec.md → advocate, wired by lead) |
| R15 | Role as **independent validator** (separate agent, disjoint skills from producer) | **kept** (structural; pairing recorded in reconcile) |

No responsibility is untagged → trace is complete (done-condition met). No `dropped` tags — the
two deferred capabilities (R7, R8) are explicitly preserved by reference, not lost.

## Reconcile flags (relational signals — record-only, non-blocking)

The disposition (`port-with-edits × standalone`) stands solo; these are signals for
`reconcile-cluster` to *record* in the rehome map, not blockers:

1. **Producer↔validator pairing — already satisfied.** Reconcile should formally record the pair
   (`requirements-analyst` produces, `devils-advocate` grades, lead referees) so the lead's
   dispatch structurally guarantees two different agents with disjoint skills. No new validator
   needs spinning out (contrast `setup`).
2. **Deferred-skill cross-cluster rehome.** R7/R8 (`validation-plan-artifacts`,
   `validation-task-artifacts`) → confirm the **stub vs rebind-by-reference** mechanism and log
   the re-mount edge for when plan/tasks port. Contract already pre-decides "rebound by reference
   only"; reconcile records it in the rehome map so it is not silently dropped.

*(Soft, optional:* dedupe R6 "What You Hunt For" against `analysis-specifications` at transform —
a wiring-pass nicety, not a structural decision.)*

## Decoupling-scan hits (summary)

- **Persona body (lines 49-118): ZERO deny-list tokens.** Grep-clean for sibling-agent names,
  `dispatch`, `workflow-agnostic`, injected modes/paths/phases, kernel/DAG/catalog/MCP, and
  hardcoded paths.
- **`description` (lines 4-27) + "Skills Available" (lines 44-45):** out-of-scope multi-domain
  examples (plan/tasks) + mild phase phrasing — **not** hard violations; trimmed by the
  `port-with-edits` scope edit (R10/R11) and the caller-context externalize (R12).
- **Net:** no deny-list token in any persona; independence unaffected by the trim. A clean
  positive data point for the empirical decoupling-doctrine test.
