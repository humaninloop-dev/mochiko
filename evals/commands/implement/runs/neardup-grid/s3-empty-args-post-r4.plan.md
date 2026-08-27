# Action Plan — `/mochiko:implement` (no argument given)

Governing source already loaded in full for this plan: `plugins/mochiko/schemas/implement.yaml` (all 6 sections, `vars`, `conditions`, `moments`) and `plugins/mochiko/schemas/common.yaml` (4 `extends` targets it supplies). Fail-condition count verified at 15, matching the command file's hard-coded count — no halt needed on that check.

Project-state facts gathered by reading, used to make every phase concrete below:

- `.mochiko/features/FEAT-001/entry.md` — status **delivered**, gates recorded at `gates.md`.
- `.mochiko/features/FEAT-002/entry.md` — status **selected**, two ratified work rows (W1 search-by-query, W2 index freshness), dependency on FEAT-001 which is already delivered (non-blocking). This is the only ready capability — no other `FEAT-XXX` entries exist.
- No `.mochiko/features/FEATURES.md` index file is present on disk (an absent surface to flag, not to silently create).
- `.mochiko/specs/note-search/spec.md` is accepted; its FR-103 requires "a **background index worker**" rebuilding the search index off the request path.
- `.mochiko/product/architecture/spine.md` rules the topology as "Synchronous request/response only; no queues, no background workers" — a direct textual conflict with FR-103. No AX row or NFR target exists for search-index freshness.
- `.mochiko/product/contracts/api.yaml` has no `/notes/search` path.
- `.mochiko/product/constraints-and-decisions.md` C-001 restricts external services, not in-process concurrency — leaves open whether an in-process worker even trips the spine's "no background workers" language.
- No `.mochiko/memory/codebase-analysis.md`, no governance region / `CLAUDE.md`, no `.claude/rules/mochiko/`, no `BACKLOG.md`, no `.gitignore` — and the working directory is **not a git repository**.
- `ARCHITECTURE.md` (the derived root index) exists.

---

## Phase 1 — Capability resolution

**Does:** With the command argument empty, proposes the next ready capability from the map instead of gating directly.
**Reads:** `.mochiko/features/*/entry.md` (both entries), spec files they cite.
**Finding to surface:** no `FEATURES.md` index exists to derive "next ready" from formally — the proposal instead falls back to the single entry file already in a `selected` state with ratified work rows: **FEAT-002 (Note search)**. This absence is reported to the user, not silently patched.
**Writes:** none yet.
**Seats:** none spawned yet — Delivery Manager only.
**Gate (informal, pre-run-open):** presents "FEAT-002 — Note search, work rows W1/W2, dependency FEAT-001 already delivered" and the missing-index note, and asks the user to confirm this is the batch to run.
- *User confirms FEAT-002* → proceed to Phase 2.
- *User names a different capability or a delta card instead* → re-resolve entry against that target; if neither a spec's accepted selection nor a desk-confirmed delta card exists for it, route to `/mochiko:specify` (new capability) or `/mochiko:feature` (feature-keyed delta) instead of opening this run.
- *User declines to proceed* → end here, nothing opened, nothing written.

## Phase 2 — Sufficiency check (entry)

**Does:** Grades, per selected work row (W1, W2), whether spec + architecture store + product baselines already hold enough design to build — run by a seat that authored none of these sources (staffing latitude call: e.g. `mochiko:validator`, never `technical-analyst` or `principal-architect` who would author any gap fix).
**Reads:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md` — per `mochiko:review-sufficiency`'s clause set.
**Concrete verdict expected on this batch (not a hedge — the artifacts read above show it directly):**
- W1 (search by query): contract gap — no `/notes/search` path in `contracts/api.yaml`; no data-model changes implied. Likely **gap**.
- W2 (index freshness): architecture conflict — FR-103's background worker collides with the spine's "no queues, no background workers" ruling, and no NFR row exists for the 2s freshness target. **Gap**, and a **store trip** (a change to ruled architecture content), not just a missing-artifact gap.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, the store-consult result, the store trip flagged for the user, no `quickstart.md` null-path note needed (no external-integration surface here), no `[MODIFY]` amendment against a delivered feature.
**Seats:** the independent grading seat only.
**Gate:** none yet — a disputed clause would default to gap and roll into the run-open batch; nothing here is disputed on this reading.

## Phase 3 — Run-open confirmation (the entry gate)

**Does:** One confirmation, no negotiation, covering: batch = FEAT-002, scope type = selection; both attempt bounds restated at their only redeclaration point — 3 verification attempts per cycle, 2 gap-rework rounds at final validation (schema defaults, redeclarable only here); the sufficiency verdict and its gap routing (W1 contract gap, W2 architecture conflict); the store trip (spine's "no background workers" vs FR-103) surfaced for the user's ruling — ruled now or explicitly deferred to the design checkpoint; the missing-`FEATURES.md`-index absence noted as non-failing; the done condition stated in full.
**Reads:** nothing new — assembles Phase 1/2 outputs.
**Writes:** nothing (the report from Phase 2 already exists; run-open is a confirmation, not a write).
**Seats:** Delivery Manager presents; no new seat.
**Gate — the entry gate itself:**
- *User approves as stated (bounds unchanged, store trip deferred to design checkpoint)* → run opens, proceed to Phase 4.
- *User adjusts an attempt bound* → the adjusted number is carried for the rest of this run (no further redeclaration point exists).
- *User rules the store trip now* (e.g., "in-process worker is fine, amend the spine wording") → that ruling is carried into the design phase's store-delta draft instead of being re-litigated there.
- *User rejects/holds* → run does not open; no code or design artifact is touched.

## Phase 4 — Design phase (fires — gaps were named)

**Does:** Authors exactly the two named gaps, nothing more, each on a plan the Delivery Manager approves first (producer-plan-approval floor). Staffing: `technical-analyst` for the API-contract delta (W1) and the data/NFR framing, `principal-architect` for the architecture-store delta (W2's background-worker element + NFR row), `qa-engineer` for the design-time `**TEST:**` cases feeding the future cards. `technical-analyst`/`principal-architect` never design something outside the two named gaps (plan-minimalism rung discipline applies and is disclosed).
**Reads:** `sufficiency-report.md`, current baselines (`contracts/api.yaml`, `spine.md`, `constraints-and-decisions.md`, `data-model.md`), `spec.md` for FR-101/102/103 and SC-101–103.
**Writes (deltas beside baselines, never in place):**
- `.mochiko/features/FEAT-002/contracts/api.yaml` (delta) — new `GET /notes/search` path, 200/400 responses per FR-101/FR-102.
- `.mochiko/features/FEAT-002/data-model.md` (delta, only if the worker needs tracked state, e.g. a last-indexed cursor — a design call disclosed at the rung it stops at).
- An architecture-store delta (in-flight-class elements only, per the one legal carve to "never in place") adding the search-index worker element and an NFR row for the 2s freshness target, resolving the store trip per however it was ruled at run-open (either amending the "no background workers" line or scoping the new element as an explicit, ruled exception).
- Possibly `.mochiko/features/FEAT-002/baseline-delta.md` if the worker's concurrency model surfaces a new C-XXX/D-XXX/IP-XXX decision (e.g., "in-process goroutine/thread, not a separate process" as a D-row) — same delta grammar, judgment content graded before acceptance, not asserted here.
- The map-entry assertion on `FEAT-002/entry.md`: design-implied dependencies, sharpened extent, architecture-link field filled once the store delta exists.
**Seats:** `technical-analyst`, `principal-architect`, `qa-engineer` (each plan-approved before writing); a non-author review pair before the checkpoint — `mochiko:review-plan-artifacts` (conformance to the two-item gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction, e.g. does the resolved store trip actually let the worker meet the 2s bound alongside C-001) — staffed as independent seats (e.g. `validator` + `tech-lead`), neither the same seat that authored the deltas.
**Gate:** none inside this phase besides internal plan-approval; the phase's own gate is Phase 5.

## Phase 5 — Design checkpoint gate

**Does:** Presents the signed set for the user's sign-off: the contract delta, any data-model delta, the architecture-store delta as a rendered diagram plus its named `AX-XXX` changed-element table (or, absent a render surface, the source plus the changed-element table, recorded as such), any `baseline-delta.md`, and the review pair's verdicts.
**Reads:** the Phase 4 outputs and their reviews.
**Writes:** none new (the checkpoint is a ruling, not a write); the store write itself already happened at Phase 4 as the one legal carve (in-flight-class delta elements), pending this sign-off.
**Gate — user's alone:**
- *User signs as presented* → proceed to Phase 6.
- *User requests changes* → routed back to the authoring seat(s) on a revised, still-approved plan; re-presented at this same checkpoint (not attempt-bound — the attempt economy covers verification grading, not pre-code iteration).
- *User stops here* → explicitly permitted; the run may resume the build later from this signed state.

## Phase 6 — Cycle card authoring

**Does:** A design-class, non-builder seat (e.g. `technical-analyst`, since it is not `staff-engineer`) slices the signed design into cycle cards — foundation cycles before feature cycles. Given W1/W2, a plausible slice: Cycle 1 = walking skeleton (`GET /notes/search` returning results synchronously from existing storage, no freshness worker yet — proves FR-101/FR-102/SC-101/SC-102), Cycle 2 = background index-freshness worker (FR-103/SC-103). `qa-engineer` authors each card's `**TEST:**` real-infrastructure gate within that slicing.
**Reads:** signed design deltas, `spec.md` acceptance criteria, `plugins/mochiko/schemas/tasks.yaml` (schema fallback if the `mochiko-cli template tasks` binary is absent).
**Writes:** `.mochiko/features/FEAT-002/tasks.md` — cycle cards with stories/rationale, dependencies, acceptance-criteria IDs, the `**TEST:**` gate, brownfield exposure per cycle (this touches existing code — FEAT-001's storage layer — so `[EXTEND]`/`[MODIFY]` classification applies). No task lists or file paths inside cards — the builder decomposes those at build time.
**Seats:** the card-authoring seat, `qa-engineer`; the verification seat (independent of both) reviews the cards for quality (`mochiko:review-plan-artifacts`) and buildability before the confirm.
**Gate:** none yet — Phase 7 is the card's gate.

## Phase 7 — Card confirm gate

**Does:** Presents the sliced cards and the verification seat's review for the user's ruling on the slicing, before any card is built.
**Gate — user's alone:**
- *User confirms the slicing* → proceed to Phase 8; `tasks.md` checkboxes become the live progress surface.
- *User asks for a re-slice* (e.g., merge the two cycles, or split Cycle 2 further) → routed back to the card-authoring seat, re-presented here.

## Phase 8 — Build, cycle by cycle

**Does:** `staff-engineer` builds each confirmed card test-first, decomposing it into concrete tasks at build time (disclosed in the cycle report), following `mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration` (Cycle 1/2 both touch FEAT-001's existing storage code), and `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed). Per cycle, an independent verification seat (never `staff-engineer`) grades against real infrastructure per `mochiko:testing-end-user`, plus the `mochiko:review-code-minimalism` lens reading the diff, the cycle report, and the surrounding code.
**Reads (per cycle):** the confirmed card, `cycle-report.md` as it's produced, the diff.
**Writes (per cycle):** the actual application code/tests under whatever source tree this project uses; `.mochiko/features/FEAT-002/cycle-report.md` (or per-cycle variants, per the reports envelope) and a verification report; `tasks.md`'s checkbox flips to `[x]` on a passing cycle.
**Attempt economy in play:** each cycle carries 3 verification attempts (Phase 3's bound); two consecutive rounds with unchanged findings is a no-progress stop regardless of remaining attempts.
**Gate — cycle checkpoint (batched, not per-round):** reserved-to-user questions accumulate and land together here — e.g., if the worker's concurrency approach turns out infeasible under C-001 as understood, that's an `impl.adopt-first-user-call`/`impl.constraint-challenge`-style halt to the user, not a builder decision; an infeasible card escalates as a business-scope decision; only a build-blocking question interrupts mid-cycle.
- *User answers/rules the batch* → build continues or the affected cycle reworks within its remaining attempts.
- *A bound exhausts or no-progress triggers* → halt that cycle, present state; disposition (extend, rework differently, or stop) is the user's, never assumed.
- *A cycle deviates from the signed architecture delta* (adds/removes a box or arrow, e.g. discovers the worker needs a second process) → stops immediately, presented as "build as approved, or amend the delta by the user's ruling" — never designed around silently.

## Phase 9 — Final validation (whole-build verification)

**Does, since this is selection scope:**
- Full quality-gate suite, never severity-triaged.
- Regression sweep: re-runs FEAT-001's durable gate set at `.mochiko/features/FEAT-001/gates.md` (both entries) since this feature's territory touches FEAT-001's storage — a failure here fails the run.
- Cold verification: build + gates from a dependency-cold snapshot of the uncommitted working tree copied to `.claude/worktrees/mochiko-<purpose>/`, using `git ls-files -co --exclude-standard`. **Flag:** the working directory is not currently a git repository, so this mechanism cannot run as specified until the project is git-initialized — this is surfaced to the user as a practical blocker at this phase (or earlier, at run-open, as a trip) rather than silently skipped or faked.
- Gap-finding pass (selection scope → required, not skippable): a fresh `devils-advocate`, dispatched blind in two messages — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines (never code, `tasks.md`, `**TEST:**` cases, or reports); the seat states derived expectations before probing begins.
- Mutation lens: only if governance depth reads `high` (unresolved here — no governance region exists to read a depth from; absence is surfaced, not assumed).
**Reads:** everything accumulated so far plus the built code.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md`; any spec-required-behavior-broken finding is captured with evidence and clause citation (fails validation); beyond-spec findings are advisory, each disposed by the user (fix now / `BACKLOG.md` — currently absent, would need creating / accept as designed).
**Gate:** findings batch to the checkpoint, same reserved-to-user pattern as Phase 8; gap-rework bound (2 rounds, from Phase 3) governs rework rounds here, with a finding localized to one cycle instead charging that cycle's remaining attempts.

## Phase 10 — Landing

**Does (selection scope):** executes the store landing (the signed delta's elements flip built, `FEAT-002` key clears, `As-built:`/`Drift:` fields written as judgment and independently graded, orphan check run, root `ARCHITECTURE.md` regenerated — never hand-edited) plus the map's graduation batch (W1/W2 fold into FEAT-002's extent, status → delivered dated, the `FEATURES.md` index line updates — noting again it must effectively be created here since it didn't exist, which itself is worth a call-out rather than a silent fix, spec-closure row touched). Gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted, since it doesn't yet exist) in the `**TEST:**` grammar via `qa-engineer`.
**Writes:** `.mochiko/product/architecture/spine.md` (graded fold), `ARCHITECTURE.md` (regenerated), `.mochiko/features/FEAT-002/entry.md` (status flip, extent fold), `.mochiko/features/FEATURES.md` (created/updated), `.mochiko/features/FEAT-002/gates.md`, `.mochiko/specs/note-search/spec.md` closure marker.
**Seats:** landing verification seat checks every graded fold (three-way diff: pre-fold baseline + delta vs folded result) before user acceptance.
**Gate:** none standalone — folds are prepared here but per `impl.dm-landing-whole` the landing executes whole at the moment of user acceptance (Phase 11), not before it.

## Phase 11 — Final acceptance gate — closes the run

**Does:** Presents the full evidence set (all reports, the landing package, the verdict against the done condition) as plain blocking text.
**Gate — user's alone, closes the run:**
- *Accept* → landing already staged executes as final; run closes with a done verdict, provided none of the 15 fail-condition rules stand (checked explicitly: sufficiency recorded ✓, design signed ✓, card independence ✓, all cards `[x]` ✓, quality gates passing, real-infra evidence present, no regression, no baseline edited in place, no unresolved deviation, store landing complete with built-vs-signed diff, every fold graded, gap-finding present (selection scope, not skipped), no unresolved spec-required-behavior finding, acceptance given).
- *Amend* → the user specifies the change; it's scoped either as a bounded rework (charging remaining gap-rework/cycle attempts) or, if it's bigger than this run was framed for, routes to `impl.scope-escalation-fail` — stays FAIL unless the user explicitly accepts the larger scope, which effectively reopens design/card phases for the added scope.
- *Reject* → run closes not-done; findings and state stand as the record for a future run.

---

### Notes carried through every phase
- Model tiering: any locate/enumerate/targeted-read fact-finding (e.g. re-scanning the codebase for existing search/index code before Cycle 1) dispatches to a native `Explore` subagent on `haiku`; interpretive reads (grading sufficiency, feasibility, the store-trip judgment) stay on the session tier.
- Transport floor activates the moment a second seat joins (Phase 2 onward) — all cross-seat messaging and shared-write surfaces (the store delta, `tasks.md`, the map entry) ride it for the rest of the run.
- No git mutations are ever run; commits are only suggested — compounded here by the working directory not being a git repo at all, which additionally blocks the cold-verification snapshot mechanism until resolved.