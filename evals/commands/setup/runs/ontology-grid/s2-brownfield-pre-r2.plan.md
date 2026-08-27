# Action Plan — `/mochiko:setup` on the `linkjar` workspace (plan-only, not executed)

Grounding already done for this plan (reads only, no writes): `plugins/mochiko/schemas/setup.yaml` (confirmed 6 sections, 6 `fail-condition` rules — count matches the command's hard-coded Not-done set, no halt needed), `command-labels.yaml`, the command file, and the producer/reviewer skills (`authoring-constitution`, `review-governance-intent`, `analysis-codebase`, `validation-constitution`, `authoring-feature-map`, `authoring-architecture-store`, `INTERROGATION-AGENDA.md`). Workspace scan found: no `.mochiko/`, no `CLAUDE.md`, no `.claude/rules/mochiko/` — but a real product: `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, `package.json` ("linkjar", Node ≥20, Express, better-sqlite3), `README.md`. This is a live, deployed backend service, not an empty repo — the mode proposal below is built on that evidence.

## Phase 0 — Schema load (done)
- **Does:** Read `setup.yaml` raw in full; interpret `${var}` substitutions; confirm the 6 fail-conditions tally with the command's hard-coded count.
- **Reads:** `plugins/mochiko/schemas/setup.yaml`, `plugins/mochiko/schemas/command-labels.yaml`.
- **Writes:** none.
- **Seats:** lead only.
- **Gate:** none (a rule not yet read blocks opening the run — already satisfied).

## Phase 1 — Workspace scan and mode proposal
- **Does:** Locate/enumerate pass over the repo root (no governance surfaces present; `src/`, `test/`, `package.json`, `README.md` present) → propose **brownfield**, surface type **backend-service** (Express + better-sqlite3 on Node ≥20).
- **Reads:** repo root listing, `package.json`, `README.md`.
- **Writes:** none.
- **Seats:** the locate/enumerate part dispatches to a native `Explore` subagent pinned to `model: haiku` (patterns-model-tiering); the mode judgment itself stays with the lead.
- **Gate (`setup.user-mode-ruling`, reserved to the user):** present the brownfield/backend-service proposal.
  - *User confirms brownfield* → continue to Phase 2.
  - *User overrides to greenfield* (treat `src/` as disposable scaffolding) → skip Phase 2's codebase analysis and the conflict gate in Phase 2; Phase 6 scaffolds an empty `FEATURES.md` instead of reconstructing; Phase 7's baselines bootstrap is deferred to the first `/mochiko:implement` design phase instead of seeded now.
  - *User says amend* → there is no existing `governance-intent.md` or CLAUDE.md governance region to amend; lead states this and re-asks between greenfield/brownfield.
  - *User narrows scope via a setup request* → folded into Phase 3 dimension 1 (project identity & intent) rather than changing the mode.

## Phase 2 — Brownfield codebase analysis
- **Does:** Run `mochiko:analysis-codebase`. Deterministic layer (`detect-stack.sh`, or an Explore/haiku dispatch) confirms Node/Express/better-sqlite3/eslint. Interpretive layer (stays on session tier) assesses the Essential Floor and extracts the domain entity (`Link`: id, url, title, tags, created_at).
  - **Essential Floor status, file-cited:** Security = partial (`src/auth.js` gates writes with a single static `LINKJAR_API_KEY`, no rotation/hash; `src/server.js:10-14` logs the presented key verbatim on every request); Testing = partial (`test/links.test.js` covers only `db.js`; `server.js` routes and `auth.js` are untested); Error Handling = partial (basic 400/404s, no centralized handler, no DB-failure handling); Observability = absent/partial (`console.log` only, no structured logs/metrics/request IDs).
  - **Strengths to preserve:** clean module boundaries (server/db/auth), parameterized SQL throughout, `engines` pin, eslint configured.
  - **Detected-reality-vs-declared-intent conflicts** (README's "How we work" vs. code): (1) README claims "secrets never appear in logs" — `server.js` logs the API key on every request; (2) README claims "the service runs no background jobs" — `db.js` runs a `setInterval` nightly purge inside the web process; (3) README claims "every route is covered by a test before it ships" — `server.js`'s three routes and `auth.js` have no tests.
- **Writes:** `.mochiko/memory/codebase-analysis.md` (per `codebase-analysis.yaml`).
- **Seats:** lead for interpretive judgment; Explore/haiku for the deterministic/enumerate sub-passes.
- **Gate (`setup.user-conflict-rulings`, reserved to the user):** each of the 3 conflicts is confronted in the open, not silently resolved.
  - *Logged API key:* user may rule "fix the code + author a MUST-not-log-secrets principle," or "accept as a recorded waiver with a revisit trigger." Either ruling lands in the synthesis, not silently in code.
  - *Background purge job:* user may rule "correct the README's claim, codify the job as an intentional Emergent Ceiling pattern," or "treat as a gap and file a MUST-implement item." No silent resolution either way.
  - *Untested routes:* user may rule "waive with a transition trigger" or "MUST-implement: route/auth coverage before next release." Feeds the Testing floor card's row directly.

## Phase 3 — Inline interrogation session
- **Does:** Lead runs the ten-dimension agenda inline via `mochiko:analysis-iterative`, one question per turn, adaptive (brownfield pre-fills dimension 6 from Phase 2). Concretely: dimension 1 (linkjar, small reading-list service for a browser extension, running for a small user group since last year); dimension 2 (fact profile — likely no regulated data class, user confirms); dimension 3 (backend-service, confirmed); dimension 4 (risk — API-key/secret exposure, personal reading data, not financial); dimension 5 (team reality — small/solo, per README tone); dimension 6 (pre-filled from Phase 2); dimension 7 (knowledge-management module, offered default-on); dimension 8 (deployment/release reality, `release-gates` module always offered for a deployed target); dimension 9 (values/non-negotiables — the "secrets never in logs" and "no background jobs between requests" claims surface here as candidate minted principles given Phase 2's findings); dimension 10 (deliberate exclusions).
  - Depth-level declaration: recommend `low` (small, low-risk service), user arbitrates (one-way, recorded).
  - Assert the four floor cards at the declared row; deal the backend-service shelf deck card by card, recommend-then-arbitrate; mint any dimension-9 intent no card covers; run the layered-architecture beat only if a layered card is kept/minted (unlikely for this 3-file service, but not foreclosed); record waiver rulings for any standard the user deviates from (e.g., accepting the purge job with a waiver rather than removing it).
- **Reads:** `INTERROGATION-AGENDA.md`, `catalog/` deck, `DOMAIN-DEPENDENCIES.md`, `codebase-analysis.md`.
- **Writes:** none yet (session state only, assembled into the synthesis in Phase 4).
- **Seats:** lead only (interrogation is explicitly lead-conducted, never delegated).
- **Gates:** every card ruling, module ruling, and waiver is the user's (`setup.user-card-rulings`) — each recommend-then-arbitrate step is its own micro-gate; branches are simply "user's ruling as given," recorded verbatim.

## Phase 4 — Synthesis authoring and ratification
- **Does:** Lead assembles `.mochiko/memory/governance-intent.md` (GI-XXX namespace, confidence-marked) from Phase 3. Then spawns an independent cold seat for `mochiko:review-governance-intent` under the two-message blind-dispatch protocol: message 1 names only the setup topic/project identity/goal (never the synthesis path) so the reviewer builds its angle map blind; message 2 (after the map returns) hands over the synthesis path to begin the cold read. Default staffing: solo cold seat (pair optional, lead's call) unless the user waives the stress-test entirely.
- **Reads (by the cold seat):** the frozen synthesis, `INTERROGATION-AGENDA.md`, `codebase-analysis.md`.
- **Writes:** `.mochiko/memory/governance-intent.md`; the reviewer writes no files, only a survivor report to the lead.
- **Seats:** lead (author) + one independent reviewer agent (never a session participant).
- **Gate A (`setup.coverage-survivor-routing`, reserved to the user):** any coverage finding surviving the stress test (e.g., a thin dimension-9 or a mark the reviewer disputes) is presented as a candidate topic.
  - *Explore now* → re-enter `mochiko:analysis-iterative` on that angle, re-elicited intent lands in a new GI-XXX.
  - *Rule inline* → lead resolves without a new interrogation pass.
  - *Defer* → recorded as open, synthesis proceeds.
- **Gate B (`setup.gate-synthesis-ratification`, reserved to the user, floor):** ratify the (possibly revised) synthesis.
  - *Ratify as-is* → Phase 5 begins; no surface may be authored before this point.
  - *Request edits* → lead revises, optionally a bounded delta-pass with the cold seat, re-presents.
  - *Reject* → back to Phase 3 for the disputed dimension(s).

## Phase 5 — Authoring the governance surface set
- **Does:** `mochiko:authoring-constitution` (brownfield branch), driven by the ratified synthesis + `codebase-analysis.md`. Plans first, works only on a lead-approved plan (`setup.plan-approval-producers`). Produces: CLAUDE.md governance region (ratified stamp, principle index, universal principles with floor first and `(NON-NEGOTIABLE)` marks, tech stack = Node 20/Express/better-sqlite3, quality-gates summary with real commands `npm test` / `npm run lint`, module pointers); scope-bound `.claude/rules/mochiko/*.md` (e.g. a routes/DB concern file scoped `paths: [src/**, test/**]`, plus the unconditional output-style rules file); the ledger with Three-Part records per GI-ID, waiver table, module attachments; Emergent Ceiling principles from the preserved strengths (parameterized SQL, module split), each traced or flagged; `evolution-notes` module (always in brownfield) recording the three confronted conflicts and their rulings; `knowledge-management` module scaffold if adopted (project-pinned copy, never-overwrite floor); the trace summary manifest. Any element resisting clean enforceable formulation is a flagged proposal for the acceptance gate, never silently authored.
- **Reads:** ratified synthesis, `codebase-analysis.md`, `governance-surfaces.yaml`, `ESSENTIAL-FLOOR.md`, `EMERGENT-CEILING-PATTERNS.md`, `constitution-modules/*.md`.
- **Writes:** `CLAUDE.md` (governance region only), `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/knowledge-management.md` (if adopted).
- **Seats:** an authoring seat (lead or a delegated subagent, lead's staffing call) — must not be the same seat as Phase 8's grader.

## Phase 6 — Feature map landing (brownfield reconstruction)
- **Does:** `mochiko:authoring-feature-map` derives the map from the delivered code: one candidate capability (e.g. "Link saving & retrieval") covering `POST/GET/DELETE /links` and `GET /health`, marked `delivered` + reconstructed-from-code.
- **Reads:** `codebase-analysis.md`'s extracted routes, `features-index.yaml`, `feature-entry.yaml`.
- **Writes:** `FEATURES.md` (repo root), `.mochiko/features/FEAT-XXX-*.md`.
- **Seats:** authoring seat (may be the same as Phase 5 or delegated).
- **Gate (`setup.user-map-confirmation`, reserved to the user):** entry-by-entry confirmation.
  - *Confirmed as-is* → landed.
  - *User edits naming/extent/split* → applied, then landed.
  - *User rejects an entry* → reworked before landing (never overwritten silently — never-overwrite floor).

## Phase 7 — Architecture store scaffold (unconditional) and baselines bootstrap
- **Does:** Create the store scaffold on this brownfield path (also required on greenfield, per the goal protocol): `.mochiko/product/architecture/spine.md` stub with header `Scope: backend-service`, empty `concerns.md` beside it — scaffold only, no ruled topology/AX-rows (the first `/mochiko:architecture` visit does the real reconstruction+confirmation). Bootstraps product baselines from the delivered code: `data-model.md` (Link entity), `contracts/` (the four endpoints), `constraints-and-decisions.md` (e.g. the "SQLite-only, no background jobs" constraint flagged against the actual purge job as a drift item for the architecture desk to reconcile), `quickstart.md` (`npm install`/`start`/`test`).
- **Reads:** `codebase-analysis.md`, `architecture-store.yaml`.
- **Writes:** `.mochiko/product/architecture/spine.md`, `.mochiko/product/architecture/concerns.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/quickstart.md`.
- **Seats:** authoring seat. No user gate here — this is scaffold, not ruled content.

## Phase 8 — Independent grading
- **Does:** A different agent (never Phase 5's author, never the lead if the lead authored) runs `mochiko:validation-constitution`, reading only the files on disk (CLAUDE.md region, every rules file, the ledger, the synthesis, the trace manifest) — never the author's report. Emits `VALIDATION RESULT: PASS/FAIL` with checklist score, surface integrity, trace closure, floor/module accounting, anti-patterns, and a version-bump determination.
- **Reads:** the full authored set + synthesis + trace manifest.
- **Writes:** none (verdict lands in the reviewed artifacts per the skill's floor, not a separate file).
- **Seats:** independent validator agent.
- **Gate (implicit, `setup.author-grader-default-fail`):**
  - *PASS* → Phase 9.
  - *FAIL* → lead routes each fix back to the Phase 5 author (never self-clears), re-authors, re-validates; loops until PASS.

## Phase 9 — Final acceptance
- **Does:** Lead presents the finished set flagged-proposal by flagged-proposal, with the trace summary and the validator's PASS in hand, as plain blocking text (never timed).
- **Writes:** none in this phase itself.
- **Gate (`setup.gate-final-acceptance`, reserved to the user, floor):**
  - *Accepts everything* → Phase 10.
  - *Rejects/edits one flagged item or principle* → only that item is revised, re-validated (bounded delta), re-presented.
  - *Wants to waive a floor category outright* → recorded as a formal ledger waiver (never silently dropped), re-validated, re-presented.

## Phase 10 — Close-out
- **Does:** Confirm the governance region's semver bump (per the validator's determination); suggest — never run — a commit covering the new/changed files (`setup.no-git-mutations`); report in the project's register (`templates/output-style.md`); state next steps (advisory): `/mochiko:specify` for the first feature and `/mochiko:architecture` for the architecture baseline (peer doors), plus `/mochiko:brainstorm` if knowledge-management was adopted; reconfirm `governance-intent.md` and `governance-ledger.md` are durable and never offered for deletion.
- **Not-done check (6 fail-conditions, all must be clear):** no surface authored pre-ratification; no unclosed GI→surface trace; the set graded by someone other than its author; no floor category left uncovered by both principle and waiver; user acceptance actually given; a feature map exists (reconstructed and confirmed here, since brownfield).