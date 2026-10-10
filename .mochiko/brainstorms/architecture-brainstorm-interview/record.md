# The architecture interview as brainstorm's workflow for architecture — decision record

Status: **open** · opened 2026-10-10 · lead: Claude (session) · rulings: deepeshBodh

## Topic (user's words)

> i want the architecture interview to be the brainstorm workflow for architecture. similar to
> how brainstorm works on an abstract problem, architecture workflow should be architecture

## Orientation

- Session index read 2026-10-10 (`.mochiko/brainstorms/index.md`). No earlier session on the
  desk's questioning. Two neighbours:
  - `specify-brainstorm-discovery` — open, opened the same day: the sibling session that takes up
    the same question for `/mochiko:specify`. This session is its peer for `/mochiko:architecture`.
  - `brainstorm-target-state` D19-brainstorm-only-reach (`.mochiko/brainstorms/brainstorm-target-state/record.md`
    line 557 on): "The target applies to `/mochiko:brainstorm` alone … any other command that
    might follow is a separate concern, out of this session's scope." This session is a separate
    concern of that kind, for the architecture desk.
- The word "interview" appears nowhere in the plugin (`grep -rni interview plugins/mochiko/` —
  one unrelated hit in the migration log). Lead's reading (marked as reading): the user means the
  desk's questioning — the greenfield baseline elicitation and the per-visit convergence, as the
  command and its rules describe them below.
- F1 — `plugins/mochiko/commands/architecture.md` lines 56–57: "a micro-brainstorm converges to a
  **one-line visit goal and its explicit done condition**, agreed with the user. Convergence is
  the requirement, not conversation length".
- F2 — `mochiko-cli rules architecture --section arch.sec.roles` (plugin 0.118.0), rule
  `arch.dm-author-baseline` (`class: floor`, `when: store_ruled_content=absent`): "Author the
  baseline — greenfield elicits it; brownfield reconstructs and confirms it per
  arch.tools-brownfield-reconstruction — nothing absorbed is ever silently discarded." The render
  carries no rule on how the greenfield elicitation asks.
- F3 — same render, `arch.sec.boundaries`, `arch.breadth-invariant` (`class: floor`): "every shelf
  row in scope is walked. A row may close in two seconds — `n-a`, one line, done — but it is
  never silently skipped, and the walk order is by retrofit cost"; and `arch.sec.ways-of-working`,
  `arch.recommend-then-arbitrate`: "a shelf row is dealt with its suggested default and the
  reasoning behind it, and the user forms the stance. A default is never applied by silence."
- F4 — `.mochiko/brainstorms/product-architecture-schema/record.md` line 93: "Baseline authored
  once (greenfield elicits; brownfield derives from codebase analysis)". The record rules that
  greenfield elicits; it does not say how.
- F5 — `BACKLOG.md` line 377 on, *Product-architecture first-live-run watch*: "Two-branch
  falsifier — (a) baseline shelf walk unbearably heavy at greenfield, (b) a real plan run proceeds
  without consulting the store". Still open.
- F6 — kinako (`../kinako/.mochiko/product/architecture/spine.md` lines 1–11): `Scope:
  desktop-app`; "Baseline ratified 2026-08-21 by user ruling"; line 8: "**No shelf walk has run.**
  Stances exist only where a ruling already covered the concern. No desktop shelf is seeded
  upstream". The one desk visit on record was a brownfield reconstruction, with no shelf walk.
- F7 — ai-fileops (`../ai-fileops/.mochiko/product/architecture/spine.md`): `**Scope**: desktop`
  and three empty sections; `concerns.md` holds its heading only. Scaffold-only store — the
  greenfield elicitation has not run there. One accepted brainstorm exists
  (`ai-fileops-shared-plugin`, D1–D9), the same one the specify sibling session cites.
- F8 — `skills/analysis-iterative/SKILL.md` line 42 (installed copy 0.118.0): "One question per
  turn — always". Brainstorm overrides this skill by its own precedence rule
  (`brainstorm.own-rules-win`); the architecture render does not point at this skill at all.
- Lead's reading of F1–F7 (marked as reading): the desk's "elicit" is a one-line goal convergence
  plus a shelf walk dealt row by row with a default each — a questionnaire over a fixed agenda.
  Brainstorm's discovery — frame card, decision map, fact before decision, options with a cheaper
  road, blind second list, blind angle map, cold review — exists nowhere on the desk. Neither
  dogfood repo has run the greenfield elicitation yet; kinako took the brownfield path and skipped
  the walk.
- Artifact home rendered 2026-10-10 (`mochiko-cli home --plugin-root plugins/mochiko
  .mochiko/brainstorms/architecture-brainstorm-interview`): closed set `record.md`,
  `synthesis.md`, `build-log.md`, `wave<n>-<slug>.md`; sub-directories `inputs`, `research`,
  `referents`; reports under `reports/`.
- Knowledge-management file present (`.mochiko/memory/knowledge-management.md`): the close ritual
  applies.

## Frame card — proposed 2026-10-10, awaiting the user's first reply

| Line | Text | Source |
|---|---|---|
| Problem | The person running `/mochiko:architecture` on a product (the founder; kinako has a brownfield baseline with no shelf walk, ai-fileops has an empty store and no visit yet) meets a desk whose questioning is a one-line goal convergence plus a shelf walk dealt row by row — a questionnaire over a fixed agenda. Brainstorm's discovery (frame card, decision map, fact before decision, options with a cheaper road, blind second list, blind angle map, cold review) runs nowhere on the desk, so the architecture that lands may be the shelf's defaults and the architect's guesses rather than a shape the user thought through. | user's words ("architecture interview should be the brainstorm workflow for architecture") + guess (the gap, F1–F7) |
| Destination | A ruled target for how the architecture desk thinks with the user: which of brainstorm's mechanisms the desk takes, adapts or drops; whether the shelf walk stays the agenda or becomes the decision map's fog; how an existing brainstorm record feeds the desk without re-asking; where the rules live; what the desk still owns after the thinking (health, drift, routing, store writes). Session ends with decisions a build item can carry. | guess |
| Must not break | The store and its grammar (spine, AX rows, lifecycle, derived index); the breadth invariant and floor precedence; architecture truth as the user's ruling; the author≠grader, sound-loop and plan-approval floors; the desk's downstream readers (implement's sufficiency check, feature's growth door, the drift probe); brainstorm's accepted target state (`brainstorm-target-state` D1–D23); the specify sibling's scope. | guess |
| Betting on | Brainstorm's discovery carries over to the desk's bounded, shelf-anchored question space without making the baseline visit heavier than it already is — the open falsifier (F5a) says the walk may already be unbearably heavy. | guess |
| Out of scope | Brainstorm's own rules; specify (the open sibling); setup, feature, implement; the shelf content (Stage 2 shelves, F6's missing desktop shelf); the store schema except where the new flow changes what is written; the drift probe and health view except where the new flow touches them. | guess |
| Size | standard | guess, user rules |

Fact question put with the card: which "interview" the user means — the greenfield baseline
elicitation only, or every visit (amendment, delta demand, drift disposition) — and whether a
desk run has hurt yet (kinako's baseline visit, or an ai-fileops attempt), or this is ahead of
the first greenfield run.

### Frame hardening — turn 1 (2026-10-10)

The card was re-put in plain words at the user's request ("the language is complex to
understand , simplyfy"). The user's reply, on the Problem line only:

> focusing on just the problem, yes, you are correct. It needs to be a lot tighter to what user
> wants, the brainstorm workflow does a great job , however, the architecture workflow doesnt
> leverage the same mechanics for arhitecture domain

- **Problem — confirmed**, with the user's own emphasis added: the outcome has to be *a lot
  tighter to what the user wants*; brainstorm's mechanics do that well, and the architecture
  workflow does not use the same mechanics for the architecture domain.
- Destination, Must not break, Betting on, Out of scope, Size — still awaiting the user's word.
- Fact question (which visits; whether a run has hurt yet) — still open.

### Frame hardening — turn 2 (2026-10-10)

User, on the Destination line:

> i think the end goal here is how getting architecture to help user brainstorm the part of
> architecture they need to focus on depending on the context given on the workflow. the goal
> you have is the means to it

- **Destination — corrected by the user's word.** The end goal: the architecture command helps
  the user brainstorm *the part of the architecture they need to focus on*, chosen from the
  context the workflow is given (the demand, the store's state, the product, any earlier
  record). The lead's earlier line — which brainstorm mechanics the command copies, changes or
  skips, and what it still does alone — is kept as the *means* to that goal, not the goal.
- Must not break, Betting on, Out of scope, Size — still awaiting the user's word.
- Fact question — still open.
