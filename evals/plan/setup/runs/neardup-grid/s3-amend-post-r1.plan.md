# Action Plan — `/mochiko:setup` (amend): add a customer-data-retention principle, tighten pre-release checks

*(Plan-only. No seats spawned, no files written, no gates awaited — each gate is described with its branches.)*

## Phase 0 — Schema load & workspace read (done)
- Read `plugins/mochiko/schemas/setup.yaml` and `plugins/mochiko/schemas/common.yaml` raw, in full — binding rules for this run.
- Read current state: `CLAUDE.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/governance-intent.md`, `.claude/rules/mochiko/money-handling.md`, `.mochiko/memory/constitution.md`, and confirmed `FEATURES.md` / `.mochiko/features/FEAT-001/entry.md` / `.mochiko/product/architecture/spine.md` exist.
- Findings: this is project **Peartree**, an existing ratified surface set at **v1.2.0** (production floor, depth high, modules: none). A stale `.mochiko/memory/constitution.md` (v0.9, pre-dates the current surfaces) is on disk.

## Phase 1 — Mode proposal
- Propose mode **amend** — a ratified governance region, ledger, and intent file already exist; the request is additive/tightening, not a fresh setup.
- Not ambiguous, so `setup.user-mode-ruling` doesn't force a hard gate, but I'd state the proposed mode plainly and let the user override it if they disagree before proceeding.

## Phase 2 — Flag the stale artifact
- Note `.mochiko/memory/constitution.md` (v0.9) is superseded per `setup.constitution-superseded` — plan to delete it on sight and say so in one line during the run. No other action taken on it.

## Phase 3 — Inline interrogation (lead-run, card by card, recommend-then-arbitrate)
Two cards, worked against `INTERROGATION-AGENDA.md` / the catalog deck / `DOMAIN-DEPENDENCIES.md` for grounding, not invented freehand:

- **Card A — customer data retention.** Given the fact profile already on file (customer contact details, invoice line items, bank account identifiers; EU/UK jurisdictions), interrogate: retention duration per data class, legal basis (e.g. UK/EU tax-record retention norms vs. GDPR storage-limitation), the purge/deletion mechanism, backup retention, and any legal-hold exception. Recommend candidate durations grounded in the jurisdiction facts already ratified; the user rules the actual figures and policy.
- **Card B — tighten pre-release checks.** Today's quality gates are `npm test` passing and ≥70% coverage. Interrogate what "tighten" means concretely: raise the coverage bar, add a blocking lint/typecheck step, add a dependency/security audit, add a migration dry-run check, or formalize the two-approval / weekend-coverage release discipline that's currently only informal (echoed, unenforced, in the stale v0.9 constitution's Articles II/III). Recommend options, user arbitrates which land and at what threshold.
- Every card ruling and any waiver is reserved to the user (`setup.user-card-rulings`) — I do not decide these, only propose.

## Phase 4 — Draft the synthesis
- Update `.mochiko/memory/governance-intent.md`: append new `GI-XXX` entries for the data-retention principle and the tightened/new quality-gate clause(s), plus an Amendment-log row. This is a draft — not yet ratified, nothing else touched yet.

## Phase 5 — Pre-ratification stress test (cold seat, blind dispatch)
- Spawn an independent seat (`mochiko:review-governance-intent`) in two messages, per `setup.blind-map-dispatch`:
  1. Setup topic + project identity/goal only ("Peartree invoicing — amend adding data-retention principle + tightened pre-release checks"), **never** the synthesis path — seat builds its Phase-0 blind angle map and returns it.
  2. Only then send `.mochiko/memory/governance-intent.md`'s draft path — seat does its cold read, hunt classes, cross-examination, and returns severity-classified findings + verdict.
- Alternative branch: the user may instead record an explicit waiver of this stress test (`setup.stress-test-cold-seat`) — if so, this phase is skipped and the waiver is logged.
- Any coverage finding that survives (e.g. a related gap like breach notification or backup-retention nuance) is routed to the user as a candidate topic, not a blocker: **user gate** — branches are (a) explore now via a fresh interrogation card, (b) rule it inline immediately, or (c) defer it out of this run.

## Phase 6 — Ratification gate
- **User gate:** the final synthesis text (new principle wording, retention figures, tightened gate criteria, and the semver bump — MINOR, e.g. 1.2.0 → 1.3.0, per the ledger's own amendment policy since this is new-principle/gate-change, not a floor-level or principle-removal change) is presented for ratification. This is the point before which no surface may be authored (`setup.gate-synthesis-ratification`, non-waivable floor).
- Branches:
  - **Ratify as-is** → proceed to Phase 7.
  - **Request changes** → revise synthesis, loop back through Phase 6 (and, only if the edit is material, a bounded delta re-check of Phase 5 rather than a full restart).
  - **Ratify part, defer part** (e.g. ratify the retention principle, defer the release-gate tightening) → only the ratified portion proceeds to authoring this run; the deferred portion is recorded as open, not authored.

## Phase 7 — Authoring (only after ratification)
Delegated to a producer seat (`mochiko:authoring-constitution`), which plans first and works only on a lead-approved plan (`setup.plan-approval-producers`). Planned writes:
- `CLAUDE.md` governance region only (between the `mochiko:governance:begin/end` markers; the `mochiko:domain-registry` and `mochiko:output-style` carve-outs preserved verbatim; nothing outside the markers touched): add the new principle bullet(s) under **Principles**, update **Quality gates** bullets, bump the `**Ratified:**` version/date stamp.
- `.claude/rules/mochiko/` — possibly a new dedicated rules file (e.g. a paths-scoped `data-retention.md`, mirroring the existing `money-handling.md` pattern) if the retention principle needs enforcement detail beyond a CLAUDE.md bullet; decided by where the ratified synthesis homes it.
- `.mochiko/memory/governance-ledger.md` — add Three-Part metadata (Enforcement/Testability/Rationale/Trace) for each new/changed GI-ID, append an Amendment-log row, bump the version stamp to match the region.
- `.mochiko/memory/governance-intent.md` — finalize its own Amendment-log row (durable amend baseline).
- Delete `.mochiko/memory/constitution.md`, noted in one line.
- No feature-map or architecture-store writes needed — both already exist and the never-overwrite floor holds; nothing here is "missing."
- No git mutations — commits only suggested, never run or pushed (`setup.no-git-mutations`).

## Phase 8 — Independent grading (author ≠ grader, default FAIL)
- `mochiko:validation-constitution`, a seat distinct from the Phase 7 author, reads the authored files directly (never the author's report) and defaults to FAIL until proven otherwise (`setup.author-grader-default-fail`). Checks: trace closes end-to-end from ratified GI-IDs to authored surfaces; Essential Floor categories still have a principle or recorded waiver; carve-outs and region markers intact; ledger version matches region stamp.
- **FAIL branch:** findings routed back to the Phase 7 seat for revision, re-graded; loop until PASS or the user rules otherwise.

## Phase 9 — Trace summary & final acceptance gate
- Assemble a trace summary (ratified GI-IDs → their authored surface locations) for the user.
- **User gate:** final acceptance, flagged proposal by flagged proposal, as plain blocking text — never a timed prompt (`setup.gate-final-acceptance`).
- Branches:
  - **Accept all** → proceed to close.
  - **Accept some, reject/rework others** → accepted items close; rejected/reworked items loop back to Phase 7/8.
  - **No acceptance given** → `setup.fail.no-acceptance` trips; run is Not-done.

## Phase 10 — Close & report
- Verify the done condition: feature map present (already true, untouched), governance region semver bumped and matching the ledger, trace closed and independently graded, user acceptance recorded.
- Verify none of the 6 `kind: fail` conditions stand: pre-ratification authoring, unclosed trace, author-graded-by-self, floor-category uncovered, no-acceptance, no-feature-map.
- Report next steps (advisory): `/mochiko:specify` and `/mochiko:architecture` as peer next doors; `/mochiko:brainstorm` is not offered since no knowledge-management module is attached.

## Cross-cutting notes
- **Multi-seat transport:** this run composes ≥2 independent seats (authoring, cold stress-test, grading), which trips the non-waivable transport floor (`mochiko:patterns-transport-floor`) for all cross-seat messaging and the shared writes in Phase 7.
- **Model tiering:** pure locate/enumerate reads (e.g. "does a rules file already reference retention or release checklists") route to a cheap `Explore` dispatch (`model: haiku`); interpretive reads (e.g. how to word the new ledger entries) stay on the session tier.