# The Mochiko Command Shape

The single authoritative home of the mochiko command pattern — commands and the keeper
skills reference this file; none restate it (a shape revision is one edit here plus a
re-audit of the conformant commands). **How a command consumes this:** an obligated read —
the lead Reads this file up front; the command states only its per-workflow parameters.
Everything tagged **[PARAM]** below is a parameter — it lives in the command; everything
else is shape — it lives here and only here. A command line that must restate shape content
(rare) carries an inline marked exception — `<!-- shape-exception: why -->` — the audit's
deterministic floor keys on that marker. A one-shot command conforms to Layer 1 alone; a
team-form command to both layers. (Design provenance, revision history, and the v4→v5 slot
map: `.mochiko/strips/command-shape.md`.)

## The anatomy — a goal-shaped document, five blocks

A command is **condition-first**: it states the end state, the checks that prove it, and the
constraints that bind. It is not a procedure to walk. Its structure is fixed —

frontmatter · one-line goal + obligated reads + probe seat · **Goal** · **Seats & checks** ·
**Constraints** · **Bindings** · **Recovery**

**The contract is the document, not an appendix.** `loop-discipline`'s four requirements are
this file's skeleton: done-condition = **Goal** · producer↔validator = the **Seats &
checks** table · bounds and human gates = **Constraints**. There is no `Contract` section,
and no per-run contract file is written — a per-run form whose values are constant at
authoring time is ritual, not proof (`workflow-contract.md` stays the form for loops whose
values genuinely vary per run).

**Conformance** — the authoring bar, and what the audit grades:

- Every block present **per its binding, not per its heading.** A block whose parameters are
  all vacuous for this workflow is one-lined or omitted, and the absence is *stated* ("no
  gates", "no resume table") — never left to inference. A session command with no numbered
  gates and no recovery rows is the worked case (`commands/brainstorm.md`).
- Each block carries only its own class of content.
- **Gates appear as ordered constraints, never as numbered procedural steps.** No `## Phase`
  heading, no `## The flow` heading, and no ordinal-step list inside **Constraints**.
- Every parameter this file declares for the command's form is bound.
- **The preservation standard for any rewrite:** every routing decision and every trigger
  survives; the narration around them dies. The graded exemplar — 8 lines of phase narrative
  carrying ~15 discrete rules, distilled to 3 constraint lines — is fold (a) of D5 in
  `.mochiko/brainstorms/command-succinctness-strip/record.md`. A line traceable to a
  `DECISIONS.md` row or carrying `KEPT:` Tier-2 evidence is translated into the new anatomy
  or superseded by a logged ruling; it is never dropped in the rewrite.

### The blocks

**Preamble.** One line of goal — what this run is for, in the user's terms [PARAM: the goal
line]; the obligated reads (this file, both layers for team-form · `mochiko:loop-discipline`
· `templates/agent-dispatch.md` for briefing — plus, where P6 binds a sized end-stage review,
`templates/sized-end-stage-review.md`, read at the sizing gate rather than up front); and,
team-form, the authoritative first-spawn probe [PARAM: the probe seat, per mode or stage where
it varies]. The **Goal** block states
the same run as a *checkable condition* — if either could be pasted over the other, one of
them is doing nothing.

> **Transition note (v5).** The obligated `mochiko:loop-discipline` read **stays, and its drop is
> deferred — not pending a ceremony.** The pilot checkpoint ruled that *authoring-loop* evidence
> cannot settle it: whether a goal-shaped command holds its gates without the read is answerable
> only by a run. **Named trigger for reopening:** the first live dogfooded run of a rebuilt command
> in which the gates were not rationalized and the bounds held. Until that evidence exists, a
> command that omits the read is non-conformant, not early
> (ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`; `command-succinctness-strip`
> D7 + D10).

**Goal.** One measurable end state [PARAM: the end state — the artifact set that exists, the
checks that passed, the gates that were ruled, and the user's acceptance], plus the states
that read as *not* done [PARAM: the not-done states — concrete states of this workflow].
Initial state is **FAIL**. User acceptance of the deliverable is part of the end state —
plain blocking text, never a timed prompt. Where the project carries a pinned
knowledge-management copy (`.mochiko/memory/knowledge-management.md`), its landing ritual and
invariants are part of the end state under fix-on-sight; no copy → that condition is
vacuous. A command states only its own landing additions, never the generic ritual.

**Seats & checks.** One table, one row per seat: seat · agent × skill(s) · produces or
grades · spawn (standing / cold / disposable, and when — the probe seat marked) · peer edges
[PARAM: the seat rows]. The table **is** the producer↔validator proof: independence is
visible in it, and **no row grades its own output**. Beneath it, one line naming which
validation branch this workflow runs [PARAM: the validation model — the loop's bounded
in-loop critique, or the sized end-stage review of a named artifact] — and, where Layer 2's
lifecycle default is overridden, the `**Seat lifecycle:**` line [PARAM: P17 — see Layer 2].

**Constraints.** The gates, **in order**, one bullet each, in this exact form so the set is
countable — `- **<label>** — evidence: … · rules: … · decides: …` [PARAM: the gate lines; a
review-sizing gate's line carries its default keying, e.g. heavyweight→pair, or tier-keyed].
The label is the command's own (`G3`, or a plain name where the workflow numbers nothing);
what makes it a gate is the three parts, not a number.
Then the bounds, stated once for every loop in the file [PARAM: the caps, the no-progress
exit, the kill-switch path] — the lead counts them, and out of rounds = escalate, never
done. Then this workflow's invariants — entry gates, guards, prerequisites, ordering rules,
what is deliberately out of scope — and any `KEPT:` survivor with the pointer to its evidence
[PARAM: the invariants and survivors].

**Bindings.** The concrete referents. Each artifact's path and ID namespace [PARAM: the
artifact set]. Where the artifact's uncertainty is carried [PARAM: the carrier — a
**lead-penned record** carries statement + rationale + a confidence mark per element
(`Confident / Assumed / Contested / Unsure / Deferred`), with user corrections and reversals
logged where they happen; a **producer-authored artifact** carries its own assumption /
open-question surface instead]. The fact route [PARAM: the fact seat, reality artifact, or an
Explore subagent]. The verify-pass owner, where a sized review binds [PARAM: the owning
reviewer]. And where Layer 2's devolved branch binds, the clearing unit and its checkpoint
[PARAM: the unit — e.g. a cycle, a round — any workflow-specific condition that
*de*-devolves it, and what the escalated branch's human checkpoint keys on]; the shape's own
devolving conditions are referenced, never restated.

**Recovery.** One line of pause posture [PARAM: where resume state is noted — **on the
deliverable** by default; a command names another location only when it has one], plus the
evidence → resume-at table [PARAM: the rows]. Sessions and teams do not survive `/resume`,
and a shared account limit can throttle the team and the main session together — escalation
then has nowhere to go but pause. Resume from **workspace evidence**, never a context
`phase` field, respawning only what the stage needs.

**Slot index** — the audit's parameter-completeness set: **P1** goal line · **P2** probe seat
(team-form) · **P3** end state · **P4** not-done states · **P5** seat rows · **P6**
validation model · **P7** gate lines · **P8** bounds · **P9** invariants + survivors · **P10**
artifact set (paths + ID namespaces) · **P11** uncertainty carrier · **P12** fact route ·
**P13** verify-pass owner (sized review only) · **P14** clearing unit + checkpoint keying
(devolved branch only) · **P15** pause location · **P16** resume rows · **P17** lifecycle
override (team-form, **override only** — an unbound P17 states nothing at all: the block-absence
rule above does not reach it, because Layer 2's default is what governs the silence).

## Layer 1 — form-agnostic core

**Frontmatter.** `description:` carries goal, loop shape, and gates in one breath;
`disable-model-invocation: true` — commands are user-invoked entry points, never model-fired.

**One lead.** The body addresses a single lead, who owns the loop's counters, every verdict,
every escalation, every human gate, and the user-facing conversation. Agents produce and
review; the lead adjudicates. One exception, team-form only: Layer 2's devolved clean branch
clears a unit on the verifying seat's evidence, with no lead read and no gate. The loop is a
`mochiko:loop-discipline` sound loop and every dispatch is briefed per
`templates/agent-dispatch.md` — referenced, never restated.

**The conversation is the production surface.** It belongs to the lead and the user, and it
carries no machinery: no "phase", "round", or "gate" talk in user-facing prose. (The anatomy
bans phase *headings*; this bans the vocabulary the user hears.)

**As-you-go artifact.** The deliverable is written as the work progresses, never reconstructed
at the end, in one ID namespace (P10), with its uncertainty on the artifact itself (P11). A
lead-penned record additionally reads standalone as the review surface and audit trail: review
findings and their dispositions live in a closing Review section, never interleaved with the
elements; and the lead's pen covers its own formulation only — nothing amends a user-ruled
element, and no new element exists, without the user's word.

**Sized end-stage review — a conditional read.** Where P6 binds a sized review of a
judgment-heavy record or synthesis, that branch's doctrine is single-sourced at
`templates/sized-end-stage-review.md`: Read it at the sizing gate, not up front. Where P6
binds the loop's bounded in-loop critique instead, that file is never loaded — the in-loop
branch satisfies producer↔validator on its own.

**Ground rules.** Stay kernel-free — no Python/MCP brain code, no capability catalogs, no
DAG-mediated orchestration. A command suggests commits; it never runs git mutations and
never pushes.

## Layer 2 — team transport and per-seat context lifecycle

Two independent axes. **Team transport** is how seats are spawned, addressed, and kept
independent. **Per-seat context lifecycle** is how long one seat's context lives before it is
reset. A seat recycled on a cadence is still fully team-form in transport; neither axis is
evidence about the other.

### Team transport

**Hard requirement — agent teams.** Check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in the
environment before anything else; unset → stop and tell the user how to enable it
(settings/env; Claude Code ≥ v2.1.178). The env check is a proxy — the **first teammate
spawn is the authoritative probe** (P2): if it fails, stop with the same instructions. Never
proceed teamless — **no fallback transport**, a dogfood-pilot bet marked `Contested`, to be
revisited when mochiko distributes beyond the author's machines.

**Seat transport.** A team-form seat and a one-shot subagent ride the same Agent tool, and the
substrate sometimes picks wrong, so the lead carries the mechanics — not just the vocabulary:

- **Spawning a seat** = one Agent call carrying **`name:`** (e.g. `name: producer`), phrased
  in the docs' own idiom — "create an agent team", "spawn a teammate named `<seat>`" — not
  only mochiko's "seat". **A spawn without a `name:` is a one-shot subagent — in a team-form
  command, the forbidden transport.**
- **Every later round** is a `SendMessage` to that same seat — its name, or after a recycle
  its named successor. The anti-pattern is **nameless one-shot dispatch**: a round re-spawned
  with no roster, no messaging, no independence structure. Deliberate boundary recycling under
  the lifecycle axis below is not that anti-pattern and is never read as one.
- **Verify before proceeding — check the roster, because addressability does not discriminate.**
  `SendMessage` doesn't require agent teams to be enabled, so a *named subagent* is addressable
  too and a send that merely succeeds proves nothing (one full live run passed this probe on
  subagent transport). Confirm the seat **positively, from the team's own roster**: the team
  config at `~/.claude/teams/<team>/config.json` — `<team>` being `session-` followed by the
  first eight characters of the session ID — holds a `members` array carrying each member's name
  and agent ID. **Read it and confirm the seat's `name` is in `members`.** A one-shot subagent
  never appears there. Not in `members` ⇒ the forbidden transport: kill it and respawn explicitly
  requesting an agent team; the same result again stops the run per the hard requirement, there
  being no degraded branch to take. Two corroborating signals — **observed, not documented**, so
  useful mid-run and never the proof: the harness resolving a name as a *subagent in this
  session*, and a send landing *resumed from a transcript in the background*. The agent panel
  distinguishes neither.

**Seats, not dispatches.** Teammates do **not** load `skills:` frontmatter — every spawn
prompt names the skill and role itself, plus what to Read (briefing fields:
`agent-dispatch.md`). A teammate's plain text is invisible to the lead: reports arrive as
**messages**, and every follow-up goes to the **same named seat** — or, after a recycle, to its
named successor.

**Seat legibility.** Tell the user at the start that they can watch or message any teammate
directly. Announce each seat in one line when it is filled — an unexplained teammate spawn
reads as a malfunction. Teammate housekeeping (idle notifications, acks) is never narrated
and never replied to.

**Independence by structure.** Independence is carried by **who fills the seats and when they
arrive** — disjoint agents, disjoint skills, no seat grading its own output, all checkable in
P5's table. That is what **structural separation** means here, and it is the whole of it:
never a persona's say-so, and for in-loop seats never a routing pattern. **Cold arrival is a
property of the stage, not of the traffic:** a seat is cold when it is not in the room before
its own stage, which is what the end-stage review's isolation rests on. A seat that arrives
cold at its stage still meshes in-loop from then on; the two are independent. It follows that
a **respawn is cold by design** — a seat refilled at its own stage after a pause is cold by
the same definition, so recovery never costs independence.

**In-loop mesh — the default.** *In-loop* = the workflow's own produce→check rounds, never the
sized end-stage review. Producers hand work to the verifying seat **directly**; the lead is
the exception handler, not the switchboard. Two traffic classes, and every message is one of
them:

- **Peer-routable** — verification hand-offs: work-is-ready, and the evidence and status
  coming back (the verdict on a non-clean unit is the lead's — see Clearing). A peer-routed
  gap list is a **hand-off, not a start signal**: the producing seat revises only when the
  lead opens the next round, and the lead's brief carries that hold.
- **Lead-routed** — coordination notices, policy, reported deviations, scope: rulings, not
  hand-offs. A seat relaying between other seats has become a hub, and that traffic was the
  lead's.

The lead names each seat's peer edges in its brief (P5's peer-edge column).

**Clearing under the mesh.** The lead reads the escalations and the endgame, not every clean
result. One unit of work (P14) advances on the verifying seat's **PASS-with-evidence, unread
by the lead**, when *all* of: every verification in it is a deterministic CLI check that
passed 100% · no deviation was reported · no domain-registry addition was made. **Everything
else returns to the lead** — any failure, any GUI or subjective verification, any reported
deviation, any domain-registry addition — for its verdict and whatever human checkpoint the
command binds there (P14). The devolved branch is *exactly* the deterministic-and-clean one,
and that exactness is the guard: wherever judgment exists, the verifying seat's status is
**input, never the gate**.

### Per-seat context lifecycle

**Continuity lives in the transcript or in the artifacts — named per seat, never assumed.** A
standing seat re-processes its whole transcript every round, and a gate pause past the cache
lifetime re-pays it at full price; past a few units the artifact set a fresh successor reads is
the cheaper carrier. Standing is a choice with a cost, never a property that buys continuity
free.

**The lead's context responsibility is the seats', not its own.** No lead-unilateral compaction
lever exists — the model can neither invoke nor observe compaction, on its own context or a
seat's. The seats' one real lever is **kill-and-respawn**, and it is a reset, not a summary: it
discards everything not on disk.

**The governed set — standing seats whose lifetime spans multiple units.** The criterion
governs, never a list. A seat that arrives cold and then stands is governed once it is
multi-unit: cold arrival guarantees freshness at its first round, not its fifteenth. Exempt are
cold end-stage seats — already cold at their own stage, so recycling one buys freshness it
already has — and any governed seat with **no countable unit**, cadence-exempt for want of a
denominator and covered by the user override below.

**The default cadence.** At each gate pause the lead **counts** each governed seat's completed
loop units — the command's own counted unit, the one its Bounds already count — and recycles at
**~≥3**, cache warmth composing on the same trigger: an early, still-warm pause keeps the seat
standing, where recycling costs more than it saves. **Counted, never observed** — no occupancy
measurement, no seat self-report: no surface exposes a seat's context occupancy to the lead or
to the seat itself, so a self-reported context-health line is an **invented number**. The user,
who can see the panes, may order a recycle at any gate. A command writes a lifecycle line
**only to override** — where its recycle moments genuinely differ from this default, or its
counted unit is ambiguous in the file. No override → nothing is written, and the default governs
the silence (**P17**, tagged with the other slots in The blocks).

**Respawn-as-reset.** The successor is briefed from the **existing on-disk artifact set alone** —
no lead-authored state summary, no new handoff artifact — so a seat is recyclable at a boundary
**iff** that set is current on disk, which is why the dying seat shuts down when its work is on
disk, *before* the gate pause. The successor takes a **versioned name** (`producer-2`),
announced like any seat: never reuse the dead seat's bare name — a send to a name that now
resolves to a different agent is refused rather than delivered, and a refused send is a lost
round, not an error that stops you. The **seat** persists across incarnations; a roster row
reading *standing* describes the seat, not one context.

**End-of-need shutdown.** A seat whose remaining work is zero and whose re-summons is improbable
shuts down rather than idling to session end — hygiene plus the platform's documented advice,
never a measured saving that outranks a live re-summons likelihood.

**No ritual sends.** Every send to a standing seat re-materializes its whole transcript, so
nothing is sent that is not a real dispatch: no compact requests (compaction is not a lever
either of you holds) and no stamp-only resumes — fold one-line confirmations into the next real
dispatch.

---

**Shape version:** v6 (2026-08-01 — `team-lead-strategic-compaction` TC-D1–D6 ·
`standing-seat-lifecycle` D1–D3 as amended — the **encoded subset** of the ruled D1–D4 (D3's
per-roster-declaration clause superseded by TC-D6; D4's absence is by design, per the strip
note) · `plan-run-transport-forensics` R1/R2/R3/R4: Layer 2 re-framed into its two axes —
**team transport** unchanged in content, plus a new **per-seat context lifecycle** axis
(seats-only lead responsibility · governed-set criterion · the counted-not-observed ~≥3 gate-pause
cadence · respawn-as-reset with versioned-name successors · end-of-need shutdown · the
no-ritual-sends line) · the standing-seat continuity clause retired · the fresh-spawn
anti-pattern retargeted at nameless one-shot **transport** · the first-spawn probe's broken
addressability discriminator replaced with a **documented positive roster check** (the team
config's `members` array), the harness's name-classification and send-delivery strings demoted
to observed-not-documented corroboration, refusal preserved with no degraded branch · **P17**
added to the slot set, override-only; v5 2026-07-30 — command-succinctness-strip D3 · D5 (+ folds a–c) · D6 ·
D7/D10 as amended at verify V1: the goal-shaped five-block anatomy replaces the flow/phase
body and the Contract appendix · Seat transport absorbed from `agent-dispatch.md` · the P1–P16
slot set supersedes v4's 13 tags · the obligated `loop-discipline` read retained under a
checkpoint-gated transition note · **pilot-checkpoint ruling B (same revision):** the sized
end-stage review's doctrine split out to `templates/sized-end-stage-review.md` as a
conditional read, loaded only where P6 binds it · **the Run-cost entry element dropped** by user
ruling (step-1 adjudications), retiring v3's manual-baseline carrier · **v0.34.0 pilot checkpoint:**
the `loop-discipline` read-drop **deferred to a named live-run trigger**, authoring evidence ruled
insufficient; v4 2026-07-30 — team-method-vs-command-shape D1–D3: the
in-loop mesh becomes Layer 2's default with named traffic classes · cold isolation restated as
a review-stage property · the deterministic-clean verdict devolves to the pair; v3 2026-07-23
— workflow-token-reduction wave 1: run-cost entry added · provenance header relocated to the
strip note; v2 2026-07-19 — the S8 home-revision checkpoint) · **Governed by:**
`mochiko:loop-discipline` · **Pairs with:** `agent-dispatch.md` (call-time briefing) ·
`sized-end-stage-review.md` (conditional — the sized-review branch of P6) ·
`workflow-contract.md` (the per-run contract form) · `report-format.md` (the report envelope
seats write to).
