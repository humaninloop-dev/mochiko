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
| `advocate-report-template` (template) | structuring the critic's grounded review (severity-bucketed gaps, clarifying questions, recommended status) the lead reads to own the verdict |

### Entry point (user-invoked — you run it)
| Command | Reach when |
|---------|------------|
| `/mochiko:transform-cluster <cluster>` | you want to transform a whole HIL primitive cluster into mochiko form |
| `/mochiko:setup` | you want to create, amend, or brownfield-derive the project constitution (greenfield \| brownfield \| amend) under an independent author→validator loop with a human acceptance gate |
| `/mochiko:specify` | you want to create a feature specification via the adversarial analyst↔advocate loop (the requirements-analyst authors `spec.md`, the devils-advocate stress-tests it) with a human acceptance gate |

### Agents (dispatched by the supervisor)
| Agent | Role |
|-------|------|
| `transform-producer` | assesses, reconciles, and applies recipes (skills: assess-primitive, reconcile-cluster, transform-recipes) |
| `principal-architect` | setup-cluster author — authors/updates the constitution (greenfield + brownfield) and runs codebase analysis (skills: authoring-constitution, analysis-codebase) |
| `requirements-analyst` | specify-cluster producer — authors the feature `spec.md` (prioritized user stories + FR/SC requirements) (skills: authoring-requirements, authoring-user-stories) |
| `devils-advocate` | specify-cluster adversarial critic — grounded, severity-bucketed gap review + a recommended verdict that feeds the lead's clearing decision (skills: analysis-specifications) |
| `validator` | one generic independent grader for any cluster — grades a finished artifact against a checklist, defaults to FAIL, authors nothing (skills: validation-constitution, verify-output) |

## Operating rules (context hygiene)

- **Keep assess → reconcile in one unbroken context** per cluster — reconcile reasons across all the assessments at once; a fresh context loses the sibling picture.
- **Always cross the producer↔validator boundary.** The author never grades its own output; dispatch the independent `validator` for `verify-output`. Never mount producer and validator skills on one agent.
- **The lead is the command, not an agent.** Verdict ownership, reconciliation arbitration, and the human gate live in `transform-cluster.md`.
- **One cluster per run.** Transform a whole workflow-cluster together (ROADMAP: "port the cluster together"), not stray primitives.

## Adding to the library

New primitives register here as part of the convention-wiring pass (a `verify-output` Part-A item). A primitive that is not in this router fails discoverability — it is, by construction, undiscoverable.
