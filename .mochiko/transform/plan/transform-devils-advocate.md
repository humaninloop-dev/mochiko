# TRANSFORM — devils-advocate (P4)

**Source disposition:** `assess-devils-advocate.md` (port-with-edits × standalone, light re-mount)
**Reconcile verdict:** `reconcile.md` RQ1 option (i) — `kept-but-rebind` (re-mount P9, **completeness-only**)
**Run:** `/mochiko:transform-cluster` (plan-core) · **Phase 3 (transform)** · **Date:** 2026-06-30
**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes`
**Role:** apply the finalized disposition + run the convention-wiring pass. **NOT** grade (that is
`verify-output`, run by a different agent). **Edited file:** `plugins/mochiko/agents/devils-advocate.md`.

---

## Output block (transform-recipes format)

```
TRANSFORM: devils-advocate
Applied:   port-with-edits × standalone + wiring-pass  (light — a skill re-mount)
Artifacts: plugins/mochiko/agents/devils-advocate.md  (edited — 3 coupled edits)
New partners: NONE  (a re-mount adds no split/promote/pair product; validation-feasibility +
              principal-architect re-broaden are SIBLING transforms, not this agent's)
Wiring:    classification=agent (skills: analysis-specifications, validation-plan-artifacts — both review)
           router=NOT edited here (shared router) — broadening NOTED below (separate wiring step)
           triggers=clean (self-reference "Devil's Advocate→an adversarial reviewer" already role-decoupled)
           rebinds=none owed (body grep-clean: no .humaninloop/ / DAG / catalog / MCP paths)
           single-source=referenced, not restated (persona points at the mounted skills for procedure)
Trace (realized): see table below — R7 flipped moved-to-other-cluster → kept-but-rebind (LIVE MOUNT)
```

---

## Pre-flight (sequencing gate — satisfied)

- **P9 landed BEFORE this re-mount.** `plugins/mochiko/skills/validation-plan-artifacts/SKILL.md` confirmed
  present → the live mount **resolves**, does not dangle. (A live mount of an unported skill would be a
  `verify-output` Tier-1 FAIL; the hard sequencing constraint from the assess is met.)
- **`validation-task-artifacts` confirmed ABSENT** in mochiko (`plugins/mochiko/skills/validation-task-artifacts`
  does not exist) → it **stays stubbed** in the comment, NOT live-mounted (a live mount would dangle). Tasks
  cluster deferred; re-mount when it ports.

## The three coupled edits applied (per the assess re-mount site, lines 25–29 + 36–40)

| # | Site | Before → After | Status |
|---|------|----------------|--------|
| 1 | **Frontmatter `skills:`** (was L29) | `skills: analysis-specifications` → `skills: analysis-specifications, validation-plan-artifacts` (bare comma-separated — matches every sibling agent's form) | ✅ applied |
| 2 | **Stub comment** (was L25–28) | dropped the `validation-plan-artifacts` deferral line; reworded header to `# analysis-specifications + validation-plan-artifacts mounted (specify + plan slices).` / `# Deferred — NOT dropped — re-mount here when its cluster ports:` / `#   validation-task-artifacts  (tasks cluster — task-artifact review)` — only `validation-task-artifacts` remains noted as deferred | ✅ applied |
| 3 | **Body "Skills Available"** (L34–40) | intro singular→plural ("specialized skills that provide"); ADDED bullet `**mochiko:validation-plan-artifacts**: …completeness review of planning artifacts (requirements, data-model, contracts) — coverage, measurability, consistency, and presence — with severity classification and the structured verdict format`; closing line "invoke it"→"invoke the relevant skill" (number agreement) | ✅ applied |

**Persona UNCHANGED** (Core Identity → What You Embrace carried verbatim, as the disposition requires). The
edits touch only the frontmatter and the "Skills Available" list — not one persona line.

**R11 optional `<example>` (planning-completeness, decoupled) — NOT added.** The assess marked it optional /
RQ1-informed / not required for the mount to function; the disposition handed to transform is "light — a skill
re-mount", so the minimal-treatment governor applies. The description keeps its two spec `<example>` blocks
unchanged. (Discoverability for plan-artifact review is carried by the router broadening — noted below — not by
adding a persona example this run.)

## Role boundary held — COMPLETENESS-ONLY (RQ1 option i)

devils-advocate is plan's **completeness** reviewer ONLY. It does **NOT** gain feasibility (that is
`principal-architect` + the new `validation-feasibility` skill — sibling transforms). No
feasibility / buildability / contradiction-across-artifacts responsibility was added to this persona.

- The added bullet is scoped strictly to completeness: *coverage / measurability / consistency / presence.*
  No feasibility, buildability, or cross-artifact-contradiction language.
- The mounted `validation-plan-artifacts` skill itself draws the seam: it grades cross-artifact **consistency**
  (design honors the decisions) but **hands off** cross-artifact **contradiction / NFR-feasibility /
  buildability** to `mochiko:validation-feasibility`. So even within plan review, the feasibility lens routes
  away from this agent — the boundary is enforced by *which skill is mounted*, not by prose.
- Role-boundary scan of the final file: the only `contradiction` hit is the pre-existing, untouched persona
  heuristic **"### 5. Contradictions and Conflicts"** (intra-spec: requirements that conflict with each other /
  inconsistent terminology / mutually exclusive acceptance criteria) — the skeptic's generic disposition for
  SPEC review, present since the specify port, NOT added plan-feasibility responsibility.

## Independence — preserved by construction (non-negotiable)

- devils-advocate mounts **`analysis-specifications` + `validation-plan-artifacts`** — confirmed **both are
  review/grade skills; NEITHER authors** (description checks: "reviewing an already-drafted specification" /
  "grade a producer's plan artifacts"). **No authoring skill mounted.**
- It **grades the `technical-analyst`'s output** (a *different* agent). technical-analyst mounts
  `authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling,
  patterns-api-contracts` — all authoring/design, **disjoint** from devils-advocate's review set.
- Producer skills ∩ this validator's skills = **∅**. No agent both produces and grades the plan artifacts.
  Port hazard honored: did not co-mount a review skill on the producer, nor an authoring skill on this reviewer.

## Convention-wiring pass — all 6

1. **Classification** — agent (model-invoked via `<example>` blocks). `skills:` list grew 1→2; persona-vs-procedure
   split honored (procedure rides in the two mounted skills; persona is the skeptic's disposition). ✅
2. **Router registration** — **NOT edited (shared router — out of scope this task).** **Update NOTED:**
   `skills/mochiko/SKILL.md:83` currently reads "specify-cluster adversarial critic … (skills: analysis-specifications)";
   it should broaden to the cross-workflow reviewer role + the 2-skill list (its plan **completeness** role),
   e.g. "specify + plan adversarial completeness reviewer … (skills: analysis-specifications, validation-plan-artifacts)".
   Left for the router-edit step. ⏸ noted
3. **Triggers** — kept clean. The canonical self-reference (≈L32) "You are the **Devil's Advocate**—an
   adversarial reviewer who finds what others miss" names *itself* and immediately decouples to **role** —
   confirmed still clean (self-reference, not a sibling name). No new triggers added this run. ✅
4. **Path rebinding** — none owed. Body grep-clean: zero `.humaninloop/` / `.workflow` / DAG / catalog / MCP /
   `hil-dag` tokens. The two `skills:` refs are bare ported names that resolve in mochiko. ✅
5. **Decouple persona/skill** — re-scanned final file: **ZERO deny-list tokens** — no sibling-agent names
   (requirements-analyst / technical-analyst / principal-architect / task-architect / staff-engineer /
   qa-engineer / state-analyst / ui-designer / transform-producer / validator all absent), no "dispatch",
   no "workflow-agnostic" / independence-by-declaration, no injected workflow modes/paths/phases. The new
   bullet introduced none. ✅
6. **Single-source / de-duplicate** — the persona references the mounted skills for the procedure rather than
   restating checklists (the existing single-source pointer for the spec taxonomy stays; the new bullet points
   at `validation-plan-artifacts` for the plan-completeness procedure, not a copy of it). ✅

## plugin.json

devils-advocate is **already** in the agents array (`plugins/mochiko/.claude-plugin/plugin.json:26`,
`"./agents/devils-advocate.md"`). **Confirmed — no add.** Not edited.

---

## Realized responsibility trace (flipped to final tags; ★ = delta this run)

| # | Responsibility | Assess tag | **Realized tag** | Note |
|---|----------------|-----------|------------------|------|
| R1 | Adversarial review of **specifications** via `analysis-specifications` | kept | **kept** | specify slice untouched, still live |
| R2 | Emit review **verdict** (`ready` / `needs-revision` / `critical-gaps`) | kept | **kept** | P9 emits the SAME 3-state vocabulary; lead owns the clearing verdict |
| R3 | Generate **clarifying questions / issues** (actionable, with suggestions) | kept | **kept** | unchanged |
| R4 | **Severity calibration** (Critical/Important/Minor) | kept | **kept** | P9 uses the same model |
| R5 | Adversarial **persona/values** | kept | **kept** | verbatim — persona untouched |
| R6 | **"What You Hunt For"** heuristics (5 categories) + single-source pointer | kept | **kept** | unchanged; incl. generic "Contradictions and Conflicts" (intra-spec, not plan-feasibility) |
| **R7** | Review of **planning artifacts** (requirements / data-model / contracts) via `validation-plan-artifacts` | moved-to-other-cluster (stub) | ★ **kept-but-rebind (LIVE MOUNT)** | **THE delta.** Flipped stub → live mount; `mochiko:validation-plan-artifacts` resolves (P9 landed). **Completeness-only** |
| R8 | Review of **task artifacts** via `validation-task-artifacts` | moved-to-other-cluster | **moved-to-other-cluster (stays stubbed)** | tasks deferred/unported; live mount would dangle — **kept in comment, NOT dropped** |
| **R9** | `skills:` frontmatter resolution | kept-but-rebind | ★ **kept-but-rebind** | now resolves TWO ported skills; stub comment shrank to the one deferred line |
| R10 | `description` breadth: spec example | kept | **kept** | both spec `<example>` blocks unchanged |
| R11 | `description` breadth: planning-completeness example | kept (optional re-add) | **dropped-as-optional (not owed)** | NOT re-added — light disposition + minimalism governor; discoverability via router broadening instead. *Not a capability drop:* the plan-completeness capability lives in the mounted skill (R7); only the optional discovery example was declined |
| R12 | Caller-side **phase phrasing** | moved-to-lead | **moved-to-lead (P1)** | stays externalized to `agent-dispatch`; no example re-introduced it |
| R13 | Dispatch + verdict consumption + FAIL-loop driving | moved-to-lead (P1) | **moved-to-lead (P1)** | the plan lead's orchestration, not this agent's body |
| R14 | Upstream **handoff** (reviewed artifacts exist) | moved-to-lead (P1) | **moved-to-lead (P1)** | analyst → {requirements/data-model/contracts} → advocate; lead-wired |
| R15 | Role as **independent validator** (separate agent, disjoint skills) | kept (structural) | **kept** | re-confirmed: review-only skills, disjoint from `technical-analyst` |
| R16 | **Incremental-review mode** (procedure) | folded-into-skill + moved-to-lead | **folded-into-skill** (procedure in P9) **+ moved-to-lead (P1)** (mode selection) | the skill owns the procedure; the lead picks the mode |
| R17 | Plan **reviewer ordering** (architect-once → advocate; skip-re-review) | moved-to-lead (P1) | **moved-to-lead (P1)** | the lead's sequencing, not this agent's body |

**Trace complete — every responsibility carries a tag. No silent loss.**
- R7's deferred capability is now **realized** (live mount), not lost.
- R8 is explicitly **preserved-by-reference** (stub comment), not dropped.
- R11 (optional discovery example) was **declined as not-owed** under the light disposition — and is *not* a
  capability loss: the plan-completeness review capability is carried by R7's mounted skill; only the optional
  description example was deferred (re-addable later, RQ1-informed). Logged here so the decline is auditable.

## Downstream wiring edges (NOT this agent's body — recorded for the run)

- **Router (`skills/mochiko/SKILL.md:83`)** — broaden the devils-advocate entry from "specify-cluster …
  (skills: analysis-specifications)" to its cross-workflow **completeness-reviewer** role + 2-skill list. Separate
  router-edit step (shared file; intentionally untouched here).
- **REGISTRY** — flip the devils-advocate row note "validation-plan-artifacts … deferred" →
  "validation-plan-artifacts re-mounted (plan, completeness-only)"; `validation-task-artifacts` stays deferred (tasks).

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Disposition applied:** port-with-edits ×
standalone (light re-mount) · **Role:** apply + wire only — no grade. Hand the edited artifact + this trace to
`verify-output` (run by a different agent).
