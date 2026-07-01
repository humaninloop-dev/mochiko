# ASSESSMENT: devils-advocate (P4)

**Source:** `human-in-loop/plugins/humaninloop/agents/devils-advocate.md`
**Current port:** `plugins/mochiko/agents/devils-advocate.md` (ported for `specify`, 2026-06-27 — was P3 that run)
**Run:** transform-cluster / `plan` (plan-core scope) · **Assessed:** 2026-06-30
**Assessor:** `mochiko:transform-producer` (assess-only; no edit applied, no grade)
**Mode:** **DELTA assessment** — the agent is ALREADY PORTED; this assesses only what the `plan` cluster changes.

---

## Header

```
Class:        agent → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  → full-lens (run light — delta on an already-assessed agent)
Disposition:  port-with-edits × standalone   (light — a skill re-mount)
              + flag-for-reconcile: RQ1 (reviewer architecture — relational, cluster-scoped)
Reconcile flags: RQ1 (primary) + 2 record-only edges (re-mount sequencing; plan-pair record)
```

The agent is the adversarial **completeness reviewer** for `plan` — same role it plays for `specify`,
different checklist skill. In `plan` it grades the **technical-analyst's** output (requirements /
data-model / contracts) via `validation-plan-artifacts`; in `specify` it grades the
**requirements-analyst's** `spec.md` via `analysis-specifications`. One persona, two review skills,
two workflows — independence structural in both.

---

## The delta in one line

> **Re-mount `validation-plan-artifacts` (P9) — flip R7 from `moved-to-other-cluster` (stubbed,
> comment-only) → `kept-but-rebind` (live mount) — now that P9 ports this run. Persona unchanged.
> `validation-task-artifacts` (R8) STAYS stubbed (tasks deferred).**

What is **unchanged** (carried verbatim from the specify port, re-confirmed here):
- Persona body (Core Identity → What You Embrace) — keystone-clean, **0 deny-list tokens** (scan below).
- The adversarial-reviewer role, the 3-state verdict vocabulary (`ready` / `needs-revision` /
  `critical-gaps`), the Critical/Important/Minor severity model, the independence-by-role posture.
- `analysis-specifications` mount (specify slice) — stays live; not touched.

What **changes** (the plan delta):
- **R7 re-mount** (primary) — `validation-plan-artifacts` becomes a live mount.
- **R9** frontmatter `skills:` grows by one resolvable skill; the YAML stub comment shrinks to one line.
- **R13/R14** rehome-orchestration now points at the **`plan` lead (P1)**, not the specify lead.
- **R11** description breadth — a decoupled planning-completeness example MAY be re-added (RQ1-informed,
  optional, transform-time; not required for the mount to function).
- **R16/R17** plan-specific orchestration (incremental-review mode-switching; two-phase reviewer
  ordering) — the *procedure* rides in the skill; the *when-to-run* is **moved-to-lead (P1)**.

---

## Step 1 — Branch by class

Agent → **PLAYS-a-role**. Weighting: persona-vs-procedure split, the team-role it confers, `skills:`
independence, and the persona decoupling scan. Loop-ownership (done-condition / who-drives) is NOT
weighted on the agent — it sits on the `plan` lead (P1). Unchanged from the specify assessment; the
class does not change on re-port.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict (plan delta) |
|------|---|----------------------|
| 1 | Orchestration-coupled? | **YES** — dispatched by the `plan` command supervisor (lead): sequenced after the analyst emits artifacts, fed the artifact, its verdict consumed and looped on. Orchestration-coupling (lives *around* the agent), not content-coupling (body clean). Rehomes to P1. |
| 2 | Multi-responsibility / fans out? | **YES** — now mounts **two** review skills (`analysis-specifications` for specify + `validation-plan-artifacts` for plan), reviewing across two workflows; the plan skill itself fans into P1/P2/P3 phase checks. |
| 3 | Emits a non-machine-checkable artifact? | **YES** — gap/issue report + `ready/needs-revision/critical-gaps` verdict + suggestions are model judgment (e.g. "is the User entity from FR-003 present in the data model?"), not a schema/equality check. The validator is **real**, not a degenerate deterministic assert. |

All three trip → **full 7-check lens**, run *light* (this is a re-mount delta on an agent already
full-lensed in specify; checks unchanged from that assessment are confirmed, not re-derived).

## Step 3 — The lens (weighted for agent / PLAYS-a-role; delta-focused)

**1. Orchestration test.** Content-coupling: **NONE** (grep for `DAG|catalog|MCP|node|pass|brain|hil-dag|.json|.workflow` in the current mochiko body returns zero — confirmed). Orchestration-coupling: **YES**, entirely around the agent. The responsibilities that the HIL DAG owned — dispatch/sequencing (run the advocate *after* the analyst emits the design artifacts), verdict-consumption, the FAIL-loop (needs-revision → re-author) — re-home to the **`plan` lead (P1)**, exactly as they did to the specify lead. *Delta:* the rehome target is now P1; the contract (§1) explicitly requires the plan supervisor orchestration (two-phase sequence, architect-once-then-advocate ordering, skip-re-review routing, Phase-3 incremental review) be rehomed to the lead, not dropped — but those are **P1's** responsibilities, not this agent's body (it carries none of them). No body work owed on coupling grounds.

**2. Role at two altitudes.**
- *Skill-role:* consumes review procedures to **emit a reviewable artifact** (gap/issue report + verdict). *Delta:* now consumes a **second** procedure skill — `validation-plan-artifacts` (phase-checklist review of planning artifacts) alongside `analysis-specifications`.
- *Team-role conferred:* **VALIDATOR** (adversarial completeness reviewer) — the grading half of the plan producer↔validator pair (producer = `technical-analyst`, P2). Terminal grader; its verdict is arbitrated by the lead + human gate, so it needs no downstream validator of its own. *Two-form note for reconcile:* `validation-plan-artifacts` is **checklist-structured** (phase checklists, severity rubric, verdict criteria) while `analysis-specifications` is **adversarial-critique**. The SAME persona legitimately mounts both forms — evidence the convention-5 "two forms" are two *skill* forms, not two *persona* forms. (Input for RQ1; not decided here.)

**3. Independence — re-confirmed CLEAN for the plan pair (the decisive check).**
- `devils-advocate` will mount `analysis-specifications` + `validation-plan-artifacts` — **both are review/validation skills; neither authors.**
- `technical-analyst` (plan producer, P2) mounts `authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts` (confirmed at HIL `agents/technical-analyst.md:46`) — **all author/design; none grade.**
- The skill sets are **disjoint**, on **different agents** → no agent both produces and grades the plan artifacts. Same textbook-clean split specify shipped; plan inherits it via the re-mount.
- **Port hazard to preserve (load-bearing):** the re-mount MUST keep the sets disjoint — **never** mount `validation-plan-artifacts` (or any review skill) on `technical-analyst`, and **never** mount an authoring/patterns skill on `devils-advocate`. Independence here is structural (separate agent + disjoint skills), not declared in prose. The re-mount keeps `devils-advocate` review-only → independence preserved.

**4. Verdict-sink / loop-driver.** Verdict consumed by the **`plan` lead (P1)**. On `needs-revision`/`critical-gaps` the lead's bounded loop re-dispatches the producer (`technical-analyst`). The agent's own job is to *emit* the verdict (loop-fuel) — `kept`; the *consuming* and *looping* are `moved-to-lead` (now P1). *Delta:* `validation-plan-artifacts` emits the **same** 3-state verdict vocabulary as `analysis-specifications` (confirmed: SKILL.md verdict-criteria table = `ready` / `needs-revision` / `critical-gaps`), so the loop-fuel shape the lead consumes is consistent across both review skills — no new verdict contract for P1 to learn.

**5. Sibling / overlap ("look sideways") — this is where RQ1 lives.** In `plan`, HIL runs **two** independent reviewers: `principal-architect` (feasibility / cross-artifact contradiction) and `devils-advocate` (completeness, via `validation-plan-artifacts`). Whether mochiko keeps two distinct validators, folds feasibility into the advocate, or rehomes feasibility onto the generic `validator` is **RQ1** — a relational move across `devils-advocate` ↔ `principal-architect` ↔ `validator` that **cannot be decided from this primitive alone** → `flag-for-reconcile` (detailed below). No shared-core merge signal *with the producer* (the analyst↔advocate relation is a designed pair, not an overlap). `validation-task-artifacts` (R8) remains a sibling-cluster skill (tasks), still deferred.

**6. Coupling audit.**
- *Hardcoded paths:* **NONE** in the body (grep clean).
- *`skills:` references:* `analysis-specifications` → resolves (ported, specify). `validation-plan-artifacts` → **resolves THIS run once P9 lands** at `plugins/mochiko/skills/validation-plan-artifacts/` (namespace `mochiko:validation-plan-artifacts`). **Sequencing constraint:** the re-mount may only be *applied at transform* AFTER P9 is ported — a live mount of an unported skill dangles → `verify-output` Tier-1 FAIL. (P9 confirmed NOT yet present in mochiko; lands this run.) `validation-task-artifacts` → does **not** resolve (tasks deferred) → stays stubbed out of the live list.
- *Upstream handoff:* assumes the **plan design artifacts** (requirements.md / data-model.md / contracts/api.yaml, from `technical-analyst`) already exist to review → an explicit handoff edge the **plan lead** wires (analyst → artifacts → advocate). *Delta:* the reviewed artifact set is plan's design artifacts, not specify's `spec.md`.
- *Determinism boundary:* review is **model judgment** (design-gap / severity / verdict), confirming a real grounded validator. Some `validation-plan-artifacts` sub-checks are semi-mechanical (entity-name / requirement-ID cross-referencing), but the verdict is judgment → grounded LLM validator, not a deterministic assert.

**7. Conventions + loop placement + DECOUPLING SCAN.**
- *Classification:* agent (not user/model skill). `skills:` list grows from 1 → 2. Persona-vs-procedure split honored: the planning-review **procedure** lives in `validation-plan-artifacts`; the persona stays the skeptic's disposition. ✓
- *Discoverability (router):* the `mochiko` router currently lists `devils-advocate` as "specify-cluster adversarial critic … (skills: analysis-specifications)" (`skills/mochiko/SKILL.md:83`). On re-mount + plan landing, the router entry should broaden to its cross-workflow reviewer role + the 2-skill list. **Router edit = a separate wiring step** (as in specify's §D), NOT done in assess. → wiring edge.
- *Reliable model-invocation (triggers):* agents trigger via `<example>` blocks. The description re-breadth question (R11) lives here. ✓ persona-side trigger reliability is the skill's concern.
- *Agent↔skill composition:* persona carries the disposition; skills carry the procedure. The re-mount adds a procedure skill; persona unchanged & clean. ✓
- *Producer↔validator pairing:* structurally satisfied for the plan pair (this agent IS the completeness validator; producer = `technical-analyst`; disjoint skills, different agents). Whether plan runs ONE or TWO validators is RQ1, not an independence defect of this agent. ✓ (independence) / → flag (architecture).
- *Sound-loop:* the loop it sits in has independent validation (this agent is the completeness half); done-condition, FAIL-loop, and human gate are the **plan lead's (P1)** — re-homed there.

### Decoupling-doctrine verdict (DELTA — re-confirmed clean)

**The persona is still clean. Re-mounting a skill does NOT touch the persona.** Full grep scan of
`plugins/mochiko/agents/devils-advocate.md`:
- **sibling-agent names → NONE.** (`requirements-analyst`, `technical-analyst`, `principal-architect`,
  `task-architect`, `staff-engineer`, `qa-engineer`, `state-analyst`, `ui-designer`,
  `transform-producer`, `validator` — all absent.)
- **`dispatch` → NONE** · **`workflow-agnostic` / independence-by-declaration → NONE** · **injected
  workflow modes/paths/phases in the persona → NONE** · **kernel/DAG/catalog/MCP/path tokens → NONE.**
- **Canonical "Devil's Advocate" self-reference (line 32, "You are the **Devil's Advocate**—an
  adversarial reviewer who finds what others miss") → CLEAN.** This is the agent naming **itself** and
  immediately decoupling to *role* ("an adversarial reviewer"), exactly the case the specify port flagged
  as already-decoupled. It is a self-reference, not a sibling name — allowed (same as
  `principal-architect`/`validator` naming themselves). Confirmed still clean this run.
- The independence-by-role bullet added in specify ("Authoring or fixing the spec yourself … writing
  the spec is the author's job, not the reviewer's") uses **"the author"** as a generic role (matches
  `validator.md`), names no sibling and no workflow → stays keystone-clean. *Minor transform note:* if
  the description re-breadths to plan, this bullet's "the spec" wording could generalize to "the
  artifact" — a cosmetic, non-lossy touch, not owed.

**Net:** zero deny-list tokens in any persona line; the re-mount does not introduce any. A clean
positive data point continuing the decoupling-doctrine empirical test (the rich-persona stress case
this run is `technical-analyst`, not this agent).

## Step 4 — Disposition

**`port-with-edits × standalone`** (matches the contract hypothesis — light, a re-mount)
**+ `flag-for-reconcile: RQ1`** (reviewer architecture — relational, resolved cluster-scoped).

- *Body = `port-with-edits`* (not `keep-verbatim`): the persona is keep-verbatim-grade and keystone-clean, but localized **re-mount edits** are owed — (1) add `validation-plan-artifacts` to the live `skills:` frontmatter, (2) add a `mochiko:validation-plan-artifacts` bullet to "Skills Available", (3) shrink the YAML stub comment to the one remaining deferral (`validation-task-artifacts`), (4) optional/RQ1-informed: re-add a decoupled planning-completeness `<example>` to the description. Not `redesign`: body assumes no kernel; approach sound.
- *Structural = `standalone`*: the agent stays one home, one role (the adversarial reviewer). The re-mount itself does not split/merge/promote/absorb it, and `standalone` holds in **all three RQ1 options** (devils-advocate is the completeness reviewer regardless). So the base disposition does **not** depend on a sibling decision.
- *Why an ADDED `flag-for-reconcile` rather than a structural change:* RQ1 decides what happens to **feasibility** review (principal-architect / generic validator), not to this agent's placement. BUT one RQ1 branch (option ii — fold feasibility into the advocate) would *add* feasibility-review responsibility to this agent (a merge-in from `principal-architect`) — an extension beyond the re-mount that depends on the sibling dissolving. That contingency is reconcile's to decide, so it is flagged, not guessed. The re-mount stands solo today; RQ1 may layer onto it.

## Step 5 — Responsibility trace (complete — every responsibility tagged; DELTA flips marked ★)

| # | Responsibility (currently held) | Tag (plan run) | Delta vs. specify port |
|---|----------------------------------|----------------|------------------------|
| R1 | Adversarial review of **specifications** (gaps, ambiguities, edge cases, assumption gaps, contradictions) via `analysis-specifications` | **kept** | unchanged — specify slice stays live |
| R2 | Emit review **verdict** (`ready` / `needs-revision` / `critical-gaps`) | **kept** | unchanged — `validation-plan-artifacts` emits the SAME 3-state vocabulary; lead owns the clearing verdict |
| R3 | Generate **clarifying questions / issues** (product-framed, actionable, with suggestions) | **kept** | unchanged (plan skill frames them as classified issues + suggested fixes) |
| R4 | **Severity calibration** (Critical = "will break in production"; Critical/Important/Minor) | **kept** | unchanged — `validation-plan-artifacts` uses the same Critical/Important/Minor model |
| R5 | Adversarial **persona/values** (never rubber-stamp, never downgrade, never approve with zero findings, require evidence) | **kept** | unchanged — keystone-true; decoupled; verbatim |
| R6 | **"What You Hunt For"** review heuristics (5 categories) + single-source pointer | **kept** | unchanged; pointer line still names `analysis-specifications` for the spec taxonomy (plan skill carries its own — pointer not owed) |
| **R7** | Review of **planning artifacts** (requirements / data-model / contracts) via `validation-plan-artifacts` | ★ **kept-but-rebind** (LIVE MOUNT) | **was `moved-to-other-cluster` (stub) → now live.** THE primary delta. Mount once P9 lands; namespace `mochiko:validation-plan-artifacts` |
| R8 | Review of **task artifacts** via `validation-task-artifacts` | **moved-to-other-cluster** (tasks cluster) — **stays stubbed; NOT dropped** | unchanged — tasks deferred; re-mount when tasks ports |
| **R9** | `skills:` frontmatter resolution | ★ **kept-but-rebind** | now resolves **two** ported skills (`analysis-specifications`, `validation-plan-artifacts`); stub comment shrinks to one deferred line |
| R10 | `description` breadth: spec example | **kept** | unchanged |
| **R11** | `description` breadth: planning-completeness example + design-domain nouns | ★ **kept** (re-add, decoupled) — *RQ1-informed, optional, transform-time* | **was `moved-to-other-cluster` (removed in specify) → candidate to re-add** a decoupled planning example; NOT required for the mount; the "next phase" phrasing stays externalized (see R12) |
| R12 | Caller-side **phase phrasing** ("before planning", "next phase", "before implementation") | **moved-to-lead** | unchanged — stays externalized to `agent-dispatch.md`; any re-added example MUST NOT re-introduce it |
| **R13** | Dispatch point + verdict consumption + **FAIL-loop driving** | ★ **moved-to-lead (P1)** | rehome target is now the **plan** supervisor (was specify); plan adds the architect-once-then-advocate ordering + skip-re-review routing (RQ1-coupled, P1's) |
| **R14** | Upstream **handoff** (assumes the reviewed artifact exists) | ★ **moved-to-lead (P1)** | edge is now analyst → {requirements/data-model/contracts} → advocate (was spec.md → advocate); wired by the plan lead |
| R15 | Role as **independent validator** (separate agent, disjoint skills from producer) | **kept** (structural) | re-confirmed for the plan pair (vs `technical-analyst`); disjoint-skill hazard restated |
| **R16** | **Incremental-review mode** (Phase P2: full design review + 2-3 min consistency check on prior artifacts) | ★ **folded-into-skill** (procedure rides in `validation-plan-artifacts`) **+ moved-to-lead (P1)** for the *when-to-run-which-mode* switch | NEW with the plan skill; contract §1 requires the incremental orchestration be rehomed to the lead, not dropped — the agent invokes the skill, P1 chooses the mode |
| **R17** | Plan **reviewer ordering** (architect-feasibility-once → advocate-completeness; skip-architect-re-review unless structural change) | ★ **moved-to-lead (P1)** — *RQ1-coupled* | NEW; this is the lead's sequencing, not the agent's body; assumes two reviewers → resolved with RQ1. Primarily P1's + reconcile's concern; recorded here as a dispatch-timing edge touching this agent |

**Trace complete — every responsibility carries a tag. No `dropped` tags.** R7's deferred capability
is now *realized* (live mount), not lost; R8 remains explicitly preserved-by-reference (stub). The
two new plan responsibilities (R16/R17) are orchestration that rehomes to the lead, with the R16
*procedure* riding in the mounted skill.

---

## The P9 re-mount edge (the precise wiring site — for transform, not applied here)

**Exact stub site** in the current port (`plugins/mochiko/agents/devils-advocate.md`, lines 25-29):

```
25  # Only analysis-specifications is mounted this run (the specify-core slice).
26  # Deferred — NOT dropped — re-mount here when their clusters port:
27  #   validation-plan-artifacts  (plan cluster — planning-artifact review)   ← REMOVE (now live)
28  #   validation-task-artifacts  (tasks cluster — task-artifact review)       ← KEEP (still deferred)
29  skills: analysis-specifications                                             ← ADD validation-plan-artifacts
```

**Three coupled edits the transform applies (after P9 lands):**
1. **Frontmatter (line 29):** `skills: analysis-specifications` → `skills: analysis-specifications, validation-plan-artifacts` (bare names — matches the convention used by `principal-architect.md` / `validator.md` and specify deviation #1; keeps the live value grep-clean of dangling refs).
2. **Stub comment (lines 25-27):** drop line 27; reword the header so it reads "analysis-specifications + validation-plan-artifacts mounted (specify + plan slices); deferred — NOT dropped — re-mount when its cluster ports: validation-task-artifacts (tasks)".
3. **Body "Skills Available" (lines 36-40):** intro singular → plural ("specialized skills"); add a second bullet — `**mochiko:validation-plan-artifacts**`: phase-specific review criteria for planning artifacts (requirements / data-model / contracts), issue classification, cross-artifact consistency, and the incremental review mode.

**Optional / RQ1-informed (R11):** add one decoupled planning-completeness `<example>` to the
`description` (e.g. "Review this data model for design gaps before the next stage" — but *without*
the externalized phase phrasing). Lifts discoverability for plan-artifact review; not required for
the mount to function. Defer the keep/skip call to reconcile (rides on RQ1).

**Sequencing constraint (hard):** apply this re-mount **only after** P9 (`validation-plan-artifacts`)
is ported to `plugins/mochiko/skills/validation-plan-artifacts/`. A live mount of an unported skill
dangles → `verify-output` Tier-1 FAIL. (Verified: the mochiko skill dir does not yet exist; P9 lands
this run.)

**Downstream wiring edges (not in this agent's body):**
- **Router (convention 2):** broaden the `devils-advocate` entry in `skills/mochiko/SKILL.md` to its
  cross-workflow reviewer role + 2-skill list — a separate router-edit step.
- **REGISTRY:** flip the `devils-advocate` row note from "validation-plan-artifacts … deferred" to
  "validation-plan-artifacts re-mounted (plan)"; `validation-task-artifacts` stays deferred (tasks).

---

## Decoupling scan result (summary)

- **Persona body (lines 32-end): ZERO deny-list tokens.** Grep-clean for sibling-agent names,
  `dispatch`, `workflow-agnostic`/independence-by-declaration, injected workflow modes/paths/phases,
  and kernel/DAG/catalog/MCP/path tokens. The re-mount does not add any (it touches frontmatter +
  the "Skills Available" list, not the persona).
- **Canonical "Devil's Advocate" self-reference (line 32): CLEAN** — self-name decoupled to role
  ("an adversarial reviewer"), not a sibling. Confirmed still clean, as the specify port found.
- **Independence:** structurally clean for the plan pair — `devils-advocate` (review-only:
  `analysis-specifications` + `validation-plan-artifacts`) vs `technical-analyst` (authoring/design
  only). Disjoint skills, different agents. Hazard restated: never cross-mount.
- **Net:** a clean positive — re-mounting a skill is a wiring change, not a coupling change.

## Reconcile flags

**PRIMARY — `flag-for-reconcile: RQ1` (reviewer architecture).** `plan` has two HIL reviewers:
`principal-architect` (feasibility / cross-artifact contradiction) and `devils-advocate`
(completeness, via `validation-plan-artifacts`). The mochiko shape is a relational decision across
`devils-advocate` ↔ `principal-architect` ↔ generic `validator` that no single-primitive assessment
can make:
- **(i)** keep two distinct validators — a **checklist** advocate (this agent + `validation-plan-artifacts`)
  + an **adversarial-critique** architect (feasibility). The convention-5 two-form case; `plan` would
  be the FIRST cluster to exercise it.
- **(ii)** fold feasibility into the advocate (one reviewer, the specify shape) — this agent would
  *gain* feasibility-review responsibility (a merge-in from `principal-architect`), an extension
  beyond the re-mount.
- **(iii)** rehome feasibility onto the generic `validator`.
**This agent's re-mount is valid in ALL three options** (it is the completeness reviewer regardless),
so the re-mount is NOT blocked on RQ1. **Reconcile input (not a decision):** `devils-advocate` already
demonstrates one adversarial-reviewer persona mounting EITHER form of review skill (adversarial-critique
`analysis-specifications` / checklist-structured `validation-plan-artifacts`) — evidence the two
convention-5 forms are two *skill* forms under one persona, not two personas. Human-gated (contract §4a).

**RECORD-ONLY edges (non-blocking — for the rehome map):**
1. **Re-mount sequencing.** R7 re-mount applies only after P9 lands; record the edge so it is not
   silently skipped. The mechanism (live frontmatter mount + body bullet + stub-comment shrink) is the
   reverse of the specify stub — log it in the rehome map.
2. **Plan producer↔validator pair — record it.** `technical-analyst` (P2) produces; `devils-advocate`
   grades completeness; the **plan lead (P1)** referees and owns the verdict + FAIL-loop + handoff
   wiring (R13/R14). Disjoint skills, different agents — record so the lead's dispatch structurally
   guarantees independence (no new validator needs constructing for completeness; contrast `setup`).
   The feasibility-side validator of this pair is the subject of RQ1/RQ2.

*Soft note (deferred to transform):* whether to re-add the planning `<example>` to the description
(R11) and whether to generalize the "the spec" wording to "the artifact" — both cosmetic, RQ1-informed;
let reconcile/transform settle once RQ1 resolves.

---

**Assessment version:** v1 · **Governed by:** `loop-discipline` · **Disposition:** port-with-edits ×
standalone + flag-for-reconcile: RQ1 · **Role:** assess/diagnose only — no edit, no grade applied.
