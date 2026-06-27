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
| `[x]` | `validator` | agent | Generic independent grader — `verify-output` + `validation-constitution` (merges the former `transform-validator` + `constitution-validator`) |
| `[x]` | `transform-cluster` | command | Supervisor / lead-referee (owns the loop + human gate) |

> Lead/referee is the `transform-cluster` command supervisor, not a separate agent — so the cluster ships **2 agents**, not 3.

---

## Workflows (Commands)

| Status | Name | v3 Notes |
|--------|------|----------|
| `[x]` | `setup` | Ported 2026-06-27 — redesigned into a sound loop (default-FAIL done-condition, independent `validator`, escalate-on-cap, NEW constitution-acceptance human gate). Core-only scope. |
| `[x]` | `specify` | Ported 2026-06-27 — adversarial 2-member team (analyst↔advocate), kernel-free; default-FAIL loop + NEW human acceptance gate; `state-analyst` dissolved into the lead; strategy skills deduped into `loop-discipline`. Core-only scope. |
| `[ ]` | `plan` | TBD |
| `[ ]` | `tasks` | TBD |
| `[ ]` | `implement` | TBD — needs most thought on parallel TDD slice orchestration |
| `[ ]` | `audit` | TBD |
| `[-]` | `techspec` | Deprecated in HIL (merged into `plan`); evaluate when plan is ported |

---

## Agents

| Status | Name | Workflow Affinity |
|--------|------|-------------------|
| `[x]` | `principal-architect` | `setup` — producer-only (skills: authoring-constitution, analysis-codebase); grading split to the generic `validator` |
| `[x]` | `validator` | mochiko-native generic grader (see top table) — one independent grader serves `setup` (validation-constitution) + transform (verify-output); replaces the setup-specific `constitution-validator` born from the principal-architect split |
| `[x]` | `devils-advocate` | `specify` — adversarial critic/validator (skill: `analysis-specifications`; `validation-plan-artifacts` + `validation-task-artifacts` deferred → re-mount when plan/tasks port) |
| `[x]` | `requirements-analyst` | `specify` — producer (skills: `authoring-requirements`, `authoring-user-stories`) |
| `[ ]` | `technical-analyst` | `plan` |
| `[ ]` | `task-architect` | `tasks` |
| `[ ]` | `staff-engineer` | `implement` |
| `[ ]` | `qa-engineer` | `implement`, `audit` |
| `[-]` | `state-analyst` | **Dissolved (kernel-free)** — specify slice absorbed into the `specify` command lead; non-kernel judgment deduped into `loop-discipline`; no standalone agent. plan/tasks/implement orchestration handled when those clusters port. |
| `[ ]` | `ui-designer` | `specify`, `plan` — design-track; **deferred** from specify-core (catalog never invokes it) |

---

## Skills

### Meta / foundational
| Status | Name | Notes |
|--------|------|-------|
| `[-]` | `strategy-core` | **Dissolved** — near-total dedupe into `loop-discipline` (specify run); **Gap Classification** taxonomy folded into `loop-discipline` |
| `[-]` | `strategy-specification` | **Dissolved** — dedupe into `loop-discipline`; survivors (Input Assessment, targeted-revision, spec done-condition) → specify lead |
| `[ ]` | `strategy-implementation` | Impl-pass strategy — expect the same dissolution into `loop-discipline` when `implement` ports |
| `[ ]` | `syncing-claude-md` | Cross-cutting — referenced by `setup` as a documented stub (not yet ported); port with its own cluster |
| `[ ]` | `using-git-worktrees` | Cross-cutting utility |
| `[ ]` | `using-github-issues` | Cross-cutting utility |

### Setup workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `authoring-constitution` | Core of setup — **absorbed `brownfield-constitution`** as a greenfield\|brownfield branch; canonical `ESSENTIAL-FLOOR.md` home |
| `[x]` | `brownfield-constitution` | **Merged into `authoring-constitution`** (greenfield\|brownfield branch) — no standalone skill |
| `[x]` | `validation-constitution` | Body kept verbatim; **promoted** onto the independent generic `validator` agent |
| `[x]` | `analysis-codebase` | Setup-brownfield slice + `detect-stack.sh` (verbatim); collision/spec-plan modes deferred to spec/plan cluster; Essential-Floor deduped to authoring's canonical ref |
| `[ ]` | `brownfield-integration` | Deferred — out of setup-core scope (REGISTRY mis-file; reads as implement-time) |

### Specify workflow cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `analysis-specifications` | Ported — the advocate's gap-analysis procedure (stays a gap-finder; verdict owned by the lead) |
| `[x]` | `analysis-iterative` | Ported — enrichment + general brainstorm (dual-mode kept; marked general/shared in router) |
| `[-]` | `strategy-specification` | Dissolved (see Meta/foundational section) |

### Plan / technical design cluster
| Status | Name | Notes |
|--------|------|-------|
| `[x]` | `authoring-requirements` | Ported **via specify** (re-filed plan→specify) — analyst's FR-XXX / SC-XXX authoring |
| `[ ]` | `authoring-technical-requirements` | Tech requirements |
| `[x]` | `authoring-user-stories` | Ported **via specify** (re-filed plan→specify) — analyst's P1/P2/P3 + Given/When/Then |
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
| `[x]` | `constitution-template.md` | `setup` — ported (`approved-domain-deps.md` ref softened to prose) |
| `[-]` | `constitution-context-template.md` | `setup` — **absorbed into the setup lead** (in-session + `.mochiko/memory/` state); not a standalone artifact |
| `[x]` | `codebase-analysis-template.md` | `setup` — ported verbatim |
| `[ ]` | `codebase-inventory-schema.json` | **Deferred to spec/plan cluster** — orphan in setup (no consumer after mode-scoping) |
| `[x]` | `spec-template.md` | `specify` — ported (8 body slots verbatim; header rebound to workspace-as-state) |
| `[x]` | `analyst-report-template.md` | `specify` — ported (iteration→round; counts demoted; DAG trajectory dropped) |
| `[x]` | `advocate-report-template.md` | `specify` — ported (keep-verbatim; carries the 3-state verdict, the loop driver) |
| `[-]` | `context-template.md` | **Absorbed into the lead** (in-session + `.mochiko/specs/<feature>/` workspace-as-state); peers inherit — not a standalone artifact |
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
