# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Invocation context observed:** `$ARGUMENTS` is empty. Workspace scan shows a real, running Node service ("Linkjar" — Express + better-sqlite3, `package.json` v0.4.2, `README.md` describing existing practices: "every route tested," "no secrets in logs," "SQLite only, no background jobs"), source in `src/server.js` / `src/db.js` / `src/auth.js`, tests in `test/links.test.js`. No `.mochiko/` directory, no `CLAUDE.md`, no prior governance anywhere. This shapes the plan as a **brownfield, first-ever run** — every gate below is written concretely against this repo, not abstractly.

---

## Phase 1 — Load the binding rules (mandatory first action)

- **Done:** Read `plugins/mochiko/schemas/setup.yaml` and `plugins/mochiko/schemas/common.yaml` raw, in full (already performed to build this plan). Confirm the `kind: fail` rule count in `setup.sec.fail-conditions` is exactly 6 (verified: pre-ratification-authoring, unclosed-trace, author-graded, floor-category-uncovered, no-acceptance, no-feature-map). If that count were ever off, the run halts before proceeding — not triggered here.
- **Read:** `setup.yaml`, `common.yaml`; `plugins/mochiko/schemas/command-labels.yaml` consulted for label vocabulary as rules are cited.
- **Written:** nothing.
- **Seats/skills:** lead only.
- **Gate:** none.

## Phase 2 — Entry and mode proposal

- **Done:** Since `$ARGUMENTS` is empty, propose the mode from workspace evidence: existing, running code with no `.mochiko/` state → **brownfield** is proposed.
- **Read:** repo root listing, `package.json`, `README.md` (already sampled above).
- **Written:** nothing.
- **Seats/skills:** lead; any pure directory-listing/enumeration sub-reads would tier to a `haiku`-model `Explore` subagent per `mochiko:patterns-model-tiering` (setup.model-tiering) — not needed here since the listing is already small and interpretive judgment (is this brownfield?) stays on the lead.
- **Gate (`setup.user-mode-ruling`, reserved to the user):** "This looks like an existing, deployed service with no governance yet — treat this as **brownfield**?"
  - *User confirms brownfield* → proceed to Phase 3.
  - *User overrides to greenfield* (treat existing code as irrelevant) → skip Phase 3 (codebase analysis) and Phase 10's brownfield map reconstruction; go straight to interrogation or authoring with the greenfield branch instead, and the greenfield feature-map/baseline rules (`setup.feature-map-greenfield`, deferred baselines) apply instead of the brownfield ones throughout.
  - *User rules amend* → there is no prior `governance-intent.md` or surface set to amend; the lead surfaces that an amend has nothing to act on and asks the user to re-rule between greenfield and brownfield before continuing.

## Phase 3 — Brownfield codebase analysis

- **Done (brownfield branch only):** Run `mochiko:analysis-codebase`'s Setup-Brownfield mode — deterministic stack detection (`detect-stack.sh`) plus model-judgment architecture/convention read, Essential-Floor status per category (Security / Testing / Error Handling / Observability, each present/partial/absent with file citations), domain-entity extraction, strengths-to-preserve, inconsistencies, and capability signals (routes/services) that will seed Phase 10's feature map.
- **Read:** `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, `README.md`, `package.json`.
- **Written:** `.mochiko/memory/codebase-analysis.md`.
- **Seats/skills:** a producing seat runs `mochiko:analysis-codebase`. Per `setup.plan-approval-producers`, this seat states its analysis approach and the lead approves it before it writes. Locate/enumerate sub-reads tier to a cheap `Explore` subagent; the Essential-Floor status judgment stays session-tier (interpretive).
- **Gate:** none at the user level here (this is a fact-finding artifact, not a ruling); the lead reviews it as its own checkpoint before interrogation begins.

## Phase 4 — Interrogation (inline, lead-conducted)

- **Done:** The lead runs the interrogation **itself, inline** (never delegated) — the agenda's dimensions worked adaptively via `mochiko:analysis-iterative`, then the catalog deck worked card by card, recommend-then-arbitrate. For Linkjar this concretely covers: team size/solo status, risk posture, expected lifespan, deployment target (single small server, per README), the Essential-Floor depth level (low/high) for each of Security/Testing/Error-Handling/Observability given the analysis's present/partial/absent read, whether a layered-architecture principle should be kept/minted, release-gates content, and whether the knowledge-management module is adopted.
- **Read:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`, plus `.mochiko/memory/codebase-analysis.md` from Phase 3.
- **Written:** nothing yet (interrogation is conversational; the synthesis is assembled in Phase 5).
- **Seats/skills:** lead + `mochiko:analysis-iterative`.
- **Gates (all reserved to the user, `setup.user-card-rulings`):** every agenda answer, every catalog-card ruling, every waiver, and every module adoption/decline is the user's call, offered recommend-then-arbitrate. Concrete branch shape per item: *accept the recommendation* → recorded as ruled; *adjust* → the adjusted form is recorded; *reject/waive* → a waiver record is queued for the ledger with justification.
  - **Brownfield conflict sub-gate (`setup.user-conflict-rulings`):** where the analysis contradicts a declared practice (e.g. README claims "every route covered by a test" but the analysis finds a route in `src/server.js` with no matching case in `test/links.test.js`), the lead confronts this in the open. Branches: *user rules it a real gap* → becomes a MUST-implement principle candidate for authoring; *user waives it* → a justified waiver is queued for the ledger; *user disputes the finding* → the lead re-checks the cited file before either path is taken.

## Phase 5 — Synthesis authored

- **Done:** The lead renders the frozen synthesis from everything ruled in Phase 4 — fact profile, floor-expression and deck rulings, minted intents, waivers, modules, exclusions — each carrying a `GI-XXX` ID and a confidence mark (Confident/Assumed/Contested/Unsure/Deferred).
- **Read:** `plugins/mochiko/schemas/governance-intent.yaml` (schema, raw, since this is the shipped first-class source in the absence of the CLI binary).
- **Written:** `.mochiko/memory/governance-intent.md`.
- **Seats/skills:** lead only — this is explicitly the lead's own pen.
- **Gate:** none yet (ratification is Phase 8, after the stress test).

## Phase 6 — Pre-ratification cold-seat stress test

- **Sub-gate first:** because the synthesis is the lead's own artifact (`setup.stress-test-cold-seat`), it always gets a cold-seat stress test **or the user's recorded waiver**. The lead asks: "Skip the independent stress-test of this synthesis before you ratify it?"
  - *User waives* (recorded) → skip directly to Phase 8 (ratification).
  - *User does not waive* → continue below.
- **Done:** Blind two-message dispatch (`setup.blind-map-dispatch`, non-waivable once a cold seat runs): message 1 gives the reviewer only the setup topic and project identity/goal ("Linkjar — small Express+SQLite link-saving service, first-ever brownfield governance setup") — never the synthesis path — so it builds a blind Phase-0 coverage angle map against the ten agenda dimensions. Its map must return before message 2 sends the synthesis path (`.mochiko/memory/governance-intent.md`) and the cold read (coverage + coherence lenses; both fall to one seat if solo) begins.
- **Read (by the reviewer seat):** `.mochiko/memory/governance-intent.md`, the interrogation agenda, `.mochiko/memory/codebase-analysis.md`.
- **Written:** nothing on disk — the seat returns a survivor report as a message to the lead (severity-classified findings, tally, recommended status: `ready` / `needs-revision` / `critical-gaps`).
- **Seats/skills:** an independent reviewer seat running `mochiko:review-governance-intent`, never the lead, never a session participant. If the lead composes a **pair** here, `seats: multi` fires and `mochiko:patterns-transport-floor` governs the cross-seat messaging and the cross-examination step non-waivably.
- **Gate:** none yet — findings route through Phase 7.

## Phase 7 — Coverage-survivor routing

- **Done:** For every surviving coverage finding, the lead presents it to the user as a **candidate topic questioning the setup's scope** — never silently folded, never presented as a settled card.
- **Gate (`setup.coverage-survivor-routing`, reserved to the user), per finding:**
  - *Explore now* → re-enter `mochiko:analysis-iterative` on that angle; newly elicited intent lands in the `GI-XXX` namespace, updating the synthesis (may trigger a bounded delta re-review).
  - *Rule inline* → the user answers directly; the lead folds the ruling into the synthesis.
  - *Defer* → noted as open, not resolved this run (candidate for the brownfield `evolution-notes` ledger section as a flagged follow-up).
  - A finding that overlaps an already-worked agenda dimension instead follows the ordinary interrogation-follow-up path (back into Phase 4's card mechanics), not this three-way routing.
- Coherence findings and a `critical-gaps` recommended status route the lead back into targeted interrogation follow-up until the picture is resolved or the user accepts a `needs-revision` disposition.
- **Written:** synthesis updates land back in `.mochiko/memory/governance-intent.md` as Phase 4/5 are revisited.

## Phase 8 — Ratification gate

- **Done:** Present the final (possibly revised) synthesis in full. This is the load-bearing ordering checkpoint: **no surface file is written before this gate returns "ratified"** — violating that is `setup.fail.pre-ratification-authoring`, one of the 6 fail conditions.
- **Gate (`setup.gate-synthesis-ratification`, floor, plain blocking text, never timed):** "Do you ratify this governance intent synthesis?"
  - *Ratifies* → proceed to Phase 9.
  - *Requests changes* → loop back to Phase 4/5 (revise), optionally a bounded delta-pass re-review via `mochiko:review-governance-intent` on the material edit, then re-present here.
  - *Aborts the run* → stop without authoring anything; the synthesis and any partial memory files already on disk are left in place (never deleted — `setup.durables-never-deleted` covers the synthesis and ledger specifically); report state and end.

## Phase 9 — Author the governance surface set

- **Done:** `mochiko:authoring-constitution`, **brownfield branch**, is run from the now-ratified synthesis plus `codebase-analysis.md` — Essential Floor assessed against the code, an Emergent Ceiling codifying genuinely good existing patterns (candidates from Linkjar: "every route tested before it ships," "no secrets in logs," "SQLite only / no background jobs" — each only if the synthesis actually sanctions it; an unsanctioned ceiling pattern becomes a flagged proposal instead), plus the mandatory `evolution-notes` module (always attached in brownfield).
- **Read:** `.mochiko/memory/governance-intent.md`, `.mochiko/memory/codebase-analysis.md`, `references/catalog/`, `ESSENTIAL-FLOOR.md`, `EMERGENT-CEILING-PATTERNS.md`, `COMPLIANCE-MODULES.md`, `DOMAIN-DEPENDENCIES.md`, `plugins/mochiko/schemas/governance-surfaces.yaml`.
- **Written:**
  - `CLAUDE.md` — created (none exists), governance region between `<!-- mochiko:governance:begin -->`/`end` markers: ratified stamp (version, date, floor, depth level, modules), principle index, universal principles as short imperative lines (floor ones marked `NON-NEGOTIABLE`), tech stack (Node ≥20, Express, better-sqlite3, eslint), quality-gates summary with real commands (`npm test`, `npm run lint`), module pointers, and the `mochiko:output-style` switch line written default-on (first authoring).
  - `.claude/rules/mochiko/*.md` — one file per scope-bound concern (e.g. a layer/domain rules file if layered-architecture was kept/minted, carrying the `mochiko:domain-registry` block; a Shape-5 output-style rules file scoped to deliverable-authoring paths, mandatory every run), each with `paths` globs and the standing new-file read line surfaced in the region.
  - `.mochiko/memory/governance-ledger.md` — Three-Part records per `GI-XXX`, waiver table, module attachments, the `evolution-notes` section (Essential-Floor present/partial/absent per category with the confrontation rulings from Phase 4), trace summary manifest.
  - If knowledge-management was adopted in Phase 4: `.mochiko/memory/knowledge-management.md` scaffolded from `templates/constitution-modules/knowledge-management.md` (never-overwrite floor holds thereafter).
  - Supersession check: no `.mochiko/memory/constitution.md` exists in this workspace, so `setup.constitution-superseded` is a no-op here (checked, nothing to delete).
- **Seats/skills:** a producing seat runs `mochiko:authoring-constitution`; per `setup.plan-approval-producers` it states its routing plan (which elements land where, which modules attach) and the lead approves before writing.
- **Gate:** none at the user level yet — grading and acceptance come next.

## Phase 10 — Feature map landing (brownfield reconstruction)

- **Done:** Extend Phase 3's extracted capability signals (Linkjar's routes/services in `src/server.js`, the auth check in `src/auth.js`) into a reconstructed capability set with `delivered` status and a reconstructed-from-code mark.
- **Read:** `.mochiko/memory/codebase-analysis.md`, `plugins/mochiko/schemas/features-index.yaml`, `plugins/mochiko/schemas/feature-entry.yaml`.
- **Written:** `FEATURES.md` (repo root index, created), `.mochiko/features/FEAT-XXX-<slug>.md` entry files (e.g. one per capability such as "save a link," "list saved links," write-route authentication).
- **Seats/skills:** `mochiko:authoring-feature-map`; the map-minimalism discipline (`mochiko:patterns-map-minimalism`) governs whether a signal earns its own capability or folds into an existing one.
- **Gate (`setup.user-map-confirmation`, reserved to the user), entry by entry:**
  - *Confirms* an entry → lands as-is.
  - *Corrects* an entry (wrong boundary, missed route, wrong extent) → the lead adjusts and re-presents just that entry.
  - *Rejects* an entry → dropped, with the reason noted.
  - The never-overwrite floor (`setup.map-never-overwrite`) applies to this write; since no prior map exists, this is a clean initial landing rather than a merge.

## Phase 11 — Product baselines bootstrap (brownfield, `Assumed`)

- **Done:** From the delivered code, bootstrap the product baselines.
- **Read:** `src/db.js` (schema), `src/server.js` (routes/contracts), `README.md` (existing constraints: SQLite-only, no background jobs).
- **Written:** `.mochiko/product/data-model.md` (e.g. Link entity, tags, API key), `.mochiko/product/contracts/` (existing HTTP routes as informal contracts), `.mochiko/product/constraints-and-decisions.md` (SQLite-only, no background jobs, etc., as existing decisions), `.mochiko/product/quickstart.md` (install/start/test commands).
- **Seats/skills:** same authoring seat as Phase 9/10, or a dedicated bootstrap pass — lead's staffing latitude.
- **Gate:** none separate from Phase 15's overall acceptance.

## Phase 12 — Architecture store scaffold (unconditional, every path)

- **Done:** Create `.mochiko/product/architecture/` since it does not exist, with a `spine.md` stub and an empty `concerns.md` beside it. Read which surface types the product carries — here, backend-service only (an Express HTTP service; no frontend/mobile/desktop code detected) — and declare `Scope: backend-service` on the stub's header line, write-if-absent.
- **Read:** repo structure already scanned in Phase 2/3.
- **Written:** `.mochiko/product/architecture/spine.md` (stub + `Scope:` line only — no topology), `.mochiko/product/architecture/concerns.md` (empty).
- **Seats/skills:** lead or authoring seat; `mochiko:authoring-architecture-store` grammar governs the stub shape. No architecture stance is taken here (`setup.store-ruled-content-never-here` — that's `/mochiko:architecture`'s job later).
- **Gate:** none — this is scaffold, not ruled content.

## Phase 13 — Independent grading (author ≠ grader, default-FAIL)

- **Done:** A fresh validator seat — never the Phase 9 author — runs `mochiko:validation-constitution` against the files on disk only (CLAUDE.md region, `.claude/rules/mochiko/*.md`, the ledger), cross-checked against `governance-intent.md` and the trace manifest. Emits a binary `VALIDATION RESULT: PASS/FAIL` with checklist score, surface integrity, trace closure, floor/module accounting, anti-patterns found, and a semver bump determination (MAJOR, as this establishes the floor for the first time).
- **Read:** the full surface set plus the synthesis and trace manifest — never the author's report.
- **Written:** the validator's verdict lands back into the graded files/ledger per its own protocol (dispositions recorded in the artifacts, not only in conversation).
- **Seats/skills:** an independent seat running `mochiko:validation-constitution`.
- **Branch:**
  - *PASS* → Phase 14.
  - *FAIL* → fix list routes back to the Phase 9 authoring seat for revision, then re-validation, looping until PASS or the user chooses to stop the run.

## Phase 14 — Semver and trace confirmation

- **Done:** Confirm the CLAUDE.md region's ratified-stamp semver matches the validator's determined bump, and that the trace manifest closes (every GI element has a home; every surface element cites its GI element) — this closes two of the fixed Goal clauses.
- **Written:** nothing new — confirmation only.

## Phase 15 — Final acceptance gate

- **Done:** Present the complete surface set plus the trace summary to the user, **flagged proposal by flagged proposal** (any element authoring could not cleanly formulate or any ceiling pattern lacking a sanctioning fact) alongside the overall set.
- **Gate (`setup.gate-final-acceptance`, floor, plain blocking text, never timed):**
  - *Accepts everything* → run closes as done.
  - *Rejects specific flagged proposals/principles* → bounded revision on just those elements by the authoring seat, re-validated (Phase 13 delta), re-presented here.
  - *Rejects wholesale* → if the rejection reveals a synthesis-level problem, loop back to Phase 4/5; if it's purely a formulation problem, loop back to Phase 9 only.

## Phase 16 — Fail-condition audit (default-FAIL, all 6 must be clear)

Explicit walk before declaring done:
1. `setup.fail.pre-ratification-authoring` — no surface was written before Phase 8 returned ratified. ✓ by construction above.
2. `setup.fail.unclosed-trace` — trace manifest closes across the set. ✓ checked in Phase 14.
3. `setup.fail.author-graded` — Phase 13's validator seat is distinct from Phase 9's author. ✓ by construction.
4. `setup.fail.floor-category-uncovered` — Security/Testing/Error-Handling/Observability each carry a principle or a recorded waiver in the ledger. ✓ checked against Phase 4/9 outputs.
5. `setup.fail.no-acceptance` — Phase 15 returned explicit acceptance. ✓ by construction.
6. `setup.fail.no-feature-map` — Phase 10's brownfield reconstruction was confirmed. ✓ by construction.
- If any stands unresolved, the run is not closed — it loops back to the relevant earlier phase instead.

## Phase 17 — Report and next steps

- **Done:** Report using `templates/output-style.md` register; name peer next steps (`setup.next-step`, advisory, non-blocking): `/mochiko:specify` for the first feature and `/mochiko:architecture` for the architecture baseline — neither sequenced ahead of the other — plus `/mochiko:brainstorm` since knowledge-management may have been adopted. Suggest (never run) a commit covering `CLAUDE.md`, `.claude/rules/mochiko/`, `.mochiko/memory/*`, `.mochiko/product/*`, `FEATURES.md`, `.mochiko/features/*` (`setup.no-git-mutations` — suggest only, never execute, never push).
- **Written:** nothing further; report only.