## Governance-check summary

Read `plugins/mochiko/schemas/setup.yaml` (109 rules across 6 sections) and `plugins/mochiko/schemas/common.yaml` raw, in full, as required before any other action. `kind: fail` count = 6 (`setup.fail.pre-ratification-authoring`, `.unclosed-trace`, `.author-graded`, `.floor-category-uncovered`, `.no-acceptance`, `.no-feature-map`) — matches the `.md`'s hard-coded Not-done count, so the run is not halted for a schema/count desync.

Workspace read: no `.mochiko/` directory, no `constitution.md` to supersede, no `.claude/rules/mochiko/` files, no governance region markers in `CLAUDE.md` (present but plain working notes only). `README.md` states explicitly: *"Nothing is built yet. This repository was initialized this week and holds only these notes — no source, no dependencies, no configuration."* Product sketch: "Cadence," an async-standup digest web app, two part-time engineers, one internal pilot team. This unambiguously reads as **greenfield** — no brownfield conflict-confrontation or feature-map-reconstruction rules (`setup.user-conflict-rulings`, `setup.user-map-confirmation`, `setup.feature-map-brownfield`) apply.

---

## Action Plan

### Phase 0 — Schema load (done)
- **Reads:** `plugins/mochiko/schemas/setup.yaml`, `plugins/mochiko/schemas/common.yaml` (raw, full — first-class source of truth, no binary/render step).
- **Writes:** none.
- **Seats:** lead only.
- **Gate:** none.

### Phase 1 — Workspace scan and mode proposal
- **Does:** Derives run shape from what the workspace shows rather than asking blind.
- **Reads:** `CLAUDE.md`, `README.md`, top-level directory listing, presence-check of `.mochiko/`.
- **Writes:** none.
- **Finding:** No source, no prior `.mochiko/` state → propose mode = **greenfield**.
- **Gate (`setup.user-mode-ruling`, reservation):** Present the proposal — *"This reads as a greenfield setup: no existing code, no prior governance. Proceed on that basis?"*
  - **User confirms greenfield** → continue to Phase 2 on the greenfield track (skip all `when: {mode: brownfield}` rules: `setup.user-conflict-rulings`, `setup.user-map-confirmation`, `setup.feature-map-brownfield`).
  - **User asserts brownfield or amend instead** → branch to the brownfield track: invoke `mochiko:analysis-codebase` to produce `.mochiko/memory/codebase-analysis.md` before interrogation, and activate the brownfield-only reservations above; or, for amend, scope to the delta per the agenda's amend depth rules and check for a pre-existing synthesis to load as baseline instead of starting fresh. Since the ruling is the user's, no default is assumed past this gate.
  - (Rest of this plan assumes the greenfield branch, per what the workspace shows.)

### Phase 2 — Inline interrogation (`setup.interrogation-inline`, duty; pointer `mochiko:analysis-iterative`)
- **Does:** Lead runs the interrogation itself, inline, one question per turn, adaptive — never delegated to a subagent.
- **Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md` (the ten dimensions), `catalog/README.md`, `catalog/universal-floor.md`, `catalog/backend-service.md` (project type will likely pull this shelf — "web app" + "morning email digest" implies backend-service + frontend, full-stack), `DOMAIN-DEPENDENCIES.md`, `COMPLIANCE-MODULES.md`.
- **Writes:** none yet (session state only, in-conversation).
- **Sub-steps, each carrying its own gate:**
  1. Work dimensions 1–10 adaptively (identity/intent, fact profile, type/shape, risk surface, team reality [2 part-time engineers — thin review culture, feeds enforcement fit], existing practices [none — greenfield, dimension 6 mostly moot], knowledge-management module offer, deployment/release reality, values/non-negotiables, deliberate exclusions).
     - **Gate — dimension 7, knowledge-management module (default-on):** *"Adopt the knowledge-management module (brainstorms index, BACKLOG.md, ROADMAP.md, decisions layer, ARCHITECTURE.md, GLOSSARY.md)? Default is yes; electives CHANGELOG.md / RUNBOOK.md are separate per-doc calls."*
       - **Adopt** → scaffold `templates/constitution-modules/knowledge-management.md` into `.mochiko/memory/knowledge-management.md` later in Phase 6 (`setup.km-module-scaffold`), record electives per-doc.
       - **Decline** → recorded as an explicit decline (also memorialized under dimension 10), no scaffold.
     - **Gate — dimension 8, release-gates module (default-on for a deployed product, always interrogated):** *"This ships to a pilot team — adopt the release-gates module?"*
       - **Adopt / decline** → recorded either way in the synthesis.
  2. **Depth-level declaration** (step 0 of the post-dimension sequence) — recommend-then-arbitrate. Recommend `low` given greenfield + two-person part-time team.
     - **Gate:** *"Depth level low or high?"* User rules; one-way (`low`→`high` only, never reverse).
  3. **Assert the floor** — the four Essential Floor cards (Security, Testing, Error Handling, Observability) enter at the ruled depth row; presented, not negotiated; type facts (web + backend digest) shape expression.
  4. **Deal the arbitrated deck** — shelf cards by type (backend-service; frontend shelf is a named gap since only backend-service is seeded — mint or adapt for frontend instead of copying misfit examples).
     - **Gate per card:** user keeps / tightens / drops / re-ranks each arbitrated card; every ruling recorded with a trace-ID.
  5. **Mint** — turn dimension-9 values into minted-principle intents traced to elicited intent.
  6. **Layered-architecture beat (conditional):** fires only if a layered-architecture card is kept or a layered intent minted — if so, record the `layer-rules` module ruling and run the domain-dependency seed arbitration.
  7. **Waiver rulings** — for any standard the user deviates from.
     - **Gate per waiver:** record standard, justification, optional revisit trigger. Legal-mandate module obligations are non-waivable; everything else is.

### Phase 3 — Synthesis authoring
- **Does:** Assemble the session record into the durable synthesis artifact.
- **Reads:** `plugins/mochiko/schemas/governance-intent.yaml` (schema shape, raw, since no `mochiko-cli` binary confirmed present — falls back to schema-as-truth per `setup.synthesis-artifact`).
- **Writes:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace) — the durable amend baseline.
- **Seats:** lead, inline (same seat that ran the interrogation).
- **Gate:** none yet — this is drafting, not ratification.

### Phase 4 — Pre-ratification stress test (cold seat)
- **Does:** Spawns an independent seat to adversarially stress-test the synthesis before ratification (`setup.stress-test-cold-seat`, `setup.blind-map-dispatch` — floor, non-waivable except by a recorded user waiver).
- **Transport:** `mochiko:patterns-transport-floor` activates (`setup.transport-floor`, `when: seats=multi`) since this is the run's first multi-seat moment — governs the dispatch/messaging discipline, non-waivable once triggered.
- **Dispatch (two messages, strictly ordered):**
  1. First message: setup topic and project identity/goal *only* — never the synthesis path. The seat (an agent invoking `mochiko:review-governance-intent`) builds its Phase-0 blind angle map from that alone.
  2. Only after the angle map returns: send `.mochiko/memory/governance-intent.md`'s path; the cold read begins.
- **Reads (by the spawned seat):** `.mochiko/memory/governance-intent.md`.
- **Writes:** none to disk from this seat; returns findings to the lead.
- **Seats/skill:** one cold reviewer agent (e.g. `mochiko:devils-advocate` or a `general-purpose` agent instructed to run `mochiko:review-governance-intent`), or a pair if the lead chooses greater rigor (`setup.staffing-latitude` — teammate/subagent count is the lead's call).
- **Gate — user waiver alternative:** if the user explicitly waives the cold stress-test, that waiver itself is recorded and substitutes for the seat; otherwise the cold seat is mandatory (this is a `class: floor` obligation, always delivered).

### Phase 5 — Survivor triage and ratification
- **Does:** Routes any coverage finding that survived the blind stress test, then gates ratification.
- **Gate A (`setup.coverage-survivor-routing`, routing):** For each surviving coverage gap — *"This questions the setup's scope, not a specific card: [gap description]. Explore now (re-enter `mochiko:analysis-iterative` on this angle, landing new GI-XXX intent), rule inline, or defer?"*
  - **Explore now** → loop back into Phase 2 on that angle, re-synthesize the affected portion, re-run Phase 4 on the delta if material.
  - **Rule inline** → user states the ruling directly; recorded in the synthesis.
  - **Defer** → recorded as a deliberate exclusion (dimension 10 territory).
  - A gap that overlaps an existing agenda dimension instead follows the ordinary interrogation-follow-up path (back into that dimension in Phase 2), not this routing gate.
- **Gate B (`setup.gate-synthesis-ratification`, floor gate, reserved to the user):** *"Here is the synthesis as it now stands [summary/diff]. Ratify it as the ground truth for authoring?"*
  - **Ratify** → proceed to Phase 6. This is the hard boundary: `setup.fail.pre-ratification-authoring` fails the entire run if any surface is authored before this point — so nothing in Phase 6+ starts until this gate clears.
  - **Reject / request edits** → loop back into Phase 2/3 to amend the synthesis, then re-offer this gate. No surface authoring occurs during this loop.

### Phase 6 — Governance surface authoring
- **Does:** Author the ratified intent onto native Claude Code surfaces — no `constitution.md` is ever produced.
- **Reads:** `.mochiko/memory/governance-intent.md` (ratified), `plugins/mochiko/skills/authoring-constitution/references/` (ESSENTIAL-FLOOR.md, catalog files, RFC-2119-KEYWORDS.md, EMERGENT-CEILING-PATTERNS.md, COMPLIANCE-MODULES.md), `templates/output-style.md`, `templates/constitution-modules/*.md` (for adopted modules).
- **Writes:**
  - `CLAUDE.md` — the marked governance region (`<!-- mochiko:governance:begin/end -->`), idempotently regenerated, everything outside untouched (`setup.governance-region-ownership`, floor).
  - `.claude/rules/mochiko/*.md` — `paths`-scoped rule files (including the domain-layer file if the layered-architecture beat fired, and `output-style.md` carrying the `mochiko:output-style` carve-out pair) — carve-outs (`mochiko:domain-registry` block, `mochiko:output-style` pair) preserved verbatim if any pre-existed (none do here, so these are fresh writes, not carve-out preservation).
  - `.mochiko/memory/governance-ledger.md` — waiver records, module attach/decline records, depth-level record.
  - Trace summary artifact tying every authored line back to a GI-XXX synthesis element (closes the `setup.fail.unclosed-trace` obligation).
  - `.mochiko/memory/knowledge-management.md` — only if the module was adopted in Phase 2 (`setup.km-module-scaffold`); never overwritten once written.
- **Seats:** `mochiko:tech-lead` (authors and updates the governance surface; does not grade its own output).
- **Gate (`setup.plan-approval-producers`):** Before writing, the tech-lead seat's authoring plan is presented for approval. *"Here's what I'll write across CLAUDE.md's governance region, the rule files, and the ledger — proceed?"*
  - **Approved** → write proceeds as planned.
  - **Changes requested** → plan revised and re-presented; no writes occur until approved.

### Phase 7 — Feature map and architecture store scaffold
- **Does:** Lands the two structural surfaces required at close, on the greenfield path.
- **Feature map (`setup.feature-map-greenfield`):**
  - **Writes:** `FEATURES.md` — scaffolded empty (no capabilities yet; nothing is built).
  - **Never-overwrite floor** (`setup.map-never-overwrite`) applies but is moot here — nothing pre-exists to overwrite.
- **Architecture store (`setup.store-scaffold-unconditional`, `setup.architecture-scope-handoff`):**
  - **Writes:** `.mochiko/product/architecture/spine.md` — header stub only, with a `Scope:` line declaring the surface types the product carries. Given "web app with a morning email digest" (UI + digest-generation backend), the scope reads as `backend-service + frontend-web` (full-stack composition). This is a handoff, not a ruling — the `/mochiko:architecture` desk owns the actual shelf walk; `setup` only declares scope, write-if-absent.
  - **Writes:** `.mochiko/product/architecture/concerns.md` — empty, created beside the stub.
  - **Boundary respected (`setup.store-ruled-content-never-here`):** No architecture stance, topology, or content beyond the header is authored here — that's reserved for the first `/mochiko:architecture` visit.
- **Baselines (`setup.baselines-bootstrap`):** Greenfield explicitly does **not** seed `.mochiko/product/{data-model.md, contracts/, constraints-and-decisions.md, quickstart.md}` now — deferred to the first `/mochiko:implement` run's design phase. Nothing written here.
- **Seats:** same authoring seat (`mochiko:tech-lead`) or `mochiko:product-manager` for the feature-map scaffold specifically, per `mochiko:authoring-feature-map` machinery — lead's staffing call.
- **Gate:** covered by the same plan-approval gate as Phase 6 (writes bundled into one approved plan), unless the lead splits them into a separate plan-approval round.

### Phase 8 — Independent grading (default FAIL)
- **Does:** Grades the authored surface set against the quality checklist, reading files, never the author's report (`setup.author-grader-default-fail`, floor).
- **Reads:** the actual authored files — `CLAUDE.md` governance region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, the trace summary, `FEATURES.md`, `.mochiko/product/architecture/spine.md` — plus `plugins/mochiko/skills/validation-constitution/references/QUALITY-CHECKLIST.md` and `ANTI-PATTERNS.md`.
- **Writes:** grading report only (in-conversation or a scratch note), not a project surface.
- **Seats:** an independent seat that authored none of Phase 6/7's output — e.g. `mochiko:validator` invoking `mochiko:validation-constitution`. Never the `tech-lead` seat that authored it.
- **Gate:** none directly user-facing here, but the outcome routes:
  - **PASS** → proceed to Phase 9.
  - **FAIL** → loop back to Phase 6/7 for revision by the authoring seat, then re-grade. This loop can repeat; `setup.fail.author-graded` only trips if the set is *never* graded by anyone but its author — a FAIL-and-revise loop with an independent grader does not trip it.

### Phase 9 — Final acceptance
- **Gate (`setup.gate-final-acceptance`, floor, reserved to the user):** Plain blocking text (`setup.acceptance-plain-text` — never a timed prompt), presented flagged-proposal by flagged-proposal — each place where elicited intent resisted clean enforceable formulation is surfaced individually for the user to rule on, plus the overall set.
  - **Accept in full** → proceed to Phase 10.
  - **Accept with named exceptions / reject specific proposals** → those specific items loop back to Phase 6 (or, if they're intent-level, back to Phase 2/3) for revision; re-grade the delta (Phase 8) before re-offering acceptance on the revised item(s).
  - **Reject wholesale** → run does not close; back to Phase 6 with the full feedback, or further back to Phase 2 if the disagreement is about elicited intent rather than authored expression.

### Phase 10 — Close
- **Does:** Finalizes bookkeeping and reports.
- **Writes:** semver bump recorded in the governance ledger / governance region (bump only, no other content change); ledger entry marking acceptance.
- **Git discipline (`setup.no-git-mutations`, floor):** Suggest a commit covering the new/changed files — never execute `git add`/`git commit`/`git push` on the user's behalf.
- **Reports:** user-facing prose per `templates/output-style.md` (`setup.register`), including the trace summary and next-step routing (`setup.next-step`, advisory) — `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline, presented as peer doors, plus `/mochiko:brainstorm` if the knowledge-management module was adopted.
- **Done-condition check (goal, restated against this run's actual state):**
  - Intent synthesis ratified before authoring — Phase 5 Gate B, satisfied if reached.
  - Trace closes across the set, independently confirmed — Phase 8 PASS.
  - Governance region semver bumped — this phase.
  - User accepted with trace summary in hand — Phase 9.
  - Feature map exists at close — `FEATURES.md` scaffolded empty (greenfield rule; correct for "nothing built yet").
  - `spine.md` stub + `Scope:` line exist — Phase 7, unconditional on every path.
  - Product baselines — correctly *absent* on greenfield, deferred to first `/mochiko:implement`.
- **Not-done set (6 fail conditions) — verified clear:** no pre-ratification authoring occurred (Phase 5 gated it), trace is closed (Phase 8 graded it), the set was graded by a non-author (Phase 8 seat), all four floor categories carry a principle or waiver (Phase 2 step 1 + Phase 6), user acceptance was given (Phase 9), and a feature map exists (Phase 7). Any one still open at this point means the run is **not done** and must not be reported as closed.