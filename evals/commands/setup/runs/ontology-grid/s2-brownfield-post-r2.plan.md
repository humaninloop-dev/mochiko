# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Workspace read before planning:** no `.mochiko/`, no `CLAUDE.md`, no `.claude/rules/mochiko/`, and no product code beyond the `plugins/mochiko/` plugin tree itself; not a git repository. `$ARGUMENTS` carries no setup request text. This shapes Phase 1's mode proposal below.

**Schema already loaded** (first-class read, done as part of this plan): `plugins/mochiko/schemas/setup.yaml` (all 6 sections) and `plugins/mochiko/schemas/common.yaml`, in full, raw. Confirmed the `kind: fail` count in `setup.sec.fail-conditions` is exactly 6 — in sync with the command's Not-done line, so no halt-and-surface is needed on that check.

---

## Phase 1 — Entry and mode proposal
- **Does:** Since the workspace shows no existing governance surfaces and no product code, propose **greenfield** as the run's mode.
- **Reads:** current directory listing (already sampled), absence of `CLAUDE.md`, `.mochiko/`, `.claude/rules/mochiko/`.
- **Writes:** none.
- **Gate (user-reserved):** the mode is confirmed with the user whenever ambiguous. What's confirmed: "greenfield" as the proposed mode. Branches:
  - User confirms greenfield → proceed with the greenfield path in every later phase.
  - User corrects to brownfield → Phase 2 adds a codebase-analysis pass and Phase 8 switches to feature-map reconstruction instead of an empty scaffold.
  - User corrects to amend → Phase 2 shifts to loading the existing synthesis/surfaces instead of running fresh interrogation from empty, and Phase 8's feature-map obligation becomes "surface and offer" rather than "scaffold."

## Phase 2 — Interrogation inputs
- **Does:** Assemble the inputs the interrogation will run against.
- **Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`. (Greenfield path: no brownfield codebase analysis needed.) *If mode had come back brownfield instead:* would invoke skill `mochiko:analysis-codebase` to produce `.mochiko/memory/codebase-analysis.md` first.
- **Writes:** none yet.
- **Seats/skills:** locate/enumerate-type reads for these reference files are dispatched to a native `Explore` subagent spawned with `model: haiku` (per `mochiko:patterns-model-tiering`); the interrogation content itself stays on the session tier.

## Phase 3 — Interrogation (inline, lead-run)
- **Does:** Run the interrogation myself, inline — never delegated to a seat. First the agenda's dimensions worked adaptively via skill `mochiko:analysis-iterative`, then the catalog deck worked card by card, recommend-then-arbitrate.
- **Reads:** the agenda and catalog loaded in Phase 2; the user's answers as the interrogation proceeds.
- **Writes:** none yet — this phase only accumulates rulings.
- **Gate (user-reserved, repeated per card):** every card ruling, every module ruling, and every waiver in this phase is the user's — not something I resolve unilaterally. Branches per card: user accepts the recommendation → ruling recorded as given; user overrides → ruling recorded as overridden; user defers → card stays open and is revisited before synthesis is drafted.

## Phase 4 — Synthesis authoring
- **Does:** Draft the frozen interrogation synthesis from the accumulated card/module rulings.
- **Reads:** the governance-intent template (via `mochiko-cli template governance-intent`, or `plugins/mochiko/schemas/governance-intent.yaml` read raw as the first-class fallback if the binary is unavailable).
- **Would write:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace) — the durable amend baseline. Not yet authored at plan time.
- **Note:** this synthesis is not itself a "surface" under the fail-condition guard — it precedes ratification and is what gets ratified.

## Phase 5 — Pre-ratification stress test (cold seat)
- **Does:** Spawn an independent, cold reviewer to stress-test the synthesis before it is ratified, using skill `mochiko:review-governance-intent`, via agent `mochiko:devils-advocate` (or `mochiko:validator`-style independence).
- **Two-message blind dispatch:** message 1 carries only the setup topic / project identity / goal — never the synthesis path — so the seat builds its Phase-0 angle map blind to the interrogation's conclusions; only after that map returns does message 2 send the synthesis path (`.mochiko/memory/governance-intent.md`) for the cold read. In a paired stress test, both seats build their maps independently before either sees the synthesis.
- **Reads (by the spawned seat):** the synthesis file itself once released.
- **Writes:** the seat's findings only (no surface writes).
- **Alternate branch:** the user may record an explicit waiver of this stress test instead of spawning it — that satisfies the rule in place of a cold seat.
- **Gate (user-reserved) on any surviving coverage finding:** a finding that survives the stress test questions the setup's *scope*, not a card. Each gap is presented to the user as a candidate topic, with three branches: (a) explore now — re-enter `mochiko:analysis-iterative` on that angle, landing new entries in the GI-XXX namespace; (b) rule inline — user decides on the spot without further exploration; (c) defer — gap noted but not resolved this run. A gap that overlaps an existing agenda dimension instead follows the ordinary interrogation-follow-up path from Phase 3.

## Phase 6 — Ratification gate
- **Does:** Present the (possibly revised) synthesis for ratification. This is the hard floor before any surface may be authored.
- **Gate (user-reserved, floor):** synthesis ratification is the user's alone. What's confirmed: the full synthesis content — mode, every card and module ruling, every waiver — as it stands after Phases 3–5. Branches:
  - User ratifies → proceed to Phase 7 (surface authoring may now begin).
  - User requests changes → loop back to Phase 3/4 (further interrogation or a synthesis revision), then return to this gate.
  - User declines/defers indefinitely → run halts here; no surface is authored (this is exactly the condition `setup.fail.pre-ratification-authoring` exists to prevent — authoring before this gate clears is an automatic FAIL).

## Phase 7 — Surface authoring
- **Does:** Author the governance surface set from the ratified synthesis, via skill `mochiko:authoring-constitution`. The authoring seat plans first and works only on a plan the lead (me) approves before writing (grading/fact-finding seats are exempt from this, this seat is not).
- **Would write:**
  - the marked governance region in `CLAUDE.md` (`<!-- mochiko:governance:begin/end -->`) — only that region; everything outside the markers is user content and is never touched;
  - `.claude/rules/mochiko/*.md`, `paths`-scoped;
  - skill pointers within those files;
  - `.mochiko/memory/governance-ledger.md`;
  - a trace summary linking each authored item back to its ratifying GI-XXX entry.
- **Conditional sub-step:** if a knowledge-management module was adopted during interrogation, scaffold it per `templates/constitution-modules/knowledge-management.md`, including the project-pinned copy at `.mochiko/memory/knowledge-management.md` (never-overwrite floor applies to that copy going forward).
- **Superseded-file check:** confirmed absent already (no `.mochiko/memory/constitution.md` on disk), so nothing to delete here.
- **Carve-outs:** none exist yet in this fresh workspace, so nothing to preserve this run; the obligation (preserve `mochiko:domain-registry` and the `mochiko:output-style` pair verbatim on future runs) still applies structurally going forward.
- **Gate:** an internal plan-approval checkpoint with the user before this seat writes anything (not the ratification gate itself, which already cleared in Phase 6) — described here for completeness; branches are simply approve-as-is or request plan changes before writing proceeds.

## Phase 8 — Feature map landing
- **Does (greenfield path, as proposed in Phase 1):** scaffold an empty `FEATURES.md` index. Never overwrites an existing map.
- *If mode were brownfield instead:* the codebase analysis from Phase 2 would extend into a feature-map reconstruction — delivered capabilities derived from code (routes, UI surfaces, services) — landed as `FEATURES.md` plus `.mochiko/features/` entries marked `delivered` and reconstructed-from-code, via skill `mochiko:authoring-feature-map`, gated by a user confirmation of the reconstructed map **entry by entry** (user-reserved).
- *If mode were amend instead:* a missing map is surfaced and offered to the user, never silently scaffolded.
- **Would write:** `FEATURES.md` (repo root), empty index, greenfield case.

## Phase 9 — Architecture store scaffold (unconditional, every path)
- **Does:** Create `.mochiko/product/architecture/` where it doesn't exist yet, with a `spine.md` stub (header only) and an empty `concerns.md` beside it — creates only what's missing, never overwrites existing store content.
- **Scope handoff:** determine which surface types the product carries (backend-service / frontend-web / mobile / desktop / composed) from what interrogation and the workspace revealed, and write that as the `Scope:` line in the `spine.md` header — write-if-absent only; this is a handoff to `/mochiko:architecture`, not an architecture ruling, and no architecture stance is taken here.
- **Would write:** `.mochiko/product/architecture/spine.md` (stub + `Scope:` line), `.mochiko/product/architecture/concerns.md` (empty).
- **Explicitly not written here:** any ruled architecture content — that's reserved for the first `/mochiko:architecture` visit.
- **Baselines note (greenfield):** `data-model.md` / `contracts/` / `constraints-and-decisions.md` / `quickstart.md` under `.mochiko/product/` are **not** bootstrapped this run — they seed later, at the first `/mochiko:implement` run's design phase. (Only a brownfield run would bootstrap them now, from delivered code.)

## Phase 10 — Independent grading
- **Does:** A non-author seat grades the authored surface set by reading the files themselves — never the authoring seat's report — via skill `mochiko:validation-constitution`, defaulting to FAIL until confirmed otherwise.
- **Reads:** the actual files written in Phases 7–9 (`CLAUDE.md` governance region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, `FEATURES.md`, `.mochiko/product/architecture/spine.md`), plus the ratified synthesis to check trace closure.
- **Checks:** the intent→surface trace closes with nothing dangling; every Essential Floor category carries either a principle or a recorded waiver (none left uncovered); the mode and every card/module ruling and waiver trace back to the user.
- **Writes:** a grading verdict only, no surface edits.

## Phase 11 — Version bump
- **Does:** Bump the governance region's semver as part of the same authoring pass (folded into Phase 7's write, verified again here as a goal condition), reflected in the trace summary.

## Phase 12 — Final acceptance gate
- **Does:** Present the finished surface set to the user with the trace summary in hand.
- **Gate (user-reserved, floor):** final acceptance is the user's, flagged proposal by flagged proposal, as plain blocking text — never a timed prompt. Branches:
  - User accepts every flagged proposal → run proceeds to close (Phase 13).
  - User accepts some, rejects/flags others → rejected items loop back to the relevant earlier phase (a card ruling back to Phase 3, a surface detail back to Phase 7) and this gate re-fires on the revised set.
  - User never gives acceptance → the run cannot close; this is exactly the standing condition `setup.fail.no-acceptance` guards against.

## Phase 13 — Close and report
- **Does:** Verify all six fail-conditions are cleared (pre-ratification authoring did not happen; the trace is closed; the set was graded by a non-author; no floor category is uncovered; acceptance was given; the feature map exists at close). Report to the user in the register from `templates/output-style.md`.
- **Reports/suggests (never executes):** git is never mutated by this run — commits are only suggested, never run or pushed, and this workspace isn't even a git repo yet so that would be flagged explicitly.
- **Next-step suggestion:** `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline — presented as peer doors, neither ahead of the other — plus `/mochiko:brainstorm` if the knowledge-management module was adopted in Phase 7.
- **Would write:** nothing further beyond what Phases 7–9 already produced; this phase is reporting only.

---

### Seats/skills touched across the run
`mochiko:analysis-iterative` (interrogation), `mochiko:review-governance-intent` (cold stress test, via `mochiko:devils-advocate`/`mochiko:validator`-style independence), `mochiko:authoring-constitution` (surface authoring), `mochiko:authoring-feature-map` (brownfield-only branch), `mochiko:validation-constitution` (independent grading), `mochiko:patterns-model-tiering` (routes locate/enumerate reads to a haiku-model `Explore` subagent), `mochiko:patterns-transport-floor` (activates non-waivably the moment more than one seat is composed — here, at minimum the stress-test seat and the grading seat alongside the lead).

### Gates summary (all user-reserved, none run-decided)
1. Mode ruling (Phase 1) — only if ambiguous.
2. Every interrogation card/module ruling and waiver (Phase 3, repeated).
3. Coverage-gap routing after the stress test (Phase 5) — explore / rule inline / defer.
4. Synthesis ratification (Phase 6) — the hard authoring gate.
5. Brownfield-only: conflict rulings and entry-by-entry map confirmation (Phase 1 alt-branch / Phase 8 alt-branch).
6. Final acceptance, flagged proposal by flagged proposal (Phase 12).