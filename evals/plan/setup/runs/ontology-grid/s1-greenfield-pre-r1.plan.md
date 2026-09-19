# Action Plan — `/mochiko:setup` (plan-only, not executed)

## Grounding read (already performed, no writes)

- Read `plugins/mochiko/schemas/setup.yaml` in full (428 lines). Confirmed the fail-condition set under `setup.sec.fail-conditions` has exactly 6 entries (`setup.fail.pre-ratification-authoring`, `setup.fail.unclosed-trace`, `setup.fail.author-graded`, `setup.fail.floor-category-uncovered`, `setup.fail.no-acceptance`, `setup.fail.no-feature-map`) — matches the hard-coded count, so the run is not halted for a schema/count mismatch.
- Read `plugins/mochiko/schemas/command-labels.yaml` to resolve rule labels.
- Read `CLAUDE.md` and `README.md`, and listed the repo tree.

Workspace facts gathered (read-only):
- No `.mochiko/` directory anywhere — no `governance-intent.md`, no `governance-ledger.md`, no `constitution.md` to supersede, no `FEATURES.md`, no `.mochiko/product/architecture/`.
- No `.claude/rules/mochiko/` — no prior governance region in `CLAUDE.md` (the file present is plain working notes, no `<!-- mochiko:governance:begin/end -->` markers).
- `README.md` states explicitly: "Nothing is built yet... no source, no dependencies, no configuration." Product: **Cadence**, an async-standup digest tool for distributed engineering teams; web app + morning email digest; two part-time engineers; first pilot is one internal team of six.
- Only the `plugins/mochiko/` plugin tree itself exists as "code" — not product source.

This evidence points unambiguously to **greenfield**. Per `setup.user-mode-ruling`, the mode ruling is the user's whenever ambiguous; here it's not ambiguous, but the mode is still proposed to the user for confirmation before the run commits to the greenfield path (branches described in Phase 1's gate).

---

## Phase 1 — Mode proposal and confirmation gate

- **Does:** Present the workspace evidence above and propose mode = **greenfield**.
- **Reads:** Nothing further (already read in grounding step).
- **Writes:** Nothing.
- **Seats:** None — lead-only.
- **Gate (user, `setup.user-mode-ruling`):** "I'm treating this as a greenfield setup — no prior `.mochiko/` state, no source. Confirm, or rule brownfield/amend instead?"
  - **Greenfield (confirmed):** proceed to Phase 2 on the greenfield path.
  - **Brownfield ruling:** re-scope Phase 2 to invoke `mochiko:analysis-codebase` against whatever the user points at as "existing" (even though the repo currently shows none) before interrogation; feature-map and baseline landing shift to the brownfield rules (`setup.feature-map-brownfield`, `setup.baselines-bootstrap`).
  - **Amend ruling:** halts this plan's greenfield branch — amend requires an existing `governance-intent.md` as the durable baseline (`setup.synthesis-artifact`), which does not exist here, so the user would need to clarify what is being amended before the run can proceed; I would surface that contradiction rather than silently picking a mode.

---

## Phase 2 — Interrogation input staging (greenfield path)

- **Does:** Loads the fixed interrogation inputs; no codebase analysis skill fires (that's brownfield-only).
- **Reads:**
  - `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`
  - `plugins/mochiko/skills/authoring-constitution/references/DOMAIN-DEPENDENCIES.md`
  - `plugins/mochiko/skills/authoring-constitution/references/catalog/universal-floor.md` (always in scope)
  - `plugins/mochiko/skills/authoring-constitution/references/catalog/backend-service.md` (candidate — Cadence has a server-side digest component; confirmed/declined during interrogation, not presumed here)
  - Check `.mochiko/memory/constitution.md` for supersession (`setup.constitution-superseded`) — already confirmed absent, nothing to delete.
- **Writes:** Nothing yet.
- **Seats:** None — lead-only staging.

---

## Phase 3 — Inline interrogation session

- **Does:** I run the interrogation myself, inline (`setup.interrogation-inline` — never delegated to a subagent). Two stages: (a) agenda dimensions worked adaptively via the `mochiko:analysis-iterative` skill (one question at a time, Who/Problem/Value-style discovery grounded in Cadence's README pitch), then (b) the catalog deck worked card-by-card, recommend-then-arbitrate — I propose a stance per card, the user rules it.
- **Reads:** `INTERROGATION-AGENDA.md`, catalog cards loaded in Phase 2, `DOMAIN-DEPENDENCIES.md` for cross-card conflicts.
- **Skill invoked:** `mochiko:analysis-iterative` (pointer on `setup.interrogation-inline`).
- **Writes:** Nothing yet — this session produces rulings, not files.
- **Gate (user, `setup.user-card-rulings`, repeated per card):** Every agenda-dimension answer, every catalog-card ruling, and every waiver is the user's call, not mine or a seat's.
  - **User accepts my recommendation:** ruling is recorded as-is.
  - **User overrides:** the override replaces my recommendation verbatim.
  - **User defers a card:** it is flagged open and carried into the synthesis as an explicit gap rather than silently dropped.

---

## Phase 4 — Draft the synthesis artifact

- **Does:** Renders the interrogation's rulings into the durable synthesis.
- **Writes:** `.mochiko/memory/governance-intent.md`, GI-XXX namespace — via `mochiko-cli template governance-intent`, falling back to a raw read of `plugins/mochiko/schemas/governance-intent.yaml` if the binary is unavailable.
- **Reads:** `plugins/mochiko/schemas/governance-intent.yaml` (only if the CLI binary is absent).
- **Seats:** Lead-authored (the interrogation is mine, so the synthesis transcription is mine too — this is not yet the "surface set," so `setup.fail.pre-ratification-authoring` does not apply to this write).

---

## Phase 5 — Pre-ratification stress test (cold seat)

- **Does:** Before the user ratifies, an independent seat stress-tests the synthesis (`setup.stress-test-cold-seat`), spawned blind in two messages (`setup.blind-map-dispatch`):
  1. First message: setup topic + Cadence's project identity/goal only — **never** the path to `governance-intent.md`. The seat builds a Phase 0 angle map blind to what the interrogation concluded, and that map returns before anything else is sent.
  2. Second message: the synthesis path is sent; the cold read begins against the actual file.
- **Seat:** `mochiko:devils-advocate` agent, running the `mochiko:review-governance-intent` skill (this skill's own trigger is exactly this stress-test moment).
- **Reads (by the seat):** `.mochiko/memory/governance-intent.md` only after step 2.
- **Writes:** None to product files — the seat returns findings, severity-classified.
- **Transport note:** if paired reviewers were used instead of a solo cold seat, `mochiko:patterns-transport-floor` governs the two-message dispatch and fan-in — non-waivable once triggered, referenced not restated.
- **Gate (user, alternative path):** the user may instead record a waiver of the cold stress-test (`setup.stress-test-cold-seat` permits "or the user's recorded waiver"). If waived, Phase 5 is skipped and the waiver is logged in the trace; Phase 6 then has no findings to route.

---

## Phase 6 — Route stress-test survivors

- **Does:** Any coverage finding that survives Phase 5 questions the *setup's scope*, not a card in isolation (`setup.coverage-survivor-routing`). Each survivor is presented as a candidate topic.
- **Writes:** Nothing yet.
- **Gate (user, per surviving gap):**
  - **Explore now:** re-enter `mochiko:analysis-iterative` on that specific angle; the re-elicited intent lands as new GI-XXX entries appended to the synthesis, and Phase 4/5 partially re-run for the delta.
  - **Rule inline:** the user answers the gap directly without a full sub-session; recorded straight into the synthesis.
  - **Defer:** the gap is logged as an explicitly open item, not silently dropped, and does not block ratification.
  - A gap that overlaps an existing agenda dimension instead follows the ordinary interrogation-follow-up path (back into Phase 3's normal flow), not this routing.

---

## Phase 7 — Ratification gate

- **Does:** Nothing more is authored until this gate clears (`setup.gate-synthesis-ratification`, and the hard floor `setup.fail.pre-ratification-authoring`).
- **Gate (user):** "Here is the final synthesis, with Phase 5/6 findings folded in. Ratify as-is, ratify with edits, or send back for more interrogation?"
  - **Ratify as-is:** proceed to Phase 8.
  - **Ratify with edits:** apply the edits directly to `governance-intent.md`, then proceed — no re-stress-test is mandated by the schema unless the edits are material, in which case I'd propose the bounded delta-pass variant of `mochiko:review-governance-intent` before treating it as ratified.
  - **Not ratified:** loop back to Phase 3/6; the run does not advance to authoring.

---

## Phase 8 — Author the governance surface set

- **Does:** Only now (post-ratification) is the actual surface set authored (`setup.surface-set`).
- **Producer seat:** `mochiko:tech-lead` agent, running `mochiko:authoring-constitution`.
- **Plan-approval gate (user/lead, `setup.plan-approval-producers`):** tech-lead proposes its authoring plan (which floor categories map to which files, module adoption from the interrogation's card rulings) before writing anything; I (lead) approve the plan.
- **Writes (only after plan approval):**
  - The marked governance region in `CLAUDE.md` between `<!-- mochiko:governance:begin/end -->` — idempotently regenerated, never touching content outside the markers (`setup.governance-region-ownership`).
  - `.claude/rules/mochiko/*.md`, `paths`-scoped per floor/module.
  - Skill pointers within those files.
  - `.mochiko/memory/governance-ledger.md`.
  - The trace summary (ratified GI-XXX rulings → each authored file/section).
  - If the knowledge-management module was adopted in Phase 3: `.mochiko/memory/knowledge-management.md`, scaffolded from `plugins/mochiko/templates/constitution-modules/knowledge-management.md` (`setup.km-module-scaffold`; never-overwrite floor applies going forward).
- **Carve-outs (`setup.carve-outs-preserved`):** since this is a first-ever write, there is nothing prior to preserve — the `mochiko:domain-registry` block and the `mochiko:output-style` pair are created fresh from the template defaults, not regenerated over anything.
- **Floor-category check:** every category in `plugins/mochiko/skills/authoring-constitution/references/ESSENTIAL-FLOOR.md` must land either a principle or a recorded waiver — an uncovered floor category is `setup.fail.floor-category-uncovered` and would block Phase 11.

---

## Phase 9 — Feature map scaffold (greenfield)

- **Does:** Scaffolds the empty capability index (`setup.feature-map-greenfield`) — no reconstruction needed since nothing is built.
- **Producer seat:** `mochiko:product-manager` agent, running `mochiko:authoring-feature-map`.
- **Plan-approval gate:** PM proposes the empty-scaffold plan (trivial, but still passes through `mochiko:patterns-sound-loop`'s no-size-gate rule — even an empty scaffold takes the review leg); I approve.
- **Writes:** Empty `FEATURES.md` index at repo root, shaped per `mochiko-cli template features-index` / `plugins/mochiko/schemas/features-index.yaml` if the binary is absent.
- **Never-overwrite floor (`setup.map-never-overwrite`):** applies going forward — this scaffold write is the one time it's created, never regenerated blind afterward.

---

## Phase 10 — Architecture store scaffold (unconditional)

- **Does:** Creates the store's birth scaffold on both greenfield and brownfield paths (`setup.store-scaffold-unconditional`) — scaffold only, never ruled content (`setup.store-ruled-content-never-here`).
- **Producer seat:** `mochiko:principal-architect` agent, running `mochiko:authoring-architecture-store` for the spine/header grammar.
- **Reads:** the ratified synthesis + interrogation record, to determine which surface types Cadence carries — based on the README ("A web app with a morning email digest"), the candidate `Scope:` declaration is **backend-service + frontend-web**, proposed to the user rather than assumed outright (`setup.architecture-scope-handoff` is explicit that this is a handoff, not a ruling).
- **Writes:**
  - `.mochiko/product/architecture/spine.md` — header-only stub carrying the `Scope:` line; no topology under it (greenfield).
  - `.mochiko/product/architecture/concerns.md` — empty, beside it.
- **Gate (user):** confirm or correct the proposed `Scope:` line before it's written; the user may also override it later at the `/mochiko:architecture` desk regardless of what's set here.

---

## Phase 11 — Independent grading (author ≠ grader, default FAIL)

- **Does:** The authored surface set from Phase 8 is graded by a seat that authored none of it (`setup.author-grader-default-fail`, `setup.fail.author-graded`). Grading reads the actual files, never the tech-lead's authoring report.
- **Grading seat:** `mochiko:validator` agent, running `mochiko:validation-constitution`.
- **Reads:** the actual `CLAUDE.md` governance region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, and the trace summary against the quality checklist.
- **Writes:** Nothing — returns PASS/FAIL with a concrete fix list; defaults FAIL if uncertain.
- **Branches:**
  - **PASS:** proceed to Phase 12.
  - **FAIL:** loop back to Phase 8 with the fix list; tech-lead revises, plan-approval repeats for the delta, re-grade. This loop continues until PASS or the user intervenes to redirect scope.

---

## Phase 12 — Version bump and trace closure

- **Does:** Bumps the governance region's semver and finalizes the trace summary showing every ratified GI-XXX ruling mapped to the file/section it produced, confirmed clean by Phase 11's independent grade (closes `setup.fail.unclosed-trace`).
- **Writes:** Version marker inside the governance region (part of the same Phase 8 files, updated in place); trace summary finalized in `governance-ledger.md`.
- **Seats:** tech-lead applies the bump as part of its authored surface; validator's PASS from Phase 11 is what licenses treating the trace as closed.

---

## Phase 13 — Final acceptance gate

- **Does:** Presents the complete package for acceptance, flagged proposal by flagged proposal (`setup.gate-final-acceptance`), as plain blocking text — never a timed prompt (`setup.acceptance-plain-text`).
- **Gate (user):** for each flagged item (module adoptions, waived floor categories, deferred gaps from Phase 6, the proposed architecture `Scope:` line) — accept, amend, or reject.
  - **Full acceptance:** run closes successfully; `setup.fail.no-acceptance` cleared.
  - **Partial acceptance / requested changes:** targeted rework loops back to Phase 8/9/10 for just the flagged items, then re-presents only the delta for acceptance (not a full re-run).
  - **Rejection:** run does not close; I'd surface what specifically was rejected and ask whether to revise or abandon the run.

---

## Phase 14 — Close-out reporting

- **Does:** Reports in the register defined by `plugins/mochiko/templates/output-style.md` (`setup.register`). Confirms all 6 fail-conditions are cleared:
  1. No surface authored pre-ratification ✓ (Phase 8 gated behind Phase 7)
  2. Trace closed ✓ (Phase 12)
  3. Author-graded avoided ✓ (Phase 11 used validator, not tech-lead)
  4. No uncovered floor category ✓ (checked in Phase 8/11)
  5. User acceptance given ✓ (Phase 13)
  6. Feature map present at close ✓ (Phase 9's empty scaffold)
- **Writes:** Nothing further — a suggested commit message is offered in text (`setup.no-git-mutations` — I would never run `git add`/`git commit`/`git push` myself, only suggest the command for the user to run).
- **Advisory next step (`setup.next-step`):** point to `/mochiko:specify` for Cadence's first feature and `/mochiko:architecture` for the product's architecture baseline, named as peer doors (neither implied to go first); plus `/mochiko:brainstorm` if knowledge-management was adopted in Phase 8.

---

### Durables and floors that hold across every phase (not phase-specific)
- `.mochiko/memory/governance-intent.md` and `.mochiko/memory/governance-ledger.md` are never offered for deletion once they exist (`setup.durables-never-deleted`).
- Everything outside the `<!-- mochiko:governance:begin/end -->` markers in `CLAUDE.md` is user content and is never touched by any phase above.
- Locate/enumerate-class reads (e.g., re-scanning the workspace tree, checking whether a file exists) would route to a native `Explore` subagent spawned with `model: haiku`; interpretive reads (evaluating README intent, grading synthesis coverage, judging trace closure) stay on the session tier — per `mochiko:patterns-model-tiering`. In this plan-only run, the initial workspace scan was done directly rather than via a spawned Explore subagent, since no dispatching was permitted.