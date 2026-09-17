# Action Plan — `/mochiko:implement FEAT-002`

Findings below (spec vs. baselines vs. architecture spine) come from reading the actual files in this workspace; they describe what the run would very likely surface, not a guaranteed verdict — the grading seats rule on it live.

## Phase 0 — Load the rules (done as part of producing this plan)

- Read `plugins/mochiko/schemas/implement.yaml` raw, in full (§roles, §reserved, §tools, §ways-of-working, §boundaries, §fail-conditions).
- Verified the fail-conditions section (`impl.sec.fail-conditions`) carries exactly 15 rules — matches the command's hard-coded Not-done count, so no halt-and-surface needed on that check.
- `vars` resolved for this run: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`.

## Phase 1 — Entry resolution

- Read `FEATURES.md`: `FEAT-002` = "Note search", status `selected`.
- Read `.mochiko/features/FEAT-002/entry.md`: two selected work rows, ratified via **selection scope** (spec's accepted selection, dated 2026-08-26) — not a delta card, so no delta-scope short form applies.
  - W1 — Search notes by query string (US-101; SC-101, SC-102)
  - W2 — Index stays fresh after creates (US-102; SC-103)
- Dependency line: FEAT-001 (delivered) — satisfied, not blocking.
- `FEAT-002` is a plain `FEAT-XXX`, not `EPIC-XXX` — no epic lookup, no `mochiko:authoring-epic` entry rules apply.
- Conclusion: entry gate clears — the run opens on FEAT-002, selection scope, rows W1+W2.

## Phase 2 — Sufficiency check

- Dispatch a grading seat that authored none of `spec.md`, the architecture store, or the product baselines, and that will not design or build this batch (`impl.seat-sufficiency-independence`) — a DM staffing call, e.g. `mochiko:tech-lead` or `mochiko:validator`, kept distinct from the later design seats (`technical-analyst`, `principal-architect`, `qa-engineer`) and the builder (`staff-engineer`).
- Reads (per `mochiko:review-sufficiency`, ten-clause check per row): `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`.
- Absent-surface notes to fold into the same report (`impl.absent-surfaces`, non-blocking):
  - No `.mochiko/memory/codebase-analysis.md` exists anywhere in the tree — surfaced as a brownfield-analysis gap (offer `/mochiko:setup`, or proceed with the warning logged).
  - No `.claude/rules/mochiko/` governance region exists — surfaced as a missing-governance note (`impl.briefs-name-rules-files` then has nothing to bind).
  - The architecture store has ruled content already (AX-001..003) — the "no ruled content, offer bootstrap" branch does not apply.
- Anticipated gap list on the two rows, based on the artifacts as they stand:
  - **W1 gap:** `contracts/api.yaml` defines only `POST /notes` and `GET /notes/{id}` — no `/notes/search` path, no error-response shape for the 400 case (missing/short `q`). API-contract gap.
  - **W2 gap, and a trip:** spec FR-103 requires "a background index worker" rebuilding the index off the request path. `.mochiko/product/architecture/spine.md` states, as ruled topology, "Synchronous request/response only; no queues, no background workers." That is a direct collision between required behavior and already-ruled architecture — a **trip**, not an ordinary gap: it cannot be graded `sufficient` by the grader alone (`impl.sufficiency-disputed-clause` → defaults to gap, goes to the user).
  - **W2 secondary gap:** no NFR row anywhere ties to the 2-second create-to-searchable bound (SC-103); no data-model entry represents an index/searchable-state concept.
- Output written: `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict (`sufficient` or gap list), the trip flagged for run-open, the `quickstart.md` null-path note (no external-integration surface exists here), no `[MODIFY]` amendment involved.

## Phase 3 — Run-open confirmation (the entry gate)

One blocking confirmation, presented to the user, covering exactly:
- Batch identity: FEAT-002 "Note search", selection scope, rows W1 + W2, dependency FEAT-001 already delivered.
- Attempt bounds redeclared at their only redeclaration point: per-cycle verification = 3 (default), gap-rework at final validation = 2 rounds (default) — offered for the user to keep or change now, since neither is redeclarable later.
- The sufficiency verdict and gap routing: the API-contract gap (W1), the data-model/NFR gap (W2), and the architecture trip (W2 vs. the ruled "no background workers" topology line) — all three will fire the design phase.
- The trip itself, put to the user as a ruling per `impl.user-runopen-rulings`.
- The done condition, stated plainly: every cycle card checked, test-first, verified per-cycle and whole against real infrastructure; code meets its criteria, traces to requirements, aligns with governance; acceptance landing executed whole; run closes on accept/amend/reject.

**Gate — what would be confirmed:** the attempt-bound values, and a ruling on the architecture trip.

- **Branch A — user rules "amend the architecture; allow an in-process background worker for indexing":** design phase proceeds to draft a store delta introducing/adjusting the relevant element and flow, alongside the API and data-model gaps. This is the expected, unblocking path.
- **Branch B — user rules "keep synchronous-only; reject the trip as-is":** FR-103 as written cannot be built without contradicting ruled architecture. This is bigger than an in-run design gap — it is a spec-level conflict. Per `impl.scope-escalation-fail`, work that conflicts with what the run was framed to build stays FAIL unless the user separately accepts a reframed approach (e.g., amending the spec's FR-103 wording, or accepting a different mechanism that still isn't a "background worker") — the run would not silently invent a workaround.
- **Branch C — user defers the trip on the record without ruling:** design phase proceeds on the other two gaps, but the deferred conflict still blocks W2's actual cycle later (`impl.deviation-gate`/`impl.midrun-refire` — a cycle touching that boundary halts and re-presents once concrete work reaches it).

The rest of this plan assumes Branch A, the path that lets both rows proceed; Branches B/C would truncate the plan at this gate.

## Phase 4 — Design phase (fires: all three named gaps)

- Seats and scope, each producer working only on a plan the DM approved (plan-minimalism ladder per `mochiko:patterns-plan-minimalism`, disclosed rung-wise):
  - `technical-analyst` — API-contract delta for `/notes/search` (query param, 200/400 shapes, ranking-by-recency contract) per `mochiko:patterns-api-contracts`; data-model delta if an index/searchable-state concept is needed per `mochiko:patterns-entity-modeling`; a `D-XXX` technology decision for the indexing mechanism per `mochiko:patterns-technical-decisions` — this decision must first clear `mochiko:patterns-adopt-first` (search/indexing is a commodity category; SQLite's built-in FTS capability is a candidate against hand-rolled indexing, and against `D-001`'s existing SQLite choice). The commodity-category ruling itself is not the builder's or the design seat's call — it halts to the user's checkpoint per `impl.adopt-first-user-call`.
  - `principal-architect` — the architecture-store delta resolving the trip: either amends the topology note and adds the indexing element/flow (sequence diagram for create→index→search) with a new/updated `AX-XXX` row carrying the SC-103 2-second bound as an `NFR-XXX` target, per `mochiko:patterns-system-design` + `mochiko:authoring-architecture-store`. Also checks the delta against `C-001` (single-process, no external services) — an in-process worker is consistent with C-001 as long as the delta says so explicitly.
  - `qa-engineer` — authors the `**TEST:**` real-infrastructure cases for W1 and W2 within the eventual cycle slicing (design-class seat, not the builder).
- Outputs land at `.mochiko/features/FEAT-002/` as deltas beside baselines (data-model delta, `contracts/` delta) plus the architecture-store delta at `.mochiko/product/architecture/` (signed in-flight elements only — baselines are never edited in place at this stage). DM asserts the design-implied dependency/extent sharpening back onto `FEAT-002/entry.md` and fills its Architecture link once the store delta is signed.
- Non-author review pair before the checkpoint, neither of whom authored these deltas (e.g. `mochiko:validator` for conformance, `mochiko:tech-lead` for feasibility):
  - `mochiko:review-plan-artifacts` — conformance to exactly the three named gaps, nothing more, plus card-quality (blocking).
  - `mochiko:review-feasibility` — does the store delta actually resolve the FR-103-vs-topology contradiction without opening a new one (e.g. against C-001); wrong-altitude/unjustified-structure check.

## Phase 5 — Design checkpoint (user gate, floor)

Presented: the architecture delta as a rendered diagram (or source + changed-`AX-XXX`-row table if no render surface) with its NFR target, the `/notes/search` contract delta, any data-model delta, the `D-XXX` indexing decision and its adopt-first disposition, the `**TEST:**` cases, and both reviewers' verdicts.

- **Branch — sign:** design and store delta are ratified; proceed to card authoring.
- **Branch — amend:** the flagged artifact(s) go back to their author seat for a scoped revision, re-reviewed, re-presented; does not consume the final-validation gap-rework budget (that budget is for post-build findings).
- **Branch — stop and resume later:** DM records exactly where the run paused; nothing downstream executes until the user resumes the same run.

## Phase 6 — Cycle-card authoring

- A design-class, non-builder seat (e.g. `technical-analyst`) authors `tasks.md` at `.mochiko/features/FEAT-002/` from the tasks template (`mochiko-cli template tasks`, or `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent), per `mochiko:patterns-vertical-tdd` slicing discipline — foundation cycles before feature cycles. Likely shape given the two rows:
  1. Foundation cycle — the search-index component wired to `notes-db`, no HTTP surface yet (walking skeleton).
  2. Feature cycle — `GET /notes/search` (W1: SC-101, SC-102, FR-101/102), `**TEST:**` real HTTP round trip against seeded notes.
  3. Feature cycle — background freshness (W2: SC-103, FR-103), `**TEST:**` create-then-search round trip under the 2-second bound.
- Each card carries: story/rationale, dependencies, acceptance-criteria IDs, the `**TEST:**` gate, brownfield exposure (`[EXTEND]` on the existing `api-service`/`notes-db`, since FEAT-001's own endpoints are untouched) — no task lists or file paths (builder decomposes at build time). Builder briefs would name `${rules_dir}` reads if a governance region existed; none does here, so that obligation is simply absent, not failed.
- `qa-engineer` (verification seat, non-author of the cards' slicing decisions) reviews the cards for buildability plus the `mochiko:review-plan-artifacts` quality lens before confirm.

## Phase 7 — Card confirm (user gate)

Presented: the three-cycle slicing, ordering rationale (foundation-first), and the `**TEST:**` gate set.

- **Branch — approve:** proceed to build as sliced.
- **Branch — reslice:** the card-authoring seat revises per the user's direction, re-reviewed, re-presented.
- **Branch — user adds/removes scope beyond W1/W2:** flagged under `impl.scope-escalation-fail`; requires explicit user acceptance before it's treated as in-run scope.

## Phase 8 — Build + per-cycle verification (test-first, foundation first)

For each confirmed card, in order:
- `staff-engineer` (builder) decomposes into concrete tasks (disclosed in `cycle-report.md`), runs the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`), applies `mochiko:brownfield-integration` on the existing `api-service`/`notes-db` touches, drives red→green→refactor test-first, flips the card's `[ ]` → `[x]` in `tasks.md` on completion. No git mutations — commits only suggested.
- A verification seat independent of the builder (`qa-engineer`) executes the card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user`), runs the full repository quality-gate suite (never severity-triaged — any failure fails the cycle), and applies the advisory `mochiko:review-code-minimalism` lens on the diff.
- Attempt economy: 3 verification attempts per cycle (as confirmed at run-open); two consecutive rounds with unchanged findings is a no-progress stop — halt that cycle and present state to the user rather than keep spending attempts.
- Escalation batching: reserved-to-user questions accumulate to the cycle checkpoint; anything build-blocking interrupts immediately instead of waiting.
- If the builder hits undesigned structure mid-cycle (plausible here — e.g. discovering the index needs a schema element the data-model delta didn't cover), that cycle halts and Phase 4's design phase re-fires scoped to just that discovery, then re-checkpoints (Phase 5) before resuming.
- DM surfaces rounds-consumed and seats-spawned at each checkpoint throughout.

## Phase 9 — Regression sweep, gap-finding, cold verification

- Regression sweep: re-run FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md` — restart-survival, empty-body 400, get/404) since FEAT-002 shares territory (`api-service`, `notes-db`); any failure here fails the run like any other regression.
- Gap-finding pass fires (selection scope, not delta/lane): a fresh `devils-advocate` instance, blind, two-message dispatch per `mochiko:testing-gap-finding` — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the relevant baseline rows (data-model, contracts, the new `NFR-XXX` row); never the code, `tasks.md`, `**TEST:**` cases, or reports. States derived expectations first, then probes the running system. Findings split: spec-required behavior broken → fails final validation (evidence + clause cited); beyond-spec → advisory, disposed by the user (fix-now / BACKLOG.md / accept-as-designed); a disputed kind defaults advisory and goes to the user.
- Mutation lens: owned by the verification seat (already holding code sight), at high depth only — either mutation results or an explicit stated skip, never silence.
- Cold verification: final validation builds and runs quality gates from a dependency-cold snapshot of the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`), after confirming the `.claude/worktrees` ignore entry exists. **Risk flagged for this environment specifically:** this workspace currently shows no git repository and no application source tree beyond the plugin itself, despite FEAT-001 being marked delivered — the DM would need to surface this as a real infrastructure gap (missing git repo / missing prior codebase) before this step can run, rather than silently skip it.
- Gap-rework bound: 2 rounds at run scope by default (as confirmed at run-open) for findings needing fixes; a finding localizing to one cycle's territory instead charges that cycle's own remaining attempts. Exhaustion or an unchanged-findings round halts the run and presents state — disposition is the user's.

## Phase 10 — Landing package assembly (prepared, not yet written)

Assembled for presentation, not committed to disk until acceptance:
- Store landing: the signed delta's elements flip built, `FEAT-002` key clears from the touched `AX-XXX` rows; `As-built:`/`Drift:` fields written as judgment and independently graded (e.g. by `tech-lead`); orphan check over any remaining in-flight element; `ARCHITECTURE.md` regenerated by the store skill only.
- Map graduation: W1/W2 fold into FEAT-002's Extent, vanish from Work rows; `entry.md` status → `delivered` (dated 2026-08-27); `FEATURES.md` index row for FEAT-002 updates; the note-search spec reads closed once both rows fold.
- Baseline folds (graded, three-way diff: pre-fold + delta vs. folded result): `contracts/api.yaml`, `data-model.md`, `constraints-and-decisions.md`.
- `gates.md` fold: any fix-now/backlog gap-finding results fold into `.mochiko/features/FEAT-002/gates.md` (minted), authored by `qa-engineer` in `**TEST:**` grammar; accepted-as-designed findings don't fold.
- KM landing: skipped — no `.mochiko/memory/knowledge-management.md` exists in this workspace.
- Landing verification seat checks every graded fold and confirms no map write beyond the marked W1/W2 delta occurred.

## Phase 11 — Final acceptance (user gate, floor)

Presented: full done-condition checklist — all cards `[x]`, per-cycle and whole verification evidence, clean regression sweep, gap-finding results and their dispositions, the assembled landing package.

- **Branch — accept:** DM executes the landing whole (all Phase 10 writes), closes the run with a verdict against the done condition.
- **Branch — amend:** user specifies the change; routed back to the relevant stage (design, build, or landing content) as a bounded rework round, charged against the applicable attempt/gap-rework budget.
- **Branch — reject:** no landing writes occur at all; DM records the reject verdict and presents final state.

## Phase 12 — Close: Not-done audit

Before declaring done, DM checks the 15 fail-conditions in `impl.sec.fail-conditions` (count re-verified as 15, matching the schema/`.md` pair): unrecorded sufficiency report, skipped/unsigned design phase, card built by its own author or before confirm, an unchecked card, a failing quality gate, verification claimed without evidence, a regression, an in-place baseline edit, an unresolved deviation, an incomplete store landing, an ungraded fold, a missing gap-finding pass (selection scope), an unstated skip, an unresolved spec-required gap finding, or missing user acceptance. Any one standing fails the run regardless of how far Phases 1–11 got; none standing closes the run PASS against the done condition stated at run-open.