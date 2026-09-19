# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Workspace read so far (informs the plan, changes nothing):** no `.mochiko/` directory, no `.claude/rules/mochiko/`, `CLAUDE.md` has no governance-region markers (it's a plain working-notes file), no `constitution.md` to supersede. `README.md` states "Nothing is built yet... no source, no dependencies, no configuration" — a web app (email digest) for distributed teams, two part-time engineers, first pilot an internal team of six. This is a greenfield workspace with no ambiguity signal toward brownfield.

Schema `plugins/mochiko/schemas/setup.yaml` was read raw and in full per the command's binding first action; its `setup.sec.fail-conditions` block contains exactly 6 `fail-condition` rules, matching the Not-done count declared in the command — no halt needed.

---

## Phase 0 — Schema load (complete)
- **Does:** Read `plugins/mochiko/schemas/setup.yaml` raw, in full; verified 6 `fail-condition` rules under `setup.sec.fail-conditions`.
- **Reads:** the schema file itself; `plugins/mochiko/schemas/command-labels.yaml` for label meanings.
- **Writes:** none.
- **Gate:** none (a rule not yet read would keep the run closed — already satisfied).

## Phase 1 — Mode proposal
- **Does:** Propose mode from workspace evidence: **greenfield** (no code, no prior `.mochiko/` artifacts, README self-declares "nothing built yet").
- **Reads:** `CLAUDE.md`, `README.md`, directory listing (already done above).
- **Writes:** none yet.
- **Gate (`setup.user-mode-ruling`, reserved to the user):** present the greenfield read and ask the user to confirm or override.
  - *If confirmed greenfield* → proceed to Phase 2 as planned.
  - *If user rules brownfield* → pivot: invoke `mochiko:analysis-codebase` to produce `.mochiko/memory/codebase-analysis.md` before interrogation; downstream phases pick up the brownfield branches noted inline below (feature-map reconstruction, baseline bootstrap, Essential-Floor present/partial/absent assessment, Emergent Ceiling, `evolution-notes` module).
  - *If user rules amend* → this is not a first-time setup; branch to reading the existing `governance-intent.md` as the amend baseline and re-entering only the dimensions the user flags — the rest of this plan (fresh interrogation, fresh scaffold) does not apply as written.
  - Rest of this plan assumes the confirmed-greenfield branch, since that's what the evidence shows.

## Phase 2 — Inline interrogation session
- **Does:** I run the interrogation myself, inline (`setup.interrogation-inline`), working the ten agenda dimensions adaptively via `mochiko:analysis-iterative` — one question per turn, skipping what's already settled (e.g., dimension 1 "what's being built, for whom" is already answered by README; I'd confirm rather than re-ask), probing where vague (team reality, deployment cadence, values/non-negotiables are not yet stated).
- **Reads:** `${plugin_root}/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, `DOMAIN-DEPENDENCIES.md`; locate/enumerate-class reads (e.g., re-scanning the tree for any stack signal I might have missed) dispatch to a native `Explore` subagent spawned with `model: haiku` per `mochiko:patterns-model-tiering`; interpretive judgment (e.g., weighing a vague answer) stays on the session tier.
- **Writes:** none yet — dimension answers accumulate toward the synthesis, not committed to disk mid-session.
- **Gates along the way (all reserved to the user, `setup.user-card-rulings`):**
  - **Depth-level declaration** (dimension 2/agenda's one strictness dial) — recommend-then-arbitrate `low`/`high`; user rules.
  - **Dimension 7 — knowledge-management module** — offered default-on (core whole, electives `CHANGELOG.md`/`RUNBOOK.md` per-doc). *If accepted* → Phase 6 scaffolds it. *If declined* → recorded as a dimension-10 exclusion, no scaffold.
  - **Dimension 8 — release-gates module** — always interrogated for a deployed product; user rules adopt or decline, recorded either way.
  - **Catalog card arbitration** — deck dealt from `references/catalog/`: `universal-floor.md` (every project) plus `backend-service.md` (this is a fullstack web app, so the backend/API side is dealt; `frontend.md` is a named gap in the catalog — "planned, not seeded" — so frontend-shaped principles would be minted from elicited intent rather than dealt from a card, and I'd say so plainly rather than papering over the gap). User keeps/tightens/drops/re-ranks each arbitrated card.
  - **Dimension 9 — values & non-negotiables**, phrased as enforceable behavior ("what should CI or review block?"), user's answers are the primary source for minted principles.
  - **Dimension 10 — deliberate exclusions**, user's ruling on what governance will *not* cover.

## Phase 3 — Synthesis write
- **Does:** Render the session synthesis.
- **Writes:** `.mochiko/memory/governance-intent.md` (GI-XXX namespace), via `mochiko-cli template governance-intent` if the binary is available, else by reading `plugins/mochiko/schemas/governance-intent.yaml` raw and shaping the file to that schema directly.
- **Reads:** the accumulated dimension answers and card rulings from Phase 2.
- **Gate:** none yet — this is the pre-ratification draft, not yet frozen.
- **Note:** this file, once it exists, is durable — never offered for deletion (`setup.durables-never-deleted`), regardless of what happens later in the run.

## Phase 4 — Pre-ratification stress-test (cold seat)
- **Does:** Spawn an independent cold seat (via `mochiko:review-governance-intent`) to stress-test the frozen synthesis *before* the user ratifies it. Two-message blind dispatch (`setup.blind-map-dispatch`, non-waivable floor): first message gives only the setup topic / project identity and goal (Cadence: async standup digest app) — **not** the path to `governance-intent.md` — so the seat builds its Phase 0 angle map blind to what the session actually concluded; only after that map returns do I send the synthesis path and let the cold read begin.
- **Seats:** one cold reviewer seat (agent type `mochiko:review-governance-intent`-invoking), independent of me as session lead. Since this makes the run multi-seat, `mochiko:patterns-transport-floor` governs the messaging leg for this dispatch (non-waivable once triggered).
- **Reads (by the cold seat):** `.mochiko/memory/governance-intent.md` only after its blind map is built.
- **Writes:** the seat's findings (coverage gaps, contradictions) — not committed to a project file, surfaced to me and then to the user.
- **Gate (`setup.stress-test-cold-seat`):** the stress-test itself is mandatory unless the user records an explicit waiver of it.
  - *If user waives* → skip straight to Phase 5's ratification gate with the waiver recorded in the synthesis.
  - *If run* → proceed to Phase 4a below.

### Phase 4a — Coverage-survivor routing (reserved to the user)
- **Gate (`setup.coverage-survivor-routing`):** any coverage finding that survives the stress-test is presented as a candidate topic, never silently folded in or silently dropped. For each surviving gap the user rules one of:
  - **Explore now** — re-enter `mochiko:analysis-iterative` on that angle; the newly elicited intent lands in the GI-XXX namespace (loops back into Phase 2/3 for that slice only).
  - **Rule inline** — user answers directly without a full re-elicitation pass.
  - **Defer** — recorded as an open item, not resolved this run.
  - (A gap that overlaps an already-agenda'd dimension instead takes the ordinary interrogation-follow-up path, not this routing.)

## Phase 5 — Synthesis ratification
- **Gate (`setup.gate-synthesis-ratification`, floor, reserved to the user):** the user ratifies the (possibly revised) synthesis. This is the hard line for `setup.fail.pre-ratification-authoring` — no surface authoring below this line may start before this gate clears.
  - *If ratified* → Phase 6 begins.
  - *If user asks for changes* → loop back into Phase 2/3 for the disputed dimension(s); re-offer ratification.

## Phase 6 — Governance surface authoring (producer seat)
- **Does:** A producer seat runs `mochiko:authoring-constitution` against the ratified synthesis. Since this is a producer that writes artifacts, it plans first and works only on a plan I approve (`setup.plan-approval-producers`) — so this phase itself has an internal plan-approval gate before any file is touched.
- **Seats:** one producer seat (distinct from the grading seat in Phase 8 — author ≠ grader is enforced, `setup.author-grader-default-fail`).
- **Reads:** `references/catalog/` (universal-floor.md, backend-service.md), `references/ESSENTIAL-FLOOR.md`, `references/RFC-2119-KEYWORDS.md`, `references/COMPLIANCE-MODULES.md`, the ratified `governance-intent.md`; existing `CLAUDE.md` content outside the (nonexistent, so far) governance markers, to leave user content untouched.
- **Writes (greenfield core, per `setup.surface-set`):**
  - The governance region in `CLAUDE.md`, freshly inserted between `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->` (idempotently regenerated on future runs; everything outside the markers stays untouched).
  - `.claude/rules/mochiko/*.md` — one file per scope-bound concern, each with a `paths` frontmatter glob.
  - Skill pointers for procedure-shaped standards.
  - `.mochiko/memory/governance-ledger.md` — Three-Part records per GI-ID, floor + attached modules, waivers, amendment policy.
  - The trace summary (GI element → surface home → ledger entry, closing the trace).
  - If `${memory_dir}/constitution.md` existed it would be deleted on sight with one line said about it — not applicable here since it's absent.
  - If knowledge-management was adopted in Phase 2 → scaffold per `templates/constitution-modules/knowledge-management.md`, including the project-pinned copy at `.mochiko/memory/knowledge-management.md` (never-overwrite floor applies from here on).
  - Two preserved carve-outs written at defaults since this is a first authoring, not a regeneration: the `mochiko:domain-registry` block and the `mochiko:output-style` switch line (default-on) plus `.claude/rules/mochiko/output-style.md`.
- **Boundary respected:** the architecture store's *ruled content* is never authored here (`setup.store-ruled-content-never-here`) — only its scaffold, handled separately in Phase 7.

## Phase 7 — Feature-map and architecture-store scaffolds
- **Does (feature map, greenfield):** scaffold an empty `FEATURES.md` index (`setup.feature-map-greenfield`) — no `.mochiko/features/` entries yet, since nothing is delivered. *(If Phase 1's gate had instead landed on brownfield, this step would instead reconstruct delivered capabilities from routes/UI/services in the code, land them as `FEATURES.md` + `.mochiko/features/` entries marked `delivered` + reconstructed-from-code, and require entry-by-entry user confirmation — `setup.user-map-confirmation` — before counting as done.)*
- **Does (architecture store, unconditional on both paths):** create `.mochiko/product/architecture/` with a `spine.md` stub whose header carries a `Scope:` line declaring the product's surface types (here: at minimum `backend-service` + `frontend-web`, composed as full-stack, based on README's "web app with a morning email digest"), plus an empty `concerns.md` beside it. No topology content under the stub — that's ruled content reserved for the first `/mochiko:architecture` visit, not this run.
- **Writes:** `FEATURES.md` (empty index), `.mochiko/product/architecture/spine.md` (header + `Scope:` line only), `.mochiko/product/architecture/concerns.md` (empty).
- **Not written here:** product baselines (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`) — greenfield defers these to the first `/mochiko:implement` run's design phase (`setup.baselines-bootstrap`). *(Brownfield would instead bootstrap these now from the delivered code.)*
- **Gate:** none standalone here — this scaffold folds into the same final-acceptance gate as Phase 6's output (Phase 9).

## Phase 8 — Independent grading
- **Does:** An independent validator (never the Phase 6 producer) runs `mochiko:validation-constitution` against the authored files themselves — never against the producer's self-report — defaulting to FAIL absent explicit confirmation.
- **Seats:** one grading seat, distinct from the producer seat. Multi-seat composition again triggers `mochiko:patterns-transport-floor` for any shared-write or messaging legs.
- **Reads:** the actual `CLAUDE.md` governance region, `.claude/rules/mochiko/*.md`, `governance-ledger.md`, the trace summary — the checklist graded is quality + trace-closure + floor-category coverage.
- **Writes:** a findings report (PASS or a concrete fix list) — not a project artifact, relayed to me and the user.
- **Gate (`setup.author-grader-default-fail`, floor):** no output clears without this independent read.
  - *If PASS* → Phase 9.
  - *If FAIL* → loop back to Phase 6 with the fix list, producer revises, re-grade. This loop can repeat; nothing proceeds to acceptance while any grading finding is open.

## Phase 9 — Final acceptance (reserved to the user)
- **Does:** Present the trace summary and any flagged proposals (unenforceable elicited intent, contradictions found during authoring) individually.
- **Gate (`setup.gate-final-acceptance`, floor):** the user accepts the surface set **flagged proposal by flagged proposal** — plain blocking text, never a timed prompt (`setup.acceptance-plain-text`).
  - *If accepted as-is* → proceed to Phase 10.
  - *If user rules on flagged proposals (accept/reject/modify each)* → apply the ruling, possibly a short revision loop back through Phase 6/8 for just the affected surfaces, then re-present for acceptance.

## Phase 10 — Close-out
- **Does:** Bump the governance region's semver stamp; suggest (never execute) a git commit summarizing the new governance surfaces — `setup.no-git-mutations` means I would propose a commit message, not run `git commit`; report using `templates/output-style.md` register (`setup.register`); state the next-step pointer (`/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline, presented as peer doors, plus `/mochiko:brainstorm` if knowledge-management was adopted) — advisory, not a gate.
- **Writes:** the semver bump lands inside the already-authored `CLAUDE.md` governance region (no new file).
- **Final self-check before declaring done:** walk the 6 fail-conditions from `setup.sec.fail-conditions` against what actually happened — pre-ratification authoring (none occurred, Phase 5 gated it), unclosed trace (Phase 6 emitted it, Phase 8 graded it), author-graded-by-author (Phase 8 used a distinct seat), floor-category uncovered without principle or waiver (Phase 8's checklist covers this), no acceptance (Phase 9 gated it), no feature map at close (Phase 7 scaffolded the empty index and this is greenfield, so "unscaffolded" is the only failure shape to check, and it wasn't). Any one of these still standing would fail the run regardless of how far the phases got.