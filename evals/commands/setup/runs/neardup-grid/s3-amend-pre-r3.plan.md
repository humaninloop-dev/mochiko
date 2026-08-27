# Action Plan — `/mochiko:setup add a principle covering how long we keep customer data, and tighten the pre-release checks`
*(plan-only; nothing below is executed)*

## Phase 0 — Load the schema (already performed as first action)
- **Read:** `plugins/mochiko/schemas/setup.yaml` (raw, full) and `plugins/mochiko/schemas/common.yaml` (raw, full) — done above.
- **Cross-check:** count of `kind: fail` rules in `setup.sec.fail-conditions` = 6 (`pre-ratification-authoring`, `unclosed-trace`, `author-graded`, `floor-category-uncovered`, `no-acceptance`, `no-feature-map`) — matches the command doc's hard-coded count, so no halt is needed.
- **Writes:** none.
- **Seats:** none — lead-only.

## Phase 1 — Workspace read & mode proposal
- **Read:** `CLAUDE.md`, `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, `.claude/rules/mochiko/money-handling.md`, `.claude/rules/mochiko/output-style.md`, `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/product/architecture/spine.md`.
- **Findings that shape the proposal:**
  - Governance is already ratified at v1.2.0 (GI-001…GI-007), with a live ledger, two landed principles files, and an amendment policy — this is not a bare workspace.
  - `.mochiko/memory/constitution.md` (v0.9, pre-region) is on disk and superseded (`setup.constitution-superseded`) — flagged for deletion, one line, not a discussion point.
  - `FEATURES.md` + `FEAT-001` already exist, so the feature map is present, not missing.
  - `.mochiko/product/architecture/spine.md` exists with only a `Scope: backend-service` header (no ruled topology) and its sibling `concerns.md` is absent.
- **Proposal to the user:** mode = **amend**, since a ratified surface set already governs the project and the request is additive/tightening, not a from-scratch or codify-existing pass.
- **Gate (`setup.user-mode-ruling`, reserved to the user):** present the evidence above and ask the user to confirm or correct the mode.
  - *If confirmed amend* → proceed to Phase 2 on the amend path.
  - *If the user insists on brownfield* (e.g. they want a full re-codification against the current code, not just these two asks) → re-scope: Phase 2 instead runs a full `mochiko:analysis-codebase` pass into `.mochiko/memory/codebase-analysis.md`, and the two requested topics become two cards inside that larger interrogation.
  - *If the user says something else entirely* (e.g. they actually want a brand-new project) → stop and re-derive the goal with them before continuing; the two-line request doesn't fit greenfield, so this branch is treated as a request for clarification, not silently honored.

## Phase 2 — Inline interrogation on the two requested topics
- **Run inline** (`setup.interrogation-inline`) via `mochiko:analysis-iterative`, adaptive one-question-at-a-time, then card-by-card recommend-then-arbitrate against the catalog deck.
- **Read:** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, its `catalog/` deck, `DOMAIN-DEPENDENCIES.md` — scoped to the cards touching data retention and release/quality-gate checks; the existing GI-001 fact profile (industry: professional-services billing; data classes: contact details, invoice line items, bank account identifiers; jurisdictions: EU/UK) and GI-007 (current quality gates: test suite green, coverage ≥70%) as the amend baseline.
- **Fact-finding dispatch:** any locate/enumerate read (e.g. does the repo have an existing CI workflow, lint config, or release script to ground "tighten pre-release checks") is routed to a native `Explore` subagent spawned with `model: haiku` (`setup.model-tiering`); interpretive reads (e.g. does GDPR retention guidance apply given GI-001's EU/UK jurisdiction) stay on the session tier.
- **Card topics worked with the user:**
  1. *Customer data retention* — how long each data class (contact details, invoice line items, bank account identifiers) is kept, whether a deletion/anonymization trigger exists (account closure, statutory minimum for financial records), and whether this is a new principle or an amendment to GI-003 (containment ≠ retention). Also a mechanical check: does this trip a module in the catalog deck (e.g. a data-protection/regulatory module), given `Modules: none` today.
  2. *Pre-release checks* — what "tighten" means concretely against current GI-007 (raise the 70% coverage floor, add a lint/security-audit gate, add a staging smoke test, require a second approver, etc.), and whether the existing "Load and performance testing" waiver in the ledger is affected.
- **Gate (`setup.user-card-rulings`, reserved to the user):** every card ruling and any module ruling is the user's — recorded, not inferred.
- **Writes:** none yet — this phase only produces the material for the draft synthesis.

## Phase 3 — Draft the updated synthesis
- **Write (draft, not yet ratified):** `.mochiko/memory/governance-intent.md` — append new `GI-00N — Customer data retention` principle(s) (with `Mark:` and a proposed home: CLAUDE.md region bullet vs. a dedicated path-scoped rules file, decided by the card ruling above), amend the `GI-007 — Quality gates` entry text to the tightened form, and add a pending amendment-log row with a proposed version bump (MINOR for a new principle per the ledger's own policy; the quality-gates tightening is MINOR or PATCH depending on whether the user's ruling redefines or just clarifies it).
- **Seats:** none — I author this draft directly, since interrogation and its synthesis are mine to hold the pen on.

## Phase 4 — Pre-ratification cold stress-test
- **Gate/branch first:** offer the user the option to waive this seat (`setup.stress-test-cold-seat` allows "the user's recorded waiver"). 
  - *If waived* → skip to Phase 6, with the waiver recorded in the run's report.
  - *If not waived (default)* → proceed below.
- **Seat:** `mochiko:devils-advocate`, running `mochiko:review-governance-intent`, dispatched in two messages (`setup.blind-map-dispatch`, floor):
  1. First message: setup topic ("add a customer-data-retention principle; tighten pre-release checks") + project identity (Peartree, invoicing for freelancers, EU/UK, two maintainers) + the goal — **no path to the synthesis**. The seat builds its Phase 0 angle map blind.
  2. Second message, sent only after the angle map returns: the path to the draft `.mochiko/memory/governance-intent.md` for the cold read, six-hunt-class pass, and cross-examination.
- **Read (by the seat):** the draft synthesis file; nothing else pre-shared.
- **Writes:** none — findings are returned as structured output, not filed.
- **Gate/branch (`setup.coverage-survivor-routing`, reserved to the user):** a coverage finding that survives (e.g. "retention wasn't addressed for backups/logs, only the primary DB" or "the tightened checks don't cover the deploy/rollback path") is presented as a candidate topic, not silently folded in:
  - *Explore now* → re-enter Phase 2's `mochiko:analysis-iterative` on that angle; the re-elicited intent lands in the next `GI-XXX` slot; redraft (Phase 3) and re-stress-test the delta before re-presenting.
  - *Rule inline* → user answers directly; fold into the draft without a full interrogation cycle.
  - *Defer* → note as a known gap in the run's report; does not block this run's ratification.
  - Non-coverage findings (contradictions, testability gaps, etc.) are fixed in the draft directly and re-checked, not routed to the user as scope questions.

## Phase 5 — Ratification gate
- **Present:** the final draft synthesis text (new principle wording + home, tightened GI-007 text, proposed version bump) as **plain blocking text, never a timed prompt** (`setup.acceptance-plain-text`, floor).
- **Gate (`setup.gate-synthesis-ratification`, floor):**
  - *Ratified as-is* → stamp `.mochiko/memory/governance-intent.md`'s "Confirmed at synthesis checkpoint" line and finalize its amendment-log row; proceed to Phase 6.
  - *User requests edits* → revise the draft (back to Phase 3), and re-run Phase 4's stress test if the edit is material; re-present.
  - *User declines to ratify* → run halts here. Nothing downstream is authored (this is exactly what `setup.fail.pre-ratification-authoring` exists to prevent) — the run ends Not-done, and I say so plainly rather than pushing forward.

## Phase 6 — Author the surface set
- **Plan-approval gate (`setup.plan-approval-producers`):** before any write, the authoring seat's plan is presented to the user for approval.
- **Seat:** `mochiko:tech-lead`, running `mochiko:authoring-constitution`, working only from the now-ratified `.mochiko/memory/governance-intent.md`.
- **Writes (once the plan is approved):**
  - `.mochiko/memory/constitution.md` — **deleted** (superseded, `setup.constitution-superseded`), noted in one line to the user.
  - `CLAUDE.md` — governance region only (between `<!-- mochiko:governance:begin/end -->`): add the new retention principle bullet, amend the `### Quality gates` section for the tightened checks, bump the version stamp. The `mochiko:output-style` carve-out (lines currently in the region) is preserved verbatim (`setup.carve-outs-preserved`, floor); there's no `mochiko:domain-registry` block present today, so none to preserve. Everything outside the markers (the "Notes for contributors" section) is left untouched (`setup.governance-region-ownership`, floor).
  - `.claude/rules/mochiko/` — either a new path-scoped file (e.g. `data-retention.md`, if the card ruling gave it code-path-specific enforcement mechanics, mirroring how `money-handling.md` works) or just a CLAUDE.md-region bullet (mirroring GI-003/GI-004), per the Phase 2 ruling.
  - `.mochiko/memory/governance-ledger.md` — add the Three-Part metadata block (Enforcement / Testability / Rationale / Trace) for the new retention principle, amend GI-007's existing block for the tightened checks, update the version stamp to match CLAUDE.md, add the amendment-log row, and revisit the "Load and performance testing" waiver row if the tightened checks touch it.
  - `.mochiko/product/architecture/concerns.md` — scaffolded empty (it's currently missing; `setup.store-scaffold-unconditional`, unconditional on every path, creates only what's missing). `spine.md`'s `Scope:` line already exists, so it is left alone (write-if-absent).
  - Feature map: **no write** — `FEATURES.md`/`FEAT-001` already exist, so there's nothing missing to surface or offer (`setup.map-never-overwrite`, floor: "an amend run makes none").
  - Product baselines (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`): out of scope for this amend — left unseeded, per the standing deferral to `/mochiko:architecture` or the first `/mochiko:implement` design phase.

## Phase 7 — Independent grade
- **Seat:** `mochiko:validator` (never the tech-lead who authored Phase 6), running `mochiko:validation-constitution` — reads the actual files (`CLAUDE.md`, the rules file(s), the ledger), never the tech-lead's report (`setup.author-grader-default-fail`, floor).
- **Read:** the authored `CLAUDE.md` region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, and the ratified `.mochiko/memory/governance-intent.md` as the trace source.
- **Checks:** every authored line traces to a ratified GI-ID (closing `setup.fail.unclosed-trace`); every Essential Floor category still has either a principle or a recorded waiver (`setup.fail.floor-category-uncovered`); version stamps match between CLAUDE.md and the ledger; carve-outs untouched.
- **Writes:** none — findings only.
- **Branch:** *PASS* → Phase 8. *FAIL* → fix list goes back to the Phase 6 seat, re-authored, re-graded — looping until PASS (default-FAIL discipline, no shortcut).

## Phase 8 — Final acceptance
- **Present:** the finished set, flagged proposal by flagged proposal (new retention principle, tightened quality gates, version bump, the constitution.md deletion) — plain blocking text, never timed.
- **Gate (`setup.gate-final-acceptance`, floor):**
  - *Accept all* → close the run.
  - *Accept some, reject/adjust others* → iterate only on the rejected item(s) (back to Phase 6 or even Phase 2 if the rejection reopens a card), then re-present just the changed piece(s).
  - *Reject entirely* → run ends Not-done (`setup.fail.no-acceptance`); no further action, offered to resume later.

## Phase 9 — Close-out
- **Report** (per `setup.register`/output-style carve-out already in CLAUDE.md: lite conversation, ultra reports, full documents): trace summary (GI-ID → surface line), version bump confirmation, the constitution.md deletion note, and any deferred coverage-finding gaps from Phase 4.
- **Git:** suggest a commit message covering the changed/deleted files; never run git mutations or push (`setup.no-git-mutations`, floor).
- **Next-step routing** (advisory): `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline, offered as peer doors.