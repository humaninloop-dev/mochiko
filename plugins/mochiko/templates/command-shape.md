<!--
COMMAND SHAPE — the codified mochiko command pattern
====================================================
The single authoritative home of the shape /mochiko:brainstorm and /mochiko:setup were
built in (design: .mochiko/brainstorms/pattern-codification-and-minimalism/record.md —
D1 codifies the artifact shape, D3/D8 make this template the surface, fold S2 makes it
the SOLE home). Commands and the keeper skills (authoring-commands, validation-command-shape)
REFERENCE this file; none of them restate it. A shape revision is therefore one edit here
plus a re-audit of the conformant commands — the D1 churn constraint, carried structurally.

Two layers, deliberately (fold S2 closed D2's layering thread):
- LAYER 1 — FORM-AGNOSTIC CORE: what every mochiko command carries, team-form or one-shot.
- LAYER 2 — TEAM TRANSPORT: what a command that hard-requires agent teams adds.
A one-shot command conforms to Layer 1 alone; a team-form command to both.

HOW A COMMAND CONSUMES THIS: an obligated read. A conformant command instructs its lead to
Read this file up front, then states only its per-workflow parameters. Everything tagged
[PARAM] below is a parameter — it lives in the command; everything else is shape — it lives
here and only here. A command line that must restate shape content (rare) carries an inline
marked exception — `<!-- shape-exception: why -->` — the audit's deterministic floor keys
on that marker.

S8 CHECKPOINT: the home-revision checkpoint ran at the first one-shot→team-form conversion
(specify, 2026-07-19) → shape v2: the artifact's uncertainty carrier became a [PARAM]
(lead-penned records carry confidence marks; producer-authored artifacts carry their own
assumption/open-question surface), the sized end-stage review's applicability was scoped
(an in-loop independent critique satisfies validation via the Contract clause), and the
ground rules (kernel-free · no git mutations) were homed here from the command footers.
-->

# The Mochiko Command Shape

## Layer 1 — form-agnostic core

**Frontmatter.** `description:` carries goal, loop shape, and gates in one breath;
`disable-model-invocation: true` — commands are user-invoked entry points, never model-fired.

**One lead.** The body addresses a single lead: it owns the loop (counters, verdicts,
escalation), every human gate, and the user-facing conversation; agents produce and review,
the lead adjudicates. The loop is declared a `mochiko:loop-discipline` sound loop and every
dispatch is briefed per `templates/agent-dispatch.md` — both referenced, never restated.

**The conversation is the production surface.** It belongs to the lead and the user. Never
narrate machinery — no "phase", "round", or "gate" talk in user-facing prose.

**As-you-go artifact.** The deliverable is written as the work progresses, never
reconstructed at the end, in one ID namespace [PARAM: the artifact, its path, its ID
scheme]. Its uncertainty is carried on the artifact itself [PARAM: the carrier — a
**lead-penned record** carries statement + rationale + a confidence mark per element
(`Confident / Assumed / Contested / Unsure / Deferred`) with user corrections and
reversals logged where they happen; a **producer-authored artifact** carries its own
assumption / open-question surface instead]. A lead-penned record is additionally the
review surface and the audit trail: it reads standalone, review findings with their
dispositions live in a closing Review section, never interleaved with the elements — and
the lead's pen covers only its own formulation: nothing amends a user-ruled element, and
no new element is created, without the user's word.

**Sized end-stage review.** This element binds where the workflow reviews a
judgment-heavy record or synthesis at convergence [PARAM: the reviewed artifact — or a
Contract-section declaration that the loop's bounded, in-loop independent critique is the
workflow's validation, satisfying the producer↔validator clause without a sized review].
Where it binds: at convergence the lead opens a **sizing gate**: state the
artifact's weight (element count, confidence-mark mix, reality-surface load) and the
estimated review cost; recommend **pair / single / none** [PARAM: the default keying —
e.g. heavyweight→pair, or tier-keyed]; the user rules. **None** → a review waiver in the
artifact's Review section (who waived, at which gate, why): the validator seat passes to
the user alone, deliberately and auditably. Otherwise cold reviewer(s) spawn [PARAM:
agent × `review-*` skill × lens briefs], each counterpart withheld from the other's brief;
each reads the frozen artifact cold, forms findings independently, and reports
findings-formed (count only) before its counterpart is introduced. The artifact is
**frozen** from reviewer spawn until every disposition lands (Review section excepted). A
pair then runs the **one-shot four-message cross-examination** — protocol single-sourced
at `skills/review-brainstorm/references/CROSS-EXAM.md`; owner-withdrawal only, the
counterpart persuades, never vetoes. Each reviewer returns its own survivors (severity,
concrete failure scenario, resolution path, unresolved counterpart objections attached)
and its own tally ("N raised, M survived"; fallen retrievable on ask) with a recommended
status; **the cross-set merge and the combined tally are the lead's, never a reviewer's**.
Survivors route by answer-owner:

- **User territory** (challenges to user rulings; user-declared facts, as confirmation) →
  the user, directly. Not a tie-break — theirs to answer.
- **The lead's formulation** → argue with the finding's owner, **max two exchanges
  (lead-counted)**; unresolved at the cap is a deadlock → tie-break: both positions + a
  recommendation, the user rules.
- **Facts** → checked [PARAM: the fact seat, reality artifact, or an Explore subagent],
  never argued; a fact already routed is cited, never re-routed.
- An overruled survivor marks its element `Contested`; nobody re-raises it.

One disposition per survivor — **resolved / user-ruled / recorded-open** — then one
**verify pass** [PARAM: the owning reviewer; a solo reviewer verifies the lead's folds —
it grades the repairs, not its own findings], quoting the evidence the folds landed.
**Review + verify is the bound**: a survivor still blocking after it escalates to the user
with both positions — out of bounds is never silently done.

**Contract section (authoring-time fill).** The command closes with a `Contract` section —
its authoring-time fill of `loop-discipline`'s four requirements: **Done-condition**
(initial state FAIL; the concrete not-done states named; user acceptance of the deliverable
is part of it — plain blocking text, never a timed prompt), **Producer ↔ validator**
(different agents, different skills, structural separation), **Bounds** (every cap, and who
counts it — the lead), **Human gates** (every named gate). **No per-run contract file is
written** — a per-run form whose values are constant at authoring time is ritual, not proof
(`workflow-contract.md` stays the form for loops whose values genuinely vary per run).

**Ground rules.** Stay kernel-free — no Python/MCP brain code, no capability catalogs, no
DAG-mediated orchestration. A command suggests commits; it never runs git mutations and
never pushes.

**Recovery.** Sessions and teams do not survive `/resume`, and a shared account limit can
throttle the team and the main session together — escalation then has nowhere to go but
pause. Pause posture: note resume state on the deliverable [PARAM: where]. Resume from
**workspace evidence**, never a context `phase` field, respawning only what the stage needs
[PARAM: the evidence → resume-at mapping].

## Layer 2 — team transport

**Hard requirement — agent teams.** Check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in the
environment before anything else; unset → stop and tell the user how to enable it
(settings/env; Claude Code ≥ v2.1.178). The env check is a proxy — the **first teammate
spawn is the authoritative probe** [PARAM: which seat spawns first, per mode/stage]: if it
fails, stop with the same instructions. Never proceed teamless — **no fallback transport**
(a deliberate dogfood-pilot bet, marked `Contested`; revisit when mochiko distributes
beyond the author's machines). Running the loop on one-shot subagents is not a fallback —
it is the defect this layer forbids. Spawn mechanics, the `name:` discriminator, and the
post-spawn **addressability check** are single-sourced at `templates/agent-dispatch.md`
(Seat transport) — a command names its probe seat and points there.

**Seats, not dispatches.** Teammates do **not** load `skills:` frontmatter — every spawn
prompt names the skill and role itself, plus what to Read (briefing fields:
`agent-dispatch.md`). A teammate's plain text is not visible to the lead: reports arrive as
**messages**, and every follow-up goes to the **same named seat** — that continuity is what
a standing seat buys. A fresh spawn per round is the subagent anti-pattern wearing a team's
clothes. [PARAM: the seat roster — agent, skill, spawn timing, standing or cold.]

**Seat legibility.** Tell the user at the start that they can watch or message any teammate
directly. Announce each seat in one line when it is filled — an unexplained teammate spawn
reads as a malfunction. Teammate housekeeping (idle notifications, acks) is never narrated
and never replied to.

**Independence by structure.** Cold seats (reviewers, validators) are never in the room
before their stage, and producer↔validator traffic routes through the lead — who talks to
whom is the independence guarantee, carried by the roster, never by a persona's say-so.

---

**Shape version:** v2 (2026-07-19 — the S8 home-revision checkpoint, run at the specify
conversion: uncertainty-carrier [PARAM] · sized-review applicability scoped · ground rules
homed) · **Governed by:** `mochiko:loop-discipline` · **Pairs with:** `agent-dispatch.md`
(call-time briefing + seat transport) · `workflow-contract.md` (the per-run contract form).
