# Plan — `/mochiko:setup` for this workspace

**Scenario state observed (read, not modified):** no `.mochiko/` directory and no `CLAUDE.md` at repo root; a real product exists at `package.json` ("linkjar", v0.4.2, Express + better-sqlite3), `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`. No `${memory_dir}/constitution.md` on disk (nothing to supersede). Schema `plugins/mochiko/schemas/setup.yaml` was read raw in full; fail-condition count under `setup.sec.fail-conditions` = 6, matching the Adaptive Goal Protocol's hard-coded count — no halt needed. `command-labels.yaml` read to resolve label semantics.

---

## Phase 1 — Mode proposal
**Does:** Proposes a run mode from what the workspace shows, per `setup.user-mode-ruling`.
**Reads:** repo root listing, `package.json`, `src/*`, `test/*`, absence of `.mochiko/` and `CLAUDE.md`.
**Writes:** nothing.
**Basis:** delivered code with no prior mochiko governance surface set is the brownfield shape — real routes/services (`src/server.js`, `src/auth.js`, `src/db.js`) exist to reconstruct from, and there is no existing `governance-intent.md` to amend.
**Gate (`setup.user-mode-ruling`, user-gate):** confirm proposed mode = **brownfield**.
- *User rules brownfield* → proceed to Phase 2 as scoped below.
- *User rules greenfield* (treat existing code as out of scope / pre-mochiko) → skip Phase 2's reconstruction, feature-map lands as an empty scaffold (`setup.feature-map-greenfield`), baselines bootstrap (Phase 6) is deferred to the first `/mochiko:implement` design phase instead of run now.
- *User rules amend* → this is unreachable here (no `${memory_dir}/governance-intent.md` exists to amend); would only apply on a re-run after a prior setup completed.
- *User declines to rule* → run cannot proceed past this point; sits at the gate.

## Phase 2 — Brownfield codebase analysis (conditional on Phase 1 = brownfield)
**Does:** Invokes skill `mochiko:analysis-codebase` to produce a present/partial/absent read of the existing project, per `setup.interrogation-inputs`.
**Seats/skills:** skill `mochiko:analysis-codebase` (pointer on `setup.interrogation-inputs`); locate/enumerate reads (e.g. walking `src/`, `test/`, dependency manifests) would dispatch to a native `Explore` subagent spawned `model: haiku` per `mochiko:patterns-model-tiering` (`setup.model-tiering`); interpretive reads (e.g. inferring the auth/session model from `src/auth.js`, the storage shape from `src/db.js`) stay on the session tier.
**Reads:** `package.json`, `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, plus `plugins/mochiko/skills/analysis-codebase/scripts/detect-stack.sh` output for a deterministic stack baseline.
**Writes:** `.mochiko/memory/codebase-analysis.md`.
**Gate:** none yet — this is fact-finding, not a ruling; feeds Phases 3 and 6.

## Phase 3 — Interrogation (inline, adaptive)
**Does:** Runs the interrogation myself, inline, per `setup.interrogation-inline` — first the agenda's dimensions worked adaptively via skill `mochiko:analysis-iterative`, then the catalog deck card by card, recommend-then-arbitrate.
**Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, `.../catalog/` deck (`universal-floor.md`, `backend-service.md`, etc. — `backend-service.md` is the relevant shelf for this Express service), `.../DOMAIN-DEPENDENCIES.md`; `.mochiko/memory/codebase-analysis.md` from Phase 2 informs which cards apply and surfaces reality-vs-declared-intent conflicts.
**Writes:** nothing yet (rulings accumulate toward the synthesis in Phase 4).
**Gates, all reserved to the user:**
- *Every interrogation card ruling and every module ruling* (`setup.user-card-rulings`) — each card's recommend-then-arbitrate step ends in the user's ruling, not mine.
- *Every waiver* (`setup.user-card-rulings`) — a floor category the user chooses not to cover must be a recorded waiver, not a silent drop (feeds the `setup.fail.floor-category-uncovered` check later).
- *Detected reality vs declared intent conflicts* (`setup.user-conflict-rulings`) — e.g. if the codebase analysis shows `src/auth.js` doing something the user's stated intent contradicts, that conflict is presented and ruled in the open, never silently resolved, whichever way the user rules.

## Phase 4 — Synthesis authoring
**Does:** Renders the frozen interrogation output into the synthesis artifact (`setup.synthesis-artifact`), using `mochiko-cli template governance-intent` if the binary is present, else reading `plugins/mochiko/schemas/governance-intent.yaml` raw as the first-class fallback source of truth.
**Writes:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace) — this is my pen; not yet ratified, so per `setup.fail.pre-ratification-authoring` no governance surface may be authored from it yet.
**Gate:** none at this step (writing the synthesis is not itself the ratification).

## Phase 5 — Pre-ratification stress test
**Does:** Per `setup.stress-test-cold-seat` and `setup.blind-map-dispatch`, before ratification the synthesis gets a cold-seat stress test — spawned in two messages: first the setup topic/project identity and goal only (never the synthesis path), so it builds a Phase 0 angle map blind to what the interrogation concluded; only after that map returns is the synthesis path (`.mochiko/memory/governance-intent.md`) sent and the cold read begins.
**Seats/skills:** skill `mochiko:review-governance-intent`, run by an independent reviewer (e.g. `mochiko:devils-advocate` or `mochiko:validator` acting cold), never the session lead. Since this composes more than one seat, `mochiko:patterns-transport-floor` governs the two-message dispatch and any shared-write topology (non-waivable once triggered, per `setup.transport-floor`).
**Reads (by the cold seat):** first only the topic/goal; then `.mochiko/memory/governance-intent.md` in full.
**Writes:** the cold seat's findings (coverage gaps, contradictions) — not a governance surface, so no floor conflict.
**Gate — alternative branch:** the user may instead give a recorded waiver of the stress test (`setup.stress-test-cold-seat`) in place of spawning the cold seat.
- *User accepts the stress test as designed* → proceeds as above.
- *User records a waiver* → skip straight to Phase 7 (ratification) with the waiver noted in the ledger.

## Phase 6 — Coverage survivor routing
**Does:** Any coverage finding surviving the stress test is presented as a candidate topic questioning the setup's scope, not as a card to silently re-open (`setup.coverage-survivor-routing`).
**Gate (user-gate), per surviving finding:**
- *User rules "explore now"* → re-enters `mochiko:analysis-iterative` on that angle; the re-elicited intent lands under a new GI-XXX entry, looping back to Phase 4 to fold it into the synthesis before ratification.
- *User rules "inline"* → I rule it directly and fold the answer into the synthesis, looping back to Phase 4.
- *User rules "defer"* → noted as an open thread, not blocking; synthesis proceeds to ratification as-is.
- A finding that overlaps an existing agenda dimension instead takes the ordinary interrogation-follow-up path (back into Phase 3 for that card), not this routing.

## Phase 7 — Synthesis ratification
**Gate (`setup.gate-synthesis-ratification`, floor, user-gate):** plain blocking text (`setup.acceptance-plain-text` — never a timed prompt) asking the user to ratify `.mochiko/memory/governance-intent.md` as it now stands.
- *User ratifies* → Phase 8 may begin; this is the hard prerequisite for `setup.fail.pre-ratification-authoring` to be satisfied.
- *User declines / requests changes* → loop back to Phase 3/4 for the specific disputed rulings; no surface authoring occurs in the meantime.

## Phase 8 — Governance surface set authoring
**Does:** Only now, post-ratification, authors the surface set defined by `setup.surface-set`, via skill `mochiko:authoring-constitution`.
**Seats:** an authoring seat plans first and works only on a plan I (as lead) approve (`setup.plan-approval-producers`) — the lead itself never produces (`setup.staffing-latitude`/roles floor).
**Reads:** `.mochiko/memory/governance-intent.md` (ratified), `plugins/mochiko/templates/output-style.md`, the governance-surfaces template/schema, any existing carve-out blocks (none exist yet here since `CLAUDE.md` is absent, so this is a first-write, not a regeneration).
**Writes:**
- `CLAUDE.md` — governance region between `<!-- mochiko:governance:begin/end -->` markers (setup-owned, idempotently regenerated; everything outside stays untouched — `setup.governance-region-ownership`).
- `.claude/rules/mochiko/*.md` — `paths`-scoped rule files, including preserved carve-outs (`mochiko:domain-registry` block, `mochiko:output-style` pair) once they exist on a later run (`setup.carve-outs-preserved`).
- `.mochiko/memory/governance-ledger.md`.
- A trace summary linking each authored principle back to its ratified GI-XXX ruling.
**Note:** no `${memory_dir}/constitution.md` exists here, so `setup.constitution-superseded` (delete-on-sight) is not triggered on this run.

## Phase 9 — Knowledge-management module (conditional)
**Does:** If the KM module was adopted during interrogation (Phase 3), scaffolds it per `templates/constitution-modules/knowledge-management.md`.
**Writes:** the module file plus the project-pinned copy at `.mochiko/memory/knowledge-management.md` (never-overwrite floor holds — `setup.km-module-scaffold`).
**Gate:** none new — module adoption was already a Phase 3 card ruling; this phase just executes it.

## Phase 10 — Feature map landing (brownfield path, per Phase 1 ruling)
**Does:** Extends the Phase 2 analysis into a feature-map reconstruction of delivered capabilities from `src/server.js` (routes), any UI surfaces, and `src/auth.js`/`src/db.js` (services), per `setup.feature-map-brownfield`.
**Seats/skills:** `mochiko:authoring-feature-map` for map machinery and the first-touch re-verify obligation.
**Writes:** `FEATURES.md` index + `.mochiko/features/FEAT-XXX` entries, each marked `delivered` and reconstructed-from-code, via the `features-index`/`feature-entry` templates or their schemas as raw-read fallback.
**Gate (`setup.user-map-confirmation`, user-gate):** the reconstructed map is confirmed entry by entry.
- *User confirms an entry* → it lands as-is.
- *User corrects/rejects an entry* → revise that entry and re-present before landing; per `setup.map-never-overwrite` (floor) this write can never silently clobber a prior version.
- If Phase 1 had instead been ruled greenfield: this phase would instead just scaffold an empty `FEATURES.md` index (`setup.feature-map-greenfield`) with no confirmation gate needed.

## Phase 11 — Product baselines bootstrap (brownfield path)
**Does:** Per `setup.baselines-bootstrap` (marked `Assumed` in the schema — open thread on reconstruction burden / partial-baseline poisoning), bootstraps baselines from the delivered code.
**Writes:** `.mochiko/product/data-model.md`, `.mochiko/product/contracts/`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/quickstart.md` — derived from `src/db.js` (data model), `src/server.js` routes (contracts), `src/auth.js` and `package.json` (constraints/decisions).
**Note:** on the greenfield branch this phase would be skipped entirely — baselines instead seed at the first `/mochiko:implement` design phase.

## Phase 12 — Architecture store scaffold (unconditional, both paths)
**Does:** Per `setup.store-scaffold-unconditional`, creates the store scaffold regardless of mode.
**Writes:** `.mochiko/product/architecture/spine.md` — stub whose header carries a `Scope:` line, and an empty `.mochiko/product/architecture/concerns.md` beside it.
**Reads to set the `Scope:` line (`setup.architecture-scope-handoff`):** the workspace shape already read (Express `src/server.js` = backend-service; no separate frontend/mobile/desktop surface detected in this repo) → `Scope: backend-service`.
**Boundary respected:** per `setup.store-ruled-content-never-here`, this stub carries only the `Scope:` header — no topology, no ruled content; that's reserved for the first `/mochiko:architecture` visit. No architecture stance is taken here.

## Phase 13 — Independent grading of the authored set
**Does:** Per `setup.author-grader-default-fail` (floor) and `setup.fail.author-graded`, the surface set from Phase 8 is graded by a seat that authored none of it, reading the files themselves — never the author's report.
**Seats/skills:** skill `mochiko:validation-constitution`, run by an independent validator; defaults to FAIL absent a clean read.
**Reads:** the actual `CLAUDE.md` governance region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`.
**Writes:** grading verdict/findings (not a governance surface).
**Branch on verdict:**
- *PASS* → proceed to Phase 14.
- *FAIL* → loop back to Phase 8 with the fix list; re-grade after revision (still never by the author).

## Phase 14 — Floor coverage check
**Does:** Confirms every floor category from the catalog either has a landed principle or a Phase 3-recorded waiver, per `setup.fail.floor-category-uncovered`.
**Reads:** the graded surface set plus the Phase 3 ruling/waiver record.
**Gate:** if any floor category is uncovered with no waiver on record, this alone fails the run regardless of other progress (default-FAIL fail-condition) — loops back to Phase 3 to force an explicit ruling or waiver.

## Phase 15 — Final acceptance
**Gate (`setup.gate-final-acceptance`, floor, user-gate):** plain blocking text presenting the trace summary and asking for acceptance of the surface set, flagged proposal by flagged proposal.
- *User accepts all* → proceed to Phase 16.
- *User accepts some, flags others* → the flagged proposals loop back to Phase 8 (revise) → Phase 13 (re-grade) before re-presenting; accepted proposals stand.
- *User declines outright* → run does not close; `setup.fail.no-acceptance` stands until resolved.

## Phase 16 — Version bump and durables
**Does:** Bumps the governance region's semver in `CLAUDE.md` on acceptance.
**Confirms:** `.mochiko/memory/governance-intent.md` and `.mochiko/memory/governance-ledger.md` are never offered for deletion at any point in this run (`setup.durables-never-deleted`, floor).

## Phase 17 — Reporting and hand-off
**Does:** Reports in user-facing prose per `plugins/mochiko/templates/output-style.md` (`setup.register`).
**States:** the surface set closed, the trace from ratified GI-XXX rulings to authored surfaces, the grading verdict, the feature-map landing (brownfield, confirmed) and product-baselines bootstrap, and the architecture store scaffold with `Scope: backend-service`.
**Next-step note (advisory, `setup.next-step`):** points to `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline as peer doors (neither ahead of the other), plus `/mochiko:brainstorm` if the KM module was adopted in Phase 9.
**No git mutations:** any commit is only suggested, never run or pushed (`setup.no-git-mutations`, floor).

---

## Done-condition check (would run silently before declaring close)
All 6 entries under `setup.sec.fail-conditions` verified clear: no pre-ratification authoring (Phase 4 vs Phase 8 ordering enforced), no unclosed trace (Phase 8's trace summary + Phase 17 restatement), no self-graded surface set (Phase 13's independent validator), no uncovered floor category (Phase 14), acceptance given (Phase 15), and a feature map present at close — brownfield-confirmed (Phase 10) or greenfield-scaffolded, per whichever branch Phase 1 resolved to.