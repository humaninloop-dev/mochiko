# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Invocation context (read, not written):** empty `` — no explicit request, so the mode is proposed from the workspace. The workspace root has no `.mochiko/`, no `.claude/`, and no `CLAUDE.md` — this is a first governance run. It does hold a working Node/Express service ("Linkjar": `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, `package.json`, `README.md`) that has "been running for a small group of users since last year." That combination — real shipped code, zero prior governance — points the mode proposal at **brownfield**, with the ambiguity gate below still offered rather than assumed silently.

Two concrete brownfield tensions are already visible from the files read and will be carried into the plan as named conflicts rather than smoothed over:
- README: "the service runs no background jobs — nothing runs between requests" vs. `src/db.js`'s `setInterval` nightly purge running inside the web process.
- README: "Secrets never appear in logs" vs. `src/server.js`'s request-logger printing `key=${req.header('x-api-key')}` (the API key itself) on every request.

---

## Phase 0 — Load the binding schema

- **Does:** Read `plugins/mochiko/schemas/setup.yaml` raw and in full (already done above), plus `plugins/mochiko/schemas/command-labels.yaml` for the label vocabulary. Confirms the six fail-conditions are intact (pre-ratification authoring, unclosed trace, author-graded output, an uncovered floor category, no acceptance, no feature map at close) — count matches 6, so no halt-and-surface is needed.
- **Reads:** the two schema files above.
- **Writes:** nothing.
- **Seats/skills:** none — this is the lead's own read.
- **Gate:** none.

## Phase 1 — Workspace recon and mode proposal

- **Does:** Inventory the repo root and read `package.json`, `README.md`, and the four source/test files to characterize the project (already done above: a small Express + better-sqlite3 link-saving service, single static API key, no CI config visible, no prior `.mochiko`/`.claude` artifacts). Checks for a superseded `${memory_dir}/constitution.md` on disk (none found, so no deletion needed). Proposes **brownfield** as the run mode.
- **Reads:** repo root listing, `package.json`, `README.md`, `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`.
- **Writes:** nothing.
- **Seats/skills:** none yet — lead-only recon. Locate/enumerate style reads (file listing) would ordinarily route to a cheap `Explore` subagent under model tiering; the handful of interpretive reads above (spotting the two conflicts) stay on the session tier because they require judgment, not enumeration.
- **Gate — mode ruling:** Present the proposal ("this looks brownfield — existing shipped service, no prior governance") and ask the user to confirm or override.
  - *If confirmed brownfield:* proceed to Phase 2 (codebase analysis) with full brownfield handling.
  - *If user rules greenfield instead* (treat prior code as out of scope): skip Phase 2's analysis-driven synthesis seeding and the feature-map reconstruction in Phase 11; scaffold an empty `FEATURES.md` instead, and defer product-baseline bootstrapping to the first `/mochiko:implement` run's design phase.
  - *If user rules amend* (only sensible if a synthesis already exists): not applicable here since no `.mochiko/memory/governance-intent.md` exists yet — the plan would report this branch as inapplicable and fall back to the brownfield/greenfield choice.

## Phase 2 — Brownfield codebase analysis

- **Does:** Runs the codebase-analysis procedure (skill: `mochiko:analysis-codebase`) to produce a structured baseline read of the Linkjar service — stack facts (Node ≥20, Express, better-sqlite3, ESLint, `node --test`), delivered surfaces (four routes: `POST /links`, `GET /links`, `DELETE /links/:id`, `GET /health`), the auth model (single shared static API key via `LINKJAR_API_KEY`), the persistence model (single SQLite file, in-process nightly purge job), and gaps against the README's own stated conventions (the two conflicts already spotted).
- **Reads:** the same four source files plus `package.json`/`README.md` again at analysis depth; would also run the skill's deterministic stack-detection script.
- **Writes:** `.mochiko/memory/codebase-analysis.md`.
- **Seats/skills:** `mochiko:analysis-codebase`, executed by a `mochiko:tech-lead` seat (owns "runs the codebase analysis a brownfield governance set is built on"). Enumeration sub-steps (walking `src/`, `test/`) tier down to a haiku-model `Explore` subagent; the interpretive write of the analysis file stays on the tech-lead seat.
- **Gate:** none directly, but the two detected conflicts are carried forward as flagged items for Phase 4's user-conflict gate rather than resolved here.

## Phase 3 — Interrogation (inline, lead-run)

- **Does:** The lead runs the interrogation itself, inline — never delegated to a subagent — working the agenda's dimensions adaptively (skill: `mochiko:analysis-iterative`), then the catalog deck card by card, recommend-then-arbitrate. Concretely for Linkjar this would surface cards on: API-key/secret handling (given the logged-key finding), background-job policy (given the purge-interval finding), data-sensitivity for stored URLs/tags, testing discipline (README already claims "every route covered by a test before it ships" — verify this holds, e.g. does `test/links.test.js` cover `DELETE`/`GET /health`?), and whether a knowledge-management module should be adopted.
- **Reads:** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`; cross-references `.mochiko/memory/codebase-analysis.md` from Phase 2.
- **Writes:** nothing durable yet — working notes only, feeding the synthesis draft in Phase 4.
- **Seats/skills:** `mochiko:analysis-iterative`, run by the lead in-session (not spawned out).
- **Gate — every card ruling, module ruling, and waiver is the user's:** each dimension and each deck card is arbitrated with the user one at a time (recommend, then the user decides accept/modify/reject/waive). No aggregate rubber-stamp.
- **Gate — reality vs. declared intent conflicts are confronted in the open:** the two concrete conflicts (background job vs. "no background jobs"; logged API key vs. "secrets never appear in logs") are presented explicitly, not silently resolved either direction. Branches per conflict:
  - *User rules "fix the principle to match reality"* → the synthesis encodes the as-built behavior (e.g., permits an in-process scheduled purge) and flags the README as needing a follow-up correction (out of scope for this run to edit).
  - *User rules "the principle stands, code is in violation"* → the synthesis keeps the stricter principle and the conflict is recorded as a known open violation for a future fix, not silently waived.
  - *User defers* → recorded as an open item in the synthesis, not blocking ratification but visible in the trace.

## Phase 4 — Freeze the draft intent synthesis

- **Does:** Consolidates all card rulings, module waivers, and conflict dispositions from Phase 3 into a single frozen draft synthesis document, in the GI-XXX namespace, from the governance-intent template.
- **Reads:** its own working notes from Phase 3.
- **Writes:** `.mochiko/memory/governance-intent.md` (draft state, not yet ratified).
- **Seats/skills:** lead-authored directly (the synthesis is explicitly "your pen" — the lead's own artifact, not delegated).
- **Gate:** none yet — ratification is deferred to after the cold stress-test in Phase 6.

## Phase 5 — Spawn the cold stress-test seat (blind dispatch)

- **Does:** Spawns an independent, cold reviewer seat to stress-test the draft synthesis before ratification (skill: `mochiko:review-governance-intent`), using a two-message blind dispatch: the first message gives only the setup topic ("Linkjar — a small link-saving HTTP service, first-time governance setup") and the goal, withholding the path to `governance-intent.md`, so the seat builds its Phase-0 angle map with no knowledge of what the interrogation actually concluded. Only after that angle map comes back does the second message hand over the synthesis path for the cold read itself.
- **Reads (by the spawned seat, second message onward):** `.mochiko/memory/governance-intent.md`.
- **Writes:** the reviewer's findings, held in-session (not a durable file per the schema — feeds Phase 6's gate).
- **Seats/skills:** `mochiko:review-governance-intent`, executed by a `mochiko:devils-advocate` seat (adversarial, blind-map style review matches the stress-test framing) — never the session lead, and never a participant who touched the interrogation.
- **Gate:** none directly here; the alternative is a user-recorded waiver of the stress-test entirely, which the user could invoke at this point instead of spawning the seat. If waived, the plan proceeds straight to Phase 7 with the waiver logged.

## Phase 6 — Route surviving coverage gaps

- **Does:** Any coverage finding that survives the cold stress-test is treated as a question about the *setup's scope*, not a card-level nitpick — each surviving gap is presented to the user as a candidate topic.
- **Reads:** the stress-test seat's findings from Phase 5.
- **Writes:** none yet (routing only); a chosen "explore now" branch writes back into the GI-XXX namespace.
- **Seats/skills:** none new.
- **Gate — per surviving gap, the user rules the path:**
  - *Explore now* → re-enters `mochiko:analysis-iterative` on that specific angle; the newly elicited intent is appended into the synthesis under a fresh GI-XXX entry, and the synthesis returns to a draft state (re-triggering Phase 5 lightly, scoped to just the delta, per the skill's bounded delta-pass mode) before ratification is offered again.
  - *Rule inline* → the user answers the gap directly without a full re-interrogation pass; folded into the synthesis immediately.
  - *Defer* → recorded as an explicitly open item, not blocking ratification.
  - *(A gap that turns out to overlap an existing agenda dimension instead follows the ordinary interrogation-follow-up path from Phase 3, not this routing.)*

## Phase 7 — Ratification gate

- **Does:** Presents the final, gap-routed synthesis for the user's ratification. This is the hard floor the entire run pivots on: **no governance surface is authored before this gate closes.**
- **Reads:** the finalized `.mochiko/memory/governance-intent.md`.
- **Writes:** none (a ratification marker/timestamp inside the same file, if the template calls for one).
- **Seats/skills:** none — this is the user's own act.
- **Gate — synthesis ratification is the user's, as plain blocking text (never a timed prompt):**
  - *Ratified* → proceed to Phase 8.
  - *Not ratified / changes requested* → loop back to Phase 3 (or directly to Phase 4 if it's just a wording fix) on the flagged items only, then re-offer ratification. This loop can repeat; the run does not advance to authoring under any circumstance until this gate closes.

## Phase 8 — Author the governance surface set

- **Does:** With ratified intent in hand, authors the actual governance surfaces (skill: `mochiko:authoring-constitution`): the marked governance region inside `CLAUDE.md` (between `<!-- mochiko:governance:begin -->` / `end` markers), the per-path rule files under `.claude/rules/mochiko/*.md`, skill pointers, and the governance ledger. Preserves any pre-existing carve-outs verbatim (not applicable here — no prior `CLAUDE.md` exists, so no carve-outs to protect yet). If the interrogation adopted a knowledge-management module (Phase 3), also scaffolds `templates/constitution-modules/knowledge-management.md` into the project-pinned `.mochiko/memory/knowledge-management.md`.
- **Reads:** `.mochiko/memory/governance-intent.md` (ratified), the constitution-authoring templates/schemas under the plugin root.
- **Writes:** `CLAUDE.md` (governance region only), `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, the trace summary, and conditionally `.mochiko/memory/knowledge-management.md`.
- **Seats/skills:** `mochiko:authoring-constitution`, executed by a `mochiko:tech-lead` seat. This seat plans its authoring approach first and works only on a lead-approved plan (producer seats always plan-first; grading/fact-finding seats are exempt from this).
- **Gate — plan approval:** the tech-lead seat's authoring plan is shown to the user before it writes anything; the user approves, requests changes, or rejects the approach. Branches mirror Phase 7's loop (approve → write; changes → replan; reject → return to intent-level discussion, potentially reopening Phase 6).

## Phase 9 — Independent grading of the authored surfaces

- **Does:** Grades the drafted governance surface set against the quality checklist (skill: `mochiko:validation-constitution`) — reading the actual `CLAUDE.md` region and rule files themselves, never the tech-lead seat's own report of what it did. Defaults to FAIL until proven otherwise.
- **Reads:** the files written in Phase 8, plus the ratified synthesis (to check trace closure between ratified intent and authored text) and the knowledge-management never-overwrite floor if that module was scaffolded.
- **Writes:** a grading verdict (pass/fail + fix list), held for the user, not silently auto-applied.
- **Seats/skills:** `mochiko:validation-constitution`, executed by a `mochiko:validator` seat — a different seat instance than the Phase 8 author, satisfying the author-never-grades-own-output floor.
- **Gate:** none directly (grading is not itself a user decision), but a FAIL verdict loops back to Phase 8 for revision and re-grading. This can repeat; the run cannot proceed to Phase 11's final acceptance while the grade stands at FAIL.

## Phase 10 — Cross-seat transport discipline (applies throughout Phases 5–11)

- **Does:** Because this run composes more than one seat (cold stress-test seat, authoring seat, grading seat, and the Phase 11 feature-map/architecture seats below) and multiple seats write to shared surfaces, each seat is briefed with only the scoped context it needs (e.g., the cold seat never sees the synthesis path until its second message), writes to any one file are single-writer per phase (only the tech-lead seat touches `CLAUDE.md`/`.claude/rules/mochiko/`; only the product-manager seat touches `FEATURES.md`; only the architect seat touches `spine.md`), and the lead waits for each seat's fan-in confirmation before advancing to the next phase.
- **Reads/writes:** none of its own — this is a cross-cutting discipline applied to the writes already described in the other phases.
- **Seats/skills:** governs, does not add, seats.
- **Gate:** none — non-waivable mechanical discipline, not a user decision point.

## Phase 11 — Feature-map and architecture-store landing (brownfield path)

- **Does (feature map):** Extends the Phase 2 codebase analysis into a feature-map reconstruction — deriving delivered capabilities from the code. For Linkjar this would likely surface entries such as "Link capture" (`POST /links`), "Link listing and tagging" (`GET /links` with tag filter), "Link removal" (`DELETE /links/:id`), each marked `delivered` and tagged reconstructed-from-code.
  **Does (architecture store):** Unconditionally scaffolds `.mochiko/product/architecture/spine.md` as a header-only stub declaring the product's surface-type scope (here: `backend-service` only — no frontend/mobile/desktop code exists) and an empty `concerns.md` beside it. No architectural judgment content is written — only the scope header, since ruled architectural content is explicitly out of scope for this run and belongs to the first `/mochiko:architecture` visit.
  **Does (baselines, per the `Assumed` sizing note):** Since this is brownfield, also bootstraps `.mochiko/product/` baseline files (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`) from the delivered code, with `ARCHITECTURE.md` at repo root becoming the store's derived index.
- **Reads:** `.mochiko/memory/codebase-analysis.md`, the feature-map/feature-entry templates or their raw schemas if the CLI binary is unavailable.
- **Writes:** `FEATURES.md` (repo root index) + per-capability entries under `.mochiko/features/`; `.mochiko/product/architecture/spine.md` + `concerns.md`; `.mochiko/product/data-model.md`, `.mochiko/product/contracts/`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/quickstart.md`; `ARCHITECTURE.md`.
- **Seats/skills:** `mochiko:authoring-feature-map` executed by a `mochiko:product-manager` seat; `mochiko:authoring-architecture-store` (scope-header portion only) executed by a `mochiko:principal-architect` seat for the scope declaration, plus baseline bootstrapping likely folded into the tech-lead or architect seat's brownfield pass.
- **Gate — feature-map confirmation, entry by entry:** each reconstructed capability entry (Link capture / Link listing+tagging / Link removal) is shown to the user individually for confirmation, correction, or rejection before the map is considered landed. A rejected entry is dropped or reworded per the user's correction, not silently kept.
  - *(If Phase 1's gate had instead landed on greenfield, this whole phase collapses to: scaffold an empty `FEATURES.md`, no reconstruction, no user-map-confirmation gate, and baselines are explicitly deferred rather than bootstrapped.)*

## Phase 12 — Trace closure, version bump, and final acceptance

- **Does:** Verifies the trace closes end-to-end — every authored line in `CLAUDE.md`'s governance region and the `.claude/rules/mochiko/*.md` files traces back to a specific ratified GI-XXX entry, with no orphaned authored content and no ratified intent left unexpressed. Bumps the governance region's semver. Assembles the trace summary for the user.
- **Reads:** the ratified synthesis, the authored surfaces, the Phase 9 grading verdict.
- **Writes:** a version bump inside the governance region of `CLAUDE.md`; the finalized trace summary (likely appended to or alongside `.mochiko/memory/governance-ledger.md`).
- **Seats/skills:** lead-assembled from the prior phases' outputs; no new seat needed unless the trace-closure check itself is delegated back to the Phase 9 validator seat for a final confirmation pass.
- **Gate — final acceptance, flagged proposal by flagged proposal:** the user is walked through every flagged proposal (each interrogation card ruling, each conflict disposition, each authored surface, the feature-map entries, the architecture scope line) as plain blocking text and accepts or rejects each individually. Only once every flag is accepted does the run close.
  - *If any flag is rejected at this final gate:* the specific rejected item loops back to its originating phase (a rejected card ruling → Phase 3/4; a rejected authored surface → Phase 8; a rejected feature-map entry → Phase 11) rather than restarting the whole run.

## Phase 13 — Close-out reporting

- **Does:** Reports the outcome in the project's registered output style: what was ratified, what was authored, the grading verdict, the feature-map and architecture-scope landing, and the version bump. States the two peer next-step doors — `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline — as equally-ordered options, plus `/mochiko:brainstorm` if a knowledge-management module was adopted in Phase 3.
- **Reads:** the final state of all files written above.
- **Writes:** nothing new — reporting only.
- **Seats/skills:** none — lead's closing message.
- **Gate:** none.

---

### Done-condition check (why this plan satisfies the fixed goal)

- No surface is authored before Phase 7's ratification gate closes (Phase 8 strictly follows Phase 7).
- The trace from ratified intent (Phase 4/7) to authored surfaces (Phase 8) is verified closed in Phase 12, independently confirmed by a non-author grading seat in Phase 9.
- The governance region's semver is bumped in Phase 12.
- The user accepts the full set with the trace summary in hand in Phase 12's final gate.
- The feature map is either brownfield-reconstructed and user-confirmed (Phase 11, the expected path given the code found) or, on the alternate greenfield ruling from Phase 1, left as an empty scaffold with baselines deferred rather than bootstrapped.
- The architecture store's `spine.md` stub and its scope line are written regardless of which path Phase 1 resolves to (Phase 11 covers both branches).