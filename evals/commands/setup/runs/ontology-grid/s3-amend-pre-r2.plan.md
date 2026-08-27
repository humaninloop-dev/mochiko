# Action Plan — `/mochiko:setup add a principle covering how long we keep customer data, and tighten the pre-release checks`

**Workspace read already performed for this plan** (research reads, no writes): `CLAUDE.md`, `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, `.claude/rules/mochiko/money-handling.md`, `.claude/rules/mochiko/output-style.md`, `FEATURES.md`/`.mochiko/features/FEAT-001`, `.mochiko/product/architecture/spine.md`, plus the setup schema and supporting reference files. Findings that shape the plan below:

- Project is **Peartree**, already governed — CLAUDE.md carries a ratified governance region at **v1.2.0**, a synthesis exists at `.mochiko/memory/governance-intent.md` (GI-001…GI-007), a ledger exists, and a feature map + architecture-store spine already exist from a prior run.
- A stale `.mochiko/memory/constitution.md` (v0.9, April 2026) is sitting on disk — this predates the current surfaces and is dead weight to be flagged and removed.
- The fact profile records EU/UK jurisdictions and customer contact details, but "modules triggered: none" — no GDPR module attached. A **new data-retention principle for EU/UK customer data is likely to reopen that fact**, which is a real tension worth surfacing, not quietly patching.
- "Tighten the pre-release checks" is ambiguous between the existing per-merge quality gates (GI-007: `npm test`, coverage ≥70%) and a not-yet-adopted release-gates module (environments/cadence/rollback, currently absent). Both readings are legitimate; this gets arbitrated with the user rather than assumed.

---

## Phase 1 — Mode determination
**Does:** Proposes the run's mode from workspace evidence rather than asking blind. Because a ratified region, a synthesis, and a ledger already exist, this is unambiguously an **amend** run, not greenfield/brownfield-from-scratch.
**Reads:** the files already surveyed above.
**Writes:** nothing yet.
**Seats:** lead only.
**Gate (user-held):** mode confirmation is normally the user's call when genuinely ambiguous; here the evidence is one-sided, so the lead states the read ("this is an amend on top of v1.2.0") and treats silence/agreement as confirmation rather than blocking on a formal prompt. If the user disagrees (e.g., "no, start over"), the plan would branch to a full greenfield/brownfield re-run instead of amend — everything below assumes amend.

## Phase 2 — Interrogation (inline, adaptive)
**Does:** The lead runs the interrogation itself, working the two requested changes adaptively, one question at a time, then card-by-card recommend-then-arbitrate:
1. **Data-retention principle** — elicits: retention duration (e.g., tied to UK/EU statutory record-keeping for invoices), what triggers deletion/anonymization (account closure vs. time-since-invoice), legal-hold exceptions, and where enforcement would live (a scheduled job path, if one exists or is planned).
2. **Pre-release tightening** — presents the merge-gates-vs-release-gates ambiguity explicitly and recommends a reading (given the wording says "pre-release," the release-gates module — currently unattached — is the natural fit; the alternative is simply raising the existing merge-time coverage/check bar) and lets the user arbitrate.
3. **Surfaces the GDPR tension** — flags that a retention principle for EU/UK personal data sits awkwardly next to "no modules attached," and asks whether the fact profile itself needs reopening (this is presented as a candidate topic, not silently resolved).

**Reads:** `INTERROGATION-AGENDA.md`, the catalog deck (data/testing/deployment dimension cards), `DOMAIN-DEPENDENCIES.md`, `COMPLIANCE-MODULES.md` trigger table — all under `plugins/mochiko/skills/authoring-constitution/references/`. No codebase analysis needed (not brownfield-from-scratch).
**Writes:** nothing yet — this phase only accumulates rulings.
**Seats/skills:** lead, using `mochiko:analysis-iterative` for the adaptive discovery.
**Gate (user-held):** every card ruling, every module ruling, and any waiver decision here is the user's, not the lead's inference. Branches:
- User picks release-gates module → its content (environments, cadence, gate table, rollback) gets elicited concretely in this same phase, using Peartree's real practice, not placeholders.
- User picks merge-gate tightening instead → a concrete new/raised threshold or added check is elicited instead.
- User declines to reopen the GDPR fact → the tension is recorded as a deliberate, confirmed negative (not silently dropped) and the retention principle proceeds under the current (no-module) profile.
- User elects to reopen the fact → it becomes a fact-profile amendment inside this same run, carrying its own module-attach consequence.

## Phase 3 — Draft the synthesis
**Does:** The lead (never a subagent) drafts the updated synthesis reflecting Phase 2's rulings: a new GI element for retention, whatever pre-release change was arbitrated, an amendment-log row, and — if reopened — the fact-profile update.
**Reads:** the current `.mochiko/memory/governance-intent.md` to preserve untouched GI-IDs verbatim.
**Writes:** `.mochiko/memory/governance-intent.md` (new GI-008+ entries, amendment-log row). **Not yet ratified.**
**Seats/skills:** lead only.
**Gate:** none yet — this is a draft, not a ratification.

## Phase 4 — Pre-ratification stress test (cold seat)
**Does:** An independent seat, blind to the draft, stress-tests it before the user ratifies. It is spawned in two messages: first only the setup topic and project identity/goal ("Peartree amend: add customer-data-retention principle, tighten pre-release checks") so it builds its own gap map with no knowledge of what was concluded; only after that map returns does it receive the synthesis path and do the cold read. It would likely surface the GDPR/fact-profile tension independently if Phase 2 didn't already close it out.
**Reads:** (by the cold seat, after its blind map) `.mochiko/memory/governance-intent.md` draft.
**Writes:** nothing — produces findings only.
**Seats/skills:** an independent agent (e.g., `mochiko:devils-advocate`) running `mochiko:review-governance-intent`; never the lead, never the eventual authoring seat.
**Gate (user-held):** any surviving coverage finding is presented as a candidate topic, not auto-applied — the user rules whether to explore it now (looping back into Phase 2), rule on it inline, or defer it. (The user may also waive this stress test outright, recorded explicitly — in which case Phase 4 is skipped and that waiver is logged.)

## Phase 5 — Ratification gate
**Does:** Presents the final draft synthesis (retention principle wording, the arbitrated pre-release change, any fact-profile change, semver implication) for the user to ratify.
**Reads:** nothing new.
**Writes:** nothing until ratified.
**Gate (user-held, plain blocking text, no timeout):** "Here is the updated intent — ratify as drafted?"
- **Ratify** → proceed to Phase 6 (authoring may now begin — authoring before this point would be a hard failure of the run).
- **Request changes** → loop back to Phase 2/3, revise, and — if the change is material — re-run Phase 4 before re-presenting.

## Phase 6 — Authoring
**Does:** An authoring seat plans its surface-routing approach first (which content is universal vs. scope-bound vs. ledger-only) and gets that plan approved before writing. It then regenerates only the governance region and related surfaces from the now-ratified synthesis — never re-touching content outside the markers, never re-litigating already-settled GI-003…GI-007.
**Reads:** ratified `.mochiko/memory/governance-intent.md`, current `.claude/rules/mochiko/*.md` (to preserve the output-style switch line and any carve-outs verbatim), `governance-surfaces.yaml` schema for artifact shape.
**Writes:**
- `CLAUDE.md` — governance region only: bumped ratified stamp/semver, new retention principle line with its GI trace comment, updated quality-gates summary line if that's the arbitrated path, module pointer line if release-gates attached. Everything outside the markers (Notes for contributors, etc.) untouched.
- `.mochiko/memory/governance-ledger.md` — new Three-Part record (enforcement/testability/rationale) for the retention principle, updated/added record for the pre-release change, amendment-log row, version bump.
- Possibly a new or extended `.claude/rules/mochiko/*.md` file (e.g., a retention-scoped rules file) only if the principle is scope-bound to identifiable paths — otherwise it stays a region-only universal line.
- `.mochiko/memory/constitution.md` — **deleted**, with a one-line note that it was superseded.
- No touch to `FEATURES.md`, `.mochiko/features/`, or `.mochiko/product/architecture/` — those already exist from the prior run and this amend doesn't reopen them.
- A trace summary (GI element → home surface → ledger entry) emitted as part of this output.
**Seats/skills:** an authoring seat (e.g., `mochiko:tech-lead`) running `mochiko:authoring-constitution`; sole writer of these shared surfaces for transport-floor purposes.
**Gate:** none directly — grading happens next, before acceptance.

## Phase 7 — Independent grading
**Does:** A validator that authored none of this reads the actual files — never the authoring seat's report — and defaults to FAIL absent full evidence. Checks trace closure (index line → home → ledger, for every GI element including the new one), Three-Part completeness, no placeholder tokens in the tightened checks, preserved carve-outs, correct semver bump, and — if release-gates attached — its own checklist (real environment names, concrete verification, documented rollback with a time expectation).
**Reads:** the authored `CLAUDE.md` region, the ledger, any new/changed rules file.
**Writes:** a findings verdict only, no file changes.
**Seats/skills:** a distinct validator seat (e.g., `mochiko:validator`) running `mochiko:validation-constitution`.
**Gate:** not user-facing directly — a FAIL loops back to Phase 6 for revision and re-grading; only a PASS proceeds.

## Phase 8 — Final acceptance
**Does:** Presents the finished set to the user for acceptance, flagged proposal by flagged proposal (e.g., if the GDPR tension was deferred rather than resolved, that deferral is called out explicitly here, not folded in silently).
**Reads:** nothing new.
**Writes:** nothing until accepted.
**Gate (user-held, plain blocking text, no timeout):** "Accept the updated governance set?"
- **Accept fully** → run closes; report names next steps (`/mochiko:specify`, `/mochiko:architecture` as peer next doors; `/mochiko:brainstorm` only if knowledge-management were adopted, which it isn't here).
- **Reject a specific flagged item** → loop back to the relevant earlier phase (interrogation for a scope change, authoring for a wording fix), re-validate, re-present — only the contested item is re-opened.
- **Accept part, defer part** (e.g., accept retention now, defer the pre-release-gate decision) → the accepted portion closes normally; the deferred portion is recorded as a deliberate non-change this run, not authored speculatively.

## Phase 9 — Close-out
**Does:** Reports what changed (new retention principle, whichever pre-release change was ruled, resulting version number, files touched, validator verdict) in the project's configured register, and suggests — but does not run — a commit.
**Reads:** nothing new.
**Writes:** nothing (git mutations are never performed by this run, only suggested).
**Seats:** lead only.
**Gate:** none.