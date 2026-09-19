# Action Plan — `/mochiko:setup add a principle covering how long we keep customer data, and tighten the pre-release checks`

*(Plan-only. Nothing below was executed beyond the reads already performed to ground the plan: `plugins/mochiko/schemas/setup.yaml`, `common.yaml`, `command-labels.yaml`, and the current workspace state.)*

## Phase 0 — Load the binding schema (already done)
- **Read:** `plugins/mochiko/schemas/setup.yaml` (raw, full) and `plugins/mochiko/schemas/common.yaml` (raw, full) — the first-class, no-build-step source of the run's rules. `command-labels.yaml` read for label vocabulary.
- **Check:** the schema's `kind: fail` count is 6 (`setup.fail.pre-ratification-authoring`, `.unclosed-trace`, `.author-graded`, `.floor-category-uncovered`, `common.no-acceptance` stub, `.no-feature-map`) — matches, so the run is not halted for a schema/`.md` desync.
- **Output:** none written.

## Phase 1 — Read workspace state, propose mode
- **Read:** `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `CLAUDE.md`, `.claude/rules/mochiko/money-handling.md`, `.claude/rules/mochiko/output-style.md`, `.mochiko/memory/constitution.md`, `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/product/architecture/spine.md`.
- **Findings that drive the plan:** a ratified surface set already exists at v1.2.0 (GI-001…GI-007, production floor, depth `high`, modules: none), a feature map already exists (`FEATURES.md` + `FEAT-001`), and the architecture store's `spine.md` already carries `Scope: backend-service`. A stray `.mochiko/memory/constitution.md` (v0.9, pre-dates the region/rules-file split) is present on disk.
- **Propose mode:** **amend** — the workspace already shows a ratified set, so this isn't the ambiguous case, but the mode is still stated plainly for confirmation rather than silently assumed (`setup.user-mode-ruling`).
  - **Gate (user):** "Running this as an amend to the existing v1.2.0 governance set — confirm, or tell me if you intended greenfield/brownfield instead." Branches: *confirmed* → continue as amend; *user says brownfield/greenfield instead* → re-scope Phase 2 onward to that mode's fuller interrogation depth (full analysis-codebase run for brownfield, full ten-dimension session for greenfield) before proceeding.
- **Action (no gate needed, per floor rule):** flag `.mochiko/memory/constitution.md` as superseded and delete it on sight, stated in one line to the user (`setup.constitution-superseded`). **Write:** delete `.mochiko/memory/constitution.md`.

## Phase 2 — Scope the amend micro-session
- Determined from Phase 1: this amend touches two things — (a) a new data-retention principle, (b) tightening pre-release checks, which the schema's own agenda maps to the **release-gates module**, currently unattached (`Modules: none`). A module attach is a governance event, so this is not a wording-only amend — it needs the relevant agenda slice, not a full ten-dimension re-run.
- **Scope:** dimension 2 (fact profile — data classes/retention), dimension 4 lightly (risk surface backing the retention duration and release-blocking severity), dimension 8 (deployment & release reality — release-gates module offer), dimension 9 (values/non-negotiables — enforceable retention rule, custom release-blocking checks), dimension 10 lightly (confirm nothing here is a deliberate exclusion).
- **Once-only module sweep:** since this is an amend, also offer, once, any module the synthesis records no ruling on yet (release-gates qualifies as unruled) — recorded either way.
- No action on the feature map or architecture-store scope this phase — both already present, and `setup.map-never-overwrite` / `setup.fail.no-feature-map` aren't triggered (amend + map exists).

## Phase 3 — Interrogation (inline, by the lead)
- **Duty:** run this myself, inline, via `mochiko:analysis-iterative`, one question per turn, adaptive, recommend-then-arbitrate — never delegated to a subagent (`setup.interrogation-inline`).
- **Fact-finding dispatch:** any locate/enumerate read needed mid-session (e.g. checking whether a CI config, deploy script, or existing data-deletion job already exists) is dispatched to a native `Explore` subagent spawned with `model: haiku`; interpretive reads (what the fact profile means, how to phrase a principle) stay on my own tier (`mochiko:patterns-model-tiering`).
- **Card/question flow:**
  1. Data classes (dimension 2 revisit): confirm which data classes the retention rule covers — customer contact details, invoice line items, bank identifiers already on file — and whether EU/UK jurisdiction implies a statutory minimum/maximum (e.g. tax-record retention, GDPR storage-limitation) that bounds the number the user picks.
  2. Risk surface (dimension 4, light): what a too-long or too-short retention window actually costs here (compliance exposure vs. losing dispute evidence).
  3. Values (dimension 9): the user's concrete retention period and what enforces it (a scheduled deletion/anonymization job, a manual quarterly purge, etc.) — phrased as enforceable behavior, not a mood.
  4. Deployment & release reality (dimension 8): current environments, release cadence, and what today actually blocks a release (informal/none, per the old superseded Article III). Offer the `release-gates` module (`templates/constitution-modules/release-gates.md`), recommend-then-arbitrate — the user keeps/tightens/drops the offered gate rows (staging soak, migration reversibility, changelog entry) and supplies real verification commands/dashboards plus rollback expectations.
  5. Deliberate exclusions (dimension 10, light): confirm nothing about retention or release is meant to stay unenforced.
- **Waiver rulings:** record any deviation the user chooses (e.g. an exception for legacy records already past a proposed cutoff) with justification and an optional revisit trigger.
- **User-reserved rulings throughout:** every card ruling and waiver is the user's (`setup.user-card-rulings`) — I recommend, the user arbitrates.
- **Output (not yet a governed surface):** a draft update to `.mochiko/memory/governance-intent.md` — new `GI-008` (customer data retention) and `GI-009` (release gates / tightened pre-release checks), plus an updated fact-profile note and a new amendment-log row. This is the synthesis, not one of the authored surfaces, so drafting it does not trip the pre-ratification-authoring fail condition.

## Phase 4 — Pre-ratification stress test (cold seat)
- **Duty:** mandatory unless the user records a waiver (`setup.stress-test-cold-seat`).
- **Gate (user, implicit at this step):** "Run the standard cold stress-test on the draft synthesis before you ratify it, or waive it?" Branches: *run it* → proceed below; *user waives* → skip straight to Phase 5, with the waiver recorded in the trace summary.
- **Dispatch (two messages, blind — `setup.blind-map-dispatch`, floor):**
  1. First message to a spawned agent (`mochiko:devils-advocate`, running the `mochiko:review-governance-intent` skill): the setup topic and project identity/goal only — "Peartree, an invoicing product for freelancers; amending governance to add a customer-data-retention principle and tighten pre-release checks" — **never** the synthesis file path. It returns its Phase 0 blind angle map first.
  2. Second message, only after the map returns: the synthesis path `.mochiko/memory/governance-intent.md`. The cold read runs from there — six hunt classes, cross-examination — returning severity-classified findings and a verdict.
- Because two seats are now in play (me + the cold reviewer), the multi-seat transport floor applies (`setup.transport-floor`, `mochiko:patterns-transport-floor`) — message legs on this dispatch, non-waivable once triggered, referenced not restated here.
- **Coverage-survivor routing (`setup.coverage-survivor-routing`):** any finding that survives is presented as a candidate topic, not silently folded in.
  - **Gate (user):** for each surviving finding — explore now (re-enter `analysis-iterative` on that angle, landing in the `GI-XXX` namespace), rule inline, or defer. Branches determine whether Phase 3 reopens narrowly before ratification.

## Phase 5 — Ratification gate
- **Gate (user, floor — `setup.gate-synthesis-ratification`):** present the finished synthesis draft (GI-008, GI-009, updated amendment log) as plain blocking text — confirm / edit / reject.
  - **Confirmed as-is** → proceed to Phase 6.
  - **Confirmed with edits** → apply the edits to the synthesis file, then proceed.
  - **Rejected** → return to Phase 3 for rework; no governed surface is authored in the meantime (this is exactly what `setup.fail.pre-ratification-authoring` exists to prevent).
- **Write (only on confirm/edit):** finalize `.mochiko/memory/governance-intent.md` with the ratified GI-008/GI-009 entries.

## Phase 6 — Author the surface set
- **Skill/pointer:** `mochiko:authoring-constitution` owns this procedure — referenced, not restated.
- **Plan-approval gate (user, `setup.plan-approval-producers`):** before any write, state the concrete plan — which files change and how — for approval:
  - `CLAUDE.md` governance region: add a retention principle line (home TBD — likely the region itself, alongside GI-003, since it's short and cross-cutting) and either a new **Release Gates** section (sourced from `templates/constitution-modules/release-gates.md`, populated with the ratified environments/cadence/gate table/rollback) or a tightened Quality Gates line, per what Phase 3 actually ruled; bump the `Ratified:` version stamp and `Modules:` list.
  - `.mochiko/memory/governance-ledger.md`: add Three-Part metadata (Enforcement/Testability/Rationale) for GI-008 and GI-009, a new amendment-log row, update `Modules:` and the version stamp to match the region.
  - Possibly a new paths-scoped rules file (e.g. `.claude/rules/mochiko/data-retention.md`), mirroring `money-handling.md`'s pattern, if enforcement needs to scope to specific code paths rather than living in the region.
  - Carve-outs preserved verbatim: the `mochiko:output-style` block already inside the region, untouched.
  - Semver: MINOR for the new principle alone; **MAJOR** if `release-gates` is newly attached (per the ledger's own amendment policy: "module attach or detach" = MAJOR) — the two may combine into one MAJOR bump for this amend.
  - **Gate (user):** approve this plan before writing. Branches: *approved* → write below; *user wants changes* → adjust the plan and re-present.
- **Write:** `CLAUDE.md` (region only, idempotent regenerate), `.mochiko/memory/governance-ledger.md`, and (if warranted) a new `.claude/rules/mochiko/*.md` file.
- **Floor re-check:** confirm all four Essential Floor categories (Security, Testing, Error Handling, Observability) still carry a principle or recorded waiver post-edit — neither of this amend's asks touches them, so this is a presence check, not new authoring.

## Phase 7 — Independent grading (non-author, default-FAIL)
- **Seat:** spawn `mochiko:validator` running the `mochiko:validation-constitution` skill — reads the actual files (region, rules file(s), ledger) directly, never my authoring report; defaults to FAIL (`setup.author-grader-default-fail`, `setup.fail.author-graded`).
- Optionally, `mochiko:testing-governance-injection` for an empirical delivery probe (does the new rules-file path actually inject on its declared `paths:`, does the region reach spawned agents) — available as a follow-on evidence step given this amend touches injected surfaces.
- **Branch:** PASS → proceed to Phase 8. FAIL → findings route back to Phase 6 for revision, re-graded until PASS (loop, not a one-shot).

## Phase 8 — Trace summary
- Assemble the closed trace: ratified GI-008/GI-009 → their authored locations (region line, ledger metadata, rules file if any) → the grading verdict. This closes `setup.fail.unclosed-trace`.
- **Output:** trace summary presented as plain text (no file beyond what Phase 6 already wrote).

## Phase 9 — Final acceptance gate
- **Gate (user, floor — `setup.gate-final-acceptance`):** present the trace summary and every changed surface, flagged proposal by flagged proposal, plain blocking text.
  - **Accept all** → close the run; version stamps in the region and ledger stand as final; suggest (never run) a git commit covering the changed files (`setup.no-git-mutations`).
  - **Accept some / reject some** → rejected items route back to Phase 6 (or Phase 3 if the rejection reveals mis-elicited intent), reworked, re-graded, re-presented — run stays open.
  - **Defer** → deferred items noted in the ledger's amendment log as open; run does not close as done.

## Phase 10 — Close-condition check
- Feature map: already present (`FEATURES.md`, `FEAT-001`) — no scaffold needed, satisfies the close condition for an amend.
- Architecture store: `spine.md` already carries `Scope: backend-service` — write-if-absent rule means no write here.
- Verify all 6 `kind: fail` conditions are cleared: no pre-ratification authoring (Phases 3–5 sequencing), trace closed (Phase 8), grading was non-author (Phase 7), floor categories covered (Phase 6 check), acceptance given (Phase 9), feature map present (this phase).

## Phase 11 — Report
- Register: `templates/output-style.md`.
- Report what changed (GI-008 retention principle, GI-009/release-gates tightening, version bump, deleted stray `constitution.md`) and next steps: `/mochiko:specify` for the next feature and `/mochiko:architecture` for the product's architecture baseline — peer doors, neither ahead of the other. (Knowledge-management is not adopted in this project, so the `/mochiko:brainstorm` addition to the next-step line does not apply.)