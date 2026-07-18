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

## Flow graph

```
  ┌─ doctrine (consumed by every workflow) ─────────────────────────────┐
  │   loop-discipline ........ the four sound-loop rules                  │
  │   workflow-contract ...... the fill-in loop contract (template)      │
  └──────────────────────────────────────────────────────────────────────┘

  Each workflow below is a sound loop: a command supervisor stitches a
  producer/validator agent team to a goal under a workflow-contract.
```

**The two review-skill families** (the `validation-*`/`review-*` split, 2026-07-18 — design:
`.mochiko/brainstorms/setup-adversarial-review/record.md` D5): the prefix encodes **who owns the
clearing**. `validation-*` = the skill **issues the authoritative grade** — a binary PASS/FAIL
checklist grade, default FAIL, on the `validator` persona (today: `validation-constitution`
alone; its PASS is still human-gated downstream). `review-*` = the skill **produces
lead-adjudicated input** — severity-ranked findings and a *recommended* status that the lead or a
human adjudicates; the reviewer's verdict never clears anything by itself.

## When to reach each

### Doctrine (model-invoked — auto-reached when designing any loop)
| Skill | Reach when |
|-------|------------|
| `loop-discipline` | designing/reviewing any workflow or agent loop; deciding if a loop is sound; filling a `workflow-contract` |
| `workflow-contract` (template) | instantiating the contract for a specific workflow |
| `agent-dispatch` (template) | briefing each agent dispatch inside a loop — a caller-side guide, not a gate |

### Setup cluster (model-invoked — auto-reached during a `/mochiko:setup` run)
| Skill | Reach when |
|-------|------------|
| `authoring-constitution` | authoring or amending the project's **governance surface set** — no constitution.md: a marked CLAUDE.md governance region (stamp, index, universal principles, stack, gates summary), `paths`-scoped `.claude/rules/mochiko/` files, skill pointers, and the governance ledger (`.mochiko/memory/governance-ledger.md`), plus the trace summary manifest — formulating the ratified session synthesis (`governance-intent.md`, the traceable contract: selection is the session's, formulation + surface routing the producer's) into three-part principles; greenfield (from deck rulings + minted intents) OR brownfield (additionally codifying existing patterns: floor assessed against the code + Emergent Ceiling, from `codebase-analysis.md`); one skill, both modes |
| `analysis-codebase` | analyzing an existing codebase during a brownfield setup run — deterministic stack detection (`detect-stack.sh`) + architecture/convention extraction + tier-blind Essential-Floor status assessment, producing `.mochiko/memory/codebase-analysis.md` |
| `validation-constitution` | independently grading a drafted governance surface set (CLAUDE.md region + rules files + ledger, against the synthesis and the trace summary manifest): two-way trace closure, region/surface integrity (markers, index→home resolution, `paths` scoping, no universal-in-rules), three-part structure in the ledger, tier/waiver/floor-accounting checks, module-parameterized checks, anti-pattern + placeholder scan, quantification, semantic version-bump → binary PASS/FAIL + fix list (run by an independent validator, never the author) |
| `review-governance-intent` | serving as a cold **G3 intent reviewer** in a `/mochiko:setup` run — stress-testing the frozen, confidence-marked synthesis (`governance-intent.md`) against the interrogation agenda (+ `codebase-analysis.md` in brownfield) *before* the user ratifies it: missed dimensions, unchallenged tier calls, passive card acceptances, too-easily-resolved reality conflicts, thin-rationale echo hunts (marks are lead-self-reported — audited, never a shield); lens-split pair (coverage / coherence) or solo per the tier-keyed sizing ruling; one-shot cross-exam (single-sourced protocol) → survivors + tally + recommended status; the verify pass on the coherence lens (or solo), plus the bounded G3-edit delta-pass (an independent reviewer, never a session participant; clearing is the lead's + G3's) |

### Specify cluster (model-invoked — auto-reached during a `/mochiko:specify` run)
| Skill | Reach when |
|-------|------------|
| `analysis-iterative` (general/shared) | brainstorming or enriching sparse feature input before a spec is authored — adaptive questioning to surface Who/Problem/Value; a cross-cluster conditioner the lead invokes as a pre-step when feature input is sparse |
| `review-specifications` | finding gaps in a drafted `spec.md` — severity-bucketed gap-finding + clarifying questions that feed the lead's verdict (the critic's skill; stays a gap-finder, owns no clearing verdict) |
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
| `review-plan-artifacts` | independently grading the producer's plan artifacts for **completeness** — coverage / measurability / presence / cross-artifact consistency over the analysis + design sets → severity-classified gaps + 3-state `ready / needs-revision / critical-gaps` (the mirror-checklist half of the plan-review pair; an independent reviewer, never the author) |
| `review-feasibility` | adversarially grading the producer's plan artifacts for cross-artifact **feasibility** — contradiction / impossibility / buildability that no single artifact reveals → 3-state `feasible / needs-revision / infeasible` (the adversarial-critique half of the plan-review pair; the distinct `infeasible` = a business-level escalation; an independent reviewer, never the author) |
| `plan-template` (template) | the `plan.md` deliverable the lead assembles at Phase 4 — rolling up Key Decisions · Infrastructure/IP-XXX (constraints-and-decisions) · Entities+Sensitivity (data-model) · Endpoints+Integration (contracts/api.yaml); the lead's fill-target |
| `techanalyst-report-template` (template) | the technical-analyst producer's per-round self-disclosure (what was produced, what changed this round) — filled alongside the analysis/design artifacts, read directly by the lead + reviewers; carries no verdict |
| `feasibility-report-template` (template) | the feasibility reviewer's cross-artifact critique — the contradiction taxonomies, the 3-state feasibility verdict, and the 4-field per-issue gate fuel the human gate reads |

> The plan **completeness** reviewer reuses the shared `advocate-report-template` (registered under Specify, above) as-is — there is no plan-specific completeness report template.

### Tasks cluster (model-invoked — auto-reached during a `/mochiko:tasks` run)
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
| `review-brainstorm` | serving as a cold **end-stage reviewer** of a thinking session's `record.md` — one of a lens-briefed pair (decision-quality vs record-integrity), or solo when the user sized the review down — independent cold read FIRST (scenario stress; the five hunt classes; reality-grounding against the record's fact-checker map, sample-audited against files on the integrity lens; the `references/RECORD-FITNESS.md` standalone-record checklist) with the counterpart withheld until findings are formed; then the one-shot four-message cross-examination (attack theirs, defend or owner-withdraw yours — persuade, never veto; facts → fact-checker, never re-routed) → your survivors + tally + recommended `ready / needs-revision / critical-gaps` (cross-set merge lead-owned), plus the verify pass when assigned (integrity lens, or solo) and the fidelity sample of an on-request synthesis (an independent reviewer, never a session co-author; verdicts are lead-owned) |

> The questioning engine is `analysis-iterative` (registered under Specify, above — a general/shared skill); it is not brainstorm-specific.

### Slice cluster (model-invoked — auto-reached during a `/mochiko:slice` run)
| Skill | Reach when |
|-------|------------|
| `authoring-slices` | decomposing an accepted spec into graduation slices — authoring the `slices.md` overlay: story→slice assignment (exactly-one-home), dependency-closed ordering, foundation-slice designation (shared design core AND a testable journey), cross-cutting extend obligations, the Feature-Done declaration (SC coverage map + cross-slice seams), the spec stamp, and the null exit (recommend whole-spec when a spec lacks ≥2 distinct value seams); the `task-architect` producer's decomposition skill — its cycle-slicing judgment, one level up |
| `review-slices` | independently grading a `slices.md` decomposition against the spec it indexes — story coverage, dependency closure, foundation legitimacy, sizing, extend-obligation visibility, Feature-Done SC coverage + seams, spec-stamp accuracy, overlay purity, and the depth second-guess in both directions (incl. grading a null-exit depth call) → severity-classified gaps + 3-state `ready / needs-revision / critical-gaps` (the reviewer's skill; an independent reviewer, never the author) |
| `slices-template` (template) | the `slices.md` deliverable the producer fills — Slice-order, per-slice blocks, cross-cutting placements, the Feature-Done section, and the **Graduation contract**: the single source of how downstream slice-scoped runs consume the file (staleness guard, shared-vs-per-slice artifact layout, extend-mode, graded amendment, regression safety) |
| `slicer-report-template` (template) | the producer's per-round self-disclosure (drafted decomposition or the null-exit recommendation, slicing rationale, Feature-Done coverage) — read directly by the lead; carries no verdict |

> Downstream consumption is carried by the artifact itself: `/mochiko:plan`, `/mochiko:tasks`, and `/mochiko:implement` each carry a slice-scoped entry variant keyed to `slices.md` presence, honoring its Graduation contract. Feature-close execution of the Feature-Done section has no owning workflow yet (deferred to the `audit` scoping — see `BACKLOG.md`).

### Entry point (user-invoked — you run it)
| Command | Reach when |
|---------|------------|
| `/mochiko:setup` | you want to create, amend, or brownfield-derive the project's governance (greenfield \| brownfield \| amend) — no constitution.md: governance lands on native surfaces (a marked CLAUDE.md governance region, `paths`-scoped rules files, skill pointers, a governance ledger). The lead interrogates your governance intent first (tier, type, risk, values; ten dimensions, deck arbitration, waiver rulings), a **sized cold review** stress-tests the confidence-marked synthesis before ratification (tier-keyed default — pair at `production`/`regulated`, single at `poc`/`internal`, or a recorded waiver; you rule at the gate), and you ratify the synthesis at a named checkpoint before anything is authored; then a producer teammate authors the surface set from that synthesis as a traceable contract and an independent validator teammate grades trace closure from the files, closing on your acceptance with a trace summary. **Requires agent teams** (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`); refuses without them |
| `/mochiko:specify` | you want to create a feature specification via the adversarial analyst↔advocate loop (the requirements-analyst authors `spec.md`, the devils-advocate stress-tests it) with a human acceptance gate |
| `/mochiko:slice` | you want to decompose an accepted spec into **graduation slices** — ordered story groups that then run `/mochiko:plan` → `/mochiko:tasks` → `/mochiko:implement` **per slice** instead of whole-spec — via the task-architect→devils-advocate loop with a human acceptance gate on `slices.md`; null-exit-aware (a spec without ≥2 distinct value seams gets a reviewed whole-spec recommendation instead) |
| `/mochiko:plan` | you want to create a feature's analysis→design implementation plan via the producer→two-reviewer loop (technical-analyst authors; principal-architect grades feasibility, devils-advocate grades completeness) with a human acceptance gate on `plan.md` — slice-scoped when `slices.md` is present (extend-mode over the accumulated shared artifacts) |
| `/mochiko:tasks` | you want to structure an accepted plan into implementation tasks — `task-mapping.md` (story→cycle mapping) + `tasks.md` (cycle→TDD tasks) — via the task-architect→devils-advocate loop with a human acceptance gate on `tasks.md`; next step `/mochiko:implement` |
| `/mochiko:implement` | you want to turn an accepted `tasks.md` into working, verified code — cycle-by-cycle (foundation → feature) via the staff-engineer→qa-engineer producer→verifier loop, with a confidence-based per-cycle gate and a named final-acceptance gate on the implementation |
| `/mochiko:brainstorm` | you want to think a problem through with the lead one question at a time — a fact-checker (seated from the start when the topic touches existing code) maps the reality surface into the record verbatim and verifies claims against the files as you go; at convergence **you size the review** at a named gate: a **lens-split cold pair** by default (independent reads, one four-message cross-examination, only survivors reach you), a single reviewer for a lean record, or none (waiver recorded); the deliverable is one decision record (`record.md`; a fidelity-checked `synthesis.md` on request after acceptance), with pipeline entry offered, never defaulted. **Requires agent teams** (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`); refuses without them |

### Agents (dispatched by the supervisor)
| Agent | Role |
|-------|------|
| `principal-architect` | **cross-workflow** — setup-cluster author (authors/updates the governance surface set, greenfield + brownfield; runs codebase analysis) **and** plan-cluster **feasibility reviewer** (grades the analyst's plan artifacts for cross-artifact buildability; grades a different agent's work, never its own authoring) (skills: authoring-constitution, analysis-codebase, review-feasibility) |
| `requirements-analyst` | specify-cluster producer — authors the feature `spec.md` (prioritized user stories + FR/SC requirements) (skills: authoring-requirements, authoring-user-stories) |
| `devils-advocate` | **cross-workflow** adversarial reviewer — specify-cluster critic (grounded, severity-bucketed spec-gap review), plan-cluster **completeness reviewer** (coverage / measurability / consistency / presence over the plan artifacts), tasks-cluster **task-artifact reviewer** (vertical-slice integrity, TDD structure, story→cycle→task traceability over `task-mapping.md`/`tasks.md`), brainstorm-cluster **end-stage reviewer(s)** (sized at the user's convergence gate — a lens-split cold pair by default over the finished record: independent findings first, then a one-shot cross-examination; survivors only — spawn prompts name the skill + role + lens, since teammates ignore `skills:` frontmatter), setup-cluster **G3 intent reviewer(s)** (sized at the tier-keyed gate — a coverage/coherence cold pair or solo over the frozen, confidence-marked synthesis before ratification), **and** slice-cluster **decomposition reviewer** (coverage, dependency closure, foundation legitimacy, Feature-Done coverage, depth over `slices.md`); recommends a verdict that feeds the lead's clearing decision, never the gate (skills: review-specifications, review-plan-artifacts, review-task-artifacts, review-brainstorm, review-governance-intent, review-slices) |
| `technical-analyst` | plan-cluster PRODUCER — authors the six analysis+design artifacts (requirements · constraints-and-decisions · NFRs · data-model · API contracts · quickstart); never grades its own output (skills: authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts) |
| `task-architect` | **cross-workflow** PRODUCER — tasks-cluster (structures an accepted plan into `task-mapping.md` story→cycle mapping + `tasks.md` cycle-based TDD task list) **and** slice-cluster (decomposes an accepted spec into the `slices.md` graduation-slice overlay: foundation designation, dependency-closed ordering, Feature-Done declaration); never grades its own output (skills: patterns-vertical-tdd, authoring-slices) |
| `staff-engineer` | implement-cluster PRODUCER — implements each cycle through red/green/refactor TDD and brownfield EXTEND/MODIFY integration; emits an honest `cycle-report.md`; never grades its own output; the verification skill is never mounted here (skills: executing-tdd-cycle, brownfield-integration) |
| `qa-engineer` | implement-cluster VALIDATOR — independently verifies each cycle against real infrastructure (quality-gate exit codes + captured evidence), emits a verification report + checkpoint recommendation that feeds the lead's verdict; grades a different agent's work, mounts no producer skill (skills: testing-end-user) |
| `validator` | one generic independent grader for any cluster — grades a finished artifact against a checklist, defaults to FAIL, authors nothing (skills: validation-constitution) |

## Operating rules (context hygiene)

- **Always cross the producer↔validator boundary.** The author never grades its own output; the lead dispatches an independent validator that Reads the artifact itself. Never mount producer and validator skills on one agent.
- **The lead is the command, not an agent.** Verdict ownership, iteration bounds, and the human gate live in the workflow's `commands/<name>.md` supervisor, not in any persona.
- **Keep a producer↔validator round in one unbroken context** so the validator reasons across the whole artifact at once — a fresh context loses the picture.

## Adding to the library

New primitives register here when they are authored. A primitive that is not in this router fails discoverability — it is, by construction, undiscoverable.
