# Plan — `/mochiko:setup` (plan-only, not executed)

**Entry:** no setup request text was supplied — mode gets proposed purely from what the workspace shows.

**Workspace read already performed for this plan:** the directory contains only the `plugins/mochiko/` plugin source tree (skills, agents, schemas, templates). There is no `CLAUDE.md`, no `.mochiko/` directory, no `.claude/rules/mochiko/` directory, and no application/product code outside the plugin itself. That shape points the mode proposal at **greenfield**.

---

## Phase 0 — Load the governing rules (done)
- **Read:** `plugins/mochiko/schemas/setup.yaml` (raw, in full) and `plugins/mochiko/schemas/common.yaml` (raw, in full) — the binding rule set for this run, interpreted live, no build step.
- **Write:** none.
- Confirmed the "not-done" checklist this run is graded against has exactly 6 entries (a surface authored before ratification; an unclosed intent→surface trace; the surface set graded only by its author; a floor category with no principle and no recorded waiver; user acceptance never given; no feature map at close). If that count ever drifts from 6, the run would halt and surface the mismatch before closing — not applicable here, count matches.

## Phase 1 — Propose and confirm the run mode
- **Done:** propose mode = **greenfield**, based on the absence of existing product code, `CLAUDE.md`, and any prior `.mochiko/memory/governance-intent.md`.
- **Gate (user):** the mode ruling is always the user's whenever it's ambiguous.
  - *User confirms greenfield* → proceed as scaffolded below (empty feature map, baselines deferred to first implement run).
  - *User says brownfield* → switch to codebase-driven analysis (invoke the codebase-analysis skill), reconstruct a feature map from delivered code, bootstrap product baselines now, and route any reality-vs-declared-intent conflicts to the user openly.
  - *User says amend* → load the existing `.mochiko/memory/governance-intent.md` as the amend baseline; since none exists yet, this branch would instead surface that no synthesis/map exists and offer to run greenfield-style setup rather than silently scaffolding one.

## Phase 2 — Assemble interrogation inputs
- **Read:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, and `DOMAIN-DEPENDENCIES.md`.
- **Check:** whether a superseded `.mochiko/memory/constitution.md` exists on disk (none does here) — would be deleted on sight with a one-line note if found.
- Not applicable on this branch: brownfield codebase analysis into `.mochiko/memory/codebase-analysis.md` (only fires on the brownfield branch).
- **Skills involved:** `mochiko:analysis-codebase` (brownfield branch only).

## Phase 3 — Inline interrogation (run by the lead directly, no seat spawn)
- **Done:** work the agenda's dimensions adaptively, one question at a time, then the catalog deck card by card, recommend-then-arbitrate.
- **Gate (user, repeated per card/module/waiver):** every card ruling, every compliance-module adoption/rejection, and every floor waiver is the user's call, not a recommendation the lead can finalize alone.
  - *User accepts the recommendation* → it's recorded as ruled.
  - *User overrides or amends* → the override is recorded instead.
  - *User defers a card* → tracked as open, revisited before ratification.
- **Skills involved:** `mochiko:analysis-iterative`.
- **Write:** none yet — this stage builds the synthesis content in-session.

## Phase 4 — Author the synthesis
- **Write:** `.mochiko/memory/governance-intent.md`, GI-XXX numbered entries, rendered from the governance-intent template (or the shipped schema read raw if the template tool is unavailable).
- Done by the lead directly, not delegated — this is the run's own pen.

## Phase 5 — Pre-ratification stress test (cold seat) — sizing gate
- **Done:** size the review as a single cold reviewer or a pair.
- **Two-message dispatch discipline:** first message to the reviewer carries only the setup topic, project identity, and goal — never the synthesis file path — so its angle map is built blind; only after that map returns does the synthesis path get sent and the cold read begin. If a pair is used, both build their maps independently before either sees the synthesis.
- **Seat/skill involved:** `mochiko:review-governance-intent` (via `mochiko:devils-advocate` or `mochiko:validator`-style independent seat). *(Not actually dispatched in this plan-only run.)*
- **Gate (user, waivable):** the user may explicitly waive this stress test.
  - *User lets it run* → proceed to Phase 6 with whatever coverage findings surface.
  - *User records an explicit waiver* → skip straight to Phase 7 (ratification), with the waiver noted in the trace.

## Phase 6 — Triage stress-test findings
- **Gate (user, per surviving finding):** each finding is presented as a candidate topic questioning the setup's *scope*, not as a nitpick on a single card.
  - *Explore now* → re-enter adaptive discovery on that angle; newly elicited intent is folded into the GI-XXX namespace.
  - *Rule inline* → user decides on the spot without further discovery.
  - *Defer* → logged, left open for a later run.
  - A finding that just overlaps an already-covered agenda dimension instead follows the ordinary interrogation-follow-up path (no separate routing).
- Loops back to Phase 4/5 if the synthesis changes materially.

## Phase 7 — Synthesis ratification (hard gate)
- **Gate (user):** ratification of the synthesis is exclusively the user's, and nothing gets authored before it.
  - *User ratifies* → authoring may begin (Phase 8).
  - *User asks for changes* → loop back into Phase 3/4, re-run the cold stress test if the change is material, re-present.
  - *User declines/stops here* → run halts; only the synthesis (and interrogation notes) exist on disk; no governance surfaces are written; run stays open/incomplete.

## Phase 8 — Author the governance surface set
- **Write (only after Phase 7 ratifies):**
  - The marked governance region in `CLAUDE.md` (between `<!-- mochiko:governance:begin/end -->`), regenerated idempotently; everything outside the markers stays untouched.
  - `.claude/rules/mochiko/*.md` path-scoped rule files.
  - Skill pointers.
  - `.mochiko/memory/governance-ledger.md`.
  - A trace summary tying each ratified GI-XXX item to the surface content it produced.
  - If the knowledge-management module was adopted in Phase 3: scaffold it, including the project-pinned copy at `.mochiko/memory/knowledge-management.md` (never overwritten once it exists).
- Any existing marked carve-outs (a domain-registry block, an output-style switch pair) would be preserved verbatim rather than regenerated — not applicable yet, nothing pre-exists.
- **Producer plan-approval gate (user):** the authoring seat plans its writes first; the lead/user approves the plan before it writes.
  - *Plan approved* → authoring proceeds.
  - *Plan sent back* → revised and re-presented before any write happens.
- **Seat/skill involved:** `mochiko:authoring-constitution` (author seat). *(Not dispatched in this plan-only run.)*
- Fact-finding sub-reads inside this phase that are pure locate/enumerate work would route to a cheap `Explore` subagent (haiku); interpretive reads stay on the session-tier seat.

## Phase 9 — Feature map landing
- **Write (greenfield branch, this case):** scaffold an empty `FEATURES.md` index only — no entries.
- *(Brownfield branch, not taken here: reconstruct delivered capabilities from code into `FEATURES.md` + `.mochiko/features/` entries marked `delivered`/reconstructed-from-code, then gate on the user confirming the map entry by entry before it counts as closed.)*
- *(Amend branch, not taken here: the run writes no map at all — a missing map is surfaced and offered to the user, never silently scaffolded.)*
- Never overwrites existing map content; only fills what's missing.
- **Skill involved:** `mochiko:authoring-feature-map` (brownfield/landing use only).

## Phase 10 — Architecture store scaffold (unconditional, every path)
- **Write:** `.mochiko/product/architecture/` created where missing, with a `spine.md` stub (header + `Scope:` line) and an empty `concerns.md` beside it — creates only what's absent, never overwrites existing store content.
- The `Scope:` line is filled write-if-absent from what surface types (backend-service / frontend-web / mobile / desktop / composed) the interrogation revealed; an existing value is left alone. No architecture stance or ruled content is authored here — it's a handoff to a future `/mochiko:architecture` run.
- On this greenfield branch: baseline files (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`) are **not** bootstrapped now — deferred to the first `/mochiko:implement` design phase.
- *(Brownfield branch, not taken here: those baselines would be bootstrapped now, from delivered code.)*

## Phase 11 — Independent grading of the authored set
- **Read (by a seat that authored none of it):** the drafted `CLAUDE.md` governance region, the `.claude/rules/mochiko/` files, and the governance ledger — read directly, never taken on the author's summary. Default posture is FAIL until actively confirmed.
- **Skill involved:** `mochiko:validation-constitution` (independent grader seat). *(Not dispatched in this plan-only run.)*
- **Branch:** FAIL → loop back to Phase 8 with the fix list, re-author, re-grade, until PASS. Author and grader stay different seats throughout.

## Phase 12 — Final acceptance (hard gate)
- **Done:** present the finished set to the user, flagged proposal by flagged proposal, as plain blocking text (never a timed/auto-expiring prompt).
- **Gate (user):**
  - *Accepts all* → close-out (Phase 13).
  - *Accepts some, flags others* → only the flagged items loop back to Phase 8 for revision, then re-present; accepted items stand.
  - *Rejects broadly* → loop further back, potentially reopening interrogation topics; run does not close without some acceptance.

## Phase 13 — Close-out
- **Write:** bump the governance region's semver.
- **Verify:** the ratified-intent → authored-surfaces trace closes end to end, with Phase 11's independent grade in hand.
- **Never write/delete:** the synthesis (`.mochiko/memory/governance-intent.md`) and the ledger (`.mochiko/memory/governance-ledger.md`) are permanent — never offered for deletion.
- **Report (plain text, not executed):** next steps — `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline, presented as peer doors, neither ahead of the other (plus `/mochiko:brainstorm` if knowledge-management was adopted).
- **Suggest, never run:** a git commit of the new/changed files — no git mutation or push is executed by the run itself.

---

## Cross-cutting constraints applied throughout (not a phase)
- **Seats/model routing:** any seat spawned (stress-test reviewer, authoring seat, grader) is the lead's staffing call (single agent or a team, at its discretion); locate/enumerate fact-finding rides a cheap `Explore`-style dispatch, interpretive/judgment reads stay at session tier.
- **Multi-seat messaging discipline:** if more than one seat is active at once (e.g., a paired stress-test, or overlapping author/grader timing), cross-seat messages and any shared-file writes follow single-writer/no-race discipline for the duration.
- **No self-grading anywhere:** the synthesis author (the lead) is never its own cold reviewer; the surface-set author is never its own grader.
- **All destructive/mutating git actions:** suggested only, never executed by this run.

## Done condition for this invocation
The run would count as complete only when: the synthesis was ratified before any surface was authored (Phase 7 before Phase 8), the intent→surface trace closes and was independently confirmed (Phase 11), the feature map exists per the greenfield/brownfield/amend split (Phase 9), the architecture store's `spine.md` stub and `Scope:` line exist (Phase 10), the governance region's semver was bumped (Phase 13), and the user gave final acceptance flagged proposal by flagged proposal (Phase 12). Any one of the six not-done conditions standing (pre-ratification authoring, unclosed trace, author-only-graded set, an uncovered floor category, missing acceptance, or no feature map at close) fails the run.