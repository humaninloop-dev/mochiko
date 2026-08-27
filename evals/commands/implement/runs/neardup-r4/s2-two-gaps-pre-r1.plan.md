# Action Plan — `/mochiko:implement FEAT-002` (plan-only; nothing executed)

## Phase 0 — Load binding rules (done)

**Done:** Read `plugins/mochiko/schemas/implement.yaml` raw, in full, plus `plugins/mochiko/schemas/common.yaml` (pulled in by the `extends: common.register` stub). Confirmed the `kind: fail` set totals 15 nodes — matches the `.md`'s hard-coded Not‑done count, so no halt is triggered on that check.

**Read:** both schema files above; noted `vars:` (`attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`).

**Written:** nothing.

---

## Phase 1 — Resolve entry: FEAT-002 as a capability, not an epic

**Done:** `FEAT-002` matches `FEAT-XXX`, not `EPIC-XXX` — resolves as a single capability entry, so `mochiko:authoring-epic` entry rules do not apply. Scope condition resolves to **selection** (`.mochiko/features/FEAT-002/entry.md` states "selection source: the spec's accepted selection, 2026-08-26").

**Read:**
- `.mochiko/features/FEAT-002/entry.md` — Status: selected; two work rows: **W1** (search by query, US‑101, SC‑101/SC‑102) and **W2** (index freshness, US‑102, SC‑103); dependency FEAT‑001 (delivered).
- `FEATURES.md` — confirms FEAT‑002 row, FEAT‑001 already `delivered`.
- `.mochiko/specs/note-search/spec.md` — Status accepted; FR‑101–FR‑103, SC‑101–SC‑103, edge cases; FR‑103 requires "a background index worker" off the request path.
- `.mochiko/features/FEAT-001/entry.md`, `gates.md` — delivered, durable gate set of 3 `**TEST:**` cases (persistence-after-restart, empty-body 400, get/404).

**Dependency check:** W1 and W2 both depend only on FEAT‑001, which is `delivered` — no blocking; batch is eligible to run now.

**Written:** nothing.

---

## Phase 2 — Sufficiency check (`impl.sufficiency-binding-verdict`, pointer `mochiko:review-sufficiency`)

**Done:** Dispatch a seat that authored none of the spec, store, or baselines to grade each selected row (W1, W2) against the spec, the architecture store, and the product baselines, per the ten-clause check. Candidate seat: `mochiko:validator` (independent, defaults to FAIL, never grades its own work) — `requirements-analyst` authored the spec and is disqualified; `technical-analyst`/`principal-architect` are reserved as design seats and should stay unspent until gaps are confirmed.

**Read (by the grading seat):**
- `.mochiko/specs/note-search/spec.md` (again, as grading input)
- `.mochiko/product/architecture/spine.md` — ruled topology states *"Synchronous request/response only; no queues, no background workers."*
- `.mochiko/product/data-model.md` — single `Note` entity, no index/search entity
- `.mochiko/product/constraints-and-decisions.md` — C‑001 (single-process, no external services), D‑001 (SQLite via bundled driver, adopt‑first already ruled for storage), D‑002 (no web framework)
- `.mochiko/product/contracts/api.yaml` — only `POST /notes` and `GET /notes/{id}`; no search endpoint

**Findings this check would surface (concrete to this batch):**
1. **Contradiction/gap** — FR‑103's background index worker directly collides with the spine's ruled "no queues, no background workers" statement. This is a structural gap requiring an architecture‑store delta, not a documentation-only gap.
2. **Gap** — no API contract exists for `GET /notes/search`; request/response/error shapes (SC‑101/SC‑102, the edge case on short query terms) are unspecified in `contracts/api.yaml`.
3. **Gap** — no design decision exists for the search/indexing technology (commodity category: full‑text search). This is an adopt‑first candidate (e.g., SQLite FTS5, in‑process, compatible with C‑001) versus a naive scan — undecided.
4. **Absent surface, not a gap, not failing** (`impl.absent-surfaces`): no `.mochiko/memory/codebase-analysis.md`, no `CLAUDE.md`/governance region, no `.claude/rules/mochiko/` — these get surfaced to the user, never auto‑resolved, and the check still runs its absent‑baseline branch against them.
5. **Anomaly outside the formal clause set, still worth flagging at run‑open**: the working tree has no source code at all (`find` over the repo root shows only `.mochiko`, `plugins`, two index files) despite FEAT‑001 being marked `delivered` with "all cycle cards checked, final validation clean." That claim is unverifiable from disk as it stands — a candidate for an in‑flight conflict the user rules on at run‑open (proceed treating this as effectively greenfield build, or pause to reconcile FEAT‑001's status first).

**Written:** `sufficiency-report.md` in `.mochiko/features/FEAT-002/` — the store‑consult result, the gap list above (findings 1–3), the trips for the user (findings 1, 3, and the absent‑surface/anomaly items), any no‑delta claim (none — a store delta is implicated), the `quickstart.md` null‑path note (no real external‑integration surface exists here), no `[MODIFY]` amendment against a delivered feature is named by this row set. Per‑row verdict: **W1: gap** (contract + index-tech), **W2: gap** (architecture conflict).

---

## Phase 3 — Run‑open confirmation (the entry gate)

**Gate:** One blocking, plain-text confirmation (`impl.acceptance-plain-text`) that would state: the batch is FEAT‑002 (Note search), scope type **selection**; the attempt bounds redeclared here — 3 attempts per cycle, 2 gap‑rework rounds at run scope (schema defaults, unless the user overrides at this single redeclaration point); the sufficiency verdict and its gap routing (findings 1–3 above route to a design phase); the trips reserved to the user (the architecture‑spine collision, the adopt‑first indexing call, the absent governance/codebase-analysis surfaces, the FEAT‑001-delivered-but-no-code anomaly); and the done condition (every cycle card checked, test‑first, independently verified against real infrastructure per‑cycle and whole, acceptance landing executed whole, run closes on accept/amend/reject).

**Onward branches:**
- **User rules the architecture-spine trip → allow the change:** design phase (Phase 4) authors a store delta adding a background‑worker element/arrow; the deviation gate (`impl.deviation-gate`) is pre‑cleared by this same sign‑off since it happens before any card is built.
- **User rules the architecture-spine trip → disallow background workers:** FR‑103 is escalated back toward the spec's business intent — the design phase instead designs synchronous (write‑time) indexing, which is itself a scope deviation from spec.md and needs its own confirmation before the design phase treats it as ratified, or the run pauses pending a spec amendment.
- **User rules the architecture-spine trip → defer:** recorded on `sufficiency-report.md` as deferred; the design phase cannot close the affected gap and the run halts at the design checkpoint until resolved.
- **User rules the adopt‑first indexing call:** picks a candidate (e.g., SQLite FTS5) or explicitly asks the design phase to propose alternatives — either way this becomes a D‑XXX entry the design phase authors, never a builder‑decided call (`impl.adopt-first-user-call`).
- **User rules the absent‑governance/codebase-analysis surfaces:** proceed as-is (no obligated rules‑file reads bind, since `impl.briefs-name-rules-files` only fires `when: governance_region: present`), or pause and route to `/mochiko:setup` first.
- **User rules the FEAT‑001-anomaly:** proceed treating the codebase as empty/greenfield for this build, or pause to reconcile FEAT‑001's recorded status against disk state before continuing.
- **User declines to open the run:** stop here; no design phase, no cards, nothing further happens.

Assuming the user opens the run (any of the "proceed" branches above), planning continues.

**Written:** nothing new yet — the confirmation only restates Phase 2's report and captures rulings that Phase 4 will act on.

---

## Phase 4 — Design phase (fires: gaps were named)

**Done:** Scoped strictly to the named gaps — architecture delta for the indexing mechanism, API contract for `GET /notes/search`, and the index-technology decision — nothing beyond. Each producing seat plans first on a plan the run approves (`impl.plan-approval-producers`), rung‑justified per `mochiko:patterns-plan-minimalism`.

**Seats:**
- `architect_seat` (`principal-architect`) — owns the store delta: new topology element/arrow for the index mechanism (worker or in‑process trigger, per the run‑open ruling), new/updated AX‑XXX row(s), NFR target for the 2‑second freshness bound (SC‑103). Uses `mochiko:patterns-system-design` (delta diagram, sequence diagram for the create→index→search flow) and `mochiko:authoring-architecture-store` for spine grammar.
- `design_seat` (`technical-analyst`) — owns `contracts/api.yaml`'s delta (new `GET /notes/search` endpoint: query param, 200/400 responses, ranking) via `mochiko:patterns-api-contracts`, and `constraints-and-decisions.md`'s delta: the new D‑XXX for the indexing technology via `mochiko:patterns-technical-decisions`/`mochiko:patterns-adopt-first`, plus any NFR-XXX wiring into the store's concern rows.
- `qa_seat` (`qa-engineer`) — authors design‑time acceptance test cases for W1/W2 within the design seats' slicing.
- `mochiko:authoring-feature-map` obligation (`impl.design-map-assertion`) — the design phase asserts design‑implied dependencies/extent onto FEAT‑002's entry with provenance and fills the architecture link once the store delta exists.

**Model tiering:** any locate/enumerate reads during design (e.g., checking whether other features already touch `notes-db`) dispatch to a native `Explore` subagent with `model: haiku`; interpretive/judgment reads (spec intent, architecture fit) stay on the session tier — per `mochiko:patterns-model-tiering`.

**Independent review pair (`impl.design-review-pair`):** a non‑author seat (e.g. `mochiko:tech-lead` or `mochiko:validator`) applies `mochiko:review-plan-artifacts` (conformance to the named gap list, card quality — blocking) and `mochiko:review-feasibility` (contradiction/buildability, and the architecture pass since a store delta exists).

**Read:** everything from Phase 2 plus the run‑open ruling on the architecture‑spine trip and the adopt‑first indexing call.

**Written:**
- `.mochiko/product/architecture/spine.md` **delta** (not in place — beside it, per `impl.baselines-never-in-place`), rendered diagram or (no render surface confirmed: `mochiko-cli` is absent from this environment) the source-plus-changed-element-table fallback.
- `.mochiko/product/contracts/api.yaml` delta for the new endpoint.
- `.mochiko/product/constraints-and-decisions.md` delta (new D‑XXX).
- `.mochiko/features/FEAT-002/` design outputs (deltas beside baselines, per `impl.design-outputs-home`).
- `.mochiko/features/FEAT-002/entry.md` — map assertion (dependencies, extent, architecture link).

**Gate — design checkpoint (`impl.gate-design-checkpoint`):** the user signs the design and the store delta (rendered diagram + changed AX‑XXX rows, or the source+table fallback given the absent binary). 
- **Branch — signs as‑is:** proceed to Phase 5.
- **Branch — requests changes:** design seats rework within scope; re‑review by the same non‑author seat; re‑present.
- **Branch — stops here:** the user may halt and resume the build later — run pauses with the signed (or unsigned) state recorded; nothing downstream happens until resumed.

---

## Phase 5 — Cycle cards + card confirm

**Done:** A design‑class, non‑builder seat (e.g. `design_seat`/`technical-analyst`, or `qa_seat` for the `**TEST:**` cases within that slicing) authors `tasks.md` cycle cards from the signed design, per `mochiko:patterns-vertical-tdd` — foundation cycles (the index mechanism / index‑population plumbing) before feature cycles (the search endpoint itself, freshness behavior). Concretely this batch likely slices as: **Cycle A** — index infrastructure + write‑path integration (foundation, underlies both W1 and W2); **Cycle B** — `GET /notes/search` endpoint and ranking (W1, US‑101); **Cycle C** — freshness/latency behavior under the 2‑second bound (W2, US‑102) — final slicing is the authoring seat's judgment, not fixed here.

Each card carries stories/rationale, dependencies, acceptance criteria by ID (SC‑101/102/103), a `**TEST:**` real‑infrastructure gate, and brownfield exposure — FEAT‑001's existing `POST /notes`/`GET /notes/{id}` path and `notes-db` are `[EXTEND]` targets since search reads that store.

**Review before confirm (`impl.card-review-before-confirm`):** the verification seat (independent, non‑author — e.g. `qa_seat` or the tech-lead used in Phase 4) reviews card quality (`mochiko:review-plan-artifacts`) and buildability (own judgment).

**Read:** the signed design deltas, `spec.md`'s cited acceptance criteria, `tasks_schema` (`plugins/mochiko/schemas/tasks.yaml`) since `mochiko-cli` is absent — the schema Read raw is the first‑class source of truth for card rendering in this environment.

**Written:** `.mochiko/features/FEAT-002/tasks.md` (cycle cards).

**Gate — card confirm (`impl.gate-card-confirm`):** the user rules the slicing before any build begins.
- **Branch — approves:** proceed to Phase 6.
- **Branch — requests re‑slicing:** cards revised, re‑reviewed, re‑presented — no card is built before this gate clears.

---

## Phase 6 — Build, cycle by cycle

**Done:** `builder_seat` (`staff-engineer`) takes each approved card in order (foundation before feature), decomposes it into concrete tasks at build time — disclosed in `cycle-report.md` — and builds test‑first (`mochiko:executing-tdd-cycle`: red → green → refactor), following `mochiko:brownfield-integration` on the `[EXTEND]` touches to the existing `POST /notes` path and `notes-db`, and `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed — e.g., checking whether SQLite's built‑in FTS5 already covers the ranking/filter need before writing custom indexing code).

**Per‑cycle verification:** `qa_seat` (`qa-engineer`, never the implementer) grades against real infrastructure via `mochiko:testing-end-user`, plus the advisory `mochiko:review-code-minimalism` lens (reads the diff, the cycle report, and the surrounding code — reuse claims never on trust). A cycle consumes one of its 3 attempts each time it's graded, whatever the round is called (`impl.attempt-per-grade`); two consecutive rounds with unchanged findings is a no‑progress stop (`impl.no-progress-stop`) that halts the cycle and presents state to the user. Escalations and Important‑or‑above findings batch at the cycle checkpoint; Minor findings default to a `BACKLOG.md` booking. A build‑blocking undesigned‑structure discovery halts that cycle and re‑fires a scoped design‑phase redo (`impl.midrun-refire`), anchored to the already‑signed delta.

**Read (per cycle):** the card, the relevant baseline deltas, existing code at the touched paths (once any exists), `.claude/rules/mochiko/` obligated reads — none currently exist, so this obligation is inert this run.

**Written:** source code at whatever paths the design implies (none exist yet in this repo, so Cycle A would be creating the first application code under this scope); `tasks.md` checkbox flips as cycles complete (`impl.progress-surface`); a `cycle-report.md` and a verification report per cycle in `.mochiko/features/FEAT-002/` (`impl.reports-envelope`).

**No user gate mid‑phase** unless a card proves infeasible (escalated as a business‑level scope decision, `impl.infeasible-card-escalation`), an ambiguity surfaces, or an attempt/no‑progress bound trips — each of those routes to the batched checkpoint or a blocking interrupt per `impl.escalation-batching`.

---

## Phase 7 — Final validation (whole‑build)

**Done:**
- **Regression sweep** (`impl.regression-sweep`): re‑runs FEAT‑001's durable gate set from `.mochiko/features/FEAT-001/gates.md` (the 3 `**TEST:**` cases: restart‑persistence, empty‑body‑400, get/404) plus this feature's own gates exercising the shared `notes-db` seam. A failure here fails the run.
- **Cold verification** (`impl.cold-verification`): builds and runs quality gates from a dependency‑cold snapshot of the uncommitted working state via `git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-note-search/`.
  - **Concrete blocker found in this environment:** the working directory is confirmed **not a git repository** (`git status` → `fatal: not a git repository`), so `git ls-files` cannot run as specified. This is a trip that would need surfacing at (ideally) run‑open, or at latest here: **gate branch** — the user is asked whether to `git init` the working tree so cold verification can proceed as specified, or to accept an alternate cold‑snapshot mechanism for this run. No `git init` or other action is taken without that confirmation (initializing version control here is a state‑changing action worth confirming, consistent with the "actions affecting shared/local state" guidance).
  - Also ensures the `/.claude/worktrees` ignore entry exists — currently no `.gitignore` exists at all, so this entry would need adding once/if a git repo is established.
- **Gap‑finding pass** (`impl.gap-finding-scope`, scope=selection → runs): blind two‑message dispatch to a fresh `gap_finder_seat` (`devils-advocate`) that built nothing here and saw no test cases. First message: `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines' relevant slices only — never code, `tasks.md`, `**TEST:**` cases, or reports (`mochiko:testing-gap-finding`). Mutation lens skipped/disclosed per `depth` — no governance region exists to declare `depth: high`, so this run would state the mutation lens as not applicable/skipped rather than silently omitted.

**Read:** accumulated gate sets, the cold‑snapshot build output, gap‑finder's probe results.

**Written:** `final-validation-report.md` in `.mochiko/features/FEAT-002/` — regression results, cold‑verification results (or the stated git‑repo blocker/resolution), gap‑finding findings split by kind (spec‑required‑behavior‑broken → fails the validation; beyond‑spec → advisory, disposition reserved to the user at Phase 9's checkpoint).

---

## Phase 8 — Landing (scope: selection)

**Done (`impl.landing-selection` + `impl.store-landing` + `impl.graded-fold`):**
- Store landing: the signed delta's elements flip to `built`, FEAT‑002 keys clear, `As-built:`/`Drift:` fields written as judgment and independently graded, orphan check runs, `ARCHITECTURE.md` regenerated by the store skill (never hand‑edited).
- Every touched baseline (`contracts/api.yaml`, `constraints-and-decisions.md`, possibly `data-model.md` if search introduces new attributes) folds exactly once via a graded three‑way diff, checked by the landing verification seat.
- Map graduation: W1/W2 fold into FEAT‑002's extent lines, status set `delivered` (dated), `FEATURES.md` index line updates, the `note-search` spec's index row is touched — closes exactly when both selected rows have folded.
- Gap‑finding findings ruled fix‑now or backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted, since it doesn't yet exist), authored by `qa_seat` in the `**TEST:**` grammar; findings accepted as designed do not fold.

**Written:** `.mochiko/product/architecture/spine.md` (folded), `ARCHITECTURE.md` (regenerated), `.mochiko/product/contracts/api.yaml` (folded), `.mochiko/product/constraints-and-decisions.md` (folded), `.mochiko/features/FEAT-002/entry.md` (extent/status), `FEATURES.md`, `.mochiko/specs/note-search/spec.md` index status, `.mochiko/features/FEAT-002/gates.md` (minted).

---

## Phase 9 — Final acceptance and close

**Gate (`impl.gate-final-acceptance`):** plain blocking text presenting the whole build's evidence — cycle reports, final‑validation report, landing diffs — for accept / amend / reject.
- **Branch — accept:** run closes; a verdict is issued against the done condition, checking off every card `[x]`, test‑first build, independent per‑cycle and whole verification against real infrastructure, criteria traceability, governance alignment (none formally present this run), whole‑landing execution. `impl.dm-close-verdict` obliges stating this explicitly.
- **Branch — amend:** specific changes requested; routes back to the relevant phase (a cycle rebuild, a design amendment, or a landing correction) rather than closing.
- **Branch — reject:** run does not close as delivered; state is presented and the disposition (rework vs. abandon this batch) is the user's from there.

**Close‑time Not‑done check:** before declaring done, the run verifies none of the 15 `kind: fail` conditions stand — most pointedly here: `impl.fail.sufficiency-unrecorded` (cleared by Phase 2's report), `impl.fail.design-skipped` (cleared by Phase 4's signed checkpoint), `impl.fail.card-independence`/`impl.fail.card-unchecked` (cleared by Phases 5–6), `impl.fail.regression`/`impl.fail.no-evidence` (cleared by Phase 7), `impl.fail.baseline-in-place`/`impl.fail.ungraded-fold`/`impl.fail.store-landing-incomplete` (cleared by Phase 8), `impl.fail.gap-finding-missing` (applies — this is a selection‑scope run, so the pass is mandatory, not skippable), `impl.fail.spec-gap-unresolved`, and `impl.fail.no-acceptance` (cleared only once Phase 9 actually accepts).

**Written:** none beyond the reports/folds already listed — Phase 9 is verdict and gate only.