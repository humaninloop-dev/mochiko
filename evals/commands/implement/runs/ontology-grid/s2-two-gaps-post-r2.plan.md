# Action Plan — `/mochiko:implement FEAT-002` (plan-only; not executed)

## Grounding read (already performed, informs every phase below)

- `plugins/mochiko/schemas/implement.yaml` + `common.yaml` + `command-labels.yaml` read raw, in full. Confirmed the `kind: fail` set = **15** rules — matches the `.md`'s hard-coded Not-done count, so no halt-and-surface is triggered.
- `FEATURES.md`: FEAT-002 "Note search" is `selected`, entry at `.mochiko/features/FEAT-002/entry.md`.
- `FEAT-002/entry.md`: scope source is **selection scope** (spec's accepted selection, ratified 2026-08-26). Two rows: **W1** (US-101, SC-101/102, search by query) and **W2** (US-102, SC-103, index freshness). Dependency: FEAT-001 (delivered) — satisfied, no dependency block.
- `note-search/spec.md`: FR-103 requires "a **background index worker** rebuilds the index off the request path."
- `architecture/spine.md`: topology states flatly *"Synchronous request/response only; no queues, no background workers."* This directly contradicts FR-103 — a concrete, load-bearing structural conflict.
- Concern catalog has AX-001 (persistence), AX-002 (logging), AX-003 (auth, n/a) — no row targets a search/index-latency NFR, and SC-103 (≤2s freshness) plainly bears NFR load.
- `constraints-and-decisions.md`: D-001/D-002 lock SQLite + stdlib HTTP; no decision names a search mechanism (FTS5 vs. LIKE-scan vs. external) — a named commodity category (search) with no weighed alternative.
- `data-model.md` / `contracts/api.yaml`: no search entity, no `/notes/search` path yet, but the OpenAPI file has an attachable seam (existing `paths:` block).
- No store rows are `open`/`not-now` — AX-003 is `n-a` with a trigger, not a trip. No other in-flight feature exists, so no clause-10 collision.
- **Workspace anomalies found by direct inspection**, not derivable from the schema alone: no `.git` at all; no `CLAUDE.md` / `.claude/rules/mochiko` (no governance region → `depth` unresolved); no `.mochiko/memory/` at all (no `codebase-analysis.md`, no `knowledge-management.md`, no `governance-intent.md`); no `BACKLOG.md`/`ROADMAP.md`; and **no application source tree anywhere**, despite FEAT-001 being marked `delivered (2026-08-22)` with a durable gate set. These are all `impl.absent-surfaces` cases — none auto-resolve, none fail the run, all get surfaced to the user — but two of them (no git, no code) collide with hard floor mechanics (`impl.cold-verification`, the regression sweep) and need an explicit ruling before Phase 7 can run for real.

---

## Phase 1 — Entry gating

**Does:** Confirm `FEAT-002` resolves as a capability ID (not `EPIC-XXX`), confirm it carries selected work rows with ratified scope, confirm no selected row depends on an undelivered row.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md` (dependency check).
**Writes:** none.
**Result:** scope type = **selection**. W1 and W2 both eligible to proceed (FEAT-001 dependency already delivered). No epic-lookup, no delta-card path, no "propose next capability" branch — FEAT-002 was named explicitly and is ready.

## Phase 2 — Sufficiency check

**Does:** Run the ten-clause sufficiency grade over the batch (`mochiko:review-sufficiency`), one seat, per row (W1, W2), against spec/store/baselines only.
**Seat:** an independent grading seat that authored none of `spec.md`, the store, or the baselines — `mochiko:validator` (never designs or builds; exempt from plan-approval as a grading seat). Fenced from code, `tasks.md`, `**TEST:**` cases, cycle reports, and FEAT-002's own run-output directory.
**Reads:** `note-search/spec.md`, `architecture/spine.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `FEAT-002/entry.md`, `FEAT-001/entry.md`.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` (report-format envelope).
**Anticipated verdict (to be confirmed by the actual grading seat, not pre-decided here):**
- Clause 4 (structural trigger) — **gap**: FR-103's background worker has no home in the ruled "no background workers" topology.
- Clause 5 (NFR targets) — **gap**: SC-103's freshness bound has no AX-XXX concern row.
- Clause 6 (commodity exposure) — **gap**: search mechanism named nowhere, no weighed alternatives on record.
- Clause 9 (delivered-feature exposure) — **to grade carefully**: if the worker must hook the create path FEAT-001 delivered, that's a `[MODIFY]` amendment on FEAT-001's entry, auto-gap regardless.
- Clauses 2/3 (contract/data exposure) — likely locatable/no-gap (existing `paths:`/`Note` seams), but formally graded, not assumed.
- No store trips, no in-flight conflicts.
This yields a **gap list**, not a `sufficient` verdict → the design phase will fire.

## Phase 3 — Run-open confirmation (**GATE — the entry gate**)

**Does:** One confirmation to the user, no negotiation. Presented together:
- Batch identity and scope type: FEAT-002 (Note search), selection scope, rows W1+W2.
- Attempt bounds redeclared at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation (schema defaults; either could be redeclared here, otherwise stand).
- The sufficiency verdict and its gap routing (Phase 2's gap list → design phase fires).
- Trips/conflicts: none found (no `open`/`not-now` rows, no in-flight collisions).
- The absent-surface findings from the grounding read, bundled here since they're not auto-resolvable: no governance region (depth unresolved — the mutation-lens rule, gated on `depth: high`, is inapplicable unless the user states a depth), no `.mochiko/memory/` (offer `/mochiko:setup` or proceed greenfield with the warning logged — the run has no evidence either way since FEAT-001's code isn't visible), no git repository at all (blocks `impl.cold-verification`'s literal mechanism later), and no application source tree despite FEAT-001 claiming delivery (the regression sweep in Phase 7 needs FEAT-001's real code to exist somewhere).
- Done condition restated: every cycle card checked, built test-first, verified per-cycle and whole-build against real infrastructure, criteria traced, governance aligned, landing executed whole, final acceptance closes the run.

**Gate — what's confirmed:** the batch/scope/bounds/verdict/absent-surface disposition as one package.
**Branches:**
- *Ruled to proceed as scoped* → continue to Phase 4 (design phase) with the gap list as-is, and a disposition on the absent surfaces (e.g., "proceed greenfield, treat depth as low, and git-init before final validation" or "the real repo lives elsewhere — point me at it").
- *Ruled to widen/narrow scope, or to route the missing-code anomaly elsewhere first* → the run pauses here; if the anomaly means this isn't the right workspace, the correct next step is outside this run (e.g., re-point to the actual repo) — the plan below assumes the user rules "proceed here."
- *Ruled to defer a trip/conflict* → none exist to defer in this scenario, so this branch is moot for FEAT-002 specifically.

## Phase 4 — Design phase (fires: gaps were named)

**Does:** Author exactly the named gaps, nothing more, each on a plan I approve first (design seats are producers, not exempt).
**Seats (my staffing call, per `impl.staffing-latitude`):**
- `principal-architect` (architect_seat) — the store delta: a new in-flight architecture element for the background index worker (a container or an async flow off `api-service`), an updated topology note (no longer purely synchronous), a new/updated AX-XXX concern row carrying the SC-103 NFR target, a C4-container delta diagram + sequence diagram for the index-rebuild flow, per `mochiko:patterns-system-design` and `mochiko:authoring-architecture-store`.
- `technical-analyst` (design_seat) — resolves clause 6 via `mochiko:patterns-adopt-first`: weighs SQLite FTS5 vs. hand-rolled scan vs. external search engine against C-001 (no external services) and D-001 (SQLite); the actual pick is **not** this seat's or my call — `impl.adopt-first-user-call` reserves the commodity-category ruling to the user, so this seat's output is a weighed proposal for the design checkpoint, not a decision. Also authors the `contracts/api.yaml` delta for `GET /notes/search` and any `data-model.md` delta if the chosen mechanism needs a documented search-index entity.
- `qa-engineer` (qa_seat) — authors the design-time acceptance **TEST:** cases the gap closure implies (feeding Phase 6's cards).
**Reads:** `sufficiency-report.md`, `note-search/spec.md`, product baselines, `FEAT-002/entry.md`.
**Writes:** `.mochiko/features/FEAT-002/` deltas beside baselines (`data-model.md`-delta if needed, `contracts/api.yaml`-delta, `constraints-and-decisions.md`-delta carrying the proposed D-XXX), plus the architecture-store delta itself (in-flight-class elements — the one legal in-place-adjacent carve, since it's signed at the checkpoint). `FEAT-002/entry.md` gets the design-implied dependency/extent assertion and the Architecture link filled once the store delta exists.
**Also:** if clause 9 confirms a FEAT-001 touch, this phase writes the `[MODIFY]` marked delta on FEAT-001's entry too.

**Review pair (non-author, before checkpoint):** `mochiko:tech-lead` grades feasibility/buildability/contradiction (`mochiko:review-feasibility`); `mochiko:validator` grades conformance to the named gap list and completeness (`mochiko:review-plan-artifacts`) — both distinct from the authoring seats above.

## Phase 5 — Design checkpoint (**GATE — the user's**)

**Confirms:** the store delta (rendered diagram + the changed-element table: new worker element, updated topology note, new/updated AX-XXX row with the SC-103 target); the adopt-first proposal for the search mechanism (the actual pick is the user's ruling here, not a design-seat default); the `contracts/api.yaml` and any `data-model.md` deltas; any `[MODIFY]` amendment on FEAT-001; the two review verdicts; and (carried over from Phase 3) whatever git/code-location disposition was ruled, now made concrete against what the design actually needs to build against.
**Branches:**
- *Sign* → proceed to Phase 6 (cards), design outputs stand as the deltas cards build against.
- *Amend* (e.g., reject the background-worker shape for a synchronous on-write index, pick a different search technology, adjust the NFR target) → the same design seats revise the scoped deltas only, re-reviewed, re-presented — no new gaps admitted beyond the amendment's own scope.
- *Stop* → the run may pause here entirely; state is preserved, resumable later with no code written (`impl.gate-design-checkpoint` explicitly allows this).

## Phase 6 — Cycle-card authoring

**Does:** Slice the signed design into cycle cards, foundation before feature, per `mochiko:patterns-vertical-tdd`.
**Seat:** a design-class, non-builder seat — `technical-analyst` (who led the design) authors the cards; `qa-engineer` authors each card's closing **TEST:** bundle within that slicing (`impl.seat-card-author-independence`).
**Anticipated slicing (illustrative, not prescriptive — the actual seat decides Simple/Split/Merge):**
1. Foundation cycle — the chosen search-index mechanism wired to `notes-db`, demonstrable minimally.
2. Feature cycle — `GET /notes/search` (US-101, SC-101/SC-102): ranked matches, empty-miss, 400 on missing `q`.
3. Feature cycle — background freshness (US-102, SC-103): create→searchable within 2s, non-blocking create.
4. Conditional cycle — the FEAT-001 `[MODIFY]` amendment, if clause 9 confirmed one, sequenced before the cycles that depend on it.
**Writes:** `.mochiko/specs/note-search/tasks.md` (cycle cards; per `impl.cards-template`/`tasks.yaml`) — no task lists, no file paths, cards cite IDs only.
**Review (independent of the card author):** `mochiko:validator` grades card quality/conformance (`mochiko:review-plan-artifacts`) and buildability, per `impl.card-review-before-confirm`.

## Phase 7 — Card confirm (**GATE — the user's**)

**Confirms:** the cycle sequence and dependency order, each card's stories/acceptance-criteria IDs/TEST gate/brownfield exposure, and the attempt bounds as they'll apply per cycle.
**Branches:**
- *Confirm* → build begins, in the confirmed order.
- *Amend slicing* → card author revises, re-reviewed, re-confirmed before any card is built.
- *Reject* → run halts pending the user's rescoping instruction; no card is built before confirmation (`impl.fail.card-independence` is exactly this failure mode if skipped).

## Phase 8 — Build, per cycle (test-first, foundation first)

For each confirmed card, in order:
- **Builder** (`staff-engineer`) plans decomposition into concrete tasks; plan needs my approval before work starts (producer seat, not exempt).
- Pre-code minimalism ladder run at decomposition (`mochiko:patterns-code-minimalism`), disclosed rung-wise in the cycle report.
- Brownfield handling (`mochiko:brownfield-integration`) on any `[EXTEND]`/`[MODIFY]` touch — reads the whole touched file first, preserves its interface. *(Concretely blocked here if FEAT-001's actual code cannot be located in this workspace — flagged in Phase 3/9, not silently worked around.)*
- Red→green→refactor via `mochiko:executing-tdd-cycle`; `cycle-report.md` discloses decomposition, difficulties, deviations, any `domain_deps_added`.
- If the builder hits undesigned structure mid-cycle: halt that cycle, re-fire the design phase scoped to the discovery (back through a mini Phase 4/5 loop before resuming).
- Any build-time technical decision lands as `baseline-delta.md` at discovery, never in place.
- **Verification** (`qa-engineer`, never the implementer): executes the card's **TEST:** gate against real infrastructure (`mochiko:testing-end-user`), plus the advisory `mochiko:review-code-minimalism` lens on the diff + cycle report + surrounding code.
- Quality gates run the full repository suite — **concretely, this suite doesn't visibly exist in this workspace** (no manifest, no source found); this needs resolving from the Phase 3 disposition before it can literally execute.
- Each grading pass consumes one of the 3 per-cycle attempts; two consecutive unchanged-finding rounds is a no-progress stop presented to the user, not silently retried.
- Non-build-blocking escalations/findings batch to the cycle checkpoint; a build-blocking one interrupts immediately.
- On green: flip the card's checkbox in `tasks.md`.
**Writes per cycle:** code + tests (location depends on the resolved codebase state), `cycle-report.md` and verification report under `.mochiko/features/FEAT-002/`, checkbox flips in `tasks.md`.

## Phase 9 — Final validation (whole-build)

**Does:**
- **Regression sweep**: re-run FEAT-001's durable gates (`.mochiko/features/FEAT-001/gates.md`) plus FEAT-002's own gates over any seam FEAT-001 already delivered. *(Concretely: cannot execute against real infrastructure without FEAT-001's actual delivered code present — this is the sharp edge of the earlier-flagged anomaly; a failure here that traces to "code isn't in this workspace" is an environment problem, not a FEAT-002 regression, and should be reported as such rather than papered over.)*
- **Cold verification**: snapshot the uncommitted working tree (`git ls-files -co --exclude-standard`) into `.claude/worktrees/mochiko-notesearch/`, after confirming the `.claude/worktrees` gitignore entry exists. *(Concretely blocked: no `.git` exists at all. This needs the user's ruling from Phase 3 — e.g., `git init` first — before this step is more than aspirational; `git init` itself is a repo-affecting action I would confirm before taking, even though it's not destructive.)*
- **Gap-finding pass** (fires — selection scope): a **fresh** `devils-advocate` instance, blind, two-message dispatch — first message carries only `spec.md`, `sufficiency-report.md`, design deltas, and the baselines' NFR-XXX rows (never code/tasks/tests/reports); it states derived expectations, then probes the running system with the probe kit. Mutation lens applies only if `depth: high` — currently unresolved (no governance region), so this needs the Phase 3 disposition to know whether it fires.
- Findings split: spec-required-behavior-broken → fails final validation, must close within the 2-round gap-rework bound (or the owning cycle's remaining attempts if localized); beyond-spec findings are advisory, disposed by the user as fix-now / BACKLOG.md (which doesn't exist yet — would be minted) / accept-as-designed.
**Writes:** `final-validation-report.md`, any `gates.md` fold candidates, gap-finding report.

## Phase 10 — Landing (staged; executes only at acceptance, and whole)

Because scope = selection:
- Store landing: fold the signed delta (flip in-flight elements built, clear FEAT-002's key), write graded `As-built:`/`Drift:` on touched AX rows (graded by the landing verification seat, independent of whoever wrote them), run the orphan check, regenerate `ARCHITECTURE.md` (never hand-edited).
- Graded three-way-diff folds for each touched baseline: `contracts/api.yaml`, `data-model.md` (if touched), `constraints-and-decisions.md` (new D-XXX).
- Map graduation: W1/W2 fold into FEAT-002's Extent, vanish from Work rows; status → `delivered (dated)`; `FEATURES.md` index line updates; `note-search` spec reads closed once both rows fold.
- `gates.md` minted for FEAT-002 (currently absent), folding any fix-now/backlog gap findings in the **TEST:** grammar via `qa-engineer`.
- No knowledge-management landing obligation (no `knowledge-management.md` exists).

## Phase 11 — Final acceptance (**GATE — the user's**)

**Confirms:** the done-condition checklist (all cards `[x]`, verified per-cycle and whole, criteria traced, landing staged), the 15 fail-conditions checked clean (or exceptions named), and how the earlier environmental flags (no governance region, no git, no visible FEAT-001 code) actually resolved in practice during the run.
**Branches:**
- *Accept* → Phase 10's staged landing executes for real; run closes, verdict recorded against the done condition; FEAT-002 → `delivered`.
- *Amend* → targeted rework re-enters Phase 8/9 scoped to the amendment, re-validated, re-presented for acceptance.
- *Reject* → no landing executes; W1/W2 return to `selected`; state and reasoning recorded for a future run.

## Phase 12 — Run close

**Does:** State the verdict against the fixed done condition and explicitly re-check the 15 `kind: fail` conditions (none standing, or named exceptions) — `impl.dm-close-verdict`. No further writes beyond what Phase 10 already landed.