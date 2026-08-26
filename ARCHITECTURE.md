# Architecture — the mochiko plugin

Current-state map of the shipped plugin at [`plugins/mochiko/`](plugins/mochiko/) (v0.91.0,
[`plugin.json`](plugins/mochiko/.claude-plugin/plugin.json)). Scope is the plugin only — the
repo-side knowledge plane (`.mochiko/`, the operating docs) is covered by
[`CLAUDE.md`](CLAUDE.md). Rationale for every boundary here lives in the decisions layer
([`DECISIONS.md`](DECISIONS.md)); this doc records the resulting system. Maintained per
`mochiko:authoring-architecture-store` (its predecessor `mochiko:authoring-architecture` retired v0.81.0): updated at landings that change components, boundaries, or
data flow.

## System overview

Mochiko is a kernel-free Claude Code plugin: a product-delivery pipeline (governance → spec →
implementation) run entirely through native primitives — markdown command supervisors,
agent-team personas, and skills. There is no orchestration engine: each command
*is* the orchestrator for its workflow, and every command is a contract the lead composes a run
toward — a verifiable done-condition plus a non-waivable frame, in one of two anatomies (see
Command form) — with named decisions reserved to the user.

```mermaid
flowchart LR
  user(("User"))
  user -->|"/mochiko:* + gate rulings"| commands
  subgraph plugin ["plugins/mochiko/"]
    commands["commands/ — 6 supervisors"]
    agents["agents/ — 10 personas"]
    skills["skills/ — 38 skills"]
    templates["templates/ — report schemas + envelopes"]
    schemas["schemas/ — artifact schemas (YAML)"]
    commands -->|"spawn seats, each dispatch self-briefed"| agents
    agents -->|"carry procedure from"| skills
    commands -->|"fill-targets"| templates
    skills -->|"render via mochiko-cli,<br/>or Read raw when absent"| schemas
  end
  agents -->|"author / grade"| target[("target project: .mochiko/ artifacts,<br/>governance surfaces, working code")]
```

The pipeline the commands form (each stage user-gated; work-row cutting and selection ride
inside specify; brainstorm's record may seed any downstream stage — drawn at its typical
hand-off, specify). `/mochiko:plan` retired at v0.91.0: implement is the single downstream run,
its entry sufficiency check and conditional in-run design phase absorbing what the stage did:

```mermaid
flowchart LR
  setup["/mochiko:setup"] --> specify["/mochiko:specify"]
  specify -->|"per capability-batch:<br/>one capability + its selected work rows"| implement["/mochiko:implement"]
  brainstorm["/mochiko:brainstorm"] -.->|"record may seed a stage"| specify
  feature["/mochiko:feature"] -.->|"growth rows · delta cards"| implement
  architecture["/mochiko:architecture"] -.->|"store stances the check reads"| implement
```

## Layer model

Four primitive layers compose every workflow. The composition conventions (the five axes) are
pinned in [`CLAUDE.md`](CLAUDE.md#skill-library-conventions-five-axes).

| Layer | Home | Count | Role |
|---|---|---|---|
| **Commands** | [`plugins/mochiko/commands/`](plugins/mochiko/commands/) | 6 | User-invoked contracts (`disable-model-invocation: true`) in two anatomies: `setup` / `specify` / `brainstorm` state Goal (default FAIL) · Harness (plan approval for producing seats · author ≠ grader independence · decisions reserved to the user) · Bindings (v8 rebuild, v0.48.0); `feature` / `architecture` / `implement` are six-section **charters** — Delivery-Manager lead, the always-happens floor as owned responsibilities, the non-waivable floor in Boundaries (feature desk v0.68.0; the pipeline pair v0.69.0; the architecture desk v0.81.0). `plan` retired at v0.91.0. The lead plans and orchestrates the run — teammates or subagents per seat is its call. |
| **Agents** | [`plugins/mochiko/agents/`](plugins/mochiko/agents/) | 10 | Personas (all `model: opus`) that carry judgment and declare `skills:`. A persona contains no trace of any workflow — decoupling by absence; caller-side context rides the dispatch brief. |
| **Skills** | [`plugins/mochiko/skills/`](plugins/mochiko/skills/) | 38 | Procedure. One user-invoked router ([`skills/mochiko/`](plugins/mochiko/skills/mochiko/SKILL.md)) indexes the other 37, which are model-invoked with graded MUST/SHOULD triggers in their descriptions. Deterministic sub-checks ride as `scripts/` inside skills (e.g. `analysis-codebase`'s `detect-stack.sh`); depth rides as `references/`. |
| **Templates** | [`plugins/mochiko/templates/`](plugins/mochiko/templates/) | 7 + `constitution-modules/` | **Report schemas** (per-seat reports) over the shared `report-format.md` envelope, plus that envelope and its deliverable-side twin `artifact-format.md`, and `output-style.md`. The **artifact schemas** re-homed to [`plugins/mochiko/schemas/`](plugins/mochiko/schemas/) as YAML data at v0.76.0 — the source of truth the `mochiko-cli` binary renders over and agents Read raw when it is absent (7 pipeline schemas after `plan.yaml` retired at v0.91.0, plus the two architecture-store schemas). The former doctrine homes (`workflow-contract.md`, `agent-dispatch.md`, `sized-end-stage-review.md`) were deleted at the doctrine purge (v0.46.0–v0.47.0) — their mechanics live inline in each command. `constitution-modules/` is setup's module library (knowledge-management, layer-rules, release-gates, evolution-notes). |

The plugin manifest, [`.claude-plugin/plugin.json`](plugins/mochiko/.claude-plugin/plugin.json),
registers the command, agent, and skill directories and carries the version — packaging,
outside the four layers (`templates/` and `schemas/` are referenced by commands and skills,
not registered; `schemas/` is data the Templates row above accounts for).

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

### Command form (two anatomies)

Every command states a verifiable done-condition that defaults to FAIL, a frame — plan
approval before any producing seat works, author ≠ grader independence, the decisions
reserved to the user — and the homes the lead cannot invent (paths, templates, entry
conditions). Three commands (`setup`, `specify`, `brainstorm`) carry it as the v8 **goal +
harness** anatomy — Goal · Harness · Bindings (v8 rebuild, v0.48.0; task layer
de-granularized + slice absorbed into specify, v0.49.0). Three (`feature`, `architecture`,
`implement`) carry it as six-section **charters** — Identity & Mission · Adaptive Goal
Protocol · Roles & Responsibilities · Tools · Ways of Working · Boundaries — with the
always-happens floor as the Delivery Manager's owned responsibilities and the non-waivable
floor in Boundaries (the feature desk at v0.68.0, D10; the pipeline pair at v0.69.0, ADR
`2026-08-13-charter-plan-implement`; the architecture desk at v0.81.0 — `plan` retired at
v0.91.0, leaving `implement` the pipeline's only charter). The two desks carry a **per-visit**
goal contract (converge to a done condition); `implement` carries a **per-run** one. The lead
plans and orchestrates the run within that frame; teammates vs subagents is its per-seat call.
There is no run registry and no daemon; commands evolve independently.

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
| producer | `tech-lead` × `analysis-codebase` (brownfield), `authoring-constitution` |
| intent reviewer(s) | `devils-advocate` × `review-governance-intent` — sized pair / single / waiver |
| validator | `validator` × `validation-constitution` — binary PASS/FAIL from the files |

```mermaid
flowchart LR
  lead["lead: /mochiko:setup"]
  user(("User"))
  reviewers["devils-advocate ×<br/>review-governance-intent"]
  producer["tech-lead ×<br/>authoring-constitution"]
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
UX-bearing · constraints · out-of-scope) with the capability map an obligated read, closing
in a one-screen synthesis the user confirms; within it the `product-manager` seat states the
**capability frame** — which capabilities the territory touches, extend-vs-mint — as a
nouns-and-verbs hypothesis that never enumerates stories. Stories are then authored inside
that frame (a hypothesis, not an anchor: stories win any conflict, resolved at the
post-stories confirm step). After stories the PM confirms or adjusts the frame, **cuts the
work rows** grouped per capability, and runs the story filter (rejections recorded, never
silent); the user rules the **selection** — which work rows build now — with the
per-capability completeness view (pending rows · stubs · kills) visible at the moment of
choice. The staged map delta executes as one atomic batch at spec acceptance. A UX-bearing
spec carries a **Screens & Flows** section — the SCR/FLOW manifest plus a clickable low-fi
prototype under `prototype/` (`product-engineer` × `authoring-prototype`), authored in story
lockstep, the user clicking each story's screens as they land; not UX-bearing takes the
waiver line.

| Seat | Wiring |
|---|---|
| producer | `requirements-analyst` × `authoring-requirements`, `authoring-user-stories` — stories and FR/SC authored inside the PM's frame |
| product manager | `product-manager` × `authoring-feature-map` (+ `patterns-map-minimalism` discipline) — capability frame at intent · confirm + work-row cutting + filter after stories · selection card |
| prototype producer (UX-bearing) | `product-engineer` × `authoring-prototype` (Screens & Flows manifest + `prototype/` app, story lockstep with the analyst) |
| critic | `devils-advocate` × `review-specifications` (capability/work-row derivation + map-delta grade, Screens & Flows prototype walk included) |

```mermaid
flowchart LR
  user(("User"))
  lead["lead: /mochiko:specify"]
  pm["product-manager ×<br/>authoring-feature-map +<br/>patterns-map-minimalism"]
  producer["requirements-analyst ×<br/>authoring-requirements +<br/>authoring-user-stories"]
  proto["product-engineer ×<br/>authoring-prototype"]
  critic["devils-advocate ×<br/>review-specifications"]
  user <-->|"intent probes → confirmed synthesis<br/>(capability frame included)"| lead
  lead -->|"frame brief · derivation brief"| pm
  lead -->|"seeded template + intent-keyed brief"| producer
  lead -->|"UX-bearing: story-lockstep briefs"| proto
  pm --> spec[(".mochiko/specs/&lt;spec&gt;/:<br/>spec.md (Intent · FR/SC · Screens &amp; Flows ·<br/>Feature Selection) · stories/US-*.md ·<br/>staged map delta")]
  producer --> spec
  proto --> spec
  user <-->|"clicks each story's screens"| proto
  spec -->|"graded from the files, prototype walked"| critic -->|"advocate-report.md"| lead
  lead -->|"selection + spec acceptance (whole)"| user
```

### Implement — check sufficiency, design what is missing, execute the cycle cards

[`commands/implement.md`](plugins/mochiko/commands/implement.md). The pipeline's single
downstream run since `/mochiko:plan` retired at v0.91.0
([`plan-stage-utility`](.mochiko/brainstorms/plan-stage-utility/record.md) D1–D7). Entry gates
on the ratified selection, not on a pre-built package: a **sufficiency check** grades whether
the spec, the architecture store, and the product baselines already carry enough for a builder
to build each selected work row. Zero gaps goes straight to card authoring and build; any gap
fires an **in-run design phase scoped to exactly the named gaps, nothing else**, whose output
a non-author seat grades and the user signs at a blocking checkpoint before the first cycle.
Cycles then run foundation-before-feature, each card decomposed by its builder at build time
(disclosed in the cycle report), implemented through red/green/refactor, and independently
verified against real infrastructure. The per-cycle checkpoint carries the shape's only
devolved branch: a cycle whose verifications are all deterministic CLI checks at 100% pass,
with no reported deviation and no new domain dependencies (`domain_deps_added` empty), clears
on the verifier's evidence without a lead read — anything else fires the human-adjudicated
checkpoint.

| Seat | Wiring |
|---|---|
| sufficiency grader | a seat that authored none of the graded sources × `review-sufficiency` — ten clauses per selected work row under selection scope; under delta scope a **three-clause** form per delta card (criteria testable · touched surfaces identified · store consult and trip check run). Verdict binding, a disputed clause defaults to gap and goes to the user |
| design producers (conditional) | staffing is the lead's call, scoped to the named gaps: `technical-analyst` × `patterns-technical-decisions`, `patterns-entity-modeling`, `patterns-api-contracts`, `authoring-technical-requirements` · `principal-architect` × `patterns-system-design` for a store delta · `qa-engineer` for the `**TEST:**` cases. `staff-engineer` stays the builder and never designs its own work |
| design grader | a non-author seat × `review-plan-artifacts` (completeness) / `review-feasibility` (contradiction, buildability, the architecture pass) — both re-scoped at v0.91.0 from the retired plan package onto the design-phase output and the sufficiency check's own honesty |
| card producer | `patterns-vertical-tdd` on a technical-analyst-class design seat — `tasks.md` as cycle cards, **never the builder who will execute them**; the verification seat reviews the cards, and an infeasible judgment escalates to the user as a business-level scope call |
| producer | `staff-engineer` × `executing-tdd-cycle`, `brownfield-integration`, `patterns-code-minimalism` (the pre-code ladder at decomposition, rungs disclosed) |
| verifier | `qa-engineer` × `testing-end-user`, `review-code-minimalism` (advisory `minimalism:` findings; reads diff + cycle report + codebase) — never mounted on the producer |
| arch-diff | `principal-architect` × `authoring-architecture-store` — disposable, built-vs-signed at final validation |
| arch-scribe | `principal-architect` × `authoring-architecture-store` — disposable, folds built structure into `ARCHITECTURE.md` |

```mermaid
flowchart LR
  lead["lead: /mochiko:implement"]
  suff["sufficiency grader ×<br/>review-sufficiency"]
  design["design phase<br/>(gap-scoped, conditional)"]
  cards["card producer ×<br/>patterns-vertical-tdd"]
  se["staff-engineer ×<br/>executing-tdd-cycle +<br/>brownfield-integration +<br/>patterns-code-minimalism"]
  qa["qa-engineer ×<br/>testing-end-user +<br/>review-code-minimalism"]
  user(("User"))
  lead -->|"at entry, per work row"| suff
  suff -->|"sufficiency report:<br/>verdicts · gaps · trips"| lead
  lead -->|"any gap"| design
  design --> deltas[(".mochiko/features/FEAT-XXX/:<br/>signed store delta ·<br/>data-model + contract deltas vs<br/>.mochiko/product/ baselines")]
  lead -->|"zero gaps"| cards
  deltas --> cards
  cards --> tasks[("tasks.md cycle cards")]
  tasks -->|"cycle N"| se
  se --> code[("working code +<br/>cycle-report.md")]
  code -->|"TEST: gates, quality gates,<br/>real infrastructure +<br/>minimalism lens"| qa
  qa -->|"verification report +<br/>recommendation"| lead
  lead -->|"run-open routing · design sign-off ·<br/>card confirm · deviation consent ·<br/>final acceptance"| user
```

Case distinguishes two artifacts here: lowercase `architecture.md` is the per-capability design
artifact under `.mochiko/features/FEAT-XXX/`; uppercase `ARCHITECTURE.md` is a repo's living
system map (the class of doc this file is), folded at landings by the scribe seats.

The run unit is the **capability-batch** — one capability plus exactly its selected work rows
(a `/mochiko:feature` delta card enters in delta scope and gates on the desk-confirmed card
directly); batches order by the rows' dependency closure, and design deltas land against the
`.mochiko/product/` baselines. An **epic run always fires the design phase** for the joint
spine, naming cross-member seam owners at design time. The signed store delta is guarded
twice: the producer's deviation self-check at cycle open/close, and the arch-diff seat's
built-vs-signed report at final validation — the trigger fires on any delta signed this run,
whenever signed. A builder hitting undesigned structure mid-cycle **halts the cycle**; the
design phase re-fires scoped to the discovery. Build-time `D-XXX` / `C-XXX` / `IP-XXX` writes
are never in-place edits — they land as `baseline-delta.md` entries whose judgment content the
landing verification seat grades independently; a commodity-category adopt-first ruling or an
`IP-XXX` provisioning call is never builder-decided and halts the cycle to the user. The three
disposable/entry seats (sufficiency grader at entry, arch-diff at final validation, arch-scribe
at finalize) run outside the cycle loop. The acceptance landing executes the graduation whole:
delivered work rows fold into the capability's extent (pending rows persist), every touched
product baseline takes its graded delta fold, and the map bookkeeping (index line · In-flight
pointer · specs-index row) lands in the same moment — no separate feature-close stage exists.

### Feature — the product desk

[`commands/feature.md`](plugins/mochiko/commands/feature.md). The first of the library's
**charter-form** commands (`plan.md` and `implement.md` joined at v0.69.0,
`architecture.md` at v0.81.0; `plan.md` retired at v0.91.0) — six sections
(Identity & Mission · Adaptive Goal Protocol · Roles & Responsibilities · Tools · Ways of
Working · Boundaries); its audit re-keys to *floor present + per-visit goal contract
present*. The lead is chartered **Delivery Manager of the product
desk**: the advisory front door to the capability map. A visit opens with the map-health
report (stale stubs · unfolded deltas · cap pressure · a what-next line), converges to a
one-line goal with an explicit done condition, then routes the demand by the
**capability-write test** — capability writes (mint · merge · retire · status) are sacred to
`/mochiko:specify` or a user grooming ruling; work rows are delivery bookkeeping the desk may
cut through the **growth door** (extend-verdict only; several rows, a new UX surface, or
cross-capability reach route to specify regardless). Growth rows dispatch in selection scope,
bug/improvement delta cards in delta scope; the desk runs no delivery harness. The
`product-manager` seat carries extend-vs-mint and cap-trip grooming proposals;
`principal-architect` co-signs domains, dormant until the first cap-trip.

### Architecture — the architecture desk

[`commands/architecture.md`](plugins/mochiko/commands/architecture.md), charter-form since
v0.81.0. The peer of the product desk over the **product architecture store** at
`.mochiko/product/architecture/` — capabilities are what the product does, the store is how it
is built, and neither desk writes the other's truth. A visit opens with the health view read
from the derived index (`open` rows carrying no stance · stale `not-now` revisit triggers ·
fired upgrade triggers awaiting routing · orphan in-flight elements · the drift register),
converges to a one-line goal and done condition, and closes with a verdict against it. The
lead writes no architecture truth alone: every stance, baseline, and amendment is the user's
ruling on a produced-and-graded proposal.

| Seat | Wiring |
|---|---|
| producer | `principal-architect` × `authoring-architecture-store`, `patterns-architecture-shelves` — baseline authoring (greenfield elicit, brownfield reconstruct-and-confirm), shelf-walk stance batches, amendments, delta authoring |
| judgment grader | `tech-lead` — independently grades the architect's judgment writes before the user ratifies; transcription-only changes ride the landing audit instead |
| drift probe | an empirical seat reading the codebase against the store's `As-built:` claims — never the seat that wrote the claim |

The store is the standing home the implement run's sufficiency check reads for structural
triggers and NFR concern rows, and that its design phase contests deltas against. Fired
upgrade triggers route to `/mochiko:feature`'s growth door.

### Shared seats

Several personas serve multiple clusters — the reuse axis of the agent layer:
`devils-advocate` reviews in four (specify, brainstorm, setup, and implement's design phase);
`tech-lead` authors governance in setup, grades the architect's judgment writes at the
architecture desk, and reviews feasibility in implement's design phase;
`principal-architect` is the architecture desk's producing seat, authors store deltas in
implement's design phase, scribes/diffs `ARCHITECTURE.md` at implement landings, and co-signs
domains at the product desk (dormant until the first cap-trip); `product-manager` frames and
derives in specify and carries the product desk's grooming proposals; `qa-engineer` verifies
cycles and authors the design phase's `**TEST:**` cases; `validator` grades setup's surfaces
(and any artifact handed to it with an explicit checklist). The cycle-card craft
(`patterns-vertical-tdd`) is seatless — lead-dispatched to whichever design seat fits the run,
never to the builder who will execute the cards.

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
- **`FEATURES.md` + `.mochiko/features/`** — the capability map: the index, per-capability
  entry files at `FEAT-XXX-<slug>.md` (work rows riding the entries, pending|live), and
  per-capability run dirs (`FEAT-XXX/`: the sufficiency report, design-phase deltas and the
  signed store delta when the phase ran, `tasks.md` cycle cards, per-seat reports).
- **`.mochiko/product/`** — the product baselines (`data-model.md` · `contracts/` ·
  `constraints-and-decisions.md` · `quickstart.md`) that per-capability deltas fold into at
  acceptance landings, plus the architecture store at `architecture/` (NFR concern rows and
  their targets live there since v0.81.0).
- **`.mochiko/specs/<spec>/`** — the delivery-event record: `spec.md` (Intent + Feature
  Selection), `stories/US-*.md`, the staged map delta until acceptance.
- **`.mochiko/brainstorms/<slug>/`** — `record.md` (+ optional `synthesis.md`) and the session
  index.
- **The working tree** — implement's deliverable is the code itself; `tasks.md`'s per-card
  checkboxes are the progress ledger.
- **Kill-switches** — per-run stop files (`SETUP_STOP`, `SPECIFY_STOP`, `IMPLEMENT_STOP`)
  checked before every seat send; `PLAN_STOP` died with its command at v0.91.0.

Recovery is workspace-as-state everywhere: each command's Recovery table maps artifact evidence
to a resume stage, so an interrupted run continues without any registry.
