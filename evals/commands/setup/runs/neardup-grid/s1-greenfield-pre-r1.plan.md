## Action Plan — `/mochiko:setup` (plan-only, not executed)

**Schema load (completed as the first action).** Read `plugins/mochiko/schemas/setup.yaml`, `plugins/mochiko/schemas/common.yaml`, and `plugins/mochiko/schemas/command-labels.yaml` raw and in full — the run's binding rules, the shared blocks it extends, and the label vocabulary. Also scanned the workspace: repo root holds only `CLAUDE.md` (informal working notes, no governance region markers), `README.md` (product pitch for "Cadence," an async-standup digest tool — explicitly "nothing built yet, no source, no dependencies"), and a `plugins/` directory. No `.mochiko/` tree, no `FEATURES.md`, no `ARCHITECTURE.md`, no `.claude/rules/mochiko/`, no prior `constitution.md` to supersede.

---

### Phase 1 — Propose and gate the mode
- **Reads:** `CLAUDE.md`, `README.md`, directory listing (already done above).
- **Finding:** no code, no prior `.mochiko` state, no governance region — this reads as a fresh project, not a codebase to reverse-engineer, and not a prior governance set to amend.
- **Proposal:** greenfield.
- **Gate to the user:** confirm the proposed mode (greenfield / brownfield / amend).
  - *Ruling: greenfield* → proceed with Phases 2–12 as written below.
  - *Ruling: brownfield* (e.g. code exists elsewhere not visible here, or unwritten conventions are considered binding) → branch into a codebase-analysis pass (`mochiko:analysis-codebase`) writing `.mochiko/memory/codebase-analysis.md`, add the brownfield-only user gates (conflict rulings, entry-by-entry feature-map confirmation), and replace the greenfield empty-map scaffold in Phase 9 with a reconstructed, user-confirmed map.
  - *Ruling: amend* → since no `.mochiko/memory/governance-intent.md` exists yet, this ruling has no durable baseline to amend against; the gap would be surfaced back to the user rather than assumed, before any further phase proceeds.
- Remaining phases assume the greenfield ruling, since it matches the observed evidence.

### Phase 2 — Inline interrogation (run by the lead, not a spawned seat)
- **Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, its `catalog/` deck, and `DOMAIN-DEPENDENCIES.md`.
- **Skill:** `mochiko:analysis-iterative`, worked adaptively one question at a time over the agenda's dimensions, grounded in Cadence's specifics — small part-time two-engineer team, six-person pilot, non-technical audience, plain-language digest copy already noted in `CLAUDE.md`.
- Then the catalog deck, card by card, recommend-then-arbitrate: the lead proposes a stance per card (e.g. testing rigor sized for a two-person team, plain-language copy conventions, optional modules like knowledge-management) and the user rules it.
- **Gate, repeated per card/module/waiver:** the user's ruling is required before advancing.
  - *Accept as recommended* → record as ruled.
  - *Modify* → record the modified stance.
  - *Reject/defer* → record as an explicit open item or waiver, never silently dropped.
- No files are written in this phase — it is conversational capture toward the synthesis.

### Phase 3 — Author the synthesis (unratified)
- **Write:** `.mochiko/memory/governance-intent.md`, rendered from the governance-intent template (or `plugins/mochiko/schemas/governance-intent.yaml` read raw if the CLI template is unavailable), capturing every ruled card/module from Phase 2 under `GI-XXX` IDs.
- Nothing downstream may be authored from this file yet — it is a draft pending ratification.

### Phase 4 — Pre-ratification stress test (cold seat)
- **Seat:** an independent participant — e.g. `mochiko:devils-advocate` or a fresh instance running `mochiko:review-governance-intent` — never the interrogation/synthesis author.
- **Dispatch, two messages:**
  1. Setup topic + Cadence's project identity/goal only — no path to `governance-intent.md`. The seat builds a blind angle map of what a governance set for this kind of project should cover.
  2. After the angle map returns, send the path to `.mochiko/memory/governance-intent.md`; the seat performs its cold read/stress test against the actual draft.
- Introducing this second seat activates multi-seat messaging discipline for its duration (single-writer on shared files, fan-in confirmation before the lead acts on its findings).
- **Alternative branch:** the user may explicitly waive the cold seat; recorded as a waiver, with the lead's own review substituting.

### Phase 5 — Coverage-gap routing (only if Phase 4 surfaces gaps)
- **Gate per surviving finding**, presented as a candidate topic:
  - *Explore now* → re-enter Phase 2's interrogation on that angle, append new `GI-XXX` cards, re-run the stress test on the delta.
  - *Rule inline* → user rules it directly, recorded in the synthesis.
  - *Defer* → noted as an open item, not blocking ratification.
  - *Overlaps an existing agenda dimension* → routed back through ordinary Phase 2 follow-up instead.

### Phase 6 — Ratification gate
- **Gate:** the user ratifies the (possibly gap-amended) synthesis.
  - *Ratified* → proceed to Phase 7.
  - *Not ratified* → loop back to Phase 2/3 for revision; nothing is authored downstream in the meantime.

### Phase 7 — Author the governance surface set
- **Precondition:** synthesis ratified.
- **Seat:** a producer such as `mochiko:tech-lead`, running `mochiko:authoring-constitution`, on a plan the lead approves before authoring begins.
- **Writes:**
  - `CLAUDE.md` — only the marked region between `<!-- mochiko:governance:begin -->` / `end` is inserted/regenerated; the existing "Cadence — working notes" content stays untouched.
  - `.claude/rules/mochiko/*.md` — paths-scoped rule files reflecting the ratified `GI-XXX` cards.
  - If a knowledge-management module was adopted in Phase 2: scaffold from `templates/constitution-modules/knowledge-management.md` plus a project-pinned copy at `.mochiko/memory/knowledge-management.md`.
  - `.mochiko/memory/governance-ledger.md` — the durable ledger.
  - A trace summary tying every authored rule back to its ratified `GI-XXX` card.
- Any existing marked carve-outs (domain-registry, output-style) would be preserved verbatim — none exist yet in this repo, so this is a no-op check. No `constitution.md` exists to supersede (already confirmed absent).

### Phase 8 — Independent grading
- **Seat:** someone who authored none of Phase 7's output — e.g. `mochiko:validator` or a distinct `mochiko:tech-lead` instance — running `mochiko:validation-constitution` against the files themselves (never the author's report), default FAIL.
- **Checks:** checklist coverage, the ratified-intent→authored-rule trace closes, no floor category is left without either a principle or a recorded waiver.
  - *Pass* → proceed to Phase 9.
  - *Fail* → findings return to the Phase 7 author for revision, then re-grade; the run does not advance on a FAIL.

### Phase 9 — Feature map and architecture store scaffolding
- **Write:** empty `FEATURES.md` at repo root (greenfield scaffold — no `.mochiko/features/` entries fabricated, since nothing is delivered yet).
- **Write (unconditional on every mode):** `.mochiko/product/architecture/` if absent, containing:
  - `spine.md` — header-only stub declaring a `Scope:` line. Based on the README ("a web app with a morning email digest"), the natural declaration is `backend-service` + `frontend-web`. This is a mechanical handoff only — no architecture stance is taken here; the actual ruling happens at the first `/mochiko:architecture` visit.
  - `concerns.md` — empty, alongside it.
- **Explicitly deferred:** `data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md` under `.mochiko/product/` — greenfield leaves these to the first `/mochiko:implement` run's design phase, not created now.

### Phase 10 — Version bump and trace-summary assembly
- Bump the semver marker inside the `CLAUDE.md` governance region.
- Assemble the trace summary for acceptance: which `GI-XXX` cards map to which authored rule files, which modules were adopted, what was scaffolded.

### Phase 11 — Final acceptance gate
- **Gate:** present the finished surface set and trace summary as plain blocking text (never a timed prompt), flagged proposal by flagged proposal.
  - *Accepted* → run closes; report success.
  - *Specific proposals flagged/rejected* → return only those items to Phase 7 for revision, re-grade the revised subset (Phase 8), re-present — loop until every flagged item is resolved or explicitly deferred by the user.

### Phase 12 — Close-out verification and report
- Verify none of the six Not-done conditions stand: nothing authored before ratification; the ratified-intent→surface trace closes; the surface set was graded by a non-author; no floor category lacks a principle or recorded waiver; user acceptance was given; a feature map exists at close (here, the empty greenfield scaffold).
- **Report to the user** (plain register, no timed prompts): what was created (exact paths from Phases 7–9), what was deliberately deferred (product baselines, the architecture ruling itself), and next steps — `/mochiko:specify` for Cadence's first feature and `/mochiko:architecture` for the product's architecture baseline, offered as peer doors, plus `/mochiko:brainstorm` if a knowledge-management module was adopted.
- No git commands are run beyond suggesting a commit of the new/changed files — never staged, committed, or pushed by the run itself.