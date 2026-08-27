# Action Plan — `/mochiko:setup add a principle covering how long we keep customer data, and tighten the pre-release checks`

*(Plan-only. No agents spawned, no files written below phase 0.)*

## Phase 0 — Load the binding schema (already done to build this plan)
- **Read:** `plugins/mochiko/schemas/setup.yaml` (raw, full), `plugins/mochiko/schemas/common.yaml` (raw, full) — the rule source of truth for this run.
- **Done:** established which rules are floor vs. must vs. advisory, the 6-item Not-done/FAIL set, and the reserved-to-user list.
- No writes, no seats.

## Phase 1 — Read workspace state and propose the mode
- **Read:** `CLAUDE.md`, `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, `.claude/rules/mochiko/money-handling.md`, `.claude/rules/mochiko/output-style.md`, `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/product/architecture/spine.md`.
- **Findings:** a ratified governance set already exists (v1.2.0, "Peartree", production floor, depth high, modules: none) with principles GI-001…GI-007. A stale `.mochiko/memory/constitution.md` (pre-migration, April 2026) is still on disk — superseded, to be deleted on sight. A feature map and architecture spine already exist.
- **Proposal:** mode = **amend** — the request ("add a principle," "tighten the pre-release checks") is amendment-shaped against an already-ratified set, not a from-scratch setup.
- **Gate (mode ruling, reserved to the user):** present the proposed mode and the evidence above.
  - *If the user confirms amend* → continue to Phase 2 on that basis.
  - *If the user rules a different mode* (e.g. wants a full re-interrogation) → restart the run under that mode's path instead; this plan's remaining phases would be re-scoped accordingly.
- No writes yet.

## Phase 2 — Inline interrogation, scoped to the two requested changes
- **Seat:** the lead itself, inline (not delegated) — `setup.interrogation-inline`, via `mochiko:analysis-iterative`, worked adaptively, recommend-then-arbitrate.
- **Read (as needed, fact-finding dispatched to a cheap `Explore` subagent on `model: haiku` per `mochiko:patterns-model-tiering`; interpretive judgment stays on-session):** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, its `catalog/` deck cards for data-retention and quality-gate topics, `DOMAIN-DEPENDENCIES.md`.
- **Card 1 — customer-data retention principle:** recommend a retention duration and scope (invoice line items, contact details, bank identifiers — tying back to the existing GI-003 containment principle and the EU/UK fact profile's storage-limitation exposure), a deletion/expiry mechanism, and whether backups/exports inherit the same clock. Arbitrated live with the user.
- **Card 2 — tightened pre-release checks:** recommend candidates against the current GI-007 baseline (test pass + coverage ≥70%): a higher/added coverage gate, lint/typecheck as a hard gate, a migration dry-run check, a dependency/security audit step, and whether the stale constitution's Article III release-window norm ("tag Fridays only if someone's around") should be formally reinstated or explicitly dropped. Arbitrated live with the user.
- **Gate (every card ruling is the user's — `setup.user-card-rulings`):** each recommendation is accepted, modified, or rejected card by card. A rejected recommendation gets a reframed alternative on the same card; the user may also defer a sub-question, which is logged as open rather than assumed.
- No writes — interrogation is conversational.

## Phase 3 — Draft the updated synthesis
- **Write (draft, not yet ratified):** `.mochiko/memory/governance-intent.md` — mint a new principle entry (e.g. `GI-008 — Customer data retention`) and amend the existing `GI-007 — Quality gates` entry with the tightened checks ruled in Phase 2; append a pending amendment-log row and propose the next version number (MINOR bump, e.g. v1.3.0, consistent with the ledger's semver policy: new principle / gate-strength change).
- **Read:** the current `governance-intent.md` and `governance-ledger.md` amendment-policy section (already reviewed) to keep versioning consistent.
- No CLAUDE.md / rules / ledger writes yet — those are gated on ratification.

## Phase 4 — Pre-ratification stress test (cold seat)
- **Seat:** `mochiko:devils-advocate` (or equivalent independent reviewer), running `mochiko:review-governance-intent`.
- **Dispatch (two-message blind protocol, `setup.blind-map-dispatch`, floor):** message 1 carries only the setup topic and project identity/goal (Peartree, invoicing SaaS, amend adding a retention principle and tightened release gates) — never the synthesis path — so the seat's Phase 0 angle map is built blind; only after that map returns does message 2 send `.mochiko/memory/governance-intent.md` and the cold read begins.
- Because this composes a second seat, `seats: multi` fires and `mochiko:patterns-transport-floor` governs the messaging (non-waivable once triggered).
- **Read (by the seat):** the draft `governance-intent.md`, `governance-ledger.md`, `CLAUDE.md` region, `.claude/rules/mochiko/*.md`.
- **Output:** severity-ranked findings, no writes.
- **Gate (coverage-survivor routing, reserved to the user):** any surviving finding questions the run's scope, not a card.
  - *User rules "explore now"* → re-enter Phase 2 on that specific angle; re-elicited intent lands in the GI-XXX namespace.
  - *User rules inline* → quick ruling folded directly into the draft synthesis, no full re-interrogation.
  - *User rules defer* → logged as an open item in the eventual trace summary, non-blocking.
- Loop back to a revised Phase 3 draft if anything changed, then re-run this stress test only if the change was material.

## Phase 5 — Ratification gate
- **Gate (`setup.gate-synthesis-ratification`, floor — reserved to the user):** the final wording of GI-008 and the amended GI-007, plus the proposed version number, is presented for ratification. Nothing downstream may author onto CLAUDE.md / rules / ledger before this clears (an authored surface here would trip `setup.fail.pre-ratification-authoring`).
  - *Ratifies as-is* → proceed to Phase 6.
  - *Requests wording changes* → back to Phase 3 (and Phase 4 again if the change is material), then re-present.
  - *Rejects one or both items* → the rejected item is simply dropped from this run; only what's ratified proceeds. If both are rejected, the run has nothing left to author and closes early with a "no changes made" report.

## Phase 6 — Author the surface set
- **Seat:** `mochiko:authoring-constitution` (producer — plans first, plan approved by the user before writing, per `setup.plan-approval-producers`).
- **Gate:** the seat's plan (exactly which files change and how) is shown for approval before any write.
- **Write:**
  - `CLAUDE.md` — governance region only (`<!-- mochiko:governance:begin/end -->`): new retention principle line (tagged with its GI-ID comment, `NON-NEGOTIABLE` if ruled so), amended Quality gates section, bumped `Ratified:` stamp. Content outside the markers untouched.
  - `.claude/rules/mochiko/` — a new file (e.g. `data-retention.md`) and/or an extended pre-release-checks file, if the card's "home" ruling from Phase 2 sends detailed enforcement there rather than inline in CLAUDE.md (mirroring how `money-handling.md` is homed today).
  - `.mochiko/memory/governance-ledger.md` — Three-Part metadata block (Enforcement/Testability/Rationale/Trace) for the new GI-ID(s), amendment-log row, bumped `Version:` line, and any waiver entry if the user declined part of a recommendation (with justification and revisit trigger).
  - `.mochiko/memory/governance-intent.md` — finalized version/date.
  - **Delete** `.mochiko/memory/constitution.md` on sight (superseded), stated in one line in the run's report.
- Feature map and architecture store are **not** touched: `FEATURES.md`/`FEAT-001` already exist (amend makes no feature-map write — `setup.map-never-overwrite`), and `spine.md` already carries content, not just a bare `Scope:` stub (`setup.store-ruled-content-never-here`).

## Phase 7 — Independent grade
- **Seat:** `mochiko:validation-constitution` — must not be the authoring seat (`setup.fail.author-graded` fires otherwise). Default FAIL.
- **Read:** the newly-authored CLAUDE.md region, `.claude/rules/mochiko/*.md`, the ledger, and the ratified `governance-intent.md` — checking trace closure (every ratified GI-ID lands on an authored surface and vice versa — `setup.fail.unclosed-trace`) and that every Essential Floor category still carries either a principle or a recorded waiver (`setup.fail.floor-category-uncovered`).
- **Output:** PASS / FAIL with a fix list; no writes.
  - *PASS* → Phase 8.
  - *FAIL* → back to Phase 6 for the named gaps only, then re-grade (loop until PASS).

## Phase 8 — Trace summary and final acceptance gate
- **Assemble:** a trace summary mapping ratified GI-IDs to their authored file locations.
- **Gate (`setup.gate-final-acceptance`, floor — reserved to the user):** presented flagged proposal by flagged proposal.
  - *Accepts all* → Phase 9.
  - *Accepts some, rejects others* → rejected items rolled back/not landed, accepted items stand, report reflects the partial outcome (this is not a FAIL — `setup.fail.no-acceptance` only fires if acceptance was never given at all).
  - *Rejects outright* → back to Phase 5/6 with the specific revision requested.
- No new reads beyond what Phases 6–7 produced; no writes here (any revision writes happen back in Phase 6 on the next loop).

## Phase 9 — Close and report
- **Verify the done condition:** ratified-before-authored ✓ (Phase 5 gated Phase 6), trace closed and independently graded ✓ (Phase 7 PASS), semver bumped in both CLAUDE.md and the ledger ✓, user accepted with trace summary in hand ✓ (Phase 8), feature map present at close ✓ (pre-existing, untouched), architecture store's `spine.md`/`Scope:` present ✓ (pre-existing, untouched).
- **Confirm no Not-done condition stands:** none of the 6 `kind: fail` nodes in `setup.sec.fail-conditions` apply (pre-ratification-authoring, unclosed-trace, author-graded, floor-category-uncovered, no-acceptance, no-feature-map — each checked against the phases above).
- **Report to user:** what changed (new retention principle, tightened quality gates, version bump, ledger updates, deleted stale `constitution.md`), and suggest — never run — a git commit (`setup.no-git-mutations`).
- **Next-step routing (advisory):** point to `/mochiko:specify` for the next feature and `/mochiko:architecture` for the architecture baseline as peer doors (the `/mochiko:brainstorm` leg is omitted since the ledger shows `Modules: none` — knowledge-management wasn't adopted).
- No further writes.