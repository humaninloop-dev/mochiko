# Command Architecture Realignment — Decision Record

**Status:** accepted (2026-08-02) — un-reviewed bare session; no cold review ordered, no
recorded waiver beyond the user's "settled" acceptance
**When:** 2026-08-02
**Form:** bare session (direct `analysis-iterative` invocation; lead + user)
**Topic:** the fundamental architecture of mochiko commands — aligning them to Claude Code's
`/goal` model (condition-driven completion) and native agent-team orchestration
(lead plans, delegates, and synthesizes). The user's opening position: a command's role is
very high level — it should not carry procedural details. Current commands are asserted
not fit for purpose.

## Opening position (user, verbatim intent)

- Commands should align to goals per https://code.claude.com/docs/en/goal — goals achieved
  by planning and orchestrating teammate agents and skills per
  https://code.claude.com/docs/en/agent-teams.
- The role of a command is very high level, not procedural.
- Current commands are not fit for purpose.

## Reality surface (lead-gathered, session start)

- Command set: 6 files, 1,249 lines total — brainstorm 184 · implement 234 · plan 227 ·
  setup 265 · slice 169 · specify 170.
- Current anatomy (shape v7/v8 lineage, `command-succinctness-strip` CS-D1–D10 +
  `lead-owned-process-flexibility`): five blocks — Goal · Seats & checks · Constraints ·
  Bindings · Recovery. "The Contract becomes the document." Runtime `/goal` was **rejected
  as additive** at CS-D1; the ruling was goal-*shaped documents*, not the `/goal` feature.
- Shape v7 already moved rigor to lead judgment: stated default pipeline, lead departs at
  will, departures recorded, non-waivable floor (user gates · cold-graded folds ·
  self-declared bounds · departure trail).
- `templates/command-shape.md` deleted at v0.46.0; `validation-command-shape` skill deleted
  at v0.45.0; doctrine purge waves 1–2 (v0.46.0–v0.47.0) made commands self-contained.
- `/goal` doc facts: condition ≤4,000 chars; evaluator is a small fast model reading only
  what the conversation surfaced (no tools); effective condition = one measurable end state +
  a stated check + constraints that must hold; `/goal` is a wrapper around a session-scoped
  prompt-based Stop hook.
- Agent-teams doc facts: lead coordinates in natural language; shared task list with
  dependencies + self-claim; teammates load CLAUDE.md/skills/MCP but not the lead's history;
  spawn prompt carries task-specific context; subagent definitions reusable as teammates
  (their `skills:`/`mcpServers:` frontmatter NOT applied); docs advise 3-5 teammates,
  5-6 tasks each; hooks (TeammateIdle/TaskCreated/TaskCompleted) available as quality gates.

## Session trail

**Q1 — what does "not fit for purpose" mean concretely?**
User: the sheer volume of detail inside a command. Cannot believe all of it is required;
the shape as-is says the lead's judgment cannot be trusted to work toward a goal and figure
things out. Diagnosis is not a specific run failure — it is a stance: the architecture
encodes distrust of the lead, and that distrust is the defect.

Lead's framing of the fork this opens: most command bulk is scar tissue from documented
failures (transport probe ← teams silently never formed, v0.9.1 + v0.38.0 forensics;
recovery tables ← sessions/teams do not survive `/resume`; counted bounds + floor gates ←
lead-owned-process-flexibility driver: a 2-element amend consuming ~18 rulings and a
3-round validator loop; seat lifecycle ← the 836k-token completeness seat). The question is
whether those lessons die, move to another home, or stay.

**Q2 — what happens to the encoded lessons when detail leaves the command?**
Options: A delete outright · B relocate to skills/rules off the read path · C floor-only
(goal + non-waivable floor stay; all choreography deleted). Lead recommended C with a B
element (transport mechanics to one shared reference).
User: **C only — no moving, no creating skills or shared mechanics.** Plus a reframe that
sharpens the whole session: *the goal of the command is to create the harness required for
the goal.* A command's job is to stand up the harness — the team, the gates, the bounds —
that the goal needs; achieving the goal within that harness is the lead's judgment.

## Decisions

### D1 — Commands become goal + harness; all choreography dies in place `Contested`
A command carries (a) the goal — a verifiable completion condition — and (b) the harness
required for that goal: the non-waivable floor the lead operates under. Stage/seat
choreography, default pipelines, recovery tables, and procedural detail are deleted, not
relocated: no new skills, no shared-mechanics files are created to receive them. The lead
is trusted to plan and orchestrate toward the goal within the harness.
Marked `Contested` on the delta from the lead's recommendation: the lead recommended
relocating transport mechanics to a shared reference; the user ruled no relocation of any
kind. Rationale (user): the volume of encoded detail is itself the defect — it encodes
distrust of the lead's judgment.

**Q3 — harness contents sort (recommend-then-arbitrate).**
Lead's recommended keep-set: goal condition · user gates (as reserved decisions, not gate
machinery) · author≠grader (one sentence) · team-transport requirement (one sentence) ·
bindings. Drop-set: counted bounds/caps/kill-switches · seat tables and agent×skill
assignments · recovery tables · seat lifecycle + weight card + departure trail · KM landing
steps (CLAUDE.md carries the ritual repo-side). Two reversals flagged explicitly before
ratification: dropping the departure trail reverses the v7 ruling of 2026-08-01; dropping
counted bounds removes the brake the lead-owned-process-flexibility postmortem paid for.
User: **holds, deliberate** — and adds the harness should heavily rely on **plan mode**
(agent-teams doc: teammate plan approval — teammate works read-only until the lead approves
its plan; lead approves autonomously, user criteria can steer it).

### D2 — Harness keep/drop set ruled; plan approval replaces procedural control `Confident`
Keep in the command: goal condition (measurable end state + stated check + not-done list) ·
decisions reserved to the user, one line each · author≠grader, one sentence · team-transport
requirement, one sentence · bindings (artifact paths + templates). Drop, dying in place:
counted bounds, round caps, kill-switch tokens · seat tables and fixed agent×skill
assignments · recovery tables · seat lifecycle/recycling, the run-start weight card, the
departure trail · KM landing steps. Both flagged reversals (departure trail — reverses a
v7 ruling two days old; counted bounds — removes the postmortem-paid brake) were put to the
user plainly and ratified deliberately.
Control moves from procedural rails to **native plan approval**: teammates plan in
read-only mode; work starts only when the lead approves the plan. The harness names this
reliance; the mechanics are the platform's, not the command's.

**Q4 — where does plan approval bite?**
Options: A universal (every seat) · B producers only (graders/fact-finders exempt) ·
C replaces grading (rejected framing — conflicts with the kept author≠grader line).
User: **B**, per the lead's recommendation.

### D3 — Plan approval for producer seats only `Confident`
Any seat that writes artifacts or code spawns with plan approval required and works
read-only until the lead approves its plan; grading and fact-finding seats are exempt.
Plan approval controls work before it is spent; author≠grader checks it after it exists —
two different failure modes, both kept, approval spent only where mistakes cost rework.

**Q5 — rollout: pilot-first or one wave?**
Lead recommended A (pilot on `plan`, live-run, then convert the five) — the new shape's bet
is unproven and pilot-first is the CS-D10 precedent. User: **B — one wave, all six.**

### D4 — One conversion wave across all six commands `Contested`
All six commands are rebuilt to the goal+harness shape in a single wave — one ceremony, one
audit round. Marked `Contested`: the lead recommended pilot-first; the user ruled the wave.
Risk accepted: a shape defect discovered post-wave is six commands wide.

**Q6 — the v8 anatomy skeleton.** Lead proposed Goal · Harness · Bindings (~30–50 lines per
command), goals rewritten in artifact-and-acceptance terms with no stage/gate vocabulary.
User's response raised one objection: **"I don't want to discount subagents"** — reopening
the team-transport line kept at D2.

**Q7 — how transport-neutral?**
Options: A fully neutral (no transport line; lead picks teammates vs subagents per seat,
per the platform doc's own split) · B team default with subagents allowed (soft steer).
User: **A.**

### D5 — Transport-neutral harness; the team-transport line dies `Confident`
The harness carries no transport mandate. The lead chooses per seat: teammates where seats
must talk to each other or the user needs to address them; subagents for focused
report-back work (cold grades, fact-finding). Amends D2 within this session: the kept
"team-transport requirement, one sentence" line is dropped from the keep-set. Note: the two
documented transport failures (v0.9.1, v0.38.0 forensics) were failures to satisfy a
*mandate* that teams form; with the mandate gone, that defect class dissolves — a subagent
is now a legitimate choice, not a silent breach.

### D6 — Shape v8 anatomy: Goal · Harness · Bindings `Confident`
Three blocks. **Goal**: measurable end state + stated check + not-done list, written in
artifact-and-acceptance terms only — no stage or gate vocabulary (existing Goal blocks
referencing G-numbers are rewritten). **Harness**: plan approval for producer seats ·
author≠grader one-liner · decisions reserved to the user, one line each. **Bindings**:
artifact paths, templates, entry conditions — what the lead cannot invent. Nothing else:
no Constraints block, no Seats table, no Recovery table. Target ~30–50 lines per command.

## Acceptance

Put to the user plainly before close: the wave rewrites six Goal blocks and deletes lines
traceable to prior DECISIONS.md rows (CS-D1–D10 anatomy, TC-D1–D6 lifecycle, the v7
floor/weight-card/departure-trail set, the transport rows); each departs only as a recorded
supersession-by-ruling, this record being the ruling; strip notes will be large and
multiple prior DECISIONS rows gain supersession annotations. User: **"settled."**

Streak note: Q4/Q5/Q7 were three consecutive single-letter rulings; flagged at wrap. Two of
the three went against or past the lead's recommendation (D4 wave-over-pilot, D5 fully
neutral), so the streak was read as engagement, not passive acceptance — no re-deal forced.

## Supersessions this ruling causes (annotate at build)

- CS-D1–D10 five-block anatomy (`command-succinctness-strip`) → D6 three-block anatomy.
- Shape v7 stated-default-pipeline + weight card + departure trail + counted bounds
  (`lead-owned-process-flexibility`) → D1/D2.
- TC-D1–D6 seat-lifecycle cadence carriers in commands → D2 (lines die in place; the
  session-record rationale stays historical).
- Team-transport mandate rows (`setup-v3-team-defect` D1 · `plan-run-transport-forensics`
  probe · the "Hard-require agent teams, no fallback" standing bet, `Contested` 2026-07-04)
  → D5 transport-neutral.
- Recovery tables, seat tables, KM-landing command steps across all six commands → D2.

## Build obligations

- One wave, all six commands → shape v8 (Goal · Harness · Bindings), ~30–50 lines each.
- Goals rewritten in artifact-and-acceptance terms; no stage/gate vocabulary.
- Producer seats spawn plan-approval-required; graders/fact-finders exempt.
- Strip notes per command (large; supersession-by-ruling entries citing this record);
  independent author≠grader audit per command (`mochiko:validator`, command's own text).
- DECISIONS.md: supersession annotations on the affected prior rows; ROADMAP standing-bet
  line ("Hard-require agent teams") superseded.

## Open questions

- Whether the KM landing ritual, dropped from command text (D2), still fires reliably from
  CLAUDE.md/rules alone — watch at the first post-wave command run.
- First live run of a v8 command is the shape's first evidence (no pilot, D4) — watch and
  record.
