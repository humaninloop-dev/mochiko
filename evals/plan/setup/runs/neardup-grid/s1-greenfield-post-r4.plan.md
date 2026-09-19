# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Scenario basis** (read this pass, no writes made): repo root holds only `README.md` (project "Cadence" — an async standup digest web app, "Nothing is built yet... no source, no dependencies, no configuration") and a non-governance `CLAUDE.md` ("Cadence — working notes", no `<!-- mochiko:governance:begin/end -->` markers). No `.mochiko/` directory, no `.claude/rules/mochiko/`, not a git repo. This strongly indicates **greenfield**.

Schema check: `plugins/mochiko/schemas/setup.yaml` carries exactly **6** `kind: fail` rules under `setup.sec.fail-conditions` — matches the command's hard-coded count, so the pair is in sync; no halt needed on that account.

---

## Phase 0 — Load governing schema (already done this pass)
- **Reads:** `plugins/mochiko/schemas/setup.yaml` (raw, full), `plugins/mochiko/schemas/common.yaml` (raw, full — pulled in by `extends:` stubs), `plugins/mochiko/schemas/command-labels.yaml`.
- **Writes:** none.
- **Seats/skills:** none — lead-only.
- No gate.

## Phase 1 — Entry & mode proposal
- **Reads:** workspace signals — `README.md`, `CLAUDE.md`, absence of `.mochiko/`, absence of `.claude/rules/mochiko/`, absence of `package.json`/source/`.git`. Per model-tiering (`setup.model-tiering` → `common.model-tiering`), this locate/enumerate sweep would ride a native `Explore` subagent spawned `model: haiku`, not the session tier.
- **Proposal:** mode = **greenfield** (notes-only repo, no delivered code to reconstruct from).
- **Writes:** none.
- **Gate (`setup.user-mode-ruling`, reservation):** the lead presents "propose greenfield — confirm or override?" as plain blocking text.
  - *User confirms greenfield* → continue to Phase 2 as below.
  - *User rules brownfield* → pivot: run `mochiko:analysis-codebase` into `.mochiko/memory/codebase-analysis.md`, extend into feature-map reconstruction (`setup.feature-map-brownfield`) and product-baseline bootstrap (`setup.baselines-bootstrap`) later in the run; conflicts between detected reality and declared intent become an explicit user gate (`setup.user-conflict-rulings`).
  - *User rules amend* → look for an existing `.mochiko/memory/governance-intent.md` to use as the amend baseline; if absent, surface that and offer to seed it rather than silently scaffolding; a missing feature map is likewise surfaced and offered, never auto-scaffolded (`setup.map-never-overwrite`).
  - The rest of this plan assumes the confirmed-greenfield branch.

## Phase 2 — Interrogation (inline, lead-run — no subagent)
- **Duty:** `setup.interrogation-inline` — the lead itself runs this, never delegated to a seat.
- **Skill:** `mochiko:analysis-iterative` for the adaptive agenda dimensions, then the catalog deck card by card, recommend-then-arbitrate.
- **Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck (at minimum `backend-service.md`, likely paired with a frontend/web-facing card set given "web app with a morning email digest"), `DOMAIN-DEPENDENCIES.md`, `ESSENTIAL-FLOOR.md`, `COMPLIANCE-MODULES.md`, `EMERGENT-CEILING-PATTERNS.md`.
- **Writes:** none yet — rulings held in-session pending synthesis.
- **Gate (`setup.user-card-rulings`, reservation — repeats per card/module):** each card's ruling and every waiver is the user's. Branches per card: adopt / decline / defer ("not-now") / waive-with-reason. Lead records the ruling and moves to the next card; no aggregate approval substitutes for per-card rulings.

## Phase 3 — Synthesis authoring
- **Writes:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace), built from the governance-intent template/schema (`plugins/mochiko/schemas/governance-intent.yaml` as the first-class fallback). This is the run's first artifact write and happens only after Phase 2 closes, before any governance surface exists — satisfies the non-waivable `setup.fail.pre-ratification-authoring` floor.
- **Superseded-file check (`setup.constitution-superseded`):** if `.mochiko/memory/constitution.md` existed on disk it would be deleted on sight with a one-line note; not present in this workspace, so no-op here.
- **Seats/skills:** lead only.
- No gate yet (ratification is Phase 5).

## Phase 4 — Pre-ratification stress test (cold seat)
- **Gate (`setup.stress-test-cold-seat`):** the lead asks whether to spawn a cold reviewer for the synthesis, or to record a waiver instead.
  - *User authorizes a cold seat* → would spawn **`mochiko:devils-advocate`** (or the equivalent `mochiko:review-governance-intent` procedure) as the cold INTENT REVIEWER.
    - **Dispatch discipline (`setup.blind-map-dispatch`, floor, non-waivable):** two-message spawn. Message 1 carries only the setup topic and project identity/goal (Cadence: async-standup digest web app) — never the synthesis path — so the seat builds its Phase-0 angle map blind. Only after that map returns does message 2 hand over `.mochiko/memory/governance-intent.md` for the cold read. If staffed as a pair, both build maps independently before either sees the synthesis (`setup.staffing-latitude` — solo vs pair is the lead's call; solo proposed here given project scale).
  - *User waives* → no cold seat spawned; the waiver is recorded in the trace/ledger and the run proceeds straight to Phase 5.
- **Gate (`setup.coverage-survivor-routing`, routing):** any coverage gap the stress test surfaces is presented as a candidate scope topic, not a card defect.
  - *Explore now* → re-enter `mochiko:analysis-iterative` on that angle; new intent lands under a fresh GI-XXX entry, folded into the synthesis before ratification.
  - *Rule inline* → user rules the point directly; folded into the synthesis.
  - *Defer* → logged as out of scope for this run; does not block ratification.
  - *Overlaps an existing agenda dimension* → routed back through ordinary Phase-2 follow-up instead.
- **Writes:** possible edits to `.mochiko/memory/governance-intent.md` from folded findings; no governance surface yet.

## Phase 5 — Ratification gate
- **Gate (`setup.gate-synthesis-ratification`, floor, non-waivable):** the (possibly revised) synthesis is presented to the user for ratification as plain blocking text (`setup.acceptance-plain-text` → never a timed prompt).
  - *Ratified as-is* → proceed to Phase 6.
  - *Ratified with edits* → apply edits to `.mochiko/memory/governance-intent.md`, then proceed.
  - *Not ratified* → loop back to Phase 2/3/4 as needed; no surface may be authored until this gate closes.

## Phase 6 — Author the governance surface set
- **Seat:** would spawn **`mochiko:tech-lead`** as the producer (its role explicitly covers authoring the governance surface). Per `common.plan-approval-producers`, it plans first and works only on a lead-approved plan.
- **Internal gate:** lead approves the tech-lead seat's authoring plan (which surfaces, which modules, which rule-file paths) before any write proceeds.
- **Skill:** `mochiko:authoring-constitution` (composition and read scope), consuming the ratified synthesis.
- **Writes (by the tech-lead seat, following the ratified synthesis):**
  - `CLAUDE.md` — regenerate the marked region `<!-- mochiko:governance:begin --> … <!-- mochiko:governance:end -->` idempotently; the existing "Cadence — working notes" content outside the markers is left untouched (`setup.governance-region-ownership`, floor).
  - `.claude/rules/mochiko/*.md` — new paths-scoped rule files per adopted module/domain, including the domain-layer file carrying the `mochiko:domain-registry` block and `.claude/rules/mochiko/output-style.md` carrying the `mochiko:output-style` switch pair. Neither exists yet, so both are created fresh (the carve-out-preservation floor, `setup.carve-outs-preserved`, would only bind on a future amend run).
  - `.mochiko/memory/governance-ledger.md` — the run's ledger.
  - If a knowledge-management module was adopted in Phase 2: scaffold from `templates/constitution-modules/knowledge-management.md`, plus the pinned copy at `.mochiko/memory/knowledge-management.md`.
  - `FEATURES.md` — empty index scaffold (`setup.feature-map-greenfield`).
  - `.mochiko/product/architecture/` — created if absent, with a `spine.md` stub (header + `Scope:` line, write-if-absent) declaring the surface types this product carries (likely `frontend-web` + `backend-service`, composed) per `setup.architecture-scope-handoff`, and an empty `concerns.md` beside it — done unconditionally regardless of mode (`setup.store-scaffold-unconditional`). No architecture *ruled content* is authored here (`setup.store-ruled-content-never-here`, floor) — that's `/mochiko:architecture`'s job.
  - Product baselines (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`) are **not** written this run — greenfield defers them to the first `/mochiko:implement` design phase.
  - A trace summary linking every ratified card/module ruling and waiver to the surface(s) it landed on.

## Phase 7 — Independent grade of the surface set
- **Seat:** would spawn **`mochiko:validator`** (never the author) running `mochiko:validation-constitution` against the actual files on disk — CLAUDE.md governance region, `.claude/rules/mochiko/*.md`, the ledger — defaulting to FAIL until every clause clears: mode fidelity, per-card/module traceability, waiver traceability, trace closure, and Essential Floor category coverage (every floor category carries a principle or a recorded waiver — `setup.fail.floor-category-uncovered`).
- **Optional add-on (SHOULD, offered not assumed):** `mochiko:testing-governance-injection` — an empirical probe that the CLAUDE.md region and `.claude/rules/mochiko/` paths actually inject into spawned agents. Would be proposed to the user as a cheap first-run sanity check; declinable.
- **Writes:** none beyond the grader's findings/report.
- **Branch:** FAIL/needs-revision → loop to Phase 6 with the fix list before Phase 8; PASS → proceed.

## Phase 8 — Final acceptance
- **Gate (`setup.gate-final-acceptance`, floor, non-waivable):** the surface set is presented to the user flagged proposal by flagged proposal, alongside the trace summary and the validator's grade, as plain blocking text.
  - *Accept all* → proceed to close.
  - *Accept some / reject some* → rejected flags loop back to Phase 6 for rework and re-presentation; accepted flags lock.
  - *Reject outright* → loop further back (Phase 5 or even Phase 2) depending on what the rejection reveals.

## Phase 9 — Close
- Bump the CLAUDE.md governance region's semver.
- Confirm the greenfield feature-map scaffold and the deferred-baselines split are both honored as designed.
- Confirm `.mochiko/product/architecture/spine.md` stub + `Scope:` line and empty `concerns.md` exist.
- Re-check the 6 `kind: fail` conditions do not stand: pre-ratification authoring, unclosed trace, author-graded, floor-category-uncovered, no-acceptance, no-feature-map.
- Report in `templates/output-style.md` register: summarize what was written, the trace, the semver bump; suggest (never run) a commit — and since this workspace is not yet a git repo, explicitly flag that a `git init` would be the user's call first, not something the run performs.
- Report next-step routing: `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline (peer doors, neither ahead of the other), plus `/mochiko:brainstorm` if knowledge-management was adopted.

---

**Seats/skills that would be involved overall:** lead (inline interrogation, orchestration), `mochiko:devils-advocate` (cold synthesis stress test, or a user waiver in its place), `mochiko:tech-lead` (surface authoring), `mochiko:validator` (independent grade), optionally `mochiko:testing-governance-injection` (empirical probe). `mochiko:patterns-transport-floor` would only actually trigger if two of these seats were composed concurrently (e.g., a paired cold review) — as sequenced above, each seat's work gates the next, so multi-seat concurrency isn't required by default.