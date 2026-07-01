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
| `[x]` | `assess-primitive` | skill | Branch-by-class + 7-check lens + responsibility trace; **+ command-altitude rule (generic discipline → `dedupe`, not `moved-to-lead`) (2026-06-30)** |
| `[x]` | `reconcile-cluster` | skill | Relational verdicts + rehome-orchestration |
| `[x]` | `transform-recipes` | skill | Per-disposition recipes + convention-wiring pass; **+ thin-command `redesign` target + wiring step 6 (single-source) (2026-06-30)** |
| `[x]` | `verify-output` | skill | Done-condition checker (independent gate); **+ altitude / single-source check (item 8) (2026-06-30)** |
| `[x]` | `mochiko` | skill | User-invoked router / library index |
| `[x]` | `transform-producer` | agent | Assess / reconcile / apply |
| `[x]` | `validator` | agent | Generic independent grader — `verify-output` + `validation-constitution` (merges the former `transform-validator` + `constitution-validator`) |
| `[x]` | `transform-cluster` | command | Supervisor / lead-referee (owns the loop + human gate) |

> Lead/referee is the `transform-cluster` command supervisor, not a separate agent — so the cluster ships **2 agents**, not 3.

---

## Workflows (Commands)

| Status | Name | v3 Notes |
|--------|------|----------|
| `[x]` | `setup` | Ported 2026-06-27 — redesigned into a sound loop (default-FAIL done-condition, independent `validator`, escalate-on-cap, NEW constitution-acceptance human gate). Core-only scope. **Re-transformed to thin/altitude shape 2026-06-30 (385→78 lines; independently verified PASS) — see `COMMAND-ALTITUDE-SYNTHESIS.md`.** |
| `[x]` | `specify` | Ported 2026-06-27 — adversarial 2-member team (analyst↔advocate), kernel-free; default-FAIL loop + NEW human acceptance gate; `state-analyst` dissolved into the lead; strategy skills deduped into `loop-discipline`. Core-only scope. **Re-transformed to thin/altitude shape 2026-06-30 (329→66 lines; independently verified PASS) — see `COMMAND-ALTITUDE-SYNTHESIS.md`.** |
| `[x]` | `plan` | Ported 2026-07-01 — thin/altitude analysis→design loop (`plan.md`, 82 lines; independently verified PASS). **First cluster to run convention-5's two-form** (two distinct validators: `principal-architect`+`validation-feasibility` feasibility × `devils-advocate`+`validation-plan-artifacts` completeness), and the **first net-new command since the altitude fix** — thin by construction. Producer `technical-analyst`; lead owns verdict + 4 gates HIL lacked. Full run record: `.mochiko/transform/plan/`. |
| `[x]` | `tasks` | Ported 2026-07-01 — thin/altitude Mapping→Tasks loop (`tasks.md`, 77 lines; independently verified PASS, single round). **RQ-A resolved = keep two artifacts** (`task-mapping.md` source of truth, `tasks.md` table derived echo). Single-reviewer **specify shape** (`task-architect` produces ↔ `devils-advocate`+`validation-task-artifacts` grades); lead owns verdict + 4 gates HIL lacked (default-FAIL, lead-owned verdict, cap+`TASKS_STOP`, G5 acceptance). **Decoupling doctrine held on the run's hardest surface** (task-architect: 5 sibling-names + phase-mode + context-file all cut, zero residual). Full run record: `.mochiko/transform/tasks/`. |
| `[ ]` | `implement` | TBD — needs most thought on parallel TDD slice orchestration |
| `[ ]` | `audit` | TBD |
| `[-]` | `techspec` | Deprecated in HIL (merged into `plan`); **confirmed excluded (plan port, 2026-07-01)** — `plan` IS the merged form; techspec's `integrations.md`/`data-sensitivity.md` fold into `data-model.md` (sensitivity) + `contracts/api.yaml` (x-integration) |

---

## Agents

| Status | Name | Workflow Affinity |
|--------|------|-------------------|
| `[x]` | `principal-architect` | `setup` producer (authoring-constitution, analysis-codebase) **+ `plan` feasibility reviewer** (validation-feasibility, re-broadened 2026-07-01) — produce-in-setup / review-in-plan (decoupling-legit); **G1: never re-mounts `validation-constitution`** (no constitution self-grade) |
| `[x]` | `validator` | mochiko-native generic grader (see top table) — one independent grader serves `setup` (validation-constitution) + transform (verify-output); replaces the setup-specific `constitution-validator` born from the principal-architect split |
| `[x]` | `devils-advocate` | **cross-workflow reviewer** — `specify` critic (`analysis-specifications`) + `plan` completeness reviewer (`validation-plan-artifacts`) + `tasks` task-artifact reviewer (`validation-task-artifacts` **re-mounted live 2026-07-01** — the last stubbed mount now closed) |
| `[x]` | `requirements-analyst` | `specify` — producer (skills: `authoring-requirements`, `authoring-user-stories`) |
| `[x]` | `technical-analyst` | `plan` — producer (skills: authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts); authors the 6 analysis+design artifacts, never grades |
| `[x]` | `task-architect` | `tasks` producer — decoupled (skills: `patterns-vertical-tdd`); authors `task-mapping.md` + `tasks.md`, never grades. Ported 2026-07-01; report → `taskarchitect-report-template` (no self-verdict). The run's heaviest port + hardest decoupling surface (zero residual deny-list tokens) |
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
| `[x]` | `authoring-technical-requirements` | Ported (plan, 2026-07-01) — owns TR/C/NFR/IP + `constraints-and-decisions.md` + C↔D/IP traceability; DS→`patterns-entity-modeling`, x-integration→`patterns-api-contracts`, technique→`patterns-technical-decisions` (thin declarations + refs) |
| `[x]` | `authoring-user-stories` | Ported **via specify** (re-filed plan→specify) — analyst's P1/P2/P3 + Given/When/Then |
| `[x]` | `patterns-api-contracts` | Ported (plan) — API contracts (OpenAPI) + **x-integration boundary authoring** (added — canonical home; repaired the dangling authoring-technical-requirements ref) |
| `[x]` | `patterns-entity-modeling` | Ported (plan) — data-model + **canonical data-sensitivity 4-level taxonomy** (absorbed from authoring-technical-requirements; single `data-model.md` template) |
| `[x]` | `patterns-technical-decisions` | Ported (plan) — decision technique / ADR; references authoring-technical-requirements for the `constraints-and-decisions.md` artifact (boundary + dedupe) |
| `[x]` | `validation-feasibility` | **NEW — net-new, not a HIL port (plan, 2026-07-01).** Feasibility reviewer's adversarial-critique skill on `principal-architect`; cross-artifact contradiction / impossibility / buildability → 3-state incl. `infeasible`; the convention-5 **two-form** partner of `validation-plan-artifacts` (homes the feasibility review the setup PA port disclaimed) |
| `[ ]` | `authoring-roadmap` | Roadmap authoring — **deferred** (setup-brownfield track, not plan-core) |
| `[ ]` | `patterns-flow-mapping` | Flow design — **deferred** (design track / ui-designer, not plan-core) |
| `[ ]` | `patterns-interface-design` | UI patterns — **deferred** (design track / ui-designer, not plan-core) |
| `[x]` | `patterns-vertical-tdd` | Ported (tasks, 2026-07-01) — the producer's vertical-slice + TDD structuring skill; **canonical home of the `**TEST:**` verification-task grammar** (folded from the task-architect persona); conforms to `tasks-template` as the canonical `tasks.md` structure; design-time boundary vs deferred `executing-tdd-cycle` (implement) held |

### Implement / execute cluster
| Status | Name | Notes |
|--------|------|-------|
| `[ ]` | `executing-tdd-cycle` | TDD cycle execution |
| `[x]` | `validation-plan-artifacts` | Ported **via plan** (2026-07-01; re-filed implement→plan-cluster) — completeness mirror-checklist on `devils-advocate`; **absorbed `cross-artifact-checklist.md`**; feasibility checks deduped → `validation-feasibility` |
| `[x]` | `validation-task-artifacts` | Ported (tasks, 2026-07-01) — the reviewer's task-artifact gap-finder on `devils-advocate`; gap report + recommended 3-state (**verdict lead-owned**, not clearing PASS/FAIL); embedded report templates deduped → `advocate-report-template`; disjoint boundary vs `validation-plan-artifacts` (task vs plan artifacts); two-phase Mapping/Tasks/Cross-Artifact checklists kept (Branch A); mirrors `patterns-vertical-tdd` |
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
| `[x]` | `advocate-report-template.md` | **shared (specify + plan)** — ported via specify (keep-verbatim; 3-state verdict); **reused as-is by plan's completeness review** (RQ4, 2026-07-01 — no new file) |
| `[-]` | `context-template.md` | **Absorbed into the lead** (in-session + `.mochiko/specs/<feature>/` workspace-as-state); peers inherit — not a standalone artifact |
| `[x]` | `plan-template.md` | Ported (plan, 2026-07-01) — the `plan.md` deliverable; the lead's fill-target; `[...]` model-fill style (not `{{}}`) |
| `[-]` | `plan-context-template.md` | **Absorbed into the plan lead** (memory-model — 3rd confirmation; in-session + `.mochiko/specs/<feature>/` workspace-as-state; state recovery reads workspace evidence) — not a standalone artifact |
| `[x]` | `tasks-template.md` | Ported (tasks, 2026-07-01) — the `tasks.md` deliverable skeleton; **the canonical `tasks.md` structure** (`patterns-vertical-tdd` conforms to it); `[...]` model-fill, frontmatter dropped; each cycle ends in a `**TEST:**` real-infra verification task (P3↔P5 alignment fixed this run); Story→Cycle table = derived echo of `task-mapping.md` |
| `[-]` | `tasks-context-template.md` | **Absorbed into the tasks lead** (memory-model, 4th confirmation — in-session + `.mochiko/specs/<feature>/` workspace-as-state; state recovery reads workspace evidence; the cross-workflow plan-complete entry gate rebound to `plan.md` presence) — not a standalone artifact |
| `[x]` | `taskarchitect-report-template.md` | **NEW (tasks, 2026-07-01)** — the `task-architect` producer's per-round non-verdict self-disclosure report (mirrors `techanalyst-report-template` + tasks-specific Vertical-Slice-Rationale / TDD-Structure sections); **no `Completion`/`Ready-for-Review` field** (lead owns the verdict) |
| `[ ]` | `evolution-roadmap-template.md` | `plan`-filed but **deferred** (roadmap track, not plan-core) |
| `[x]` | `architect-report-template.md` | Ported (plan) — **renamed `feasibility-report-template.md`**; feasibility reviewer's report; 3-state incl. `infeasible`; Next-Steps routing lifted to lead; `type:` DAG frontmatter dropped |
| `[x]` | `techanalyst-report-template.md` | Ported (plan) — producer self-disclosure; **`completion_status` dropped** (independence — verdict is lead-owned); iteration→round; brain-era counts demoted |
| `[-]` | `cross-artifact-checklist.md` | **Folded into `validation-plan-artifacts`** (plan; orphan — no consumer + near-total dup; the tasks validator self-contains its own) |
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
