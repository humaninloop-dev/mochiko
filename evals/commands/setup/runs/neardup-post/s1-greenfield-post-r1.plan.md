# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Entry:** `$ARGUMENTS` is empty. Workspace evidence already gathered (read-only): repo root holds only `CLAUDE.md` and `README.md` describing "Cadence" (an async-standup digest app) — no source, no dependencies, no config, not a git repo, and no `.mochiko/` directory anywhere in the tree. `plugins/mochiko/` is the plugin installation itself, not project state.

## Phase 0 — Load the binding rules (already performed as prep for this plan)

- **Read**, raw and in full: `plugins/mochiko/schemas/setup.yaml`, `plugins/mochiko/schemas/common.yaml`, `plugins/mochiko/schemas/command-labels.yaml`.
- Confirms the schema's `kind: fail` count is 6 (matches the `.md`'s hard-coded Not-done count) — no halt needed.
- No writes. No seats.

## Phase 1 — Propose the run mode from workspace evidence

- **Read:** `CLAUDE.md`, `README.md`, directory listing (all already done above).
- **Evidence:** README states "Nothing is built yet... no source, no dependencies, no configuration"; no `.mochiko/` state directory exists. This is unambiguously **greenfield** — not amend (no prior governance-intent/ledger to amend) and not brownfield (no delivered code to reconstruct from).
- **Gate (light-touch, still a user ruling per `setup.user-mode-ruling`):** State the proposed mode — greenfield — and ask the user to confirm or override.
  - *If confirmed →* proceed with the greenfield path for every mode-gated rule below (`setup.feature-map-greenfield`, no `setup.user-conflict-rulings`/`setup.user-map-confirmation` obligations, baselines deferred to first `/mochiko:implement`).
  - *If user overrides to brownfield →* would first need to point at actual delivered code to analyze; since none exists, I'd flag the mismatch and ask what code the user means before continuing.
  - *If user overrides to amend →* would need a prior `governance-intent.md`/ledger to amend; none exists, so I'd flag that amend has no baseline yet and recommend greenfield instead, deferring to the user's final call.
- No writes yet.

## Phase 2 — Assemble interrogation inputs

- **Read:** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `references/catalog/` deck (`README.md`, `universal-floor.md`, `backend-service.md`, and any other surface-type files the product's stack implies), `references/DOMAIN-DEPENDENCIES.md`, `references/ESSENTIAL-FLOOR.md`, `references/COMPLIANCE-MODULES.md`, `references/RFC-2119-KEYWORDS.md`, `references/EMERGENT-CEILING-PATTERNS.md`.
- Since mode = greenfield, `mochiko:analysis-codebase` (brownfield-only, per `setup.interrogation-inputs`) is **not** invoked — there is no code to analyze.
- Model tiering (`setup.model-tiering` → `common.model-tiering`): any locate/enumerate read here (e.g., "does a `catalog/` entry exist for surface type X") would route to a native `Explore` subagent spawned with `model: haiku`; the interpretive read of agenda/catalog content itself stays on the session tier since I need to reason over it directly.
- No writes.

## Phase 3 — Run the interrogation inline (lead's own duty, not delegated)

- Per `setup.interrogation-inline`, I run this **myself, inline** — never delegated to a subagent seat.
- Work the agenda's dimensions adaptively via `mochiko:analysis-iterative` (one question at a time, enrichment-style), covering things like: product surface types (web app, email digest — likely `backend-service` + `frontend-web`), team size/ownership, compliance posture, data sensitivity, testing/release discipline, knowledge-management appetite.
- Then work the catalog deck card by card, **recommend-then-arbitrate**: for each card I propose a stance, the user rules it.
- **Reserved to the user throughout (`setup.user-card-rulings`):** every card ruling, every module ruling (e.g., adopting the knowledge-management module), every waiver of an Essential Floor category.
- No writes yet — this phase produces working notes toward the synthesis, not the synthesis file itself.

## Phase 4 — Author the synthesis artifact

- **Would write:** `.mochiko/memory/governance-intent.md`, rendered from the governance-intent template (`mochiko-cli template governance-intent`, falling back to reading `plugins/mochiko/schemas/governance-intent.yaml` raw if the binary is unavailable), using the `GI-XXX` ID namespace for each captured ruling.
- Before writing, check for a stale `.mochiko/memory/constitution.md` on disk (`setup.constitution-superseded`) — none exists in this workspace, so nothing to delete; this check is a no-op here but would be stated in one line either way.
- This is the durable amend baseline going forward — never deleted (`setup.durables-never-deleted`).
- Seat: none — I author this myself as lead, per the interrogation being my own duty.
- **Plan-approval note:** since I (the lead) am the one authoring this artifact via my own inline interrogation rather than a spawned producing seat, `common.plan-approval-producers` binds to any seat I *do* spawn later (stress-test, constitution-authoring, grading) — those seats plan first and work only on a plan I approve; the interrogation itself has no separate plan-approval step because it *is* the adaptive, card-by-card approval loop with the user.

## Phase 5 — Pre-ratification stress test (cold seat)

- Per `setup.stress-test-cold-seat` (floor) and `setup.blind-map-dispatch` (floor, two-message dispatch):
  1. Spawn a cold reviewer seat (Agent tool, `subagent_type: mochiko:devils-advocate` or an agent following `mochiko:review-governance-intent`) with **only** the setup topic and project identity/goal — "Cadence, an async-standup digest tool, greenfield governance setup" — **never** the path to `governance-intent.md`.
  2. Wait for its Phase-0 blind angle map to return.
  3. Only then send the synthesis path (`.mochiko/memory/governance-intent.md`) and let the cold read proceed against the actual content.
  - If I choose to pair two reviewers (staffing latitude, `setup.staffing-latitude`), both build their angle maps independently before either sees the synthesis.
- Since this introduces a second seat, `seats: multi` fires — `common.transport-floor` (`setup.transport-floor`) activates: any cross-seat messaging or shared-write coordination between me and the stress-test seat(s) follows `mochiko:patterns-transport-floor`'s message-race and single-writer discipline, referenced not restated.
- Output: severity-ranked coverage/consistency findings against the frozen synthesis.
- No file writes by this seat — it's a grading/fact-finding seat, exempt from plan-approval, and it does **not** touch `governance-intent.md`.

## Phase 6 — Route surviving coverage findings

- **Gate (`setup.coverage-survivor-routing`):** For each finding that survived the cold review and questions the *setup's scope* (not just a card), present it as a candidate topic and let the user rule the path:
  - *Explore now →* re-enter `mochiko:analysis-iterative` on that angle; newly elicited rulings land in the `GI-XXX` namespace, extending the synthesis, then loop back through Phase 5's stress test on the delta if material.
  - *Rule inline →* the user answers directly without a full re-elicitation pass; I fold the ruling into the synthesis.
  - *Defer →* noted and explicitly left open, not silently dropped.
  - A finding that merely overlaps an existing agenda dimension follows the ordinary interrogation-follow-up path (back into Phase 3), not this gate.

## Phase 7 — Ratification gate

- **Gate (`setup.gate-synthesis-ratification`, floor):** Present the finalized `governance-intent.md` synthesis (with any Phase 6 amendments folded in) and ask the user to ratify it as-is.
  - *If ratified →* proceed to Phase 8; this is the hard boundary — no surface may be authored before this point (`setup.fail.pre-ratification-authoring` is a fail condition guarding exactly this).
  - *If the user asks for changes →* loop back into Phase 3/4 to revise the synthesis, then re-run the applicable slice of Phase 5's stress test before returning here.

## Phase 8 — Author the governance surface set

- Seat: spawn/invoke `mochiko:authoring-constitution` (skill; the producing "seat" here is effectively me driving that skill's procedure, or a delegated producer subagent under a plan I approve first, per `common.plan-approval-producers`).
- **Would write:**
  - The governance region in `CLAUDE.md`, bounded by `<!-- mochiko:governance:begin/end -->` markers — idempotently regenerated, everything outside the markers (the existing "Cadence" notes) left untouched (`setup.governance-region-ownership`, floor).
  - `.claude/rules/mochiko/*.md` — `paths`-scoped rule files derived from the ratified GI-XXX rulings.
  - `.mochiko/memory/governance-ledger.md` — the durable ledger, never deleted (`setup.durables-never-deleted`).
  - A trace summary linking each authored surface line back to its ratifying `GI-XXX` entry (closing the trace obligation owned by `mochiko:authoring-constitution`, bound via `setup.surface-set`).
  - If the knowledge-management module was adopted in Phase 3: scaffold per `templates/constitution-modules/knowledge-management.md`, including the project-pinned copy at `.mochiko/memory/knowledge-management.md` (never-overwrite floor holds on subsequent runs).
  - Preserve verbatim (never regenerate over user values) any existing `mochiko:domain-registry` block and the `mochiko:output-style` switch pair — moot on this first run since neither exists yet, but the floor still applies going forward.
  - Feature map (greenfield, `setup.feature-map-greenfield`): scaffold an empty `FEATURES.md` index only — no `.mochiko/features/` entries yet, since nothing is delivered.
  - Architecture store scaffold (unconditional on every path, `setup.store-scaffold-unconditional`): create `.mochiko/product/architecture/` with a `spine.md` stub carrying a `Scope:` header line (declaring e.g. `backend-service + frontend-web` per `setup.architecture-scope-handoff`, write-if-absent) and an empty `concerns.md` beside it. No topology content — that's `/mochiko:architecture`'s job later (`setup.store-ruled-content-never-here`, floor).
  - Product baselines (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`) are **not** written here — greenfield defers them to the first `/mochiko:implement` design phase (`setup.baselines-bootstrap`).
- Bump the governance region's semver as part of this authoring pass.
- Git: no mutations run; I would only *suggest* a commit afterward (`setup.no-git-mutations`, floor) — never execute `git add`/`git commit`/push myself.

## Phase 9 — Independent grading

- Per `common.author-grader-default-fail` / `setup.author-grader-default-fail` (floor): whoever authored Phase 8 does not grade it.
- Seat: spawn `mochiko:validation-constitution` (or `mochiko:tech-lead` acting as grader per that skill's pointer) as a **fresh, non-author** seat reading the actual files on disk — `CLAUDE.md`'s governance region, `.claude/rules/mochiko/*.md`, the ledger — never my authoring report.
- Grades against `plugins/mochiko/skills/validation-constitution/references/QUALITY-CHECKLIST.md` and `ANTI-PATTERNS.md`; specifically checks every Essential Floor category has either a principle or a recorded waiver (guards `setup.fail.floor-category-uncovered`).
- Default posture: FAIL until the grader affirmatively confirms.
  - *If FAIL →* loop back to Phase 8 to fix the named gaps, then re-grade (no self-clearing).
  - *If PASS →* proceed to Phase 10.

## Phase 10 — Final acceptance gate

- **Gate (`setup.gate-final-acceptance`, floor):** Present the accepted-grade surface set and the trace summary to the user, flagged proposal by flagged proposal, as plain blocking text (`common.acceptance-plain-text` — never a timed prompt).
  - *If accepted →* run is complete; proceed to Phase 11.
  - *If the user rejects one or more flagged proposals →* those specific items loop back to Phase 3 (re-interrogate that ruling) or Phase 8 (re-author that surface) as appropriate, then re-grade the delta before returning here.

## Phase 11 — Close-out self-audit and reporting

- Before declaring done, verify none of the 6 `kind: fail` conditions stand:
  1. A surface authored before ratification — no (Phase 7 gated Phase 8).
  2. An unclosed trace from ratified intent to authored surfaces — no (Phase 8's trace summary).
  3. The surface set graded only by its author — no (Phase 9, independent seat).
  4. A floor category with neither principle nor waiver — no (Phase 9 checked this).
  5. No user acceptance — no (Phase 10 gate passed).
  6. No feature map at close — no (empty `FEATURES.md` scaffolded in Phase 8, greenfield-appropriate).
- Report in the register defined by `templates/output-style.md` (`common.register`).
- State next steps (`setup.next-step`, advisory): `/mochiko:specify` for the first feature, and `/mochiko:architecture` for the product's architecture baseline — peer doors, neither ahead of the other — plus `/mochiko:brainstorm` if the knowledge-management module was adopted.
- No further writes in this phase.