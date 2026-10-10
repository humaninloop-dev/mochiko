# Blind second list — the shared thinking stage ("the engine")

Seat: blind second-list seat, fresh, 2026-10-11. Given the hardened frame and one question; not
given the lead's options. Fence held: nothing under `.mochiko/brainstorms/implement-brainstorm-led/`,
the brainstorms index, or the user's project memory was opened.

Grounding read first-hand: the `brainstorm`, `specify`, `implement` and `architecture` renders
(binary 0.4.0 · grammar 2 · plugin 0.119.0); `plugins/mochiko/skills/analysis-iterative/SKILL.md`;
`plugins/mochiko/migrations/README.md`; the `command-common` block in `0001-genesis.yaml`;
`crates/mochiko-cli/src/model.rs` and `render.rs` (skill grammar limits); the near-dup ADR; the
accepted `specify-brainstorm-discovery` and `architecture-brainstorm-interview` records; `BACKLOG.md`;
`DECISIONS.md` rows for the skill-content schema.

Routing disclosure: two locate/enumerate dispatches to a native `Explore` subagent spawned
`model: haiku` ended without delivering a report. The locate reads (greps, line numbers, render
sizes) were then run by this seat on the session tier. Nothing below rests on an unread source.

## Facts that bound every road

Quoted, with path and line. My reading of each is marked as a reading.

- **F1 — a skill cannot hold a fail condition or a moment.**
  `crates/mochiko-cli/src/model.rs:224`: "`fail` is command-side only; a skill schema carrying it is a
  finding". `crates/mochiko-cli/src/render.rs:47-48`: "`kind: fail` and `enforces:` are illegal in a
  skill schema (skill-content-schema D9/M2), and skills declare no `moments:` block (D3)".
  *Reading:* whatever form the engine takes, each command keeps its own moment and its own
  `kind: fail` rule. `spec.fail.story-outside-slate` cannot move out of specify.
- **F2 — the common library is for boilerplate.** `.mochiko/decisions/2026-08-28-near-dup-convergence.md:39-40`:
  "D8's other limbs (per-command default, stub-carried binding, block = boilerplate never judgment)
  stand unchanged." `.mochiko/brainstorms/architecture-brainstorm-interview/record.md:1053-1055` (S15):
  the shared stage "is judgment work — a stage the four commands run — and does not fit the near-dup
  convergence item's charter ("boilerplate only, never judgment")".
- **F3 — common blocks already take per-command parameters.** `plugins/mochiko/migrations/0001-genesis.yaml:9793`,
  inside `common.model-tiering`: "`${explore_model}`, interpretive or absence-driven reads on the session
  tier". The brainstorm render resolves it to "spawned model: haiku". *Reading:* a parameterised
  common block works today with no binary change.
- **F4 — the repo's shape for "one discipline, many commands" is a skill plus a pointer stub.**
  `0001-genesis.yaml:9777` `common.transport-floor`, with `pointer: mochiko:patterns-transport-floor`;
  brainstorm and implement both render `*.transport-floor` from it, each with `when: seats=multi` kept
  locally. `patterns-sound-loop`'s description: "Single source of the floor; fourth sibling of the
  minimalism trio."
- **F5 — the existing questioning skill already calls itself the engine, and is prose by ruling.**
  `plugins/mochiko/skills/analysis-iterative/SKILL.md:52`: "The adaptive questioning the Overview
  describes is the single questioning engine. … only the agenda and the concluding artifact differ."
  `DECISIONS.md:51`: "analysis-iterative · grooming-operating-docs · testing-governance-injection stay
  prose". Setup also points at it (`0001-genesis.yaml:2056-2058`, the catalog deck). Brainstorm and
  specify both subordinate it: `brainstorm.own-rules-win`, `spec.own-rules-win` ("Where
  mochiko:analysis-iterative and specify's own rules differ, specify's rules win").
- **F6 — skill schemas carry conditions set by the caller.** The `review-brainstorm` preamble:
  "`lens` · values: decision-quality|record-integrity · resolution: entry-derived · note: the lens the
  spawn brief names." `DECISIONS.md:52` (skill-content-schema D4): the skill grammar inherits
  "`conditions:`+`when:` … `vars:`".
- **F7 — only the lead can hold the conversation.** `specify-brainstorm-discovery/record.md:151`
  (F-H2): the subagent tool filter "removes these tools" — `AskUserQuestion` among them.
  `spec.lead-holds-story-stage`: "no teammate holds a conversation with the user."
- **F8 — implement's candidate set is fixed upstream.** `impl.design-gaps-only`: "Design seats author
  exactly the named gaps, nothing more". `impl.scope-escalation-fail`: "work bigger than the run was
  framed stays FAIL unless the user accepts it". `plugins/mochiko/commands/implement.md:78`: the
  run-open is "One confirmation, no negotiation". `impl.gate-design-checkpoint` is `class: floor`.
- **F9 — specify's story stage is pinned by id.** `spec.fail.story-outside-slate` (`class: floor`,
  `kind: fail`) has "enforces: spec.story-stage, spec.slate-binds-authoring, spec.gate-slate-confirm".
  *Reading:* those three ids must stay live under any road, or the fail rule's `enforces:` dangles.
- **F10 — implement's tools section is near the inline ceiling.** `mochiko-cli rules implement
  --section impl.sec.tools` renders 26,201 bytes; `DECISIONS.md:49` records "inline ceiling ≈ 30k
  chars" for one `!` line. *Reading:* any road that inlines the engine's text into implement needs a
  new section and a new `!` line, not a longer `impl.sec.tools`.
- **F11 — the scheduling the frame overrides.** `BACKLOG.md:42-45`: the shared-stage session's
  "Trigger: build items A and B landed". `BACKLOG.md:77-86`: the near-dup convergence session sits
  "beside (never merged with)" the shared-stage session, "which carries the judgment half."

## What the four instances share, and where they part

*Reading*, from the renders and the two records' *Mechanisms carried* batches
(`specify-brainstorm-discovery/record.md:674-685`, `architecture-brainstorm-interview/record.md:1335-1349`).

**Shared in three or four instances today:** the lead holds the conversation inline · one fork per
turn · at least two real options, one the do-less shape · the fact a fork hangs on is in hand first ·
a rough mock inside the message · a status line (ruled · askable · in fog) · own-turn-or-batch: a
defaults batch confirmed on one screen, never applied by silence · a ratified-yes streak flag, counted
over forks only · a vague answer re-asked as two concrete options · a stall clause (three turns
without shrinking, offer a step back) · a stop rule (everything in or out, nothing open) · inputs
already in hand open the stage as facts, with no size gate · a skip hands the set in and skips the
forks, never the close · provenance per item (`user's words` · `lead's guess, ratified` · `blind
list` · `handed in`) · a blind second list from a fenced fresh seat · the lead transcribes the user's
rulings with no sound loop, then passes the pen with the user's words quoted · an item outside the
ruled set goes to the user before anyone authors it.

**Where they part (the instance bindings):**

| | brainstorm | specify | desk | implement |
|---|---|---|---|---|
| before the stage | frame card, size ruling | intent synthesis, capability frame | health view, visit goal | sufficiency verdict, run-open |
| candidates come from | the decision map | journeys inside the frame | shelf rows, store elements, blind list | the gap list only (F8) |
| unit ruled | a decision card | line + priority + seed test | part + direction + "how I'd know" | (unbuilt) |
| ruled set lives in | `record.md` | `spec.md` `## User Stories` | visit file `.mochiko/architecture/desk/<date-slug>/` | (unbuilt) |
| who authors after | nobody: the record is the output | requirements-analyst | principal-architect | design seats |
| independent leg | cold review of the record | stress-test | tech-lead sameness grade | design review pair |
| blind list | per costly fork | once per run | once per visit, skipped on a one-row ask | (unbuilt) |

*Reading:* the stage has two halves. **(M) the turn mechanics** — how a fork is put, batched,
stopped. **(B) the ruled-set contract** — what one ruled item is, what provenance it carries, and how
the set binds whoever authors next. Brainstorm runs (M) only, because its deliverable is the
lead-written record and no seat authors from it. Specify, the desk and implement run both halves.
Implement differs once more: its candidates are fixed by the gap list, so its blind list cannot add
candidates without becoming scope escalation (F8). Its useful job there is the forks hidden inside
each gap, not missing gaps.

## Roads — what the engine owns (scope)

- **S1 — Mechanics only.** The engine owns (M); each command keeps all of (B).
  *Cost:* smallest engine; brainstorm fits it whole. *Risk:* the half that fixes implement's real
  problem (drafts authored before anything is ruled) stays as three copies that drift. Implement
  would get the conversation craft without a binding set. *Fact:* the problem line is about
  "sign-offs over drafts seats already authored" — that problem sits in (B).
- **S2 — Mechanics plus the ruled-set contract, the contract switchable.** The engine owns (M) and
  (B). (B) is gated by a caller-set condition (for example `binds_author: yes|no`, entry-derived as
  in F6). Brainstorm sets it to `no`.
  *Cost:* one condition and its `when:` gates in the engine. *Risk:* the engine has to name (B) in
  abstract nouns ("the ruled set", "the close screen", "the author") that each command binds to its
  own. A loose binding reopens the gap the frame names. *Fact:* F6, plus the two records' *Mechanisms
  carried* lists, which already match item for item.
- **S3 — Contract only.** The engine owns (B); the mechanics stay local.
  *Cost:* small. *Risk:* leaves brainstorm out of the engine altogether. The user's D6 named four
  commands. *Fact:* `architecture-brainstorm-interview/record.md:1009-1011` (D6-desk-stays-engine):
  "brainstorm's deciding, specify's story stage, the desk's focus stage, and implement's design phase
  … all run the same engine."
- **S4 — Whole stage, one close artifact.** The engine also owns the file where the ruled set lands
  (one template, for example the desk's visit file).
  *Cost:* one template, one home grammar. *Risk:* this breaks specify as built: its slate lives in
  `spec.md`'s `## User Stories` (`spec.slate-home`). It also breaks brainstorm, whose record is its
  output. *Fact:* `spec.slate-home`, and the frame's "must not break … specify's story stage as
  built".

## Roads — what form the engine takes (delivery)

- **D0 — Nothing.** Implement keeps sign-offs only; the three copies drift.
  *Cost:* none. *Risk:* breaks a ruled promise (S14, `architecture-brainstorm-interview/record.md:1574`: "implement's design phase runs the shared
  thinking stage"). The drift risk was accepted only "until the convergence session lands"
  (same record, 1029-1030).
- **D1 — A fourth copy.** Write implement's own stage into implement's schema, as specify and the desk
  did (spec D5-specify-own-rules; the desk default *Where the rules live*, record:1375). No engine.
  *Cost:* one command's migration, in a new section because of F10. *Risk:* four near-identical
  rule sets, and the user asked for one engine. Every later mechanism fix becomes four migrations and
  four audits.
- **D2 — Sequence it: ship A and B as ruled first, extract later.** Build item A lands the desk's own
  copy; the engine comes after (the current BACKLOG trigger, F11).
  *Cost:* cheapest now. *Risk:* the desk's rules are written twice, once local and then as stubs, which
  is two strip trails and two audit rounds for the same text. The frame's "one combined build" exists
  to avoid exactly this.
- **D3 — Reach implement only through the desk.** No engine. Implement's design phase offers a desk
  visit (D5-visit-file-input's offer default) and reads the visit file.
  *Cost:* nothing beyond build item A. *Risk:* it covers only structural gaps. Data-model, contract
  and quickstart gaps (`impl.design-outputs-home`) have no desk, so they stay sign-off-only. It also
  costs the user two commands per feature.
- **D4 — The engine is `/mochiko:brainstorm`.** Specify, the desk and implement pause and hand off to
  a brainstorm run, then take its record in as handed-in facts.
  *Cost:* no new primitive; specify already accepts a named record (`spec.stage-inputs-in-hand`).
  *Risk:* two commands per moment. The record is not slate-shaped or store-shaped, and small asks
  gain nothing. Already rejected once at the desk: "(D) nothing new at the desk, a `/mochiko:brainstorm`
  record handed in — two commands, a record that is not store-shaped, small visits gain nothing"
  (record:557-558). Commands cannot call each other (`disable-model-invocation: true` in each
  command's frontmatter).
- **D5 — Grow `mochiko:analysis-iterative` into the engine.** Convert it to a rendered skill, add the
  stage content, and retire the `own-rules-win` clauses.
  *Cost:* no new skill name, and the "engine" claim already sits in its body (F5). *Risk:* setup
  reads it too (F5) and setup is out of scope, so every engine change would reach setup's
  interrogation. It reverses a recorded ruling that it stays prose (`DECISIONS.md:51`). Specify
  rejected this road too: "(B) a third output shape in the shared questioning skill — grows a file
  setup and brainstorm read" (`specify-brainstorm-discovery/record.md:592-593`).
- **D6 — A new rendered engine skill, run through one pointer stub per command.** A new
  `patterns-*` family skill in the log (working name `patterns-thinking-stage`), with its own
  sections (trigger · scope · discipline · inputs · disclosure · reserved) and caller-set conditions
  (F6). One `common.thinking-stage` pointer block reaches it, and each command extends that block
  with a stub, keeping its class and `when:` local. This is the transport-floor shape (F4). Each
  command keeps its moment and its `kind: fail` (F1), plus its instance bindings.
  *Cost:* one new skill pair; one common block, which fits the boilerplate bar because it is a
  pointer line (F2); stubs plus supersession of about 10 brainstorm and 9 specify stage rules, with
  their strips; build item A authored straight into the stub form; implement gains one moment and a
  handful of binding rules; about five author≠grader audit units; the contract suite re-run. No
  binary change. *Risk:* delivery depends on the lead invoking the skill, although the pointer
  renders in the command and the dependency hook delivers on the Skill-tool call. The engine is
  phrased abstractly and binds through each command's local rules. The two `own-rules-win` clauses
  need a new precedence: the engine wins on mechanics, and the command binds parameters. Specify's
  as-built behaviour must render to the same obligations; the near-dup wave's "extends-aware
  partition" check is the precedent (`2026-08-28-near-dup-convergence.md:12`).
- **D7 — Parameterised command-common blocks, inlined.** The engine is a family of `common.stage-*`
  blocks with `${stage_unit}`, `${stage_close}` and similar vars, and each command's rule ids
  survive as `extends:` stubs (F3; near-dup R3).
  *Cost:* no new document kind, no Skill hop, and the text always arrives in the command's own render.
  *Risk:* it needs a ruling that widens the "boilerplate never judgment" limb (F2), against S15's
  explicit reading. A block carries no section and no conditions, so no single place shows the whole
  stage. It inlines into implement next to the ceiling (F10), so implement needs a new section. It
  also merges the judgment half with the near-dup convergence that F11 keeps separate.
- **D8 — A new CLI document kind (`stage`) with typed slots.** `mochiko-cli rules <command>` splices
  the stage document into the command's render, filling declared slots (candidates, unit, close home,
  blind-seat inputs).
  *Cost:* crate code, a grammar bump to 3, new ops, a `mochiko-cli` release carrying the
  supply-chain controls, a coordinated `plugin.json` bump, and the contract suite. *Risk:* the
  largest build for four instances. The kernel-class bright line holds, since this is composition and
  not judgment, but it still needs a recorded ruling. Grammar skew halts every consumer until they
  upgrade (GI-020).
- **D9 — Share the artifact, not the conversation.** One ruled-set file template, which is the desk's
  visit file. Implement writes it when it runs its own stage, and reads it when the desk wrote it. The
  mechanics stay per command.
  *Cost:* small, and it rides build item A. *Risk:* it covers only the desk and implement (S4's
  break applies to specify and brainstorm), and the conversation craft keeps drifting. *Fact:* D5 of
  the desk record already makes the visit file a design input, so implement writing the same shape
  is one step on.
- **D10 — A facilitator seat holds the conversation.** A persona carries the stage, and the commands
  dispatch it.
  *Cost:* a new persona. *Risk:* the facts rule it out. A subagent cannot ask the user (F7), a
  teammate-held conversation is undocumented, and two accepted records rule "no teammate holds a
  conversation". Listed only so the list is complete.

## The split, under my pick (S2 delivered as D6)

**The engine owns:** all of (M) as listed above, and (B) behind the caller's condition: the unit's
common form (the item, the user's direction in their words, one "how I'd know" sentence or `open`,
never invented); the provenance vocabulary; the stop rule and the stall clause; own-turn-or-batch with
the one-screen batch; the blind second list's fence and its dispatch shape (a fresh persona-less seat
spawned `model: opus`, never shown the set, the conversation or the close file; each difference
becomes a candidate the user rules); a skip skips the forks, never the close; transcription, then the
pen passes with the user's words quoted; an off-set item goes to the user before it is authored; a
load-bearing outside claim is live-checked before its item is ruled.

**Each command keeps:** its moment, and where the stage sits in its run; its pre-stage (frame card ·
intent and capability frame · health and visit goal · sufficiency and run-open); its candidate
source; its unit's extra fields (spec's priority; brainstorm's full decision card); the home and
template where the ruled set lands; the blind list's frequency, inputs and job; the producer, the
grader and the gate wording; its `kind: fail` (F1); and everything after authoring.

**How implement plugs in (the instance bindings it would need):**
- a new moment after run-open and before any design seat plans, firing only when the sufficiency
  check names gaps. The run-open stays "one confirmation, no negotiation" (F8).
- candidates are the gap list, plus any desk visit file keyed to the feature, which enters as
  `handed in`. A candidate outside the gap list is scope escalation to the user, never a widened gap.
- the ruled set lands in a file in the feature home, in the desk's visit-file template. The design
  seats then read one shape whichever command ran the stage (D9 folded in).
- the blind list runs once per design phase. It is given the gap list and returns the forks inside
  each gap. It is skipped on a single-gap re-fire (`impl.midrun-refire`), mirroring the desk's
  one-row skip.
- the design review pair also grades sameness of meaning against the ruled set, as tech-lead does at
  the desk. The design checkpoint stays the user's floor, now over drafts authored inside rulings the
  user already made.
- a new `kind: fail`: a design-phase write with no ruled set behind it, mirroring
  `spec.fail.story-outside-slate`. It is scoped to runs where the design phase fired.
- an epic runs the stage once, over the joint gap list (`impl.epic-design-always-fires`).

## Points the lead should put to the user (not settled by this list)

1. Precedence. Today the command's rules win over the questioning skill (F5). Under an engine, does
   the engine win on mechanics while the command only binds parameters? Or can a command override a
   mechanic by a named exception, such as brainstorm's blind list per fork?
2. Implement's blind list: does it look for the forks inside each gap (my reading), or is it dropped?
   Adding candidates is closed by F8.
3. How specify's as-built stage is proven unchanged: a render-diff partition, as in the near-dup
   wave, or plan-only grids, or both.
4. Whether the engine's pointer stub is `class: floor` in each command (it protects more, and leaves
   only by an anchored ruling) or `must`.

## Pick

S2 delivered as D6: a new rendered engine skill, run through one pointer stub per command, with the
ruled-set half switchable so brainstorm runs the mechanics only. Why: it is the shape the repo already
uses for every multi-command discipline (F4), it needs no binary change and no widening of the
boilerplate bar (F2), and the grammar already keeps each command's moment and fail rule local (F1).
