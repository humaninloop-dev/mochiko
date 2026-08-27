# Action Plan — `/mochiko:setup` (plan-only, no execution)

## Workspace read already performed (per the command's own first-action rule)

- `plugins/mochiko/schemas/setup.yaml` and `plugins/mochiko/schemas/common.yaml` read raw, in full — 6 `kind: fail` nodes confirmed present (`setup.fail.pre-ratification-authoring`, `.unclosed-trace`, `.author-graded`, `.floor-category-uncovered`, `.no-acceptance`, `.no-feature-map`), matching the `.md`'s hard-coded Not-done count — no halt needed.
- `plugins/mochiko/schemas/command-labels.yaml` read for label vocabulary.
- Workspace probed: no `.mochiko/`, no `CLAUDE.md`, no `FEATURES.md`, no `ARCHITECTURE.md`, no `.claude/rules/mochiko/`. `package.json` names the product `linkjar` (Node/Express, `better-sqlite3`, ESLint), with existing source at `src/server.js`, `src/db.js`, `src/auth.js` and a test at `test/links.test.js`. Not a git repository.

This is a **first-ever** governance run on an existing, ungoverned codebase.

---

## Phase 1 — Mode proposal

**Does:** Proposes mode from the workspace evidence above: real source and tests exist, no prior `.mochiko/` or CLAUDE.md governance region — **brownfield** is the natural read (not greenfield, since code predates the run; not amend, since no synthesis or surface set exists to amend).
**Reads:** the Glob results already gathered (`src/*`, `test/*`, `package.json`).
**Writes:** nothing yet.
**Gate (`setup.user-mode-ruling`, reservation):** the mode is confirmed with the user before anything else proceeds. *Branches:* user confirms brownfield → continue to Phase 2 unchanged; user asserts greenfield instead (e.g., "treat existing code as throwaway spike") → skip Phase 2's codebase analysis and brownfield feature-map reconstruction, go straight to interrogation with an empty map at close; user asserts amend → halt this plan shape entirely, since no prior synthesis/surface set exists for an amend to target — this would need to be surfaced back to the user as a contradiction before anything else runs.

## Phase 2 — Brownfield codebase analysis

**Does:** Invokes skill `mochiko:analysis-codebase` to extract tech stack, conventions/architecture, domain entities (link records, auth), and an Essential-Floor present/partial/absent assessment (Security / Testing / Error Handling / Observability) against `plugins/mochiko/skills/authoring-constitution/references/ESSENTIAL-FLOOR.md`'s canonical categories. Locate/enumerate sub-reads (e.g., walking `src/`, `test/`) route to a native Explore subagent spawned `model: haiku` per `mochiko:patterns-model-tiering`; interpretive reads (judging what a pattern *means*) stay on the session tier.
**Reads:** `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, `package.json`, any README/CI config found.
**Writes:** `.mochiko/memory/codebase-analysis.md`.
**Seats:** lead-run or a delegated fact-finding seat (staffing latitude) — either way, exploration sub-reads are model-tiered as above.
**Gate:** none formal here, but any detected-reality-vs-declared-intent conflict this surfaces gets confronted with the user, never silently resolved — folded into Phase 3's `setup.user-conflict-rulings` gate rather than resolved here.

## Phase 3 — Interrogation session (inline, lead-run)

**Does:** The lead runs the ten-dimension agenda from `INTERROGATION-AGENDA.md` adaptively via `mochiko:analysis-iterative` (one question per turn, skip what's settled), then deals the catalog deck (`catalog/universal-floor.md`, `catalog/backend-service.md` — the only seeded type shelf, matching this Express backend) recommend-then-arbitrate, card by card. Elicits: project identity/intent, fact profile (compliance triggers), type (backend/service), risk surface, team reality, existing practices (informed by Phase 2's findings), knowledge-management module adoption (default-on, core-whole/electives-per-doc), deployment/release reality, values/non-negotiables, deliberate exclusions. The **one** strictness dial — floor depth `low`/`high` — is declared here, recommend-then-arbitrate.
**Reads:** `INTERROGATION-AGENDA.md`, `catalog/README.md`, `catalog/universal-floor.md`, `catalog/backend-service.md`, `COMPLIANCE-MODULES.md`, `.mochiko/memory/codebase-analysis.md`.
**Writes:** nothing durable yet — session state only, until Phase 4 freezes it.
**Gates, all reserved to the user:**
- `setup.user-card-rulings` — every card ruling, module ruling, and waiver. *Branches per card:* keep as-is / tighten / drop (recorded, not silent) / re-rank.
- `setup.user-conflict-rulings` — any brownfield reality-vs-intent conflict (e.g., analysis shows no secret-scanning CI but the user claims one exists) is put to the user in the open. *Branches:* user corrects the stated fact / user directs a remediation principle to be minted / user accepts the gap and records a waiver.
- KM module adoption — *branches:* adopt core whole (+ optional electives `CHANGELOG.md`/`RUNBOOK.md` per-doc) / decline (recorded in dimension 10 exclusions).
- Depth-level declaration — *branches:* `low` / `high`, one-way only.
- `setup.coverage-survivor-routing` (fires only if Phase 5's cold seat, run afterward, surfaces a scope gap) — *branches:* explore now (re-enter `mochiko:analysis-iterative` on that angle, landing in the GI-XXX namespace) / rule inline / defer.

## Phase 4 — Freeze the synthesis

**Does:** Renders the session output into the durable amend baseline.
**Reads:** `plugins/mochiko/schemas/governance-intent.yaml` (schema, if the `mochiko-cli` binary is unavailable) or invokes `mochiko-cli template governance-intent`.
**Writes:** `.mochiko/memory/governance-intent.md`, GI-XXX namespace — each element (fact, module trigger, waiver, minted principle, exclusion) traced and marked `Confident/Assumed/Contested/Unsure/Deferred`.
**Gate:** none yet — this is a freeze, not a ratification; **no surface may be authored past this point without ratification** (`setup.fail.pre-ratification-authoring` is the hard floor this whole run must not trip).

## Phase 5 — Pre-ratification stress test (cold seat)

**Does:** *Would* spawn an independent cold reviewer using skill `mochiko:review-governance-intent` — never the interrogation's own pen. Two-message blind dispatch (`setup.blind-map-dispatch`, floor): message 1 carries only the setup topic and linkjar's project identity/goal (no path to `governance-intent.md`), the seat builds its Phase-0 angle map blind to the session's conclusions and returns it; only then does message 2 hand it the synthesis path and the cold read begins. Candidate seat: `mochiko:devils-advocate` or a general reviewer agent, run solo by default (staffing latitude) or as a pair for a coherence + coverage split.
**Reads (by the spawned seat, not the lead):** `.mochiko/memory/governance-intent.md`, `INTERROGATION-AGENDA.md`, `.mochiko/memory/codebase-analysis.md`.
**Writes:** none directly — findings return as a message to the lead; dispositions land inside `governance-intent.md`'s own Review section, never conversation-only.
**Gate (`setup.stress-test-cold-seat`):** the user may instead record a waiver of the cold-seat stress-test outright. *Branches:* user allows the cold seat to run (default path above) / user records a waiver — the run proceeds to Phase 6 without an independent stress-test, and that waiver itself becomes part of the accepted trace.
**Transport floor:** the moment this run composes more than one seat (this is the first), `mochiko:patterns-transport-floor` activates for message legs (this dispatch) and stays active for every later multi-seat step (Phases 7, 8) — non-waivable once triggered.
**Follow-on gate:** any surviving coverage finding routes back to Phase 3's `setup.coverage-survivor-routing` gate before ratification.

## Phase 6 — Synthesis ratification

**Does:** Presents the frozen (and, if run, stress-tested) synthesis to the user for ratification — the floor gate `setup.gate-synthesis-ratification`, and the point past which `setup.fail.pre-ratification-authoring` becomes checkable.
**Reads:** `.mochiko/memory/governance-intent.md` in its final state.
**Writes:** nothing — ratification is a user act, not a file write, though the ledger will record it downstream.
**Gate:** plain blocking text, never a timed prompt (`common.acceptance-plain-text`). *Branches:* user ratifies as-is → proceed to Phase 7; user requests changes to specific GI elements → loop back to Phase 3/4 for those elements only, then re-present; user rejects the run's framing entirely → the run halts here, nothing downstream is authored.

## Phase 7 — Author the governance surface set

**Does:** *Would* spawn an authoring seat (e.g. `mochiko:tech-lead`, whose brief is exactly this: author/update the governance surface set) working skill `mochiko:authoring-constitution` against the now-ratified synthesis. Per `setup.plan-approval-producers`, this seat plans first and works only on a plan the lead/user approves before any file is touched.
**Reads:** `.mochiko/memory/governance-intent.md`, `templates/output-style.md`, `templates/constitution-modules/*.md` (for any adopted modules, e.g. `knowledge-management.md`), any existing carve-out content (none found — first run).
**Writes:**
- CLAUDE.md — the marked governance region only (`<!-- mochiko:governance:begin/end -->`), idempotently regenerated; everything outside the markers is untouched (there is no existing CLAUDE.md here, so this is a full first write of the region).
- `.claude/rules/mochiko/*.md` — `paths`-scoped rule files (e.g. security, testing, error-handling, observability, output-style, and a KM file if adopted).
- `.mochiko/memory/governance-ledger.md` — the depth-level stamp, waivers, module attaches.
- The trace summary (GI-ID → authored surface).
- One-line note only if a stray `.mochiko/memory/constitution.md` were found (`setup.constitution-superseded`) — not applicable here, none exists.
**Seats/skills:** `mochiko:tech-lead` (author) + `mochiko:authoring-constitution` (skill), under the plan-approval gate.
**Gate (plan approval):** the lead/user reviews the authoring plan before writes happen. *Branches:* approve as proposed → write proceeds; request changes to scope (e.g., add/drop a module, change file layout) → seat re-plans; reject → return to Phase 6 for re-ratification of the disputed element.

## Phase 8 — Independent grading (author ≠ grader)

**Does:** *Would* spawn a non-author seat (e.g. `mochiko:validator`) running skill `mochiko:validation-constitution` against the drafted set — reads the files themselves, never the tech-lead's report; defaults to FAIL. Checks: quality checklist (universal core + adopted-module fragments), vague-language/anti-pattern scan (including excess-governance), trace closure, floor/module accounting, version bump determination.
**Reads:** the CLAUDE.md governance region, every file under `.claude/rules/mochiko/`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/governance-intent.md`, the trace summary manifest.
**Writes:** the `VALIDATION RESULT` block lands inside the reviewed artifacts (per its floor — never conversation-only evidence).
**Gate:** none reserved to the user here — this is grading, not a user ruling — but the outcome gates progress. *Branches:* PASS → proceed to Phase 9; FAIL → returns to Phase 7 for revision by the (still non-grading) author seat, then re-validates; this loop is the mechanism that prevents `setup.fail.author-graded` from ever tripping.

## Phase 9 — Feature map landing (brownfield)

**Does:** Extends Phase 2's analysis into a feature-map reconstruction via skill `mochiko:authoring-feature-map` — derives linkjar's delivered capabilities from the code itself (e.g., link-saving/reading routes in `src/server.js`, storage in `src/db.js`, an auth capability in `src/auth.js` — exact names depend on what Phase 2 actually found in those files, not guessed here).
**Reads:** `.mochiko/memory/codebase-analysis.md`, `src/server.js`, `src/db.js`, `src/auth.js`; schemas `plugins/mochiko/schemas/features-index.yaml` and `feature-entry.yaml` if the CLI binary is absent.
**Writes:** `FEATURES.md` (repo root index) + `.mochiko/features/FEAT-XXX-<slug>.md` entries, each marked `delivered` status and reconstructed-from-code.
**Gate (`setup.user-map-confirmation`, reservation):** confirmed entry by entry. *Branches per entry:* confirm as reconstructed / rename or resize the capability / reject as not a real durable capability (folds elsewhere or is dropped, recorded).
**Floor:** never-overwrite (`setup.map-never-overwrite`) — moot on this first run since no `FEATURES.md` exists yet, but binds any future amend touching this file.

## Phase 10 — Architecture store scaffold (unconditional, every path)

**Does:** Creates the store layout where missing — never overwriting existing content (none exists here).
**Reads:** Phase 2's analysis for surface-type detection — linkjar is a plain Express backend with no detected frontend build, so the scope reads as `backend-service` only.
**Writes:** `.mochiko/product/architecture/spine.md` — a stub with only the header and a `Scope: backend-service` line (write-if-absent), and an empty `.mochiko/product/architecture/concerns.md` beside it. No topology or `AX-XXX` content — that is explicitly out of this run's reach (`setup.store-ruled-content-never-here`, floor); the first `/mochiko:architecture` visit is what elicits/reconstructs real content.
**Gate:** none — this is unconditional scaffold, not a ruling; the `Scope:` line remains user-overridable later at the architecture desk.

## Phase 11 — Product baselines bootstrap (brownfield-only, `Assumed`)

**Does:** Since this is brownfield, bootstraps the baselines from delivered code rather than deferring them to a first implement run.
**Reads:** `src/db.js` (schema/entities), `src/server.js` (routes), `src/auth.js` (auth flow), Phase 2's analysis.
**Writes:** `.mochiko/product/data-model.md`, `.mochiko/product/contracts/`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/quickstart.md`.
**Seats:** an authoring seat (e.g. `mochiko:technical-analyst` or `mochiko:principal-architect`, lead's call per staffing latitude) under the same plan-approval and sound-loop discipline as Phase 7 — a judgment-authored write to a governing surface, so it plans first, a non-author reviews, the user rules (per `mochiko:patterns-sound-loop`, referenced not restated).
**Gate:** plan approval before writing, same shape as Phase 7's branches.

## Phase 12 — Trace closure check

**Does:** Confirms every ratified GI-XXX element resolves to an authored surface (a principle, a waiver record, a module attach, or an explicit non-applicability) across Phases 7–11's outputs together — this is what keeps `setup.fail.unclosed-trace` from tripping. Owned jointly by the authoring skill's trace summary and the validator's grading pass already run in Phase 8; this phase is the lead's own closure sweep before presenting acceptance.
**Reads:** the trace summary manifest, `governance-intent.md`, all authored surfaces.
**Writes:** nothing new — confirms what's already on disk.
**Gate:** none — an internal check; a gap found here loops back to Phase 7/9/11 for the specific unresolved element, not a separate user decision.

## Phase 13 — Final acceptance

**Does:** Presents the completed set to the user, flagged proposal by flagged proposal (module attaches, minted principles, waivers, depth level, feature-map entries, the store's declared scope) — `setup.gate-final-acceptance`, floor.
**Reads:** everything authored across Phases 7–11.
**Writes:** the semver bump on the governance region per Phase 8's determination, applied at acceptance.
**Gate:** plain blocking text. *Branches:* accept all → run closes, Goal is met; accept most but flag specific items for deferral → those flagged items are recorded as open, and the Goal's "user accepted the set with the trace summary in hand" condition is only satisfied for what was actually accepted — deferred items would need to be resolved before the run can be called fully done; reject a load-bearing item (e.g. the depth level itself) → loop back to Phase 3 for that dimension, re-author, re-grade, re-present.

## Phase 14 — Close-out report and next-step routing

**Does:** Reports in the register from `templates/output-style.md`; states the version bump, what was scaffolded vs. authored vs. deferred; suggests (never runs) a git commit (`setup.no-git-mutations`, floor — doubly moot here since this workspace isn't a git repo, but the rule still holds as a standing floor for any future run where it is one); recommends `/mochiko:specify` for the first feature and `/mochiko:architecture` for the architecture baseline as peer next doors, plus `/mochiko:brainstorm` if the knowledge-management module was adopted.
**Reads:** nothing new.
**Writes:** nothing — a conversational report only.
**Gate:** none.

---

## Not-done set — the 6 `kind: fail` conditions this plan is built to avoid tripping

1. A surface authored before ratification (guarded by ordering Phase 7 strictly after Phase 6).
2. An unclosed intent→surface trace (Phase 12).
3. The set graded only by its own author (Phase 8's independent validator, never the Phase 7 author).
4. A floor category with neither principle nor recorded waiver (Phase 3's card rulings + Phase 8's floor-accounting check).
5. No user acceptance (Phase 13).
6. No feature map at close — brownfield reconstruction unconfirmed (Phase 9's entry-by-entry confirmation gate closes this).