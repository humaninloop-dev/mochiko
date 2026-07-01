---
name: mochiko
description: User-invoked router for the mochiko skill library. Indexes the available skills, agents, and workflows and says when to reach each. Start here when you want to transform a human-in-loop primitive into mochiko form, or to find the right mochiko skill for a task.
disable-model-invocation: true
---

# Mochiko Router

> **User-invoked entry point.** This skill indexes the mochiko library and points you to the right primitive. It can *invoke* model-invoked skills and *hint* at other user-invoked entries (it cannot fire them — invoke those yourself).
>
> `disable-model-invocation: true` is the Claude-Code mechanism for user-invoked classification. If your harness names this differently, bind to its equivalent (tracked as a portability item in `BACKLOG.md`).

## What mochiko is

The kernel-free successor to human-in-loop. Discipline lives in the skill library; native agent teams + a markdown command supervisor handle orchestration. Every workflow is a **sound loop** (see `loop-discipline`). See `ROADMAP.md` for the thesis and `PLAYBOOK.md` for the transformation doctrine.

## Flow graph

```
  ┌─ doctrine (consumed by every workflow) ─────────────────────────────┐
  │   loop-discipline ........ the four sound-loop rules                  │
  │   workflow-contract ...... the fill-in loop contract (template)      │
  └──────────────────────────────────────────────────────────────────────┘

  TRANSFORM A PRIMITIVE  (the first dogfood cluster)
  run:  /mochiko:transform-cluster <cluster>          ← user-invoked entry (hint)
        │
        ▼
   triage ─→ assess-primitive ─→ reconcile-cluster ─→ transform-recipes ─→ verify-output
             (per primitive)      (cluster scope)      (apply disposition)  (independent gate)
        agents:  transform-producer ───────────────────────────────┘  validator
        lead/referee = the transform-cluster command supervisor
```

## When to reach each

### Doctrine (model-invoked — auto-reached when designing any loop)
| Skill | Reach when |
|-------|------------|
| `loop-discipline` | designing/reviewing any workflow or agent loop; deciding if a loop is sound; filling a `workflow-contract` |
| `workflow-contract` (template) | instantiating the contract for a specific workflow |
| `agent-dispatch` (template) | briefing each agent dispatch inside a loop — a caller-side guide, not a gate |

### Transformer cluster (model-invoked — auto-reached during a transform run)
| Skill | Reach when |
|-------|------------|
| `assess-primitive` | diagnosing one primitive: class, disposition, responsibility trace |
| `reconcile-cluster` | resolving cross-primitive moves (pair/split/merge/promote) + rehoming orchestration, after all primitives are assessed |
| `transform-recipes` | applying a finalized disposition; running the convention-wiring pass every port pays |
| `verify-output` | independently grading a transformed artifact (run by an independent validator, never the author) |

### Setup cluster (model-invoked — auto-reached during a `/mochiko:setup` run)
| Skill | Reach when |
|-------|------------|
| `authoring-constitution` | authoring or amending a constitution at `.mochiko/memory/constitution.md` — greenfield (opinionated defaults: Essential Floor + recommended architectural principles) OR brownfield (codify existing patterns: Essential Floor + Emergent Ceiling, from `codebase-analysis.md`); one skill, both modes |
| `analysis-codebase` | analyzing an existing codebase during a brownfield setup run — deterministic stack detection (`detect-stack.sh`) + architecture/convention extraction + Essential-Floor status assessment, producing `.mochiko/memory/codebase-analysis.md` |
| `validation-constitution` | independently grading a drafted constitution: three-part-structure check, anti-pattern + placeholder scan, quantification, semantic version-bump → binary PASS/FAIL + fix list (run by an independent validator, never the author) |

### Specify cluster (model-invoked — auto-reached during a `/mochiko:specify` run)
| Skill | Reach when |
|-------|------------|
| `analysis-iterative` (general/shared) | brainstorming or enriching sparse feature input before a spec is authored — adaptive questioning to surface Who/Problem/Value; a cross-cluster conditioner the lead invokes as a pre-step when feature input is sparse |
| `analysis-specifications` | finding gaps in a drafted `spec.md` — severity-bucketed gap-finding + clarifying questions that feed the lead's verdict (the critic's skill; stays a gap-finder, owns no clearing verdict) |
| `authoring-requirements` | writing technology-agnostic functional requirements (FR-XXX) with measurable success criteria (SC-XXX) and edge cases |
| `authoring-user-stories` | writing prioritized user stories (P1/P2/P3) with independently testable Given/When/Then acceptance scenarios |
| `spec-template` (template) | the `spec.md` the analyst authors and the loop converges on — lead-seeded; header `status` carries the loop's done-condition |
| `analyst-report-template` (template) | structuring the producer's per-round disclosure (summary, assumptions, what-changed-this-round) the lead reads directly |
| `advocate-report-template` (template) | **shared (specify + plan)** — structures the completeness reviewer's grounded review (severity-bucketed gaps, clarifying questions, recommended status) the lead reads to own the verdict; the plan completeness reviewer reuses it as-is |

### Plan cluster (model-invoked — auto-reached during a `/mochiko:plan` run)
| Skill | Reach when |
|-------|------------|
| `authoring-technical-requirements` | authoring the technical-requirements layer — TR-XXX decomposition, C-XXX hard constraints, NFR-XXX (numeric target + measurement method), IP-XXX provisioning, and the `constraints-and-decisions.md` artifact + C↔D / IP traceability; declares DS-XXX / INT-XXX as **analysis concerns only** (the per-attribute sensitivity taxonomy lives in `patterns-entity-modeling`, the per-endpoint `x-integration` boundary in `patterns-api-contracts`) |
| `patterns-technical-decisions` | making/documenting a technology or architecture decision — evaluating ≥2 alternatives against weighted criteria, trade-offs + consequences, brownfield-alignment scoring, ADR record depth, marking NEEDS CLARIFICATION; owns the decision *technique* (the `constraints-and-decisions.md` artifact it fills is owned by `authoring-technical-requirements`) |
| `patterns-entity-modeling` | modeling a feature's domain data — entities, attributes/conceptual types, relationships (cardinality + delete behavior), state machines, and per-attribute data-sensitivity classification (the canonical 4-level Public/Internal/Confidential/Restricted taxonomy); authors the canonical `data-model.md` — the single home for data-sensitivity |
| `patterns-api-contracts` | designing the API-contract layer — user action → REST endpoint (method/idempotency/naming), request/response schemas (mapping data-model types to OpenAPI), error design, list pagination, and the per-endpoint `x-integration` boundary for endpoints wrapping external systems; assembles `contracts/api.yaml` — owns the API contract + x-integration format |
| `validation-plan-artifacts` | independently grading the producer's plan artifacts for **completeness** — coverage / measurability / presence / cross-artifact consistency over the analysis + design sets → severity-classified gaps + 3-state `ready / needs-revision / critical-gaps` (the mirror-checklist half of the plan-review pair; an independent validator, never the author) |
| `validation-feasibility` | adversarially grading the producer's plan artifacts for cross-artifact **feasibility** — contradiction / impossibility / buildability that no single artifact reveals → 3-state `feasible / needs-revision / infeasible` (the adversarial-critique half of the plan-review pair; the distinct `infeasible` = a business-level escalation; an independent reviewer, never the author) |
| `plan-template` (template) | the `plan.md` deliverable the lead assembles at Phase 4 — rolling up Key Decisions · Infrastructure/IP-XXX (constraints-and-decisions) · Entities+Sensitivity (data-model) · Endpoints+Integration (contracts/api.yaml); the lead's fill-target |
| `techanalyst-report-template` (template) | the technical-analyst producer's per-round self-disclosure (what was produced, what changed this round) — filled alongside the analysis/design artifacts, read directly by the lead + reviewers; carries no verdict |
| `feasibility-report-template` (template) | the feasibility reviewer's cross-artifact critique — the contradiction taxonomies, the 3-state feasibility verdict, and the 4-field per-issue gate fuel the human gate reads |

> The plan **completeness** reviewer reuses the shared `advocate-report-template` (registered under Specify, above) as-is — there is no plan-specific completeness report template.

### Tasks cluster (model-invoked — auto-reached during a `/mochiko:tasks` run)
| Skill | Reach when |
|-------|------------|
| `patterns-vertical-tdd` | structuring an accepted plan into implementation tasks — vertical-slice identification (foundation vs feature cycles), test-first cycle structure (red/green/refactor), the `**TEST:**` verification-task grammar, and story→cycle→task traceability; the `task-architect` producer's authoring skill (teaches the craft — the `tasks.md` skeleton it fills is `tasks-template`) |
| `validation-task-artifacts` | independently grading the producer's task artifacts (`task-mapping.md` / `tasks.md`) for **completeness** — vertical-slice integrity, TDD test-first ordering, `**TEST:**` presence, cycle sizing, and story→cycle→task traceability → severity-classified gaps + 3-state `ready / needs-revision / critical-gaps` (the reviewer's skill; an independent validator, never the author). **Boundary:** this grades **task artifacts**; `validation-plan-artifacts` (Plan, above) grades **plan artifacts** — disjoint artifacts, disjoint checks |
| `tasks-template` (template) | the `tasks.md` deliverable the `task-architect` fills — the cycle→TDD-task skeleton (foundation sequential + feature `[P]`, per-task file path, `[US#]`, `[EXTEND]`/`[MODIFY]` markers, `**TEST:**` block, and the Story→Cycle table as a derived echo of `task-mapping.md`) |
| `taskarchitect-report-template` (template) | the `task-architect` producer's per-round self-disclosure (what was produced, vertical-slice rationale, TDD structure) — filled alongside `task-mapping.md`/`tasks.md`, read directly by the lead; carries no verdict |

### Entry point (user-invoked — you run it)
| Command | Reach when |
|---------|------------|
| `/mochiko:transform-cluster <cluster>` | you want to transform a whole HIL primitive cluster into mochiko form |
| `/mochiko:setup` | you want to create, amend, or brownfield-derive the project constitution (greenfield \| brownfield \| amend) under an independent author→validator loop with a human acceptance gate |
| `/mochiko:specify` | you want to create a feature specification via the adversarial analyst↔advocate loop (the requirements-analyst authors `spec.md`, the devils-advocate stress-tests it) with a human acceptance gate |
| `/mochiko:plan` | you want to create a feature's analysis→design implementation plan via the producer→two-reviewer loop (technical-analyst authors; principal-architect grades feasibility, devils-advocate grades completeness) with a human acceptance gate on `plan.md` |
| `/mochiko:tasks` | you want to structure an accepted plan into implementation tasks — `task-mapping.md` (story→cycle mapping) + `tasks.md` (cycle→TDD tasks) — via the task-architect→devils-advocate loop with a human acceptance gate on `tasks.md`; next step `/mochiko:implement` |

### Agents (dispatched by the supervisor)
| Agent | Role |
|-------|------|
| `transform-producer` | assesses, reconciles, and applies recipes (skills: assess-primitive, reconcile-cluster, transform-recipes) |
| `principal-architect` | **cross-workflow** — setup-cluster author (authors/updates the constitution, greenfield + brownfield; runs codebase analysis) **and** plan-cluster **feasibility reviewer** (grades the analyst's plan artifacts for cross-artifact buildability; grades a different agent's work, never its own authoring) (skills: authoring-constitution, analysis-codebase, validation-feasibility) |
| `requirements-analyst` | specify-cluster producer — authors the feature `spec.md` (prioritized user stories + FR/SC requirements) (skills: authoring-requirements, authoring-user-stories) |
| `devils-advocate` | **cross-workflow** adversarial reviewer — specify-cluster critic (grounded, severity-bucketed spec-gap review), plan-cluster **completeness reviewer** (coverage / measurability / consistency / presence over the plan artifacts), **and** tasks-cluster **task-artifact reviewer** (vertical-slice integrity, TDD structure, story→cycle→task traceability over `task-mapping.md`/`tasks.md`); recommends a verdict that feeds the lead's clearing decision, never the gate (skills: analysis-specifications, validation-plan-artifacts, validation-task-artifacts) |
| `technical-analyst` | plan-cluster PRODUCER — authors the six analysis+design artifacts (requirements · constraints-and-decisions · NFRs · data-model · API contracts · quickstart); never grades its own output (skills: authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts) |
| `task-architect` | tasks-cluster PRODUCER — structures an accepted plan into implementation tasks (`task-mapping.md` story→cycle mapping + slice rationale, then `tasks.md` cycle-based TDD task list); never grades its own output (skills: patterns-vertical-tdd) |
| `validator` | one generic independent grader for any cluster — grades a finished artifact against a checklist, defaults to FAIL, authors nothing (skills: validation-constitution, verify-output) |

## Operating rules (context hygiene)

- **Keep assess → reconcile in one unbroken context** per cluster — reconcile reasons across all the assessments at once; a fresh context loses the sibling picture.
- **Always cross the producer↔validator boundary.** The author never grades its own output; dispatch the independent `validator` for `verify-output`. Never mount producer and validator skills on one agent.
- **The lead is the command, not an agent.** Verdict ownership, reconciliation arbitration, and the human gate live in `transform-cluster.md`.
- **One cluster per run.** Transform a whole workflow-cluster together (ROADMAP: "port the cluster together"), not stray primitives.

## Adding to the library

New primitives register here as part of the convention-wiring pass (a `verify-output` Part-A item). A primitive that is not in this router fails discoverability — it is, by construction, undiscoverable.
