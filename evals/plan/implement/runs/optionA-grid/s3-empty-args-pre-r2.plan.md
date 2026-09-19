# Action Plan — `/mochiko:implement` (plan-only, not executed)

## Scenario grounding (read, not written)

Reads already performed to make this plan concrete:
- `plugins/mochiko/schemas/implement.yaml` (raw, full) and `command-labels.yaml` — binding rules, vars substituted (`attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`). Fail-condition count in the schema = 15, matching the command's hard-coded Not-done count — no halt needed on that check.
- `.mochiko/features/FEAT-001/entry.md` — status `delivered`, gates at `gates.md`.
- `.mochiko/features/FEAT-002/entry.md` — status `selected`, work rows W1 (search by query, US-101) and W2 (index freshness, US-102), both ratified 2026-08-26 from the spec's accepted selection; dependency FEAT-001 is `delivered`, so it does not block.
- `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/features/FEAT-001/gates.md`.
- Checked for absent surfaces: no `.claude/rules/mochiko/` (governance region), no `.mochiko/memory/` directory at all (no `codebase-analysis.md`, no `knowledge-management.md`), no `.mochiko/FEATURES.md` index found, no product source tree found in the workspace.

No `$ARGUMENTS` was supplied with this invocation, so the entry step must propose a capability rather than take one as given.

---

## Phase 1 — Entry resolution and scope typing

**Does:** Since no capability/epic ID was given, scan the map for the next ready capability. FEAT-001 is delivered; FEAT-002 is `selected` with ratified rows W1/W2 and its only dependency already delivered — it is the natural candidate to propose. Type the run as **selection scope** (source: the spec's accepted selection), not delta scope (no `/mochiko:feature` delta card is in play) and not an epic (no `EPIC-XXX` involved).
**Reads:** `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`.
**Writes:** none yet.
**Seats/skills:** none spawned yet — this is the Delivery Manager's own read.
**Note:** the absent `.mochiko/FEATURES.md` index is flagged here as an anomaly to carry into the run-open presentation, since selection-scope landing later expects to update that index line.

## Phase 2 — Sufficiency check

**Does:** Grade FEAT-002's two work rows against the spec, the architecture store, and the product baselines, per row, using the sufficiency review procedure. The grading seat must not have authored any of the spec, the store, or the baselines, and will not go on to design or build this batch — so it cannot be `technical-analyst` or `principal-architect` (baseline/store authors) or `staff-engineer` (builder); a natural staffing call is `qa-engineer` or `devils-advocate`.

Concrete findings this grading would surface from the material read:
- **Gap** — `contracts/api.yaml` has no `/notes/search` endpoint; the API contract layer for W1/W2 doesn't exist.
- **Gap** — the data model has no representation of a search index or ranking basis; FR-101's "ranked newest first" and FR-103's freshness behavior aren't modeled.
- **Trip** — the spec's FR-103 requires a background index worker, but the ruled architecture spine states "Synchronous request/response only; no queues, no background workers." This is a direct conflict with already-ruled architecture, not a mere gap — it's reserved to the user's ruling, not something the grader clears alone.
- **Possible commodity-category question** worth flagging for the design phase / build time: full-text search is a commodity capability (SQLite FTS5 is on-shelf) — an adopt-first check will likely apply once design begins.

**Reads:** spec, spine, constraints-and-decisions.md, data-model.md, contracts/api.yaml (all above).
**Writes:** `sufficiency-report.md` in the feature dir (`.mochiko/features/FEAT-002/sufficiency-report.md`) — binding, per-row verdict, gap list, the trip, and the absent-surface notes below.
**Absent surfaces surfaced (not auto-resolved, not run-failing):** missing governance region (`.claude/rules/mochiko/` absent); missing/absent `.mochiko/memory/codebase-analysis.md` on what is functionally a brownfield touch (FEAT-002 extends FEAT-001's persisted store) — offer `/mochiko:setup`, or proceed greenfield with the warning logged. The architecture store itself is present and ruled, so no store-bootstrap offer is needed.
**Seats:** one independent sufficiency-grading seat (e.g. `qa-engineer` or `devils-advocate`), spawned read-only against the sources above.

## Phase 3 — Run-open confirmation (**user gate**)

**Does:** Present one confirmation, no negotiation:
- Batch: FEAT-002 (Note search), selection scope, rows W1 + W2.
- Attempt bounds restated at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation — offer the user the chance to redeclare either now.
- Sufficiency verdict: two design gaps (API contract, data model) that will fire the design phase; one trip (background-worker requirement vs. the ruled synchronous-only architecture) needing the user's ruling; the adopt-first question flagged as likely to recur at build/design time.
- Absent-surface notices: no governance region present; no `codebase-analysis.md`; the `FEATURES.md` index file wasn't found.
- The done condition, stated plainly: every cycle card checked, test-first and independently verified against real infrastructure, criteria traced, governance respected (none present to violate here), the landing executed whole, closing only at final acceptance.

**Gate and branches:**
- **On the architecture trip** — user rules one of: (a) amend the architecture to add a background-worker element (routes the design phase to include a principal-architect-authored store delta), or (b) keep the synchronous-only architecture and have the design phase find a synchronous-compatible way to satisfy freshness (e.g., reindex inline on create, which may itself require renegotiating FR-103's wording — an ambiguity that would then also need the user's sign-off since it touches the spec's stated requirement), or (c) defer the ruling on the record, which then blocks any design work depending on it until it's resolved.
- **On the governance/codebase-analysis absence** — user rules: run `/mochiko:setup` first (this run pauses), or proceed greenfield with the gap logged in the sufficiency report.
- **On attempt bounds** — user either accepts the defaults (3 / 2) or redeclares new values now (this is the only point they can be changed).
- If the user instead declines to open the run at all, the plan stops here — no design, no code.

**Writes:** none beyond the sufficiency report already drafted in Phase 2 (the confirmation itself is a ruling, recorded, not a file write of new content).

## Phase 4 — Design phase (fires because gaps exist)

**Does:** Author exactly the two named gaps, nothing more, each on a plan the Delivery Manager approves first:
- `technical-analyst` (design seat): API contract delta adding `GET /notes/search` (query param `q`, 200/400 responses) to sit beside `contracts/api.yaml`, and a data-model delta covering how search/ranking is represented — run through the plan-minimalism ladder (e.g., a computed index over the existing `Note.text`/`created_at` columns may clear at the "already exists" or "minimum now" rung rather than justifying a new entity).
- `principal-architect` (architect seat): a store delta **only if** the user ruled in Phase 3 to amend the architecture for a background worker — a rendered C4-container delta diagram plus the changed `AX-XXX` rows, or (if no render surface) the source plus a changed-element table.
- `qa-engineer`: authors the `**TEST:**` real-infrastructure cases for the design gaps within its slicing.
- The design phase also asserts the design-implied dependencies and sharpened extent onto FEAT-002's `entry.md` with provenance, and fills the architecture link if a store delta exists.

**Reads:** `sufficiency-report.md`, `spec.md`, current baselines (contracts, data-model, constraints-and-decisions, spine).
**Writes (as deltas beside baselines, never in place):** `.mochiko/features/FEAT-002/contracts/` delta, `.mochiko/features/FEAT-002/data-model.md` delta (appliable before/after form), and — only if the architecture-amend branch was ruled — a store delta at `.mochiko/product/architecture/` per the store's delta grammar, plus the updated `FEAT-002/entry.md` map assertion.
**Review pair (non-author):** one seat grades conformance to the gap list and card-quality readiness (blocking), a second grades feasibility/buildability and contradiction, including the architecture pass if a store delta exists. Neither is `technical-analyst` or `principal-architect`.

**Gate — design checkpoint (user):** the user signs the design and any store delta before any code is written; may also stop here and resume the build later.
- **Accept as-is** → proceed to cycle-card authoring.
- **Amend** → the design seat(s) revise scoped to the amendment; the review pair re-grades; loop back to this gate.
- **Stop** → run pauses; nothing downstream happens until resumed.

## Phase 5 — Cycle-card authoring and card confirm (**user gate**)

**Does:** A design-class seat (not the builder) slices the two work rows into cycle cards, foundation before feature: plausibly Cycle 1 = synchronous search endpoint (W1, FR-101/102, SC-101/102 — a walking skeleton against the existing notes store) and Cycle 2 = index-freshness mechanism (W2, FR-103, SC-103), built on top of Cycle 1 and whatever architecture ruling Phase 3/4 settled. `qa-engineer` authors the `**TEST:**` real-infrastructure gate within each card's slice. Each card carries: stories/rationale, dependencies, acceptance-criteria IDs, the `**TEST:**` gate, and brownfield exposure — both cycles touch the existing `api-service`/`notes-db` from FEAT-001, so they're `[EXTEND]` and pull in the brownfield-integration procedure at build time. No task lists or file paths belong in the cards; that's build-time decomposition.
**Reads:** signed design deltas, spec.md's acceptance criteria, the tasks schema/template.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` (cycle cards).
**Review before confirm:** the verification seat (independent of the card author) reviews for quality/conformance and buildability.

**Gate — card confirm (user):** rules the slicing before any build starts.
- **Approve** → build begins, Cycle 1 first.
- **Request re-slice** → card-authoring seat revises; re-reviewed; back to this gate.
- **Flag a card infeasible** → escalated as a business-level scope decision back to the user rather than silently reworked.

## Phase 6 — Build and per-cycle verification

**Does, per cycle, in order (foundation then feature):**
- `staff-engineer` decomposes the confirmed card into concrete tasks at build time (disclosed in the cycle report), builds test-first on an approved plan, following the brownfield-integration procedure (reads the whole touched file first, preserves the existing interface) and the pre-code minimalism ladder (rungs disclosed).
- The verification seat (never the implementer) runs the card's `**TEST:**` gate against real infrastructure, captures evidence, and applies the code-minimalism lens to the diff, the cycle report, and the surrounding code (advisory only, any severity).
- Quality gates run the full repository suite, never severity-triaged — a failure fails the cycle outright.
- Locate/enumerate reads during this phase are dispatched to a cheap `Explore` subagent (haiku); interpretive reads stay on the main seat.

**Writes per cycle:** `cycle-report.md` (decomposition, difficulties, deviations, domain deps added) and a verification report, both under `.mochiko/features/FEAT-002/`; `tasks.md` checkbox flips as each card completes.

**In-flight branch points:**
- If the builder hits undesigned structure mid-cycle → halt that cycle, re-fire the design phase scoped to the discovery (back through Phase 4's grade and checkpoint), then resume.
- If a cycle's change would add/remove a box or arrow, or move a responsibility, beyond the signed delta → stop and present it: build as approved, or amend the delta by the user's ruling — never proceed silently. This is the most likely real trip here, since Cycle 2 (index freshness) is exactly where a background-worker decision would materialize in code.
- If a commodity-category question arises (e.g., hand-rolled substring search vs. SQLite FTS5) → halts that decision point to the user; the rest of the run proceeds elsewhere.
- Attempt economy: 3 verification attempts per cycle by default; two consecutive rounds with unchanged findings is a no-progress stop — halt the cycle and present state to the user rather than continuing to spend attempts.
- Non-build-blocking questions accumulate and are presented together at the next checkpoint rather than interrupting immediately; only a question the build truly cannot proceed past interrupts mid-cycle.

## Phase 7 — Final validation (whole build)

**Does:** Once all cards are checked: build and run the full quality-gate suite from a dependency-cold snapshot of the uncommitted working tree (copied via `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-<purpose>/`, confirming the ignore entry exists first). Re-run FEAT-001's durable `gates.md` `**TEST:**` cases as a regression sweep, since Cycle 2 touches the same `notes-db` territory. Because this is selection scope, the blind gap-finding pass is required (not skippable): a fresh `devils-advocate` is dispatched with a two-message protocol — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines (never code, `tasks.md`, `**TEST:**` cases, or any report); it states its expected behavior before probing begins. The verification seat (which already holds code sight) runs the mutation lens if this pass is run at high depth — otherwise it must state the skip explicitly rather than omit it silently.
**Writes:** `final-validation-report.md` under `.mochiko/features/FEAT-002/`; the built-vs-signed diff if a store delta exists.
**Findings routing:** anything showing spec-required behavior broken fails final validation outright (cited clause, evidence attached); beyond-spec findings are advisory, and each one's disposition (fix now / book to `BACKLOG.md` / accept as designed) is the user's call, batched at the checkpoint. A disputed finding-kind classification defaults to advisory and also goes to the user.
**Attempt economy:** 2 gap-rework rounds by default at this stage (or charged against a specific cycle's remaining attempts if a finding localizes there); bound exhaustion or an unchanged-findings round halts the run and presents state — disposition is the user's.

## Phase 8 — Acceptance landing (executed whole)

**Does, only once final validation is clean and every reserved-to-user item has been ruled:**
- If a store delta was signed: flip its elements to built, clear their `FEAT-XXX` keys, write graded As-built/Drift fields on the touched rows, run the orphan check, and regenerate the derived `ARCHITECTURE.md`.
- Fold every touched baseline exactly once via a graded three-way diff (pre-fold baseline + delta vs. folded result), checked by a landing verification seat distinct from the design author: `contracts/api.yaml`, `data-model.md`, and `constraints-and-decisions.md` if an adopt-first ruling produced a new D-XXX/IP-XXX row.
- Selection-scope landing: FEAT-002's rows W1/W2 fold into its extent and vanish from "pending"; status flips to `delivered`, dated; the `FEATURES.md` index line updates (this is where the earlier-flagged missing index file would need to be seeded, since a graded fold expects a target to fold into).
- Fold ruled-fix-now/backlog gap findings into `.mochiko/features/FEAT-002/gates.md` (minted since it doesn't yet exist), authored by `qa-engineer` in the `**TEST:**` grammar; anything accepted-as-designed does not fold.
- The landing verification seat checks all of the above folds; since this is a plain selection-scope run (not a lane), no map-delta boundary check applies beyond the ordinary fold check.

**Writes:** the folded `contracts/api.yaml`, `data-model.md`, possibly `constraints-and-decisions.md` and the architecture spine, `FEATURES.md`, `FEAT-002/entry.md` (status → delivered), `FEAT-002/gates.md`.

## Phase 9 — Final acceptance (**user gate**)

**Does:** Present the completed landing plainly (no timed prompt) and ask the user to rule.
- **Accept** → run closes; state the verdict against the done condition (every card checked and verified, landing executed whole) and confirm none of the 15 fail-conditions stand.
- **Amend** → scope the amendment, loop it back to whichever phase it belongs to (a cycle rework, a gap-rework round, or — if it's genuinely new scope — flagged as a scope escalation that itself needs the user's explicit acceptance before it's in-run work), then re-present acceptance.
- **Reject** → the run does not close as done; state is presented as-is, and the disposition (e.g., leave the rows pending, hold the run) is the user's.

## Phase 10 — Close

**Does:** Surface a summary of rounds consumed and seats spawned at each checkpoint across the run, and close with an explicit verdict against the done condition, re-confirming the fail-condition tally (15) still matches the schema. No further writes beyond what Phase 8 already produced; no git mutations are ever run — at most, a commit is suggested to the user.