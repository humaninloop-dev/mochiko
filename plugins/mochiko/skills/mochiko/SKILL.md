---
name: mochiko
description: User-invoked router for the mochiko skill library. Indexes the available skills, agents, and workflows and says when to reach each. Start here to find the right mochiko skill, agent, or workflow for a task.
disable-model-invocation: true
---

# Mochiko Router

> **User-invoked entry point.** This skill indexes the mochiko library and points you to the right primitive. It can *invoke* model-invoked skills and *hint* at other user-invoked entries (it cannot fire them — invoke those yourself).
>
> `disable-model-invocation: true` is the Claude-Code mechanism for user-invoked classification. If your harness names this differently, bind to its equivalent (tracked as a portability item in `BACKLOG.md`).

## What mochiko is

The kernel-free successor to human-in-loop. Discipline lives in the skill library; native agent teams + a markdown command supervisor handle orchestration. Every workflow is a **sound loop** (see `loop-discipline`). See `ROADMAP.md` for the thesis.

## How the library composes

Doctrine consumed by every workflow: `loop-discipline` (the four sound-loop rules) and the
`workflow-contract` template. Each workflow below is a sound loop — a command supervisor
stitches a producer/validator agent team to a goal under a workflow-contract.

**The two review-skill families** (the `validation-*`/`review-*` split, 2026-07-18 — design:
`.mochiko/brainstorms/setup-adversarial-review/record.md` D5): the prefix encodes **who owns the
clearing**. `validation-*` = the skill **issues the authoritative grade** — a binary PASS/FAIL
checklist grade, default FAIL, on the `validator` persona (today: `validation-constitution`
and `validation-command-shape`; a PASS is still human-gated downstream). `review-*` = the skill **produces
lead-adjudicated input** — severity-ranked findings and a *recommended* status that the lead or a
human adjudicates; the reviewer's verdict never clears anything by itself.

## When to reach each

### Doctrine (model-invoked — auto-reached when designing any loop)
| Skill | Reach when |
|-------|------------|
| `loop-discipline` | designing/reviewing any workflow or agent loop; deciding if a loop is sound; filling a `workflow-contract` |
| `workflow-contract` (template) | instantiating the contract for a specific workflow |
| `agent-dispatch` (template) | briefing each agent dispatch inside a loop — a caller-side guide, not a gate |
| `command-shape` (template) | the codified command pattern's **sole authoritative home** (Layer 1 form-agnostic core · Layer 2 team transport) — obligated-read by conformant commands; referenced, never restated |

### Setup cluster (model-invoked — auto-reached during a `/mochiko:setup` run)
| Skill | Reach when |
|-------|------------|
| `authoring-constitution` | authoring or amending the project's **governance surface set** (no constitution.md — CLAUDE.md governance region + `paths`-scoped rules files + skill pointers + governance ledger + trace manifest) from the ratified session synthesis (`governance-intent.md`); greenfield and brownfield in one skill |
| `analysis-codebase` | analyzing an existing codebase during a brownfield setup run — deterministic stack detection (`detect-stack.sh`) + architecture/convention extraction + tier-blind Essential-Floor status assessment, producing `.mochiko/memory/codebase-analysis.md` |
| `validation-constitution` | independently grading a drafted governance surface set against the synthesis + trace manifest — trace closure, surface integrity, three-part structure, tier/waiver accounting, anti-pattern + placeholder scans, version bump → binary PASS/FAIL + fix list (never the author) |
| `review-governance-intent` | serving as a cold **G3 intent reviewer** in a setup run — stress-testing the frozen, confidence-marked synthesis *before* the user ratifies it (coverage/coherence pair or solo per the sizing ruling) → survivors + tally + recommended status (never a session participant; clearing is the lead's + G3's) |
| `testing-governance-injection` | empirically probing that an **accepted** surface set delivers — disposable probe subagents (plus a negative control) verify rules inject on the promised paths and change behavior; findings versioned, fed to an amend run, never auto-fixed. Delivery only — static grading stays `validation-constitution` |
| `grooming-operating-docs` | a knowledge-management invariant cap/bound trips at a command boundary (horizon caps, item bound/count, dead pointer, status disagreement, `[x]` in BACKLOG) — fix-on-sight groom resolving everything from the project-pinned copy at `.mochiko/memory/knowledge-management.md` |
| `authoring-architecture` | authoring/updating `ARCHITECTURE.md` (the KM module's living system view) at a plan/implement landing on structural change — current state only; rationale links to the decisions layer, never restated |

### Specify cluster (model-invoked — auto-reached during a `/mochiko:specify` run)
| Skill | Reach when |
|-------|------------|
| `analysis-iterative` (general/shared) | brainstorming or enriching sparse feature input before a spec is authored — adaptive questioning to surface Who/Problem/Value; a cross-cluster conditioner the lead invokes as a pre-step when feature input is sparse |
| `review-specifications` | finding gaps in a drafted `spec.md` — severity-bucketed gap-finding + clarifying questions that feed the lead's verdict (the critic's skill; stays a gap-finder, owns no clearing verdict) |
| `authoring-requirements` | writing technology-agnostic functional requirements (FR-XXX) with measurable success criteria (SC-XXX) and edge cases |
| `authoring-user-stories` | writing prioritized user stories (P1/P2/P3) with independently testable Given/When/Then acceptance scenarios |
| `spec-template` (template) | the `spec.md` the analyst authors and the loop converges on — lead-seeded; header `status` carries the loop's done-condition |
| `analyst-report-template` (template) | structuring the producer's per-round disclosure (assumptions, what-changed-this-round) the lead reads directly |
| `advocate-report-template` (template) | **shared (specify + plan + slice)** — structures every pipeline reviewer's grounded review, machine-first (severity-classified `findings:` YAML, clarifying questions, recommended verdict, one-line `strengths:`) the lead reads to own the verdict |
| `report-format` (template) | **shared (all workflows)** — the report envelope every workflow report follows: machine-first frontmatter, conditional prose (failures keep narrative; clean reports are frontmatter-only), no-self-verdict, no-restatement; each report template carries only its payload schema over this envelope |
| `artifact-format` (template) | **shared (all workflows)** — the deliverable envelope the pipeline artifacts follow (spec, requirements, constraints-and-decisions, nfrs, data-model, contracts, quickstart, plan, task-mapping, tasks, slices, codebase-analysis): dense-by-construction + human-legible — reference-by-ID, per-artifact ID index, statement-carries-the-content, size guidance, omit-empty, density-is-not-a-gap review rule; artifact templates and authoring skills carry only their own section schema over this envelope |

### Plan cluster (model-invoked — auto-reached during a `/mochiko:plan` run)
| Skill | Reach when |
|-------|------------|
| `authoring-technical-requirements` | authoring the technical-requirements layer — TR-XXX decomposition, C-XXX hard constraints, NFR-XXX (numeric target + measurement method), IP-XXX provisioning, and the `constraints-and-decisions.md` artifact + C↔D / IP traceability; declares DS-XXX / INT-XXX as **analysis concerns only** (the per-attribute sensitivity taxonomy lives in `patterns-entity-modeling`, the per-endpoint `x-integration` boundary in `patterns-api-contracts`) |
| `patterns-technical-decisions` | making/documenting a technology or architecture decision — evaluating ≥2 alternatives against weighted criteria, trade-offs + consequences, brownfield-alignment scoring, ADR record depth, marking NEEDS CLARIFICATION; owns the decision *technique* (the `constraints-and-decisions.md` artifact it fills is owned by `authoring-technical-requirements`) |
| `patterns-system-design` | designing a feature's architecture at design time — the container-level topology + current→target delta (`architecture.md`): a C4-container delta diagram (mermaid flowchart carrier — subgraph boundaries, protocol+purpose arrows, delta styled), sequence diagrams for qualifying flows (≥2 components, non-trivial ordering/failure), a container-level component table + `D-XXX`-linked delta summary, and a conditional deployment view; seeds/bootstraps the current-state baseline, scopes to the delta neighborhood; authored **before** `data-model.md`/`contracts` (which conform to the approved shape) — distinct from `authoring-architecture` (the repo-level `ARCHITECTURE.md` operating doc, folded post-hoc at landing) |
| `patterns-entity-modeling` | modeling a feature's domain data — entities, attributes/conceptual types, relationships (cardinality + delete behavior), state machines, and per-attribute data-sensitivity classification (the canonical 4-level Public/Internal/Confidential/Restricted taxonomy); authors the canonical `data-model.md` — the single home for data-sensitivity |
| `patterns-api-contracts` | designing the API-contract layer — user action → REST endpoint (method/idempotency/naming), request/response schemas (mapping data-model types to OpenAPI), error design, list pagination, and the per-endpoint `x-integration` boundary for endpoints wrapping external systems; assembles `contracts/api.yaml` — owns the API contract + x-integration format |
| `review-plan-artifacts` | independently grading the producer's plan artifacts for **completeness** — coverage / measurability / presence / cross-artifact consistency over the analysis + design sets, plus the **architecture artifact** (component-table↔diagram coverage, qualifying-flow sequence coverage, data-model/contracts conform to the approved shape) → severity-classified gaps + 3-state `ready / needs-revision / critical-gaps` (the mirror-checklist half of the plan-review pair; an independent reviewer, never the author) |
| `review-feasibility` | adversarially grading the producer's plan artifacts for cross-artifact **feasibility** — contradiction / impossibility / buildability that no single artifact reveals, plus the **architecture pass** (topology feasibility + governance conformance) when `architecture.md` is in scope → 3-state `feasible / needs-revision / infeasible` (the adversarial-critique half of the plan-review pair; the distinct `infeasible` = a business-level escalation; an independent reviewer, never the author) |
| `plan-template` (template) | the `plan.md` deliverable the lead assembles at Phase 4 — rolling up Key Decisions · Infrastructure/IP-XXX (constraints-and-decisions) · Entities+Sensitivity (data-model) · Endpoints+Integration (contracts/api.yaml); the lead's fill-target |
| `techanalyst-report-template` (template) | the technical-analyst producer's per-round self-disclosure (what was produced, what changed this round) — filled alongside the analysis/design artifacts, read directly by the lead + reviewers; carries no verdict |
| `feasibility-report-template` (template) | the feasibility reviewer's cross-artifact critique — the contradiction taxonomies, the 3-state feasibility verdict, and the 4-field per-issue gate fuel the human gate reads |

> The plan **completeness** reviewer reuses the shared `advocate-report-template` (registered under Specify, above) as-is — there is no plan-specific completeness report template.

### Structuring — mapping & tasks (model-invoked — auto-reached within the `/mochiko:plan` run's structuring stage)
| Skill | Reach when |
|-------|------------|
| `patterns-vertical-tdd` | structuring an accepted plan into implementation tasks — vertical-slice identification (foundation vs feature cycles), test-first cycle structure (red/green/refactor), the `**TEST:**` verification-task grammar, and story→cycle→task traceability; the `task-architect` producer's authoring skill (teaches the craft — the `tasks.md` skeleton it fills is `tasks-template`) |
| `review-task-artifacts` | independently grading the producer's task artifacts (`task-mapping.md` / `tasks.md`) for **completeness** — vertical-slice integrity, TDD test-first ordering, `**TEST:**` presence, cycle sizing, and story→cycle→task traceability → severity-classified gaps + 3-state `ready / needs-revision / critical-gaps` (the reviewer's skill; an independent reviewer, never the author). **Boundary:** this grades **task artifacts**; `review-plan-artifacts` (Plan, above) grades **plan artifacts** — disjoint artifacts, disjoint checks |
| `tasks-template` (template) | the `tasks.md` deliverable the `task-architect` fills — the cycle→TDD-task skeleton (foundation sequential + feature `[P]`, per-task file path, `[US#]`, `[EXTEND]`/`[MODIFY]` markers, `**TEST:**` block, and the Story→Cycle table as a derived echo of `task-mapping.md`) |
| `taskarchitect-report-template` (template) | the `task-architect` producer's per-round self-disclosure (what was produced, vertical-slice rationale, TDD structure) — filled alongside `task-mapping.md`/`tasks.md`, read directly by the lead; carries no verdict |

### Implement / execute cluster (model-invoked — auto-reached during a `/mochiko:implement` run)
| Skill | Reach when |
|-------|------------|
| `executing-tdd-cycle` | executing an accepted cycle's task list at runtime — red/green/refactor TDD, runtime task parsing, targeted rework when specific tasks fail, and the `cycle-report.md` schema; the `staff-engineer` producer's execution skill. Runtime cycle **EXECUTION** — for design-time cycle *structuring* / test-first ordering, that's `patterns-vertical-tdd` (Tasks) |
| `testing-end-user` | runtime verification against real infrastructure — parsing `**TEST:**` tasks, executing Setup/Action/Assert, capturing evidence, running the quality gates + classifying results by exit code, the runtime CLI/GUI/SUBJECTIVE confidence classification, and the verification report + checkpoint presentation; the `qa-engineer` validator's skill (mounted on qa **only**, never on staff) |
| `brownfield-integration` | safely **EXTEND**ing/**MODIFY**ing existing code at implement-time — the read-before-write checklist, interface preservation, and conflict detection when a cycle touches an existing codebase; the `staff-engineer` producer's 2nd skill (the `[EXTEND]`/`[MODIFY]` marker vocabulary itself is owned by `patterns-vertical-tdd`) |

### Brainstorm cluster (model-invoked — auto-reached during a `/mochiko:brainstorm` run)
| Skill | Reach when |
|-------|------------|
| `review-brainstorm` | serving as a cold **end-stage reviewer** of a thinking session's `record.md` (lens-briefed pair or solo per the user's sizing) — independent cold read first, then the one-shot cross-examination → survivors + tally + recommended status (never a session co-author; verdicts lead-owned) |

> The questioning engine is `analysis-iterative` (registered under Specify, above — a general/shared skill); it is not brainstorm-specific.

### Slice cluster (model-invoked — auto-reached during a `/mochiko:slice` run)
| Skill | Reach when |
|-------|------------|
| `authoring-slices` | decomposing an accepted spec into graduation slices — the `slices.md` overlay (story→slice homes, dependency-closed order, foundation designation, Feature-Done, spec stamp), with the null exit when a spec lacks ≥2 distinct value seams; the `task-architect`'s decomposition skill |
| `review-slices` | independently grading a `slices.md` decomposition against the spec it indexes — coverage, dependency closure, foundation legitimacy, Feature-Done, overlay purity, and the depth second-guess in both directions → severity-classified gaps + 3-state recommended verdict (never the author) |
| `slices-template` (template) | the `slices.md` deliverable — Slice-order, per-slice blocks, Feature-Done, and the **Graduation contract**: the single source of how downstream slice-scoped runs consume the file |
| `slicer-report-template` (template) | the producer's per-round self-disclosure (drafted decomposition or the null-exit recommendation, slicing rationale, Feature-Done coverage) — read directly by the lead; carries no verdict |

> Downstream consumption is carried by the artifact itself: `/mochiko:plan` and `/mochiko:implement` each carry a slice-scoped entry variant keyed to `slices.md` presence, honoring its Graduation contract. Feature-close execution of the Feature-Done section has no owning workflow yet (deferred to the `audit` scoping — see `BACKLOG.md`).

### Framework maintenance (model-invoked — reached when authoring or auditing mochiko's own commands)
| Skill | Reach when |
|-------|------------|
| `authoring-commands` | authoring a new `commands/*.md` supervisor in the codified shape, converting a command's form, or running a strip wave with version-stamped notes (`.mochiko/strips/`, repo-side); the `command-architect`'s skill — references `command-shape.md`, never restates it |
| `validation-command-shape` | independently grading a command's conformance to the shape — deterministic grep floor (references present, no restated single-sourced prose, exceptions marked, frontmatter) beneath a prose judgment ceiling (altitude, parameter completeness, contract-fill soundness), plus the strip-note audit at wave close → binary PASS/FAIL + fix list on the `validator` persona (never the author) |

### Entry point (user-invoked — you run it)
| Command | Reach when |
|---------|------------|
| `/mochiko:setup` | you want to create, amend, or brownfield-derive the project's governance (lands on native surfaces — no constitution.md). The lead interrogates your intent (ten dimensions), a **sized cold review** stress-tests the synthesis, you ratify at a named checkpoint; then a producer authors the surface set and an independent validator grades it, closing on your acceptance. **Requires agent teams** (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`); refuses without them |
| `/mochiko:specify` | you want to create a feature specification via the adversarial analyst↔advocate loop (the requirements-analyst authors `spec.md`, the devils-advocate stress-tests it) with a human acceptance gate |
| `/mochiko:slice` | you want to decompose an accepted spec into **graduation slices** — ordered story groups that then run `/mochiko:plan` → `/mochiko:implement` **per slice** instead of whole-spec — via the task-architect→devils-advocate loop with a human acceptance gate on `slices.md`; null-exit-aware (a spec without ≥2 distinct value seams gets a reviewed whole-spec recommendation instead) |
| `/mochiko:plan` | you want to turn an accepted spec into an accepted implementation **package** — analysis, the **architecture** delta (signed off early at a rendered-diagram gate), detailed design, and the task breakdown (`task-mapping.md` + `tasks.md`) — via a producer→reviewer team loop (technical-analyst + system-architect + task-architect author; principal-architect grades feasibility then the architecture's topology/governance, devils-advocate grades completeness then the task artifacts) with the architecture sign-off early and one final package-acceptance gate; slice-scoped when `slices.md` is present; next step `/mochiko:implement` |
| `/mochiko:implement` | you want to turn an accepted `tasks.md` into working, verified code — cycle-by-cycle (foundation → feature) via the staff-engineer→qa-engineer producer→verifier loop, with a confidence-based per-cycle gate and a named final-acceptance gate on the implementation |
| `/mochiko:brainstorm` | you want to think a problem through one question at a time — a fact-checker verifies claims against the files as you go; at convergence **you size the cold review** at a named gate (lens-split pair default / single / recorded waiver); the deliverable is `record.md`, pipeline entry offered, never defaulted. **Requires agent teams** (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`); refuses without them |

### Agents (dispatched by the supervisor)
| Agent | Role |
|-------|------|
| `principal-architect` | **cross-workflow** — setup-cluster author (authors/updates the governance surface set, greenfield + brownfield; runs codebase analysis) **and** plan-cluster **feasibility reviewer** (grades the analyst's plan artifacts for cross-artifact buildability; grades a different agent's work, never its own authoring) (skills: authoring-constitution, analysis-codebase, review-feasibility) |
| `requirements-analyst` | specify-cluster producer — authors the feature `spec.md` (prioritized user stories + FR/SC requirements) (skills: authoring-requirements, authoring-user-stories) |
| `devils-advocate` | **cross-workflow** adversarial reviewer — spec-gap critic, plan completeness, task artifacts, brainstorm end-stage (sized pair/solo; spawn prompts name skill + role + lens, since teammates ignore `skills:` frontmatter), setup G3 intent (sized), and slice decomposition; recommends verdicts that feed the lead's clearing decision, never the gate (skills: review-specifications, review-plan-artifacts, review-task-artifacts, review-brainstorm, review-governance-intent, review-slices) |
| `system-architect` | plan-cluster PRODUCER — authors the design-time architecture artifact (`architecture.md`: container-level topology + current→target delta, qualifying-flow sequence diagrams, component register + `D-XXX`-linked delta summary); topology judgment (boundaries, sync/async, responsibility placement, buildability) upstream of entity/contract detail; never grades its own output (skills: patterns-system-design, patterns-technical-decisions) |
| `technical-analyst` | plan-cluster PRODUCER — authors the analysis+design artifacts (requirements · constraints-and-decisions · NFRs · data-model · API contracts · quickstart when applicable) downstream of the approved architecture; never grades its own output (skills: authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts) |
| `task-architect` | **cross-workflow** PRODUCER — the `/mochiko:plan` structuring stage (structures the accepted design into `task-mapping.md` story→cycle mapping + `tasks.md` cycle-based TDD task list) **and** slice-cluster (decomposes an accepted spec into the `slices.md` graduation-slice overlay: foundation designation, dependency-closed ordering, Feature-Done declaration); never grades its own output (skills: patterns-vertical-tdd, authoring-slices) |
| `staff-engineer` | implement-cluster PRODUCER — implements each cycle through red/green/refactor TDD and brownfield EXTEND/MODIFY integration; emits an honest `cycle-report.md`; never grades its own output; the verification skill is never mounted here (skills: executing-tdd-cycle, brownfield-integration) |
| `qa-engineer` | implement-cluster VALIDATOR — independently verifies each cycle against real infrastructure (quality-gate exit codes + captured evidence), emits a verification report + checkpoint recommendation that feeds the lead's verdict; grades a different agent's work, mounts no producer skill (skills: testing-end-user) |
| `validator` | one generic independent grader for any cluster — grades a finished artifact against a checklist, defaults to FAIL, authors nothing (skills: validation-constitution, validation-command-shape) |
| `command-architect` | **framework-facing producer** — authors/converts commands in the codified shape and first-passes strip waves; never grades its own output — the `validator` audits via `validation-command-shape` (skills: authoring-commands) |

## Operating rules (context hygiene)

- **Always cross the producer↔validator boundary.** The author never grades its own output; the lead dispatches an independent validator that Reads the artifact itself. Never mount producer and validator skills on one agent.
- **The lead is the command, not an agent.** Verdict ownership, iteration bounds, and the human gate live in the workflow's `commands/<name>.md` supervisor, not in any persona.
- **Keep a producer↔validator round in one unbroken context** so the validator reasons across the whole artifact at once — a fresh context loses the picture.

## Adding to the library

New primitives register here when they are authored. A primitive that is not in this router fails discoverability — it is, by construction, undiscoverable.
