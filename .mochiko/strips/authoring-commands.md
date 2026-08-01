# Strip notes — `skills/authoring-commands/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25).

## [v0.44.0] KEPT: the strip-note write instruction (`:82-83`) — operative, not provenance
- **Tier-2 evidence:** *"**Log every outcome** per the entry formats in `.mochiko/strips/README.md`
  — one note per primitive at `.mochiko/strips/<primitive>.md`, **repo-side, never under
  `plugins/`**"* survived the D7 full scrub **deliberately**. It is the instruction that tells an
  author where to write the note and where never to write it — the second half is itself the guard
  against the defect the strips README calls fix-on-sight (*"a future wave that writes a strip note
  anywhere under `plugins/` is a defect"*). Scrubbing it would delete the skill's own working
  procedure, not a citation.
- **The rule this establishes, for the next sweep:** D7's classes are **provenance and history**.
  A path that is *cited* (where a claim came from) scrubs; a path that is *used* (where the skill
  performs its function) stays. Lead-ruled 2026-08-01 after the enumeration surfaced the collision.
- **Exits with the move:** this skill leaves the shipped tree at the deferred D6 trio move, taking
  the pointer with it — at which point it is repo-side text pointing at a repo-side home, and the
  leak question closes for good.

## [v0.44.0] DEFERRED: the D6 trio move — prep notes, not executed
- **Disposition:** none yet. Recorded here so the next session does not re-derive it; the lead
  carries the remainder to BACKLOG. **Gate:** the S14 probe failed on the agent half in-session
  (spawn accepted, agent instantiated generic, `mochiko-probe` absent from the available types) —
  repo-side agents do not load mid-session, so the probe must be re-run in a **fresh session**
  (spawn `mochiko-probe` as a teammate + confirm the repo-side skill fires on its trigger) before
  any file moves. The probe pair stays on disk as that instrument:
  `.claude/skills/mochiko-probe/SKILL.md` + `.claude/agents/mochiko-probe.md`.
- **Move set (3 single files; neither skill has a `references/` subdirectory):**
  - `plugins/mochiko/agents/command-architect.md` → `.claude/agents/command-architect.md`
  - `plugins/mochiko/skills/authoring-commands/SKILL.md` → `.claude/skills/authoring-commands/SKILL.md`
  - `plugins/mochiko/skills/validation-command-shape/SKILL.md` → `.claude/skills/validation-command-shape/SKILL.md`
- **Shipped-tree edits owed at the move:**
  - `skills/mochiko/SKILL.md` — delete `### Framework maintenance` whole (heading + table header +
    both rows) and the `command-architect` row in the Agents table; **also** `:28`, which names
    `validation-command-shape` as a live example of the `validation-*` family.
  - **Ruled, not overlooked:** the `authoring-commands` row in that section (`:118`) still names
    `.mochiko/strips/` — *"running a strip wave with version-stamped notes (`.mochiko/strips/`,
    repo-side)"*. It survived the v0.44.0 D7 scrub **by lead ruling**: it is neither cited
    provenance nor an operative path, but a **description** of what the skill does, and cutting the
    path would half-describe the skill for its final shipped version. One descriptive mention is
    the same accepted cost as shipping the trio itself one more version. It dies with the section
    at this move — an auditor reading the untouched line should read it here, not as a miss.
  - `agents/validator.md` — `:27` `skills: validation-constitution, validation-command-shape` drops
    the second name, and the `:44-47` descriptive bullet goes. Needs a `.mochiko/strips/validator.md`
    supersession entry when done. **Deliberately NOT done at v0.44.0** (lead ruling): the skill
    still ships this version, so the declaration is accurate today and dropping it now would be the
    premature edit — it lands *with* the move.
  - **Finding, recorded so the record's rationale is not silently wrong next session:** D6 rests on
    F38/F39, read as *"the trio's consumers are exclusively mochiko's own command authoring."* That
    is **not exact** — the shipped generic `validator` agent is a fourth consumer, mounting
    `validation-command-shape` in its own frontmatter. The consequence is cosmetic in mechanism
    (Layer 2: teammates do not load `skills:` frontmatter; every spawn names its skill) but real as
    a **declaration**: unmoved, it leaves a shipped agent claiming a capability adopters no longer
    have. It does not overturn D6 — the fix is two one-line edits — but the rationale should be read
    as "exclusively, except the generic validator's declared mount."
  - `.claude-plugin/plugin.json` — the `agents` array goes 10 → 9 entries, dropping
    `"./agents/command-architect.md"`. Lead's close-ceremony edit.
- **Repo-side edits owed:** this repo's `CLAUDE.md` gains the trio's new home and reach-for
  guidance (S14's discoverability replacement for the router rows that leave) · the `mochiko:`
  prefix drops from every reference to the three.
- **Already done, so the move does not owe it:** the P9 lockstep re-key of
  `validation-command-shape` landed **in place** this wave (entry in that primitive's note).
- **Open sub-question for the move (see the wave report):** the trio's own five class-1 pointers.
  Two of them — `authoring-commands` `:82-83` — are **operative**, not provenance: they tell the
  author where to write the note. Scrubbing those would break the skill's function.

## [v0.38.0] Two stale pointers in the Overview corrected (shape v6 wave)

- **Disposition:** corrected in place (correction class) — two clauses, one sentence, no
  doctrine added or removed. Wave note: `.mochiko/strips/command-shape.md` v0.38.0.
- **Tier failed:** n/a — factual staleness in a pointer, fixed on sight while the same wave held
  the pen on the primitives being pointed at.
- **Content, both clauses:**
  - "(Layer 1 form-agnostic core · **Layer 2 team transport**)" → "**Layer 2 team transport and
    per-seat context lifecycle**". Made stale **by this revision**, which re-framed Layer 2 into
    two axes; a keeper skill naming one of them would send an author to the home expecting
    transport alone.
  - "Dispatch briefing **+ seat transport**: `templates/agent-dispatch.md`" → "Dispatch
    briefing: …". This one was stale **before this wave**: Seat transport left
    `agent-dispatch.md` for shape Layer 2 at **v0.33.0** (command-succinctness-strip D6) and the
    keeper skill's pointer was never re-swept. Corrected here rather than carried, and its
    pre-existing provenance is stated so this wave takes credit only for noticing it.
- **Kept deliberately:** the whole Overview otherwise — the four-jobs framing, the
  never-restates-its-content rule, the Read-it-first-every-run obligation, and the
  `loop-discipline` pointer. **Job 4 (shape-home revision) is unchanged**: this wave *executed*
  Job 4 and found its procedure sound, so nothing in it was edited on the strength of one run.
- **Consumers assessed:** `authoring-commands` is read by the command-architect agent at
  authoring time only — no command or runtime surface loads it, so the correction costs no run
  any bytes. The `mochiko:command-architect` agent's own description references the shape
  generically ("a codified shape", "the shape's single-sourced home") and carries **no** layer
  enumeration — checked, unaffected. `validation-command-shape` names the file, not the layers.

## [v0.28.0] One README-duplicated rationale clause stripped (body 93 → 92, −1%, deep under-band)

> Count convention: figures use the cluster's uniform `total − 4` measure. This SKILL's
> frontmatter is actually 12 lines (multi-line description), so the literal body is 85 → 84 —
> the delta and percentage are unaffected.
- **Disposition:** deduped → `.mochiko/strips/README.md` (Read: holds the repo-side rule with
  its full rationale and relocation history): Job-3 step 4's parenthetical explaining *why*
  strip notes never live under `plugins/` ("the plugin directory is the shipped artifact;
  operational logs must not distribute") → "(why: the README)". The rule itself — repo-side,
  never under `plugins/` — stays in the skill.
- **Tier failed:** 1 (verified home)
- **Content:** one parenthetical clause
- **Consumers assessed:** wave-open enumeration — 2 citing files (command-architect, mochiko
  router); no section anchors. Session ruling: wave-4 batch-3 ratified 2026-07-25.

## [v0.28.0] KEPT: the remaining body (under-band survivor ruling, 1% vs 30–70)
- **Tier-2 evidence:** the sharpest post-doctrine data point of the pass — this skill was
  authored *in* the succinctness era and already practices the discipline it teaches: it
  declares the command shape's single home and "never restates its content", every Job is a
  numbered behavioral procedure (author / convert / strip) with no quick-reference layer, and
  the four Common-mistakes bullets each name a failure with no other plugin-side home (the
  strip doctrine's repo-side records are not shipped with the plugin, so the skill's compact
  teaching of Tier-1/Tier-2, the ≥3-consumer escalation, and the logging invariant is the only
  copy a plugin consumer receives). Zero further strips proposed. D1: the band is a calibration
  bar, not a quota. Primary evidence for the wave-4-close watch-item (whether 30–70 is the
  right band for post-doctrine skills). Session ruling: wave-4 batch-3 ratified 2026-07-25.
