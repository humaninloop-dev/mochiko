# Architecture — the mochiko plugin

Current-state map of the shipped plugin at [`plugins/mochiko/`](plugins/mochiko/) (v0.48.0,
[`plugin.json`](plugins/mochiko/.claude-plugin/plugin.json)). Scope is the plugin only — the
repo-side knowledge plane (`.mochiko/`, the operating docs) is covered by
[`CLAUDE.md`](CLAUDE.md). Rationale for every boundary here lives in the decisions layer
([`DECISIONS.md`](DECISIONS.md)); this doc records the resulting system. Maintained per
`mochiko:authoring-architecture`: updated at landings that change components, boundaries, or
data flow.

## System overview

Mochiko is a kernel-free Claude Code plugin: a product-delivery pipeline (governance → spec →
slices → plan → implementation) run entirely through native primitives — markdown command
supervisors, agent-team personas, and skills. There is no orchestration engine: each command
*is* the orchestrator for its workflow, and every command is a goal + harness contract the lead
composes a run toward, with named decisions reserved to the user.

```mermaid
flowchart LR
  user(("User"))
  user -->|"/mochiko:* + gate rulings"| commands
  subgraph plugin ["plugins/mochiko/"]
    commands["commands/ — 5 supervisors"]
    agents["agents/ — 8 personas"]
    skills["skills/ — 25 skills"]
    templates["templates/ — artifact + report schemas, doctrine homes"]
    commands -->|"spawn seats, each dispatch self-briefed"| agents
    agents -->|"carry procedure from"| skills
    commands -->|"fill-targets"| templates
  end
  agents -->|"author / grade"| target[("target project: .mochiko/ artifacts,<br/>governance surfaces, working code")]
```

The pipeline the commands form (each stage user-gated; slicing rides inside specify; brainstorm's record
may seed any downstream stage — drawn at its typical hand-off, specify):

```mermaid
flowchart LR
  setup["/mochiko:setup"] --> specify["/mochiko:specify"]
  specify -->|"per slice when the spec's Delivery Slices section decomposes"| plan["/mochiko:plan"]
  specify -->|"whole spec"| plan
  plan --> implement["/mochiko:implement"]
  brainstorm["/mochiko:brainstorm"] -.->|"record may seed a stage"| specify
```

## Layer model

Four primitive layers compose every workflow. The composition conventions (the five axes) are
pinned in [`CLAUDE.md`](CLAUDE.md#skill-library-conventions-five-axes).

| Layer | Home | Count | Role |
|---|---|---|---|
| **Commands** | [`plugins/mochiko/commands/`](plugins/mochiko/commands/) | 5 | User-invoked goal+harness contracts (`disable-model-invocation: true`). Each file states its Goal (default FAIL), Harness (plan approval for producing seats · author ≠ grader independence · decisions reserved to the user), and Bindings (v8 rebuild, v0.48.0; task layer de-granularized + slice absorbed into specify, v0.49.0). The lead plans and orchestrates the run — teammates or subagents per seat is its call. |
| **Agents** | [`plugins/mochiko/agents/`](plugins/mochiko/agents/) | 8 | Personas (all `model: opus`) that carry judgment and declare `skills:`. A persona contains no trace of any workflow — decoupling by absence; caller-side context rides the dispatch brief. |
| **Skills** | [`plugins/mochiko/skills/`](plugins/mochiko/skills/) | 25 | Procedure. One user-invoked router ([`skills/mochiko/`](plugins/mochiko/skills/mochiko/SKILL.md)) indexes the other 24, which are model-invoked with graded MUST/SHOULD triggers in their descriptions. Deterministic sub-checks ride as `scripts/` inside skills (e.g. `analysis-codebase`'s `detect-stack.sh`); depth rides as `references/`. |
| **Templates** | [`plugins/mochiko/templates/`](plugins/mochiko/templates/) | 14 + `constitution-modules/` | Two kinds: **artifact schemas** (spec — Intent + Delivery Slices sections included, plan, tasks as cycle cards, governance-intent, …) over the shared `artifact-format.md` envelope; **report schemas** (per-seat reports) over the shared `report-format.md` envelope. The former doctrine homes (`workflow-contract.md`, `agent-dispatch.md`, `sized-end-stage-review.md`) were deleted at the doctrine purge (v0.46.0–v0.47.0) — their mechanics live inline in each command. `constitution-modules/` is setup's module library (knowledge-management, layer-rules, release-gates, evolution-notes). |

The plugin manifest, [`.claude-plugin/plugin.json`](plugins/mochiko/.claude-plugin/plugin.json),
registers the command, agent, and skill directories and carries the version — packaging,
outside the four layers (templates is referenced by commands and skills, not registered).

### Boundaries between layers

- **Classification** — user-invoked primitives (the 6 commands, the router skill) may invoke
  model-invoked skills; never each other.
- **Persona ⟂ workflow** — workflow knowledge reaches an agent only through its dispatch brief
  (composed by the dispatching command) and mounted skills. Spawn prompts name skill + role
  explicitly, since teammates ignore `skills:` frontmatter.
- **Producer ≠ grader** — every reviewable artifact is graded by a structurally independent
  seat: different agent, different skill. A verification skill is never mounted on the seat it
  would grade.
- **Two review families** — the skill prefix encodes who owns the clearing:
  `validation-*` issues the authoritative binary PASS/FAIL (on the `validator` persona,
  default FAIL, human-gated downstream); `review-*` produces severity-ranked findings with a
  *recommended* status that the lead adjudicates — it never clears anything by itself.
- **Single-sourced homes** — the command shape, the report/artifact envelopes, and each
  cluster's contract live in exactly one file; commands and skills reference them at altitude
  (pointer depth, never restated).

### Command form (goal + harness)

Every command is a goal + harness contract (v8 rebuild, v0.48.0; task layer de-granularized + slice absorbed into specify, v0.49.0): a verifiable done-condition
that defaults to FAIL, a harness — plan approval before any producing seat works, author ≠
grader independence, the decisions reserved to the user — and the bindings the lead cannot
invent (paths, templates, entry conditions). The lead plans and orchestrates the run within
that frame; teammates vs subagents is its per-seat call. There is no run registry and no
daemon; commands evolve independently.

## Cluster map

Each workflow is one command plus its seats — the team roles it spawns, each an agent × the
skills briefed onto it. Full when-to-reach guidance and the
per-skill index live in the router ([`skills/mochiko/SKILL.md`](plugins/mochiko/skills/mochiko/SKILL.md));
this map records the wiring.

### Setup — governance from interrogated intent

[`commands/setup.md`](plugins/mochiko/commands/setup.md). The lead interrogates the user's
intent inline (ten dimensions via `analysis-iterative`, then the catalog deck), a sized cold
review stress-tests the synthesis before the user ratifies it, then a producer↔validator loop
authors and grades the governance surface set — there is no `constitution.md`.

| Seat | Wiring |
|---|---|
| producer | `principal-architect` × `analysis-codebase` (brownfield), `authoring-constitution` |
| intent reviewer(s) | `devils-advocate` × `review-governance-intent` — sized pair / single / waiver |
| validator | `validator` × `validation-constitution` — binary PASS/FAIL from the files |

```mermaid
flowchart LR
  lead["lead: /mochiko:setup"]
  user(("User"))
  reviewers["devils-advocate ×<br/>review-governance-intent"]
  producer["principal-architect ×<br/>authoring-constitution"]
  validator["validator ×<br/>validation-constitution"]
  lead -->|"interrogation, inline"| synthesis[("governance-intent.md")]
  synthesis --> reviewers -->|"survivors + tally"| lead
  lead -->|"ratified contract"| producer --> surfaces[("CLAUDE.md region ·<br/>.claude/rules/mochiko/ ·<br/>governance-ledger.md")]
  surfaces --> validator -->|"fix list"| producer
  lead ---|"user rulings + acceptance"| user
```

Optional post-acceptance probe: `testing-governance-injection` empirically verifies the rules
inject. `grooming-operating-docs` fires at command boundaries when a knowledge-management
cap or bound trips.

### Brainstorm — think together, review cold

[`commands/brainstorm.md`](plugins/mochiko/commands/brainstorm.md). The session is the lead and
the user (questioning inline via `analysis-iterative`), plus a fact-checker teammate — a
neutral, skill-less seat filled only when the topic has a reality surface — whose map lands
verbatim in the record. At convergence the user sizes a cold review at a named gate.

| Seat | Wiring |
|---|---|
| fact-checker | neutral empiricist, no skill mounted — conditional on a reality surface |
| reviewer(s) | `devils-advocate` × `review-brainstorm` — sized pair (decision-quality / record-integrity lenses) / single / waiver |

```mermaid
flowchart LR
  user(("User"))
  lead["lead: /mochiko:brainstorm"]
  fc["fact-checker<br/>(no skill)"]
  rev["devils-advocate ×<br/>review-brainstorm"]
  user <-->|"one question per turn"| lead
  fc -->|"reality map, verbatim"| record[("brainstorms/&lt;slug&gt;/record.md")]
  lead --> record
  record -->|"frozen, at convergence"| rev -->|"survivors + tally"| lead
  lead -->|"sizing + acceptance gates"| user
```

Deliverable: one decision record (a fidelity-checked `synthesis.md` on request). Pipeline entry
is an offer, never a default.

### Specify — feature specification

[`commands/specify.md`](plugins/mochiko/commands/specify.md). An **intent stage** opens the
run — an adaptive-probe agenda (`analysis-iterative`: scope · delivery · depth-rigor ·
constraints · out-of-scope) closing in a one-screen synthesis the user confirms, which
governs the authoring brief, the slicing shape, and the stress-test's rigor and lands as the
spec's Intent section. Then a standing author and a cold critic iterate across bounded rounds
until the user accepts. The spec carries a **Delivery Slices** section — a graduation-slice
decomposition (`authoring-slices`, lead-dispatched) or the single-slice line — co-accepted
with the spec; its Graduation contract is the single home for how slice-scoped plan/implement
runs consume it.

| Seat | Wiring |
|---|---|
| producer | `requirements-analyst` × `authoring-requirements`, `authoring-user-stories` (+ `authoring-slices` for the Delivery Slices section, or a seat of the lead's choosing) |
| critic | `devils-advocate` × `review-specifications` (Delivery Slices grade included) |

```mermaid
flowchart LR
  user(("User"))
  lead["lead: /mochiko:specify"]
  producer["requirements-analyst ×<br/>authoring-requirements +<br/>authoring-user-stories +<br/>authoring-slices"]
  critic["devils-advocate ×<br/>review-specifications"]
  user <-->|"intent probes → confirmed synthesis"| lead
  lead -->|"seeded template + intent-keyed brief"| producer
  producer --> spec[("specs/&lt;feature&gt;/spec.md<br/>(Intent · stories/FR/SC ·<br/>Delivery Slices)")]
  spec -->|"graded from the file"| critic -->|"advocate-report.md"| lead
  lead -->|"spec acceptance (whole)"| user
```

### Plan — implementation package, architecture first

[`commands/plan.md`](plugins/mochiko/commands/plan.md). Four producer stages (analysis →
architecture → detailed design → structuring) under two reviewer lenses. The architecture is
the first design artifact and stops the run at a rendered-diagram sign-off before
anything is designed against it.

| Seat | Wiring |
|---|---|
| producer | `technical-analyst` × `authoring-technical-requirements`, `patterns-technical-decisions`, `patterns-entity-modeling`, `patterns-api-contracts` |
| system-architect | `system-architect` × `patterns-system-design` — authors `architecture.md` + structural D-XXX rows (the persona also declares `patterns-technical-decisions`) |
| cycle-card producer | `patterns-vertical-tdd` on a seat of the lead's choosing — `tasks.md` as cycle cards (no fixed persona) |
| feasibility | `principal-architect` × `review-feasibility` — analysis, then the architecture pass |
| completeness | `devils-advocate` × `review-plan-artifacts` (cycle cards included) |
| architecture scribe | `principal-architect` × `authoring-architecture` — disposable, at finalize; records the initial `ARCHITECTURE.md` baseline when the target repo has none |

Case distinguishes two artifacts here: lowercase `architecture.md` is the per-feature design
artifact under `.mochiko/specs/<feature>/`; uppercase `ARCHITECTURE.md` is a repo's living
system map (the class of doc this file is), folded at landings by the scribe seats.

```mermaid
flowchart LR
  lead["lead: /mochiko:plan"]
  ta["technical-analyst<br/>analysis + detailed design"]
  sa["system-architect<br/>architecture.md"]
  cards["cycle cards ×<br/>patterns-vertical-tdd<br/>(lead-chosen seat)"]
  feas["principal-architect ×<br/>review-feasibility"]
  comp["devils-advocate ×<br/>review-plan-artifacts"]
  lead --> ta & sa & cards
  ta & sa & cards --> pkg[("specs/&lt;feature&gt;/: requirements ·<br/>constraints-and-decisions · nfrs ·<br/>architecture · data-model ·<br/>contracts/api.yaml ·<br/>tasks (cycle cards) · plan.md")]
  pkg --> feas & comp
  feas & comp -->|"reports"| lead
  lead -->|"architecture sign-off ·<br/>package acceptance"| user(("User"))
```

The architecture scribe runs at finalize, outside the round loop, and is omitted from the
diagram. Slice-scoped when the spec's Delivery Slices section decomposes: shared artifacts at
the feature root, per-slice artifacts under `slices/<slice>/`, per the spec's Graduation
contract.

### Implement — execute the cycle cards

[`commands/implement.md`](plugins/mochiko/commands/implement.md). Cycle-by-cycle (foundation
before feature), each card decomposed by its builder at build time (disclosed in the cycle
report), implemented through red/green/refactor, and independently verified
against real infrastructure. The per-cycle checkpoint carries the shape's only devolved branch:
a cycle whose verifications are all deterministic CLI checks at 100% pass, with no reported
deviation and no new domain dependencies (`domain_deps_added` empty), clears on the verifier's
evidence without a lead read — anything else fires the human-adjudicated checkpoint.

| Seat | Wiring |
|---|---|
| producer | `staff-engineer` × `executing-tdd-cycle`, `brownfield-integration` |
| verifier | `qa-engineer` × `testing-end-user` — never mounted on the producer |
| arch-diff | `principal-architect` × `authoring-architecture` — disposable, built-vs-approved at final validation |
| arch-scribe | `principal-architect` × `authoring-architecture` — disposable, folds built structure into `ARCHITECTURE.md` |

```mermaid
flowchart LR
  lead["lead: /mochiko:implement"]
  se["staff-engineer ×<br/>executing-tdd-cycle +<br/>brownfield-integration"]
  qa["qa-engineer ×<br/>testing-end-user"]
  lead -->|"cycle N"| se
  se --> code[("working code +<br/>cycle-report.md")]
  code -->|"TEST: gates, quality gates,<br/>real infrastructure"| qa
  qa -->|"verification report +<br/>recommendation"| lead
  lead -->|"deviation consent · final acceptance"| user(("User"))
```

The approved `architecture.md` is briefed input, guarded twice: the producer's diagram-anchored
deviation self-check at cycle open/close, and the arch-diff seat's built-vs-approved report at
final validation. The two disposable seats (arch-diff at final validation, arch-scribe at
finalize) run outside the cycle loop and are omitted from the diagram.

### Shared seats

Three personas serve multiple clusters — the reuse axis of the agent layer: `devils-advocate`
reviews in four (specify, plan, brainstorm, setup); `principal-architect` authors in
setup, reviews feasibility in plan, and scribes/diffs architecture in plan and implement;
`validator` grades setup's surfaces (and any artifact handed to it with an explicit
checklist). The slicing and cycle-card crafts (`authoring-slices`, `patterns-vertical-tdd`)
are seatless — lead-dispatched to whichever producer seat fits the run.

## Cross-cutting doctrine

| Primitive | Carries |
|---|---|
| `report-format.md` / `artifact-format.md` (templates) | The two shared envelopes every report and pipeline artifact follows. |
| `analysis-iterative` (skill) | The shared questioning engine — lead-inline in brainstorm, setup's interrogation, and specify's intent stage. |
| `grooming-operating-docs` (skill) | Fix-on-sight restoration of a target project's operating docs when a pinned knowledge-management cap trips at a command boundary. |

## Data flow — what lands where

The plugin ships no state and integrates with no external services — every read and write is
the **target project's** workspace:

- **Governance surfaces** — a marked region in the target's `CLAUDE.md`,
  `.claude/rules/mochiko/*.md`, skill pointers; setup-owned and idempotently regenerated.
- **`.mochiko/memory/`** — `governance-intent.md`, `governance-ledger.md`,
  `codebase-analysis.md` (brownfield), and the project-pinned `knowledge-management.md` every
  command resolves at runtime for its KM landing.
- **`.mochiko/specs/<feature>/`** — the pipeline artifacts: `spec.md` (Intent + Delivery
  Slices included), the plan package, `tasks.md` (cycle cards), per-seat reports;
  `slices/<slice>/` when slice-scoped.
- **`.mochiko/brainstorms/<slug>/`** — `record.md` (+ optional `synthesis.md`) and the session
  index.
- **The working tree** — implement's deliverable is the code itself; `tasks.md`'s per-card
  checkboxes are the progress ledger.
- **Kill-switches** — per-run stop files (`SETUP_STOP`, `SPECIFY_STOP`, `PLAN_STOP`,
  `IMPLEMENT_STOP`) checked before every seat send.

Recovery is workspace-as-state everywhere: each command's Recovery table maps artifact evidence
to a resume stage, so an interrupted run continues without any registry.
