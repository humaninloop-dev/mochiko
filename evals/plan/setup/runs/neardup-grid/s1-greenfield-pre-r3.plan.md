# Action Plan — `/mochiko:setup` (plan-only, not executed)

**Invocation context observed:** empty `$ARGUMENTS`. Workspace root holds `CLAUDE.md`, `README.md`, and the `plugins/mochiko/` plugin tree — no `.mochiko/` directory yet, no source code, no dependencies. `README.md` states explicitly: "Nothing is built yet... no source, no dependencies, no configuration." `CLAUDE.md` is working notes with no governance region markers. This is a **greenfield** signal — the mode would be proposed as greenfield, subject to user override at the mode gate.

---

## Phase 1 — Load binding rules (completed as part of planning)

- **Reads:** `plugins/mochiko/schemas/setup.yaml` (raw, full) and `plugins/mochiko/schemas/common.yaml` (raw, full) — both already read to produce this plan.
- **Writes:** none.
- **Result:** every `${var}` placeholder resolved (memory dir `.mochiko/memory`, product dir `.mochiko/product`, plugin root token), the six not-done conditions counted and confirmed as exactly six against the command file's hard-coded count (they match — no halt needed).

## Phase 2 — Workspace scan and mode proposal

- **Reads:** repo root listing, `CLAUDE.md`, `README.md`, presence/absence of `.mochiko/memory/governance-intent.md` and `.mochiko/memory/constitution.md`.
- **Findings used to propose mode:** no prior governance-intent file, no legacy constitution.md to retire, no delivered code to reconstruct a feature map from. Mode proposal: **greenfield**.
- **Writes:** none.
- **Gate:** the mode, whenever ambiguous, is the user's ruling, not the run's. Here it isn't ambiguous (workspace evidence points one way), so the lead states the proposed mode and asks for a quick confirm-or-correct rather than a heavy gate.
  - **If confirmed greenfield:** proceed to Phase 3 on the greenfield path.
  - **If user says brownfield instead** (e.g., "there's a private repo elsewhere / more exists than the notes show"): switch to the brownfield path — Phase 3's interrogation still runs, but Phase 4 additionally dispatches a codebase-analysis pass, and later phases pick up the brownfield-only obligations (feature-map reconstruction with entry-by-entry user confirmation, product-baseline bootstrap, conflict-ruling gate for detected-reality-vs-declared-intent mismatches).
  - **If user says amend instead** (there's governance to update, not create): later phases skip fresh scaffolding and instead look for what's missing and offer it rather than auto-creating it.

## Phase 3 — Inline interrogation (lead-run, no subagent)

- **Who:** the lead itself runs this, inline — not delegated to a seat.
- **Mechanism:** the adaptive-discovery skill drives dimension-by-dimension discovery of the project's real priorities, then a catalog deck is worked card by card, recommend-then-arbitrate.
- **Reads:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck (e.g. `universal-floor.md`, `backend-service.md`), `DOMAIN-DEPENDENCIES.md`. On a brownfield path, also `.mochiko/memory/codebase-analysis.md` (produced by a codebase-analysis dispatch first).
- **Skill involved:** `mochiko:analysis-iterative` (adaptive discovery), backed by `mochiko:authoring-constitution` reference material.
- **Writes:** none yet — this phase accumulates rulings in working memory, not on disk.
- **Gate (repeated, per card):** every card ruling, every module-adoption ruling, and every waiver is the user's — the lead recommends, the user decides. In this plan-only framing, each card-level gate branches as: **user accepts recommendation** → ruling recorded as-is; **user modifies** → ruling recorded with the user's edit; **user defers/skips** → card left open, revisited before synthesis is drafted.

## Phase 4 — Draft the synthesis

- **Writes:** `.mochiko/memory/governance-intent.md` (GI-XXX id namespace), rendered from the governance-intent template (or, if the `mochiko-cli` binary is unavailable, drafted directly against `plugins/mochiko/schemas/governance-intent.yaml` read raw as the fallback source of truth).
- **Content:** every ruling collected in Phase 3, structured as the durable amend baseline for future setup runs.
- **Gate:** none yet — this is authorship of the synthesis, not a surface. The floor is explicit that nothing beyond the synthesis may be authored before ratification.

## Phase 5 — Pre-ratification stress test (cold seat)

- **Who:** an independent seat — `mochiko:review-governance-intent` (or `mochiko:devils-advocate` acting in that capacity) — never the lead, since the lead wrote the synthesis.
- **Dispatch discipline (non-waivable):** two messages. First message: only the setup topic, project identity, and goal — never the path to `governance-intent.md` — so the seat builds a blind Phase-0 angle map of what coverage *should* look like before seeing what was actually concluded. Only after that map returns does the second message send the synthesis path and open the cold read.
- **Reads (by the spawned seat, second message onward):** `.mochiko/memory/governance-intent.md`.
- **Writes:** a findings/verdict artifact (in-conversation or a scratch report), not a governance surface.
- **Gate — user's ruling, or a recorded waiver:** the stress-test itself is mandatory unless the user explicitly waives it and that waiver is recorded. If waived, this phase is skipped but the waiver is logged as part of the trace.
- **Onward branch on findings:**
  - **No coverage gaps found, no material objections:** proceed to Phase 6.
  - **Coverage gap(s) survive the stress test:** each is presented to the user as a candidate topic, not silently folded in or silently dropped. The user rules the path per gap: (a) explore now — re-enter the interrogation skill on that angle, with the newly elicited intent landing in the same GI-XXX namespace and Phase 4 re-run for the affected slice; (b) rule inline — the user answers directly without a full re-interrogation pass; (c) defer — logged as open, synthesis proceeds without it. A gap that merely overlaps an already-worked agenda dimension follows the ordinary follow-up path within Phase 3 instead of this routing.

## Phase 6 — Ratification gate

- **Gate:** synthesis ratification is the user's alone. The lead presents the (possibly revised) synthesis and asks for an explicit plain-text accept — never a timed or implicit prompt.
  - **If ratified:** the synthesis is now frozen as the ratified baseline; Phase 7 may begin. This is the hard line — no governance surface may be authored before this point.
  - **If not ratified (user wants changes):** loop back to Phase 3/4 for the disputed portion, redraft, and re-present. The run does not proceed to authoring until this gate closes positively.

## Phase 7 — Author the governance surface set (producer seat, plan-approved)

- **Who:** a producer seat (e.g. `mochiko:tech-lead` acting through `mochiko:authoring-constitution`) — plans its authoring approach first and works only on a plan the lead has approved; this seat does not grade its own output.
- **Reads:** ratified `.mochiko/memory/governance-intent.md`, existing `CLAUDE.md` (to preserve everything outside the governance markers), any existing `.claude/rules/mochiko/*.md`, the governance-surfaces template's marked carve-out blocks (`mochiko:domain-registry`, `mochiko:output-style` pair) so they're preserved verbatim rather than regenerated over.
- **Writes:**
  - `CLAUDE.md` — the marked region between `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->`, idempotently regenerated; nothing outside those markers touched (this run's existing `CLAUDE.md` content — the working-notes prose — stays untouched).
  - `.claude/rules/mochiko/*.md` — path-scoped rule files, one of which carries the preserved `mochiko:domain-registry` block; another the `output-style.md` file carrying the preserved `mochiko:output-style` switch line pairing.
  - `.mochiko/memory/governance-ledger.md` — the durable ledger of what was decided and why.
  - A trace summary (ratified ruling → authored surface, for every ruling).
  - If a legacy `.mochiko/memory/constitution.md` were found on disk, it would be deleted on sight with a one-line note — not applicable here since none exists.
  - If knowledge-management was adopted as a module during interrogation: scaffold per `templates/constitution-modules/knowledge-management.md`, plus a project-pinned copy at `.mochiko/memory/knowledge-management.md`, which afterward is never overwritten by a later run.
  - Feature map: greenfield → scaffold an empty `FEATURES.md` index (never overwriting one that already carries content). Brownfield → reconstruct delivered capabilities from code into `FEATURES.md` + `.mochiko/features/` entries marked `delivered` and reconstructed-from-code, subject to Phase 8's confirmation gate. Amend → no feature-map write is made by this run at all; a missing map is only surfaced and offered.
  - Architecture store scaffold (unconditional on every path, creating only what's missing): `.mochiko/product/architecture/spine.md` as a header-only stub declaring the `Scope:` line (backend-service / frontend-web / mobile / desktop / composed), plus an empty `concerns.md` beside it. No architecture stance or ruled content is authored here — this is scaffold only; the first `/mochiko:architecture` visit is what actually elicits or reconstructs real content.
  - Greenfield only: product baselines (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md` under `.mochiko/product/`) are explicitly **not** written now — left to seed at the first implement run's design phase. (Brownfield would bootstrap them now from delivered code; not applicable on this greenfield path.)
  - Repo-root `ARCHITECTURE.md` is left as the store's derived index, not separately authored content.
- **Gate (brownfield-only branch, not applicable here but noted for completeness):** detected reality-vs-declared-intent conflicts, and the reconstructed feature map entry-by-entry, are each the user's ruling, confronted in the open — this phase would pause for both on that path.

## Phase 8 — Independent grading of the authored set

- **Who:** a seat that authored none of the surfaces (e.g. `mochiko:validator` or a second `mochiko:tech-lead` pass under `mochiko:validation-constitution`) — reads the files themselves, never the producer's self-report, and defaults to FAIL absent explicit pass evidence.
- **Reads:** the actual `CLAUDE.md` governance region, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-ledger.md`, the trace summary, cross-checked against the Essential Floor category list and the ratified synthesis.
- **Writes:** a grading verdict (pass/fail + fix list), not a governance surface.
- **Branch:**
  - **PASS (every floor category covered by either a principle or a recorded waiver, trace closes cleanly):** proceed to Phase 9.
  - **FAIL:** loop back to Phase 7 for the producer seat to close the named gaps, then re-grade. The run does not proceed to final acceptance on a FAIL verdict.

## Phase 9 — Final acceptance gate

- **Gate:** final acceptance of the whole surface set is the user's, presented flagged proposal by flagged proposal (not an all-or-nothing bulk accept), as plain blocking text — never a timed prompt.
  - **If accepted (in full or per-proposal):** the governance region's semver is bumped, the trace summary is handed to the user as the record of ratified-intent → authored-surface, and the run proceeds to close.
  - **If a specific flagged proposal is rejected:** that proposal is revised (loop to Phase 7 for just that surface) and re-presented; accepted proposals elsewhere are not re-litigated.

## Phase 10 — Close

- **Checks against the goal's done condition:** synthesis ratified before any authoring (Phase 6 gate satisfied before Phase 7 ran) · trace closes and was independently graded (Phase 8) · governance region semver bumped · user acceptance given (Phase 9) · feature map exists per mode (greenfield empty scaffold present) · architecture store `spine.md` stub with `Scope:` line present.
- **Reports:** a plain-language closing summary per the run's output-style template, plus the next-step routing: `/mochiko:specify` for the first feature and `/mochiko:architecture` for the product's architecture baseline, named as peer next steps (neither implied as first), plus `/mochiko:brainstorm` if knowledge-management was adopted.
- **Writes:** none beyond what Phases 7–9 already wrote.
- **Never in scope, any phase:** running or suggesting git mutations beyond a plain suggestion to commit (no actual git commands executed, no push); deleting `.mochiko/memory/governance-intent.md` or `.mochiko/memory/governance-ledger.md`; touching anything in `CLAUDE.md` outside the marked governance region; overwriting a feature map or architecture-store file that already carries real content.

## Not-done set — this run fails if any one of these holds at close

1. Any surface was authored before the synthesis was ratified (Phase 6 gate skipped or bypassed).
2. The trace from ratified intent to authored surfaces doesn't close across the set.
3. The governance surface set was graded by no one but its own author (Phase 8 skipped or self-graded).
4. Any Essential Floor category ends up with neither a principle nor a recorded waiver.
5. User acceptance was never given (Phase 9 gate never closed positively).
6. No feature map exists at close on the path that required one (greenfield scaffold missing, brownfield reconstruction unconfirmed, or an amend's missing map neither surfaced nor offered).