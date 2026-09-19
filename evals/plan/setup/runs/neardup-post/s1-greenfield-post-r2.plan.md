## Action Plan — `/mochiko:setup` on the Cadence repo (plan-only, nothing executed)

**Workspace read already performed to ground this plan:** `plugins/mochiko/schemas/setup.yaml` and `plugins/mochiko/schemas/common.yaml` (the run's binding rules, read raw per the command's first-action instruction); `CLAUDE.md` (working notes only, no `<!-- mochiko:governance:begin/end -->` region present); `README.md` (Cadence — an async standup digest product, "nothing is built yet... no source, no dependencies, no configuration"); globs for `.mochiko/`, `.claude/rules/mochiko/`, `FEATURES.md`, `ARCHITECTURE.md`, `package.json`, and any source files — all absent except the `plugins/mochiko/` plugin tree itself. No prior setup run has touched this repo.

---

### Phase 0 — Load the schema (done)
Read `setup.yaml` and `common.yaml` raw and in full before any other action, so every later step is checked against this run's actual rules rather than assumption. Confirms the "not-done" fail set has exactly six entries (pre-ratification authoring, unclosed trace, author-graded output, an uncovered floor category, no acceptance, no feature map) — count matches, so the run is not halted for drift. Nothing written.

### Phase 1 — Propose the run's mode
**Read:** `CLAUDE.md`, `README.md`, directory listing for source/config/dependency files.
**Finding to present:** no source code, no dependency manifest, no prior `.mochiko/` state — README says the repo was "initialized this week" and holds only notes. This reads as **greenfield**.
**Write:** none.
**Gate — mode confirmation (reserved to the user):** present "greenfield" as the proposed mode with the evidence above.
- *If the user confirms greenfield* → proceed to Phase 2 on the greenfield path.
- *If the user instead rules brownfield* (e.g., they know of code not visible here, or intend to codify external practice) → branch to the brownfield path: invoke the codebase-analysis skill first, producing `.mochiko/memory/codebase-analysis.md`, and every brownfield-only obligation below (feature-map reconstruction, conflict rulings, map confirmation) activates.
- *If the user rules amend* (there is governance to update, not create) → skip interrogation-from-zero; locate the existing synthesis and surface set instead, and treat this as a targeted amendment pass over Phases 3–10.
This plan continues on the **greenfield branch**, since that's what the workspace evidence supports.

### Phase 2 — Assemble interrogation inputs
**Read:** `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md`, the `catalog/` deck (currently seeded for universal-floor and backend-service; frontend/mobile/desktop shelves are planned gaps to be honest about if Cadence's shape needs them), `DOMAIN-DEPENDENCIES.md`.
**Write:** none yet.
No codebase analysis on this branch (greenfield has no code to analyze).

### Phase 3 — Run the interrogation inline
Conducted by the run lead directly (never delegated to a subagent), one question at a time, adaptively across the ten agenda dimensions — project identity (an async standup digest, two part-time engineers, six-person internal pilot first), fact profile, project type & shape (web app + email digest → likely a full-stack surface scope), risk surface, team reality (small, part-time — enforcement must fit that), existing practices (none yet), a knowledge-management module offer (default-on, the user must actively decline), deployment & release reality, values/non-negotiables, and deliberate exclusions. Includes the single project-wide depth-level dial (`low`/`high`), offered recommend-then-arbitrate, and the catalog deck dealt card-by-card once the selecting dimensions are settled.
**Write:** `.mochiko/memory/governance-intent.md` (the synthesis, GI-XXX-numbered), drafted from the session.
**Reserved to the user throughout:** every card ruling, module ruling, waiver, and the depth-level declaration — none of these are ever decided by the run itself.

### Phase 4 — Size and dispatch the pre-ratification stress test
Before ratification, the synthesis must face a cold reviewer who never sat in the interrogation — unless the user records a waiver.
**Gate — cold-seat or waiver:** ask whether to run the independent stress test (default) or accept a recorded waiver of it.
- *Default (run it)* → decide single reviewer or a pair, spawn blind: first message carries only the setup topic and project identity/goal (never the synthesis path), so the reviewer's angle map forms independent of what interrogation concluded; the synthesis path is sent only after that map returns. Spawning a second seat here makes this a multi-seat run, which activates the messaging/write-collision floor for every exchange with that seat from this point forward.
- *User waives it* → record the waiver in the synthesis and skip to Phase 7, carrying the gap forward as an accepted risk.
**Write:** none besides the waiver record if that branch is taken.

### Phase 5 — Cold review executes, findings return
The reviewer(s) read the frozen synthesis, the agenda (as the coverage yardstick), and produce a severity-classified survivor report with a recommended status (ready / needs-revision / critical-gaps) — sent to the lead, never authored into the artifact directly.
**Gate — coverage-survivor routing (reserved to the user):** any surviving coverage finding questions the run's *scope*, not a single card. For each one, present it as a candidate topic and the user rules:
- *Explore now* → re-enter the adaptive interrogation on that angle; newly elicited intent lands under a new GI-XXX entry.
- *Rule inline* → the user answers directly without a full re-elicitation pass.
- *Defer* → recorded as a deliberate exclusion.
A finding that just overlaps an already-covered agenda dimension follows the ordinary interrogation follow-up path instead of this gate.

### Phase 6 — Disposition and update the synthesis
Fold every ruling from Phase 5 back into `.mochiko/memory/governance-intent.md`. No new file; this is a revision of the Phase-3 write.

### Phase 7 — Ratification gate
**Gate (reserved to the user, non-waivable):** the user ratifies the synthesis. This must happen before any surface is authored — authoring first is an automatic run failure.
- *Ratified* → proceed to Phase 8.
- *Not ratified / wants changes* → loop back into Phase 3 (further interrogation) or Phase 6 (re-disposition), then return here. No surface work starts in the meantime.

### Phase 8 — Author the governance surface set
A producer seat (plans first, works only on a lead-approved plan) formulates from the ratified synthesis — selection stays the synthesis's, wording and surface placement are this step's job.
**Write:**
- The `CLAUDE.md` governance region between the `mochiko:governance` markers (created fresh here since none exists) — ratified stamp, principle index, universal principles as short imperative lines, tech-stack/quality-gate summary, module pointers. Everything outside the markers (the existing "Cadence — working notes" content) is left untouched.
- `.claude/rules/mochiko/*.md` — one scope-bound concern per file, `paths`-scoped, only if any principle is scope-bound rather than universal.
- `.mochiko/memory/governance-ledger.md` — Three-Part records per principle, floor + module accounting, waivers.
- The trace summary manifest (every synthesis element mapped to its authored home).
- If the knowledge-management module was adopted in Phase 3: `.mochiko/memory/knowledge-management.md` plus the core bundle scaffold.
- Unconditionally, regardless of what else was ruled: `.mochiko/product/architecture/spine.md` as a header-only stub carrying a `Scope:` line (declaring which surface types — e.g. backend-service + frontend-web for Cadence's shape, as settled in Phase 3) and an empty `concerns.md` beside it — scaffold only, no architectural stances taken here.
- `FEATURES.md` — an empty index (greenfield scaffold; no entries to reconstruct since nothing is built).
- Product baselines (`data-model.md`, `contracts/`, `constraints-and-decisions.md`, `quickstart.md`) are **not** written on this branch — greenfield defers them to the first `/mochiko:implement` run's design phase.
- If a stale `.mochiko/memory/constitution.md` were found on disk it would be deleted here with a one-line note — none exists in this repo.

### Phase 9 — Independent grading
A seat that authored none of Phase 8's output reads the actual files (never the producer's report) against the validation checklist — enforceability, trace closure (index → home → ledger), floor/module accounting, anti-pattern scan (vague language, excess governance), and determines the semver bump (MAJOR/MINOR/PATCH). Defaults to FAIL until every item is confirmed from the files.
- *FAIL* → fix list returns to Phase 8's producer for revision; re-grade after. Repeats until PASS. The same seat never clears its own work.
- *PASS* → proceed to Phase 10.

### Phase 10 — Final acceptance gate
**Gate (reserved to the user, non-waivable, plain blocking text — never a timed prompt):** present the surface set for acceptance, flagged proposal by flagged proposal (any place authoring had to flag something the synthesis didn't cleanly cover, e.g. intent that resisted enforceable wording).
- *Accept as-is* → proceed to close-out.
- *Accept with edits* → apply the edits, then confirm the final state was accepted.
- *Reject a flagged proposal* → lead dispositions it (drop it, or rework it) — a rework that touches selection loops back to Phase 3/7 rather than being decided inside authoring.

### Phase 11 — Close-out and report
Apply the version bump from Phase 9's determination to the governance region's semver. Report in the project's configured register (`templates/output-style.md`) summarizing: mode taken (greenfield), the trace from ratified intent to every authored surface, the grading verdict, and confirmation that all six not-done conditions are clear (ratified-before-authored, trace closed, independently graded, no floor category left uncovered/unwaived, user acceptance given, feature map present at close — here the empty greenfield scaffold). Suggest a commit message covering the new/changed files — never run the commit or push it. Point to next steps as peer doors: `/mochiko:specify` for Cadence's first feature, `/mochiko:architecture` for the product's architecture baseline (plus `/mochiko:brainstorm` if knowledge-management was adopted).