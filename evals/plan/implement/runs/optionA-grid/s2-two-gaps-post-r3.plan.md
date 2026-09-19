# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

**Resolved before Phase 1:** `FEAT-002` is a plain capability ID (`.mochiko/epics/` doesn't exist, so this isn't an epic). Its entry (`.mochiko/features/FEAT-002/entry.md`) already carries two selected work rows — W1 (search by query, US-101) and W2 (index stays fresh, US-102) — ratified via "the spec's accepted selection, 2026-08-26." So this is a **selection-scope** run, not a delta-card run.

---

## Phase 0 — Load the rulebook
Read `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` raw and in full (the command's binding rules and the shared blocks it `extends`), and `plugins/mochiko/schemas/command-labels.yaml` for label meaning. Substitute `${var}` placeholders from `implement.yaml`'s own `vars:` block (attempt bound = 3 per cycle, gap-rework bound = 2 rounds, seat defaults: staff-engineer builds, technical-analyst/principal-architect/qa-engineer design, devils-advocate gap-finds). No writes; this is the interpretive frame for everything after it.

## Phase 1 — Entry & scope resolution
Read: `.mochiko/features/FEAT-002/entry.md`, `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`.
Confirm: FEAT-002's two rows carry ratified scope; its declared dependency, FEAT-001, shows `delivered` — so it does not block. No writes.

## Phase 2 — Sufficiency check
Seat: an independent grading seat that authored none of the sources it grades (e.g. the `validator` agent), running the sufficiency-review procedure.
Reads: `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`.
Concrete findings this batch is very likely to surface:
- **W1** (search endpoint): `contracts/api.yaml` has no `/notes/search` path — a contract gap. No recorded technology decision for how query matching is implemented — an open design question.
- **W2** (fresh index): the spec's FR-103 requires "a background index worker," but the architecture spine's Topology explicitly states "Synchronous request/response only; no queues, no background workers." That's a direct conflict between required behavior and the currently ruled architecture — a structural gap, not a simple omission.
- Absent-surface notes (never auto-resolved, never fail the run): no `.mochiko/memory/codebase-analysis.md` (there is no source code anywhere in the tree yet, so greenfield is the honest read); no governance region under `.claude/rules/mochiko/`; no external-integration surface, so `quickstart.md` takes the null path.
Write: `.mochiko/features/FEAT-002/sufficiency-report.md` recording the per-row verdict (gap list for W1 and W2), the absent-surface notes, and the trips to carry into the run-open gate.

## Phase 3 — Run-open confirmation (USER GATE)
One confirmation, stated plainly, covering:
- Batch identity: FEAT-002 "Note search," selection scope, rows W1 + W2.
- Attempt bounds restated at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation — open to the user to change here, nowhere else.
- The sufficiency verdict and its routing: both rows need a design phase — W1 for the missing API contract entry and an open technology choice, W2 for a genuine architecture conflict (spec needs a background worker; the ruled spine currently forbids one).
- Trips/conflicts for the user's ruling: (a) no governance region present — proceed without one, or pause for `/mochiko:setup` first; (b) no codebase-analysis on record — proceed greenfield with that logged, since no code exists yet; (c) this workspace is not a git repository, which the run's cold-verification snapshot step depends on — needs a ruling on whether to `git init` (a local, reversible action) before build starts; (d) a constraint to watch going into design: the single-process constraint (no external services) means the background worker must be in-process, not a separate service — flagged now so the architect designs to it.
- The done condition, stated once: every cycle card checked, built test-first, verified per-cycle and as a whole against real infrastructure; code traces to SC-101/102/103; store and baselines updated only via graded folds; the run closes on the user's accept/amend/reject.

**Gate branches:**
- *Approve as-is* → proceed to Phase 4 with default bounds, governance/codebase-analysis absences accepted as logged, user agrees to `git init` locally.
- *Adjust attempt bounds* → new values carried through the rest of the run.
- *Overrule the background-worker gap* (e.g. "a synchronous rebuild is fine, no worker needed") → this is effectively a spec change; record it and redirect that row's design scope accordingly, or route it out to `/mochiko:feature`/`/mochiko:specify` if it changes accepted acceptance criteria.
- *Decline git init* → cold-verification's snapshot mechanism can't run as specified; halt and surface this as an unresolved dependency rather than silently skip evidence collection later.
- *Defer/reject entirely* → run stops here; nothing downstream executes.

*(Plan continues assuming approval, since the branches for a non-approval are stated above.)*

## Phase 4 — Design phase (fires because both rows named gaps)
Seats, each working only on a plan approved beforehand, each authoring strictly its named gap:
- **technical-analyst**: contract delta adding `GET /notes/search` to `contracts/api.yaml` (query param `q`, 200/400 responses); a data-model delta only if the design needs a persisted index structure; a new technology-decision entry in `constraints-and-decisions.md` for the query/index mechanism (this is a commodity category — e.g. SQLite FTS5 vs. hand-rolled LIKE-scan — so the adopt-first choice itself is called out as a user ruling, not the analyst's or builder's to make).
- **principal-architect**: an architecture-store delta adding an in-process index-worker element to the spine's topology (respecting the single-process constraint), a new concern row for the ≤2s freshness requirement, and a small delta diagram/sequence view since this changes system shape.
- **qa-engineer**: the design-time `**TEST:**` acceptance cases feeding the future cycle cards (SC-101 match/no-match, SC-102 missing-q 400, SC-103 create→search round trip ≤2s).
Writes: files beside their baselines under `.mochiko/features/FEAT-002/` (contract delta, data-model delta if any, in-flight-class store delta), plus the sharpened extent/dependency assertion back onto `entry.md`.

## Phase 5 — Design review (independent, non-author)
A seat that authored none of the above (e.g. tech-lead + validator, or one seat covering both lenses) grades the package two ways: does it cover exactly the named gap list and nothing more, with solid card-quality groundwork (blocking); is it feasible and free of contradiction — in particular, does the in-process worker actually respect the single-process constraint, and does the FTS/LIKE choice sit well against the existing "no framework, two endpoints" minimalism stance. Findings go back to the DM; anything failing loops the responsible design seat, charged against the 2-round gap-rework bound agreed at run-open.

## Phase 6 — Design checkpoint (USER GATE, floor)
Present: the architecture delta (diagram if rendered, else source plus a changed-element table) showing the new index-worker element and its concern row; the API contract delta; the technology-decision candidate (adopt-first ruling surfaced here if not already settled at run-open); the `**TEST:**` cases. The user signs the design and the store delta, or asks for changes, or stops here to resume later.

**Gate branches:**
- *Sign as-is* → proceed to Phase 7.
- *Sign with changes* → design seats revise (charged to gap-rework), re-reviewed, re-presented.
- *Stop here* → run pauses; state is recorded; a later resume re-enters at card authoring.
- *Reject the whole background-worker shape* → this exceeds this run's authority to resolve alone (it contests an already-ruled architecture statement); record it and stop, pointing back to `/mochiko:architecture` or `/mochiko:specify` for that renegotiation.

## Phase 7 — Cycle-card authoring
A design-class, non-builder seat authors `.mochiko/features/FEAT-002/tasks.md` from the tasks template, sliced foundation-first: **Cycle 1** — the search endpoint over existing notes (W1, SC-101/SC-102), no new architecture dependency yet; **Cycle 2** — the background index worker and freshness behavior (W2, SC-103), depending on Cycle 1. Each card carries stories, dependencies, acceptance-criteria IDs, a `**TEST:**` real-infrastructure gate, and its brownfield exposure (`[EXTEND]` on the existing api-service/notes-db) — no task lists or file paths.

## Phase 8 — Card review (independent)
The non-builder verification seat reviews both cards for quality and buildability before they're put to the user.

## Phase 9 — Card confirm (USER GATE)
Present the two-cycle slicing for the user's ruling.

**Gate branches:**
- *Approve* → proceed to build.
- *Re-slice* (merge/split differently) → card seat revises, re-reviewed, re-presented.
- *Defer/reject* → run pauses here.

## Phase 10 — Build: Cycle 1 (search endpoint, W1)
**staff-engineer** decomposes the card into concrete tasks (disclosed in the cycle report), follows brownfield-integration practice extending the existing service, runs the pre-code minimalism check at decomposition (e.g., is a plain SQL query enough before reaching for FTS5), and builds test-first.
Writes: source implementing `GET /notes/search`; `.mochiko/features/FEAT-002/cycle-1-report.md`.
Verification: **qa-engineer** (never the builder) runs the `**TEST:**` gate against real infrastructure (real SQLite file, real HTTP calls) for SC-101/SC-102, the code-minimalism lens (advisory), and the full repository quality-gate suite. Up to 3 graded attempts; two consecutive rounds with unchanged findings halts the cycle for the user rather than retrying indefinitely. Card checkbox flips only on a clean pass.

## Phase 11 — Build: Cycle 2 (background index worker, W2)
Same builder/verifier wiring, building the in-process worker per the signed delta and technology decision. If the builder hits structure the design didn't cover, that cycle stops and the design phase re-fires scoped to just the discovery (back through Phases 4–6 for that slice) before resuming. Any attempt to add a box/arrow/queue beyond the signed delta is presented to the user rather than built silently.
Verification: SC-103's create→search round-trip test against real infrastructure, plus quality gates and the minimalism lens.

## Phase 12 — Final validation (selection-scope run: gap-finding pass required)
- Cold snapshot: `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-note-search/` (depends on the git-init ruling from Phase 3), after confirming the `.claude/worktrees` ignore entry exists.
- Regression sweep: re-run FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md` — its three `**TEST:**` cases), since FEAT-002 reads the same notes store.
- Full quality-gate suite, cold.
- Gap-finding: a fresh `devils-advocate` instance, dispatched blind — first message carries only spec.md, the sufficiency report, design deltas, and baselines; never code, tasks.md, test cases, or reports. It states expectations, then probes the running system for spec-required behavior the builder/QA might have missed (e.g., the &lt;2-char query rejection, the in-flight-rebuild edge case). Mutation lens runs only if this run is being executed at high depth; otherwise it's explicitly logged as skipped, not silently omitted.
- Findings split: spec-required behavior broken fails final validation and must be resolved within the 2-round gap-rework bound; beyond-spec findings go to the user as fix-now / backlog / accept-as-designed.

## Phase 13 — Acceptance landing (USER GATE, executed whole)
Present the final-validation report and evidence for accept / amend / reject.
- **Accept** → one whole landing action: flip the new index-worker element from in-flight to built in the store, write its graded as-built/drift fields, run the orphan check, regenerate `ARCHITECTURE.md`; fold the contract and data-model deltas and the new technology-decision row via graded three-way diffs; fold the map graduation — W1/W2 vanish into FEAT-002's Extent, entry status flips to delivered (dated 2026-08-27), `FEATURES.md`'s FEAT-002 line updates, the spec reads closed; fold any fix-now/backlog gap findings into a new `.mochiko/features/FEAT-002/gates.md`.
- **Amend** → routed back to whichever phase the amendment targets, scoped to just that change.
- **Reject** → run closes unlanded; state recorded as-is.

## Phase 14 — Close
Issue a verdict against the done condition, explicitly checking all 15 not-done clauses in the schema's fail-conditions section (count cross-checked: 15 present, in sync). Report status to the user; suggest commit points but make no git mutations and no pushes.

---

**Cross-cutting, applies throughout:** more than one seat is composed here, so cross-seat messaging and any shared write to the store/baselines follow the transport floor (single-writer discipline on shared surfaces); locate/enumerate-style reads route to a cheap `Explore` dispatch, interpretive reads stay on the session tier; non-build-blocking questions batch at each checkpoint rather than interrupting mid-cycle; advisory minimalism findings never gate a cycle.