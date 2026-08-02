# Goals-Driven Commands — Ground-Up Command Architecture Purge

**Status:** accepted — cold-reviewed (solo `devils-advocate`, model-overridden to Fable after the
pinned-Opus spawn failed on this key: 15 raised → 15 carried → 15/15 dispositioned incl. the
4-ruling user batch; verdict `critical-gaps` on the pre-repair record, all four Criticals
repaired/ruled below), user-accepted 2026-08-02
**When:** 2026-08-02
**Session form:** direct `analysis-iterative` invocation (bare session, solo lead + one cold review pass)
**Pilot case:** `/mochiko:specify`

## Problem statement

The user wants to purge the current command style and architecture and relook from the
ground up. The seed model: **a command is a leader with goals. It does not need procedure
knowledge. It has teammates.** The current shape v7 goal-form commands (product of
`command-succinctness-strip` CS-D1–D10 and `lead-owned-process-flexibility`) still carry
substantial process mass: transport mechanics, seat lifecycle, counted bounds, weight
cards, departure trails, recovery tables, floor gates. This session rules that mass away.

## Decisions

### D1 — Roster is resources, not mandate `Confident`

The leader's role is to achieve the goal with the resources it has — teammates, skills.
It is **not** forced to select only from a declared roster: it may spawn outside it, work
inline, or skip seats. The plan is the leader's, composed per run. The leader's role is
not always to plan itself — it may delegate planning to a teammate and manage delivery.

### D2 — Native functionality over hand-rolled machinery `Confident` — *advisory rationale, not a binding (amended at review, I5)*

Grounded in the agent-teams docs (read this session): prefer the **shared task list**
(self-claim, dependency auto-unblock, file-locked), **plan approval** (teammate plans
read-only until lead approves — the native form of "leader gets teammate to plan"), and
free direct teammate-to-teammate messaging. This kills: lead-routed hand-off machinery,
transport probe prose, round counting, and the hand-rolled recovery table.

**No restriction on teammates collaborating.** (User ruling, verbatim intent.)

**Status after review (I5):** under D1 (resources-not-mandate) and D3 (zero process),
D2 cannot bind and has no surviving carrier — it is recorded as the session's
*rationale* for why hand-rolled machinery is safe to delete, not as an instruction any
command carries. The lead of each run rediscovers or ignores native machinery by
judgment.

**Doc grounding quoted (I3 repair):** "The task list directory persists locally and is
never uploaded, so resumed sessions keep their tasks." · "The team config directory is
removed when the session ends." · "No session resumption with in-process teammates:
`/resume` and `/rewind` do not restore in-process teammates." So: the task list is the
resume state for *work items*; teams themselves die with the session and are respawned —
consistent with the prior Recovery finding that sessions and teams do not survive
`/resume`.

**Team-formation risk (C4) — DEFERRED to its own session (user ruling).** Two recorded
incidents (`setup-v3-team-defect` fork B set-and-ignored; `plan-run-transport-forensics`
"the team never existed") show teams do not reliably form even with explicit transport
prose; zero-process commands carry none. Whether goal-form runs degrade to one-shot
subagents — and whether that matters enough to warrant any surviving mechanism — is
explicitly not ruled here. Until that session, the risk is open and unmitigated by
design; the R3 refusal-stands ruling from the forensics session is **not** superseded by
this record.

### D3 — Goals-driven, zero process `Confident` — SUPERSEDES the Q2/Q4 rulings within this session

Earlier in this same session the user accepted "goal + user gates + one stated invariant
(author ≠ grader)" (Q2-C) and the lead recommended "cold at grade time" (Q4-B). The user
then superseded both: **everything is lead judgment. Goals are defined; judgment is not
mentioned to the lead — neither told to exercise it nor which invariants to hold.**
Commands are elevated to goals-driven. No process in the command file — no stated
invariants, no stated gates, no stated verification discipline. If the lead independently
chooses author-separate grading or cold review, that is its judgment, unprompted.

### D4 — Goal only: no roster in the command file `Contested` — re-affirmed at review (I1)

Lead recommended a one-line resource roster (facts about available expertise, not
assignments) on discovery-misfire grounds. User ruled **B: goal only, for now**. The
command names no resources. Marked `Contested`; the discovery-misfire risk (wrong
persona picked by description-matching) is accepted and observable in dogfood.

**Review fold (I1):** the original rationale's "descriptions load into every session" is
broken by this repo's own prior finding (`skill-succinctness-strip` C1: delivery
truncates ~1.8k-char descriptions mid-sentence; `validation-constitution` rendered no
description at all) — some resources are not merely mis-picked but *invisible*. The user
re-affirmed goal-only with that fact surfaced: **adding a roster is a post-dogfood
option**, not a now-mitigation. The ruling stands on accepted risk, not on the broken
premise.

### D5 — Information needs, not artifact prerequisites `Confident`

Driver: the user wants command flexibility — someone without a `spec.md` must still be
able to use the other commands. Resolution: commands declare **information needs**, not
artifact prerequisites. `plan` needs "requirements that are understood" — satisfiable by
a mochiko spec, a PRD, an issue, the codebase, or asking the user; the lead judges
sufficiency and elicits what is missing. Output locations (`.mochiko/specs/…`) become
**default conventions the lead may follow, not contracts**. The pipeline becomes
emergent: when the artifact exists, the next command's lead naturally finds and uses it;
when it doesn't, no refusal. Deliverable specification in the goal is outcome-shaped
(Q6 landed as B-with-a-default; the A recommendation — full path/template/section
contract — was not adopted).

### D6 — Pilot specify; surface count deferred `Confident` — *pilot widened to implement at build (user ruling, 2026-08-02)*

**Build-time amendment:** at the pilot wave the user widened the pilot to **specify +
implement** (precedent: the v0.43.0 user-widened conversion wave). implement rebuilt to the
same three-block form with no per-file draft — form conformance audited against the rulings
with the shipped specify.md as reference; audit PASS. The sequence pin holds: neither rebuild
deletes anything shared. Surface count remains deferred.

With process gone, whether six commands still deserve to exist (slice as a sub-goal of
plan; brainstorm as bare `analysis-iterative`) is a real question — deferred. Pilot the
new form on `specify` first; surface count is a later session with per-merge consumer
assessment.

### D7 — Purge boundary: commands + process doctrine + process-shaped skills `Confident` (as recommended)

The user wants an **aggressive purge**. Ruled scope (option B):

**Dies:**
- The six command files' process mass — rebuilt goals-only.
- Command-process doctrine: strip ceremony (`.claude/rules/mochiko/primitive-edits.md`),
  command-shape residue, transport doctrine, `templates/agent-dispatch.md`,
  `templates/workflow-contract.md`, output-style/report-format process carriers.
- **Process-shaped skills**: grader/review machinery (`validation-*` checklist graders,
  cross-exam apparatus in `review-*` skills). Verification procedure is dead weight when
  verification is lead judgment.

**Survives:**
- **Craft skills** (authoring-requirements, authoring-user-stories, patterns-*, analysis-*,
  brownfield-integration, executing-tdd-cycle …) — the "quality lives in the skill
  library" bet stands; craft is knowledge teammates genuinely lack without skills.
- **Agent personas** — as discoverable resources.
- **Artifact templates** (spec-template etc.) — as resources the lead may find and use.
- **KM layer of this repo** (DECISIONS.md, brainstorm index, strips history) — killing it
  orphans the decision history that makes future purges auditable. Existing strip notes
  are frozen history and stay. *(Rationale corrected at review, I4: the original "it
  governs this repo's own operating docs, not command runs" was factually wrong — the KM
  module's enforcement was built as command steps (v0.29.0) and specify carries a
  user-repo KM-landing binding. The user-repo consequence is ruled separately below.)*
- **Governance injection as goal content** (C1, user-ruled as recommended): the
  production floor built by PO-D1–D7 / SD-D1–D6 / OO-D1–D7 reaches runs through
  commands; that survives — not as process steps but as *outcome*: a goal-form command's
  done-condition includes conformance to repo governance where a governance region
  exists. How the lead delivers governance to teammates is its judgment.
- **User-repo KM landing as goal content** (I4, user-ruled `Contested` against the
  reviewer/lead recommendation to drop it): where a user repo carries
  `.mochiko/memory/knowledge-management.md`, a goal-form command's done-condition
  includes the operating docs reflecting the outcome. The ritual's *steps* die; the
  *state* ("docs agree") is part of done.
- **CLAUDE.md edit surface** (I8 repair): the purge's build waves must edit this repo's
  CLAUDE.md in lockstep — the five-axes conventions (axis 4 names `agent-dispatch.md`;
  axis 5 mandates producer↔validator pairing), the "Editing a shipped primitive is
  itself a landing" paragraph, and the `primitive-edits.md` pointer all describe the
  pre-purge library and are in scope.

Skill-by-skill kill list is build-time work: each skill classified craft vs process at
the purge wave, with borderline cases (e.g. `review-specifications` — craft lens inside
process framing) surfaced for ruling. **Each build-time kill lands its own DECISIONS.md
row (C2 repair)** — no deletion whose only record is build-wave conversation.

### D8 — Doctrine supersession by this record, enumerated and sequence-pinned *(narrowed at review: C2, C3, I7)*

This session supersedes protected content across the **enumerated set**: shape v7
(`lead-owned-process-flexibility`), CS-D1–D10 (`command-succinctness-strip`), TC-D1–D6
seat-lifecycle carriers in commands (`team-lead-strategic-compaction` /
`plan-run-transport-forensics` build), validator-worktree carriers in implement, and the
primitive-edit ceremony (`.claude/rules/mochiko/primitive-edits.md` + its CLAUDE.md
paragraph) across the whole `plugins/mochiko/` surface (commands, skills, templates —
scope aligned with D7's Dies list per I7). Per-line strip entries are waived **for this
enumerated set only**; the waiver is itself part of the ruling this record carries.

**Not superseded:** R3 refusal-stands (deferred with C4) · anything a build wave kills
outside the enumerated set — each such kill lands its own DECISIONS.md row (C2).

**Sequence pin (C3 + I7 + user ruling at acceptance):**
1. Record repaired and **accepted** (this state).
2. **KM landing first** — DECISIONS.md rows (including the strip-waiver ruling itself),
   BACKLOG item, ROADMAP touch — *before any build*.
3. **Pilot wave**: rebuild `specify.md` only. Deletes nothing shared — every template,
   skill, and doctrine file the five old-form commands reference stays in place.
4. **Later waves**: remaining commands rebuild; shared doctrine/template deletion lands
   with the **last** command rebuild, never before.

The non-circularity of superseding the ceremony by ruling depends on this ordering: the
ruling (rows) exists before any ceremony-protected content is touched — the ceremony's
own supersession-by-ruling path, exercised once, wholesale, as its last act.

## Draft: the new specify.md (pilot form) — post-review

```markdown
---
description: Produce an accepted feature specification — prioritized user stories,
  functional requirements, and measurable success criteria — from a feature description.
disable-model-invocation: true
---

# Specify

**Goal:** the feature described in `$ARGUMENTS` has an accepted specification: who it
serves, what problem it solves, prioritized user stories, functional requirements, and
measurable success criteria — precise enough that design and planning can build on it
without going back to the user for basics. Where this repo carries mochiko governance
(a governance region in CLAUDE.md), the specification conforms to it.

**You need:** a feature description clear on who / problem / value — from `$ARGUMENTS`,
the user, or the codebase.

**Done when:** the user has accepted the specification, it is stored durably —
`.mochiko/specs/<feature>/spec.md` is the convention later work looks for — and, where
this repo keeps mochiko operating docs, they reflect it.
```

(~20 lines against today's 163. Exact wording is build-time; this fixes the form: goal,
information need, done-condition with convention-as-default, governance and KM as
*outcomes* (C1, I4), `disable-model-invocation` kept (M1), the "before authoring"
ordering directive removed (M2). No roster, no gates, no bounds, no recovery, no
transport.)

## Review (cold pass, 2026-08-02)

Solo `devils-advocate` cold review (persona's pinned Opus unavailable on this key —
spawned with a Fable override; a transcript-visible departure, noted here). Verdict on
the pre-repair record: **critical-gaps** — 4 Critical / 8 Important / 3 Minor, 15/15
dispositioned:

- **C1** governance injection unclassified → user-ruled **survives as goal content** (folded into D7 + draft).
- **C2** waiver scope exceeded enumeration → D8 narrowed; build-time kills land DECISIONS rows.
- **C3** pilot/purge sequencing broke live commands → D8 sequence pin; pilot deletes nothing shared.
- **C4** team-formation risk vs recorded incidents → **deferred to its own session** (user ruling); R3 not superseded.
- **I1** D4 premise broken by skill-strip C1 finding → surfaced; user re-affirmed goal-only, roster = post-dogfood option.
- **I2** dogfood watches lose instrumentation → accepted on record; watch evidence = transcripts + artifacts.
- **I3** D2 platform claims unverified → doc lines quoted into D2.
- **I4** KM-survival rationale wrong → corrected; user-repo KM landing **survives as goal content** (`Contested`).
- **I5** D1/D2 unreconciled → D2 demoted to advisory rationale, no carrier.
- **I6** no audit path for the rebuild → pilot audit = cold validator conformance-to-D1–D5 read (no process leaked in).
- **I7** D8 self-licensing circular absent ordering → sequence pin (accept → rows → build).
- **I8** CLAUDE.md contradictions post-purge → added to purge edit surface.
- **M1** `disable-model-invocation` kept · **M2** ordering directive removed · **M3** cleared as coherent.

## Open threads

1. **Surface count** (D6) — six → fewer? Later session.
2. **Skill kill list** (D7) — per-skill craft/process classification at build time, each
   kill with its own DECISIONS.md row.
3. **User acceptance** — D3 removed stated gates; the draft's done-condition still
   names user acceptance because it is part of the *goal* (an accepted spec), not
   process. Watch in dogfood whether leads treat it as such.
4. **Discovery misfire** (D4 `Contested`, re-affirmed post-I1) — watch dogfood for
   wrong-persona picks and invisible-resource misses; roster is the post-dogfood option.
5. **Downstream commands** during pilot: plan/slice/implement stay old-form and fully
   functional — the pilot wave deletes nothing they reference (D8 sequence pin).
6. **Team formation under zero process** (C4) — dedicated session pending; until then
   goal-form runs may degrade to one-shot subagents, unmitigated by design.
7. **Dogfood observability** (I2) — trails/counters die with process; watches read
   transcripts and artifacts only. Accepted loss.

## Ruling trail (in-session supersession)

- Q2: user accepted C (goal + user gates + author≠grader invariant) → **superseded by D3**.
- Q4: cold-at-grade-time recommended → **superseded by D3** before ruling landed.
- Q5: roster-as-resources recommended → user ruled goal-only (D4, `Contested`).
- Q6: full deliverable contract recommended → reframed by user's flexibility driver
  into D5 (information needs; convention-as-default).
- Q7: user ruled C (defer surface count) as recommended.
- Q8: user ruled B (as recommended) with "purge aggressively" intent noted.
