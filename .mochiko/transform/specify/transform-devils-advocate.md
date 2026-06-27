# TRANSFORM: devils-advocate (P3)

**Source:** `human-in-loop/plugins/humaninloop/agents/devils-advocate.md`
**Artifact:** `plugins/mochiko/agents/devils-advocate.md`
**Run:** transform-cluster / `specify` (specify-core scope) · Phase 3 (transform) · **Applied:** 2026-06-27
**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes`
**Inputs:** reconcile.md §D row P3 + §E P3 trace + §G wiring + Agenda 7a · assess-devils-advocate.md · principal-architect.md / validator.md (convention shape) · agent-dispatch.md (externalization target)

> Transform applies a finalized, human-gate-ACCEPTED disposition; it does not decide and does not grade its own output. Independent grading is `mochiko:verify-output`, run by `mochiko:validator` (a DIFFERENT agent).

---

## Applied

```
TRANSFORM:    devils-advocate
Disposition:  port-with-edits × standalone (VALIDATOR; CONFIRMED at the Phase-2 human gate)
Applied:      port-with-edits × standalone + convention-wiring pass
Artifacts:    plugins/mochiko/agents/devils-advocate.md (CREATE)
New partners: NONE — the producer↔validator pair already exists (this agent IS the validator).
              No split / promote / merge / absorb. Nothing promoted onto this agent.
Structural:   no change to placement or to the agent's role; one home (the specify-pair critic).
```

Body treatment was `port-with-edits` (not `keep-verbatim`): the persona prose is keep-verbatim-grade and keystone-clean, but four localized edits are owed — (1) trim `description` to the spec slice, (2) prune "Skills Available" to one skill, (3) stub the two deferred validation skills out of the live `skills:` list, (4) confirm independence-by-role + non-lossy soft-dedupe pointer. Not `redesign`: the body assumes no kernel and the approach is sound.

---

## The four edits (what changed, and why)

### Edit 1 — `skills:` frontmatter: keep only `analysis-specifications`; stub the two deferred (Agenda 7a)
- HIL: `skills: analysis-specifications, validation-plan-artifacts, validation-task-artifacts`
- mochiko: `skills: analysis-specifications` (bare name — see convention note below)
- `validation-plan-artifacts` + `validation-task-artifacts` **removed from the live list** so the frontmatter never points at an unported skill (a live mount would dangle → `verify-output` Tier-1 FAIL).
- **They are STUBBED, not dropped.** A YAML comment immediately above the `skills:` line names both, tags them *Deferred — NOT dropped — re-mount here when their clusters port*, and records the cluster each belongs to. The durable re-mount-edge record is R7/R8 below. This realizes the contract's "rebound by reference only."
- **Stub mechanism = YAML comment at the stub site** (not a body blockquote): keeps the persona body keystone-pure while marking the exact line where the two skills re-attach. The live `skills:` value is grep-clean (one resolvable skill).

### Edit 2 — `description` trimmed to the spec slice
- Opening line dropped the deferred-domain nouns "planning artifacts, and task artifacts" → "stress-tests **specifications**…". Added a decoupled output clause ("returns a severity-ranked gap report with clarifying questions and a **recommended** verdict" — "recommended" because the **lead** owns the clearing verdict, reconcile §1b).
- HIL `<example>` 2 (planning artifacts / data model / "next phase") and `<example>` 3 (task artifacts / "before implementation begins") **removed** — out-of-scope domain nouns + caller-side phase phrasing trim out together with the deferred skills (R11/R12).
- HIL `<example>` 1 (spec review) **reworked** and a **second** spec-slice example added → two examples, matching the mochiko 2-example convention (`principal-architect`, `validator`). Both are spec-slice, decoupled (no "before planning"/"next phase"), and surface the verdict vocabulary the agent legitimately produces.

### Edit 3 — "Skills Available" pruned to one skill, namespace rebound
- Three HIL bullets → one bullet: **`mochiko:analysis-specifications`** (`humaninloop:` → `mochiko:` rebind in the body ref).
- Intro reworded singular ("a specialized skill").

### Edit 4 — independence-by-role confirmed + soft-dedupe as a pointer
- **Independence by role:** added one `What You Reject` bullet — *"Authoring or fixing the spec yourself — you surface the gaps and hand them back; writing the spec is the author's job, not the reviewer's."* States independence as a **posture** (reviewer ≠ author), names **no sibling agent** and **no workflow**, keystone-true on any job. Mirrors `validator.md` ("you produce verdicts, never content… the author's job"). Independence remains **structural** (separate agent + disjoint skill set: a review-only skill, no authoring skill); the persona asserts the posture, not the mechanism.
- **Soft-dedupe (optional) realized as a non-lossy pointer:** kept the five "What You Hunt For" categories (they are persona-load-bearing — they characterize *how* this skeptic thinks) and added one trailing line naming **`mochiko:analysis-specifications`** as the single source of truth for the detailed gap taxonomy / severity rubric / output format. Mirrors `principal-architect`'s Essential-Floor pointer. Chosen over content-removal because P6's final ported shape can't be assumed to 1:1 cover the categories, so deletion risked specificity loss; the pointer reduces drift without dropping persona.

---

## Convention-wiring pass (all five ran)

| # | Pass | Result for this agent |
|---|------|------------------------|
| 1 | **Classification** | Agent (not user/model-classified like a skill). `skills:` list set to one skill. Persona-vs-procedure split honored: disposition/values in the agent, the gap-finding *procedure* in `analysis-specifications`. `model: opus`, `color: red` (validator-color convention, matches `validator.md`). |
| 2 | **Router registration** | **Deferred to the separate router-edit step** (reconcile §D "EDIT `skills/mochiko/SKILL.md`"). Per task scope, the shared router was **NOT** edited here. To register: agent `devils-advocate` (VALIDATOR / adversarial critic; reach for spec gap-review with a verdict). |
| 3 | **Trigger phrasing** | Agents trigger via `<example>` blocks, not graded skill triggers. Two spec-slice examples set to the review *work-context*; verdict vocabulary surfaced. (Model-invoked trigger reliability lives on the skill, not the agent.) |
| 4 | **Path rebinding** | No hardcoded paths in this agent (grep-clean). Namespace rebind `humaninloop:` → `mochiko:` applied to the one body skill ref → `kept-but-rebind`. Kernel/DAG/catalog/MCP paths: none present (nothing to drop). |
| 5 | **Decouple persona/skill** | Persona body **0 deny-list hits** (confirmed below). Independence stated by role; no sibling-agent name; no "dispatch"; no "workflow-agnostic"; no injected workflow modes/paths/phases. Caller-side phase phrasing externalized (removed from the agent; home = the caller's dispatch brief per `agent-dispatch.md`). |

---

## Realized responsibility trace (every responsibility carries a final tag)

| # | Responsibility | Final tag | Realized in the artifact |
|---|----------------|-----------|--------------------------|
| R1 | Adversarial review of **specifications** — gaps, ambiguities, edge cases, assumption gaps, contradictions | **kept** | Core Identity + "What You Hunt For" (verbatim); via `mochiko:analysis-specifications` |
| R2 | Emit review **verdict** (`ready` / `needs-revision` / `critical-gaps`) | **kept** | "What You Produce" #2 (verbatim); framed as a *recommended* verdict in the description (lead owns the clearing verdict) |
| R3 | Generate **clarifying questions** (product-framed, concrete options) | **kept** | "What You Produce" #3 (verbatim); surfaced in example 2 |
| R4 | **Severity calibration** (critical = "will break in production") | **kept** | Quality Standards + Adversarial Calibration (verbatim) |
| R5 | Adversarial **persona/values** — never rubber-stamp, never downgrade, never approve with zero findings, require evidence | **kept** | Core Identity / Adversarial Calibration / What You Reject / What You Embrace (verbatim) |
| R6 | **"What You Hunt For"** review heuristics (5 categories) | **kept** (soft-dedupe done as a *pointer*, non-lossy) | Categories kept verbatim + one canonical-source pointer line to `analysis-specifications` |
| R7 | Review of **planning artifacts** via `validation-plan-artifacts` | **moved-to-other-cluster** (plan cluster) — **stub / rebind-by-reference; NOT dropped** | Removed from live `skills:`; named in the YAML stub comment; re-mount edge logged here for when plan ports |
| R8 | Review of **task artifacts** via `validation-task-artifacts` | **moved-to-other-cluster** (tasks cluster) — **stub / rebind-by-reference; NOT dropped** | Removed from live `skills:`; named in the YAML stub comment; re-mount edge logged here for when tasks ports |
| R9 | `skills:` frontmatter resolution | **kept-but-rebind** | `analysis-specifications` (bare in frontmatter per convention) → body ref `mochiko:analysis-specifications`; two deferred refs stubbed out |
| R10 | `description` breadth: spec example | **kept** (trimmed to spec slice) | Example 1 reworked; example 2 added (both spec-slice) |
| R11 | `description` breadth: plan/tasks examples + domain nouns | **moved-to-other-cluster** (deferred with R7/R8) | HIL examples 2 & 3 removed; opening-line domain nouns removed; return when those clusters port |
| R12 | Caller-side phase phrasing ("before planning", "next phase", "before implementation") | **moved-to-lead** | Removed from the agent entirely; home = the caller's dispatch brief (`agent-dispatch.md`), filled by the lead — NOT baked into the persona |
| R13 | Dispatch point + verdict consumption + **FAIL-loop driving** | **moved-to-lead** | Not in this agent (rehome-orchestration → the `specify` supervisor's bounded loop, §B.1). The persona names no loop. |
| R14 | Upstream **handoff** (assumes spec.md exists to review) | **moved-to-lead** | Not in this agent (explicit handoff edge analyst → spec.md → advocate, wired by the lead, §B.5) |
| R15 | Role as **independent validator** (separate agent, disjoint skills) | **kept** (structural) | Disjoint skill set preserved (review-only); independence-by-role posture added to What You Reject; no authoring skill, no sibling named |

**No responsibility is untagged → trace complete (done-condition met). No `dropped` tags** — the two deferred capabilities (R7, R8) are explicitly preserved by reference (stub + logged re-mount edge), not lost. No `moved-to-validator` (the validator already exists; nothing promoted onto it).

---

## Independence (by ROLE) — confirmation

- **Structural:** this agent mounts a **review-only** skill (`analysis-specifications`); it carries **no authoring skill**. The producer (a different agent, not named here) carries the authoring skills. Skill sets are disjoint, on different agents → no agent both produces and grades. The Phase-2 gate CONFIRMED the pair (no construction needed — the sharp contrast with `setup`).
- **By role, not by mechanism:** the persona states the posture ("you surface gaps and hand them back; writing the spec is the author's job") and names **no sibling agent** and **no workflow**. "the author" is a generic role (matches `validator.md` usage), not a sibling name. Independence is carried by the disjoint-skill + separate-agent **structure**, exactly as `agent-dispatch.md` prescribes ("the persona asserts nothing about this — the structure carries it").
- **Verdict-ownership is NOT claimed here:** the agent produces a *recommended* verdict; the **lead** owns the clearing verdict and the loop (reconcile §1b, §B.1). The persona contains no loop/done-condition/gate language → no leak.

## Decoupling scan (the run's empirical data point)

Persona body (Core Identity → What You Embrace): **0 deny-list tokens.**
- sibling-agent names → **NONE** (the only `devils-advocate` mentions are the agent naming *itself* in the description examples — allowed, as in `principal-architect`/`validator`).
- `dispatch` → **NONE** · `workflow-agnostic` / independence-by-declaration → **NONE** · injected workflow modes/paths/phases in the persona → **NONE**.
- kernel/DAG/catalog/MCP, hardcoded paths → **NONE**.
- Generic experiential vocabulary kept verbatim ("mid-sprint", "in a release", "will break in production") is keystone-true persona flavor, not workflow coupling.
- The HIL coupling that DID exist (plan/tasks domain nouns + phase phrasing) lived only in the description/examples (caller context) and is gone after the scope trim + externalize. Net: a clean positive for the decoupling doctrine — no deny-list token slipped into a persona; coupling was caller-side and trimmed.

---

## Deviations / judgment calls (flagged for the independent grader)

1. **Frontmatter `skills:` uses the BARE name `analysis-specifications`, not `mochiko:analysis-specifications`.** The task prompt wrote the prefixed form for the frontmatter, but this is shorthand for "the rebound mochiko skill": (a) both live, PASS'd mochiko agents (`principal-architect.md`, `validator.md`) use **bare** names in their frontmatter `skills:` list, and (b) reconcile §G itself lists the P3 rebind target as bare `analysis-specifications`. The namespace rebind is realized in the **body** "Skills Available" ref (`mochiko:analysis-specifications`), exactly as both existing agents do. This is convention-alignment, not a scope change — and it keeps the live `skills:` value grep-clean for the Tier-1 dangling-ref check.
2. **Deferred-skill stub = YAML comment at the `skills:` line** (rather than a body blockquote). Chosen to log the re-mount edge precisely at the stub site while keeping the persona body keystone-pure. The full, durable re-mount record is R7/R8 above; the artifact comment is the courtesy marker. (`verify-output` reads artifact + this trace; both name the deferral as not-mounted.)
3. **Two small additive persona edits beyond a pure trim:** one `What You Reject` independence-by-role bullet, and one canonical-source pointer line under "What You Hunt For." Both are explicitly requested by the disposition ("confirm independence by role"; "optional soft dedupe"), keystone-framed, sibling-free, and mirror established mochiko precedents (`validator.md`, `principal-architect.md`). Net persona is still keep-verbatim-grade plus these two minimal, non-lossy additions.
4. **Not done in this task (by instruction / scope):** the shared router (`skills/mochiko/SKILL.md`) was **not** edited — router registration is the separate §D row. `agent-dispatch.md` was **not** edited — it is a generic caller-side template; specify-specific dispatch phrasing is the lead's (P1's) job. `loop-discipline` was **not** touched.

## Hand-off

- **Artifact:** `plugins/mochiko/agents/devils-advocate.md`
- **This trace:** `.mochiko/transform/specify/transform-devils-advocate.md`
- **Next:** independent grade via `mochiko:verify-output`, run by `mochiko:validator` (a DIFFERENT agent than this producer). Pending downstream wiring: router registration of the agent; re-mount of `validation-plan-artifacts` / `validation-task-artifacts` when the plan/tasks clusters port; REGISTRY flip on cluster finalize.

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Disposition:** port-with-edits × standalone (CONFIRMED)
