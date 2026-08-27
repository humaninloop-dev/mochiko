# Action Plan — `/mochiko:setup` run: "add a principle covering how long we keep customer data, and tighten the pre-release checks"

*(Plan-only. No agents dispatched, no files written, no gates actually opened — each gate below is described with its onward branches instead of being executed.)*

## Phase 0 — Load the binding schema

**Done:** Raw, full Read of `plugins/mochiko/schemas/setup.yaml` and `plugins/mochiko/schemas/common.yaml` (already completed as this run's first action, per the command's own instruction). Confirmed the `kind: fail` count in `setup.sec.fail-conditions` is exactly 6, matching the Not-done line — no halt needed.
**Read:** the two schema files above.
**Written:** nothing.
**Seats/skills:** none — lead-only, no delegation.
**Gate:** none.

## Phase 1 — Workspace discovery, mode proposal, unconditional housekeeping

**Done:** Inspect the workspace for an existing governance surface set before proposing a run mode. Findings already surfaced by reading:
- `CLAUDE.md` — carries a live governance region, `<!-- mochiko:governance:begin/end -->`, ratified v1.2.0, production floor, depth high, modules none.
- `.mochiko/memory/governance-intent.md` — synthesis with GI-001…GI-007 already ratified (2026-05-14) and amended once (coverage floor, 2026-06-02).
- `.mochiko/memory/governance-ledger.md` — v1.2.0, Three-Part metadata for GI-003 and GI-005, one recorded waiver (load/perf testing).
- `.claude/rules/mochiko/money-handling.md` and `output-style.md` — both present and path-scoped.
- `.mochiko/memory/constitution.md` — a stray pre-migration v0.9 doc, explicitly superseded by the CLAUDE.md region.
- `FEATURES.md` + `.mochiko/features/FEAT-001/` — feature map already present (Invoice drafting, `selected`).
- `.mochiko/product/architecture/spine.md` — present, `Scope: backend-service` already declared; `concerns.md` is **missing** from that directory.

Because a full, ratified surface set already exists, the workspace is unambiguous: this is an **amend** run, not greenfield/brownfield.

Two unconditional, ratification-independent housekeeping actions apply regardless of what the interrogation concludes:
- Delete `.mochiko/memory/constitution.md` (superseded — `setup.constitution-superseded`), with a one-line disclosure to the user in the same turn.
- Create the missing `.mochiko/product/architecture/concerns.md` as an empty file (`setup.store-scaffold-unconditional` — the scaffold is unconditional and creates only what's missing; `spine.md`'s `Scope:` line is already populated, so it's left alone per write-if-absent).

**Read:** all files listed above (already done).
**Written:** `.mochiko/memory/constitution.md` (deleted), `.mochiko/product/architecture/concerns.md` (created empty).
**Seats/skills:** lead-only; no delegation (these are mechanical, unconditional binding obligations, not judgment calls).

**Gate — mode confirmation** (`setup.user-mode-ruling`, reservation): present the proposed mode ("amend, based on the existing ratified v1.2.0 surface set") to the user for confirmation.
- *User confirms amend* → proceed to Phase 2 scoped as an amendment against the existing GI-001…GI-007 baseline.
- *User overrides toward a full re-interrogation (e.g. treats this as a fresh baseline reset)* → out of the scope implied by the request; would restart the agenda from Phase 0 of the interrogation with the full catalog deck rather than the two touched topics — flagged as a much larger run and confirmed explicitly before proceeding that way.

## Phase 2 — Inline interrogation on the two touched topics

**Done:** Run the interrogation myself, inline (`setup.interrogation-inline` — never delegated to a seat), using `mochiko:analysis-iterative` for adaptive one-question-at-a-time discovery, then the catalog deck card-by-card, recommend-then-arbitrate. Scoped as an amendment, so only cards touched by the two requested topics are worked, against the existing synthesis as baseline:

- **Topic A — customer data retention.** Which data classes from the existing fact profile (GI-001: contact details, invoice line items, bank account identifiers) does the retention period cover — all three or a subset; the retention duration per class; the jurisdictional driver (EU/UK per GI-001 — UK/EU tax law typically requires invoice records for 6–7 years, which may conflict with a shorter "delete promptly" instinct); the deletion/anonymization mechanism; the triggering event (account closure vs invoice finalization + N years); and any exceptions (legal hold). Also check whether this trips a privacy/PII module in the catalog deck that the current fact profile didn't already attach.
- **Topic B — tightening pre-release checks.** What exists today (GI-007: test suite green, coverage ≥70%) and what's judged insufficient; candidate new gates (migration dry-run, dependency/security scan, manual QA sign-off, changelog entry); whether the superseded constitution's old Article III ("tag a release on Fridays only if somebody is around") is back in scope or stays retired; whether this raises the coverage threshold, adds parallel gates, or both; NON-NEGOTIABLE vs advisory.

**Read:** `${CLAUDE_PLUGIN_ROOT}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`, scoped to the cards these two topics touch (`setup.interrogation-inputs`).
**Written:** nothing yet — this phase is dialogue only.
**Seats/skills:** lead-only, via `mochiko:analysis-iterative`. Any locate/enumerate fact-finding sub-lookups during this phase (e.g. scanning the deck for a matching card) route through `mochiko:patterns-model-tiering` to a haiku-model `Explore` subagent; interpretive judgment stays on the session tier.

**Gate — every card ruling, every waiver** (`setup.user-card-rulings`, reservation): each card is presented recommend-then-arbitrate; the user rules each one.
- *User accepts the recommendation* → the ruling locks in as stated.
- *User overrides the value* (e.g. a different retention window, a different gate set) → the override locks in instead.
- *User defers a card* → noted as open, carried forward rather than silently dropped, and excluded from this run's ratified synthesis.

## Phase 3 — Draft synthesis, pre-ratification stress test, ratification

**Done:** Draft the updated synthesis myself directly — "the synthesis is your pen" (`setup.stress-test-cold-seat`) — as an amendment to the existing GI-001…GI-007 set:
- Mint **GI-008 — Customer data retention** (new principle; home likely the CLAUDE.md governance region, mirroring GI-003, possibly with a dedicated `.claude/rules/mochiko/data-retention.md` if it needs path-scoped detail like `money-handling.md`) — exact wording and home fixed by Phase 2's rulings, not preempted here.
- Amend **GI-007 — Quality gates** in place (consistent with the existing precedent of the 1.0.0→1.2.0 coverage-floor raise staying under GI-007) to reflect the tightened pre-release checks.
- Append an Amendment log row proposing the next version (provisionally 1.2.0 → 1.3.0 under the ledger's semver table — MINOR for new principle/waiver-shaped change; escalates to MAJOR only if a ruling turns out floor-level or incompatible with an existing NON-NEGOTIABLE).

Then dispatch the **pre-ratification stress test** — a cold seat, always required unless the user records an explicit waiver (`setup.stress-test-cold-seat`). Two-message dispatch, non-waivable once triggered (`setup.blind-map-dispatch`, floor):
1. First message: project identity and goal only ("Peartree — invoicing for freelancers; amend run adding a customer-data-retention principle and tightening pre-release checks") — never the synthesis path. The seat builds its Phase 0 blind angle map and returns it.
2. Second message, only after the map returns: the synthesis path (`.mochiko/memory/governance-intent.md`) so the seat runs its cold read and cross-examination per `mochiko:review-governance-intent`.

**Read (by the stress-test seat):** `.mochiko/memory/governance-intent.md` (post-draft), the fact profile and existing GI entries for consistency.
**Written:** `.mochiko/memory/governance-intent.md` (draft update with GI-008 and amended GI-007, pending ratification).
**Seats/skills:** an independent cold seat (e.g. `mochiko:devils-advocate`, or a paired instance) running `mochiko:review-governance-intent`; never the lead, never a participant in Phase 2.

**Gate A — coverage-survivor routing** (`setup.coverage-survivor-routing`, conditional on the stress test surfacing a gap outside the two agreed topics): each surviving gap presented as a candidate topic.
- *User says explore now* → re-enter `mochiko:analysis-iterative` on that angle; re-elicited intent lands in the GI-XXX namespace; loop back into Phase 2/3.
- *User rules inline* → resolved directly without a full re-interrogation.
- *User defers* → noted, not resolved this run.
- *(A gap that overlaps an already-worked agenda dimension instead follows the ordinary interrogation-follow-up path — no separate gate.)*

**Gate B — synthesis ratification** (`setup.gate-synthesis-ratification`, floor, always delivered): present the full draft synthesis (GI-008 text, amended GI-007 text, proposed version bump) as plain blocking text.
- *User ratifies* → proceed to Phase 4; no surface (CLAUDE.md region / rules files / ledger) may be authored before this point (`setup.fail.pre-ratification-authoring` is a hard FAIL otherwise).
- *User requests changes* → loop back into Phase 2 on the specific point, redraft, re-present.
- *User declines the amendment entirely* → run ends with no surface-set writes; the existing v1.2.0 set stays authoritative; only the Phase 1 housekeeping (constitution.md deletion, concerns.md scaffold) persists, since those are unconditional and independent of this ratification.

## Phase 4 — Author the governance surface set

**Done:** Once ratified, update the surface set per `mochiko:authoring-constitution`'s composition rules:
- Regenerate the CLAUDE.md governance region idempotently: add the GI-008 retention principle line, update the GI-007 quality-gates lines, bump the ratified-version stamp, preserve the two marked carve-outs verbatim (`mochiko:domain-registry` block if present, and the `mochiko:output-style` switch line) — never touching anything outside the `<!-- mochiko:governance:begin/end -->` markers.
- Add or update the path-scoped rules file(s) under `.claude/rules/mochiko/` — e.g. a new `data-retention.md` if GI-008's enforcement needs file-scoped detail, or fold it into the region if it's cross-cutting.
- Update `.mochiko/memory/governance-ledger.md`: Three-Part metadata (enforcement/testability/rationale) for GI-008 and the amended GI-007, version bump matching the region stamp, amendment log row.
- Feature map: **no write** — an amend run makes none (`setup.map-never-overwrite`); `FEATURES.md` already exists so `setup.fail.no-feature-map` doesn't apply.
- Architecture store: no ruled content authored here — only the already-completed unconditional scaffold from Phase 1 (`setup.store-ruled-content-never-here`).

**Read:** current CLAUDE.md, `money-handling.md`, `output-style.md`, the ratified synthesis from Phase 3.
**Written:** `CLAUDE.md` (governance region only), `.claude/rules/mochiko/data-retention.md` (new, if scoped that way), `.claude/rules/mochiko/*.md` amendments as needed, `.mochiko/memory/governance-ledger.md`.
**Seats/skills:** either the lead directly via the `mochiko:authoring-constitution` skill, or a delegated `mochiko:tech-lead` seat working from a lead-approved plan (`setup.plan-approval-producers` — staffing choice is latitude, `setup.staffing-latitude`).
**Gate:** none new here — this phase only proceeds because Gate B in Phase 3 already cleared.

## Phase 5 — Independent grading

**Done:** Grade the authored surface set from the files themselves, never from the authoring report, by a seat that authored none of it (`setup.author-grader-default-fail`, floor, default FAIL). Run `mochiko:validation-constitution` against: the CLAUDE.md region, the `.claude/rules/mochiko/` files, and the ledger — checking the ratified-intent → authored-surface trace closes, the version bump is correct, and every Essential Floor category still carries either a principle or a recorded waiver. Optionally follow with `mochiko:testing-governance-injection` as an empirical regression check that the new/changed rules actually inject on their promised paths.

**Read:** the same surface-set files as authored in Phase 4 (fresh read, not the authoring report).
**Written:** nothing (grading output only, reported to the user).
**Seats/skills:** `mochiko:validator` (or an equivalent independent seat) running `mochiko:validation-constitution`; must be a different seat instance than whoever authored Phase 4.
**Gate:** none directly, but a FAIL verdict loops back to Phase 4 (or Phase 2/3 if the gap is in the synthesis itself) and re-grades — this is a loop, not a user gate.

## Phase 6 — Final acceptance

**Done:** Present the trace summary — GI-008 (new) and GI-007 (amended) text, files touched, version bump (e.g. 1.2.0 → 1.3.0), and confirmation that Phase 5's grade passed — flagged proposal by flagged proposal, not as one blanket approval.
**Read:** nothing new.
**Written:** nothing yet — pending the gate.
**Seats/skills:** lead-only, reporting.

**Gate — final acceptance** (`setup.gate-final-acceptance`, floor; delivered as plain blocking text, never a timed prompt — `common.acceptance-plain-text`):
- *User accepts all flagged proposals* → run closes as done.
- *User accepts some, rejects others* → accepted pieces stay; rejected pieces are backed out of the Phase 4 writes, the surface set is re-authored minus those pieces, re-graded (back to Phase 5) on the changed portion, then re-presented for acceptance on the remainder.
- *User rejects entirely* → no acceptance given → `setup.fail.no-acceptance` stands; the run does not close as done; either abandon (existing v1.2.0 stays authoritative beyond the Phase 1 housekeeping) or loop back into Phase 2 to rework.

## Phase 7 — Not-done audit (default FAIL)

**Done:** Walk the 6 `kind: fail` rules and confirm none stands before declaring the run closed:
1. `pre-ratification-authoring` — surface writes only happened after Gate B (Phase 3) ✓ if the ordering above was followed.
2. `unclosed-trace` — Phase 6's trace summary closes ratified intent → authored surfaces ✓.
3. `author-graded` — Phase 5's grader was independent of Phase 4's author ✓.
4. `floor-category-uncovered` — confirmed by Phase 5's validator run ✓.
5. `no-acceptance` — Phase 6 produced explicit acceptance ✓.
6. `no-feature-map` — `FEATURES.md` was already present at entry; amend made no feature-map writes ✓.

Also confirm the close condition's feature-map/baseline split: feature map present (already true), architecture store's `spine.md`/`concerns.md` scaffold complete (Phase 1) — both satisfied.

**Read:** the final state of every file touched.
**Written:** nothing new.
**Seats/skills:** lead-only.
**Gate:** none — this is an internal self-check, not a user-facing gate.

## Phase 8 — Report and next steps

**Done:** Deliver the user-facing register per `templates/output-style.md`, respecting this project's own carve-out (conversation `lite`, reports `ultra`, documents `full`, unless "normal mode" was invoked): summarize what changed (GI-008 added, GI-007 tightened, version bump, files touched), route to next steps (`/mochiko:specify`, `/mochiko:architecture` as peer doors — no `/mochiko:brainstorm` suggestion since knowledge-management module is `none`), and suggest (never execute) a commit covering the amendment — no git mutation or push is ever run by this run (`common.no-git-mutations`, floor).
**Read:** nothing new.
**Written:** nothing new.
**Seats/skills:** lead-only, reporting.
**Gate:** none.