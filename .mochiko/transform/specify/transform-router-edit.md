# Transform (realized trace) — router registration, cluster `specify`

Run: `transform-cluster specify` · Phase 3 (transform-recipes) · Step: **convention-wiring pass — router registration** (run once for the whole cluster to avoid write conflicts)
Producer: `transform-producer` · Skill: `mochiko:transform-recipes` (convention-wiring pass, step 2 "Router registration")
Applied: 2026-06-27 · Authoritative inputs: `reconcile.md` §G (Router registration row), §D (final names/classifications), §E (per-primitive router trace tags)

> **Scope of this artifact.** This is the *single consolidated router edit* for the specify cluster — the one shared write that all the per-primitive ports depend on. It is NOT a full primitive transform; each primitive's body port (P1–P13) is applied separately. This trace covers only the `router=registered?` line of the convention-wiring pass, realized for every specify primitive at once.
>
> **Not graded here.** Independence is non-negotiable: this edit is handed to `verify-output` (run by a different agent) for grading. No self-grade performed.

---

## TRANSFORM: specify-cluster router registration

```
Applied:   wiring-pass (router-registration step only) — one consolidated edit
Artifacts: plugins/mochiko/skills/mochiko/SKILL.md (EDITED — the user-invoked router)
New partners: none (no split/promote in this cluster; the producer↔validator pair already exists)
Wiring:    classification=(set per-primitive in each body port, not here)
           router=REGISTERED (this artifact)
           triggers=(set per-primitive in each skill's description, not here)
           rebinds=(namespace/path rebinds applied in each body port, not here)
```

## What was edited

One file: `plugins/mochiko/skills/mochiko/SKILL.md`. Three non-overlapping insertions, each mirroring the existing **setup-cluster** indexing format (and, for templates, the **doctrine-section** `(template)` precedent):

1. **New `### Specify cluster` section** inserted after the `### Setup cluster` section and before `### Entry point` — a `| Skill | Reach when |` table holding the four skills + three templates.
2. **`/mochiko:specify` row** appended to the existing `### Entry point (user-invoked — you run it)` table, directly after `/mochiko:setup`.
3. **Two agent rows** (`requirements-analyst`, `devils-advocate`) inserted into the `### Agents (dispatched by the supervisor)` table, before the generic shared `validator`.

## What was registered (the §G list, realized)

| Kind | Primitive | Classification / role registered | Router placement | Source |
|---|---|---|---|---|
| Command | `/mochiko:specify` | **user-invoked** (hint — indexed, not auto-fired) | Entry point table | §D P1, §G |
| Agent | `requirements-analyst` | **producer** — authors `spec.md`; `skills: authoring-requirements, authoring-user-stories` | Agents table | §D P2, §G |
| Agent | `devils-advocate` | **adversarial critic / validator team-role** — grounded gap review + *recommended* verdict (lead owns the clearing verdict); `skills: analysis-specifications` | Agents table | §D P3, §A 1a/1b, §G |
| Skill | `analysis-iterative` | **model-invoked**; marked **general/shared** (cross-cluster brainstorm + spec-input enrichment) | Specify cluster table | §D P5, §A 5b, §G |
| Skill | `analysis-specifications` | **model-invoked**; spec gap-finding (stays a gap-finder, no clearing verdict) | Specify cluster table | §D P6, §A 1d, §G |
| Skill | `authoring-requirements` | **model-invoked**; FR-XXX / SC-XXX | Specify cluster table | §D P7, §G |
| Skill | `authoring-user-stories` | **model-invoked**; P1/P2/P3 + Given/When/Then | Specify cluster table | §D P8, §G |
| Template | `spec-template` | inert template (`(template)` marker) | Specify cluster table | §D P11, §G |
| Template | `analyst-report-template` | inert template (`(template)` marker) | Specify cluster table | §D P12, §G |
| Template | `advocate-report-template` | inert template (`(template)` marker) | Specify cluster table | §D P13, §G |

`loop-discipline` — **already registered** (Doctrine section); NOT re-registered here. Its additive Gap Classification content is a separate `loop-discipline` edit (§D "loop-discipline EDIT" row, §A Agenda 3b) and is intentionally NOT duplicated in this router pass.

## Trace (realized) — router-registration tag per primitive

The router entry is the `classification + router → kept-but-rebind (wiring)` slice of each primitive's §E trace. Realized here:

- P1 `/mochiko:specify` — router discoverability entry → **kept-but-rebind** (registered as user-invoked hint in Entry point table). ✔
- P2 `requirements-analyst` — router/discoverability → **kept-but-rebind** (registered as producer; skills list mirrored: `authoring-requirements, authoring-user-stories`). ✔
- P3 `devils-advocate` — router/discoverability → **kept-but-rebind** (registered as critic; skills list = `analysis-specifications` only — the deferred `validation-plan-artifacts`/`validation-task-artifacts` deliberately NOT listed, per §A 7a / §E P3). ✔
- P5 `analysis-iterative` — `#11 triggers / router → kept-but-rebind (wiring)` → **kept-but-rebind** (registered model-invoked, **general/shared** marker per §A 5b/7b). ✔
- P6 `analysis-specifications` — router/discoverability → **kept-but-rebind** (registered model-invoked, gap-finder framing; no clearing verdict, per §A 1d). ✔
- P7 `authoring-requirements` — `classification + router → kept-but-rebind (wiring)` → **kept-but-rebind** (registered model-invoked). ✔
- P8 `authoring-user-stories` — `router → kept-but-rebind` → **kept-but-rebind** (registered model-invoked). ✔
- P11 `spec-template` — `R8 discoverability → kept-but-rebind (wiring)` → **kept-but-rebind** (registered as `(template)`). ✔
- P12 `analyst-report-template` — discoverability → **kept-but-rebind** (registered as `(template)`). ✔
- P13 `advocate-report-template` — `R8 discoverability → kept-but-rebind` → **kept-but-rebind** (registered as `(template)`). ✔

Every primitive in §D that is **CREATE/EDIT** and router-eligible now has a discoverability home. No router-registration responsibility from §E is left homeless.

## Exclusions — confirmed NOT registered

**Dissolved primitives** (§D: NOT created — absorbed/dissolved; must not appear in the router):
- `state-analyst` (P4 — absorb-into-lead) — NOT registered. ✔
- `strategy-specification` (P9 — absorb-into-lead + dedupe) — NOT registered. ✔
- `strategy-core` (P10 — drop × dedupe-into-loop-discipline) — NOT registered. ✔
- `context-template` (P14 — drop × absorb-into-lead) — NOT registered. ✔

**Deferred primitives** (out of CORE scope this run):
- `ui-designer` (design track, deferred) — NOT registered. ✔
- `validation-plan-artifacts` / `validation-task-artifacts` (devils-advocate's two deferred skills, rebind-by-reference only, §A 7a) — NOT registered, and NOT listed in `devils-advocate`'s skills cell. ✔

A grep of the edited router for these eight names returns zero hits in the new content.

## Deviations / notes (for the grader)

1. **Templates listed inline with a `(template)` marker, under a "model-invoked" section header.** The setup cluster (the format the task said to mirror) has no templates, so the template-listing format was taken from the established **doctrine-section precedent** — `workflow-contract (template)` and `agent-dispatch (template)` already sit in a table whose header reads "(model-invoked — …)". The section header names the dominant class; the `(template)` marker disambiguates the inert rows. This matches existing router convention; no new format was invented. (If the grader prefers templates in a separate sub-table, that is a router-wide format change that should also touch the doctrine section — out of scope for this single-cluster registration.)
2. **`analysis-iterative` marked `(general/shared)` in the name cell**, mirroring the `(template)` suffix style, to signal it is cross-cluster (per §A 5b/7b: plan/tasks/implement will inherit, not re-register). Registered under the specify cluster because specify is the first workflow to need it (extraction, not speculative pre-definition).
3. **Agent ordering:** the two cluster-specific specify agents were placed before the generic shared `validator`, keeping cluster agents (transform-producer, principal-architect, requirements-analyst, devils-advocate) grouped ahead of the one shared grader — consistent with the existing reading order.
4. **`devils-advocate` is the specify critic, distinct from the generic `validator` agent.** It plays the validator team-role in the specify loop (grounded gap review + *recommended* status) but does not own the clearing verdict — the lead does (§A 1b). It is therefore registered as its own agent, not folded into the shared `validator`.
5. **No flow-graph box added for specify.** The setup cluster has no flow-graph box either; mirroring setup means the flow graph (transform-only) is left untouched. The frontmatter `description` was likewise left unchanged (setup's addition did not alter it).
6. **Independence preserved by registration:** producer (`requirements-analyst`) and critic (`devils-advocate`) are registered as separate agents with **disjoint** skills cells — no agent carries both produce and grade skills. The router's existing "Operating rules" (never mount producer + validator skills on one agent) are not violated by this registration.

## Scope guard

- Only `plugins/mochiko/skills/mochiko/SKILL.md` was edited. No other file touched (the `loop-discipline` Gap Classification edit and every per-primitive body port are separate steps).
- No kernel/DAG/catalog/MCP path or vocabulary introduced.
- This artifact + the realized tags above stand alone for `verify-output` (Part-A discoverability check) without needing to consult the producer.

---

**Trace version:** v1 · **Governed by:** `loop-discipline` · **Next:** `verify-output` (independent grade, different agent) + the remaining per-primitive body ports.
