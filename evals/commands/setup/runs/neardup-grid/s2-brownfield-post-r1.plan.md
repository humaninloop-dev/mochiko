# Plan — `/mochiko:setup` (this invocation, evaluation run)

**Entry.** `$ARGUMENTS` empty. Workspace already inspected (read-only, no writes): repo root has `README.md`, `package.json` (project `linkjar`, v0.4.2 — Express + `better-sqlite3` link-saving service), `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`. No `.mochiko/`, no `.claude/`, no `CLAUDE.md`. This is real delivered code with no prior governance run — the workspace signal points at **brownfield**, not greenfield.

---

## Phase 0 — Load the binding schema (mandatory first action)

- **Does:** Reads, in full, before anything else fires.
- **Reads:** `plugins/mochiko/schemas/setup.yaml` (six sections: roles, reserved, tools, ways-of-working, boundaries, fail-conditions) and `plugins/mochiko/schemas/common.yaml` (shared blocks the schema's `extends:` stubs pull from), plus `plugins/mochiko/schemas/command-labels.yaml` for label meanings.
- **Writes:** nothing.
- **Seats/skills:** lead only.
- **Gate:** none. (Already done for this evaluation — confirmed 6 `kind: fail` rules exist in `setup.sec.fail-conditions`, matching the `.md`'s hard-coded count, so no halt-and-surface condition is triggered.)

## Phase 1 — Propose the mode

- **Does:** Presents the workspace read (existing `src/`, `test/`, `package.json`, absence of any `.mochiko`/`.claude`/`CLAUDE.md`) and proposes **brownfield**.
- **Reads:** nothing new (reuses Phase 0's workspace scan).
- **Writes:** nothing.
- **Seats/skills:** lead only.
- **Gate (`setup.user-mode-ruling`, reserved to the user):** *What's confirmed* — is the run brownfield, greenfield, or amend?
  - **Brownfield (expected ruling):** continue to Phase 2 with codebase analysis in scope.
  - **Greenfield override:** skip codebase-driven reconstruction; interrogation runs from the agenda/catalog alone, feature map scaffolds empty, baselines deferred to first `/mochiko:implement`.
  - **Amend override:** since no `.mochiko/memory/governance-intent.md` exists yet, there is no baseline to amend — this branch would be surfaced back to the user as inconsistent with workspace state, asking them to re-rule between greenfield/brownfield instead of proceeding on a nonexistent baseline.
  - All three branches converge back into the same phase sequence below, scoped by whichever mode was ruled.

## Phase 2 — Brownfield analysis (mode-conditional: brownfield)

- **Does:** Runs `mochiko:analysis-codebase` to characterize the existing product before interrogation opens.
- **Reads:** `src/server.js`, `src/db.js`, `src/auth.js`, `test/links.test.js`, `package.json`, `README.md`; runs the skill's `detect-stack.sh` for a deterministic stack baseline. Enumeration-style reads (file listing, route/endpoint scanning) would tier to a native `Explore` subagent spawned with `model: haiku` per `mochiko:patterns-model-tiering`; interpretive reads (e.g., characterizing `auth.js`'s security posture) stay on the session tier.
- **Writes:** `.mochiko/memory/codebase-analysis.md`.
- **Seats/skills:** `mochiko:analysis-codebase`, naturally carried by a `mochiko:tech-lead`-type seat (the agent whose remit includes "runs the codebase analysis a brownfield governance set is built on").
- **Gate:** none yet — this is fact-finding, not a ruling.

## Phase 3 — Interrogation (inline, card by card)

- **Does:** The lead runs the interrogation itself, inline (`setup.interrogation-inline`) — first the agenda's dimensions worked adaptively via `mochiko:analysis-iterative`, then the catalog deck card by card, recommend-then-arbitrate.
- **Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`, plus `.mochiko/memory/codebase-analysis.md` from Phase 2.
- **Writes:** nothing yet (synthesis lands next phase).
- **Seats/skills:** lead + `mochiko:analysis-iterative`.
- **Gates (all reserved to the user, `setup.user-card-rulings`):** every card ruling, every compliance-module ruling, every waiver is the user's, worked one at a time.
  - Per card: user picks the recommended option, an alternative, or a custom ruling → each accepted ruling is folded into the running synthesis and interrogation continues to the next card.
  - Brownfield-only (`setup.user-conflict-rulings`): where detected code reality conflicts with a stated intent (e.g., `auth.js` reveals a pattern the user's stated principle contradicts), the conflict is surfaced in the open for the user to rule, never silently resolved — ruling folds in the same way and interrogation continues.

## Phase 4 — Author the synthesis (draft, pre-ratification)

- **Does:** Compiles the folded interrogation into the durable intent artifact.
- **Reads:** the accumulated card/module/waiver rulings from Phase 3.
- **Writes:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace), from `mochiko-cli template governance-intent` or, if the binary is absent, `plugins/mochiko/schemas/governance-intent.yaml` read raw.
- **Seats/skills:** lead.
- **Gate:** none yet — this is a draft, not yet ratified. (No governance *surface* — CLAUDE.md region, `.claude/rules/mochiko/*.md` — may be authored before the next gate; doing so would trip `setup.fail.pre-ratification-authoring`.)

## Phase 5 — Pre-ratification stress test (cold seat)

- **Does:** Spawns an independent cold reviewer against the draft synthesis, per `setup.stress-test-cold-seat` and the blind two-message dispatch floor `setup.blind-map-dispatch`.
  - **Message 1:** setup topic / project identity ("linkjar," an Express + sqlite link-saving service) and goal only — never the synthesis path. The seat builds its Phase 0 angle map blind.
  - **Message 2** (sent only after the map returns): the synthesis path (`.mochiko/memory/governance-intent.md`); cold read begins.
- **Reads (by the spawned seat, not the lead):** `.mochiko/memory/governance-intent.md`.
- **Writes:** nothing from this phase directly (findings are reported, not filed).
- **Seats/skills:** an independent reviewer seat running `mochiko:review-governance-intent` (e.g. `mochiko:devils-advocate`-shaped), never the lead who wrote the synthesis. Transport floor (`setup.transport-floor`, active — `seats: multi` triggers the moment this second seat is composed) governs the two-message split and any shared-write handling, per `mochiko:patterns-transport-floor` — referenced, not restated here.
- **Gate — cold-seat waiver:** *what's confirmed* — run the stress test, or accept the user's recorded waiver of it.
  - **Run it (expected):** proceed to the survivor-routing gate below.
  - **User waives it:** the waiver is recorded, and the run proceeds straight to Phase 6 ratification without a cold read — this waiver is itself one of the "every card ruling, module ruling, and waiver is the user's" (`setup.user-card-rulings`) items an independent grade must later confirm.
- **Gate — coverage-survivor routing (`setup.coverage-survivor-routing`), for findings that survive the cold read:** *what's confirmed* — each surviving finding questions the setup's scope (not a single card), presented as a candidate topic.
  - **Explore now:** re-enters `mochiko:analysis-iterative` on that angle; the re-elicited intent lands in the GI-XXX namespace, folding back into the draft synthesis, then re-checked before ratification.
  - **Rule inline:** the user rules the gap directly without a new interrogation pass; folds in the same way.
  - **Defer:** noted, synthesis proceeds to ratification without it.
  - (A finding that overlaps an existing agenda dimension instead takes the ordinary interrogation-follow-up path from Phase 3, not this routing.)

## Phase 6 — Ratification gate

- **Does:** Presents the (possibly revised) synthesis in full for the user's ratification.
- **Reads:** final `.mochiko/memory/governance-intent.md`.
- **Writes:** nothing until ratified.
- **Seats/skills:** lead.
- **Gate (`setup.gate-synthesis-ratification`, floor — plain blocking text, never timed):** *what's confirmed* — the entire synthesis: mode, every card ruling, every module ruling, every waiver.
  - **Ratifies as-is:** proceeds to Phase 7 — surfaces may now be authored.
  - **Requests changes:** loops back into Phase 3 (targeted card revisits) or Phase 5 (re-stress-test if the change is material), then re-presents this same gate.
  - **Rejects/aborts:** run halts here. Only the draft synthesis exists on disk; no governance surface, feature map, or store content is authored (would otherwise trip `setup.fail.pre-ratification-authoring`). Goal is not met; run ends without close.

## Phase 7 — Author the governance surface set

- **Does:** Plans the surface-authoring work first (`common.plan-approval-producers` — any writing seat plans before it writes, works only on an approved plan), then executes against `mochiko:authoring-constitution`.
- **Reads:** ratified `governance-intent.md`; any existing `.mochiko/memory/constitution.md` (none found here, so no supersede-and-delete needed per `setup.constitution-superseded`); the governance-surfaces template/schema for carve-out shape.
- **Writes:**
  - Governance region in `CLAUDE.md` (`<!-- mochiko:governance:begin/end -->`), idempotently regenerated; everything outside those markers stays untouched (`setup.governance-region-ownership` — floor).
  - `paths`-scoped `.claude/rules/mochiko/*.md` module rule files, one per ratified module ruling.
  - `.claude/rules/mochiko/output-style.md` and the CLAUDE.md output-style switch line, preserved verbatim as carve-outs if already present, authored fresh otherwise (`setup.carve-outs-preserved`).
  - `.mochiko/memory/governance-ledger.md` — the trace ledger.
  - If a knowledge-management module was ratified: `.mochiko/memory/knowledge-management.md`, scaffolded from `templates/constitution-modules/knowledge-management.md`, never overwritten thereafter (`setup.km-module-scaffold`).
  - Governance region semver bumped.
- **Seats/skills:** producer seat carrying `mochiko:authoring-constitution` (the `mochiko:tech-lead` agent, consistent with its remit to author/update the governance surface).
- **Gate:** internal plan-approval only (lead approves the authoring plan before the seat writes) — not a new user-facing gate; the user's substantive ruling already happened at ratification.

## Phase 8 — Feature map + product baselines landing

- **Does (brownfield path, as ruled in Phase 1):** Extends the Phase 2 analysis into a feature-map reconstruction of delivered capabilities (e.g., link-save/list/auth routes visible in `src/server.js`/`src/auth.js`).
- **Reads:** `.mochiko/memory/codebase-analysis.md`, `src/*.js`.
- **Writes:** `FEATURES.md` index + `.mochiko/features/` entries, each marked `delivered` and reconstructed-from-code, via the features-index/feature-entry templates or schemas if the binary is absent; also bootstraps `.mochiko/product/` baselines — `data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md` — from the delivered code (`setup.baselines-bootstrap`, the `Assumed` clause). Never overwrites an existing map (`setup.map-never-overwrite`, floor).
- **Seats/skills:** `mochiko:authoring-feature-map` (naturally the `mochiko:product-manager` agent).
- **Gate (`setup.user-map-confirmation`, reserved to the user, brownfield-only):** *what's confirmed* — the reconstructed map, entry by entry.
  - **Confirms an entry:** it lands as written.
  - **Corrects an entry:** correction folds in before landing; never silently overwritten afterward.
  - (Greenfield path, not taken here, would instead scaffold an empty `FEATURES.md` with no confirmation loop; an amend path would surface a missing map as an offer rather than auto-scaffold it.)

## Phase 9 — Architecture store scaffold (unconditional, every path)

- **Does:** Creates `.mochiko/product/architecture/` if missing.
- **Reads:** the analysis from Phase 2 to determine which surface types the product carries — here, a single Express/sqlite backend service, no frontend/mobile/desktop signal.
- **Writes:** `.mochiko/product/architecture/spine.md` stub with a header `Scope:` line (write-if-absent only — set to `backend-service` here) and an empty `concerns.md` beside it. No ruled architectural content is authored (`setup.store-ruled-content-never-here`, floor) — that's reserved for the first `/mochiko:architecture` visit.
- **Seats/skills:** lead (mechanical scaffold, no judgment call).
- **Gate:** none — this is a non-waivable, content-free scaffold step.

## Phase 10 — Independent grade of the authored surface set

- **Does:** Grades the CLAUDE.md governance region, the `.claude/rules/mochiko/*.md` files, and the ledger against the quality checklist, reading the files themselves — never the author's report — defaulting to FAIL.
- **Reads:** the surfaces written in Phase 7.
- **Writes:** nothing (a grading verdict, reported).
- **Seats/skills:** `mochiko:validation-constitution`, run by a seat that authored none of it — e.g. `mochiko:validator` (never the `mochiko:tech-lead` seat from Phase 7) — enforcing `setup.author-grader-default-fail` / `setup.fail.author-graded`.
- **Gate:** not user-facing by default; a FAIL verdict loops back to Phase 7 for revision and re-grades. A PASS proceeds to Phase 11.

## Phase 11 — Trace summary

- **Does:** Compiles the closed trace: ratified card/module/waiver → the specific rule file, region line, or ledger entry it produced — satisfying the "trace closes and an independent grade confirmed it" goal clause and avoiding `setup.fail.unclosed-trace`.
- **Reads:** `governance-intent.md`, the authored surfaces, the Phase 10 grade.
- **Writes:** the trace summary (part of the surface set per `setup.surface-set`).
- **Seats/skills:** lead.
- **Gate:** none — feeds directly into final acceptance.

## Phase 12 — Final acceptance gate

- **Does:** Presents the full surface set with the trace summary, flagged proposal by flagged proposal.
- **Reads:** everything authored in Phases 7–11.
- **Writes:** nothing until accepted.
- **Seats/skills:** lead.
- **Gate (`setup.gate-final-acceptance`, floor — plain blocking text, never timed, `common.acceptance-plain-text`):** *what's confirmed* — each flagged proposal individually (each rule file's content, the feature-map entries, the architecture `Scope:` line, any waiver).
  - **Accepts all:** run closes — proceeds to Phase 13.
  - **Rejects one or more flagged items:** targeted revision loops back to Phase 7 (or Phase 8/9 if map/scope-related) for just those items, re-grades if a surface changed (back through Phase 10), re-presents this gate.
  - **Defers:** noted as an open follow-up; the goal is not met and the run does not close (`setup.fail.no-acceptance` would stand if the run were force-closed here).

## Phase 13 — Close-out

- **Does:** Confirms none of the 6 `kind: fail` conditions stand (pre-ratification-authoring, unclosed-trace, author-graded, floor-category-uncovered, no-acceptance, no-feature-map); reports in the register voice from `templates/output-style.md` (`setup.register`); suggests — never runs — a commit (`setup.no-git-mutations`, floor); routes next steps.
- **Reads:** nothing new.
- **Writes:** nothing new.
- **Seats/skills:** lead.
- **Gate:** none — informational close.
- **Next-step routing (`setup.next-step`, advisory):** recommends `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline as peer doors (neither ahead of the other), plus `/mochiko:brainstorm` if the knowledge-management module was adopted in Phase 3.