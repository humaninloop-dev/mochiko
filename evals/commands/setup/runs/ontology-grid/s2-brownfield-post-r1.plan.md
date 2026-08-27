## Action Plan — `/mochiko:setup` for this workspace (plan-only, not executed)

**Workspace found during read-only reconnaissance:** no `.mochiko/`, no `CLAUDE.md`, no `.claude/rules/mochiko/`. The repo (not a git repo) holds a small live Express + better-sqlite3 service, "Linkjar" (`src/server.js`, `src/db.js`, `src/auth.js`, one test file, `README.md` stating informal conventions: "every route tested," "secrets never logged," "SQLite only, no background jobs"). This is a first-time setup on **existing, running code** — the mode I would propose is **brownfield**, subject to the user's ruling.

Schema pre-check already performed (required before any planning): `setup.yaml` and `common.yaml` read raw and in full; `kind: fail` count in `setup.sec.fail-conditions` = 6, matching the mission's stated count — no halt needed.

---

### Phase 0 — Schema load (done)
- **Read:** `plugins/mochiko/schemas/setup.yaml`, `plugins/mochiko/schemas/common.yaml`, `plugins/mochiko/schemas/command-labels.yaml`.
- **Done:** confirmed fail-set count (6/6), resolved all `extends:` stubs (`setup.no-git-mutations`, `setup.acceptance-plain-text`, `setup.transport-floor`) against `common.yaml`.
- **Write:** none. **Seats:** lead only.

### Phase 1 — Workspace scan & mode proposal
- **Read:** repo root listing, `README.md`, `package.json`, `src/**`, `test/**` (already done above); probe for `.mochiko/memory/governance-intent.md`, `CLAUDE.md`, `.claude/rules/mochiko/`, a stray `.mochiko/memory/constitution.md` (none found — `setup.constitution-superseded` not triggered).
- **Propose:** mode = brownfield (existing delivered service, no prior mochiko artifacts).
- **Gate (`setup.user-mode-ruling`, reservation):** present the proposal and ask the user to confirm or override among `greenfield / brownfield / amend`.
  - **If brownfield confirmed** → proceed to Phase 2 as planned.
  - **If user rules greenfield** → skip codebase analysis and brownfield conflict-checking; treat the interrogation as a fresh scaffold despite the existing code (their call to make, not mine to second-guess further).
  - **If user rules amend** → an amend needs a prior ratified `governance-intent.md` as baseline, which doesn't exist here; I would surface that inconsistency and ask them to reconsider, but the final ruling stays theirs.
- **Write:** none.

### Phase 2 — Brownfield codebase analysis (brownfield branch only)
- **Skill:** `mochiko:analysis-codebase` (pointer: `setup.interrogation-inputs`), including its deterministic `detect-stack.sh` baseline.
- **Read:** `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, `package.json`, `README.md`'s "How we work" section (as *declared* intent to test against detected reality). Locate/enumerate sub-reads (file listing) dispatch to a `model: haiku` Explore subagent per `mochiko:patterns-model-tiering`; interpretive reads (does the code actually match the README's claims?) stay on the session tier.
- **Derive:** architecture scope = `backend-service` (Express HTTP API, no frontend/mobile/desktop surface detected) for the `Scope:` handoff line.
- **Write:** `.mochiko/memory/codebase-analysis.md`.
- **Seats:** lead-run analysis (or a dispatched fact-finding seat per `setup.staffing-latitude` — my call, no gate).

### Phase 3 — Interrogation (inline, card by card)
- **Duty (`setup.interrogation-inline`):** run this myself, inline — never delegated.
- **Skill:** `mochiko:analysis-iterative` for the adaptive agenda dimensions, then the catalog deck card by card, recommend-then-arbitrate.
- **Read:** `INTERROGATION-AGENDA.md`, `DOMAIN-DEPENDENCIES.md`, catalog cards `catalog/universal-floor.md` and `catalog/backend-service.md` (scope-matched), `COMPLIANCE-MODULES.md` for optional modules (e.g. knowledge-management).
- **Gate (`setup.user-conflict-rulings`, brownfield only):** any place Phase 2's detected reality conflicts with README's declared conventions (e.g. is every route actually tested? are secrets actually kept out of logs? is there really no background job?) is confronted with the user openly, never silently resolved. Branch: user rules how each conflict resolves in the synthesis (accept reality, accept intent as the goal going forward, or something else) — no default.
- **Gate (`setup.user-card-rulings`, reservation):** every card ruling and every module-adoption ruling (e.g. adopt knowledge-management or not) is the user's, recorded as I go.
- **Write:** none yet — rulings accumulate toward Phase 4's synthesis.

### Phase 4 — Author the synthesis
- **Write:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace), from the governance-intent template/schema. This is my pen alone — no delegation (`setup.stress-test-cold-seat` calls it out explicitly as "your pen").
- **Read:** nothing new; consolidates Phase 3's rulings.

### Phase 5 — Pre-ratification stress test (cold seat)
- **Seat/skill:** a cold seat running `mochiko:review-governance-intent` (e.g. `mochiko:devils-advocate` or `mochiko:tech-lead` agent type), spawned per the **blind two-message dispatch** (`setup.blind-map-dispatch`, floor):
  1. First message: setup topic + project identity/goal only ("Linkjar governance setup") — never the synthesis path. Seat returns its blind Phase 0 angle map.
  2. Second message: send `.mochiko/memory/governance-intent.md` path; seat does the cold read, six hunt classes, cross-examination, returns severity-classified findings + a verdict.
- **Transport floor:** this is the run's first spawned second seat, so `seats: multi` activates `mochiko:patterns-transport-floor` here and for every seat spawned afterward (referenced, never restated).
- **Alternative:** the user may instead record a waiver of this cold seat entirely (`setup.stress-test-cold-seat` allows a "recorded waiver") — gate: I'd offer that option; if taken, log the waiver and skip straight to Phase 6.
- **Gate (`setup.coverage-survivor-routing`):** any coverage finding surviving the stress test is presented as a candidate topic, not silently folded in. Branches per finding:
  - **explore now** → re-enter `mochiko:analysis-iterative` on that angle, re-elicited intent lands under a new GI-XXX entry, synthesis updated.
  - **rule inline** → user gives a direct ruling, recorded into the synthesis without a full re-elicitation.
  - **defer** → logged as an open item, does not block ratification.
- **Read:** the seat reads `governance-intent.md` itself (never my summary of it). **Write:** none by the lead in this phase; the synthesis may be amended per the branch taken.

### Phase 6 — Ratification gate
- **Gate (`setup.gate-synthesis-ratification`, floor, kind: gate):** present the (possibly Phase-5-amended) synthesis for the user's ratification. This is the hard line — **no surface may be authored before this fires**, on pain of `setup.fail.pre-ratification-authoring`.
  - **Ratified** → proceed to Phase 7.
  - **Changes requested** → loop back into Phase 3/4 to amend the synthesis, then re-present; a bounded delta-pass re-stress-test only if the edit is material (per `mochiko:review-governance-intent`'s delta-pass mode), not a full re-run.
  - **User stops here** → run ends with nothing authored; the fail condition is never triggered because nothing was ever written past this point.
- **Write:** none until ratified.

### Phase 7 — Author the governance surface set
- **Duty (`setup.plan-approval-producers`):** the authoring seat plans first; I approve the plan before it writes anything.
- **Seat/skill:** `mochiko:authoring-constitution` (pointer for `setup.surface-set`), run by a producer seat distinct from whoever grades it later (`setup.author-grader-default-fail`).
- **Write:**
  - `CLAUDE.md` — created (doesn't exist yet), containing the marked governance region `<!-- mochiko:governance:begin/end -->`; the region's semver is bumped as part of this write (goal condition). Nothing outside the markers is touched (there's nothing there yet, so this is pure creation, not overwrite) — `setup.governance-region-ownership` floor.
  - `.claude/rules/mochiko/*.md` — `paths`-scoped rule files, including `output-style.md` and a domain-layer rules file carrying the `mochiko:domain-registry` carve-out block, seeded (not overwritten, since absent) — `setup.carve-outs-preserved` floor.
  - `.mochiko/memory/governance-ledger.md` — the durable ledger.
  - A trace summary tying every authored line back to its ratified GI-XXX entry (closes `setup.fail.unclosed-trace`).
  - **If** knowledge-management was adopted in Phase 3: scaffold `templates/constitution-modules/knowledge-management.md` → `.mochiko/memory/knowledge-management.md` (project-pinned copy; never-overwrite floor applies from here on, `setup.km-module-scaffold`).
- **Unconditional, same phase, any mode:** architecture store scaffold — create `.mochiko/product/architecture/` where missing, with a `spine.md` stub whose header carries the `Scope:` line (`backend-service`, from Phase 2) and an empty `concerns.md` beside it (`setup.store-scaffold-unconditional` + `setup.architecture-scope-handoff`). No ruled architecture content is authored here — that's reserved for the first `/mochiko:architecture` visit (`setup.store-ruled-content-never-here` floor).
- **Brownfield feature-map landing (`setup.feature-map-brownfield`):** extend Phase 2's analysis via `mochiko:authoring-feature-map` into reconstructed capabilities from `src/server.js`'s routes (e.g. "save a link," "list saved links," "API-key auth on writes") → `FEATURES.md` + `.mochiko/features/` entries, `delivered` status, reconstructed-from-code mark.
  - **Gate (`setup.user-map-confirmation`, reservation):** the user confirms the reconstructed map entry by entry.
    - **Confirmed as-is** → map lands unchanged.
    - **User corrects/renames/drops entries** → map authored with those corrections; once landed, the map is never bulk-overwritten again (`setup.map-never-overwrite` floor going forward).
- **Baselines bootstrap (`setup.baselines-bootstrap`, "Assumed"):** bootstrap `.mochiko/product/` baselines — `data-model.md` (from `src/db.js`'s schema), `contracts/` (from the Express routes), `constraints-and-decisions.md` (from README's stated hard constraints: SQLite-only, no background jobs), `quickstart.md` — derived from delivered code, since this is brownfield.

### Phase 8 — Independent grading
- **Seat/skill:** a non-author validator (e.g. `mochiko:validator` or `mochiko:tech-lead` running `mochiko:validation-constitution`) reads the **actual files on disk** — never my report of them — and grades against the quality checklist. Default FAIL until confirmed (`setup.author-grader-default-fail` floor; closes `setup.fail.author-graded`).
- Also empirically probes delivery via `mochiko:testing-governance-injection` — confirms `CLAUDE.md`'s region and `.claude/rules/mochiko/*.md` actually inject on their promised paths and change behavior, not just that they parse.
- **Branch — FAIL:** grader's fix list routes back to Phase 7 for revision by the (same) author seat, then re-grades. **Branch — PASS:** proceed.
- **Read:** every authored file from Phase 7. **Write:** a grading report (not a governed surface).

### Phase 9 — Essential Floor coverage check
- **Read:** `ESSENTIAL-FLOOR.md` against the authored rule set.
- **Gate:** any floor category with neither a principle nor a recorded waiver is surfaced to the user: rule it now, or explicitly waive it (closes `setup.fail.floor-category-uncovered`).

### Phase 10 — Final acceptance gate
- **Gate (`setup.gate-final-acceptance`, floor):** present the full surface set — CLAUDE.md region, rules files, ledger, feature map, architecture stub, trace summary — flagged proposal by flagged proposal, as **plain blocking text**, never a timed prompt (`setup.acceptance-plain-text` floor).
  - **Full acceptance** → run closes; goal condition ("user accepted the set with the trace summary in hand") satisfied.
  - **Partial acceptance (some items flagged)** → only the flagged items return to the author seat for a bounded revision, then re-graded and re-presented on just that delta.
  - **Outright decline** → `setup.fail.no-acceptance` stands; run does not close, halts pending the user's further direction.

### Phase 11 — Close & report
- **Verify** the goal's close condition: feature map present and user-confirmed (brownfield), `spine.md` stub + `Scope:` line present, governance region present with bumped semver, trace summary delivered, none of the 6 `kind: fail` conditions standing.
- **Write:** none beyond what's already landed — this phase only checks and reports.
- **Report** per `templates/output-style.md` (`setup.register`): plain user-facing prose, no timed prompts. I would only *suggest* git commands (e.g. `git init && git add -A && git commit`) since this isn't yet a git repo — I would never run them myself (`setup.no-git-mutations` floor).
- **Next-step routing (advisory, `setup.next-step`):** point to `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product baseline as peer doors, plus `/mochiko:brainstorm` if knowledge-management was adopted.