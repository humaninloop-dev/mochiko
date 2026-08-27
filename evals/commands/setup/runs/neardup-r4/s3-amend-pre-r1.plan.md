# Action Plan — `/mochiko:setup add a principle covering how long we keep customer data, and tighten the pre-release checks`

*(Plan-only. No agent spawned, no file written, no user-input wait executed — gates below are described, not taken.)*

## Phase 0 — Load the schema (done in this planning pass)

- **Reads:** `plugins/mochiko/schemas/setup.yaml` (raw, full), `plugins/mochiko/schemas/common.yaml` (raw, full).
- **Check:** count of `kind: fail` nodes in `setup.sec.fail-conditions` = 6 (`pre-ratification-authoring`, `unclosed-trace`, `author-graded`, `floor-category-uncovered`, `no-acceptance`, `no-feature-map`). Matches the `.md`'s hard-coded Not-done count — no halt needed.
- **Writes:** none.

## Phase 1 — Workspace read & mode determination

- **Reads (already performed):** `CLAUDE.md`, `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, `.claude/rules/mochiko/money-handling.md`, `.claude/rules/mochiko/output-style.md`, `FEATURES.md`, `.mochiko/product/architecture/`, `.mochiko/features/FEAT-001`.
- **Findings surfaced to the user (not decided unilaterally):**
  1. A ratified surface set already exists (region v1.2.0, GI-001…GI-007) → propose **mode: amend**. Unambiguous from the workspace, so this is a stated proposal, not a blocking reservation gate (`setup.user-mode-ruling` only reserves the ruling *when ambiguous*) — the user can still correct it.
  2. `.mochiko/memory/constitution.md` (v0.9) is present and superseded — flag for deletion in one line, per `setup.constitution-superseded`, noting its two still-unmigrated articles (two-approvals-on-money-PRs, Friday-release-only-if-someone's-around) so nothing is silently lost.
  3. The current fact profile (GI-001) already declares jurisdictions "EU and UK" plus a data class of customer contact details — this matches the `gdpr` trigger row in `COMPLIANCE-MODULES.md` ("Personal data of EU/UK residents" → `gdpr`, legal-mandate, mechanical, unwaivable) — yet the ledger shows **Modules: none**. This looks like the exact "temporal backstop" case the compliance-modules fail-safe describes: a fact that should have mechanically triggered a module, apparently missed. Flagged as a candidate topic for Phase 2, not silently fixed.
  4. Reading the current ledger against the four Essential Floor categories: **FLOOR-SEC, FLOOR-ERR, FLOOR-OBS carry neither a principle nor a recorded waiver** (only GI-003/004/005/007 exist, and the one recorded waiver is load/performance testing). Left uncovered at close, this trips `setup.fail.floor-category-uncovered` regardless of today's two requested changes. Flagged as a second candidate topic — explicitly not folded into scope without the user's say-so.
- **Writes:** none yet.

## Phase 2 — Inline interrogation (lead-conducted, `mochiko:analysis-iterative`, single seat)

Scoped to the amend delta per `INTERROGATION-AGENDA.md`'s amend rule ("a micro-session scoped to the delta … the relevant agenda slice"), plus the two Phase 1 findings offered once:

- **Dim 2 (fact profile):** re-confirm data classes/jurisdiction; put the GDPR trigger match on the table with consequence-stated confirmation ("EU/UK personal data is on record — the `gdpr` module attaches mechanically unless this reading is wrong; if it attaches, it is legal-mandate/unwaivable and is the natural home for a retention obligation"). The user's answer decides whether the new principle mints plain or lands inside `gdpr`.
- **Dim 4 (risk surface):** what data loss / compliance exposure the retention policy is protecting against.
- **Dim 8 (deployment & release reality — always interrogated):** environments, cadence, what currently blocks a release, rollback expectations; offer the `release-gates` module **once** (never ruled on in the existing synthesis) — this is the literal "beyond per-merge quality gates" pre-release layer the user's phrase points at. Also re-open GI-007's existing 70%-coverage single threshold against the Essential-Floor `high`-row default (80% warn / 60% block) and ask whether to align, keep custom, or set a new number.
- **Dim 9 (values / non-negotiables):** the exact retention duration/policy in enforceable terms ("what should CI or review block?"), and the exact pre-release blocking criteria the user wants added.
- **Dim 10 (deliberate exclusions):** confirm what the retention principle does *not* cover; record the Phase-1 floor-gap finding here explicitly if the user chooses to defer it, so it's a recorded exclusion, never a silent gap.
- **Floor-gap routing gate (modeled on `setup.coverage-survivor-routing`):** for the FLOOR-SEC/ERR/OBS gap — the user is asked to rule one of: fold into this amend's scope now, rule inline with a quick recorded waiver, or defer. If deferred, the plan notes (openly) that `setup.fail.floor-category-uncovered` remains a live risk at close and the run may finish with that gap explicitly recorded rather than silently resolved.
- **Output:** the amended synthesis, drafted in place at `.mochiko/memory/governance-intent.md` — new GI element(s) for data retention (either plain-minted or under the `gdpr` module), the `release-gates` module content (or a recorded decline), the revised/kept GI-007 threshold, and whatever the floor-gap ruling produced. This is *not* a "surface" under `setup.surface-set`, so drafting it here does not trip `setup.fail.pre-ratification-authoring`.
- **Writes (would-be):** `.mochiko/memory/governance-intent.md` (amended in place).

## Phase 3 — Pre-ratification cold stress test

- **Seat:** an independent seat (or pair) running `mochiko:review-governance-intent`, dispatched per the two-message blind protocol (`setup.blind-map-dispatch`): first message carries only the setup topic / project identity and goal (no synthesis path), so its Phase-0 angle map is built blind; the synthesis path is sent only after the map returns.
- **Reads:** the amended `.mochiko/memory/governance-intent.md` (once released), the project's existing surfaces for context.
- **Output:** severity-ranked findings plus a recommended verdict, defaulting to FAIL posture.
- **Gate — `setup.coverage-survivor-routing`:** any coverage finding that survives this stress test questions the amend's *scope*, not a card. Each such gap is presented to the user as a candidate topic with three branches: **explore now** (re-enter `mochiko:analysis-iterative` on that angle, landing new GI-XXX elements), **rule inline**, or **defer**. A gap that overlaps an already-worked agenda dimension (e.g. a Dim-8 detail the stress test pokes at) instead follows the ordinary interrogation follow-up path back into Phase 2.
- **Writes:** none (review-only).

## Phase 4 — Ratification gate

- **Gate (`setup.gate-synthesis-ratification`, floor):** the assembled, stress-tested synthesis is presented to the user for ratification — this is reserved to the user, never taken by any seat.
  - **What's confirmed:** the new data-retention GI element and its enforceable wording; the GDPR-module ruling (attach vs. stays plain-minted); the release-gates module content (environments/cadence/gate table/rollback) or its recorded decline; the GI-007 coverage-threshold decision; the floor-gap disposition from Phase 2; any surviving stress-test findings and how they were routed.
  - **Branch — user ratifies as presented:** proceed to Phase 5 (authoring).
  - **Branch — user edits inline:** the edit is folded into the synthesis, and if the edit is material, it recycles through a bounded delta-pass of the stress test (`mochiko:review-governance-intent`'s verify-pass mode) before ratification is re-offered.
  - **Branch — user rejects:** return to Phase 2 with the stated objection; no surface is authored.
  - No surface is authored on any branch until this gate closes — the direct guard against `setup.fail.pre-ratification-authoring`.
- **Writes:** none (gate only); the ratified synthesis stays at `.mochiko/memory/governance-intent.md`.

## Phase 5 — Author the surface set

- **Seat:** a producer seat running `mochiko:authoring-constitution`, in **amend** branch — regenerates only what sits between the CLAUDE.md governance markers and the setup-owned rules files/ledger, preserving untouched principles verbatim (GI-003…GI-006 keep their GI-IDs and wording unless the user separately touched them) and the two protected carve-outs (`mochiko:domain-registry`, `mochiko:output-style`) untouched.
- **Plan-approval sub-gate (`setup.plan-approval-producers`):** the producer plans its routing first (which surface each new/changed GI element lands on) and works only on a lead-approved plan.
- **Concrete routing for this amend:**
  - New retention principle → CLAUDE.md region index line + ledger Three-Part record (GI-00X); if it landed under `gdpr`, formulated at the legal-mandate stratum (unwaivable) per `COMPLIANCE-MODULES.md`.
  - `release-gates` module (if attached) → one CLAUDE.md region summary line + pointer, full gate table/rollback detail in `governance-ledger.md`.
  - Revised GI-007 → updated region line + ledger record, same GI-ID (a redefinition, not a new element).
  - Floor-gap disposition → either new FLOOR-SEC/ERR/OBS principles + rules files, or explicit waiver rows in the ledger's Waivers table — whichever the Phase 2 ruling chose.
- **Deletion:** `.mochiko/memory/constitution.md` removed on sight (flagged in Phase 1), one-line note in the report.
- **Semver:** per the ledger's amendment policy — MAJOR if any module attach/detach occurred (gdpr and/or release-gates), otherwise MINOR (new principle + threshold change), consistent with the project's own 1.0.0→1.2.0 precedent for a coverage-threshold change.
- **Writes (would-be):**
  - `CLAUDE.md` (governance region only, regenerated between markers)
  - `.claude/rules/mochiko/*.md` (any new scope-bound files the routing calls for, e.g. a security/error/observability rules file if the floor gap is closed here; existing files preserved except where their content changed)
  - `.mochiko/memory/governance-ledger.md` (regenerated whole, semver bumped, waivers/exceptions/amendment log updated)
  - `.mochiko/product/architecture/spine.md` — write-if-absent `Scope:` line only if somehow still missing (unconditional store-scaffold obligation, `setup.store-scaffold-unconditional` / `setup.architecture-scope-handoff`); given the store already exists here, this is a no-op check, not a rewrite.
  - The trace summary (manifest: every GI element → primary home + companions).
  - `.mochiko/memory/constitution.md` — deleted.

## Phase 6 — Independent grading

- **Seat:** a different agent than Phase 5's author, running `mochiko:validation-constitution` against the actual authored files (never the author's report) — default FAIL until proven otherwise (`setup.author-grader-default-fail`).
- **Reads:** the regenerated `CLAUDE.md` region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, the trace summary.
- **Checks:** trace closure (every ratified GI element realized or flagged), the Essential-Floor account-for-all-four-categories requirement, module-content routing sanity (release-gates checklist fragment, if attached), RFC 2119 usage, no unsanctioned selection beyond the ratified synthesis.
- **Output:** PASS/FAIL with a concrete fix list; a FAIL loops back into Phase 5 for revision, re-graded by the same independent seat.
- **Writes:** none (review-only) unless the loop authors a revision, which stays inside Phase 5's write set.

## Phase 7 — Acceptance gate

- **Gate (`setup.gate-final-acceptance`, floor):** the finished surface set is presented to the user for final acceptance, **flagged proposal by flagged proposal** — any point Phase 5 or Phase 6 flagged (e.g. wording that resisted enforceable formulation) is ruled on individually, not swept in.
  - **Branch — accepted:** proceed to Phase 8 (close).
  - **Branch — a flagged proposal rejected / edit requested:** the specific item returns to Phase 5 (or, if it changes selection rather than formulation, back to Phase 2), re-graded before re-offering acceptance.
- **Writes:** none (gate only).

## Phase 8 — Close

- **Checks against the fixed goal / Not-done set:**
  - Feature map at close: this is an amend run, so `setup.map-never-overwrite` applies and no map write is made; since `FEATURES.md` and `.mochiko/features/FEAT-001` already exist, the map-presence condition of `setup.fail.no-feature-map` is already satisfied — no action needed unless Phase 1/2 surfaced the map as missing (it isn't).
  - Product baselines / architecture store: already present (`.mochiko/product/architecture/`) — `spine.md`'s `Scope:` line write-if-absent check only.
  - Governance region semver bumped (Phase 5).
  - User acceptance obtained (Phase 7).
  - Trace closed and independently confirmed (Phase 6).
  - Every floor category accounted for — resolved per whatever the Phase 2 floor-gap ruling was (closed now, or explicitly deferred/waived and recorded, never silent).
- **Reports:** user-facing summary per `templates/output-style.md` register, including the trace summary; next-step pointer to `/mochiko:specify` and `/mochiko:architecture` as peer doors (plus `/mochiko:brainstorm` if knowledge-management is active — it already is, per the current region's operations section).
- **Writes:** none beyond what Phase 5 already wrote — this phase is verification and reporting only.

---

**Net new/changed paths across the run:** `.mochiko/memory/governance-intent.md` (amended), `.mochiko/memory/constitution.md` (deleted), `CLAUDE.md` (region regenerated), `.claude/rules/mochiko/*.md` (existing preserved, new file(s) only if the floor gap is closed in this amend), `.mochiko/memory/governance-ledger.md` (regenerated), `.mochiko/product/architecture/spine.md` (Scope-line check only, likely no-op).

**Seats used:** setup lead (inline interrogation, Phase 2) · cold stress-test seat (Phase 3, `mochiko:review-governance-intent`) · authoring producer (Phase 5, `mochiko:authoring-constitution`) · independent validator (Phase 6, `mochiko:validation-constitution`). Once more than one seat is live (Phase 3 onward), `mochiko:patterns-transport-floor` governs composition/messaging for the rest of the run.