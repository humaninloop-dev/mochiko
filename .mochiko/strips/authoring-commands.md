# Strip notes — `skills/authoring-commands/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25).

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
