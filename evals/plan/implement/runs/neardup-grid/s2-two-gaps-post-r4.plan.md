# Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Ground truth gathered before planning

- `FEATURES.md` / `.mochiko/features/FEAT-002/entry.md`: capability **FEAT-002 "Note search"**, status `selected`, work rows **W1** (search by query, US-101) and **W2** (index freshness, US-102), both unchecked, selection source = the spec's accepted selection (2026-08-26). Architecture link on the entry is explicitly unfilled. Dependency FEAT-001 is `delivered` — no dependency block.
- `.mochiko/specs/note-search/spec.md`: accepted spec, no UX surface (HTTP API only, no Screens & Flows).
- `.mochiko/product/architecture/spine.md`: ruled topology states *"Synchronous request/response only; no queues, no background workers"* — this directly conflicts with spec FR-103's requirement of a **background index worker**.
- `.mochiko/product/constraints-and-decisions.md`: C-001 (single-process, no external services), D-001 (SQLite), D-002 (stdlib HTTP, no framework). No decision covers full-text search/indexing technology.
- `.mochiko/product/contracts/api.yaml`: only `POST /notes` and `GET /notes/{id}` exist — no `/notes/search` path.
- `.mochiko/product/data-model.md`: only the `Note` entity; nothing describing a search index.
- `.mochiko/features/FEAT-001/gates.md`: 3 durable `**TEST:**` gates from the delivered note-capture run — these must regression-sweep clean.
- No `CLAUDE.md` anywhere (governance region absent) → `governance_region: absent`, so `depth` is unresolved and `impl.briefs-name-rules-files` does not bind.
- No `.mochiko/memory/` directory at all (no `knowledge-management.md`, no `codebase-analysis.md`) → `km_file: absent`.
- No `.claude/` directory, and **this working tree is not a git repository** (`git status` → "not a git repository"). This is not one of the schema's named absent-surface branches but is a real environmental gap: `impl.cold-verification`'s dependency-cold snapshot is defined via `git ls-files -co --exclude-standard`, and `impl.craft-verify-bindings`/code-minimalism review reads a diff. Without a repo neither can execute as written.

Schema fail-count parity check: `plugins/mochiko/schemas/implement.yaml` section `impl.sec.fail-conditions` carries exactly **15** `kind: fail` nodes — matches the `.md`'s hard-coded Not-done count, so no halt is triggered on that audit.

---

## Phase 0 — Load the binding rules

**Does:** Raw, full Read of `plugins/mochiko/schemas/implement.yaml` (already done above) and, because it carries `extends: common.*` stubs, raw full Read of `plugins/mochiko/schemas/common.yaml` in the same first action (already done above). Substitute every `${var}` from `implement.yaml`'s `vars:` block: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`.
**Reads:** the two schema files only.
**Writes:** none.
**Seats/skills:** none yet — this is the run's own instruction-load step.
**Gate:** none.

---

## Phase 1 — Entry gating

**Does:** Resolve `FEAT-002` against `FEATURES.md` → a plain capability ID (not `EPIC-XXX`), scope = **selection**. Confirm the entry carries ratified scope (it does: "selection source: the spec's accepted selection, 2026-08-26") and that W1/W2 are still open (unchecked). Check dependency order: FEAT-002 declares dependency on FEAT-001, which is `delivered` — the batch is unblocked, no wait needed.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md` (to confirm delivered status).
**Writes:** none.
**Seats/skills:** none — lead-level bookkeeping. Locate/enumerate reads of this kind would tier to a `haiku`-model `Explore` subagent per `mochiko:patterns-model-tiering` if the map were larger; here it's a single direct read.
**Gate:** none (routing only fires if entry were missing — not the case here).

---

## Phase 2 — Sufficiency check (binding, per row)

**Does:** Dispatch the sufficiency-grading seat — a seat that authored none of `spec.md`, the architecture store, or the product baselines (so not `technical-analyst`/`principal-architect` if either authored those; a fresh grading seat, e.g. `mochiko:validator` or an unbiased `devils-advocate`-class seat, is spawned to run the ten-clause check from `mochiko:review-sufficiency`) per row, selection scope. This seat is exempt from plan approval as a grading seat.

Applying the ten clauses to W1 and W2 against the ground truth above, the expected verdict (the grading seat's actual call at run time may differ, but the visible evidence points this way):

| Clause | W1 (search) | W2 (freshness) |
|---|---|---|
| 1 Testable criteria | clear GWT/SC oracles — no gap | clear — no gap |
| 2 Contract exposure | `GET /notes/search` absent from `contracts/api.yaml` — **gap** | rides the same endpoint — no separate gap |
| 3 Data exposure | `Note` entity locatable; no gap | possible new index/rebuild-metadata entity — flag for design seat |
| 4 Structural trigger | spine bans background workers; FR-103 requires one — **gap** (store delta needed) | same **gap** |
| 5 NFR targets | no AX concern row carries a freshness/latency target — **gap** | same **gap** |
| 6 Commodity exposure | search/indexing mechanism (e.g. SQLite FTS5 vs. custom) unresolved, no weighed alternatives on record — **gap** | same **gap** |
| 7 Dependency order | resolvable within batch | resolvable |
| 8 UX trace | n/a (no Screens & Flows) | n/a |
| 9 Delivered-feature exposure | if the chosen indexing approach touches the notes-db schema/triggers FEAT-001 delivered (AX-001 persistence), this is a touched surface owned by a delivered row — **likely gap**, would carry a `[MODIFY]` amendment on FEAT-001's entry | same |
| 10 In-flight exposure | no other in-flight capability — no gap | no gap |

**Expected verdict: not sufficient.** Gap list: (a) architecture-store structural delta for the background index worker, staying single-process per C-001; (b) a new AX concern row with the 2-second freshness NFR target; (c) a commodity/adopt-first decision on the search-indexing technology; (d) a `contracts/api.yaml` delta adding `GET /notes/search`; (e) possibly a `data-model.md` delta for index/rebuild metadata; (f) possibly a `[MODIFY]` amendment on FEAT-001's entry if persistence is touched.

**Reads:** `spec.md`, the architecture store (`spine.md`), `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `.mochiko/features/FEAT-002/entry.md`. Never code, `tasks.md`, or this batch's own run-output dir (none exists yet).
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdicts, the gap list keyed to clauses, the store-consult result, any store trips (none visible here — no `open`/`not-now` rows collide) for the user's run-open disposition, and the `quickstart.md` null-path note (no real external-integration surface in this feature).
**Seats/skills:** the independent grading seat; `mochiko:review-sufficiency` (pointer).
**Gate:** none directly — a disputed clause would default to gap and route to the user, but nothing here requires a mid-check dispute; the verdict is presented as part of the run-open confirmation next.

---

## Phase 3 — Run-open confirmation (the entry gate — USER GATE)

**Does:** Present one blocking, plain-text confirmation (no timed prompt) that:
- Names the batch (FEAT-002, work rows W1+W2) and scope type (selection).
- Restates both attempt bounds at their only redeclaration point: **3** verification attempts per cycle, **2** gap-rework rounds at final validation — as defaults unless the user overrides them now.
- Presents the sufficiency verdict and its gap routing (Phase 2's gap list → design phase fires).
- Presents trips/conflicts for the user's ruling: none from the store-consult itself, but I would also surface here, as environmental absent-surfaces per `impl.absent-surfaces` (never auto-resolved, never run-failing): the missing governance region, the missing `.mochiko/memory/codebase-analysis.md` (offer `/mochiko:setup` or proceed greenfield with a logged warning), and — flagged separately, since it isn't one of the schema's named branches but blocks literal execution of `impl.cold-verification` and diff-based review — that the working tree is not a git repository, recommending `git init` before build-time verification needs it.
- States the done condition (every cycle card checked, test-first, independently verified per-cycle and whole, criteria traced, governance aligned, landing executed whole, run closes at final acceptance).

**Reads:** the sufficiency report just written, `implement.yaml`'s `vars:` for the bound defaults.
**Writes:** none (the confirmation itself is recorded as part of the sufficiency report / subsequent design-phase framing, not a new file).
**Gate — what's confirmed:** batch/scope, attempt bounds, gap routing, environmental trips, done condition.
- **Branch: user confirms as presented.** Proceed to Phase 4 (design phase) with 3/2 bounds standing.
- **Branch: user redeclares a bound** (e.g., raises gap-rework to 3). That value stands for the rest of this run only; recorded in the report.
- **Branch: user rules a trip** — e.g., says "skip git init, I'll verify manually" or "run `/mochiko:setup` first for the codebase analysis." Each ruling is recorded and shapes how Phase 7's cold-verification and Phase 1's brownfield read are actually carried out.
- **Branch: user stops here.** The run may pause after entry with the sufficiency report on record; nothing downstream has fired yet, so no design work is at risk.

---

## Phase 4 — Design phase (fires because Phase 2 named gaps)

**Does:** Fires scoped to exactly the named gaps, nothing more — each producing seat works on a plan I (the lead) approve first (design/build seats are not exempt from plan approval; only grading/verification/fact-finding seats are).
- `principal-architect` (architect_seat) authors the architecture-store delta: a new in-process background index-worker element (kept single-process to respect C-001 — introducing it as a separate service would trip `impl.constraint-challenge` and pause on that colliding decision alone), plus a new AX concern row carrying the SC-103 2-second freshness NFR target. This is a judgment-authored write to a governing surface, so it rides the sound-loop floor (`mochiko:patterns-sound-loop`) and, since this run now composes multiple seats (`seats: multi`), the transport floor (`mochiko:patterns-transport-floor`) governs any cross-seat messaging and the shared write to the store delta file.
- `technical-analyst` (design_seat) authors: the commodity/adopt-first evaluation for the search-indexing mechanism (`mochiko:patterns-adopt-first` — a genuine off-the-shelf candidate, SQLite FTS5, is in play, so build-vs-buy must be argued in writing, not builder-decided), the resulting `D-XXX` row as a `baseline-delta.md` entry against `constraints-and-decisions.md`, the `contracts/api.yaml` delta adding `GET /notes/search` (`mochiko:patterns-api-contracts`), and, if warranted, a `data-model.md` delta for index/rebuild metadata (`mochiko:patterns-entity-modeling`).
- If clause 9 held (persistence touch), the design phase also writes the `[MODIFY]` amendment as a marked delta on FEAT-001's entry, and the map assertion (design-implied dependencies, sharpened extent, architecture-link fill) lands on FEAT-002's entry with provenance (`mochiko:authoring-feature-map`).
- Every design element runs the simplest-execution ladder (`mochiko:patterns-plan-minimalism`), rung-justified in the output.
- Rung-appropriate exploration (e.g., "does any existing code already implement search-adjacent scanning") tiers to a `haiku` Explore subagent; interpretive judgment (is FTS5 sufficient, does this collide with C-001) stays on the session-tier seats per `mochiko:patterns-model-tiering`.

**Reads:** `sufficiency-report.md`, current `spine.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `spec.md` (for the cited acceptance criteria).
**Writes (deltas beside baselines, never in place):** `.mochiko/features/FEAT-002/` — an architecture-store delta (rendered diagram + AX-XXX row-change table, or source-plus-table if no render surface), `data-model.md` delta (if needed), `contracts/` delta, `baseline-delta.md` (the D-XXX search-tech decision), and the `[MODIFY]` amendment on `.mochiko/features/FEAT-001/entry.md` if triggered.
**Seats/skills:** `principal-architect`, `technical-analyst`; pointers `mochiko:patterns-system-design`, `mochiko:authoring-architecture-store`, `mochiko:patterns-technical-decisions`, `mochiko:authoring-technical-requirements`, `mochiko:patterns-adopt-first`, `mochiko:patterns-api-contracts`, `mochiko:patterns-entity-modeling`, `mochiko:authoring-feature-map`.

**Review pair (before the checkpoint, non-author):** `mochiko:review-plan-artifacts` (conformance to the exact gap list — blocking if it drifts materially — plus completeness/measurability) and `mochiko:review-feasibility` (contradiction/buildability, and the architecture pass since a store delta exists). Both seats authored none of the design outputs.

**Gate — design checkpoint (USER GATE, floor):** the user signs the design and the store delta together, on the rendered diagram plus the changed AX-XXX rows (or source-plus-table). No code is written before this signs.
- **Branch: user signs as-is.** Proceed to Phase 5 (card authoring) with the signed delta as the anchor for the deviation gate.
- **Branch: user asks for changes** (e.g., prefers a different search technology than FTS5, or wants the worker framed differently). The producing seat revises on a newly approved plan; the review pair re-grades; re-presented at the checkpoint. No named attempt-bound governs this loop explicitly (bounds are cycle/final-validation scoped) — it iterates until signed or the user halts the run.
- **Branch: user stops at the checkpoint.** Explicitly legal — the run may resume the build later from the signed (or still-unsigned) state.

---

## Phase 5 — Cycle card authoring & card confirm

**Does:** A design-class seat that is never the builder — `qa_seat` (`qa-engineer`) is typical staffing here, authoring the `**TEST:**` real-infrastructure gates within its slicing, alongside `technical-analyst` for the narrative slicing — maps US-101/US-102 into cycle cards using `mochiko:patterns-vertical-tdd`: foundation cycles before feature cycles. Given the design, the expected slice is: **Cycle 1 (foundation)** — the background index worker + chosen indexing mechanism wired to the existing notes store, no HTTP surface yet; **Cycle 2 (feature)** — `GET /notes/search` endpoint (US-101, SC-101/SC-102); **Cycle 3 (feature)** — freshness verification end-to-end (US-102, SC-103), reusing Cycle 1's worker. Each card carries stories/rationale, dependencies, acceptance-criteria IDs, a `**TEST:**` gate, and brownfield exposure (Cycle 1 touching FEAT-001's persistence layer is flagged `[MODIFY]`/`[EXTEND]`); no task lists or file paths (the builder decomposes those at build time).
**Reads:** the signed design deltas, `spec.md`'s acceptance criteria, `tasks.yaml` schema (or `mochiko-cli template tasks` if the binary is present) for the card grammar.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` (cycle cards, unchecked).
**Seats/skills:** `qa-engineer`, `technical-analyst`; pointer `mochiko:patterns-vertical-tdd`.
**Review (independent, before confirm):** the verification seat (`qa-engineer`, or another seat than the card author if `qa-engineer` authored the cards — independence must hold: the card author is never the confirming reviewer's same seat instance for that judgment) reviews for quality (`mochiko:review-plan-artifacts`) and buildability.

**Gate — card confirm (USER GATE, floor):** the user rules the slicing before any build starts.
- **Branch: confirmed as proposed.** Build begins at Cycle 1.
- **Branch: user wants different slicing** (e.g., merge Cycles 2/3, or split further). Re-authored, re-reviewed, re-confirmed.

---

## Phase 6 — Build: test-first cycles, foundation first

**Does, per cycle, in order (1 → 2 → 3):**
- `staff-engineer` (builder_seat) decomposes the card into concrete tasks at build time (disclosed in the cycle report), drives red→green→refactor (`mochiko:executing-tdd-cycle`), and for Cycle 1's touch to FEAT-001's persistence code, reads the whole existing file first and follows its patterns (`mochiko:brownfield-integration`). Every decomposition step runs the pre-code ladder (`mochiko:patterns-code-minimalism`) — e.g. checking whether SQLite's built-in FTS5 already covers the indexing need before hand-rolling anything, given D-001/D-002's existing stdlib-first posture.
- The builder never designs its own gaps; if it hits undesigned structure (e.g., discovers the worker needs a scheduling primitive nobody specified), that cycle halts and the design phase re-fires scoped to just that discovery (Phase 4 loop, same checkpoint), anchored to the signed delta once one exists.
- Verification (never the implementer) runs `mochiko:testing-end-user` against real infrastructure per the card's `**TEST:**` gate (evidence captured, never assumed), plus the `mochiko:review-code-minimalism` lens reading the diff, the cycle report, and the surrounding codebase.
- A cycle consumes an attempt every time it's graded; default 3 attempts per cycle (or whatever Phase 3 redeclared). Two consecutive rounds with unchanged findings is a no-progress stop — halt that cycle and present state to the user.
- Findings route by severity: Minor → default `BACKLOG.md` booking, never in-cycle; Important-or-above → blocks the cycle, joins the batched checkpoint. Reserved-to-user questions (ambiguity, adopt-first/IP-XXX calls, deviation-gate trips) accumulate to one batch at the cycle checkpoint unless build-blocking, which interrupts immediately.
**Reads (per cycle):** the card, the signed design deltas, existing FEAT-001 code the cycle touches, `constraints-and-decisions.md`.
**Writes (per cycle):** working code; `.mochiko/features/FEAT-002/cycle-report.md` (append, dated) per cycle — disclosed decomposition, honest difficulties, deviations, `domain_deps_added`; `tasks.md` checkbox flip on completion; verification reports in the same envelope.
**Seats/skills:** `staff-engineer`, verification seat (e.g. `qa-engineer`); `mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration`, `mochiko:patterns-code-minimalism`, `mochiko:testing-end-user`, `mochiko:review-code-minimalism`.
**Gate — cycle checkpoint (per cycle, batched):** escalations/findings land together; user rules any Important+ finding, any ambiguity, any adopt-first/IP-XXX call, any deviation-gate trip (build as approved vs. amend the signed delta) before that cycle's next attempt.

---

## Phase 7 — Final validation (whole-build, real infrastructure)

**Does:**
- Runs the full repository quality-gate suite (never severity-triaged — any failure fails the run per `impl.fail.quality-gate`).
- Regression sweep: re-runs FEAT-001's durable gates at `.mochiko/features/FEAT-001/gates.md` (the 3 `**TEST:**` cases — restart persistence, empty-body 400, 200/404 lookup) since this feature's territory touches FEAT-001's persistence surface; a failure here fails the run like any other regression.
- Cold verification: build and gate-run from a dependency-cold snapshot of the uncommitted working state via `git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`, after confirming the `/.claude/worktrees` ignore entry exists. **This is where the Phase 3 git-repo gap becomes concrete**: with no `.git` present, this command has nothing to list — the run would need the user's Phase-3 ruling (git-init first, or an accepted deviation) to have already resolved this, or it stalls here pending that ruling.
- Gap-finding pass (fires — selection scope): a fresh `devils-advocate` (gap_finder_seat), never a seat that built these cycles or saw the design-time test cases, is dispatched two-message and blind — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines (`data-model.md`, `contracts/`, the store's NFR-XXX rows) — states derived expectations, then probes (`mochiko:testing-gap-finding`), including the mutation lens if `depth: high` (unresolved here per the absent governance region — flagged, not assumed).
- Findings split: spec-required behavior broken → fails final validation (evidence + clause cited); beyond-spec → advisory, disposition reserved to the user (fix now / `BACKLOG.md` / accept as designed). Disputed finding-kind defaults advisory and goes to the user.
- Gap-rework bound: default 2 rounds (or Phase 3's redeclaration) at run scope; a finding localized to one cycle's territory instead charges that cycle's remaining per-cycle attempts.
**Reads:** the whole built codebase, `gates.md`, the signed design deltas, `spec.md`, baselines.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md` (or the equivalent envelope file) — gate results, regression results, cold-verification evidence, gap-finding findings and dispositions.
**Seats/skills:** verification seat for the sweep/cold run, fresh `devils-advocate` for gap-finding; `mochiko:testing-gap-finding`.
**Gate (implicit, batched with Phase 8):** beyond-spec finding dispositions and any disputed finding-kind route to the user, typically folded into the acceptance presentation next.

---

## Phase 8 — Final acceptance (USER GATE, floor)

**Does:** Presents the whole build against the done condition: every card `[x]`, test-first, per-cycle and whole verification evidence, criteria traced, governance conformed (or explicitly n/a given the absent governance region), plus the proposed landing package (store fold, FEAT-002 map graduation, baseline folds for `data-model.md`/`contracts/`/`constraints-and-decisions.md`, `gates.md` fold for fix-now/backlog gap findings) — as plain blocking text, never timed.
**Reads:** all reports produced so far.
**Writes:** none yet — landing executes only on acceptance.
**Gate — accept / amend / reject:**
- **Branch: accept.** Proceeds immediately to Phase 9 — the landing executes whole.
- **Branch: amend.** Scoped rework against the specific gap, charged to the gap-rework bound (or the localized cycle's attempts); re-presented at acceptance after rework.
- **Branch: reject.** Run halts without landing; disposition (hold the run, or the user accepts descoping some rows back to pending) is the user's call — nothing here is a graded-fold-worthy commit until acceptance.

---

## Phase 9 — Landing (executes whole, only on acceptance)

**Does (selection-scope landing):**
- Store landing: the architecture delta's elements flip `built`, their FEAT-002 keys clear; the touched rows' `As-built:`/`Drift:` fields are written as judgment and independently graded (not by the authoring architect); the orphan check runs; the store skill regenerates the derived root `ARCHITECTURE.md` (never hand-edited).
- Map graduation batch: W1 and W2 fold into FEAT-002's extent lines and vanish from the work-rows list; entry status flips to `delivered`, dated; `FEATURES.md`'s FEAT-002 index line updates; the note-search spec reads closed once both selected rows have folded (derived, never asserted).
- Baseline folds: each touched baseline (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`) folds exactly once via a graded three-way diff (pre-fold baseline + delta vs. folded result, delta applied whole, nothing else changed) — checked by the landing verification seat, never the design author.
- If a `[MODIFY]` amendment was written on FEAT-001's entry (Phase 4 clause-9 branch), it folds here too.
- `gates.md` fold: any gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted), authored in the `**TEST:**` grammar by `qa-engineer`; findings accepted as designed do not fold.
- `impl.km-landing` does not apply here (no `knowledge-management.md` present).
**Reads:** the signed deltas, the final-validation report, the accepted acceptance ruling.
**Writes:** `.mochiko/product/architecture/spine.md` fold + regenerated `ARCHITECTURE.md`; `.mochiko/features/FEAT-002/entry.md` (status → delivered, extent folded); `FEATURES.md`; `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md` (folded, not in-place edits before this point); `.mochiko/features/FEAT-002/gates.md`; `.mochiko/features/FEAT-001/entry.md` if amended.
**Seats/skills:** landing verification seat (independent grader of the folds); `mochiko:authoring-architecture-store`, `mochiko:authoring-feature-map`.

---

## Phase 10 — Close

**Does:** States a verdict against the done condition, checked against the 15-rule Not-done set (`impl.sec.fail-conditions`) — e.g. confirms every card is `[x]`, the quality gate suite passed, no unresolved deviation, the store landing carried its built-vs-signed diff, every touched baseline landed via a graded fold, the gap-finding pass ran (selection scope, so `impl.fail.gap-finding-missing` would fire if it hadn't), no spec-required finding was left unresolved, and user acceptance was actually given. Surfaces rounds consumed and seats spawned across the run in the close report.
**Reads:** all prior reports.
**Writes:** the close verdict, appended to the run's report envelope.
**Gate:** none further — this is post-acceptance.

---

## Summary of user gates encountered, and their branches

| Gate | Confirms | Accept branch | Alternative branches |
|---|---|---|---|
| Run-open (Phase 3) | batch/scope, attempt bounds, sufficiency verdict + gap routing, environmental trips (no-git, no governance region, no codebase-analysis.md), done condition | proceed to design phase | redeclare bounds; rule each trip; pause |
| Design checkpoint (Phase 4) | design + store delta together, on the rendered diagram/AX-row table | proceed to card authoring | request revision (loop); pause and resume later |
| Card confirm (Phase 5) | cycle slicing | build begins | re-slice and re-confirm |
| Cycle checkpoints (Phase 6, per cycle) | batched Important+ findings, ambiguity, adopt-first/IP-XXX calls, deviation-gate trips | next attempt/cycle proceeds | rule each item; carve/halt on no-progress or exhaustion |
| Final acceptance (Phase 8) | whole build vs. done condition + proposed landing | landing executes (Phase 9) | amend (bounded rework, re-present); reject (halt, no landing) |