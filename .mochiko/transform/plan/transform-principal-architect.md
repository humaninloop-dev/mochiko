# TRANSFORM — principal-architect (P3)

**Source disposition:** `assess-principal-architect.md` (port-with-edits × flag-for-reconcile — body contingent on RQ1)
**Reconcile verdict:** `reconcile.md` RQ1 option **(i)** + RQ2 → **`port-with-edits × standalone`**: re-broaden to plan's feasibility reviewer; mount NEW `validation-feasibility`; **G1** (never re-mount `validation-constitution`). Resp. #7 (feasibility review, was homeless) → **`folded-into-skill`**.
**Run:** `/mochiko:transform-cluster` (plan-core) · **Phase 3 (transform)** · **Date:** 2026-07-01
**Producer:** `mochiko:transform-producer` · **Skill applied:** `mochiko:transform-recipes`
**Role:** apply the finalized disposition + run the convention-wiring pass. **NOT** decide (assess/reconcile own that), **NOT** grade (`verify-output`, a different agent, owns that).
**Edited file:** `plugins/mochiko/agents/principal-architect.md` (a **delta** edit on the existing setup port — re-broadens; does not rewrite).

> **Reconstruction note.** The artifact edit landed on disk in a prior session that hit its limit before
> emitting this trace. This file is reconstructed **from the on-disk artifact itself** (every claim below is
> grep- or line-cited against `plugins/mochiko/agents/principal-architect.md`) cross-checked against the
> reconcile spec (§RQ1 / §RQ2 / §JOB 2b / §JOB 3). No further edit was made to the artifact.

---

## Output block (transform-recipes format)

```
TRANSFORM: principal-architect
Applied:   port-with-edits × standalone + wiring-pass   (re-broaden: carve-out removed, +1 skill mounted)
Artifacts: plugins/mochiko/agents/principal-architect.md   (edited — re-broadened persona; skills 2→3)
New partners: NONE authored here. The mount target validation-feasibility (skill) and the emission target
              feasibility-report (P13, renamed) are SIBLING transforms (transform-validation-feasibility.md,
              transform-feasibility-report-template.md) — both present, neither edited by this primitive.
Wiring:    classification=agent (model-invoked; skills: authoring-constitution, analysis-codebase, validation-feasibility — bare)
           router=NOT edited here (shared router) — PA-row broaden + validation-feasibility entry NOTED below
           triggers=agent <example> discovery (governance + feasibility, decoupled); graded work-context trigger rides on the mounted skill, not the agent
           rebinds=none owed (body grep-clean: no .humaninloop/ / DAG / catalog / MCP paths; ESSENTIAL-FLOOR.md ref is skill single-sourcing)
           single-source=referenced, not restated (persona points at validation-feasibility for the 6-class procedure; at authoring-constitution for the Essential Floor; feasibility-report owns report shape)
Trace (realized): see table below — #7 flipped flag-for-reconcile → folded-into-skill (homeless review HOMED)
```

---

## Pre-flight (mount-resolves gate — satisfied)

- **`validation-feasibility` landed BEFORE this mount.** `plugins/mochiko/skills/validation-feasibility/SKILL.md`
  confirmed present and **scope-aligned**: six cross-artifact contradiction classes, 3-state
  `feasible / needs-revision / infeasible` verdict, per-issue gate fuel, and G1 (plan artifacts, never the
  constitution). The live mount on `skills:` **resolves, does not dangle**. (A live mount of an unported skill
  would be a `verify-output` Tier-1 FAIL.)
- **Cross-wave dependency CLOSED.** `transform-validation-feasibility.md` (line 107 item a) recorded the open
  edge *"principal-architect (P3) must add `validation-feasibility` to its `skills:` list when re-broadened."*
  On-disk `skills:` line 40 now carries it → the edge is closed by this artifact.

---

## The re-broaden edits applied (per assess §Disposition line 150 — the RQ1-(i) branch)

The assess prescribed exactly this edit set *iff RQ1 keeps feasibility on PA*; reconcile chose (i), so all landed:

| # | Site | Setup-port (narrowed) → plan (re-broadened) | On-disk evidence |
|---|------|---------------------------------------------|------------------|
| 1 | **Anti-feasibility carve-out** (was ~L107–109: *"Out of scope: … is **not** your responsibility"*) | **REMOVED** | grep `not your responsibility` / `out of scope` / `Out of scope` = **0** |
| 2 | **`skills:` frontmatter** | `authoring-constitution, analysis-codebase` → `…, validation-feasibility` (bare, comma-separated — sibling-agent form) | L40 |
| 3 | **Framing** (body opener) | authoring-only → *"establishes **and evaluates** governance standards … and you review technical artifacts for cross-artifact feasibility"* | L43 |
| 4 | **Three-Part Rule** | *"Every standard you **write**"* → *"Every standard you **write or evaluate**"* (evaluate altitude restored) | L80 |
| 5 | **What You Produce** | 2 items → **3** (re-added **"Feasibility Reviews"**) — roadmap + CLAUDE.md sync **not** re-added (stay deferred) | L65–67 |
| 6 | **`## Feasibility Review` section** | re-added, **decoupled**; procedure deferred to the skill, not inlined | L120–126 |
| 7 | **Feasibility `<example>` block** | re-added; names artifact **types** (requirements/constraints/NFRs) + the 3-state verdict — no workflow/agent token | L21–28 |
| 8 | **Decouple wiring fix** (HIL L138 *"(that is the reviewer's job)"*) | → *"a separate reviewer's concern"* (role, not implied agent) | L124 |

**Body treatment = `port-with-edits` (not `redesign`), confirmed:** the setup port's structure and voice are
preserved intact (Core Identity, Quality Standards, Three-Part Rule, Your Judgment, What You Reject/Embrace,
Essential Floor all carried); the edits are localized re-broadenings + one skill mount. **Structural = `standalone`:**
one agent, one home; the feasibility *procedure* spins out to the partner skill, the agent itself does not split.

---

## G1 — HONORED (hard, artifact-cited)

The assess's Guardrail #G1 and reconcile §RQ2 both forbid re-creating the original constitution self-grade:

- **`validation-constitution` is NOT mounted** — grep of the on-disk artifact = **0** occurrences. `skills:`
  (L40) = `{authoring-constitution, analysis-codebase}` (setup producer role) ⊕ `{validation-feasibility}`
  (plan reviewer role) — **disjoint artifact domains**.
- **The persona does not re-create constitution grading.** Feasibility operates over **plan analysis/design
  artifacts, never the constitution** — stated twice: L51 (*"It operates over those artifacts, never the
  constitution"*) and L126 (*"you operate over technical analysis and design artifacts, never the constitution
  — that is a different artifact domain with its own validator"*). This mirrors the mounted skill's own G1
  (SKILL.md L27, L105).
- **No produce+grade of one artifact on one agent:** PA authors the *constitution* in setup (graded by the
  generic validator, a different agent) and reviews the *technical-analyst's* plan artifacts (a different agent).
  Never the same artifact. Independence holds by construction.

---

## Convention-wiring pass — all 6

1. **Classification** — agent (model-invoked via `<example>` blocks). `skills:` list grew **2→3** (bare
   comma-separated). Persona-vs-procedure split honored: the **persona keeps the care/judgment** (the adversarial
   stance L122, the *"infeasible verdict survives"* conviction L124); the **procedure** (six classes, per-issue
   evidence, verdict rendering) rides in the mounted `validation-feasibility` — L126 points at it explicitly
   rather than inlining it. ✅
2. **Router registration** — **NOT edited here (shared `skills/mochiko/SKILL.md`, out of this primitive's scope).**
   Two updates **NOTED** for the router-edit step: (a) broaden the `principal-architect` agent-row from setup-only
   governance to also its **plan feasibility-reviewer** role + the 3-skill list; (b) add the model-invoked
   `validation-feasibility` skill entry (already flagged in `transform-validation-feasibility.md` item 2). ⏸ noted
3. **Trigger phrasing** — the agent carries **Task-dispatch discovery** via its `<example>` blocks (governance +
   the re-added feasibility example L21–28, decoupled to artifact types + verdict). The **graded work-context
   trigger** lives on the mounted skill's `description` (`validation-feasibility` — *"review feasibility /
   cross-artifact contradictions / …"*), not on the agent — correct: agents don't carry graded phrase triggers. ✅
4. **Path rebinding** — **none owed.** Body grep-clean: no `.humaninloop/`, no `.workflow`, no DAG/catalog/MCP/
   `hil-dag` tokens. The one path (L118, `authoring-constitution`'s `references/ESSENTIAL-FLOOR.md`) is
   **legitimate skill single-sourcing** (assess explicitly cleared it), not a workflow path to rebind. Artifact
   **types** named generically. ✅
5. **Decouple persona/skill** — the *"(that is the reviewer's job)"* → *"a separate reviewer's concern"* fix
   landed (L124). **ZERO coupling deny-list tokens:** grep = 0 for `dispatch`, `workflow-agnostic`, every
   sibling-agent name (`technical-analyst` / `devils-advocate` / `requirements-analyst` / `qa-engineer` /
   `staff-engineer` / `task-architect` / `state-analyst`), `Phase 1/2/-1/-2`, and the workflow names
   `plan` / `setup`. **One role-noun** — `validator` (L126, *"its own validator"*) — used as **by-role
   independence language** (matches the mounted skill's own idiom, SKILL.md L105 *"the constitution validator"*);
   it decouples constitution-grading *out* to a separate role, it is not a dispatch/coupling reference. Surfaced
   for the verify pass to consciously accept or scrub (see §Discrepancy). ✅ (clean; one flagged role-noun)
6. **Single-source / de-duplicate** — the persona **references, never restates**: `validation-feasibility` for
   the 6-class procedure + verdict rendering (L126); `authoring-constitution`'s `ESSENTIAL-FLOOR.md` for the four
   Essential-Floor categories (L118, *"rather than working from a copy in this persona, so there is one source of
   truth"*); the `feasibility-report` template owns the report markdown shape (not reproduced in the persona). ✅

---

## Realized responsibility trace (assess rows flipped to final tags; ★ = delta this run)

Every HIL `principal-architect` responsibility, traced through the setup port (carried) and the plan delta (now):

| # | Responsibility (HIL) | Assess tag | **Realized tag** | Where / on-disk note |
|---|----------------------|-----------|------------------|----------------------|
| 1 | Greenfield constitution authoring | kept | **kept** | via `authoring-constitution` (L40, L49). Setup slice, untouched. |
| 2 | Brownfield constitution authoring | moved-to-sibling-skill | **moved-to-sibling-skill** | `authoring-constitution` greenfield\|brownfield branch (L49; example 3 L30–37). No separate brownfield skill. |
| 3 | Constitution quality grading (`validation-constitution`) | moved-to-validator | **moved-to-validator** · **G1** | generic `validator` (setup self-grade fix). ★ **NOT re-mounted** — grep `validation-constitution` = 0. No self-grade recreated. |
| 4 | Codebase analysis (`analysis-codebase`) | kept | **kept** | via `analysis-codebase` (L40, L50). Setup slice, untouched. |
| 5 | Evolution roadmap authoring (`authoring-roadmap`) | moved-to-other-cluster | **moved-to-other-cluster** | Deferred; **confirmed NOT re-added** — "What You Produce" has 3 items, no roadmap. Tracked, not dropped. |
| 6 | CLAUDE.md governance sync (`syncing-claude-md`) | moved-to-other-cluster | **moved-to-other-cluster** | Deferred; confirmed absent. Tracked, not dropped. |
| **7** | **Cross-artifact feasibility review** (the procedure: 6 conflict classes) | **flag-for-reconcile** | ★ **folded-into-skill** | **THE delta — homeless review HOMED.** Procedure → `validation-feasibility` (mounted L40; pointed at L126). Persona **retains the authorship/judgment capability** (What You Produce #3 L67; §Feasibility Review L120–126) — PA *is* the feasibility reviewer; only the step-by-step drops into the skill. No silent drop. |
| 7b | Feasibility verdict taxonomy (`feasible / needs-revision / **infeasible**`) | flag-for-reconcile | **kept** | All 3 states. Persona holds the *"`infeasible` verdict survives … a business-level decision to escalate, never a louder `needs-revision`"* stance (L124); the skill renders all three (Step 5). Escalation signal preserved. |
| 7c | Feasibility ≠ completeness division-of-labor (in/out-of-scope) | flag-for-reconcile + wiring | **kept** + decoupled | L124 keeps the split (*complete / alternatives-weighed / individually-measurable → "a separate reviewer's concern"*); the *"(reviewer's job)"→role* wiring fix is the decouple. |
| 7d | Architect-report artifact format | flag-for-reconcile | **moved-to-sibling-template** (separate primitive) | P13 → renamed **`feasibility-report`** (`transform-feasibility-report-template.md`). Skill points at it (SKILL.md L145). **Correctly absent** from this agent. |
| 8 | *"Review ONCE before completeness"* ordering + Feasibility-Rejection Loop (verdict-sink, routing) | moved-to-lead | **moved-to-lead (P1)** | The plan lead's orchestration. **Realized by ABSENCE** from the persona — grep confirms no `Phase` / ordering / rejection-loop / "Next Steps" / routing token re-inlined. |
| 9 | Persona-narrowing artifacts of the setup port (the carve-out; authoring-only framing; *write*-only Three-Part Rule) | kept-but-rebind *(if PA hosts feasibility)* | **kept-but-rebind** | The narrowing is **reversed**: carve-out removed (grep 0); *"and evaluates"* restored to framing (L43) + Three-Part Rule (L80); *"Feasibility Reviews"* re-added (L67). |

**Trace complete — every responsibility carries a tag. No silent loss.**
- **#7's homeless responsibility is now realized** (folded-into-skill + capability kept on the re-broadened
  persona), not lost — this is the assess's CRITICAL silent-drop #1 (*"ALREADY PORTED" is a trap*) closed.
- **#7b `infeasible`** (assess silent-drop #2) preserved as a distinct escalation state.
- **#3 G1** held: `validation-constitution` stays off PA — the original self-grade is **not** re-created.
- **#5 / #6** remain tracked deferrals (not re-added, not dropped).

---

## Independence attestation (non-negotiable — re-verified from the artifact)

- **PA reviewer skill-set ∩ producer skill-set = ∅.** PA mounts `{authoring-constitution, analysis-codebase}`
  (setup producer role) ⊕ `{validation-feasibility}` (plan reviewer role). The producer `technical-analyst`
  mounts `{authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling,
  patterns-api-contracts}` — all authoring/design, **disjoint** from PA's review skill.
- **Grades a different agent's output:** the feasibility review grades the **technical-analyst's** plan
  artifacts, never PA's own authoring. PA's setup-authored constitution is graded by the **generic `validator`**,
  not by PA.
- **Never produces + grades the same artifact.** Two independent reviewers (PA = feasibility, advocate =
  completeness) both grade the analyst's output; neither is the producer; the **lead** owns the clearing verdict.
- **G1:** `validation-constitution` is not on PA (grep 0); feasibility never touches the constitution (L51, L126).

---

## Discrepancy check (on-disk artifact vs reconcile spec)

**No material deviation.** The artifact faithfully realizes reconcile's finalized P3 disposition
(`port-with-edits × standalone`; re-broaden to feasibility reviewer; mount NEW `validation-feasibility`; G1),
and JOB 3's tag assignments (#7 folded-into-skill; #7b/#7c kept; #8 moved-to-lead). All eight prescribed edits
landed; the mount resolves; the decouple scan is coupling-clean.

**One item surfaced for the verify pass to consciously accept or scrub (NOT a coupling defect):**
- **L126 `its own validator`** contains the literal token `validator`, which is also the name of a sibling
  agent. Here it is **by-role independence language** (the constitution has *its own* separate validating role —
  it decouples grading *away* from this persona), matching the mounted skill's established idiom (SKILL.md L105
  *"the constitution validator"*, L27). It is **not** a dispatch/coupling reference to the `validator` agent.
  Because the assess's clean-scan of the setup port relied on this token being *absent*, it is surfaced so the
  reviewer can either accept it as role-language (recommended — it is exactly the "independence stated by role"
  the decouple pass wants) or scrub it to a non-agent noun (e.g. *"its own independent grader"*). The lead
  decides; this producer does not.

---

## Downstream wiring edges (NOT this agent's body — recorded for the run)

- **Router (`skills/mochiko/SKILL.md`, shared)** — broaden the `principal-architect` agent-row to its
  cross-workflow role (setup governance **+** plan feasibility review) with the 3-skill list; and add the
  model-invoked `validation-feasibility` entry (also flagged in `transform-validation-feasibility.md` item 2).
- **REGISTRY** — flip the `principal-architect` row note to record the plan re-broaden (feasibility reviewer;
  `validation-feasibility` mounted); keep `authoring-roadmap` / `syncing-claude-md` as tracked deferrals.
- **`plugin.json`** — `principal-architect` is already in the agents array (setup port); **no add** owed by this
  delta edit (verify at `plugins/mochiko/.claude-plugin/plugin.json`).

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Disposition applied:** `port-with-edits ×
standalone` (re-broaden + 1 skill mount) · **Role:** apply + wire only — no grade. Hand the edited artifact +
this trace to `verify-output` (run by a **different** agent).
