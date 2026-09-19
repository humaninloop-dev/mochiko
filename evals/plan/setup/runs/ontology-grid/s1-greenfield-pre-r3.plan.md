## Plan-only evaluation output — `/mochiko:setup` (no arguments) on the `Cadence` repo

Reads already performed to make this plan scenario-concrete (all read-only, permitted): `plugins/mochiko/schemas/setup.yaml` (full, 6 `fail-condition` rules confirmed — matches the command's hard-coded count, no halt needed), `command-labels.yaml`, `plugins/mochiko/commands/setup.md`, the workspace root, `CLAUDE.md`, `README.md`, plus the downstream schemas/skills the run would invoke (`governance-intent.yaml`, `governance-surfaces.yaml`, `authoring-constitution/SKILL.md`, `review-governance-intent/SKILL.md`, `validation-constitution/SKILL.md`, `INTERROGATION-AGENDA.md`, `catalog/README.md`, `architecture-store.yaml`, `authoring-feature-map/SKILL.md`).

**Workspace facts found:** not a git repo; no `.mochiko/` directory; no `.claude/rules/mochiko/`; `CLAUDE.md` exists but is plain working notes with no governance markers; `README.md` states "Nothing is built yet... no source, no dependencies, no configuration" for a planned web app ("Cadence") with an async-standup email digest, two part-time engineers, first pilot one internal team of six. This is unambiguous greenfield evidence.

---

### Phase 1 — Load binding rules (done)
- **Does:** Reads `plugins/mochiko/schemas/setup.yaml` raw, in full, before proposing anything; substitutes `${var}` placeholders; cross-checks the `setup.sec.fail-conditions` count against the command `.md`'s hard-coded "6".
- **Reads:** `plugins/mochiko/schemas/setup.yaml`, `plugins/mochiko/schemas/command-labels.yaml`, `plugins/mochiko/commands/setup.md`.
- **Writes:** none.
- **Seats/skills:** lead only.
- **Gate:** none (a rule not read would keep the run closed; schema was read whole, so the run is open). Result: count matches (6) — no halt.

### Phase 2 — Workspace scan and mode proposal
- **Does:** Cheap enumerate/locate scan of the repo root and for `.mochiko/`, `.claude/rules/mochiko/`, `.git`; interpretive read of `CLAUDE.md`/`README.md` to frame project identity.
- **Reads:** repo root listing, `CLAUDE.md`, `README.md`.
- **Writes:** none.
- **Seats/skills:** the locate/enumerate leg is model-tiering-eligible (a native `Explore` subagent spawned `model: haiku`, per `mochiko:patterns-model-tiering`) — the lead did it directly here since the tree is 3 entries; the interpretive read of the two docs stays on the session tier either way.
- **Gate (`setup.user-mode-ruling`, user-gate):** the lead proposes **greenfield** from the evidence above and presents it as plain blocking text, never a timed prompt (`setup.acceptance-plain-text`).
  - **Branch — user confirms greenfield:** proceed to Phase 3 as planned below.
  - **Branch — user rules brownfield instead:** halt this plan's Phase 3 and first run `mochiko:analysis-codebase` (pointer: `setup.interrogation-inputs`) to produce `.mochiko/memory/codebase-analysis.md`; given there is no source at all, that analysis would almost certainly read back "absent" across the board — the lead would surface that mismatch to the user before continuing, since a brownfield ruling over an empty repo is itself worth confirming.
  - **Branch — user rules amend instead:** amend requires an existing ratified synthesis/surface set; none exists (no `.mochiko/memory/governance-intent.md`, no governance region in `CLAUDE.md`). The lead would say so and ask the user to re-rule, since there is nothing to amend from.

*(Everything from here assumes the confirmed branch: greenfield.)*

### Phase 3 — Inline interrogation session (not executed in this plan-only run)
- **Does:** Lead runs the ten-dimension agenda from `INTERROGATION-AGENDA.md` adaptively, one question per turn, via `mochiko:analysis-iterative` craft (`setup.interrogation-inline`) — project identity, fact profile, type/shape, risk surface, team reality (two part-time engineers, informal review culture per the existing `CLAUDE.md` notes), existing practices (none yet), knowledge-management adoption (offered default-on), deployment/release reality, values/non-negotiables, deliberate exclusions. Then: depth-level declaration (lead recommends `low` for greenfield, user rules — one-way ratchet), the four universal-floor cards asserted at the declared row, the `backend-service` shelf dealt if the type includes an API side (fullstack per README) with the honest gap noted that no `frontend.md` shelf is seeded yet, minting from the values dimension, the layered-architecture beat if triggered, waiver rulings.
- **Reads:** `INTERROGATION-AGENDA.md`, `catalog/universal-floor.md`, `catalog/backend-service.md`, `COMPLIANCE-MODULES.md`, `DOMAIN-DEPENDENCIES.md` (if the layered beat fires).
- **Writes:** none yet — this phase is conversational; the synthesis is assembled at its close.
- **Seats/skills:** lead only, inline.
- **Gates (all `setup.user-card-rulings` / `setup.user-mode-ruling`-adjacent, continuous through this phase, plain blocking text each time):** every dimension answer, the depth-level declaration, every floor-card expression, every arbitrated deck ruling (keep/tighten/drop/re-rank), every mint, every module ruling (KM adopt/decline; layer-rules if triggered), every waiver — each is the user's, not inferred.
  - **Branch — KM adopted:** noted for Phase 7's scaffold obligation.
  - **Branch — KM declined:** recorded as a durable decline, memorialized again under deliberate exclusions (dimension 10); no scaffold in Phase 7.

### Phase 4 — Synthesis assembly
- **Does:** Lead assembles `.mochiko/memory/governance-intent.md` from the session's answers, shaped by the `governance-intent` schema skeleton (GI-001, GI-002, … sequential IDs; a confidence mark per element).
- **Reads:** `plugins/mochiko/schemas/governance-intent.yaml` (already read above for shape).
- **Writes (would, not executed):** `.mochiko/memory/governance-intent.md` — first draft, provisional/frozen pending review (this is the traceable contract, not a "surface"; the fail-condition `setup.fail.pre-ratification-authoring` concerns *surfaces*, so drafting the synthesis itself before ratification is correct sequencing, not a violation).
- **Seats/skills:** lead only.
- **Gate:** none yet — this feeds the sizing decision in Phase 5.

### Phase 5 — Pre-ratification stress test (cold seat)
- **Does:** Per `setup.stress-test-cold-seat` and `setup.blind-map-dispatch` (floor, non-waivable dispatch shape): lead states a sizing weight (element count · mark mix · reality-surface load) and defaults to a **pair** of independent reviewers on first ratification (a lighter sizing would need a recorded departure trail — for a small first-ratification greenfield synthesis a single reviewer with a stated departure is plausible, but the default stands absent a reason to depart). Each reviewer is spawned in **two messages**: first only the setup topic/project identity/goal (blind — the reviewer builds its Phase-0 coverage/coherence angle map before knowing anything the interrogation concluded), then, once that map returns, the synthesis path (`.mochiko/memory/governance-intent.md`) to begin the cold read. In a pair, cross-examination follows per the shared `CROSS-EXAM.md` protocol.
- **Reads (by the spawned reviewer, not the lead):** the frozen synthesis, `INTERROGATION-AGENDA.md` (coverage yardstick); brownfield-only inputs are n/a here.
- **Writes:** none directly — findings return as a message to the lead (never a report file); dispositions land inside the synthesis's own Review section when the lead folds them.
- **Seats/skills:** one or two independent agents running `mochiko:review-governance-intent` — never the lead, never a session participant.
- **Gate — user waiver of the stress test (`setup.stress-test-cold-seat`, always either the cold seat or a recorded user waiver):**
  - **Branch — no waiver (default):** review runs as above.
  - **Branch — user explicitly waives:** the lead records the waiver reason directly in the synthesis's Review section (`none` is never the lead's own call) and proceeds straight to Phase 6; the synthesis reaches ratification unreviewed, with the waiver as its audit trail.
- **Gate — coverage-survivor routing (`setup.coverage-survivor-routing`, user-gate):** any coverage finding that survives is presented as a candidate topic, never folded into a card silently.
  - **Branch — user says explore now:** re-enter `mochiko:analysis-iterative` on that angle inline (back to Phase 3-style questioning), re-elicited intent lands in fresh GI-IDs.
  - **Branch — user rules inline:** the lead records the ruling directly against the relevant GI element(s).
  - **Branch — user defers:** recorded as an open, undispositioned item — this would itself likely keep the synthesis at `needs-revision`, so it can't silently vanish before Phase 6.

### Phase 6 — Synthesis ratification
- **Does:** Lead presents the fully assembled synthesis (with the Review section's dispositions and verify pass filled in) to the user.
- **Reads:** the assembled `.mochiko/memory/governance-intent.md`.
- **Writes:** none new — this is a confirmation over the Phase 4/5 artifact.
- **Gate (`setup.gate-synthesis-ratification`, floor — the user's, plain blocking text):**
  - **Branch — confirmed:** proceed to Phase 7; this is the hard line — no surface may be authored before this point (`setup.fail.pre-ratification-authoring`).
  - **Branch — edit requested:** lead incorporates the edit; a **material** edit routes back through a bounded delta-pass with the still-seated reviewer(s) before re-presenting; a wording-level edit re-presents directly.
  - **Branch — rejected:** returns to the relevant slice of Phase 3 (interrogation) on the disputed dimension(s), then re-assembles and re-presents.

### Phase 7 — Surface authoring
- **Does:** Lead drafts an authoring plan (which ratified element routes to which surface — CLAUDE.md region line vs. a scope-bound rules file vs. a skill pointer) and gets the user's approval on that plan before any producer writes (`setup.plan-approval-producers`). A producer seat then authors the surface set per `mochiko:authoring-constitution`'s greenfield branch.
- **Reads:** the ratified `.mochiko/memory/governance-intent.md`; existing `CLAUDE.md` (to regenerate only the marked region, leaving the current working-notes content untouched — `setup.governance-region-ownership`); `references/ESSENTIAL-FLOOR.md`, `references/catalog/*`, `templates/constitution-modules/knowledge-management.md` (if KM was adopted).
- **Writes (would):**
  - `CLAUDE.md` — the `<!-- mochiko:governance:begin/end -->` region, regenerated in place: Ratified stamp, principle index, universal principles (floor first, `(NON-NEGOTIABLE)`), technology stack, quality-gates summary (real commands — for a repo with no code yet, this will be sparse/declared rather than detected), governance operations block, the `mochiko:output-style` switch line written default-on (first run, so nothing to preserve yet).
  - `.claude/rules/mochiko/*.md` — one file per scope-bound concern the synthesis routed there, plus the unconditional Shape-5 `output-style.md` file every run emits.
  - `.mochiko/memory/governance-ledger.md` — full Shape-3 ledger: floor + depth + modules header, waivers table, amendment policy, exception registry, Three-Part records per GI-ID, amendment log's first "ratified" row.
  - Trace summary manifest (Shape 4), emitted with this authoring round for Phase 9/10.
  - If KM adopted: `templates/constitution-modules/knowledge-management.md` scaffold plus the project-pinned `.mochiko/memory/knowledge-management.md` (`setup.km-module-scaffold`; never-overwrite floor holds on later runs).
- **Seats/skills:** a producer seat (e.g. `mochiko:tech-lead`, whose description names authoring "greenfield defaults or a brownfield codification") running `mochiko:authoring-constitution`; never the lead itself, and never the same seat that will grade it.
- **Gate:** none new here beyond the pre-approved plan; grading happens in Phase 9.

### Phase 8 — Feature map and architecture store scaffolds (unconditional landing)
- **Does (greenfield-specific):**
  - Scaffolds an **empty** `FEATURES.md` index at repo root (`setup.feature-map-greenfield`) — no capability derivation at setup time, since there is no code or story input yet; no user-map-confirmation gate (that's brownfield-only, `setup.user-map-confirmation`).
  - Scaffolds `.mochiko/product/architecture/` unconditionally (both paths, `setup.store-scaffold-unconditional`): a `spine.md` stub whose header carries a `Scope:` line (e.g. `backend-service` + `frontend-web`, since Cadence is a web app with an email digest — this scope reading is a handoff, not a ruling; the `/mochiko:architecture` desk or the user may override that line later) and an empty `concerns.md` beside it. No topology, no AX rows — that's ruled content reserved for the first `/mochiko:architecture` visit (`setup.store-ruled-content-never-here` floor).
  - Explicitly does **not** bootstrap `.mochiko/product/{data-model.md, contracts/, constraints-and-decisions.md, quickstart.md}` — greenfield defers those to the first `/mochiko:implement` run's design phase (`setup.baselines-bootstrap`).
- **Reads:** `plugins/mochiko/schemas/features-index.yaml`, `plugins/mochiko/schemas/architecture-store.yaml` (already read above for shape).
- **Writes (would):** `FEATURES.md` (empty index), `.mochiko/product/architecture/spine.md` (stub), `.mochiko/product/architecture/concerns.md` (empty).
- **Seats/skills:** since both writes are pure scaffold (no judgment content), the lead or the same producer seat can do this directly — no separate feature-map or architecture-store seat is warranted for an empty scaffold.
- **Gate:** none (never-overwrite floor applies on any later run, `setup.map-never-overwrite`; not relevant on first write).

### Phase 9 — Independent grading (author ≠ grader, default FAIL)
- **Does:** A validator seat that authored none of the surfaces reads the files on disk — never the producer's report — and grades the set as one deliverable against `mochiko:validation-constitution`'s checklist (module-parameterized to whatever the synthesis actually selected), producing a binary `VALIDATION RESULT: PASS/FAIL` with checklist tally, surface integrity, trace closure, floor/module accounting, anti-patterns, and a semver bump determination.
- **Reads:** the `CLAUDE.md` governance region, every `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/governance-intent.md`, the trace summary manifest.
- **Writes:** the verdict and per-finding dispositions land in the reviewed artifacts themselves (never conversation-only).
- **Seats/skills:** `mochiko:validator` running `mochiko:validation-constitution` — a distinct agent from the Phase 7 producer.
- **Gate (`setup.author-grader-default-fail`, floor; guarded by fail-condition `setup.fail.author-graded`):**
  - **Branch — PASS:** proceed to Phase 10.
  - **Branch — FAIL:** the fix list routes back to the Phase 7 producer for revision; the same independent validator re-grades; loop until PASS. No partial credit, no "simple project" exception.

### Phase 10 — Final acceptance
- **Does:** Lead presents the trace summary, any flagged proposals (elements that resisted enforceable formulation or genuine gaps the producer surfaced), and the waivers list — flagged proposal by flagged proposal — as plain blocking text.
- **Reads:** the validated trace summary manifest.
- **Writes:** none new until the user rules on any flagged item.
- **Gate (`setup.gate-final-acceptance`, floor — the user's; guarded by fail-condition `setup.fail.no-acceptance`):**
  - **Branch — accepted as-is:** proceed to Phase 11.
  - **Branch — accepted with rulings on flagged proposals:** the ruled proposals route back through a scoped re-authoring pass (Phase 7) touching only those elements, then a scoped re-validation (Phase 9) before the final trace summary is re-presented.
  - **Branch — rejected:** returns to whichever earlier phase the rejection actually concerns (a synthesis-selection dispute reopens Phase 3/6; a formulation dispute reopens Phase 7).

### Phase 11 — Close
- **Does:** Confirms the goal's done condition across the set: intent ratified before authoring (Phase 6 preceded Phase 7) · trace closes GI→surfaces and was independently graded from files (Phase 9 PASS) · the governance region's version is stamped/bumped (first ratification's initial semver, per the validator's Phase 9 determination) · user accepted with the trace summary in hand (Phase 10) · feature map exists (empty scaffold, Phase 8) · architecture store's `spine.md`/`Scope:` line exists (Phase 8). Confirms none of the 6 fail-conditions stand. States, once and never again offers to delete, that `.mochiko/memory/governance-intent.md` and `governance-ledger.md` are durable (`setup.durables-never-deleted`). Suggests — never runs — a git init/commit, since this workspace isn't a git repo yet and the run never performs git mutations (`setup.no-git-mutations`). Reports in the register from `templates/output-style.md` (`setup.register`) and names the next-step doors: `/mochiko:specify` for the first feature and `/mochiko:architecture` for the architecture baseline (peer doors, neither ahead of the other), plus `/mochiko:brainstorm` if KM was adopted (`setup.next-step`, advisory).
- **Reads:** final state of all written surfaces for the closing summary.
- **Writes:** none beyond what Phases 4–8 already produced.
- **Seats/skills:** lead only.
- **Gate:** none — this phase only reports a state already gated in Phases 6, 9, and 10.

---

**Cross-cutting, applies across all phases above:**
- **Transport floor** (`setup.transport-floor`, non-waivable once triggered): this run composes more than one seat (Phase 5 reviewer(s), Phase 7 producer, Phase 9 validator) — `mochiko:patterns-transport-floor` governs the message legs (notably the Phase 5 two-message blind dispatch) and any topology legs on shared writes (e.g. sequencing Phase 7's `CLAUDE.md` write against Phase 8's scaffold writes rather than letting them race).
- **Model tiering** (`setup.model-tiering`): every seat brief carries the routing rule — locate/enumerate/targeted-read fact-finding rides a cheap `Explore` (`model: haiku`) dispatch; interpretive or absence-driven reads (e.g. judging whether a card's expression fits the project) stay on the session tier.
- **No fabricated file writes in this response:** everything marked "(would)" above is a description of the write this run would perform, not an executed action — consistent with this being a plan-only evaluation.