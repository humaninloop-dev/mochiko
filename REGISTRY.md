# Mochiko Migration Registry

Track every primitive from `human-in-loop` — check off as it lands in mochiko. Items grouped by type, then by workflow affinity. A primitive may appear in multiple workflows.

**Status key**: `[ ]` not yet ported · `[x]` ported · `[~]` ported but needs upgrade · `[-]` deliberately excluded

---

## Mochiko-native primitives (net-new, not HIL ports)

The transformer cluster — the dogfooded implementation of [`PLAYBOOK.md`](PLAYBOOK.md). It is the tool that ports everything below it. Built before any HIL primitive is ported; proven by running it on the `setup` cluster.

| Status | Name | Type | Notes |
|--------|------|------|-------|
| `[x]` | `loop-discipline` | skill | Doctrine: the four sound-loop rules |
| `[x]` | `workflow-contract` | template | Fill-in loop contract (default FAIL) |
| `[x]` | `assess-primitive` | skill | Branch-by-class + 7-check lens + responsibility trace |
| `[x]` | `reconcile-cluster` | skill | Relational verdicts + rehome-orchestration |
| `[x]` | `transform-recipes` | skill | Per-disposition recipes + convention-wiring pass |
| `[x]` | `verify-output` | skill | Done-condition checker (independent gate) |
| `[x]` | `mochiko` | skill | User-invoked router / library index |
| `[x]` | `transform-producer` | agent | Assess / reconcile / apply |
| `[x]` | `transform-validator` | agent | Independent grading (verify-output) |
| `[x]` | `transform-cluster` | command | Supervisor / lead-referee (owns the loop + human gate) |

> Lead/referee is the `transform-cluster` command supervisor, not a separate agent — so the cluster ships **2 agents**, not 3.

---

## Workflows (Commands)

| Status | Name | v3 Notes |
|--------|------|----------|
| `[ ]` | `setup` | **Start here** — port + upgrade; shed brain-mediated DAG invocation |
| `[ ]` | `specify` | **Second** — rebuild as adversarial 2-member agent team |
| `[ ]` | `plan` | TBD |
| `[ ]` | `tasks` | TBD |
| `[ ]` | `implement` | TBD — needs most thought on parallel TDD slice orchestration |
| `[ ]` | `audit` | TBD |
| `[-]` | `techspec` | Deprecated in HIL (merged into `plan`); evaluate when plan is ported |

---

## Agents

| Status | Name | Workflow Affinity |
|--------|------|-------------------|
| `[ ]` | `principal-architect` | `setup` |
| `[ ]` | `devils-advocate` | `specify` (adversarial critic) |
| `[ ]` | `requirements-analyst` | `specify` (producer) |
| `[ ]` | `technical-analyst` | `plan` |
| `[ ]` | `task-architect` | `tasks` |
| `[ ]` | `staff-engineer` | `implement` |
| `[ ]` | `qa-engineer` | `implement`, `audit` |
| `[ ]` | `state-analyst` | `specify`, `plan`, `tasks`, `implement` — orchestration coordinator |
| `[ ]` | `ui-designer` | `specify`, `plan` — design-track |

---

## Skills

### Meta / foundational
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `strategy-core` | Universal workflow patterns; consumed by state-analyst |
| `[ ]` | `strategy-specification` | Spec-pass strategy |
| `[ ]` | `strategy-implementation` | Impl-pass strategy |
| `[ ]` | `syncing-claude-md` | Cross-cutting — keep as-is |
| `[ ]` | `using-git-worktrees` | Cross-cutting utility |
| `[ ]` | `using-github-issues` | Cross-cutting utility |

### Setup workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `authoring-constitution` | Core of setup |
| `[ ]` | `brownfield-constitution` | Brownfield variant |
| `[ ]` | `validation-constitution` | Gates setup completion |
| `[ ]` | `analysis-codebase` | Feeds setup — brownfield analysis |
| `[ ]` | `brownfield-integration` | Brownfield onboarding |

### Specify workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `analysis-specifications` | Feeds specify — gap analysis |
| `[ ]` | `analysis-iterative` | Brainstorm / enrichment input |
| `[ ]` | `strategy-specification` | (also listed above) |

### Plan / technical design cluster
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `authoring-requirements` | Requirements authoring |
| `[ ]` | `authoring-technical-requirements` | Tech requirements |
| `[ ]` | `authoring-user-stories` | User stories |
| `[ ]` | `authoring-roadmap` | Roadmap authoring |
| `[ ]` | `patterns-api-contracts` | API design patterns |
| `[ ]` | `patterns-entity-modeling` | Data model patterns |
| `[ ]` | `patterns-flow-mapping` | Flow design |
| `[ ]` | `patterns-interface-design` | UI patterns |
| `[ ]` | `patterns-technical-decisions` | ADR / decision records |
| `[ ]` | `patterns-vertical-tdd` | Vertical slice patterns |

### Implement / execute cluster
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `executing-tdd-cycle` | TDD cycle execution |
| `[ ]` | `validation-plan-artifacts` | Gates plan completion |
| `[ ]` | `validation-task-artifacts` | Gates task completion |
| `[ ]` | `testing-end-user` | End-user testing |

### Analysis / design cluster
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `analysis-screenshot` | Screenshot / design extraction |
| `[ ]` | `authoring-design-system` | Design system authoring |

---

## Templates

| Status | Name | Workflow Affinity |
|--------|------|-------------------|
| `[ ]` | `constitution-template.md` | `setup` |
| `[ ]` | `constitution-context-template.md` | `setup` |
| `[ ]` | `codebase-analysis-template.md` | `setup` |
| `[ ]` | `codebase-inventory-schema.json` | `setup` |
| `[ ]` | `spec-template.md` | `specify` |
| `[ ]` | `analyst-report-template.md` | `specify` |
| `[ ]` | `advocate-report-template.md` | `specify` |
| `[ ]` | `context-template.md` | `specify`, `plan`, `tasks`, `implement` |
| `[ ]` | `plan-template.md` | `plan` |
| `[ ]` | `plan-context-template.md` | `plan` |
| `[ ]` | `tasks-template.md` | `tasks` |
| `[ ]` | `tasks-context-template.md` | `tasks` |
| `[ ]` | `evolution-roadmap-template.md` | `plan` |
| `[ ]` | `architect-report-template.md` | `plan` |
| `[ ]` | `techanalyst-report-template.md` | `plan` |
| `[ ]` | `cross-artifact-checklist.md` | `plan`, `tasks` |
| `[ ]` | `approved-domain-deps.md` | `implement` |

---

## Catalogs

| Status | Name | Notes |
|--------|------|-------|
| `[-]` | `specify-catalog.json` | Brain-mediated DAG — excluded (kernel-free) |
| `[-]` | `implement-catalog.json` | Brain-mediated DAG — excluded (kernel-free) |

---

## Scripts

| Status | Name | Notes |
|--------|------|-------|
| `[-]` | `check-prerequisites.sh` | Brain/MCP prerequisites — excluded |
| `[-]` | `common.sh` | Brain/MCP common — excluded |
| `[-]` | `create-new-feature.sh` | Brain-mediated — excluded |
| `[-]` | `setup-plan.sh` | Brain-mediated — excluded |

---

_Update this file as primitives are ported. Change `[ ]` to `[x]` when landed, `[~]` if ported but needs a v3 upgrade pass, `[-]` if deliberately excluded._
