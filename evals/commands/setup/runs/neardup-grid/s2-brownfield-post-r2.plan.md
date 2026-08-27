# Action Plan — `/mochiko:setup` (plan-only, no execution)

**Scope check performed before planning:** Read `plugins/mochiko/schemas/setup.yaml` and `plugins/mochiko/schemas/common.yaml` raw, in full (per the command's first-action instruction), plus `plugins/mochiko/schemas/command-labels.yaml` for label resolution. Counted the `kind: fail` nodes in `setup.sec.fail-conditions`: 6 (`pre-ratification-authoring`, `unclosed-trace`, `author-graded`, `floor-category-uncovered`, `no-acceptance`, `no-feature-map`) — matches the `.md`'s hard-coded Not-done count, so no halt is triggered on that check.

**Workspace read performed to make this plan concrete:** no `.git`, no `.mochiko/`, no `CLAUDE.md`, no `.claude/`. `package.json` names the project `linkjar` (Express + better-sqlite3, `src/server.js`, `src/db.js`, `src/auth.js`, one test file). `README.md` describes a running HTTP link-saving service with explicit operating claims ("every route is covered by a test before it ships," "secrets never appear in logs," "SQLite is the only datastore," "runs no background jobs"). This is a real, already-shipping codebase with zero prior governance — the concrete signal the mode proposal in Phase 1 is built from.

---

## Phase 1 — Mode proposal (gate)

**Does:** Proposes the run's mode from the workspace evidence above: existing `src/`/`test/` code, a `package.json` with real dependencies, a README describing production behavior, but no `.mochiko/`, no `CLAUDE.md`, and no git history to inspect — this is a **brownfield** project standing up governance for the first time, not a from-scratch **greenfield** start and not an **amend** of an existing surface set (there's nothing to amend).

**Reads:** workspace root listing, `package.json`, `README.md`, `src/*.js`, `test/*.js` (already done above).

**Writes:** none.

**Gate — `setup.user-mode-ruling` (reservation, user-gate):** presents "brownfield" as the proposed mode with the evidence above and asks the user to confirm or override.
- **Ruling: confirm brownfield** → proceed to Phase 2 (codebase analysis) with all `when: {mode: [brownfield]}` rules active (`setup.user-conflict-rulings`, `setup.user-map-confirmation`, `setup.feature-map-brownfield`, `setup.baselines-bootstrap`'s brownfield branch).
- **Ruling: override to greenfield** → skip Phase 2's codebase analysis, run interrogation from a blank slate, and at close scaffold an empty `FEATURES.md` (`setup.feature-map-greenfield`) instead of reconstructing one; product baselines are left to seed at the first `/mochiko:implement` design phase (`setup.baselines-bootstrap`).
- **Ruling: override to amend** → refuses cleanly, since there is no existing surface set on disk to amend; falls back to re-asking between brownfield/greenfield.

For the rest of this plan, the brownfield branch is carried forward as the illustrative path, with the greenfield divergence noted where it applies.

---

## Phase 2 — Brownfield codebase analysis

**Does:** Runs the `mochiko:analysis-codebase` skill (pointer on `setup.interrogation-inputs`) over `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, `package.json`, and `README.md` to build a present/partial/absent read of the existing project and a deterministic stack baseline (routes, the SQLite schema, the API-key auth check, test coverage shape, the "no background jobs" claim).

**Reads:** all files under `src/` and `test/`, `package.json`, `README.md`.

**Writes:** `.mochiko/memory/codebase-analysis.md`.

**Seats/skills:** `mochiko:analysis-codebase` as a fact-finding seat — exempt from `setup.plan-approval-producers` (fact-finding seats are exempt). Its own file-locate/enumerate sub-reads route through a native `Explore` subagent spawned `model: haiku` per `mochiko:patterns-model-tiering`; the interpretive synthesis (what the analysis *means* for governance) stays on the session tier.

**Gate:** none — this phase is preparatory input, not a ruling point.

---

## Phase 3 — Interrogation (inline, agenda → catalog, card by card)

**Does:** I run the interrogation myself, inline (`setup.interrogation-inline` — never delegated to a seat). Two stages:
1. Works the agenda's dimensions adaptively via `mochiko:analysis-iterative`, informed by the Phase 2 analysis (e.g., probing whether "SQLite only, no background jobs" and "secrets never appear in logs" are meant as binding constraints or just current-state description).
2. Works the catalog deck card by card, recommend-then-arbitrate. Given linkjar is a backend HTTP service with no frontend/mobile/desktop surface, the relevant deck is `universal-floor.md` + `backend-service.md` (not the full catalog); `DOMAIN-DEPENDENCIES.md` is checked against `express`/`better-sqlite3` for any domain-triggered modules.

**Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, `.../references/catalog/universal-floor.md`, `.../references/catalog/backend-service.md`, `.../references/DOMAIN-DEPENDENCIES.md`, `.../references/ESSENTIAL-FLOOR.md` (to track which floor categories still need a principle or waiver), `.../references/COMPLIANCE-MODULES.md`, `.mochiko/memory/codebase-analysis.md`.

**Writes:** none yet — cards are worked and ruled, but the synthesis document isn't authored until Phase 5.

**Gate — `setup.user-card-rulings` (reservation, user-gate), repeated per card:** every agenda-dimension ruling, every catalog-card ruling, and every waiver (e.g., waiving a floor category rather than adopting a principle for it) is put to the user card by card.
- **Ruling: adopt/rule as recommended** → the ruling is recorded toward the synthesis.
- **Ruling: modify the recommendation** → the modified position is recorded instead.
- **Ruling: waive a floor category** → recorded as an explicit waiver (later checked at close by `setup.fail.floor-category-uncovered` — a category with neither a principle nor a recorded waiver fails the run).

---

## Phase 4 — Reality-vs-intent conflict rulings (brownfield only)

**Does:** Cross-checks Phase 3's card rulings against Phase 2's codebase analysis for contradictions — concretely, this would surface if, say, a card ruling says "background jobs are fine going forward" while the README/code show "runs no background jobs" as a hard current invariant, or if the auth story ruled in interrogation doesn't match what `src/auth.js` actually does today.

**Reads:** `.mochiko/memory/codebase-analysis.md` against the Phase 3 card rulings (no new files).

**Writes:** none — conflicts are surfaced, not silently resolved into the synthesis.

**Gate — `setup.user-conflict-rulings` (reservation, user-gate, brownfield-only):** each detected conflict is put to the user in the open.
- **Ruling: code wins, adjust the interrogation position** → the affected Phase 3 card is re-ruled.
- **Ruling: intent wins, code will need to change later** → the conflict is recorded in the synthesis as a known gap between current code and ruled intent, to be closed by later implementation work, not by this run.
- **Ruling: not actually a conflict (my read was wrong)** → dismissed, no change.

*(Greenfield path: this phase does not fire — no `codebase-analysis.md` exists to conflict against.)*

---

## Phase 5 — Author the synthesis

**Does:** Compiles every ratified card ruling from Phases 3–4 into the durable synthesis artifact, in the `GI-XXX` namespace, as the amend baseline for future runs.

**Reads:** the accumulated Phase 3/4 rulings; the governance-intent template — first tries `mochiko-cli template governance-intent` (checked above: the `mochiko-cli` binary is **absent** in this workspace), so falls back to reading `plugins/mochiko/schemas/governance-intent.yaml` raw as the first-class source of truth for the artifact's shape.

**Writes:** `.mochiko/memory/governance-intent.md` (draft, pre-ratification).

**Seats/skills:** authored by me as lead, since this is the direct output of the inline interrogation I ran — no separate authoring seat for this artifact.

**Gate:** none yet — this is a draft. Authoring any *governance surface* before ratification of this draft would trip `setup.fail.pre-ratification-authoring`, so nothing beyond this synthesis file is written past this point until Phase 7 clears.

---

## Phase 6 — Pre-ratification stress-test (cold seat)

**Does:** Before the user is asked to ratify, the draft synthesis gets an independent cold read. Per `setup.blind-map-dispatch` (floor, always delivered), this is a two-message dispatch to a cold seat running `mochiko:review-governance-intent`:
1. **Message 1** — the setup topic and project identity/goal only ("standing up governance for `linkjar`, a small Express/SQLite link-saving service"), *not* the synthesis path. The seat builds its Phase 0 angle map blind to what interrogation concluded, and that map returns before anything else is sent.
2. **Message 2** — now the path to `.mochiko/memory/governance-intent.md` is sent, and the cold read against that blind map begins.

**Reads (by the seat):** `.mochiko/memory/governance-intent.md` and whatever it independently pulls for its angle map.

**Writes:** none by this seat (grading only).

**Gate — `setup.stress-test-cold-seat` (independence, user-gate) branch:** the default is a cold seat as above.
- **User waives the cold seat explicitly** → the recorded waiver substitutes for this phase, and Phase 7's ratification proceeds without an independent stress-test finding in hand (this is the one way this phase is skipped; it is not skippable on my own judgment).

**Onward routing — `setup.coverage-survivor-routing` (routing, user-gate):** any coverage finding that survives the stress-test questions the *setup's scope*, not a single card — each gap is presented as a candidate topic.
- **Ruling: explore now** → re-enters `mochiko:analysis-iterative` on that angle; newly-elicited intent lands in the `GI-XXX` namespace, folded into the Phase 5 draft before ratification is asked for again.
- **Ruling: rule inline** → resolved directly as an additional card ruling, folded into the draft.
- **Ruling: defer** → recorded as an explicitly deferred gap, not blocking ratification.
- *(A gap that overlaps an existing agenda dimension instead takes the ordinary Phase 3 interrogation-follow-up path rather than this routing.)*

---

## Phase 7 — Synthesis ratification (gate)

**Does:** Presents the (possibly Phase-6-revised) synthesis to the user for ratification — the hinge point the whole run's authoring is gated on.

**Reads:** `.mochiko/memory/governance-intent.md` in its current state.

**Writes:** none.

**Gate — `setup.gate-synthesis-ratification` (floor, gate):** plain blocking text, never a timed prompt (`setup.acceptance-plain-text`).
- **Ruling: ratify** → Phases 8–13 (all governance-surface authoring) become permitted; ratifying here is what keeps this run off `setup.fail.pre-ratification-authoring`.
- **Ruling: request changes** → loops back to Phase 3/4 on the flagged cards only, then re-drafts (Phase 5) and re-offers ratification; no surface authoring starts in the meantime.
- **Ruling: reject outright** → the run halts here. No governance surfaces are authored. The run's done condition is not met (`setup.fail.pre-ratification-authoring` is avoided only because nothing was authored, but `setup.fail.no-acceptance` and the overall goal are unmet) — this is a legitimate stop, not a failure to force past.

---

## Phase 8 — Author the governance surface set

**Does:** With ratified intent in hand, runs `mochiko:authoring-constitution` to compose the actual surface set. First checks for a stray `.mochiko/memory/constitution.md` (`setup.constitution-superseded`) — none exists in this workspace, so nothing to delete or announce. Then authors:
- the marked governance region in `CLAUDE.md` (`<!-- mochiko:governance:begin/end -->`) — since no `CLAUDE.md` exists yet, this is a first-visit creation, not a regeneration; the region is idempotently regenerable on future runs, everything outside its markers is user content and untouched from here on (`setup.governance-region-ownership`);
- `paths`-scoped rule files under `.claude/rules/mochiko/*.md`;
- skill pointers;
- the trace-summary groundwork tying every authored line back to a `GI-XXX` ruling.

Because this is a first run, the marked carve-outs (`mochiko:domain-registry` block in the domain-layer rules file, the `mochiko:output-style` pair) are being *created* fresh here, not preserved — `setup.carve-outs-preserved` governs *later* amend runs regenerating over them, not this one.

If the ratified intent adopted the knowledge-management module (a Phase 3 card ruling), scaffolds it per `templates/constitution-modules/knowledge-management.md`, including the project-pinned copy at `.mochiko/memory/knowledge-management.md` (`setup.km-module-scaffold`; its never-overwrite floor holds on any future run).

**Reads:** `.mochiko/memory/governance-intent.md` (ratified), `plugins/mochiko/skills/authoring-constitution/references/ESSENTIAL-FLOOR.md`, `.../RFC-2119-KEYWORDS.md`, `.../COMPLIANCE-MODULES.md`, `plugins/mochiko/templates/constitution-modules/knowledge-management.md` (if adopted).

**Writes:** `CLAUDE.md` (governance region), `.claude/rules/mochiko/*.md`, `.mochiko/memory/knowledge-management.md` (if adopted).

**Seats/skills:** an authoring seat running `mochiko:authoring-constitution`, distinct from whichever seat will grade it in Phase 12 (`setup.author-grader-default-fail`, floor). Per `setup.plan-approval-producers`, this seat plans its authoring approach first and only writes on a plan I (lead) approve.

**Gate:** none directly here (the controlling gates were Phase 7 upstream and Phase 14 acceptance downstream).

---

## Phase 9 — Feature map (brownfield reconstruction)

**Does:** Extends the Phase 2 analysis into a feature-map reconstruction — for linkjar, concretely: a "save link" capability and a "list saved links" capability derived from the routes in `src/server.js`, cross-referenced with `src/db.js`'s schema and `src/auth.js`'s write-route gating — landed with `delivered` status and a reconstructed-from-code mark.

**Reads:** `.mochiko/memory/codebase-analysis.md`; template shape from `mochiko-cli template features-index` / `template feature-entry` — again the binary is absent, so falls back to reading `plugins/mochiko/schemas/features-index.yaml` and `plugins/mochiko/schemas/feature-entry.yaml` raw.

**Writes:** `FEATURES.md` (repo root), `.mochiko/features/*.md` entries.

**Seats/skills:** `mochiko:authoring-feature-map`; plans first and writes only on an approved plan (`setup.plan-approval-producers`).

**Gate — `setup.user-map-confirmation` (reservation, user-gate, brownfield-only):** the reconstructed map is confirmed entry by entry.
- **Ruling: confirm as-is** → entries stand as `delivered`.
- **Ruling: correct an entry** (e.g., a capability boundary drawn wrong, or a route that's actually dead code) → the entry is corrected before the map is considered closed.
- **Ruling: reject an entry entirely** → it's dropped from the map.
*(Never overwritten if it already existed — `setup.map-never-overwrite` — moot here since no prior map exists.)*

*(Greenfield path: this phase is replaced by `setup.feature-map-greenfield` — an empty `FEATURES.md` scaffold, no reconstruction, no per-entry confirmation gate.)*

---

## Phase 10 — Product baselines bootstrap (brownfield, `Assumed`)

**Does:** From the delivered code, bootstraps `data-model.md` (the SQLite schema in `src/db.js`), `contracts/` (the routes in `src/server.js`), `constraints-and-decisions.md` (e.g., "SQLite only, no background jobs" as a captured constraint if ratified as binding in Phase 3/4), and `quickstart.md`.

**Reads:** `src/db.js`, `src/server.js`, `src/auth.js`, `.mochiko/memory/codebase-analysis.md`, the ratified synthesis for anything elevated to a binding constraint.

**Writes:** `.mochiko/product/data-model.md`, `.mochiko/product/contracts/`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/quickstart.md`. (`ARCHITECTURE.md` stays repo-root as the store's derived index, not touched by this bootstrap directly.)

**Gate:** none separately called out beyond the general acceptance gate in Phase 14 — this is `Assumed` scope carried under the same authoring seat as Phase 9, per `setup.baselines-bootstrap`.

*(Greenfield path: skipped entirely — baselines seed instead at the first `/mochiko:implement` design phase.)*

---

## Phase 11 — Architecture store scaffold (unconditional, every path)

**Does:** Creates `.mochiko/product/architecture/` where it doesn't exist (it doesn't, here), with a `spine.md` stub whose header carries a `Scope:` line, and an empty `concerns.md` beside it. The `Scope:` line is declared write-if-absent from what surface types the product carries — for linkjar that's `backend-service` only (Express HTTP service, no frontend/mobile/desktop code present).

Explicitly **not** written here: any actual architecture topology, stance, or ruled content — `setup.store-ruled-content-never-here` (floor) reserves that for the first `/mochiko:architecture` visit. A file holding only the `Scope:` header is scaffold, not ruled content.

**Reads:** the same code/analysis used in Phase 2 to determine surface types.

**Writes:** `.mochiko/product/architecture/spine.md` (header + `Scope: backend-service` only), `.mochiko/product/architecture/concerns.md` (empty).

**Gate:** none — this is unconditional scaffold, not a ruling.

---

## Phase 12 — Independent grade of the authored surface set

**Does:** The full authored set from Phases 8–11 (governance region, rule files, feature map, baselines, store scaffold) is graded from the files themselves — never from the authoring seat's own report — by an independent validator running `mochiko:validation-constitution` (and, where useful, an empirical delivery probe via `mochiko:testing-governance-injection` to confirm `.claude/rules/mochiko/` files actually inject on their promised paths and the `CLAUDE.md` region reaches spawned agents). Defaults to FAIL absent a clean read.

Specifically checks, among the validator's checklist: every `ESSENTIAL-FLOOR.md` category has either a principle or a recorded Phase-3 waiver (else `setup.fail.floor-category-uncovered` trips), and that the intent→surface trace closes — every ratified `GI-XXX` ruling traces to something authored, and nothing authored lacks a ratified source (else `setup.fail.unclosed-trace` trips).

**Reads:** `CLAUDE.md`, `.claude/rules/mochiko/*.md`, `FEATURES.md`, `.mochiko/features/*`, `.mochiko/product/**`, `.mochiko/memory/governance-intent.md` (ratified) — the files, not summaries.

**Writes:** none (grading only) unless a FAIL loop is entered.

**Seats/skills:** a validator seat distinct from every Phase 8–11 authoring seat (`setup.fail.author-graded` trips otherwise).

**Gate (implicit, via fail-loop):**
- **Verdict: PASS** → proceed to Phase 13.
- **Verdict: FAIL** → the specific gaps are routed back to the relevant Phase 8–11 authoring seat for revision, then re-graded; this loop repeats until PASS or the user chooses to halt the run.

---

## Phase 13 — Trace summary

**Does:** Compiles the closed trace from every ratified `GI-XXX` ruling to the specific line(s) it produced across the governance region, rule files, feature map, and store scaffold — the artifact the final acceptance gate is judged against.

**Reads:** the ratified synthesis plus every Phase 8–11 output.

**Writes:** the trace summary (part of the `setup.surface-set` binding's composition, landed alongside the rest of the set per `mochiko:authoring-constitution`'s read scope).

**Gate:** none directly — feeds Phase 14.

---

## Phase 14 — Final acceptance (gate)

**Does:** Presents the complete surface set plus the trace summary to the user, flagged proposal by flagged proposal, as plain blocking text (never a timed prompt).

**Reads:** the full authored set + trace summary.

**Writes:** none yet.

**Gate — `setup.gate-final-acceptance` (floor, gate):**
- **Ruling: accept all flagged proposals** → the run proceeds to close (Phase 15); this is what satisfies `setup.fail.no-acceptance`.
- **Ruling: accept some, request changes on specific flags** → the flagged items route back to the relevant Phase 8–11 seat, re-graded (Phase 12), re-summarized (Phase 13), and re-presented for acceptance on just those flags.
- **Ruling: reject wholesale** → the run halts without acceptance; `setup.fail.no-acceptance` stands, the goal's done condition is unmet, and nothing further is closed out.

---

## Phase 15 — Close

**Does:** On acceptance: bumps the governance region's semver as part of `mochiko:authoring-constitution`'s idempotent regeneration; confirms the feature map exists (it does, from Phase 9, user-confirmed) satisfying `setup.fail.no-feature-map`'s brownfield branch; writes/updates the durable ledger; reports in the project's register (`templates/output-style.md`) with the `setup.next-step` routing: `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline, named as peer doors neither ahead of the other (plus `/mochiko:brainstorm` if knowledge-management was adopted in Phase 3).

**Reads:** the accepted surface set.

**Writes:** `.mochiko/memory/governance-ledger.md` (durable — never offered for deletion, `setup.durables-never-deleted`, alongside `.mochiko/memory/governance-intent.md`).

**Gate:** none — this is reporting, not a ruling point. Suggests a commit of everything written (`setup.no-git-mutations` — suggest only, never mutate/push); concretely, since this workspace has no `.git` at all, the suggestion would note that too, offering `git init` as the user's call rather than assuming it.

---

## Not-done set carried throughout (default FAIL if any stands at close)

1. A surface authored before Phase 7's ratification (`setup.fail.pre-ratification-authoring`).
2. An unclosed trace from ratified intent to authored surfaces (`setup.fail.unclosed-trace`).
3. The surface set graded by no one but its own author (`setup.fail.author-graded`).
4. A floor category with neither a principle nor a recorded waiver (`setup.fail.floor-category-uncovered`).
5. User acceptance not given (`setup.fail.no-acceptance`).
6. No feature map at close (`setup.fail.no-feature-map`).

Every phase above is sequenced so none of these six can be true at a successful Phase 15 close.