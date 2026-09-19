# Action Plan — `/mochiko:setup` (amend): data-retention principle + tightened pre-release checks

## Phase 0 — Load the rule schema (done, informs every later phase)
- **Read:** `plugins/mochiko/schemas/setup.yaml` (raw, full) and `plugins/mochiko/schemas/common.yaml` (raw, full) — binding rules, the six `kind: fail` Not-done conditions, the reserved-to-user list, the floor set.
- **Write:** none.
- **Seats:** none — the lead's own first action.
- **Gate:** none.

## Phase 1 — Reconcile current workspace state
- **Read:** `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, `CLAUDE.md`, `.claude/rules/mochiko/money-handling.md` + `output-style.md`, `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/product/architecture/spine.md`.
- **Findings that shape the run:**
  - A live governance set already exists (region v1.2.0, ledger v1.2.0) — this run is an **amend**, not greenfield/brownfield.
  - `.mochiko/memory/constitution.md` (legacy v0.9) is on disk → superseded on sight (`setup.constitution-superseded`). Flagged for deletion with a one-line notice, not silently dropped.
  - The synthesis is **legacy-shaped**: it carries no `Depth level declaration`, no `Module selections`, and no `Review` section in GI-ID form, even though the ledger already asserts `high` depth. This is a migration case the agenda names explicitly (legacy synthesis with no depth GI-element defaults to `high`, recorded without a ceremony since it never ran at `low`).
  - `release-gates` (the module the user's "tighten pre-release checks" ask maps to) has **never been ruled on** — it isn't in the (absent) Module selections table at all. Per agenda dimension 8, it is *always offered* for a deployed/operated product, and an amend run must offer once every module carrying no recorded ruling.
  - A potential fact-profile gap worth confronting: GI-001 declares jurisdiction "EU and UK" and data class "customer contact details" but records "Modules triggered: none," confirming only health-data and cardholder-data negatives — it never confirms or denies GDPR's trigger ("personal data of EU/UK residents"). Since the new ask is specifically about *how long customer data is kept* — a canonical GDPR retention/minimization concern — this conflict must be surfaced to the user in the open, never silently resolved.
- **Write:** none yet.
- **Seats:** none (lead-read; mechanical file-existence lookups of this kind would route to a haiku-tier `Explore` subagent per `mochiko:patterns-model-tiering` in a live run — already satisfied here).
- **Gate:** none.

## Phase 2 — Propose and confirm the run mode
- **Done:** State the proposal plainly: "This looks like an amend to the existing v1.2.0 governance set — proceeding in amend mode." Present as a plain blocking confirmation, never a timed prompt (`setup.acceptance-plain-text`).
- **Gate — mode ruling** (`setup.user-mode-ruling`, reserved to the user):
  - **User confirms amend** → continue to Phase 3 as scoped below.
  - **User insists on a different mode** (e.g. wants a full brownfield re-derivation instead) → abandon the delta-scoped plan, re-enter the full interrogation agenda for that mode; every later phase re-scopes accordingly (out of scope for this plan, which assumes confirmation).
- **Write:** none.

## Phase 3 — Scoped interrogation (inline, lead-conducted via `mochiko:analysis-iterative`)
Per the amend rule, this is a micro-session over the delta only — not the full ten dimensions. The delta touches:

1. **Dimension 2 (fact profile) — reopen for retention + the GDPR gap.** Ask, one question at a time: what customer-data classes are retained, for how long, what happens at expiry (hard delete vs. anonymize vs. archive), and explicitly confront the EU/UK-jurisdiction-vs-no-GDPR-trigger conflict from Phase 1. This conflict is a **reserved user ruling** (`setup.user-conflict-rulings`) — confronted openly, never resolved by the lead.
   - **Gate branch A:** user confirms GDPR does apply → the `gdpr` module attaches mechanically (legal-mandate stratum, unwaivable obligations), which is a governance event (semver MAJOR).
   - **Gate branch B:** user confirms it's out of scope (e.g. no EU/UK personal data actually processed despite the recorded fact, or an existing DPA/processor arrangement already covers it) → record the negative with its stated consequence; no module attaches; the retention principle still gets minted as an ordinary (non-legal-mandate) principle.
2. **Dimension 9 (values/non-negotiables) — mint the retention principle.** Elicit the enforceable form: the retention period itself, per data class if they differ (e.g. invoice data vs. contact details vs. bank identifiers), and what "kept" means operationally (backups included/excluded, logs included/excluded).
3. **Dimension 8 (deployment & release reality) — the "tighten pre-release checks" ask.** Since `release-gates` has never been ruled on, offer it now (default-on for a deployed product): environments, promotion rules, cadence, what currently blocks a release, rollback expectations. Distinguish explicitly from the existing per-merge Quality Gates (GI-007) — ask whether "tighten" means (a) adding release-gate content that doesn't exist today, (b) raising the existing merge-time coverage/test bar, or both.
   - **Gate branch A:** user adopts `release-gates` → its content (environments/cadence/gate table/rollback) is elicited now and routed to the module at authoring time.
   - **Gate branch B:** user declines `release-gates` even after the offer → the decline is recorded as a durable ruling (never re-asked unless reopened); "tighten pre-release checks" is then satisfied only by editing the existing Quality Gates (GI-007) values, per whatever the user specifies.
4. **Depth-level backfill.** Since the synthesis carries no GI-element for depth level, record it now as `high` (matching the ledger, no flip ceremony — it never operated at `low`).
5. **Module selections backfill.** Offer `knowledge-management` and `layer-rules` once each, since neither has a recorded ruling either — each answer (adopt or decline) is recorded either way, whatever it is; this is bookkeeping riding along with the release-gates offer, not new scope the user asked for.
6. **Waiver check.** If the user wants a shorter retention period than is operationally realistic today, or wants to defer some release-gate content, that becomes a recorded waiver (D4), never a silently loosened principle.

- **Reserved to the user throughout:** every card ruling, the GDPR conflict ruling, and every waiver (`setup.user-card-rulings`, `setup.user-conflict-rulings`).
- **Write:** none yet — this phase is elicitation only.
- **Seats:** the lead, inline.
- **Gate:** each branch above is a live in-session ruling, not a single end-of-phase gate.

## Phase 4 — Assemble the delta synthesis
- **Write:** `.mochiko/memory/governance-intent.md`, updated **delta-wise**: untouched GI-001–GI-007 keep their IDs and marks verbatim; new GI-IDs are minted for the retention principle intent, the depth-level declaration, the module-selection rows (release-gates, knowledge-management, layer-rules — each ruled either way), any GDPR fact-profile correction, and any new waiver. One Amendment Log row is appended dating this session.
- **Read (inputs to this write):** the Phase-3 answers; `plugins/mochiko/schemas/governance-intent.yaml` (already read, governs the shape).
- **Seats:** the lead authors this directly (it is the lead's pen, per `setup.stress-test-cold-seat` — hence the next phase's cold seat).
- **Gate:** none yet — ratification is Phase 6.

## Phase 5 — Pre-ratification stress test (cold seat, floor)
- **Done:** Spawn an independent reviewer (`mochiko:devils-advocate` running the `mochiko:review-governance-intent` procedure) in **two messages**, per the non-waivable dispatch floor:
  1. First message: setup topic + project identity/goal only — *not* the synthesis path. It builds a blind Phase-0 angle map (coverage lens) with no knowledge of what the interrogation concluded.
  2. Second message, only after the map returns: the synthesis path (`.mochiko/memory/governance-intent.md`) — the cold read begins.
- **Read (by the cold seat):** the updated synthesis file, blind-built angle map.
- **Write:** none by the lead in this phase; the reviewer returns findings, it does not edit the synthesis.
- **Gate — coverage-survivor routing** (`setup.coverage-survivor-routing`, reserved to the user): each surviving coverage gap is presented as a candidate topic, not silently folded in.
  - **User rules "explore now"** → re-enter `mochiko:analysis-iterative` on that angle inline; the re-elicited intent lands under fresh GI-IDs; loop back into Phase 4 to fold it in.
  - **User rules "rule inline"** → the lead records the ruling directly into the synthesis without a full re-interrogation pass; loop back into Phase 4.
  - **User rules "defer"** → the gap is recorded as a noted-but-deferred item, not folded into this run's scope; proceed to Phase 6.
  - A gap that turns out to overlap an already-open agenda dimension (e.g. more retention detail) takes the ordinary interrogation-follow-up path back into Phase 3.

## Phase 6 — Ratification gate (floor, non-waivable)
- **Done:** Present the assembled synthesis (delta highlighted against the prior version) as plain blocking text.
- **Gate — synthesis ratification** (`setup.gate-synthesis-ratification`):
  - **Ratified as-is** → proceed to Phase 7. No surface may be authored before this point (`setup.fail.pre-ratification-authoring` is one of the six fail conditions).
  - **User requests edits** → apply them to the synthesis file (still delta-wise, still tracked in the Amendment Log candidate entry) and re-present; loop until ratified.
  - **User rejects the whole delta** → stop here; nothing downstream is written; the existing v1.2.0 surface set stands unchanged.

## Phase 7 — Author the surface-set delta
- **Seats:** a producing seat (e.g. `mochiko:tech-lead`, running the `mochiko:authoring-constitution` skill) — never the lead itself, and never the same seat that will grade it.
- **Plan-approval gate** (`setup.plan-approval-producers`): the authoring seat proposes its routing plan (where the retention principle lands — likely a short imperative line in the CLAUDE.md region plus full Three-Part detail in the ledger, or a new scope-bound `.claude/rules/mochiko/data-retention.md` if the operational detail is path-identifiable; where release-gates content lands per the module-assembly table) — the lead approves before any file is touched.
  - **Approved** → seat proceeds to write.
  - **Lead sends it back** (routing looks wrong, e.g. misses a floor category) → seat revises the plan, re-submits.
- **Write (by the authoring seat, once approved):**
  - `CLAUDE.md` — governance region only, regenerated idempotently between the markers: new principle index line + universal line (if universal) for retention, updated Quality-gates / Release-gates summary line + module pointer, semver bump (MINOR for a new principle/module; MAJOR instead if the GDPR module attached), the ratified-stamp line updated. Everything outside the markers, and the two preserved carve-outs (`domain-registry` block, `output-style` switch line), is left untouched.
  - `.claude/rules/mochiko/*.md` — a new or updated scope-bound file if retention detail is path-identifiable (e.g. export/deletion job code), carrying `paths` globs tested against every path that can violate it.
  - `.mochiko/memory/governance-ledger.md` — new/updated Three-Part records (enforcement/testability/rationale) for the retention principle and the release-gates content, any new waiver row, any GDPR obligations at the legal-mandate stratum if triggered, ledger Version bumped to match the region stamp, Amendment Log row appended.
  - `.mochiko/memory/constitution.md` — deleted, with the one-line notice surfaced to the user (never authored into).
  - A **trace summary** mapping every GI element (including the untouched GI-001–GI-007) to its primary home + companion entries.
  - Anything that resists enforceable formulation (e.g. a vague "keep data as long as needed") is **not** authored as vagueness — it becomes a flagged proposal for Phase 9, never silently dropped or silently fixed.

## Phase 8 — Independent grading (author ≠ grader, default FAIL)
- **Seats:** an independent seat (`mochiko:validator`, running `mochiko:validation-constitution`) — never the Phase-7 authoring seat.
- **Read:** the authored files themselves (`CLAUDE.md`, the rules files, the ledger, the trace summary) — never the authoring seat's self-report.
- **Write:** a findings/verdict report (not the governed files).
- **Outcome branches:**
  - **PASS** → proceed to Phase 9.
  - **FAIL** (default posture until proven otherwise) → findings go back to the Phase-7 seat for revision; re-grade; loop until PASS or the user intervenes to waive a specific finding (the user may waive, the grader may not clear its own or the author's work).

## Phase 9 — Final acceptance gate (floor, non-waivable)
- **Done:** Present the trace summary and every flagged proposal from Phase 7, one at a time, as plain blocking text.
- **Gate — final acceptance** (`setup.gate-final-acceptance`):
  - **Accepted in full** → proceed to Phase 10.
  - **A flagged proposal rejected** → drop it from the surface set; if that leaves a floor category uncovered, loop back to Phase 3 to elicit a principle or a recorded waiver for it (this is one of the six fail conditions — never left silently uncovered).
  - **A flagged proposal accepted with edits** → loop back to Phase 7 for that item only, then re-grade the delta in Phase 8 before re-presenting.

## Phase 10 — Close-condition checks
- **Feature map:** amend mode makes no feature-map writes. `FEATURES.md` + `.mochiko/features/FEAT-001/entry.md` already exist and are untouched (never overwritten) — the "missing map surfaced and offered" clause doesn't fire since the map is present.
- **Architecture store:** `.mochiko/product/architecture/spine.md` already exists — the unconditional scaffold creates nothing new here. Check its header's `Scope:` line: if present, leave it alone (write-if-absent only); if somehow absent, declare `backend-service` now (from the existing GI-002 type). No architecture *stance* is taken in this run — that's the `/mochiko:architecture` desk's job.
- **Reporting:** register per `templates/output-style.md`; report the version bump, the deleted legacy `constitution.md`, and the routing next-step (`/mochiko:specify`, `/mochiko:architecture`, plus `/mochiko:brainstorm` if `knowledge-management` was adopted in Phase 3).
- **Git:** suggest a commit covering the changed files; never run a git mutation. (Note: this workspace is not currently a git repository, so this would first need the user's opt-in to `git init` before any commit could be suggested.)
- **Write:** none beyond the reporting text.

## Phase 11 — Not-done audit (default-FAIL closeout)
Before declaring the run complete, verify none of the six `kind: fail` conditions stand:
1. No surface was authored before Phase 6's ratification.
2. The trace from ratified intent to authored surfaces closes (checked by Phase 8's grader).
3. The surface set was graded by a seat other than its author (Phase 8).
4. Every Essential Floor category still carries a principle or a recorded waiver (unaffected categories carry forward from v1.2.0; nothing was newly left bare).
5. Final acceptance was given (Phase 9).
6. A feature map exists at close (already true, untouched per Phase 10).

If the schema's `kind: fail` count found at Phase 0 were ever not 6, this phase would halt and surface that mismatch instead of closing — not applicable here, since it is 6.