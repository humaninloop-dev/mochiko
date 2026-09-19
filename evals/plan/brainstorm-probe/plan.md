Read the schema, the command file, the two bound skills (`analysis-iterative`, `review-brainstorm`), the output-style register, and the current `.mochiko/brainstorms/index.md` state (one closed sample entry; `.mochiko/memory/knowledge-management.md` doesn't exist in this workdir, so the KM close ritual is conditionally skipped). No product architecture store, feature map, or prior repo-health/digest artifacts exist here — this is a thin/fresh workspace. Below is the full plan; nothing was written or spawned.

---

# Action Plan — `/mochiko:brainstorm whether mochiko should add a lightweight weekly repo-health digest`

## Phase 0 — Load the binding schema (done above)
- **Does:** Reads `plugins/mochiko/schemas/brainstorm.yaml` raw, in full, per the command's mandatory first action; confirms `fail-condition` count = 4 (matches the .md's hard-coded "4 rules" — no halt needed).
- **Reads:** `plugins/mochiko/schemas/brainstorm.yaml`, `plugins/mochiko/schemas/command-labels.yaml`.
- **Writes:** none.
- **Seats/skills:** lead only.
- **Gate:** none.

## Phase 1 — Entry & session bookkeeping
- **Does:** Topic is non-empty (`whether mochiko should add a lightweight weekly repo-health digest`), so no "what are we thinking through" fallback fires. Derive kebab-slug — proposed `repo-health-digest` (topic already scopes it to "weekly" and "lightweight," so those qualifiers are decision content, not slug bloat). Read `${index_path}` before opening (already done — one closed sample entry present, confirming the index format: `## <slug> — desc / When / Status / Artifacts`). Enter the session by adding an `open` entry for `repo-health-digest`.
- **Reads:** `.mochiko/brainstorms/index.md` (done).
- **Writes:** `.mochiko/brainstorms/index.md` — new entry, `Status: open`.
- **Seats/skills:** lead (`brainstorm.index-bookkeeping`).
- **Gate:** none — bookkeeping, not a ruling.

## Phase 2 — Inline adaptive questioning (mochiko:analysis-iterative)
- **Does:** Lead runs the questioning itself, one question per turn, format adapted to the user's state (`brainstorm.lead-inline-questioning`). Not delegated to a subagent — this is the lead's own inline loop, per the rule. For this specific topic, the question sequence would track things like:
  1. What "repo health" means here — build/CI status, stale PRs, dependency drift, test flake rate, lint/security findings, or some subset (open probe, not multiple-choice, since the topic is unsure-shaped at this stage).
  2. Audience — solo maintainer, team, or downstream users of the mochiko plugin.
  3. Delivery surface — a written artifact under `.mochiko/`, a chat digest on demand, a scheduled push (would touch the `schedule` skill / cron), or Slack/email (out of mochiko's current surface unless already integrated).
  4. Cadence trigger — literal weekly cron vs. "next time the user opens a session after 7 days."
  5. Data sources and cost — what it reads (git log, existing `.mochiko/memory/knowledge-management.md` caps, architecture-store drift probes, feature-map staleness) and whether that's cheap enough to justify "lightweight."
  6. Relationship to existing mochiko surfaces — does this duplicate `mochiko:grooming-operating-docs`' cap/bound sweeps, or sit above them as a rollup.
  7. Opt-in/off and where the toggle lives.
  8. MVP scope vs. deferred scope (e.g., v1 = local file digest; v2 = pushed notification).
  Each answer is converged into a `D1`, `D2`, … decision in `${record_path}`, each carrying statement + rationale + confidence (`Confident`/`Assumed`/`Contested`/`Unsure`/`Deferred`).
- **Reads:** user's live answers; any fact-finding dispatch during the conversation (e.g., "does mochiko already have a scheduled-digest mechanism") is routed per `mochiko:patterns-model-tiering`: locate/enumerate lookups → a native `Explore` subagent spawned with `model: haiku`; interpretive or absence-driven reads stay on the session tier. (In this plan-only run, no subagent is actually spawned — the equivalent lookups were done directly above via Glob/Grep.)
- **Writes:** `.mochiko/brainstorms/repo-health-digest/record.md` — written incrementally as the conversation progresses, never reconstructed at the end (`brainstorm.record-as-you-go`).
- **Seats/skills:** lead, pointer `mochiko:analysis-iterative`.
- **Gate:** none mid-conversation; the loop nudges toward convergence but never forces a premature synthesis.

## Phase 3 — Record freeze & review seat dispatch (mochiko:review-brainstorm)
- **Does:** Once decisions settle, the record is frozen. Because the record is the lead's own output, review is mandatory to a seat that is not the author (`brainstorm.record-review-independence`, `brainstorm.author-grader-default-fail`). Dispatch is two-message and blind: message 1 carries only the topic statement + goal line (never the record) so the reviewer builds its Phase 0 blind angle map with zero sight of what the session decided; message 2 (sent only after the angle map returns) carries `${record_path}` and opens the cold read (`brainstorm.blind-map-dispatch`). Solo vs. paired review is the lead's call (`brainstorm.staffing-latitude`) — for a single-topic, moderate-decision-count brainstorm like this, solo is proportionate; a pair would run both seats' Phase 0 maps independently (`brainstorm.pair-maps-independent`) before either sees the record.
- **Reads:** the reviewer reads `${record_path}` cold; applies the six hunt classes (unchallenged assumption, missing intra-decision dimension, passive acceptance, rejected-road steelman, inconsistency, excess machinery) plus scenario stress and the blind-map-vs-record coverage diff.
- **Writes:** none directly to the record — findings return as a message (survivors with severity, decision(s) touched, failure scenario, resolution path, tally, recommended status); the lead's pen is what ever edits the record.
- **Seats/skills:** one non-author review seat (e.g. spawned via Agent as a `mochiko:devils-advocate`-flavored reviewer running `mochiko:review-brainstorm`'s protocol), plus `mochiko:patterns-transport-floor` if a pair is used (message-race and shared-write discipline once more than one seat is composed) — **not actually spawned in this plan-only run.**
- **Gate — review waiver:** before dispatch, the lead would offer the user the option to skip review (`brainstorm.user-review-waiver`, reserved to the user).
  - *If user waives:* review is skipped; the waiver is recorded on `${record_path}` itself; synthesis (if later requested) would be stamped *derived, unchecked*; the run proceeds straight to Phase 5 (acceptance) — this keeps the run inside the fail-condition floor only because the waiver is recorded, not silently assumed.
  - *If user declines to waive (or doesn't answer, default-must):* proceed with the review seat dispatch as described.

## Phase 4 — Disposition of survivors
- **Does:** Two disposition paths, both user-reserved in the final call:
  - **Coverage survivors** (gaps the blind angle map found that the record never touched) are never auto-folded — each is presented as a candidate topic, and the user rules the path: explore now (re-enter the Phase-2 questioning loop on that angle, landing the result in the same `D…` namespace), rule inline, or defer (`brainstorm.coverage-survivor-routing`). A decision born from a reopen gets one bounded verify round only — internal consistency and record-fitness, no fresh cold read, no second reopen (`brainstorm.reopen-born-verify`).
  - **Non-coverage survivors** (inconsistencies, unchallenged assumptions, excess machinery, etc.) follow the ordinary fold/repair/ruling path and may be batched (`brainstorm.non-coverage-survivors`).
  - Any survivor that challenges a ruling the user already made is escalated to the user, never resolved unilaterally (`brainstorm.user-survivor-challenge`).
- **Reads:** the survivor report from Phase 3.
- **Writes:** `${record_path}` — folds, repairs, or new/amended `D…` entries land here, always through the lead's pen.
- **Seats/skills:** lead; re-entry into `mochiko:analysis-iterative` only for a coverage-survivor "explore now" branch.
- **Gate — per survivor, batched where possible:**
  - *Coverage survivor, user picks "explore now":* loop back into Phase 2's engine on that angle → new decision → one bounded verify round → folded back in.
  - *Coverage survivor, user picks "rule inline":* user's ruling is recorded directly as the disposition, no reopen.
  - *Coverage survivor, user picks "defer":* recorded as deferred (with `Deferred` confidence where it produces a decision, or a noted open question if it doesn't).
  - *Non-coverage survivor, user accepts the fold/repair:* applied to the record.
  - *Non-coverage survivor, user rejects the finding:* recorded as dismissed-with-reason (a dismissal is itself a ruling, not silence).
  - Every survivor must land in one of these dispositions — an undispositioned survivor is fail-condition `brainstorm.fail.survivor-undispositioned`.

## Phase 5 — Record acceptance
- **Does:** Presents the (possibly amended) record for acceptance. Acceptance is plain blocking text, never a timed prompt (`brainstorm.acceptance-plain-text`), and is reserved to the user — never the lead's own call (`brainstorm.user-record-acceptance`, floor-class).
- **Reads:** final `${record_path}`.
- **Writes:** none yet (acceptance itself is a user statement, not a file mutation).
- **Seats/skills:** lead presents; user rules.
- **Gate — acceptance:**
  - *User accepts:* proceed to Phase 6.
  - *User requests changes:* loop back into Phase 2/4 (amendment to a user-ruled decision, or any new decision, is itself reserved to the user — `brainstorm.user-pen-boundary`), then re-present for acceptance.
  - *User rejects outright / abandons:* the run does not close as done — an unaccepted record is fail-condition `brainstorm.fail.record-unaccepted`; the session would be left `open` in the index (or explicitly marked abandoned, per user instruction) rather than falsely marked accepted.

## Phase 6 — Landing: index update, KM ritual, next-step offer
- **Does:**
  - Updates `.mochiko/brainstorms/index.md` at acceptance (or supersession) with where the outcome landed, resolving the `open` entry from Phase 1 (`brainstorm.index-bookkeeping`; avoids fail-condition `brainstorm.fail.index-mismatch`).
  - KM close ritual: `${km_path}` = `.mochiko/memory/knowledge-management.md` does **not exist** in this workspace (confirmed by the Phase-0 Glob), so `brainstorm.km-close-ritual` is a no-op this run — nothing to run.
  - Offers a synthesis document only if the user requests one, stamped *derived — record canonical* (or *derived, unchecked* if Phase 3 was waived) (`brainstorm.synthesis-on-request`).
  - Offers pipeline entry (e.g. `/mochiko:specify`, since "should mochiko add X" often reads as a feature description once decided) strictly as a post-acceptance offer, never a default (`brainstorm.next-step-offer`).
  - Suggests a git commit for the new/updated files; never runs git mutations or pushes itself (`brainstorm.no-git-mutations`) — this workdir is also not a git repository, which would be surfaced to the user as a reason a suggested commit can't be executed here without `git init` first.
- **Reads:** `.mochiko/brainstorms/index.md`.
- **Writes:** `.mochiko/brainstorms/index.md` (status update), optionally `.mochiko/brainstorms/repo-health-digest/synthesis.md` (only on request).
- **Seats/skills:** lead; register for all user-facing prose follows `plugins/mochiko/templates/output-style.md` (`brainstorm.register`) — chat at `full` by default (no governance region exists yet in this workdir to override it).
- **Gate — synthesis request:**
  - *User asks for a synthesis:* write it, stamped per the review outcome.
  - *User doesn't ask:* skip — synthesis is never produced by default.
- **Gate — next-step offer:**
  - *User wants to proceed to `/mochiko:specify` (or another pipeline entry):* lead states that as the next command, does not invoke it itself within this run.
  - *User declines or is undecided:* the brainstorm closes here; no further action taken.

## Phase 7 — Done-condition check (default FAIL)
- **Does:** Before declaring the run complete, checks all 4 fail-conditions are clear: record accepted (Phase 5) · reviewed-or-waived (Phase 3 gate) · every survivor dispositioned (Phase 4) · index entry matches the record's final status (Phase 6). Any one still standing means the run is **not done**, regardless of how much conversation occurred.
- **Reads:** the final state of `${record_path}` and `${index_path}`.
- **Writes:** none (verification only).
- **Seats/skills:** lead.
- **Gate:** none — this is the lead's own closing check, not a user ruling.