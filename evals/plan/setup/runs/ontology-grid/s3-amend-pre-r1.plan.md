# Action Plan — `/mochiko:setup add a principle covering how long we keep customer data, and tighten the pre-release checks`

*(Plan-only evaluation run — nothing below was executed; no seat was spawned, no file was written.)*

## Phase 0 — Load the binding schema (done as part of planning)

- **Reads:** `plugins/mochiko/schemas/setup.yaml` (raw, whole — all six sections), `plugins/mochiko/schemas/command-labels.yaml`.
- **Result:** 6 `fail-condition` rules confirmed present in `setup.sec.fail-conditions`, matching the command `.md`'s hard-coded count — no halt triggered.
- **Writes:** none.
- **Seats:** none — lead-only, first-class raw read, no binary/render step.

## Phase 1 — Read workspace state to ground the mode proposal

- **Reads (already performed for this plan):**
  `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, `CLAUDE.md`, `.claude/rules/mochiko/money-handling.md`, `.claude/rules/mochiko/output-style.md`, `.mochiko/product/architecture/spine.md`, `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`.
- **Findings that shape the run:**
  - A ratified synthesis and ledger already exist (v1.2.0, "Peartree", depth `high`, floor `production`, modules: none). This is **not** a fresh project.
  - A stale `.mochiko/memory/constitution.md` (pre-region v0.9) sits on disk — per `setup.constitution-superseded`, this is superseded on sight regardless of what else this run does.
  - The fact profile already declares EU/UK jurisdictions and customer contact details as a data class, yet records "Modules triggered (mechanical): none" — no `gdpr` module attached. Per `COMPLIANCE-MODULES.md`'s trigger table, "Personal data of EU/UK residents" triggers `gdpr` (legal-mandate, unwaivable). A retention-duration principle is a canonical GDPR storage-limitation obligation, so this request plausibly **reopens** that fact rather than being a clean bolt-on principle.
  - `release-gates` (the module that carries pre-release/deployment-blocking checks, as opposed to the per-merge Quality Gates already in the region under GI-007) is **not attached**. "Tighten the pre-release checks" could mean (a) tightening GI-007's existing per-merge gates, and/or (b) adopting `release-gates` for the first time. Both are live candidates to arbitrate, not a foregone conclusion.
  - Feature map (`FEATURES.md` + `FEAT-001`) and the architecture store scaffold (`spine.md`, `Scope: backend-service`) already exist — outside this amend's scope, not touched.
- **Writes:** none.

## Phase 2 — Propose the run mode

- **Proposal:** **amend** mode — justified by the existing ratified region/ledger/synthesis found in Phase 1, not a greenfield or brownfield-analysis entry.
- **Gate (`setup.user-mode-ruling`, user-gate):** Present the proposal and the evidence for it; the user rules.
  - **User confirms amend** → proceed to Phase 3 as a delta-scoped micro-session.
  - **User names a different mode** (e.g. wants a full re-run, or disputes that this is the same project) → restart mode selection under their ruling before any interrogation; the rest of this plan would be re-scoped accordingly.
  - **User is silent/ambiguous** → do not assume; re-ask narrowly rather than default.

## Phase 3 — Scoped amend interrogation (lead-run inline, `mochiko:analysis-iterative` engine)

- **Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, `COMPLIANCE-MODULES.md`, `catalog/universal-floor.md` (FLOOR-TEST card), `templates/constitution-modules/release-gates.md`; a targeted, locate-style scan of the codebase for existing data-deletion/retention or scheduled-job code, dispatched to a native **Explore** subagent (`model: haiku`) per `mochiko:patterns-model-tiering` — this is a locate/enumerate read, not an interpretive one, so it rides the cheap tier.
- **Dimensions worked adaptively** (per `setup.interrogation-inline`, amend gets "the relevant agenda slice," not all ten):
  - **Dimension 2 (fact profile)** — reopen the EU/UK personal-data fact explicitly; consequence-stated confirmation of whether `gdpr` attaches now. This is the load-bearing question: if GDPR attaches, the retention principle becomes a **legal-mandate, unwaivable** obligation with an audit-evidence stratum; if the user rules it stays out (e.g. narrows scope), the retention principle is an ordinary contractual/product principle instead.
  - **Dimension 8 (deployment & release reality)** — re-offer `release-gates` (always re-offerable, "recorded either way" per the agenda) and/or arbitrate tightening GI-007's existing blocking checks; source real environment names, cadence, and verification commands from the user, never placeholders.
  - **Dimension 9 (values & non-negotiables)** — elicit the actual retention duration as enforceable behavior ("what should a job/review block on?"), not a mood.
  - **Dimension 10 (deliberate exclusions)** — record anything explicitly out of scope for this amend.
- **Mint:** turn the retention-duration answer into a minted-principle intent (new `GI-XXX`); record the `release-gates`/GI-007 ruling as a deck/module ruling.
- **User-card-rulings gate (`setup.user-card-rulings`):** every card, module, and waiver ruling in this phase is the user's — recommend-then-arbitrate, never asserted. Branches: user accepts a recommendation as-is / tightens it / drops it / re-ranks it / defers a further question to a later dimension. Each branch is recorded in the synthesis before moving on.
- **Writes:** none yet — this phase only accumulates the delta in working memory toward the synthesis draft in Phase 4.

## Phase 4 — Draft the amended synthesis

- **Writes:** `.mochiko/memory/governance-intent.md` — updated in place: new `GI-XXX — Customer data retention` fact/principle entry (marked by stratum — legal-mandate if GDPR attaches, else ordinary), any `GI-YYY — Release gates` or GI-007 tightening entry, the module-attach ruling (`gdpr` attach/no-attach, `release-gates` attach/no-attach) recorded in the Fact profile / Modules-triggered section, and an amendment-log row for the pending version bump.
- **Reads:** `plugins/mochiko/schemas/governance-intent.yaml` raw (shape reference, since the binary template isn't invoked in this plan-only pass).
- **Seats:** lead-authored (the synthesis is explicitly the lead's pen per `setup.stress-test-cold-seat`) — not delegated to a producer seat.

## Phase 5 — Pre-ratification stress test (cold seat, blind two-message dispatch)

- **Seat:** one cold reviewer (e.g. a `mochiko:tech-lead` or `mochiko:devils-advocate` instance) running `mochiko:review-governance-intent`, spawned per `setup.blind-map-dispatch` (floor, non-waivable): **message 1** carries only the setup topic / project identity ("Peartree, invoicing for freelancers, EU/UK") and the goal — never the synthesis path — so its Phase-0 angle map forms blind; its map must return before **message 2** sends `.mochiko/memory/governance-intent.md`'s path and the cold read begins.
- **Reads (by the seat, not the lead):** the synthesis file itself once released.
- **Writes:** none by the seat; it returns findings only.
- **Gate — user waiver alternative (`setup.stress-test-cold-seat`):** if the user instead waives the cold stress-test, that waiver is recorded and this phase is skipped; default is to run it.

## Phase 6 — Route any coverage survivors

- **Gate (`setup.coverage-survivor-routing`, user-gate):** any finding surviving the stress test that questions the amend's *scope* (e.g. "you never actually confirmed whether GDPR should attach" or "the retention duration you recorded conflicts with the money-handling audit trail") is presented as a candidate topic, not silently folded in.
  - **User rules "explore now"** → re-enter `mochiko:analysis-iterative` on that angle inline; the re-elicited intent lands in the `GI-XXX` namespace and Phase 4's draft is amended before re-stress-testing if material.
  - **User rules "inline"** → the lead rules on the narrow point directly and records it.
  - **User rules "defer"** → recorded as a deliberate exclusion (dimension 10) and left out of this run's surface set.
  - A finding that overlaps an already-worked agenda dimension (2, 8, 9) takes the ordinary interrogation-follow-up path instead of a fresh topic gate.

## Phase 7 — Synthesis ratification

- **Gate (`setup.gate-synthesis-ratification`, floor, user-gate):** present the finished synthesis (with the stress-test resolved) plainly, blocking text, no timed prompt (`setup.acceptance-plain-text`). The user confirms / edits / rejects.
  - **Confirm** → proceed to Phase 8; this is the point after which no surface may be authored (violating it trips `setup.fail.pre-ratification-authoring`).
  - **Edit** → apply the edit to `.mochiko/memory/governance-intent.md`, re-present for confirmation (loop, not a silent accept).
  - **Reject** → return to Phase 3/6 to rework the disputed element; no authoring occurs.

## Phase 8 — Author the amended governance surface set

- **Seat:** one producer seat (e.g. `mochiko:tech-lead`, which owns "authors and updates the governance surface") running `mochiko:authoring-constitution`, plan-approved first per `setup.plan-approval-producers` — the seat's authoring plan (which surfaces change, what routes where) is shown to the user before it writes anything.
- **Reads (by the seat):** the ratified `.mochiko/memory/governance-intent.md`, `references/catalog/universal-floor.md` (FLOOR-TEST card, for the GI-007 tightening if ruled), `templates/constitution-modules/release-gates.md` (if adopted), `references/COMPLIANCE-MODULES.md` (if `gdpr` attached), `plugins/mochiko/schemas/governance-surfaces.yaml` raw (artifact shapes), current `CLAUDE.md`, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md` (to preserve untouched principles verbatim and their GI-IDs).
- **Writes:**
  - `CLAUDE.md` — governance region only, regenerated idempotently between the markers: new index line(s) for the retention principle and (if adopted) `release-gates`/tightened GI-007 line, ratified-stamp semver bump, module list updated if `gdpr`/`release-gates` attached. The output-style carve-out and any `mochiko:domain-registry` block are re-emitted unchanged, never regenerated over the user's values. Everything outside the markers is untouched.
  - `.claude/rules/mochiko/*.md` — a new scope-bound file only if the retention principle's enforcement is code-path-specific (e.g. a deletion/export job's path glob); otherwise the principle stays region-only as a universal line. `output-style.md` is refreshed only to track the preserved switch values.
  - `.mochiko/memory/governance-ledger.md` — new Three-Part entries (Enforcement/Testability/Rationale) for the retention principle and any release-gates/GI-007 change, module attach row if `gdpr` triggers (stratum: legal-mandate, unwaivable), amendment-log row, waiver table update if anything is waived.
  - **Deletion:** `.mochiko/memory/constitution.md` — deleted on sight per `setup.constitution-superseded`, called out in one line in the run's reporting (unconditional, independent of the rest of this amend).
  - A **trace summary** (part of this same output, not a separate gate) mapping every touched `GI-XXX` to its home(s).
- **Boundary respected:** no unsanctioned selection — if authoring surfaces a genuine gap (e.g. "retention duration was agreed but no deletion mechanism exists in code") it is emitted as a **flagged proposal**, not silently authored or silently dropped.

## Phase 9 — Independent grading

- **Seat:** a **different** instance (never the Phase 8 author) running `mochiko:validation-constitution`, defaulting to FAIL until proven otherwise, per `setup.author-grader-default-fail` (floor) and `setup.fail.author-graded`.
- **Reads (by the grader, from the files themselves, never the author's report):** the regenerated `CLAUDE.md` region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, the trace summary, cross-checked against the ratified `governance-intent.md`.
- **Checks specifically relevant here:** trace closure for the new `GI-XXX` retention entry and any release-gates/GI-007 change; if `release-gates` was attached, the module's validator-checklist fragment (real environment names, no placeholder gates, rollback with a time expectation); if `gdpr` attached, that the obligation is marked unwaivable in the ledger.
- **Writes:** none — a grading report only (findings + PASS/FAIL), fed into Phase 10.
- **On FAIL:** loop back to Phase 8 with the concrete fix list; the author revises, the same independent grader re-reads the files (never the revision report) before Phase 10 opens.

## Phase 10 — Final acceptance

- **Gate (`setup.gate-final-acceptance`, floor, user-gate):** the grade-confirmed set is presented **flagged proposal by flagged proposal** (any point flagged in Phases 6/8) as plain blocking text, never a timed prompt.
  - **User accepts** → run closes; `CLAUDE.md`'s governance region semver is bumped (MINOR for a new principle/waiver change, or MAJOR if `gdpr`/`release-gates` module attach occurred — module attach/detach is a MAJOR-triggering event per the ledger's amendment policy already on file), acceptance recorded.
  - **User rejects a specific flagged item** → that item is reworked (back to the relevant earlier phase) while accepted items stand; nothing is force-closed.
- **Reporting register:** `templates/output-style.md` conventions (`setup.register`); next-step advisory (`setup.next-step`, advisory) — `/mochiko:specify` and `/mochiko:architecture` as peer next doors, plus `/mochiko:brainstorm` since `knowledge-management` was never adopted here (unaffected by this amend) only if it's later adopted.
- **Git discipline:** per `setup.no-git-mutations` (floor), the run would only *suggest* a commit for the changed files — it would never run `git add`/`git commit`/`git push` itself.

## Phase 11 — Not-done check (default FAIL)

Before declaring the run closed, verify none of the 6 fail-conditions stand:
1. No surface was authored before Phase 7's ratification. ✅ by construction (Phase 8 gated behind Phase 7).
2. Trace closes from ratified `GI-XXX` entries to every touched surface (region line → ledger entry → rules file if any). ✅ checked in Phase 9.
3. The surface set was graded by a seat other than its author. ✅ Phase 9's separate seat.
4. Every floor category (Security/Testing/Error-Handling/Observability) still carries a principle or recorded waiver after the amend — this amend doesn't touch those categories, so their prior standing (untouched, already present) must be confirmed unchanged, not silently dropped in the regeneration.
5. User acceptance was given (Phase 10). 
6. The feature map exists at close — already true (`FEATURES.md` + `FEAT-001`), untouched by this amend, so trivially satisfied.

If any of these is not actually true when the run reaches this point, the run is FAILed rather than reported as done — no partial-success framing.