# Action Plan — `/mochiko:setup` (plan-only; no execution)

**Workspace read (already performed, read-only):** no `.mochiko/`, no `.claude/`, no `.git`. `CLAUDE.md` is a short working-notes file with no `<!-- mochiko:governance:begin/end -->` markers. `README.md` states explicitly: *"Nothing is built yet... no source, no dependencies, no configuration."* `plugins/mochiko/` is the plugin's own tree (not product code). This evidence points to **greenfield**, unambiguously — no product code, no prior governance artifacts, no `constitution.md` to supersede.

**Schema audit:** `plugins/mochiko/schemas/setup.yaml` and `.../common.yaml` were Read raw in full (the run's mandatory first action). `kind: fail` count in `setup.sec.fail-conditions` = 6 (`pre-ratification-authoring`, `unclosed-trace`, `author-graded`, `floor-category-uncovered`, `no-acceptance`, `no-feature-map`) — matches the mission text's stated count, so the pair is in sync and the run is not halted.

---

## Phase 0 — Rule-base load (complete)

- **Done:** Read `plugins/mochiko/schemas/setup.yaml` raw, in full; Read `plugins/mochiko/schemas/common.yaml` raw, in full (both instructed as the same first action). Read `plugins/mochiko/schemas/command-labels.yaml` to resolve label meanings.
- **Read:** the two schema files above; no writes.
- **Outcome:** binding rule set for this run resolved, including `${var}` substitution (`memory_dir=.mochiko/memory`, `product_dir=.mochiko/product`, `explore_model=haiku`, `plugin_root=${CLAUDE_PLUGIN_ROOT}`) and the two `extends: common.*` stubs (`setup.no-git-mutations`, `setup.acceptance-plain-text`, plus the `when: {seats: multi}`-gated `setup.transport-floor`).

## Phase 1 — Entry & mode proposal (Gate A)

- **Done:** workspace scan for existing `.mochiko/`, `.claude/`, `CLAUDE.md` governance markers, and product code, to propose a mode per `setup.user-mode-ruling`. (In a live run, a bare locate/enumerate sweep like this routes through a native `Explore` subagent spawned `model: haiku` per `setup.model-tiering`; interpretive judgment on what the sweep means stays on the session tier.)
- **Read:** `CLAUDE.md`, `README.md`, top-level directory listing (already done above).
- **Proposal:** **greenfield** — no delivered code, no prior governance surfaces, no `.mochiko/memory/constitution.md` to flag as superseded (`setup.constitution-superseded` is a no-op here).
- **Gate A (user):** confirm or override the proposed mode.
  - **Ruling = greenfield (expected):** proceed to Phase 2 on the greenfield branch throughout.
  - **Ruling = brownfield:** insert a codebase-analysis step before interrogation (`mochiko:analysis-codebase` → `.mochiko/memory/codebase-analysis.md`), and every `when: {mode: [brownfield]}` rule activates: `setup.user-conflict-rulings`, `setup.user-map-confirmation`, `setup.feature-map-brownfield` (reconstruction + entry-by-entry user confirmation) replaces the greenfield scaffold, and `setup.baselines-bootstrap` bootstraps `.mochiko/product/` baselines from delivered code instead of deferring them.
  - **Ruling = amend:** skip interrogation-as-first-authoring; re-open `.mochiko/memory/governance-intent.md` if present (it would not be, here) and route any missing feature map through "surfaced and offered," never auto-scaffolded (`setup.fail.no-feature-map`'s third disjunct).

## Phase 2 — Interrogation input assembly (greenfield branch)

- **Done:** locate the interrogation inputs bound by `setup.interrogation-inputs`.
- **Read:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md` under the same references directory. (Greenfield: `mochiko:analysis-codebase` is not invoked — no code exists to analyze.)
- **Writes:** none.

## Phase 3 — Inline interrogation session (recurring Gate B)

- **Done:** I (the lead) run the interrogation **myself, inline** — never delegated to a subagent (`setup.interrogation-inline`, `kind: duty`). Two stages: (1) the agenda's dimensions worked adaptively via the `mochiko:analysis-iterative` skill (one question at a time, recommend-then-arbitrate), then (2) the catalog deck worked card by card, same recommend-then-arbitrate discipline.
- **Skill invoked:** `mochiko:analysis-iterative` (pointer on `setup.interrogation-inline`).
- **Gate B (user, recurring — one instance per card/dimension/module/waiver):** every interrogation card ruling, every module ruling (e.g. whether the knowledge-management module is adopted), and every waiver is reserved to the user (`setup.user-card-rulings`) — I recommend, the user rules.
  - **Ruling = accept recommendation:** that card's disposition is recorded as-is into the running synthesis draft.
  - **Ruling = amend/override:** the user's alternative replaces my recommendation; recorded verbatim.
  - **Ruling = defer/waive a floor category:** recorded as an explicit waiver (feeds `setup.fail.floor-category-uncovered` risk at grading — a category needs *either* a principle *or* a recorded waiver, never neither).
- **Writes:** none yet — this stage builds the in-session synthesis draft only; nothing is persisted to disk until Phase 4.

## Phase 4 — Synthesis authoring

- **Done:** render the interrogation's outcome into the durable synthesis artifact.
- **Would write:** `.mochiko/memory/governance-intent.md`, GI-XXX namespace, via `mochiko-cli template governance-intent` if the binary is present, else `plugins/mochiko/schemas/governance-intent.yaml` Read raw as the first-class fallback source of truth (`setup.synthesis-artifact`).
- **Boundary:** this is the **synthesis**, not yet a governance surface — `setup.fail.pre-ratification-authoring` fails the run if any surface (CLAUDE.md region, `.claude/rules/mochiko/*.md`, ledger) is authored before this synthesis is ratified. Nothing beyond `governance-intent.md` itself is written in this phase.

## Phase 5 — Pre-ratification blind stress-test

- **Done:** spawn the cold stress-test seat required by `setup.stress-test-cold-seat`, since no user waiver of this step has been recorded (a recorded waiver would skip straight to Phase 7).
- **Seat/skill:** `mochiko:review-governance-intent`, run by an independent seat (e.g. `mochiko:devils-advocate` agent type) — **never a participant in the interrogation session**, per `kind: independence`.
- **Transport discipline:** because this composes a second seat, `seats: multi` fires and `setup.transport-floor` (→ `mochiko:patterns-transport-floor`) becomes binding, non-waivable, for all messaging/shared-write topology from here on.
- **Dispatch protocol (`setup.blind-map-dispatch`, floor):** two-message spawn —
  1. First message: setup topic / project identity and goal **only** — never the path to `governance-intent.md`. The seat builds a Phase-0 angle map blind to what the interrogation concluded.
  2. Only after that map returns do I send the synthesis path; the cold read then proceeds against the actual file, never a summary.
- **Read (by the stress-test seat):** `.mochiko/memory/governance-intent.md` directly.
- **Writes:** a findings/coverage report from the stress-test seat (ephemeral to the run, not a governed surface).

## Phase 6 — Coverage-survivor routing (Gate D, conditional)

- Fires only if the stress-test seat's coverage findings survive its own cross-examination.
- **Gate D (user):** each surviving gap is presented as a candidate topic, questioning the setup's *scope*, not any single card. Three branches per gap:
  - **Explore now:** re-enter `mochiko:analysis-iterative` on that angle; re-elicited intent lands in the GI-XXX namespace (back into the Phase 4 artifact), then re-run relevant parts of Phase 5.
  - **Rule inline:** the user rules the gap directly without a full re-elicitation pass; recorded into the synthesis.
  - **Defer:** explicitly logged as deferred, not silently dropped.
  - (A gap that overlaps an existing agenda dimension instead takes the ordinary interrogation-follow-up path, not this routing.)

## Phase 7 — Ratification (Gate C, floor)

- **Gate C (user, `setup.gate-synthesis-ratification`, floor):** the user ratifies the synthesis in `.mochiko/memory/governance-intent.md` as it now stands (post Phase 5/6).
  - **Ruling = ratify:** proceed to Phase 8 — this is the only ruling that unblocks any surface authoring (`setup.fail.pre-ratification-authoring` is the enforced contrapositive).
  - **Ruling = not yet / more changes wanted:** loop back into Phase 3/6 as applicable; no surface authoring occurs until re-presented and ratified.

## Phase 8 — Surface authoring

- **Seat:** a producing seat (e.g. `mochiko:tech-lead`, whose profile explicitly authors/updates the governance surface) plans first and works only on a plan I approve (`setup.plan-approval-producers`) — I do not author this myself as lead.
- **Skill:** `mochiko:authoring-constitution` (pointer on `setup.surface-set`), which owns composition and read-scope for the whole surface set.
- **Would write:**
  - The governance region in `CLAUDE.md`, bounded by `<!-- mochiko:governance:begin/end -->`, idempotently regenerated — everything outside those markers is untouched user content (`setup.governance-region-ownership`, floor).
  - `.claude/rules/mochiko/*.md`, `paths`-scoped.
  - `.mochiko/memory/governance-ledger.md`.
  - The trace summary (ratified intent → authored surfaces).
  - If the knowledge-management module was adopted in Phase 3: scaffold per `templates/constitution-modules/knowledge-management.md`, including the project-pinned copy at `.mochiko/memory/knowledge-management.md` (its own never-overwrite floor holds thereafter).
- **Preserved verbatim, never regenerated (`setup.carve-outs-preserved`, floor):** the `mochiko:domain-registry` block in the domain-layer rules file, and the `mochiko:output-style` pair (switch line in the CLAUDE.md region + `.claude/rules/mochiko/output-style.md`) — not applicable to overwrite here since none exist yet, but the authoring skill must still respect the carve-out grammar going forward.
- **Non-waivable floors applied throughout:** `setup.no-git-mutations` (suggest commits only, never run git mutations or push — consistent with this being a non-git-initialized workspace besides), `setup.durables-never-deleted` (never offer to delete the synthesis or the ledger).

## Phase 9 — Feature map & architecture store scaffold

- **Feature map (greenfield branch, `setup.feature-map-greenfield`):** scaffold an **empty** `FEATURES.md` index only — no capabilities to derive, since no code/stories exist yet. Governed by the never-overwrite floor (`setup.map-never-overwrite`).
- **Architecture store (`setup.store-scaffold-unconditional`, unconditional on every path):** create `.mochiko/product/architecture/` where absent, with a `spine.md` stub carrying a `Scope:` line and an empty `concerns.md` beside it. Greenfield's stub is header-only, no topology under it. Nothing here is "ruled content" (`setup.store-ruled-content-never-here`, floor) — that's reserved for the first `/mochiko:architecture` visit.
- **Scope handoff (`setup.architecture-scope-handoff`):** declare which surface types the product carries (backend-service / frontend-web / mobile / desktop / full-stack composition) on the `spine.md` header's `Scope:` line, write-if-absent only. From the README ("A web app with a morning email digest"), the scope reads as full-stack (frontend-web + backend-service) — but this is a declaration/handoff, not an architecture stance; the user may override it later at the `/mochiko:architecture` desk.
- **Baselines (`setup.baselines-bootstrap`):** on greenfield, `data-model.md` / `contracts/` / `constraints-and-decisions.md` / `quickstart.md` are **not** bootstrapped now — deferred to the first `/mochiko:implement` run's design phase.
- **Skill pointer:** `mochiko:authoring-feature-map` for map machinery; `mochiko:authoring-architecture-store` governs the store-write grammar for the scaffold.

## Phase 10 — Independent grading

- **Seat:** a non-author validator seat (e.g. `mochiko:validator`), distinct from whoever authored in Phase 8 — `setup.author-grader-default-fail` (floor): no output is cleared by its own author, and grading must read the authored surfaces themselves, never the author's report. Default posture is **FAIL** until the files earn a PASS.
- **Skill:** `mochiko:validation-constitution` (the checklist-grading skill for the CLAUDE.md governance region, `.claude/rules/mochiko/` files, and the governance ledger).
- **Read (by the grader):** the CLAUDE.md governance region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, the trace summary — the files, not a summary.
- **Outcome branch:**
  - **PASS:** proceed to Phase 11.
  - **FAIL:** findings routed back to the Phase 8 producing seat for revision; the loop (author → grade) repeats until PASS or the user intervenes. `setup.fail.author-graded` remains the standing risk this loop exists to retire.

## Phase 11 — Trace closure & versioning

- **Done:** confirm the intent→surface trace closes across the full set (every ratified GI-XXX item traceable to an authored rule/section; no orphaned surface content) — an unclosed trace is `setup.fail.unclosed-trace`.
- **Would write:** bump the governance region's semver.
- **Check:** every Essential Floor category carries either a principle or a recorded waiver from Phase 3/6 — a gap here is `setup.fail.floor-category-uncovered`.

## Phase 12 — Final acceptance (Gate E, floor)

- **Gate E (user, `setup.gate-final-acceptance`):** present the accepted trace summary and walk the surface set **flagged proposal by flagged proposal** — plain blocking text, never a timed prompt (`setup.acceptance-plain-text`).
  - **Ruling = accept all:** run proceeds to close.
  - **Ruling = reject/amend a specific flagged proposal:** that item routes back to Phase 8 (or earlier, to Phase 3/6, if the rejection reveals an intent gap rather than an authoring defect); re-grade (Phase 10) before re-presenting.
  - **No ruling given / declined outright:** `setup.fail.no-acceptance` — run does not close.

## Phase 13 — Close & report

- **Fail-condition audit (mandatory before declaring done):** re-check all 6 `kind: fail` rules —
  1. `pre-ratification-authoring` — satisfied (Phase 8 gated on Phase 7 ratification).
  2. `unclosed-trace` — satisfied via Phase 11.
  3. `author-graded` — satisfied via Phase 10's non-author grading loop.
  4. `floor-category-uncovered` — satisfied via Phase 11's category check.
  5. `no-acceptance` — satisfied via Phase 12.
  6. `no-feature-map` — satisfied: greenfield empty `FEATURES.md` index exists at close (Phase 9).
- **Report (`setup.register`, per `templates/output-style.md`):** confirm the surface set is live, cite the semver bump, and state next steps (`setup.next-step`, advisory): `/mochiko:specify` for the first feature, and `/mochiko:architecture` for the product's architecture baseline — presented as peer doors, neither ahead of the other — plus `/mochiko:brainstorm` if the knowledge-management module was adopted in Phase 3.
- **Writes in this phase:** none beyond the semver bump already counted in Phase 11; this phase is reporting only.