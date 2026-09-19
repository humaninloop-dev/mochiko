## Read performed for this plan

- `plugins/mochiko/schemas/setup.yaml` (raw, full) and `plugins/mochiko/schemas/common.yaml` (raw, full) — the run's binding rules. Confirmed the `kind: fail` count is **6**, matching the Not-done set fixed by the command — no halt needed.
- `CLAUDE.md`, `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/constitution.md`, and a glob of `.claude/rules/mochiko/*.md` and `.mochiko/**` to see what the workspace already carries.

**What that showed:** this is *Peartree*, an invoicing app with a governance surface set already ratified at **v1.2.0** (`GI-001`…`GI-007`), a stale pre-region `constitution.md` (v0.9) still sitting on disk, no `FEATURES.md` index at repo root (only `.mochiko/features/FEAT-001/entry.md`), and an existing `.mochiko/product/architecture/spine.md`. The two asks in the entry request — a customer-data-retention principle, and tighter pre-release checks — are amendments to this existing set, not a fresh setup.

---

## Phase 0 — Schema load (done)

Read `setup.yaml` and `common.yaml` raw, in full, as the first action. No writes. No seats.

## Phase 1 — Workspace read and mode proposal

**Does:** proposes the run's mode from what the workspace shows.
**Reads:** `CLAUDE.md`, `.mochiko/memory/governance-intent.md`, `.mochiko/memory/governance-ledger.md`, `.claude/rules/mochiko/*.md`, `.mochiko/memory/constitution.md`.
**Writes:** none.
**Finding surfaced inline, not blocking:** `.mochiko/memory/constitution.md` is superseded (v0.9, pre-dates the CLAUDE.md region) — flagged for deletion in one line, per the standing duty that fires on sight regardless of the two specific asks.

**Gate — mode ruling (reserved to the user, always, even when the answer looks obvious):** I propose **amend** — an existing ratified v1.2.0 set plus two amendment-shaped asks. The user rules the mode.
- *User confirms amend* → continue to Phase 2 as planned below.
- *User insists on brownfield* → pivot: run `mochiko:analysis-codebase` first to (re)build `.mochiko/memory/codebase-analysis.md`, feed it into interrogation as brownfield input, and the feature-map obligation becomes the brownfield reconstruct-and-confirm path instead of amend's surface-or-offer path.
- *User insists on greenfield* → flag the contradiction against the existing ratified artifacts and require an explicit confirmation before treating prior governance as void.

The rest of this plan assumes the amend branch, since that's what the workspace and the request both point to.

## Phase 2 — Inline interrogation (moment: interrogation)

**Does:** I run this myself, inline — never delegated — working the two topics via `mochiko:analysis-iterative`'s adaptive one-question-at-a-time discovery, then the catalog deck card by card, recommend-then-arbitrate.
**Reads:** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`.
**Writes:** none yet (draft only, held in-session).
**Seats:** none besides the lead.

Two threads, scoped to the request:
- **Retention principle.** Elicit which data classes are covered (GI-001's customer contact details, invoice line items, bank account identifiers), retention duration per class, the EU/UK statutory angle (invoices typically must be kept years for tax purposes even as PII erasure rights apply), and the deletion/anonymization mechanism. I propose draft wording; the user rules the concrete numbers and scope.
- **Pre-release checks.** Elicit what "tighten" means against current `GI-007` (test pass + ≥70% coverage on new code): raise the coverage bar, add lint/typecheck/dependency-audit/migration-dry-run gates, distinguish merge-gate from release-gate, define an emergency-bypass owner. I propose candidate gates; the user rules which are adopted and whether this redefines `GI-007` or lands as a new sibling principle.

Every card ruling here is the user's, not mine — I recommend, never finalize.

## Phase 3 — Draft the synthesis

**Does:** compose the updated synthesis reflecting Phase 2's rulings.
**Writes:** a draft of `.mochiko/memory/governance-intent.md` — new GI entry for retention, an updated or new GI entry for the tightened pre-release checks, a proposed semver bump (at least MINOR for a new principle; MAJOR if the pre-release change is judged an incompatible redefinition — that classification is presented to the user, not decided unilaterally), and new Amendment-log rows. This is the pre-ratification artifact itself, not one of the "surfaces" the ratification gate protects, so drafting it now is in bounds.
**Writes to CLAUDE.md / `.claude/rules/mochiko/*` / the ledger:** none yet — those are surfaces, and authoring any of them before ratification is one of the six fail conditions.

## Phase 4 — Pre-ratification stress test (cold seat)

**Seat:** `mochiko:devils-advocate` via `mochiko:review-governance-intent`, dispatched in two messages since it must build its angle map blind:
1. First message: project identity and goal only ("Peartree, invoicing for freelancers, amend run") — never the synthesis path. It returns a blind coverage map (plausible angles here: retention-vs-erasure-rights conflict, backup/replica purge, third-party processor retention, release-gate bypass ownership, staging/prod parity).
2. Second message, after the map returns: hand it `.mochiko/memory/governance-intent.md`'s draft path; it runs the cold read, the six hunt classes, and cross-examination.

**Reads:** the draft synthesis file directly (never a summary of it).
**Writes:** none — findings only, severity-ranked, returned to me.

**Gate — coverage-survivor routing (reserved to the user):** each surviving finding is presented as a candidate topic, not silently folded in or silently dropped.
- *User says explore now* → short re-entry into `mochiko:analysis-iterative` on that angle, producing further GI entries before ratification.
- *User rules it inline* → I fold the direct answer into the draft.
- *User defers* → noted explicitly in the amendment log as an open item; ratification proceeds without it.

*(Alternative: the user may instead record a waiver of the stress-test itself, per the rule that allows a recorded waiver in place of the cold seat — I would surface that as an explicit option, not assume it.)*

## Phase 5 — Ratification gate (moment: ratification)

**Gate (floor, blocking, plain text, never timed):** the final synthesis — every GI delta, the proposed version bump — is presented to the user for ratification. No surface may be authored before this clears.
- *Ratified as presented* → Phase 6 proceeds with exactly this content.
- *User asks for wording/number changes* → loop back into Phase 2/3 on the disputed card only, redraft, re-present. No cost to looping since nothing downstream has been written.
- *User rejects one topic entirely* → drop it from this run's scope, note it, ratify the rest.
- *No ruling* → the run stays open here; nothing is authored.

## Phase 6 — Authoring the surface set (post-ratification only)

**Seat:** `mochiko:tech-lead`, using `mochiko:authoring-constitution` for composition and read scope. Plans first; works only on a plan I approve (grading/fact-finding seats are exempt from this, authoring seats are not).

**Plan-approval gate:** tech-lead submits a short plan (e.g. "add a retention bullet + GI reference to the CLAUDE.md region or a new `.claude/rules/mochiko/data-retention.md` depending on how much detail the ratified wording carries; update the Quality-gates bullets for the tightened checks; bump the ratified stamp; append ledger Three-Part metadata for each new/changed GI; append amendment-log rows in both files; delete the stale `constitution.md` with a one-line note"). I approve or send it back before any write happens.

**Writes (after approval):**
- `CLAUDE.md` — regenerate only the marked governance region idempotently; carve-outs (`mochiko:domain-registry` block, `mochiko:output-style` switch line) preserved verbatim, untouched.
- Possibly a new `.claude/rules/mochiko/data-retention.md` and/or an updated quality-gates home, if the ratified detail is too much for a CLAUDE.md bullet — home choice is `mochiko:authoring-constitution`'s judgment against the ratified content.
- `.mochiko/memory/governance-ledger.md` — version bump matching the region stamp, new Three-Part metadata (Enforcement/Testability/Rationale/Trace) blocks, amendment-log rows dated 2026-08-28.
- `.mochiko/memory/governance-intent.md` — finalized as the durable amend baseline, amendment-log rows appended.
- Delete `.mochiko/memory/constitution.md`, stated in one line.
- Feature map: `FEATURES.md` doesn't exist at repo root today. On an amend run a missing map is **surfaced and offered**, not silently scaffolded — so this phase flags its absence and offers to create it, rather than building it inline as a side effect of the two asks.
- Architecture store: `spine.md` already exists — check its header for a `Scope:` line; write-if-absent only (never overwrite). If absent, declare it from the read surface types (backend-service, per `GI-002`).

**No git mutations:** I suggest a commit message; I never run `git add`/`commit`/`push` myself.

**Transport floor:** once a second seat (the Phase 4 cold seat, then tech-lead) is composed, `mochiko:patterns-transport-floor` governs the messaging — non-waivable from that point on.

## Phase 7 — Independent grading

**Seat:** `mochiko:validator` running `mochiko:validation-constitution` — a seat that authored none of this, reading the actual files on disk (CLAUDE.md region, `.claude/rules/mochiko/*.md`, ledger), never the author's report. Defaults to FAIL.
**Reads:** the written surfaces directly.
**Writes:** none — a PASS/FAIL verdict with a concrete fix list.
- *PASS* → Phase 8.
- *FAIL* → back to tech-lead with the fix list, re-author, re-grade (still by a non-author seat) until PASS, or the user explicitly rules a waiver of a specific gap.

## Phase 8 — Final acceptance (moment: acceptance)

**Gate (floor, blocking, plain text, never timed):** the trace summary — each ratified item mapped to where it landed (e.g. "retention principle → CLAUDE.md bullet + ledger entry", "tightened `GI-007` → CLAUDE.md Quality-gates bullets + ledger entry") — presented flagged proposal by flagged proposal.
- *Accepts all* → close.
- *Accepts some, rejects others* → accepted items stand; rejected ones are reworked or dropped, with the trace summary re-presented for just the delta.
- *No acceptance* → the run does not close; nothing further is claimed done.

## Phase 9 — Close (moment: close)

**Does:** verifies the done condition and sweeps the six fail conditions before reporting.
- Pre-ratification authoring: clear — nothing was written before Phase 5.
- Unclosed trace: clear if Phase 7 confirmed it.
- Author-graded: clear — `mochiko:validator` graded, not `mochiko:tech-lead`.
- Floor-category uncovered: checked as part of Phase 7 grading against the Essential Floor category set.
- No acceptance: clear once Phase 8 passes.
- No feature map: discharged once the Phase 6 surface-and-offer on `FEATURES.md` has been made and answered — not auto-created.

**Reports to user:** what changed (new retention GI-ID and wording, tightened `GI-007` content, version bump, e.g. 1.2.0 → 1.3.0), where each landed, and routes next steps (advisory): `/mochiko:specify` for the next feature and `/mochiko:architecture` for the architecture baseline as peer doors — `/mochiko:brainstorm` only if a knowledge-management module was newly adopted, which nothing here triggers.