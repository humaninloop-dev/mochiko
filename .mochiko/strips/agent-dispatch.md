# Strip notes — `templates/agent-dispatch.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6c; ratified 2026-07-23).
v4 also fixed a staleness: the footer paragraph claimed plan/tasks/implement were still
one-shot; all seven commands have been team-form since v0.17.0 (BACKLOG conversion rows).

---

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Briefing version-history block relocated (class 2, 1,011 B / 11 lines)
- **Disposition:** superseded → relocated **verbatim** into this note (below). In-file residue: the
  bare stamp plus the live routing (`Governed by / Pairs with`), which is wiring a run consumes, not history.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim, the whole block as it stood at the scrub):**
```
**Briefing version:** v7 (2026-07-31 — team-method escalations closed
(ADR `.mochiko/decisions/2026-07-31-team-method-escalations-closed.md`): a **ninth field** by user
ruling — peer edges + hand-off holds, carried by reference to shape Layer 2 — and field 6 re-routed
for the mesh, the v3 lead-relay surviving as one of its two routings; v6 2026-08-01 — `standing-seat-lifecycle` D3 as amended by TC-D6: the
Layer-2 pointer retargeted — lifecycle joins transport at the same home, and a versioned-name
refill is named as a briefable call; v5 2026-07-30 — command-succinctness-strip D6: Seat transport relocated
to `command-shape.md` Layer 2 · the team-form roster paragraph relocated to the strip note ·
the degrades-gracefully restatement deduped; v4 2026-07-23 — header relocated to the strip
note; roster staleness fixed) · **Governed by:** `loop-discipline` · **Pairs with:**
`command-shape.md` (the command pattern, seat transport + per-seat context lifecycle) ·
`workflow-contract.md`
```
- **Kept deliberately:** the version *number* and its date stay in the file — a consumer still
  learns which revision it is reading; only the per-revision narrative left.

## [v0.44.0] Rationale-and-provenance pointer
- **Disposition:** superseded → deleted; this note is the home it pointed at.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(Rationale + provenance: `.mochiko/strips/agent-dispatch.md`.)
```
- **Kept deliberately:** the caller-side rule above it — workflow/siblings/"done" live on the caller
  side, and the skill is named as a hint, not a command.

## [v0.39.0] Field 6's verbatim-paste-only retry brief — the superseded v3 lead-relay

- **Disposition:** superseded → rewritten in place as a two-routing field. Briefing **v6 → v7**.
  The verbatim paste survives as *one* of the two routings (the case where no peer edge carried
  the feedback); what died is the assumption that it is the *only* one.
- **Tier failed:** n/a — supersession by ruling (team-method **D1/D3**, `DECISIONS.md` 2026-07-25
  row → `.mochiko/brainstorms/team-method-vs-command-shape/record.md`; escalation raised at
  `.mochiko/decisions/2026-07-30-layer-2-mesh-rewrite-executed.md`, **closed by**
  `.mochiko/decisions/2026-07-31-team-method-escalations-closed.md`). D1 made the verifying seat's
  gap list **peer-routed**; a field instructing the lead to paste it verbatim describes a relay the
  mesh retired, and re-pasting text the producer already holds is the hub drift D1 forbids.
- **Content (v6, verbatim — the whole table row):**
  ```
  | 6 | **Prior feedback (retries)** | On round > 1, paste the validator's prior issues verbatim; else omit | "Address: <validator's issues-requiring-fix>." |
  ```
- **Kept deliberately:** the field itself, its number and its name (nothing renumbers — the
  ninth field appends) · the `On round > 1 … else omit` conditionality · **the verbatim paste and
  its example cell**, unchanged in wording, now bound to the no-peer-edge routing: a one-shot
  regrade or a successor that was not in the room for the hand-off has no gap list to point at, and
  a command may bind that relay explicitly (`implement`'s P17 lifecycle line relays the failed-task
  list and the just-failed `cycle-report.md` at dispatch — this field stays consistent with it,
  never restates it).
- **Consumers assessed:** every command and skill that briefs a dispatch reads this file; **none
  restates field 6**, so no stale copy is inherited. Team-form commands with a peer-edged
  producer↔verifier pair (`implement`, and any later binder) gain the first routing; one-shot and
  lead-routed dispatches — the `review-*`/`validation-*` skill calls, `plan`'s lead-gated
  reviewers, `setup`'s lead-routed validate — keep exactly the v6 behavior under the second
  routing. `command-shape.md` Layer 2 is unchanged by this wave and remains the sole home of both
  the peer-edge obligation and the hand-off hold (field 9 points at it; it is not restated here).
  `loop-discipline` requirement 2 is amended in the same wave (its own note, same stamp).

---

## [v0.38.0] The Layer-2 pointer retargeted — lifecycle joins transport at the same home

- **Disposition:** superseded → rewritten in place. Briefing **v5 → v6**. The paragraph was
  already a ~50-word pointer after v5's relocation, so this is the minimal matching edit the
  ruling asks for, not a second home.
- **Tier failed:** n/a — supersession by ruling (`standing-seat-lifecycle` **D3**:
  *"`agent-dispatch.md`'s Seat-transport section gains the matching retarget"*, as amended by
  **TC-D6**; wave note: `.mochiko/strips/command-shape.md` v0.38.0).
- **Content (v5, verbatim):** "**Seat transport** (spawning a named teammate, the `name:`
  discriminator, the addressability probe) now lives in `templates/command-shape.md` **Layer 2**
  — command-layer-only mechanics, homed with the rest of the team transport. This file is
  form-agnostic: it briefs a call, whether that call fills a seat or fires a one-shot subagent."
- **What changed and why, clause by clause:** "the addressability probe" → "the first-spawn
  probe" (the v5 discriminator was **falsified** this same revision — naming the retired check
  in a pointer would send a reader hunting for text that no longer exists) · the pointer's
  contents list gains "the recycle cadence with its respawn-as-reset briefing", because the
  lifecycle axis now lives at the same address · the form-agnostic sentence gains a third case,
  **"refills one with a versioned-name successor"**, with the retarget stated in six words: *a
  refill is an ordinary dispatch to brief, never the transport anti-pattern.* That clause is the
  whole of D3's ask on this file — this is the one place a caller decides how to brief a
  respawn, so it is where the carve-out has to be visible.
- **Kept deliberately:** the pointer form itself (no doctrine returns to this file — the v5
  relocation stands) · the form-agnostic framing and its one-shot-subagent case, which is what
  keeps this file usable by non-command dispatches.
- **Consumers assessed:** every skill and command that briefs a dispatch reads this file; none
  restates the pointer, so nothing downstream breaks. The **eight briefing fields and the
  independence hard line are untouched** — a respawn brief is filled with the same eight fields
  as any other call, which is precisely why no new machinery was added here. `command-shape.md`
  Layer 2 carries the reciprocal wording (its own note, same stamp); `loop-discipline` untouched.

---

# v0.33.0 — briefing v4 → v5 (Seat transport leaves; the remainder assessed line-by-line)

**Wave context:** command goal-shape rebuild, **step 1 of 4** (design:
`.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D6; `DECISIONS.md` 2026-07-30).
D6 ruled two things about this file: Seat transport moves out to shape Layer 2, and **the rest is
assessed line-by-line at v5 altitude**. That assessment's outcome: the 8-field briefing table and
the one-hard-line independence section are this file's enduring job and survive intact; two
connective/duplicative lines strip; nothing else leaves. File 800 → 663 words, 5,183 → 4,175 B
(−17.1% / −19.4%) — a **true reduction only for the 137 words that were deduped or deleted**; the
~230 words of Seat transport are a *relocation*, and they now cost every command run from
`command-shape.md` instead (net run-level accounting: `.mochiko/strips/command-shape.md`, the
floor-arithmetic entry, finding 3).

**Additions this revision** — for the decision row, not strips: the closing pointer naming shape
Layer 2 as Seat transport's new home + the form-agnostic framing ("it briefs a call, whether that
call fills a seat or fires a one-shot subagent") · in the independence section, three lines
relating the per-call check to the shape's structural one (the Seats & checks table is where no
row grades its own output; this checklist is the per-call restatement at the moment of the call) ·
`command-shape.md` added to the Pairs-with line.

## [v0.33.0] Seat transport relocated out → `command-shape.md` Layer 2
- **Disposition:** relocated → `templates/command-shape.md` **Layer 2** (arrival entry:
  `.mochiko/strips/command-shape.md`, same version).
- **Tier failed:** n/a — supersession by ruling (**CS-D6**: the section is "command-layer-only
  content currently sitting in a file every skill dispatch also references"; the split cost a
  cross-file reference hop on every run). Not a minimalism strip — the content is unchanged and
  still live, one file over.
- **Content (the relocated section, faithfully compressed — the mechanics moved substantively
  verbatim):** a team-form command's seats ride the same Agent tool as one-shot subagents (no
  separate team-creation step since v2.1.178; the fork is one parameter; the substrate
  documentedly picks wrong sometimes — *"Claude may sometimes use subagents instead of creating a
  team"*, agent-teams docs) · spawning a seat = one Agent call carrying **`name:`**, phrased in
  the docs' idiom, and **a spawn without a `name:` is a one-shot subagent — the forbidden
  transport** in a team-form command · every later round is a `SendMessage` to that same name ·
  verify the first spawn yielded an **addressable teammate**, since the agent panel alone doesn't
  distinguish teammates from subagents — not addressable → kill and respawn, explicitly
  requesting a team.
- **Also relocated here (roster + provenance, non-live):** "All six commands are currently
  team-form, each per its recorded conversion assessment with a first-dogfood confirm-or-revert
  checkpoint (`.mochiko/strips/<command>.md`; assessment doctrine:
  `.mochiko/brainstorms/pattern-codification-and-minimalism/record.md`, D2). One-shot dispatch
  remains the rebuttable Layer-1 default for any future command designed on it; this section
  binds only commands that hard-require teams. Defect history + ruling:
  `.mochiko/brainstorms/setup-v3-team-defect/record.md` (D1)." **Kept deliberately:** the live
  rule in that paragraph — one-shot conforms to Layer 1 alone, team-form to both layers — was
  already stated in `command-shape.md`'s header and is not duplicated. The roster sentence is
  standing staleness risk (it was wrong once already, fixed at v4) and belongs in a non-loaded
  log, not a runtime template.
- **`SendMessage` note:** v4's transport bullet closed with "A fresh spawn per round is the
  subagent anti-pattern wearing a team's clothes." That sentence already existed **verbatim** in
  `command-shape.md` Layer 2's "Seats, not dispatches"; on arrival the duplicate died rather than
  landing twice. Deduplication, logged so the loss is not silent.
- **Consumers assessed:** all six commands name "`templates/agent-dispatch.md` (Seat transport)"
  for transport — correct while they are v4 files, re-pointed at shape Layer 2 when re-authored
  (pilot + wave). Every non-command consumer of this file (the skill dispatches: `authoring-*`,
  `review-*`, `validation-*`, and the agent personas that brief calls) used the 8-field table and
  the independence section only, never the transport section — they lose nothing and now read a
  file 19% smaller.

## [v0.33.0] Two connective lines stripped from the briefing body
- **Disposition:** deleted.
- **Tier failed:** (1) "A good brief carries the context below." — **Tier 2**: pure connective
  prose introducing the table that follows it; names no behavior and prevents no failure.
  (2) "A field you leave out isn't a failure — it's context the agent will ask for or supply from
  its own judgment." — **Tier 1**: restates the header's own degrades-gracefully rule ("None of
  it is a precondition for the agent to *function* — the agent degrades gracefully on a thin
  brief") eleven lines later, in the same file.
- **Kept deliberately:** the actionable half of (2) — "Fill what raises quality; trust the
  professional with the rest." — survives as the table's closing line, and the header's
  degrades-gracefully rule survives verbatim as the single statement of that point.

## [v0.33.0] Title placeholder `[PHASE]` → `[STAGE]`
- **Disposition:** superseded → `[STAGE]`.
- **Tier failed:** n/a — supersession by ruling (**CS-D3/D5**: the phase posture dies with the
  flow/phase body; shape v5 forbids `## Phase` headings and the anatomy speaks of stages). A
  briefing template that still says "phase" teaches the vocabulary the wave is removing.

---

## [v0.22.0] HTML comment header relocated (runtime-loaded rationale)
- **Disposition:** relocated → here (D6c). The live kernel stayed in the visible body: "a caller-side checklist, not a file you commit — fold the fields into each dispatch prompt", the degrades-gracefully point, the workflow-knowledge-stays-caller-side rule. The header's independence bullet was redundant with the body's "What the caller MUST get right" section and simply died.
- **Tier failed:** pure waste (map §5): Read-tool template loads do not strip HTML comments — the 1,783 B header (27% of the file) cost context on every team-form run.
- **Content (the relocated rationale, faithfully compressed):**
  - **Why this exists:** a mochiko agent is a self-contained professional that degrades gracefully — a sparse brief produces a worse result, not a broken agent; the checklist raises loop quality, it does not make the agent functional. The agent owns none of the workflow knowledge (which workflow, which siblings, what "done" means) — all of that lives caller-side; pushing it into a persona is the coupling this template is the antidote to.
  - **Companion note:** call-time companion to `workflow-contract.md` — the contract proves the LOOP is sound; agent-dispatch is how the caller briefs each CALL inside it well (redundant with the body footer's Pairs-with line).
  - **Header independence bullet:** independence is structural — different agent, different skill, never grade your own output (restated verbatim in the body's hard-line section; deleted as duplication).
