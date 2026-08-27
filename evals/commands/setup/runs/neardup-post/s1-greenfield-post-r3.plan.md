# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Entry state observed** (read, not written): `$ARGUMENTS` is empty. Workspace root holds only `README.md`, `CLAUDE.md`, and the `plugins/` tree — no `.mochiko/`, no `.claude/`, no git repo, no source/config. `README.md` states "Nothing is built yet." Product: **Cadence**, an async-standup digest web app, two part-time engineers, six-person pilot team, non-technical audience. `CLAUDE.md` carries no `<!-- mochiko:governance:begin/end -->` region yet.

Governing schemas already read in full per the run's own first-action duty: `plugins/mochiko/schemas/setup.yaml`, `plugins/mochiko/schemas/common.yaml`, `plugins/mochiko/schemas/command-labels.yaml`. `kind: fail` count in `setup.sec.fail-conditions` = 6, matching the Not-done set — no halt needed.

---

## Phase 0 — Schema load (done)

- **Read:** `setup.yaml`, `common.yaml`, `command-labels.yaml` raw, in full.
- **Write:** none.
- **Seats:** lead only.
- **Gate:** none (a mismatch between the schema's `kind: fail` count and 6 would halt here; none found).

## Phase 1 — Mode proposal

- **Done:** Propose mode from workspace evidence — no code, no `.mochiko/`, no existing governance region, README explicitly says nothing is built → **greenfield**.
- **Read:** root directory listing, `README.md`, `CLAUDE.md` (already done above).
- **Write:** none yet.
- **Gate (`setup.user-mode-ruling`, reserved to the user):** present "greenfield" as the proposed mode and ask the user to confirm or override.
  - *Ruling: confirmed* → proceed as greenfield through the rest of this plan.
  - *Ruling: overridden to brownfield* → re-derive via `mochiko:analysis-codebase` into `.mochiko/memory/codebase-analysis.md` before interrogation; feature-map phase switches to reconstruction (`setup.feature-map-brownfield`) and conflict/map-confirmation gates (`setup.user-conflict-rulings`, `setup.user-map-confirmation`) activate.
  - *Ruling: overridden to amend* → skip interrogation-from-scratch, load the existing `governance-intent.md` as the amend baseline (`setup.synthesis-artifact`), and skip feature-map writes entirely (`setup.map-never-overwrite` notes an amend run makes none) unless a missing map is surfaced and offered rather than scaffolded.
  - This plan continues on the **greenfield** branch below.

## Phase 2 — Inline interrogation

- **Done:** Lead runs the interrogation itself, inline (`setup.interrogation-inline`) — the ten-dimension agenda worked adaptively via `mochiko:analysis-iterative`, one question per turn, skipping what's already known from README/CLAUDE.md (project identity, audience, team size are partly pre-answered), then the catalog deck card by card, recommend-then-arbitrate.
- **Read:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, `.../catalog/` deck (`universal-floor.md`, `backend-service.md`, …), `.../DOMAIN-DEPENDENCIES.md`.
- **Write:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace), built incrementally as the session progresses, shaped by `mochiko-cli template governance-intent` or `plugins/mochiko/schemas/governance-intent.yaml` read raw if the binary is absent.
- **Seats:** lead only, no subagent.
- **Gate (`setup.user-card-rulings`, reserved to the user):** every dimension answer, catalog-card ruling, module adoption (e.g. knowledge-management, release-gates), and waiver is the user's call, not inferred.
  - *Branch:* if the user's answers surface a gap outside the current agenda dimension, route per `setup.coverage-survivor-routing` — present it as a candidate topic; user rules explore-now (re-enter `mochiko:analysis-iterative`, land under a new GI-XXX entry) / rule-inline / defer. This plan proceeds assuming no such gap surfaces mid-session; if one does, this branch is taken before continuing.

## Phase 3 — Pre-ratification stress test (cold seat)

- **Done:** Spawn an independent cold seat to stress-test the frozen synthesis before ratification (`setup.stress-test-cold-seat`), via the mandatory two-message blind dispatch (`setup.blind-map-dispatch`, non-waivable floor):
  1. First message to the seat: setup topic + Cadence's project identity and goal only — **never** the path to `governance-intent.md`. The seat builds a Phase 0 angle map blind to what interrogation concluded.
  2. Only after that map returns, send the synthesis path and let the seat run its cold read using `mochiko:review-governance-intent`.
- **Seats:** one Agent call, `subagent_type: mochiko:devils-advocate` (or `mochiko:validator`), instructed to invoke the `mochiko:review-governance-intent` skill — staffing count is lead's latitude (`setup.staffing-latitude`); a pair is also legal, each building its map independently.
- **Read (by the spawned seat):** `.mochiko/memory/governance-intent.md` (second message only).
- **Write:** none from this seat — it returns findings, doesn't author.
- **Transport:** this is now a multi-seat run, so `setup.transport-floor` (`mochiko:patterns-transport-floor`) activates — governs how the two dispatch messages and any fan-in are composed.
- **Gate:** `setup.stress-test-cold-seat` allows a user-recorded waiver instead of running this seat at all.
  - *Ruling: run it* → proceed as above.
  - *Ruling: waive* → record the waiver plainly, skip to Phase 4 with no cold-read findings.
- **Gate (`setup.coverage-survivor-routing`, reserved to the user):** any coverage finding that survives the cold read questions the setup's *scope*, not a card — present each surviving gap as a candidate topic.
  - *Ruling: explore now* → re-enter `mochiko:analysis-iterative` on that angle, fold the result into a new GI-XXX entry, re-run the affected slice of Phase 2/3.
  - *Ruling: rule inline* → user decides the gap directly, recorded in the synthesis.
  - *Ruling: defer* → noted, not resolved this run.

## Phase 4 — Ratification gate

- **Gate (`setup.gate-synthesis-ratification`, floor, non-waivable):** the user ratifies the synthesis. **No surface may be authored before this ruling** — authoring first is fail condition `setup.fail.pre-ratification-authoring`.
  - *Ruling: ratified as-is* → proceed to Phase 5.
  - *Ruling: changes requested* → loop back into Phase 2/3 on the specific dimensions, re-ratify before continuing.
  - *Ruling: declined outright* → run halts here; nothing downstream is authored.
- **Write:** none — ratification is a stamp on the existing synthesis file, not a new write.

## Phase 5 — Author the governance surface set

- **Done:** Delegate authoring to a producer seat working from the ratified synthesis (`setup.surface-set`, pointer `mochiko:authoring-constitution`). Producer plans first and works only on a lead-approved plan (`setup.plan-approval-producers`); exploration/fact-finding inside this seat's work routes per `setup.model-tiering` (locate/enumerate reads to a haiku-tier `Explore` subagent, interpretive reads stay session tier).
- **Seats:** Agent call, `subagent_type: mochiko:tech-lead` (matches its stated role: "Authors and updates the governance surface — greenfield defaults or brownfield codification").
- **Read (by the seat):** `.mochiko/memory/governance-intent.md`, `plugins/mochiko/skills/authoring-constitution/references/ESSENTIAL-FLOOR.md`, `COMPLIANCE-MODULES.md`, existing `CLAUDE.md` (to regenerate the region idempotently without touching user content outside the markers — `setup.governance-region-ownership`), any existing `.claude/rules/mochiko/*.md` (none exist yet).
- **Write:**
  - `CLAUDE.md` — insert/regenerate the `<!-- mochiko:governance:begin/end -->` region only; semver bump lives inside this region per the run's goal.
  - `.claude/rules/mochiko/*.md` — `paths`-scoped rule files, one Essential Floor category set per file grouping as the skill dictates.
  - `.mochiko/memory/governance-ledger.md` — new (first setup run).
  - Trace summary tying ratified GI-XXX entries to authored principles/modules/waivers.
  - If the knowledge-management module was adopted in Phase 2: scaffold per `templates/constitution-modules/knowledge-management.md` plus the pinned copy at `.mochiko/memory/knowledge-management.md` (`setup.km-module-scaffold`) — this pinned copy carries a never-overwrite floor on future runs.
  - Not applicable here: `setup.constitution-superseded` (delete-on-sight of a legacy `.mochiko/memory/constitution.md`) — no such file exists in this workspace, nothing to delete.
  - Not applicable here: `setup.carve-outs-preserved` — no pre-existing `mochiko:domain-registry` or `mochiko:output-style` carve-outs to preserve verbatim; both are created fresh.
- **Constraint carried forward:** every Essential Floor category must land either a principle or a recorded waiver, or fail condition `setup.fail.floor-category-uncovered` trips.

## Phase 6 — Feature map, baselines, architecture store scaffold

- **Done (greenfield branch):**
  - Scaffold an **empty** `FEATURES.md` index (`setup.feature-map-greenfield`, pointer `mochiko:authoring-feature-map`), never overwriting any existing map content (`setup.map-never-overwrite` — moot here since none exists).
  - Product baselines (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`) are **not** written this run — greenfield defers them to the first `/mochiko:implement` design phase (`setup.baselines-bootstrap`).
  - Architecture store scaffold is unconditional on every mode (`setup.store-scaffold-unconditional`, pointer `mochiko:authoring-architecture-store`): create `.mochiko/product/architecture/` where missing, with a `spine.md` stub (header only, no topology — greenfield) and an empty `concerns.md` beside it.
  - Declare the `Scope:` line on the `spine.md` header (`setup.architecture-scope-handoff`), write-if-absent only: based on Cadence's stated shape ("a web app with a morning email digest"), scope reads as `backend-service, frontend-web`. This is a handoff, not an architecture ruling — no stance is taken; `/mochiko:architecture` deals the actual shelves later.
- **Seats:** lead performs these directly — these are mechanical scaffold writes (empty index, header-only stub), not judgment writes, so no producer/grader split is triggered (`setup.store-ruled-content-never-here`: a file holding only a `Scope:` header is scaffold, not ruled content).
- **Write:** `FEATURES.md` (empty), `.mochiko/product/architecture/spine.md` (header + `Scope:` line), `.mochiko/product/architecture/concerns.md` (empty).
- **Gate:** none (greenfield has no user-map-confirmation step; that's brownfield-only).

## Phase 7 — Independent grading

- **Done:** The full authored surface set (CLAUDE.md region, `.claude/rules/mochiko/*.md`, `governance-ledger.md`, trace summary) is graded by a seat that authored none of it (`setup.author-graded` fail condition guards this) — default FAIL until it reads the actual files and passes.
- **Seats:** Agent call, `subagent_type: mochiko:validator`, instructed to run `mochiko:validation-constitution` against the quality checklist.
- **Read (by the validator):** the authored files themselves — never the tech-lead seat's authoring report.
- **Write:** a PASS/FAIL-style findings report (in-conversation, not a governed surface).
- **Gate:** if FAIL, loop back to Phase 5 for revision, then re-grade. This plan assumes eventual PASS to continue.

## Phase 8 — Final acceptance

- **Gate (`setup.gate-final-acceptance`, floor, non-waivable):** the user accepts the surface set, flagged proposal by flagged proposal, with the trace summary in hand, as plain blocking text — never a timed prompt (`setup.acceptance-plain-text`).
  - *Ruling: accept all* → run closes successfully.
  - *Ruling: accept some, flag others* → the flagged items loop back to Phase 5/7 for revision and re-grading; unflagged items stand accepted.
  - *Ruling: reject* → fail condition `setup.fail.no-acceptance` stands; run does not close.
- **Write:** none new — acceptance is recorded against the already-written files.

## Phase 9 — Reporting and next steps

- **Done:** User-facing prose follows `templates/output-style.md` (`common.register`). Report the next doors: `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline — peer doors, neither ahead of the other — plus `/mochiko:brainstorm` if knowledge-management was adopted (`setup.next-step`, advisory).
- **Write:** none.
- **Never:** suggest a commit only (`setup.no-git-mutations`) — since this workspace isn't even a git repo yet, the suggestion would include `git init` first; no git command is ever run by the run itself.

---

## Standing floors carried through every phase (non-waivable)

- Never offer to delete `governance-intent.md` or `governance-ledger.md` (`setup.durables-never-deleted`).
- Everything outside the `<!-- mochiko:governance:begin/end -->` markers in `CLAUDE.md` is user content, never touched (`setup.governance-region-ownership`).
- No git mutations, no push, ever (`setup.no-git-mutations`).
- Transport floor (`mochiko:patterns-transport-floor`) governs the Phase 3 and Phase 5/7 multi-seat messaging once more than one seat is in play.

## Not-done set (default FAIL — any one standing fails the run)

1. A surface authored before ratification (`setup.fail.pre-ratification-authoring`).
2. An unclosed trace from ratified intent to authored surfaces (`setup.fail.unclosed-trace`).
3. The surface set graded only by its author (`setup.fail.author-graded`).
4. A floor category with neither principle nor recorded waiver (`setup.fail.floor-category-uncovered`).
5. No final user acceptance (`setup.fail.no-acceptance`).
6. No feature map at close (`setup.fail.no-feature-map`).

This plan's phase order is designed so all six are satisfied by close: ratification (Phase 4) precedes authoring (Phase 5); the trace summary is authored alongside the surfaces (Phase 5); grading is by a non-author seat (Phase 7); the interrogation's catalog pass plus Phase 5's Essential Floor coverage check closes category 4; Phase 8 secures acceptance; Phase 6 scaffolds the empty `FEATURES.md` (greenfield's close condition for the feature map).