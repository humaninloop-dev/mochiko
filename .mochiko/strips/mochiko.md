# Strip notes — `skills/mochiko/` (the router)

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 1 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
149 → 143 lines (4%) but **28,040 → 22,720 bytes = 19%** — the router's fat table rows are
single physical lines, so the R2 line denominator undercounts this skill; dual accounting
recorded here and flagged on the wave's ROADMAP row.

## [v0.74.0] Router rows re-typed: `spec`/`tasks`/`plan` template files → CLI/schema-delivered (schema-based-template-guidance D1/D8)
- **Disposition:** superseded → `plugins/mochiko/schemas/{spec,tasks,plan}.yaml` + `mochiko-cli template <name>` (raw Read of the `.yaml` is the D8-first-class degraded path). The three router-index rows are re-described as CLI/schema-delivered primitives; the `(template)` designation is ruled to `(schema)` — the shape is no longer a `templates/<t>-template.md` file. The sibling report/format rows (`analyst-report-template`, `advocate-report-template`, `report-format`, `artifact-format`, `output-style`, `techanalyst-report-template`, `feasibility-report-template`) keep `(template)` because they remain markdown template files (D3 leaves them `.md`). Discoverability preserved: each re-typed row keeps its full role description.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1 one plugin CLI · D3 the 8 pipeline templates · D8 schemas ship as data files, raw Read first-class; `DECISIONS.md` "Template-schema CLI ruled" row).
- **Content (superseded, verbatim):**
  - `| \`spec-template\` (template) | the \`spec.md\` the analyst authors and the loop converges on — lead-seeded; header \`status\` carries the loop's done-condition |`
  - `| \`tasks-template\` (template) | the \`tasks.md\` deliverable — the cycle-card skeleton (per-card checkbox as the progress surface, Stories+rationale, type, dependencies, acceptance criteria by ID, \`**TEST:**\` gate, brownfield exposure) |`
  - `| \`plan-template\` (template) | the \`plan.md\` deliverable the lead assembles at Phase 4 — rolling up Key Decisions · Infrastructure/IP-XXX (constraints-and-decisions) · Entities+Sensitivity (data-model) · Endpoints+Integration (contracts/api.yaml); the lead's fill-target |`
- **Kept deliberately:** the `(template)` designation on the seven non-in-scope report/format rows (they stay `.md`); every re-typed row's role description.
- **Consumers assessed:** router is the single index; the three re-typed rows are the only in-scope-template rows here. Sibling re-points of the same 8 templates land in the same wave (P3 template deletions · P4 command re-points · P5 skill re-points).

## [v0.69.0] Universal v8-anatomy claims → two-anatomy reality (charters × 3)
- **Disposition:** superseded → rewritten in place at four sites: the *What mochiko is*
  anatomy sentence · the *How the library composes* opening paragraph (now names both
  anatomies: `setup`/`specify`/`brainstorm` v8, `feature`/`plan`/`implement` charters) · the
  `/mochiko:feature` row's "the library's one charter command" clause ("a charter command") ·
  the operating rules' "goal+harness contract" phrase ("contract").
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-13-charter-plan-implement.md` + `DECISIONS.md` row 2026-08-13 —
  charter extended to `plan.md`/`implement.md`, D10's this-command-only clause superseded).
  Repair rider: the "Every command is goal + harness" / "Each command below states its Goal,
  Harness, and Bindings" universal lines had been stale for 1/6 since the v0.68.0 charter
  landed without re-qualifying them; this wave's re-word repairs that alongside the
  extension.
- **Content (superseded, verbatim):**
  - `Every command is **goal + harness**: a verifiable done-condition (default FAIL) plus the non-waivable frame — plan approval for producing seats, author ≠ grader independence, the decisions reserved to the user, and the bindings the lead cannot invent.`
  - `Each command below states its Goal, Harness, and Bindings — the whole contract — and the` / `lead composes the run toward it.`
  - `/mochiko:feature` row clause: `the library's one charter command`
  - Operating rule clause: `live in the workflow's \`commands/<name>.md\` goal+harness contract, not in any persona.`
- **Kept deliberately:** the done-condition/default-FAIL, plan-approval, author≠grader, and
  reserved-decisions substance in both rewritten paragraphs — carried verbatim into the new
  text ("the homes the lead cannot invent" replacing "the bindings…" since three commands no
  longer have a Bindings section) · the sound-loop sentence untouched · the `/mochiko:plan`
  and `/mochiko:implement` entry rows (behavioral, already anatomy-free at v0.68.0) · every
  other row.
- **Consumers assessed:** the router is itself the discoverability surface; the four
  superseded strings appear nowhere else in the shipped tree (grep-verified); rows verified
  against the wave's shipped charter files (`plan.md`, `implement.md` on disk, audited this
  wave).

## [v0.68.0] Capability-map re-type — map/pipeline/desk rows re-keyed; `patterns-map-minimalism` row added
- **Disposition:** superseded → rewritten in place: the `authoring-feature-map` skill row (stories-first feature derivation → capability map, PM frame, work-row cutting pending|live) · the Specify-cluster consumption note (feature-keyed gating → capability-batch gating with the growth/delta scope split; folds extended to work rows) · the `/mochiko:specify` row (features-derived-from-stories → frame-first + work rows + completeness view) · the `/mochiko:feature` row (map-stewardship/lane wording incl. "promote a flat entry to parent" → the product-desk charter identity; promotion died) · the `/mochiko:plan` row (one-run-per-feature → capability-batch + row dependency closure) · the `/mochiko:implement` row (feature keying + delta folds → capability-batch + row folds with pending-rows-persist) · the `product-manager` agent row (feature derivation from stories → frame at intent + work-row cutting + desk grooming). The `patterns-map-minimalism` skill row is a pure addition (rides the decision row).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-13 "PM role & feature derivation ruled (D1–D12)" incl. its build rider; record `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md` D2/D5/D6/D7/D8 + D8's build-time rider + D9/D10/D12 and the D6 exhaustive inventory)
- **Content (superseded, verbatim):**
  - `authoring-feature-map` row: `| \`authoring-feature-map\` | deriving and maintaining the repo-level **feature map** (\`FEATURES.md\` index + \`.mochiko/features/\` entries) — the intent-stage map-read agenda, stories-first feature derivation with the story filter (recorded rejections), FEAT-XXX entry authoring (capability/extent/relations/obligations), D8 delta grammar, acceptance-time map writes incl. the specs-index row; selection is always the user's ruling |`
  - Consumption note: `> Feature-scoped consumption is keyed to the feature entry: \`/mochiko:plan\` and \`/mochiko:implement\` gate on a feature carrying ratified scope on its entry — a spec's Feature Selection or a \`/mochiko:feature\` delta card. The design surface is two-altitude: product baselines (\`data-model.md\` · \`contracts/\` · \`nfrs.md\` · \`constraints-and-decisions.md\` · \`quickstart.md\`) live at \`.mochiko/product/\` beside \`ARCHITECTURE.md\` (repo root); each feature's deltas and run artifacts live at \`.mochiko/features/FEAT-XXX/\` (entries stay \`.mochiko/features/FEAT-XXX-<slug>.md\`); acceptance folds deltas into the baselines. Deferred SCs, seams, and obligations ride the map entries (\`mochiko:authoring-feature-map\`); implement's acceptance landing executes the graduation bookkeeping — no separate feature-close stage exists.`
  - `/mochiko:specify` row's superseded clauses: `you want to spec **new-capability** work — an intent stage you confirm first (the feature map an obligated read: scope · delivery · depth · UX-bearing), stories authored, features derived from them with the story filter (rejections recorded, never silent), … a **Feature Selection** you rule on (deferred SCs shown at the moment of choice)`
  - `/mochiko:feature` row: `| \`/mochiko:feature\` | you want to work the map directly — stewardship (view/query, park a capability stub as an \`unrefined\` \`proposed\` entry, promote a flat entry to parent, retire, integrity grooming) or **feature-keyed small work** (a bug on a delivered feature, an extent-grow improvement): stable-ground triage, a delta card, dispatch to the re-keyed plan/implement. NOT new-capability work — that's \`/mochiko:specify\` |`
  - `/mochiko:plan` row's superseded clauses: `entry is a feature (FEAT-XXX) carrying a spec's Feature Selection or a feature-command delta card; … one run per feature, artifacts at \`.mochiko/features/FEAT-XXX/\``
  - `/mochiko:implement` row's superseded clauses: `entry keyed to the feature (FEAT-XXX, …) — the landing folds the feature's deltas into the product baselines (selection scope graduates the batch; delta scope folds the delta)`
  - `product-manager` row's superseded clause: `owns *which*: feature derivation from stories, the story filter (verdicts with reasons, never silent), map writes, selection advice;`
- **Kept deliberately:** every row not named above · inside the rewritten rows: the plan-the-plan proposal machinery, prototype/lockstep clauses, stress-test and acceptance clauses, the analyst-inside-PM-frame boundary, no-separate-feature-close-stage, deferred-SCs-ride-the-map — all carried verbatim or meaning-preserved into the new text.
- **Consumers assessed:** the router is itself the discoverability surface; rows verified against this wave's shipped files (charter `feature.md`, re-keyed `specify.md`/`plan.md`/`implement.md`, re-typed `authoring-feature-map`, new `patterns-map-minimalism` on disk — all audited PASS this wave).

## [v0.67.0] Architect rotation + proposal-gated plan — agent rows swapped, plan rows re-keyed
- **Disposition:** superseded → rewritten in place: the `principal-architect` agent row (cross-workflow governance author + feasibility reviewer → plan-cluster PRODUCER + altitude voice: architecture artifact, repo `ARCHITECTURE.md` fold, proposal contest brief; skills re-listed) · the `system-architect` agent row → the `tech-lead` row (cross-workflow: setup-cluster governance author + plan-cluster feasibility reviewer incl. hunt class 7 + interrogatory round) · the `review-plan-artifacts` row (fixed-checklist completeness → conformance-BLOCKING + rung-honesty-advisory + completeness within scope) · the `review-feasibility` row (+ hunt class 7 clause) · the `/mochiko:plan` entry row (fixed artifact enumeration → plan-the-plan proposal, proposed-artifacts-only, "dependency-ordered" dropped with the fixed set). The `patterns-plan-minimalism` skill row is a pure addition (rides the decision rows).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` rows 2026-08-13 architect-role restructure D1–D7 + 2026-08-12 plan-structure YAGNI D1–D7; records `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` + `.mochiko/brainstorms/plan-structure-yagni/record.md`)
- **Content (superseded, verbatim):**
  - `principal-architect` row: `| \`principal-architect\` | **cross-workflow** — setup-cluster author (authors/updates the governance surface set, greenfield + brownfield; runs codebase analysis) **and** plan-cluster **feasibility reviewer** (grades the analyst's plan artifacts for cross-artifact buildability; grades a different agent's work, never its own authoring) (skills: authoring-constitution, analysis-codebase, review-feasibility) |`
  - `system-architect` row: `| \`system-architect\` | plan-cluster PRODUCER — authors the design-time architecture artifact (\`architecture.md\`: container-level topology + current→target delta, qualifying-flow sequence diagrams, component register + \`D-XXX\`-linked delta summary); topology judgment (boundaries, sync/async, responsibility placement, buildability) upstream of entity/contract detail; never grades its own output (skills: patterns-system-design, patterns-technical-decisions) |`
  - `review-plan-artifacts` row's superseded head: `independently grading the producer's plan artifacts for **completeness** — coverage / measurability / presence / cross-artifact consistency over the analysis + design sets, the **architecture artifact** (component-table↔diagram coverage, qualifying-flow sequence coverage, data-model/contracts conform to the approved shape), and the **cycle cards** (vertical integrity, TEST-gate presence/grammar, story traceability, sizing, no pre-written task lists) → severity-classified gaps + 3-state`
  - `/mochiko:plan` row's superseded clauses: `analysis, the **architecture** delta (you sign it off on a rendered diagram before detailed design builds on it), detailed design, and the **cycle cards** (\`tasks.md\`) — independently graded for feasibility and completeness; one run per feature, dependency-ordered,`
- **Kept deliberately:** every other agent row · the plan-cluster skill rows not named above · review-feasibility's contradiction/architecture-pass body and 3-state verdict text · the cycle-card and grading vocabulary inside the surviving rows · the entry-point rows for the other five commands.
- **Consumers assessed:** the router is itself the discoverability surface; rows verified against the wave's shipped file set (agents dir: tech-lead in, system-architect gone; `patterns-plan-minimalism` on disk; re-keyed `plan.md`/`review-plan-artifacts`/`review-feasibility` all landed and audited this wave).

## [v0.61.0] Consumption note + entry rows re-keyed to the feature entry and the two-altitude design surface; `/mochiko:feature` row added
- **Disposition:** superseded → rewritten in place: the Specify-cluster consumption note (spec-root accumulation / extend-mode / Graduation-contract keying) re-keyed to the two-altitude surface (product baselines at `.mochiko/product/`, per-feature artifacts at `.mochiko/features/FEAT-XXX/`, folds at acceptance); the `/mochiko:specify`, `/mochiko:plan`, `/mochiko:implement` entry rows re-keyed (feature-keyed entry: spec selection or feature-command delta; specify scoped to new-capability work).
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/feature-sizing-and-entry-points/record.md` D9 — entry re-key, artifact re-home, two-altitude design surface; supersedes feature-map D17/D18's spec-folder layout, extend-mode-at-spec-root, and cross-spec reach clauses. Specify/feature boundary and the new row: D5/D6/D12.)
- **Content (superseded, verbatim):**
  - Consumption note: `> Feature-scoped consumption is carried by the spec's Feature Selection section + the map: \`/mochiko:plan\` and \`/mochiko:implement\` run per selected feature under the re-keyed **Graduation contract** (shared artifacts accumulate at spec root, extend-mode, per-feature trio under \`features/FEAT-XXX/\`). Deferred SCs, seams, and obligations ride the map entries (\`mochiko:authoring-feature-map\`); implement's acceptance landing executes the graduation bookkeeping — no separate feature-close stage exists.`
  - `/mochiko:specify` row's opening clause: `you want to create a feature specification —` (rest of the row kept verbatim; re-scoped to "spec **new-capability** work" per D6).
  - `/mochiko:plan` row: `you want to turn an accepted spec into an accepted implementation **package** — analysis, the **architecture** delta (you sign it off on a rendered diagram before detailed design builds on it), detailed design, and the **cycle cards** (\`tasks.md\`) — independently graded for feasibility and completeness; one run per selected feature (FEAT-XXX), dependency-ordered, shared artifacts extend-in-place at spec root; next step \`/mochiko:implement\``
  - `/mochiko:implement` row: `you want to turn accepted cycle cards into working, verified code — each card decomposed by its builder at build time, TDD-built, independently verified against real infrastructure with captured evidence, closing on your acceptance` (kept whole inside the rewritten row; the additions are the feature keying and the fold-at-landing clause).
- **Kept deliberately:** the deferred-SCs/obligations-ride-the-map sentence and the no-separate-feature-close-stage rule (still true, carried into the rewritten note) · every other row's text, including the specify row's intent-stage/stories/prototype/selection body · the graduation-bookkeeping-at-implement-landing claim (now stated as the baseline fold + scope-branched landing per D8/D15). The `/mochiko:feature` row is a pure addition (D5/D6/D12), no strip.
- **Consumers assessed:** the router is itself the discoverability surface; the re-keyed rows verified against this wave's command set (`feature.md` new, `plan.md`/`implement.md` re-keyed by the wave's other builders — row text matches the shared contracts, not files on disk mid-wave).

## [v0.58.0] authoring-slices row out; requirements-analyst row re-framed under the PM
- **Disposition:** superseded → the skill retired whole (`.mochiko/strips/authoring-slices.md` [v0.58.0]); its craft lives in `authoring-feature-map`, whose row already stands.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md`, D4/D18/D22 + D15)
- **Content (superseded, verbatim):**
  - Skills-table row: `| \`authoring-slices\` | authoring the spec's **Delivery Slices section** — graduation-slice decomposition (story→slice homes, dependency-closed order, foundation designation, extend obligations, Feature-Done, Graduation contract) or the single-slice line, keyed to the Intent section's delivery ruling; lead-dispatched, no fixed seat |`
  - Agents-table row (pre-edit): `| \`requirements-analyst\` | specify-cluster producer — authors the feature \`spec.md\` (prioritized user stories + FR/SC requirements) (skills: authoring-requirements, authoring-user-stories) |` — rewritten for the D10 story files and the D15 analyst-inside-PM-frame boundary.
- **Kept deliberately:** every other row this wave — the wave-2 seat's router ownership was these two rows only. Remaining slice-vocabulary rows (`analysis-iterative` agenda's "delivery/slicing", the Delivery-Slices consumption note, the `/mochiko:specify` + `/mochiko:plan` entry rows, `review-specifications` + `patterns-vertical-tdd` + `advocate-report-template` rows) are other seats' territory this build; a row surviving here out of step with its re-keyed primitive is a known in-flight state, converging by wave 4.
- **Consumers assessed:** the router is itself the discoverability surface; `authoring-feature-map` carries the retired row's duty.

## [v0.49.0] Router re-keyed — slice cluster + structuring table dissolved, 6→5 commands, 9→8 agents
- **Disposition:** superseded → Specify cluster (authoring-slices row + intent-stage analysis-iterative row + Delivery-Slices consumption note) and Plan cluster (patterns-vertical-tdd + tasks-template rows)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D1–D9)
- **Content:** the "Structuring — mapping & tasks" section (4 rows: patterns-vertical-tdd task framing · review-task-artifacts + boundary note · tasks-template TN.X skeleton · taskarchitect-report) · the "Slice cluster" section (4 rows + the slices.md-keyed consumption note) · the `/mochiko:slice` entry row · the `task-architect` agent row · task-mapping/slices mentions in the artifact-format chain and devils-advocate roster. Full text: git history at v0.48.0.
- **Kept deliberately:** the feature-close/audit deferral note (re-homed under the Specify cluster consumption note) · every surviving skill still carries a row (undiscoverable-by-construction rule held).
- **Consumers assessed:** the router is itself the discoverability surface; all re-pointed rows verified against the post-wave file set.

## [v0.46.0] Doctrine rows out of the router
- **Disposition:** superseded → commands self-contained; `workflow-contract` + `agent-dispatch` rows survive
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "Every workflow is a **sound loop** (see `loop-discipline`)" → each command owns and states its own loop; the composition paragraph's "Doctrine consumed by every workflow: `loop-discipline` (the four sound-loop rules)..." and "(`command-shape`)" attribution rewritten; the doctrine table's `loop-discipline` row ("designing/reviewing any workflow or agent loop; deciding if a loop is sound; filling a `workflow-contract`") and `command-shape` row ("the codified command pattern's **sole authoritative home**... obligated-read by conformant commands") deleted.
- **Kept deliberately:** the review-family split, all cluster sections, the operating rules (producer↔validator boundary, lead-is-the-command, unbroken-round) — the doctrine's behavioral content, still true.
- **Consumers assessed:** router is user-invoked; no downstream skill reads it.

## [v0.45.0] Framework-maintenance rows deleted — the trio purge
- **Disposition:** superseded → deleted (the primitives they indexed left the plugin)
- **Tier failed:** n/a — supersession by ruling (user ruling 2026-08-02; ADR
  `.mochiko/decisions/2026-08-02-framework-trio-deleted.md`)
- **Content:** the `### Framework maintenance` section whole (heading + table +
  `authoring-commands` and `validation-command-shape` rows) · the `command-architect` row in
  the Agents table · `:28`'s family example trimmed ("today: `validation-constitution` and
  `validation-command-shape`" → `validation-constitution` only) · the `validator` row's
  skills list drops `validation-command-shape`.
- **Kept deliberately:** the `validation-*`/`review-*` split doctrine paragraph itself — the
  family survives with one member; and the "Adding to the library" discoverability rule,
  which is what obliged these deletions (an unindexed primitive is undiscoverable, and an
  indexed non-primitive is a defect).
- **Consumers assessed:** router-only edit; the six command entries and remaining 27 skill
  rows untouched.

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Review-family split's design-record citation
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
— design:
`.mochiko/brainstorms/setup-adversarial-review/record.md` D5
```
- **Kept deliberately:** the split's date (2026-07-18) and the whole operative rule — the prefix encodes who owns the clearing.

## [v0.40.0] The router's contract claim corrected — a command supervisor is its own contract
- **Disposition:** superseded → rewritten in place at **two** sites: the *How the library
  composes* sentence and the `workflow-contract` reach-when cell. A **third** correction rode the
  same edit and is logged in its own entry below.
- **Tier failed:** n/a — supersession by ruling (`lead-owned-process-flexibility` **OQ-2**,
  adopted at acceptance **A2**, 2026-08-01; `DECISIONS.md` 2026-08-01 row). Wave note:
  `.mochiko/strips/command-shape.md` [v0.40.0]. The router was **not** one of the wave's five
  briefed build targets; it is a consumer the wave made stale, repaired in the same wave on the
  [v0.38.0] precedent (`authoring-commands`' layer enumeration, corrected rather than deferred),
  and flagged to the wave lead so the audit grades it rather than discovering it.
- **Content (verbatim, both sites):** "Each workflow below is a sound loop — a command supervisor
  stitches a producer/validator agent team to a goal **under a workflow-contract**." · the
  reach-when cell "`workflow-contract` (template) | **instantiating the contract for a specific
  workflow**".
- **Kept deliberately:** the doctrine-pair framing itself (every workflow consumes
  `loop-discipline` plus a carrier), the sound-loop claim, and the `loop-discipline` reach-when
  row — which reads "filling a `workflow-contract`" and stays true, that being one of the two
  cases the revived form is filled for.
- **Consumers assessed:** the router is the index — nothing greps its row text, and no primitive
  reads it as a source. The three corrected strings appear nowhere else (grep-verified).

## [v0.40.0] The router's Layer-2 enumeration corrected — a v0.38.0 residue
- **Disposition:** corrected in place (correction class).
- **Cause:** the `command-shape` reach-when cell enumerated the layers as "Layer 1 form-agnostic
  core · **Layer 2 team transport**", which shape **v6** made stale when Layer 2 was re-framed
  into two axes (`.mochiko/strips/command-shape.md` [v0.38.0], "Layer 2's heading and internal
  order rewritten"). That wave found and fixed the identical staleness in `authoring-commands`
  and recorded the router as checked — the check covered the `command-architect` agent
  description and missed this cell.
- **Content:** "(Layer 1 form-agnostic core · Layer 2 team transport)" → "(Layer 1 form-agnostic
  core, including the non-waivable floor · Layer 2 team transport **and per-seat context
  lifecycle**)".
- **Why it is logged separately from the entry above:** it is a **pre-existing** defect this wave
  merely found, not a v7 consequence. Folding the two would let the wave claim a ruling required
  an edit it did not, and would hide that the v0.38.0 consumer sweep had a hole in it — which is
  the more useful fact for the next auditor.

## [v0.25.0] Thirteen fat index rows compressed to ≤3 routing lines each (~5.1 KB)
- **Disposition:** relocated → each indexed primitive's own `description:` / SKILL.md (the declared single source — the R4b item-1 precedent, applied to the index): rows for authoring-constitution, validation-constitution, review-governance-intent, testing-governance-injection, review-brainstorm, authoring-slices, review-slices, slices-template, `/mochiko:setup`, `/mochiko:brainstorm`, the devils-advocate + command-architect agent rows, and authoring-commands
- **Tier failed:** 1 (each row restated its primitive's description near-verbatim — protocol mechanics, field enumerations, lens-split detail; routing needs the trigger + the boundary, both kept in every compressed row)
- **Content:** the restated protocol/enumeration prose per row; preserved per row — what it is, when to reach it, the boundary/never clauses, skill mounts, and the teams-required refusals
- **Consumers assessed:** the router is the index — nothing greps its row text; every compressed row's primitive verified to carry the dropped detail in its own file

## [v0.25.0] Flow-graph ASCII box → two prose lines (heading retitled "How the library composes")
- **Disposition:** deleted; its two facts (doctrine pair consumed by every workflow; each workflow is a sound loop under a contract) merged into prose
- **Tier failed:** 2 (the box named no behavior beyond the surviving prose)
- **Content:** the doctrine box + sound-loop caption
- **Consumers assessed:** none reference the box or the old heading

## [v0.25.0] KEPT: the two-families paragraph, operating rules, and the registration rule
- **Tier-2 evidence:** contested at the pass and kept — the `validation-*`/`review-*` prefix
  doctrine encodes who owns clearing (a routing fact used at every dispatch); the operating rules
  name the producer↔validator boundary and context-hygiene failures; "a primitive not in this
  router is, by construction, undiscoverable" is the library's discoverability axis stated as its
  failure mode. Session ruling: batch-3 ratification 2026-07-25.
