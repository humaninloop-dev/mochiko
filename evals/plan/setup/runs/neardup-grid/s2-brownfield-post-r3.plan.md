# Action Plan — `/mochiko:setup` (plan-only; nothing executed)

Rules loaded before this plan was drafted: `plugins/mochiko/schemas/setup.yaml` (full) and `plugins/mochiko/schemas/common.yaml` (full, for every `extends:` stub), plus `command-labels.yaml` for label meaning. Workspace scan shows only the `plugins/mochiko/` plugin tree itself — no `CLAUDE.md`, no `.mochiko/`, no application source, no git metadata. `` was empty, so the mode is proposed rather than given.

---

## Phase 0 — Rule load (already performed)
- **Done:** Read `setup.yaml` and `common.yaml` raw and in full; resolved every `extends:` stub and every `${var}` placeholder.
- **Read:** the two schema files above; label registry for cross-checking `labels:` values.
- **Written:** nothing.
- **Seats:** lead only.
- **Gate:** none.

## Phase 1 — Workspace read & mode proposal
- **Done:** Inspect the workspace for signals of an existing product (source files, `CLAUDE.md`, `.mochiko/`, a governance ledger) versus a bare checkout.
- **Read:** repo file tree (already globbed — only plugin content found, no app code, no prior governance, no `.mochiko/memory/`).
- **Written:** nothing yet.
- **Seats:** lead. A pure locate/enumerate sweep like this is the kind of read `common.model-tiering` routes to a haiku-model `Explore` subagent rather than the session tier; the mode *judgment* itself stays on the session tier as interpretive.
- **Gate (`setup.user-mode-ruling`, reservation):** the workspace shows no existing product code and no prior governance, so the concrete proposal for this invocation is **greenfield**. This is presented to the user as a proposal, not a default — ambiguity resolution is reserved to the user.
  - *If the user confirms greenfield:* proceed with the greenfield branch in every later phase (empty feature-map scaffold, baselines deferred, no codebase analysis).
  - *If the user rules brownfield instead* (e.g., they intend to point this run at code that lives outside what was scanned, or there's a repo they'll attach): switch to the brownfield branch — Phase 4 adds `mochiko:analysis-codebase`, Phase 11 becomes feature-map reconstruction with entry-by-entry confirmation, and `setup.user-conflict-rulings` activates for any reality-vs-declared-intent conflict.
  - *If the user rules amend:* skip interrogation-from-scratch in favor of amending the existing `governance-intent.md` (none exists here, so this branch would first have to explain that no durable synthesis is on disk to amend, and fall back to asking whether they mean greenfield instead).

## Phase 2 — Superseded-artifact check
- **Done:** Check for a legacy `.mochiko/memory/constitution.md`.
- **Read:** `.mochiko/memory/` (does not exist in this workspace).
- **Written:** nothing — none found, so nothing to delete; this would be stated in one line per `setup.constitution-superseded` if it had existed.
- **Gate:** none.

## Phase 3 — Seat wiring for this run
- **Done:** Decide teammates/subagents per seat (`setup.staffing-latitude` — lead's call). Proposed wiring for this invocation:
  - **Interrogation:** run personally, inline, by the lead — never delegated (`setup.interrogation-inline` is a duty on the lead, not a subagent).
  - **Pre-ratification cold stress-test seat:** an independent subagent (e.g. `mochiko:devils-advocate` or a fresh instance running `mochiko:review-governance-intent`), spawned blind.
  - **Governance-surface producer:** `mochiko:tech-lead` (owns authoring/updating the governance surface via `mochiko:authoring-constitution`).
  - **Independent grader:** `mochiko:validator` running `mochiko:validation-constitution` — never the producer, satisfying `common.author-grader-default-fail`.
  - **Feature-map / architecture-store scaffolds:** mechanical, schema-shaped writes; lead performs them directly per the skill grammar (`mochiko:authoring-feature-map`, `mochiko:authoring-architecture-store`), since no judgment content is being ruled here (`setup.store-ruled-content-never-here`).
- **Written:** nothing (planning only).
- **Cross-cutting binding:** the moment a second seat exists, `seats: multi` fires (`setup.transport-floor`, floor, non-waivable) — all cross-seat messaging and any shared-write surface (the ledger, the surface files) route through `mochiko:patterns-transport-floor`'s composition/messaging discipline for the rest of the run.
- **Gate:** none directly, but this wiring is disclosed to the user as part of the run's opening statement.

## Phase 4 — Interrogation inputs
- **Done:** Load the fixed interrogation materials.
- **Read:** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`.
  - *(Brownfield/amend branch only, not taken here under the greenfield proposal):* also run `mochiko:analysis-codebase`, writing `.mochiko/memory/codebase-analysis.md`.
- **Written:** nothing under the greenfield branch.
- **Seats:** lead (interpretive read, session tier).
- **Gate:** none.

## Phase 5 — Inline interrogation
- **Done:** Work the agenda's dimensions adaptively via `mochiko:analysis-iterative`, then the catalog deck card by card, recommend-then-arbitrate.
- **Read:** agenda + deck (from Phase 4); user responses as they're given.
- **Written:** nothing yet — this phase produces the draft content that Phase 6 persists.
- **Seats:** lead only, inline (`setup.interrogation-inline`).
- **Gate (`setup.user-card-rulings`, reservation, repeated per card):** every card ruling, module adoption/decline, and waiver is the user's — described, never assumed, for each card.
  - *Adopt path:* the module/principle enters the draft synthesis.
  - *Decline path:* the module is left out; if it's a floor category, its absence must later carry a recorded waiver or the run fails at close (`setup.fail.floor-category-uncovered`).
  - *Waiver path:* recorded verbatim as the user's waiver, feeding the trace.

## Phase 6 — Synthesis authored and persisted
- **Done:** Compose the interrogation's outcome into the durable synthesis.
- **Read:** `plugins/mochiko/schemas/governance-intent.yaml` (schema fallback, since no `mochiko-cli` binary is confirmed present in this workspace) as the first-class template source.
- **Written:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace) — this is the amend baseline going forward.
- **Seats:** lead (author of the synthesis — this is explicitly called out as "your pen" in `setup.stress-test-cold-seat`).
- **Gate:** none yet — ratification comes after the stress-test in Phase 7–9.

## Phase 7 — Pre-ratification cold stress-test
- **Done:** Spawn the independent stress-test seat in **two messages**, per `setup.blind-map-dispatch` (floor, non-waivable):
  1. First message: setup topic / project identity / goal only — no path to `governance-intent.md`. The seat builds its Phase 0 blind angle map on that alone.
  2. Only after the map returns: send the synthesis path, and the cold read proceeds via `mochiko:review-governance-intent`.
- **Read (by the stress-test seat):** `.mochiko/memory/governance-intent.md`, once unblinded.
- **Written:** a findings/verdict artifact returned by the seat (not a durable file by default — held in-run unless the skill's procedure calls for a persisted report).
- **Seats:** one independent cold seat (or a pair, each building its map independently, per `setup.blind-map-dispatch`).
- **Gate (`setup.stress-test-cold-seat`):** this step is mandatory unless the user records an explicit waiver of the cold seat.
  - *If the user allows the cold seat to run:* proceed as above.
  - *If the user waives it:* record the waiver verbatim in the trace, and skip straight to Phase 9 (ratification) — this is a materially riskier branch since no independent stress-test occurred, but it's the user's call to make.

## Phase 8 — Coverage-survivor routing
- **Done:** Any coverage finding that survives the stress-test (i.e., a real gap, not refuted) is presented as a candidate **scope** topic, never framed as a card-level nitpick.
- **Read:** the stress-test's surviving findings.
- **Written:** nothing yet — contingent on the user's ruling below.
- **Gate (`setup.coverage-survivor-routing`, routing):** for each surviving gap, the user rules the path:
  - *Explore now:* re-enter `mochiko:analysis-iterative` on that angle; anything newly elicited lands in the GI-XXX namespace (back to Phase 5/6 for that slice).
  - *Rule inline:* the user decides the point directly without a full re-elicitation; folded into the synthesis.
  - *Defer:* noted and left out of this run's scope, folded into the trace as a known, deliberately deferred gap.
  - *(If the gap actually overlaps an agenda dimension already covered, it instead takes the ordinary interrogation-follow-up path rather than this routing.)*

## Phase 9 — Ratification gate
- **Done:** Present the (possibly revised) synthesis for ratification.
- **Read:** the final `.mochiko/memory/governance-intent.md`.
- **Written:** nothing new — ratification is a state, not a file mutation, though the synthesis file itself may have been touched by Phase 8 revisions.
- **Gate (`setup.gate-synthesis-ratification`, floor — this is the goal's hinge: no surface may be authored before this fires):**
  - *If the user ratifies:* proceed to Phase 10 (surface authoring may begin only now).
  - *If the user requests changes:* loop back to Phase 5/6/8 as scoped by the requested change, then re-present ratification. No surface authoring occurs in this branch.
  - *If the user declines outright:* the run halts short of authoring anything; `setup.fail.pre-ratification-authoring` and `setup.fail.no-acceptance` remain unresolved failure conditions and the run is reported as incomplete rather than forced forward.

## Phase 10 — Governance surface authoring
- **Done:** The producer seat (`mochiko:tech-lead`) plans first (`common.plan-approval-producers` — any artifact-writing seat plans and works only on a lead-approved plan), the lead approves the plan, then the seat authors the surface set via `mochiko:authoring-constitution`.
- **Read:** ratified `governance-intent.md`; any existing `CLAUDE.md` (none here) to check for the marked region and the two carve-outs (`mochiko:domain-registry` block, `mochiko:output-style` pair) — none exist yet in this workspace, so nothing to preserve, only to create.
- **Written:**
  - `CLAUDE.md` — the marked governance region (`<!-- mochiko:governance:begin/end -->`), idempotently generated; nothing outside the markers touched (none exists outside them here).
  - `.claude/rules/mochiko/*.md` — `paths`-scoped rule files, including the domain-registry and output-style carve-out files if those modules are adopted.
  - `.mochiko/memory/governance-ledger.md` — the durable ledger.
  - A trace summary tying every authored surface back to its ratified GI-XXX ruling.
  - If the knowledge-management module was adopted during interrogation: `.claude/rules/mochiko/knowledge-management.md`-shaped scaffold per `templates/constitution-modules/knowledge-management.md`, plus the project-pinned copy at `.mochiko/memory/knowledge-management.md` (write-once; its never-overwrite floor holds on any future run).
- **Seats:** `mochiko:tech-lead` (producer) under lead-approved plan.
- **Gate:** the plan-approval step above is itself a lightweight gate — the lead confirms the producer's plan before it writes. No user-facing gate yet; that's Phase 15.

## Phase 11 — Feature map landing
- **Done (greenfield branch, per the Phase 1 proposal):** scaffold an empty `FEATURES.md` index (`setup.feature-map-greenfield`).
- **Read:** `plugins/mochiko/schemas/features-index.yaml` (schema fallback for shape).
- **Written:** `FEATURES.md` (empty index) at repo root. Never overwrites an existing map (`setup.map-never-overwrite`, floor) — moot here since none exists.
- **Seats:** lead (mechanical scaffold).
- **Gate:** none on greenfield. *(Brownfield branch, not taken: reconstruction from `codebase-analysis.md` into `FEATURES.md` + `.mochiko/features/` entries marked `delivered`/reconstructed-from-code, gated on `setup.user-map-confirmation` — user confirms entry by entry.)*

## Phase 12 — Product baselines
- **Done:** Note the baseline-seeding disposition rather than write anything.
- **Read:** none additional.
- **Written:** nothing — under greenfield, `setup.baselines-bootstrap` explicitly defers `data-model.md` / `contracts/` / `constraints-and-decisions.md` / `quickstart.md` to the first `/mochiko:implement` run's design phase. *(Brownfield branch would bootstrap these at `.mochiko/product/` from delivered code instead.)*
- **Gate:** none.

## Phase 13 — Architecture store scaffold (unconditional on every path)
- **Done:** Create the store's skeleton regardless of mode.
- **Read:** `plugins/mochiko/schemas/architecture-store.yaml` for the spine/concerns shape; workspace signals for which surface types the product carries (backend-service / frontend-web / mobile / desktop / full-stack / monorepo) to populate the `Scope:` line.
- **Written:**
  - `.mochiko/product/architecture/` (created — does not exist).
  - `spine.md` stub, header carrying a `Scope:` line, write-if-absent only.
  - empty `concerns.md` beside it.
  - No topology or ruled content beyond the header (`setup.store-ruled-content-never-here`, floor) — that's reserved for the first `/mochiko:architecture` visit.
- **Seats:** lead, following `mochiko:authoring-architecture-store` grammar exactly (referenced, not reinvented).
- **Gate:** none — this is a non-waivable but non-judgmental scaffold; the user may later override the `Scope:` line at the architecture desk.

## Phase 14 — Independent grading of the authored set
- **Done:** `mochiko:validator` reads the actual files on disk (not the producer's report) against `mochiko:validation-constitution`'s checklist, defaulting to FAIL until every clause clears.
- **Read:** `CLAUDE.md` governance region, every `.claude/rules/mochiko/*.md`, `governance-ledger.md`, the trace summary; cross-checked against ratified `governance-intent.md`.
- **Written:** a grading verdict (pass/fail + fix list), held in-run or logged, not a product surface.
- **Seats:** `mochiko:validator` — never `mochiko:tech-lead` (the producer), satisfying `setup.fail.author-graded` / `common.author-grader-default-fail`.
- **Gate:** none user-facing yet, but a FAIL verdict here loops back to Phase 10 for revision before acceptance is ever offered.

## Phase 15 — Final acceptance gate
- **Done:** Present the surface set to the user, flagged proposal by flagged proposal, as plain blocking text (`common.acceptance-plain-text` — never a timed prompt).
- **Read:** the graded surface set from Phase 14.
- **Written:** nothing new by this step itself — acceptance is recorded, not authored.
- **Gate (`setup.gate-final-acceptance`, floor):**
  - *If the user accepts everything:* proceed to close (Phase 16).
  - *If the user flags specific proposals for change:* only those items loop back to Phase 10 (revision), then re-grade (Phase 14) before re-presenting just the changed items.
  - *If the user rejects the run outright:* the run ends without acceptance; `setup.fail.no-acceptance` stands, and this run is reported as not-done rather than force-closed.

## Phase 16 — Close and report
- **Done:** Bump the governance region's semver; finalize the trace summary; suggest (never execute) a commit.
- **Read:** the accepted surface set.
- **Written:** semver bump inside the `CLAUDE.md` governance region; final trace summary content (likely folded into `governance-ledger.md` or the region itself, per `mochiko:authoring-constitution`'s composition rules).
- **Seats:** lead reports; `common.no-git-mutations` (floor) means a commit is suggested in text only — no `git` command is run, nothing is pushed.
- **Reporting:** register per `templates/output-style.md`; next-step line offered per `setup.next-step` — `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product baseline, named as peer doors (plus `/mochiko:brainstorm` if knowledge-management was adopted).
- **Gate:** none — this phase only fires once Phase 15 has already gated on acceptance.

## Phase 17 — Not-done / default-FAIL self-check
Before the run is declared complete, all six `kind: fail` nodes are checked against what actually happened (schema count cross-checked at 6, matching the `.md`'s Not-done line — no drift found in this schema read):
1. `setup.fail.pre-ratification-authoring` — no surface was authored before Phase 9's ratification fired.
2. `setup.fail.unclosed-trace` — every authored surface in Phase 10/16 traces to a ratified GI-XXX item.
3. `setup.fail.author-graded` — Phase 14's grader was never the Phase 10 producer.
4. `setup.fail.floor-category-uncovered` — every Essential Floor category carries either an adopted principle or a recorded waiver from Phase 5/8.
5. `setup.fail.no-acceptance` — Phase 15 acceptance was actually given, not assumed.
6. `setup.fail.no-feature-map` — `FEATURES.md` exists at close (Phase 11's greenfield scaffold, or brownfield's confirmed reconstruction, or an amend's surfaced-and-offered map).

Any one of these standing open at close fails the run regardless of how far the other phases got.