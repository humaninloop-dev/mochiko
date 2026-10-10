# The architecture interview as brainstorm's workflow for architecture — decision record

Status: **accepted** 2026-10-11 (user: "accept") · cold read `critical-gaps` (29 raised → 24 survivors, 24/24 dispositioned: S1, S5, S13 user-ruled one by one; S8 and S14 the user's word; S11, S12, S16, S23 ruled inline as coverage candidates; the rest one user-confirmed repair batch), verify round 1 NOT CLEAN (21/24 CLEAN, V1–V6 lead-repaired), verify round 2 NOT CLEAN, nothing Critical (8/9 CLEAN; V7 user-stamped, V4 and V8–V10 lead-repaired, closed seat-unverified by the user's word) · **superseded in part 2026-10-11** by `implement-brainstorm-led` (the default *Where the rules live* — its desk-wins precedence flipped by the user's word — and D6-desk-stays-engine's hand-off to a later session; build items A and B ship with that session's build item C) · frozen for the cold read 2026-10-11 after the accept screen ("confirm") · opened 2026-10-10 · lead: Claude (session) · rulings: deepeshBodh

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

### Frame hardening — turn 3 (2026-10-10): the specify landing read, at the user's ask

User: "I want you to get context from latest changes in specify workflow which lines up with what
i want to do with architecture". Read 2026-10-10: the sibling record (now **accepted and built**
at v0.119.0, commit `202dae7 story-discovery`), its index entry, the specify command's current
render (`mochiko-cli rules specify`, plugin 0.119.0), and the BACKLOG item it left.

- F9 — `specify-brainstorm-discovery` D1-story-discovery-stage (`.mochiko/brainstorms/specify-brainstorm-discovery/record.md` line 443 on):
  "`/mochiko:specify` gains a story-discovery stage that runs after the intent synthesis is
  confirmed and before any story is authored. In it the story set is worked with the user the way
  brainstorm works decisions: forks put one per turn, each with at least two real options of which
  one is the do-less shape … the facts a fork hangs on in hand before it is put, closing in a ruled
  slate." Lead's reading: the shape is *a discovery stage, run like brainstorm, closing in a ruled
  set that binds the authoring after it*.
- F10 — `specify-brainstorm-discovery` D2-line-plus-test (the same record, line 480 on): the user rules "the story line — who, does
  what, why — with its priority and its independent test"; scenarios and requirements are authored
  afterwards inside the ruled lines. Lead's reading: the user rules a small unit per item, the
  seat expands it without change of meaning.
- F11 — `specify-brainstorm-discovery` D3-lead-holds-stage (the same record, line 514 on, `Contested`): "The story stage is run by the
  lead, inline, in the main conversation … No teammate holds a conversation with the user". The
  user's own words: "the lead is the manager of the teammates."
- F12 — `specify-brainstorm-discovery` D4-blind-journey-list (the same record, line 548 on): "one fresh seat with the confirmed intent
  synthesis and the capability frame — never the slate or the conversation — and it returns the
  journeys it sees … each difference is a candidate row the user rules in or out. One list per spec
  run".
- F13 — `specify-brainstorm-discovery` D5-specify-own-rules (the same record, line 578 on): the stage's rules "live in specify's own
  schema" with a precedence clause over `mochiko:analysis-iterative`; "Convergence of the three
  commands' ways (brainstorm, specify, the architecture desk) into a `common.<slug>` block is
  handed to a later session, opened when the sibling architecture session lands". The BACKLOG
  item *Three-command convergence session* (BACKLOG.md, *Specify story-stage build* section) names
  this session's landing as its trigger.
- F14 — same record, *Defaults batch — confirmed*, *Mechanisms carried* (line 666 on): carried —
  one fork per turn · two real options, one do-less · fact in hand first · an in-message mock ·
  a status line (ruled · askable · in fog) · ratified-yes streak flag · vague answer re-asked as two
  options · stop rule · provenance mark per row (`user's words` · `lead's guess, ratified` · `blind
  list` · `handed in`) · own-turn-or-batch · stall clause. Dropped — "a frame card (the intent
  screen is it) · decision cards and a record file (the spec is the record) · a blind angle map and
  cold review (the stress-test exists)".
- F15 — same section, *Inputs already in hand* and *Size and skip*: "a brainstorm record named in
  `$ARGUMENTS`, or stories already in story form, open the stage as facts; only what they leave
  open is asked"; "no size gate; a decisive user gets confirmations and a fast wrap-up"; a skip
  hands stories in — "A skip with nothing in hand is not a skip — the stage runs".
- F16 — `specify-brainstorm-discovery` D6-two-confirm-screens (the same record): intent confirmed on its own screen first, the slate
  on a second; `specify-brainstorm-discovery` D7-dogfood-criteria-deferred (`Deferred`): what a run is judged on waits for the
  first ai-fileops run.
- F17 — `mochiko-cli rules specify --section spec.sec.roles` (plugin 0.119.0),
  `spec.lead-holds-story-stage`: "You run the story stage yourself, inline in the main
  conversation, with the capability frame in hand; no teammate holds a conversation with the user.
  The requirements-analyst seat authors the stories from the confirmed slate." And
  `spec.analyst-brief-and-pen`: the lead writes the slate as "transcription of the user's rulings,
  not authoring — before the requirements-analyst seat opens it, then pass the pen explicitly".
- Lead's reading of F9–F17 against the desk (marked as reading). The specify shape maps onto the
  desk almost one to one: *intent synthesis* ↔ the desk's one-line visit goal (already confirmed
  with the user) · *story-discovery stage* ↔ a new architecture-discovery stage, before the
  architect seat authors anything · *slate* ↔ a ruled set of which architecture parts this visit
  decides, and what each part is (the stance per row, or the shape of the delta) · *analyst authors
  from the slate* ↔ the principal-architect seat authors the baseline, stance batch or delta from
  the ruled set, graded by tech-lead as today · *blind journey list* ↔ a blind list of the
  architecture angles the context implies · *handed-in inputs* ↔ a named brainstorm record opens
  the stage as facts. Two things do not map: the desk's shelf walk (a fixed agenda specify has no
  counterpart to — the breadth invariant says every row is walked), and the desk's per-visit
  convergence (specify has one fixed goal). The user's corrected destination — "help the user
  brainstorm the part of architecture they need to focus on depending on the context" — reads as
  the slate's job here: which part, chosen from context, before any shelf row is dealt.

### Frame hardening — turn 4 (2026-10-10): the user widens the scope

User:

> i want to expand the scope, i want to relook at what architecture workflow builds, i want it to
> build and maintain diagrams https://github.com/cathrynlavery/diagram-design like, but not
> limited to state machine, ER data model , sequence , flowchat . diagram that explain to human
> and agents how the project works.

- **Destination — widened by the user's word.** Two parts now: (1) *how the desk thinks with the
  user* — the part of the architecture to focus on, chosen from context (turn 2); (2) *what the
  desk builds and maintains* — diagrams in the manner of the named repo: state machines, ER data
  models, sequence diagrams, flowcharts, and others — diagrams that explain to humans and to
  agents how the project works. The lead's earlier Out-of-scope guess ("the store schema except
  where the new flow changes what is written") no longer holds as written; it is re-put with the
  next card.
- Fact work opened on the user's word: the named repo (what it is, which diagram kinds and
  tooling), and what the desk and its skills produce as diagrams today (spine template,
  `patterns-system-design`, the diagram conventions reference, entity-modeling's state machines).

#### Facts gathered for the widened scope (2026-10-10)

- F18 — `https://github.com/cathrynlavery/diagram-design` (GitHub API, read 2026-10-10;
  description field): "Editorial diagram design for Claude Code, Codex, GitHub Copilot, Factory
  Droid, and Pi. 44 diagram types. Self-contained HTML + SVG. No shadows. No Mermaid slop." README
  line 3: "An agent skill … that draws editorial diagrams as one self-contained HTML+SVG file, in
  your brand, with design rules baked in." Stars: 48,412; last updated 2026-10-10.
- F19 — same README, line 175: "The result is not an editable source format: there is no text to
  diff in a pull request and no canvas to drag a box on. For docs that live in Git and change
  every sprint, Mermaid is still the better fit. Diagram Design is for the diagram you are going
  to publish." Line 647, under "When *not* to use this skill": "**Diagrams that live in Git and
  change every sprint** → Mermaid. It diffs, and GitHub renders it." Line 653: "Before drawing,
  ask: *would a reader learn more from this than from a well-written paragraph?* If no, don't
  draw."
- F20 — same repo, `docs/screenshots/` (file tree): the shipped types include `architecture`,
  `architecture-delta`, `data-flow`, `db-schema`, `deployment`, `dependency`, `er`, `flowchart`,
  `sequence`, `state`, `it-state`, `swimlane`, `layers`, `uml-class`, `journey`, `story-map`,
  among 44. `docs/adr/0012-architecture-delta.md`: "The new grammar is **Before · Changes ·
  After**: two equal local grids joined by an exact, object-addressable change ledger" with
  statuses ADDED · REMOVED · CHANGED · MOVED · REWIRED, verified by
  `scripts/verify-architecture-delta.py`. `docs/adr/0004`: SKILL.md capped at 40,000 bytes, a
  router with detail in `references/`.
- F21 — `mochiko-cli template architecture-spine` (plugin 0.119.0): "`## Container diagram` — One
  C4-container view as a mermaid block … One diagram, not a gallery — a second view earns its
  place only where the first cannot carry it." "`## Key flows` — Sequence diagrams for the flows
  that qualify". Budgets: Container diagram 30 lines · Elements 40 · Key flows 30 · As-built notes
  15. Lead's reading: the store carries exactly two diagram kinds today — one container flowchart
  and the qualifying sequence diagrams — both mermaid, both line-budgeted.
- F22 — `plugins/mochiko/skills/patterns-system-design/SKILL.md` (prose skill, no rule set in the
  log) and `references/DIAGRAM-CONVENTIONS.md` lines 3–4: "**C4-as-method, flowchart-as-carrier**
  (mermaid's dedicated C4 syntax is experimental — use standard `flowchart` / `sequenceDiagram`
  and apply C4 discipline by hand), and **the delta is visible**". Line 55: "the diagram must be
  valid mermaid that renders — it is presented as a *picture*, never as a raw code block standing
  in for one." The skill's pieces: container delta diagram · sequence diagrams for qualifying
  flows · checkpoint table · conditional deployment view (only with `IP-XXX` rows).
- F23 — `.mochiko/brainstorms/architecture-design-primitive/record.md` line 158:
  "D8-rendered-diagram-signoff — The sign-off presents the *rendered* diagram, never raw source";
  line 208: "R5 is a bounded, recorded exception to D8-rendered-diagram-signoff's never-raw-source
  rule" (degrade-with-record when no render surface exists). The kinako dogfood (BACKLOG, the
  *Architecture-primitive dogfood* item) found "R5's degrade-with-record fallback exercised
  (un-rendered, G3 stamp)" and "the rendered-diagram primary path" still open.
- F24 — `plugins/mochiko/skills/patterns-entity-modeling/references/RELATIONSHIP-PATTERNS.md` line
  153 "## Relationship Diagram Notation" / line 155 "### Text-Based Diagram Format";
  `STATE-MACHINES.md` line 37 "### Diagram", line 158 "Diagram matches transitions table". No
  `mermaid`, `erDiagram` or `stateDiagram` token in that skill. Lead's reading: ER and state
  machines exist in mochiko as text notation inside a feature's `data-model.md`, authored at
  design time by the technical-analyst, never in the architecture store and never as a rendered
  diagram.
- F25 — kinako live state (`../kinako/.mochiko/product/architecture/spine.md`, 233 lines): one
  mermaid block (line 45, `flowchart TB`), `## Key flows` at line 195 with no mermaid block under
  it; root `ARCHITECTURE.md` carries one mermaid block. Two feature-level `architecture.md` files
  exist under `.mochiko/specs/`; no `stateDiagram` or `erDiagram` anywhere in `.mochiko/`.
- Lead's reading of F18–F25 (marked as reading). Three things stand out for the deciding: (a) the
  named repo's own author says its output is for publishing, and points at Mermaid for diagrams
  that live in git and change — which is what "build and maintain" and "explain to agents" ask
  for; the user's "like, but not limited to" may mean the *taste and the type range*, not the
  HTML+SVG format — a fact question for the user; (b) mochiko already owns two of the four kinds
  the user names (sequence, flowchart-as-container-view) in the store, and the other two (state
  machine, ER) at feature level as text; what is missing is a product-level home, a maintenance
  rule, and a rendered path that works; (c) the kinako record shows the rendered path was never
  exercised live — the diagrams were signed un-rendered.

### Frame hardening — turn 5 (2026-10-10)

User: "mermaid text diagrams with its taste.  i didnt understand the second fact you asked"

- F26 (user) — **Format: Mermaid text diagrams, drawn with the named repo's taste** (and, per
  turn 4, its range of types — "not limited to state machine, ER data model, sequence,
  flowchart"). Not its HTML+SVG files. Lead's reading: the diagrams stay diffable text in git,
  readable by agents as source and rendered for humans; the repo's design rules (one accent, few
  colours, delete first, "would a reader learn more than from a paragraph") become the taste the
  drawing follows, applied through Mermaid's means.
- The second fact question (which visits; whether a run has hurt) was not understood; re-put in
  plain words next turn.

### Frame hardening — turn 6 (2026-10-10): hardened

User: "every visit, i havent felt it yet, ok for 3 to 6"

- F27 (user) — the brainstorm-style thinking applies on **every visit**: the first (baseline) and
  every later one (a feature's delta demand, a drift finding, an amendment).
- F28 (user) — the user has **not yet felt the problem in a run**: no desk run has hurt; the
  diagnosis of the current desk comes from reading its rules and the dogfood stores (F1–F7), and
  carries `Assumed` until a run shows it (as the sibling's OQ1 does for specify).
- Must not break · Betting on · Out of scope · Size — confirmed ("ok for 3 to 6").

## Frame card — hardened 2026-10-10

| Line | Text | Source |
|---|---|---|
| Problem | The user running `/mochiko:architecture` meets a desk whose questioning is a one-line goal convergence plus a shelf walk dealt row by row with a default each — a questionnaire over a fixed agenda. Brainstorm's mechanics do a great job of getting tight to what the user wants; the architecture workflow does not use the same mechanics for the architecture domain, so the architecture that lands may be shelf defaults plus the architect's guesses, not tight to what the user wants. Not yet felt in a run (F28). | user's words (turns 1, 6) + the lead's reading of the rules (F1–F7) |
| Destination | Two parts. (a) The architecture command helps the user brainstorm *the part of the architecture they need to focus on*, chosen from the context the workflow is given — on every visit (F27); the mechanics question (which of brainstorm's and specify's mechanisms the desk takes, adapts, drops; what the desk still owns) is the means. (b) The command builds and maintains diagrams that explain to humans and to agents how the project works — state machine, ER data model, sequence, flowchart, and more — as Mermaid text drawn with the taste and type range of `cathrynlavery/diagram-design` (F26). Session ends with decisions a build item can carry. | user's words (turns 2, 4, 5, 6) |
| Must not break | The store and its grammar (spine, AX rows, lifecycle, derived index); the breadth invariant (every shelf row in scope is walked somewhere) and floor precedence; architecture truth as the user's ruling; the author≠grader, sound-loop and plan-approval floors; the desk's downstream readers (implement's sufficiency check, feature's growth door, the drift probe); the specify story stage as built at v0.119.0. | lead's guess, confirmed |
| Betting on | Specify's shape — a discovery stage before authoring, closing in a ruled set the seat authors from — transfers to the desk without making a visit heavier than it already is; and one diagram set can serve humans (rendered) and agents (text) without two copies that drift. | lead's guess, confirmed |
| Out of scope | Brainstorm's and specify's own rules; the three-command convergence (its own later session, BACKLOG *Three-command convergence session*); setup, feature, implement; the shelf content (Stage 2 shelves, the missing desktop shelf). | lead's guess, confirmed |
| Size | standard — close to too-big; if the diagram half grows, its fog is handed to a named later session rather than rushed | lead's proposal, confirmed |

Open frame lines: none. Deciding may start.

- F29 — `mochiko-cli rules authoring-architecture-store` (plugin 0.119.0):
  `authoring-architecture-store.health-view-binding`: "The health view is a section of the index
  and no separate artifact exists — five counts, each naming its rows: open rows (walked, no
  stance formed) · stale `not-now` triggers … · fired triggers awaiting routing · orphan elements
  · the drift r[egister]"; `authoring-architecture-store.readability-bar`: "A reader new to the
  repo can place any file or component in the system from the index alone; an index too thin for
  that is regenerated with more of the spine, never padded with hand-written prose";
  `authoring-architecture-store.index-regenerated-every-write` (`class: floor`): the root
  `ARCHITECTURE.md` "is a projection of the store, regenerated on every store write — this skill
  is its single writer." Lead's reading: the store has no artifact for a *visit* — no place today
  where "what this visit decided to focus on" would be written; the readability bar is already
  the "explain to humans and agents" obligation, in prose-and-one-thumbnail form.

## Run notes

- Floor read-back — model tiering, 8: class-key-session-tier · seat-default-key ·
  override-is-the-pin · seat-deviation-bounds · persona-less-grader-pin · worker-return-is-a-claim ·
  brief-obligation · worker-seat-set-reserved.
- Floor read-back — transport floor, 11: message-lane-trigger · topology-lane-trigger ·
  neither-lane-waivable · composition-steer · single-writer-per-surface · mesh-hold ·
  content-pinned-supersession · quiesce-before-cold-grade · no-ritual-sends · fan-in-confirmation ·
  version-floor. Message lane fires (the review seat is resumed later with the record); topology
  lane does not (the lead is the sole writer of this record and of the index entry; seats return
  text or write only their own report file). Claude Code 2.1.294 ≥ 2.1.224 (version floor).
- Session tier: lead on `claude-fable-5-1` (the user's `/model` switch at session open).
- Seats (2026-10-10, at frame hardening):
  - Blind-map review seat `review-seat` — `mochiko:devils-advocate`, persona default tier
    (`opus`), no override; given the problem, destination and out-of-scope lines only, never
    this record or the index; invokes `mochiko:review-brainstorm` for the blind angle map; writes
    only `reports/angle-map.md`. To be resumed with the record for the cold read (message two).
  - Blind second-list seat `second-list-shape` — persona-less `general-purpose`, spawned
    `model: opus` (disclosed: an interpretive seat, pinned to the strong alias; not a grader);
    given the frame and the shape question only, never the lead's options or the record; returns
    text, writes nothing.
  - Every brief carries the model-tiering routing line.
- Fan-in expected: 2 returns (blind map · second list on the shape fork).
- Sibling session `specify-brainstorm-discovery` is accepted and built; its index entry is final.
  `.mochiko/brainstorms/index.md` is no longer a concurrent write surface for this session; this
  session edits only its own entry, re-reading the file before each write.

- Fan-in: second list arrived 2026-10-10 (`research/second-list-shape.md`, eight roads; fence
  held; no Explore spawn — direct greps on the files the brief named). Blind map: pending.

## Decision map — first shown with Q1 (2026-10-10)

**Askable now**
- Q1 — Where the brainstorm-style thinking sits in a desk visit and what it closes in (the shape
  fork; costly to undo; own turn; blind second list taken).

**In fog** (what each hangs on) — half A, how the desk thinks
- The unit the user rules per focus item — the part named, the user's direction in their words,
  and how they would know it is right (mirror of specify's line-plus-test) or `open`; hangs on Q1.
- Who holds the conversation — the lead inline (`specify-brainstorm-discovery` D3-lead-holds-stage and the user's word
  "the lead is the manager of the teammates") or the architect seat; hangs on Q1; likely a default.
- A blind second list of architecture angles per visit (mirror of `specify-brainstorm-discovery` D4-blind-journey-list)
  — on which visits, from which inputs (goal, `Scope:` line, spine, the brainstorm record); hangs
  on Q1 and the size item.
- Visit size — a crisp one-row ask against a greenfield baseline: skip = the ask handed in as the
  ruled set (mirror of `spec.handed-in-slate`), no size gate (specify refused one); hangs on Q1.
- Inputs already in hand — a brainstorm record named at the desk opens the stage as facts (mirror
  of `spec.stage-inputs-in-hand`); the desk's `argument-hint` names no record today (seat fact);
  default after Q1.
- Where the ruled set lives — the brief only · the AX rows' `Ruling`/`Rationale` fields · a new
  store entry kind (the store home declares only `spine.md` and `concerns.md` — seat fact); hangs
  on Q1.
- Mechanisms carried and dropped (mirror of specify's *Mechanisms carried*): frame card → the
  visit goal · record → the store · cold review → tech-lead's grade · decision map kept as the
  focus map · own-turn-or-batch over shelf rows · stall clause · provenance mark per ruling; a
  default batch after Q1.
- A `kind: fail` for a store write (baseline, stance batch, delta) with no ruled set behind it
  (mirror of `spec.fail.story-outside-slate`); default after Q1.
- The brownfield confirm step and the greenfield elicit — whether each becomes the stage's close
  (seat: roads 3 and 5 amend `arch.dm-author-baseline`); hangs on Q1.
- Where the rules live — the desk's own schema with a precedence clause (precedent twice:
  `brainstorm.own-rules-win`, `spec.own-rules-win`); default.

**In fog** — half B, what the desk draws
- Which diagram kinds live at product level in the store, and which stay feature-level (today:
  container + sequence in the store as Mermaid; ER and state machines feature-level as text,
  F21/F24) — its own fork, askable after Q1.
- Where the diagrams live — the spine's sections (30-line budgets, F21) · a `diagrams/` sub-home
  in the store · the derived index; hangs on the kinds fork.
- How diagrams stay true — each diagram cites the store IDs it draws; a store write touching a
  cited element flags it stale in the health view; redraw at the landing fold; the drift probe
  covers them; hangs on where they live.
- Taste — `diagram-design`'s rules written into mochiko's diagram conventions (one accent colour,
  delete first, the paragraph test, a node budget, Before · Changes · After for deltas, F19/F20);
  a default after the kinds fork.
- Rendering for humans — `architecture-design-primitive` D8-rendered-diagram-signoff ("rendered,
  never raw source") against the fact that kinako signed un-rendered (F23, F25) and Mermaid
  renders on GitHub and in IDEs; amends an existing ruling, so the user's own turn.
- Agents as readers — the derived index's readability bar is already the "explain" obligation
  (F29); whether diagrams embed in the index or link from it; hangs on where they live.
- Who draws product-level ER and state machines — the architect, or the technical-analyst whose
  entity skill owns them at feature level; hangs on the kinds fork.
- Two notations for one thing — feature-level text ER/state (F24) and product-level Mermaid;
  convergence would touch `patterns-entity-modeling`; flag — likely handed to a later session.
- What a dogfood run is judged on — last (precedent: `specify-brainstorm-discovery` D7-dogfood-criteria-deferred deferred it to the first run).

**Out of scope** — brainstorm's and specify's own rules · the three-command convergence · setup,
feature, implement · shelf content.

## Question trail

### Q1 — Where does the brainstorm-style thinking sit in a visit, and what does it close in? (put 2026-10-10)

Lead's options, written before the second list was read: (A) a focus stage after the visit goal
converges and before the architect authors anything, closing in a ruled focus set — the parts of
the architecture this visit decides, each with the user's direction — with the rest of the shelf
scope dealt as one confirmed defaults batch, the architect authoring inside the set · (B) deepen
the shelf walk itself — every row dealt as a fork with fact, two options and the case against the
default, real forks on their own turn, the rest in a confirmed batch, no new artifact (cheaper
shape) · (C) replace the one-line goal convergence with a full brainstorm per visit (frame card,
map, forks), closing when the map is empty · (D) nothing new in the desk — a `/mochiko:brainstorm`
record handed in as facts (cheapest). Second list: eight roads. Differences shown to the user: the
seat added status quo (1), draft-first-then-brainstorm (6), a blind list alone (7) and a split by
visit size (8); it split the lead's (A) into a visit slate (3) and a two-tier triage (5), and
flagged on (5) that a focus-first walk "fights retrofit-cost order" and "risks … rubber-stamping on
the rest, which is the quiet narrowing the shelf skill already declined"; its fact that the store
home carries only `spine.md` and `concerns.md`, so a ruled set has no home today. Lead's pick: A
(the seat's 3 + 5 together); cheaper shape: B (the seat's 4).

- Fan-in: blind map arrived 2026-10-10 (`reports/angle-map.md`, 32 angles / 14 load-bearing /
  7 classes, `record_contact: none` — the record's path alone appeared in the harness git-status
  snapshot at seat start, no content; one haiku Explore spawn returned no report, the reads
  redone first-hand; one artifact-home hook deny moved the seat's write from Bash to Write).
  Fan-in complete: 2 of 2.
- Record edits: the artifact-home hook denied a shell append to this file after the fold was
  composed (the earlier appends had passed its best-effort parse); every later edit goes through
  Write/Edit, as the sibling session's did.

## Decision map — blind map folded (2026-10-10, before Q1 is ruled)

Load-bearing angles folded, each marked `(blind map N)`; the map's numbers are the seat's local
labels, never this record's decision ids. Q1 stands unruled; the angles bearing on it are shown
to the user before the ruling.

**Bearing on Q1**
- Focus against the breadth invariant: is "the part to focus on" an ordering-and-depth choice
  inside the full walk, or a selection that needs a recorded supersession of two floors
  (`arch.breadth-invariant`, `patterns-architecture-shelves.breadth-invariant`); and does breadth
  bind once per baseline or on every visit `(blind map 1)`.
- Who picks the focus, from what — `$ARGUMENTS`, the health view (a floor that runs before the
  ask), the spec, the map, code; a desk-picked focus is a narrowing, so the focus is a guess the
  user rules, and health items may seed it or crowd out the user's own demand `(blind map 2)`.
- Two done conditions in one visit — the desk closes on a one-line goal (`arch.dm-converge-goal`,
  enforced by `arch.fail.no-verdict`); brainstorm closes on an empty map; specify's stage closes
  when nothing is open — the stage's close must sit inside the visit's done condition
  `(blind map 8)`.
- The cheaper road, do nothing at the desk — route architecture thinking to
  `/mochiko:brainstorm` and add only a handed-in door, as specify did; weighed by name as Q1's
  option D `(blind map 6)`.
- In-visit state — the desk has no session artifact; store writes land only as graded, ruled
  batches, so an interrupted baseline visit loses its map and rulings in progress; where in-visit
  state lives is a new home or the brainstorm home, either a migration `(blind map 15)`; joins the
  fog item *where the ruled set lives*.
- Visit kinds differ in kind — greenfield elicits; brownfield is a confirmation wall over an
  `Assumed` derivation; a drift finding takes a closed three-way disposition; fired triggers route
  out; "every visit" may not mean every kind `(blind map 14, not load-bearing, folded)`; joins the
  fog item *the brownfield confirm step*.

**In fog — half A, additions and marks**
- A feature's delta demand against implement's design phase: nothing routes a feature's
  structural need to the desk today; implement's design phase writes the `proposed` elements and
  the signed drawing; what a feature-delta visit produces that the design phase consumes, and
  through which read `(blind map 3)` — new item; hangs on Q1.
- Off-shelf concerns: kinako's 33 AX rows are mostly product-specific, not the 13 shelf
  dimensions; how off-shelf concerns enter, and whether the blind list becomes their door
  `(blind map 9)` — joins the blind-list item.
- Outside claims: who live-checks a load-bearing outside claim (provider limits, pricing) before a
  stance lands — the tech-lead grader rule carries no grading-skill pointer `(blind map 11)` — new
  item; hangs on the mechanisms batch.
- Streak calibration: a 13-row walk where most defaults fit makes a ratified streak the normal
  case; brainstorm's three-in-a-row flag would fire constantly; telling informed acceptance from
  fatigue `(blind map 18)` — joins the mechanisms item.
- Provenance in the store: an AX row carries Ruling and Rationale only, so "defaults plus guesses"
  cannot be seen or measured; a provenance field is a template migration `(blind map 19, folded)`
  — joins the mechanisms item.
- Size per visit, never from the governance dial (`arch.no-depth-dial-coupling` is a floor); each
  added seat multiplies cost `(blind map 13, folded)` — joins the size item.
- Who holds the conversation `(blind map 5, folded)` — marks the existing item.

**In fog — half B, additions and marks**
- ER and state-machine ownership: the product data model lives at `.mochiko/product/data-model.md`,
  a baseline written in place by implement's design phase, and state machines belong to
  `patterns-entity-modeling`; a desk-drawn ER or state diagram is either a second home that
  drifts or a desk write into a baseline it does not own `(blind map 22)` — marks and sharpens the
  *who draws* and *two notations* items; fact to verify: the product home's declared files.
- Earning rule per diagram type: "One diagram, not a gallery" (spine template) and the denied
  undeclared `##` headings; without a per-type earning rule and the plan-minimalism ladder the
  set sprawls `(blind map 23)` — joins the kinds item.
- Upkeep: who redraws when a feature lands — implement's store landing or the next desk visit;
  whether a diagram is an as-built claim the drift probe grades; a stale diagram misleads agents
  more than a missing one `(blind map 24)` — marks the *stay true* item.
- Taste into Mermaid is an untested bet: the repo's rules are SVG-level; which survive Mermaid's
  `themeVariables` and `classDef` and GitHub's renderer; which of its 44 types have a Mermaid
  carrier; renderer limits memory-asserted, a live check owed `(blind map 26)` — marks the taste
  item; fact work owed before its default.
- Derived by code or hand-drawn: diagrams generated from the element table would be kernel-class
  tooling (admitted only by recorded ruling); hand-drawn ones are judgment writes under the sound
  loop; whether diagrams ride into the regenerated index `(blind map 25, folded)` — joins the
  *where they live* item.
- Render verification: no renderer ships; a validator in `mochiko-cli` would need its own
  admission `(blind map 27, folded)` — joins the rendering item.
- Readers: which agent reads which diagram and at what token cost; dark-mode colours and
  accessibility titles for humans `(blind map 28, folded)` — joins the readers item.
- One build or two: the two halves have different owners, homes and unknowns; bundled, (a) can
  stall on (b) `(blind map 32, folded)` — new item, last, with the dogfood criteria.
- Evidence: kinako's rows read 26 decided · 7 open · no `not-now` or `n-a`; what the stores show
  about defaults-plus-guesses, and what observation after the build shows tighter rulings
  `(blind map 30)` — marks the dogfood-criteria item.

**Dropped from the map, listed for the end read** (not load-bearing per the seat): 4 the overlap
of the options rule with the recommend-then-arbitrate floor (noted against Q1's option B) · 7
convergence cost later · 10 show-before-asking sketches seeding maintained diagrams · 12 a second
cold review per visit · 16 non-backend surfaces have no shelf · 17 wants versus technology, "you
pick" as delegation · 20 rationale's home and a record per visit · 21 re-opening ruled rows · 29
convention collisions (the queued trust-boundary convention, BACKLOG.md:907) · 31 downstream
contracts and gates.

- F30 — `mochiko-cli home --plugin-root plugins/mochiko .mochiko/product` (plugin 0.119.0):
  deliverables `data-model.md` · `constraints-and-decisions.md` · `quickstart.md` (entries, one
  `###` per entry); sub-directories `architecture`, `contracts`, `design`. Kinako carries all
  three files (`../kinako/.mochiko/product/data-model.md` line 1: "# Data model — product
  baseline", "Reconstructed 2026-08-22 from delivered code"); ai-fileops carries only the
  `architecture` and `design` sub-directories. Confirms blind map 22: the product-level data
  model has a home and an owner outside the architecture store.

**Q1 ruling.** The five Q1-bearing blind-map angles were shown first. User: "A, as recommended" —
ratified (streak: 1) → D1-focus-stage-first.

## Decisions

### D1-focus-stage-first — A focus stage after the visit goal, before the architect authors — `Confident`

**Statement.** `/mochiko:architecture` gains a focus stage. It runs after the visit's one-line
goal is agreed and before the principal-architect seat authors anything — baseline, stance batch,
amendment or delta. In it the lead and the user brainstorm which parts of the architecture this
visit decides, the way brainstorm works decisions: forks one per turn, two real options of which
one is the do-less shape, the fact a fork hangs on in hand first. It closes in a ruled **focus
set**: each part named (a store element, an AX row, a shelf row or a spine area) with the user's
direction in their words. The rest of the shelf scope in play is dealt as one defaults batch —
every row shown with its suggested default and confirmed on a single screen, never applied by
silence — so every row in scope is still walked: focus is depth and order inside the full walk,
never a skip (blind map 1; `arch.breadth-invariant` stands). The focus is a guess the desk proposes
and the user rules, never a desk-picked narrowing (blind map 2). The architect seat plans and
authors inside the ruled set; an element off the set goes to the user before it is authored. The
stage's close sits inside the visit's done condition, not beside it (blind map 8); the visit goal
stays the first agreed screen.

**Why.** The user's destination: the desk "help[s] user brainstorm the part of architecture they
need to focus on depending on the context" (turn 2). The specify precedent built the same shape
the same day (F9–F17), and the desk's two gaps — no mechanism that picks a focus from context, no
ruled set binding the architect — are the ones the stage fills. The breadth invariant is kept by
the batch, which is brainstorm's own own-turn-or-batch rule applied to shelf rows.

**Rejected roads.** (B) deepen the walk itself, no new artifact — keeps every visit long and
picks no focus from context; it also amends the shelf skill's own recommend-then-arbitrate floor
(blind map 4, second list road 4). (C) a full brainstorm per visit — a six-line frame card on a
one-row ask. (D) nothing new at the desk, a `/mochiko:brainstorm` record handed in — two commands,
a record that is not store-shaped, small visits gain nothing (blind map 6, second list road 2).
Second-list roads not taken: 1 status quo (depth left to whichever lead runs the visit); 6 draft
first then brainstorm at the gate (anchors the user on the architect's guesses — the failure the
frame names); 7 a blind list alone (does not deepen the questioning); 8 a size gate (specify
refused one).

**Accepted risk.** A new ruled artifact with no home in the store today (second-list fact; blind
map 15) — ruled at the *where the ruled set lives* item. The defaults batch can turn into a rubber
stamp (second list road 5's warning) — the batch screen shows every row with its default and its
cost, and a row the user pulls gets its own turn. Three agreed screens per visit — goal, focus
set, rows (S2; repaired at verify V6).
What a feature-delta visit produces for implement's design phase is still open (blind map 3).

**How it was decided.** Q1, its own turn; the blind second list taken
(`research/second-list-shape.md`) and its differences shown; the five blind-map angles bearing
on it shown; the case against the pick given; user: "A, as recommended" — ratified (streak: 1).

**Changed at review (S10, S11, S13; user: "ok" on the batch and the coverage paths, S13 on its
own turn, 2026-10-11).** (S13) The stage, the visit file and the blind list fire only on a visit
that rules something — a baseline, a stance, an amendment, a feature's delta thinking; a store
query writes nothing; a drift visit records its dispositions on the rows as today, with no
stage, no file and no list; a fired trigger's routing writes nothing. The user's "every visit"
(F27) is narrowed by the user's own word. (S11) The defaults batch covers only rows with no
stance yet — `open`, or newly in scope; a `decided`, `not-now` or `n-a` row is never re-dealt by
a batch and changes only as a focus item the user pulls in: an amendment demand, a fired
trigger, a drift finding. (S10) Focus items and batch rows are each walked in retrofit-cost
order, and each batch row shows its suggested default, what it costs and what would argue
against it — `arch.breadth-invariant`'s order clause and the shelf skill's
recommend-then-arbitrate floor stand unamended; the second list's road-5 warning is answered by
that, not dropped.

## Decision map — reshown after D1-focus-stage-first (2026-10-10)

**Decided.** D1-focus-stage-first.

**Askable now** (dependency order)
- Q2 — The unit the user rules per focus item (real choice; unblocks the home item and the
  mechanisms batch).
- Q3 — Where the ruled focus set and in-visit state live (costly to undo — a migration either
  way; hangs on Q2).
- Q4 — The blind list of architecture angles: on which visits, from which inputs (a seat per
  visit; real choice).
- Q5 — A feature's delta demand: what a desk visit produces that implement's design phase
  consumes (blind map 3; real choice).

**Defaults — named as they arise, confirmed in one batch at wrap-up** (say "pull X" to open any
on its own turn): who holds the conversation (the lead, inline) · mechanisms carried and dropped
· a `kind: fail` for a store write with no ruled focus set behind it · the brownfield confirm and
the greenfield elicit run as the stage over the derivation · no size gate, a crisp ask handed in
as a focus set of one · a brainstorm record named at the desk opens the stage as facts · rules in
the desk's own schema with a precedence clause · outside claims live-checked before a stance
lands · the ratified streak counted over forks, never over batch rows · provenance per focus item.

**In fog — half B** unchanged (kinds · where they live · stay true · taste, fact work owed ·
rendering, the user's turn · readers · who draws ER and state · two notations · one build or two
· dogfood criteria, last).

**Out of scope.** Unchanged.

### Q2 — What unit does the user rule per focus item? (put 2026-10-10)

Hangs on D1-focus-stage-first; unblocks Q3 (what the home must hold) and the mechanisms
batch. Options: (a) the part and the user's direction only (cheaper shape) · (b) the part, the
direction, and one sentence on how the user would know it is right — the consequence they would
see, or the trigger that would reopen it — or `open` when they have none yet (mirror of specify
`specify-brainstorm-discovery` D2-line-plus-test; the sentence feeds the AX row's upgrade trigger or target) · (c) the full
stance ruled live — stance word, ruling, rationale, trigger, per row. Lead's pick: (b); case
against: an unsure user has no such sentence, so `open` must be legal and carried as an open
question, never invented. A one-item mock was shown in the message (show-before-asking).
**User: "as recommded"** — ratified (streak: 2) → D2-part-direction-know.

### D2-part-direction-know — The user rules the part, their direction, and one "how I'd know" sentence — `Confident`

**Statement.** In the focus stage the unit the user rules per focus item is three things: the
part named (a store element, an AX row, a shelf row or a spine area) · the user's direction for
it, in their own words · one sentence on how they would know it is right — the consequence they
would see, or what would make them reopen it — or `open` when they have none yet. The sentence
is the seed of the row's upgrade trigger or target; the architect seat expands it without change
of meaning, and the stance word, rationale and as-built fields are authored afterwards, inside
the ruled direction. An `open` sentence is carried as an open question on the row, never
invented by the seat.

**Why.** A direction alone leaves the architect free to read it several ways — the looseness the
problem names, one layer down; a full stance ruled live moves the row volume into the
conversation. Specify ruled the same shape for stories the same day (`specify-brainstorm-discovery` D2-line-plus-test, F10), and
the "how I'd know" sentence maps onto fields the AX row already has (trigger, targets).

**Rejected roads.** (a) part and direction only — cheaper, the meaning stays open to the seat's
reading. (c) full stance ruled live — every row a long turn.

**Accepted risk.** More words per item than (a). An unsure user has no sentence; `open` must be
legal, and a stage that pushes for one is back to guessing.

**How it was decided.** Q2, its own turn, three options with the cheaper shape named, a one-item
mock shown, the case against the pick given; user: "as recommded" — ratified (streak: 2).

**Changed at review (S19, batch; user: "ok", 2026-10-11).** Where the know-sentence lands: in
the visit file always; on a `not-now` row as its upgrade trigger; on a `decided` row as the first
line of its `Rationale`; a target with a number only when the user gives the number — never
derived from the sentence. The earlier "trigger or target" read as if a sentence could become a
measurement on its own.

#### Facts for Q3 (2026-10-10)

- F31 — `mochiko-cli home --plugin-root plugins/mochiko .mochiko/runs/sample-run`: the `runs`
  home `.mochiko/runs/<run-id>/` is "for raw output (captured console, logs, dumps) … which is
  ephemeral — deleted at the run's acceptance, never a record"; `.mochiko/` is closed — "no
  declared home governs this path … a write here is refused". Twenty-eight declared homes, among
  them `feature-desk · .mochiko/features/desk/<date-slug>/` and `product-concerns ·
  .mochiko/product/architecture/concerns/`.
- F32 — `mochiko-cli home --plugin-root plugins/mochiko .mochiko/features/desk/2026-10-10-sample`:
  "home: feature-desk — One specify-desk sitting · directory: `.mochiko/features/desk/<date-slug>/`
  · deliverables: `derivation.md` · no template · 300 lines, whole file · reports:
  `.mochiko/features/desk/<date-slug>/reports/`". Kinako has two sittings
  (`../kinako/.mochiko/features/desk/2026-09-09-sandbox-runner`, `…/2026-09-19-feat-001-r1-4-and-b69`),
  each with `derivation.md`. Lead's reading: the feature desk already keeps one directory per
  sitting; the architecture desk has none — the precedent for a per-visit artifact exists at the
  sibling desk.
- F33 — `mochiko-cli rules authoring-architecture-store --section
  authoring-architecture-store.sec.artifact`: `authoring-architecture-store.one-home-one-writer`
  (`class: floor`): "One home. One writer. The product's architecture lives at
  `.mochiko/product/architecture/`, and every write to it goes through this skill."
  `authoring-architecture-store.present-tense-no-history`: "A row's own `Ruling` and `Rationale`
  belong on the row; rationale already living in a decision record or landing report is linked,
  never restated." Kinako's `concerns.md` carries 21 cites of `DECISIONS.md` or a brainstorm
  record. Lead's reading: the store already expects rationale to live in a record it links to; a
  visit file would be such a record.

### Q3 — Where do the ruled focus set and the in-visit state live? (put 2026-10-10)

Hangs on D1-focus-stage-first and D2-part-direction-know; unblocks the provenance default,
the fail-condition default and the hand-off default. Options: (a) nowhere but the architect's
brief — no file, the rulings reach the store only through the architect's graded write (cheapest;
an interrupted visit loses its state; "defaults plus guesses" stays invisible — blind map 15, 19)
· (b) straight into the store rows — the lead transcribes each ruling as it lands into
`concerns.md`/`spine.md` (stance, the user's words as the ruling, the "how I'd know" seed), the
architect elaborates afterwards; no new file, but a provenance field on the rows (a template
migration), a pen hand-off rule, and no home for the in-progress map · (c) a visit file, the
feature desk's shape (F32): `.mochiko/product/architecture/desk/<date-slug>/` with one `focus.md`
— the goal and done condition, the focus set with provenance, the defaults batch, the blind list's
differences and their rulings, the verdict — written as the visit goes, the rationale home the
rows link to (F33), the architect's brief pointing at it; one new home and template (one
migration). Lead's pick: (c); cheaper shape: (a).

Trail: the user asked for plainer words (re-put); then "if c, how will they get reflected in
main files? why not b vs c?" — answered: the architect authors the rows from the visit file at
the set's close, in the same visit, tech-lead grades the rows against the file for sameness of
meaning, each row links to the file for its why; (b) is closer to specify's own shape (the slate
in the spec), (c) to the feature desk's. Then "with version control, git and github, do we still
have same concerns" — answered: git mostly covers the half-done visit (the desk never commits,
the user does), already carries history (the store's present-tense rule leans on it), and does
not carry per-row provenance, the in-visit thinking, or two writers in one working tree.
**User: "okay c"** — the pick taken after two probes (streak: 3) → D3-visit-file-home.

### D3-visit-file-home — The focus set and the visit's state live in one visit file beside the store — `Confident`

**Statement.** Each visit that rules something (S13) gets its own directory beside the store and
outside `.mochiko/product/` (S5; title and statement repaired at verify V3), in the feature desk's
shape (F32): a home the build names outside `.mochiko/product/` — working path
`.mochiko/architecture/desk/<date-slug>/` — with one deliverable (working
name `focus.md`; the build names it) and a `reports/` sub-directory. The file holds the visit's
goal and done condition, the focus set — each item with the user's direction, the "how I'd know"
sentence or `open`, and its provenance (`user's words` · `lead's guess, ratified` · `blind list` ·
`handed in`) — the defaults batch as confirmed, the blind list's differences and their rulings,
and the closing verdict. The lead writes it as the visit goes (transcription of the user's
rulings — no sound loop). At the set's close the principal-architect seat authors the store rows
and spine elements from it in the same visit — stance, the user's words as the ruling, the
sentence landed where D2-part-direction-know's changed-at-review says (the trigger on a
`not-now` row, the first rationale line on a `decided` one; V6), the rationale it adds — and
tech-lead grades the rows
against the file for sameness of meaning; each row links to the file as its rationale home
(`present-tense-no-history`, F33). The visit does not close until the rows are in. The store
stays one home, one writer per file: the lead holds the visit file's pen, the architect the
store's.

**Why.** The desk is a standing ledger visited many times; per-visit files keep the thinking and
the provenance out of the rows while the rows stay present-tense. The feature desk already keeps
one directory per sitting, so the shape exists. Git covers the half-done visit and the history,
and does not cover provenance, the in-visit thinking, or two writers in one tree (the user's
question, answered in the trail).

**Rejected roads.** (a) the architect's brief only — state loss on an interrupted visit, no
provenance. (b) straight into the store rows — specify's own shape, workable with git, but it
needs a provenance column on every row, a pen hand-off inside one file, and still gives the
open forks, the batch and the blind list's differences no home.

**Accepted risk.** One new home and template (a migration); one directory per visit; rulings sit
in two places between the set's close and the rows' write — bounded by the same-visit rule and
the tech-lead sameness grade.

**How it was decided.** Q3, its own turn, three options with the cheaper shape named, re-put in
plain words at the user's ask, two user probes answered (how (c) reaches the rows; whether git
removes (b)'s concerns); user: "okay c" — ratified after probing (streak: 3).

**Changed at review (S5; own turn; user: "as recommded", 2026-10-11).** The visit directories
move out of the store and out of `.mochiko/product/` altogether (V3): a new home the build
names — working path `.mochiko/architecture/desk/<date-slug>/` —
as the feature desk's `.mochiko/features/desk/` sits beside the map's truth files. Inside the
store they collided with `arch.single-writer-store` and the `one-home-one-writer` floor, and a
run paused for a desk visit (D5-visit-file-input) would have found the file under `.mochiko/product/` as an
unmarked hunk (`impl.fail.unmarked-baseline-write`). The rows still link to the file; the lead
holds its pen, the architect the store's. Rejected: two floor amendments plus a run-pause
exception; forbidding a desk visit while a run is open.

**Changed at review (S2, batch; user: "ok", 2026-10-11).** The desk's existing user gate on the
architect's rows stands: after tech-lead's grade, the stance batch, baseline or amendment lands
only on the user's word (`arch.user-reserved-rulings`, `arch.truth-user-ruling`,
`arch.sound-loop-floor` — none superseded), and the user sees the concrete row text there — the
chosen provider, the tenancy model. The sameness grade is additive to that gate, never a
substitute. A visit therefore has three agreed screens — the goal, the focus set, the rows —
and the "not heavier" bet carries that.

### Q4 — The blind list of architecture angles: on which visits, from which inputs? (put 2026-10-10, case-against-first form — the ratified streak stands at 3)

Hangs on D1-focus-stage-first and D3-visit-file-home; unblocks the off-shelf-concerns
item (blind map 9) and the size default. Mirror of `specify-brainstorm-discovery` D4-blind-journey-list: a fresh
persona-less seat, `model: opus`, gets the visit goal, the `Scope:` line, the store's index
(spine thumbnail, AX summary, health) and any handed-in record — never the focus set or the
conversation — and returns the parts, concerns and forks it sees; each difference from the focus
set is a candidate item the user rules in or out. Options: (a) every visit, one list (a seat per
visit, the transport floor on every visit; a one-row ask pays a seat) · (b) every visit except a
single-row ask — when the visit's goal names one row and the user hands in its direction, the
set is one item and the list is skipped (a size-gate-shaped exception, which specify refused for
its forks; its handed-in path still runs the list) · (c) on request only (the user who most
needs it does not know what is missing — specify's rejection) · (d) no list — the shelf's breadth
and the architect's recommendation carry completeness (cheapest). Lead's weak lean: (b).
**User: "as recommded"** (2026-10-11) — ratified on a weak lean, put case-against-first
(streak: 4) → D4-blind-list-visits.

### D4-blind-list-visits — One blind list per visit, skipped only on a single-row ask — `Confident`

**Statement.** Before the focus set closes, the lead dispatches one fresh persona-less
`general-purpose` seat spawned with an explicit `model: opus` (an interpretive dispatch, never
haiku), given the visit goal, the store's `Scope:` line, the derived index (spine thumbnail, AX
summary, health view) and any handed-in record — never the focus set, the conversation or the
visit file — and it returns the parts, concerns and forks it sees for that visit. Each difference
from the focus set is a candidate item the user rules in or out, carrying provenance `blind
list` in the visit file; a candidate the shelf does not carry is how an off-shelf concern enters
(blind map 9). One list per visit. The one exception: a single-row ask — the visit goal names one
row and the user hands in its direction, so the focus set is one item — skips the list; every
other visit, a baseline or any multi-part focus set, takes it.

**Why.** The user who most needs the list does not know what is missing (`specify-brainstorm-discovery` D4-blind-journey-list's reasoning,
F12), and kinako's ledger shows real concerns live off the shelf (33 rows against 13 shelf
dimensions — blind map 9). The desk also promises a crisp ask converges in one exchange (F1), and
a seat on a one-row stance question breaks that promise for no gain: the user already knows the
row.

**Rejected roads.** (a) every visit — a seat and the transport floor on a one-row ask. (c) on
request — the user who needs it would not ask. (d) no list — completeness left to the shelf's
breadth and the architect's recommendation, the guess surface the frame names.

**Accepted risk.** The exception is a size gate in another word — the thing specify refused for
its forks; here it is keyed on the ask's shape (one named row, direction handed in), never on a
size the user rules, and it skips the list, never the focus set's close. A seat per multi-part
visit. Late candidates can re-inflate the set; the do-less option on each keeps the default at
out.

**How it was decided.** Q4, its own turn, in the case-against-first form after three ratified
rulings, four options with the cheapest named, the lead's doubt stated; user: "as recommded" —
ratified (streak: 4).

**Changed at review (S12 coverage, S22 batch; user: "ok", 2026-10-11).** (S12) The blind seat's
inputs, and what the lead reads to propose a focus, are bound per visit: the visit goal · the
store's `Scope:` line · the derived index (spine thumbnail, AX summary, health view) · any
brainstorm record named · `FEATURES.md` and the capability entries in the visit's territory ·
the codebase analysis where one exists (brownfield) · for a feature's delta visit, the feature's
spec — never the focus set, the visit file or the conversation. On a greenfield baseline the
index is a scaffold, so the product's own description and the capability map carry the context.
(S22) The mark stays `Confident` for a stated reason: the case against the lead's lean was put
first and the user ruled on it explicitly; the lead's own doubt is recorded in How-it-was-decided
and is not the user's. The batch mitigation this card leans on — a row the user pulls gets its
own turn — is itself untested: this session's sixteen-row batch drew no pull, and "pulls per
batch" joins the untested bets.

#### Facts for Q5 (2026-10-11)

- F34 — `mochiko-cli rules implement` (plugin 0.119.0): `impl.design-seats-staffing`:
  "principal-architect for the store's `proposed` elements and the run's architecture.md
  drawing"; `impl.design-outputs-home`: design outputs land "in place in the product baselines …
  and the store's `proposed` elements where the structural trigger fired. The run's home keeps
  only architecture.md, the drawing the user signs"; `impl.absent-surfaces`: "A store with no
  ruled content: offer the /mochiko:architecture bootstrap" — the only place implement routes to
  the desk; `impl.midrun-refire`: undesigned structure found mid-run re-fires the design phase.
- F35 — `mochiko-cli rules review-sufficiency`: `review-sufficiency.lifecycle-marker-read`:
  "`proposed` is nothing promised and reads absent — never cited as planned contract";
  `review-sufficiency.clause-structural-trigger`: "Spine elements keyed to the row's own feature
  or epic, `proposed` or in the in-flight class, are excluded from the no-delta evidence: a row
  never satisfies this clause with its own planned structure." Lead's reading: a desk visit that
  wrote a feature's delta as `proposed (<FEAT>)` elements would change nothing at implement —
  the design phase would still fire over the same gap; only ruled store content counts as
  guidance there, and how the check reads a `ruled` element not yet built is not stated in the
  rendered clauses (a gap the build would have to close if that road were taken).

### Q5 — A feature's delta demand at the desk: what does the visit produce for implement? (put 2026-10-11, weak-lean form — the streak stands at 4)

Hangs on D1-focus-stage-first and D3-visit-file-home; unblocks nothing else in half A.
Today a feature's structural change is designed inside implement's design phase by the
principal-architect, written as `proposed` elements and signed at the checkpoint (F34); nothing
routes it to the desk, and a `proposed` element written at the desk would read absent to the
sufficiency check (F35). Options: (a) feature deltas stay in implement's design phase; the desk
is not a stop for them and "every visit" means the desk's own kinds — baseline, stance,
amendment, drift (cheapest; the design phase's own guess surface stays as it is, implement being
out of scope) · (b) the desk rules the delta as a store amendment — the focus stage rules the
delta's parts, the architect writes them as ruled elements, not `proposed (<key>)` — and the
design phase then finds the store sufficient (costly to get wrong: how `ruled`-not-built elements
read at sufficiency and flip at landing is unstated, F35; touches the store lifecycle and
implement's reads) · (c) the desk visit produces only its visit file, keyed to the feature, and
implement's design phase reads it as a design input — the architect drafts the `proposed`
elements inside the user's ruled focus set (one new input line in implement — a frame widening
that takes the user's word; the lifecycle untouched; the desk writes nothing to the store for a
feature delta, keeping `arch.no-delivery-harness`) · (d) defer the feature-delta kind to the
three-command convergence session, since any road past (a) touches implement (a `Deferred`
card). Lead's weak lean: (c), with the frame widened by that one line; (d) if the user would
rather not widen.
**User: "as recommended. What file will implement read? and what happens if implement needs
architecture that doesnt exists?"** (2026-10-11) — ratified on a weak lean (streak: 5) →
D5-visit-file-input; the two questions answered in the next message and folded into the
card's statement.

**Frame change (the user's word, 2026-10-11).** Out of scope: "setup, feature, implement" →
"setup, feature; implement except one design-input line (the design phase reads a desk visit
file keyed to the feature) and the offer default named below". Stamped here. Re-check of the
decisions already made against the changed line: D1–D4 concern the desk alone — consistent.

### D5-visit-file-input — A feature-delta visit hands implement its visit file; the desk writes no store delta for it — `Confident`

**Statement.** A feature's structural need may be thought through at the desk before the build:
`/mochiko:architecture <FEAT-ID>` runs the focus stage and closes in a visit file keyed to the
feature (D3-visit-file-home — the directory slug carries the joined FEAT id, and the file's header
names it). The desk writes nothing to the store for a feature delta; `proposed` elements stay
implement's (F34, F35), and `arch.no-delivery-harness` stands. Implement's design phase gains one
input line: where a desk visit file keyed to the feature exists, it is a design input beside the
sufficiency report — the principal-architect drafts the `proposed` elements and the signed
drawing inside the user's ruled focus set, and a structural need the set does not cover is put
to the user at the checkpoint as a candidate (ruled there, or the run pauses for a desk visit),
never drafted silently. Where no visit file exists, implement runs as today: the sufficiency
check names the gap, the design phase fires over it, the user signs at the checkpoint; structure
found mid-run re-fires the design phase (`impl.midrun-refire`); a store with no ruled content
still offers the desk bootstrap (`impl.absent-surfaces`). The feature's capability entry gains a
link to the visit file at the visit's close (the entry's architecture link,
`impl.design-map-assertion`, is how the design phase finds it; the slug is the fallback).

**Why.** The user's destination names every visit, a feature's delta demand among them (F27); the
only road that reaches implement's guess surface without touching the store lifecycle is a
handed-in input, the shape specify chose for brainstorm records (F15). A `proposed` element
written at the desk would read absent at sufficiency (F35), so road (b) buys nothing without a
lifecycle change.

**Rejected roads.** (a) leave it — the desk never a stop for feature changes; the design phase's
guess surface untouched. (b) the desk rules the delta as ruled elements — how `ruled`-not-built
elements read at sufficiency and flip at landing is unstated; touches the store lifecycle and
implement's reads. (d) defer to the convergence session — the user chose to widen by one line
instead.

**Accepted risk.** One implement rule in the build (a frame widening, the user's word), and a
second — the offer — carried as a default: when the sufficiency check names a structural gap and
no visit file exists, the design phase offers a desk visit first or proceeds as today, the
user's choice at run time. Two places now describe a feature's architecture thinking (the visit
file, then the signed drawing) — bounded by the drawing being drafted inside the file's ruled
set and the checkpoint candidate rule.

**How it was decided.** Q5, its own turn, in the weak-lean form (streak at 4), four options with
the cheapest named and the frame touch flagged; user: "as recommended" plus two questions —
which file implement reads, and what happens when implement needs structure that does not exist
— answered and folded into the statement (streak: 5).

**Changed at review (S17, S20, batch; user: "ok", 2026-10-11).** (S17) The desk writes no link
on the feature's capability entry — the two desks never write each other's truth; implement's
design phase fills the entry's architecture link when it reads the visit file
(`impl.design-map-assertion`), and the FEAT id in the directory slug finds the file until then.
(S20) A visit file has no freshness of its own: the design phase reads it against its own
pinned base, and where the store has moved past the file since the visit, the conflicts are
listed at the checkpoint for the user to rule.

## Decision map — reshown after D5-visit-file-input (2026-10-11)

**Decided.** D1-focus-stage-first · D2-part-direction-know · D3-visit-file-home ·
D4-blind-list-visits · D5-visit-file-input.

**Askable now — half B, dependency order**
- Q6 — Which diagram kinds the desk builds and maintains, and where their truth lives (the
  diagram half's shape fork; costly to undo).
- Q7 — Where the diagrams live and how they stay true (hangs on Q6).
- Q8 — Rendering for humans: the standing `architecture-design-primitive` D8-rendered-diagram-signoff ruling against Mermaid
  text in git (amends an existing ruling — the user's turn; hangs on Q6).

**Defaults — half A, named, confirmed in one batch at wrap-up** (say "pull X" to open one): who
holds the conversation (the lead, inline) · mechanisms carried and dropped · a `kind: fail` for a
store write with no ruled focus set behind it · the brownfield confirm and the greenfield elicit
run as the stage over the derivation · no size gate, a crisp ask handed in as a focus set of one
· a brainstorm record named at the desk opens the stage as facts · rules in the desk's own schema
with a precedence clause · outside claims live-checked before a stance lands · the ratified
streak counted over forks, never over batch rows · provenance per focus item · **the implement
offer** (a structural gap with no visit file: offer a desk visit first, or proceed as today —
the user's run-time choice).

**In fog — half B** after Q6–Q8: taste (fact work owed — which of the repo's rules survive
Mermaid and GitHub's renderer) · readers (embed or link; token cost) · who draws product-level ER
and state · two notations for one thing · one build or two · dogfood criteria, last.

**Out of scope.** As changed above.

### Q6 — Which diagram kinds does the desk build and maintain, and where does each one's truth live? (put 2026-10-11, case-against-first form — the streak stands at 5)

Hangs on D3-visit-file-home (the visit file is where a diagram would be ruled) and on the facts
F21, F24, F30; unblocks Q7, Q8, the taste and readers items, and the two ownership items. Today
the store carries two kinds — one C4-container flowchart and sequence diagrams for qualifying
flows, both Mermaid, both line-budgeted (F21); ER and state machines live as text in the product
data-model baseline, owned by implement's design phase through `patterns-entity-modeling` (F24,
F30); no flowchart or process view exists anywhere. The named repo's own test for any diagram:
"would a reader learn more from this than from a well-written paragraph? If no, don't draw"
(F19). Options: (a) keep the store's two kinds; ER and state pictures are drawn where their truth
lives — the data-model baseline gains a Mermaid `erDiagram` and `stateDiagram-v2` beside each
entity and state table (a `patterns-entity-modeling` and data-model template change — a frame
touch on implement's design-phase skill) · (b) the store gains a diagram set — each diagram one
file with one Mermaid block, a one-line caption, and a `Draws:` line naming the store and
baseline IDs it draws; kinds open, each earning its place by the paragraph test and the
plan-minimalism ladder; the container view and the sequences stay in the spine; an ER or state
view draws the data-model baseline's entities by ID and is flagged stale when they change (a view
over another home's truth, blind map 22) · (c) every diagram a derived part of the root
`ARCHITECTURE.md` index — redrawn by the store skill on every write from the store and the
baselines; one file for humans and agents, no new home; every write redraws every diagram
(cost), and the spine's budgets stay. Lead's weak lean: (b); cheapest: (a) or do nothing.

**User steers (2026-10-11), before ruling Q6:** "staying with the previous question, should we
just retire architecure then? And bring the architecture brainstorming and other related
brainstorming with implement?" — a destination-level question raised by the user's own word
(`brainstorm.user-steers`); Q6 held; the lead's assessment put as a fork (Q5b) with the case for
and against retiring the desk, plainly. Facts in hand: the desk's standing duties that implement
has no counterpart for — health view, baseline authoring outside a feature, the breadth walk over
cross-cutting concerns, the drift probe, amendments outside a feature, trigger routing (F2, F3,
F29, the `arch.dm-*` floors); implement's design phase and store landing already carry the
per-feature architecture work (F34); the desk was created for "the kinako-felt gap — no place to
say anything about auth/logging/billing outside a feature run" (`product-architecture-schema`
index entry, read at orientation); the three-command convergence item already in BACKLOG.
Options put: (1) keep the desk, D1–D5 stand, the convergence session later makes the focus stage
shared · (2) keep the desk and widen the convergence item now — implement's design phase runs the
focus stage itself when no visit file exists; one BACKLOG line, no change to this session's
decisions (lead's pick) · (3) retire the desk — stop the session, reopen the frame, plan where
each standing duty goes. **User: "i like the idea of thinking engine."** — read by the lead as
(2); a one-word confirm asked with Q6 re-put (vague-answer rule); recorded as a card on the
confirm. The user asked for both in simpler words ("i am confused ask question again in simople
language") — both re-put as one-word questions. **User: "question 1, yes and question 2, B
architecture.md is just index pointing to digagrams and summarizing them"** (2026-10-11) → the
confirm is D6-desk-stays-engine (ratified, streak: 6); Q6 is D7-diagram-set-index,
with the user's own addition on the index (the user's design move — streak reset: 0).

### D6-desk-stays-engine — The desk stays; the focus stage becomes one shared thinking stage later — `Confident`

**Statement.** `/mochiko:architecture` is kept, and D1–D5 stand. The focus stage is one instance
of a thinking stage the user wants shared: brainstorm's deciding, specify's story stage, the
desk's focus stage, and implement's design phase (which runs it itself when no visit file
exists) all run the same engine. Building that shared stage is the three-command convergence
session's job, widened to four: this session's landing writes that widening into the existing
BACKLOG item *Three-command convergence session* (trigger: this session's landing, unchanged).
No decision of this session changes.

**Why.** The desk's standing duties have no counterpart in a per-feature run — the health view,
the baseline outside any feature, the breadth walk over cross-cutting concerns, the drift probe,
amendments, trigger routing (F2, F3, F29) — and the desk was created for exactly that gap (the
`product-architecture-schema` ruling). What the user wants from "bring architecture brainstorming
to implement" — the thinking reachable where the building starts — is the shared engine, and D5-visit-file-input
already opens the door for a visit file; the engine makes the stage itself reachable.

**Rejected roads.** (3) retire the desk — a large landing (the command, 23 floors, the feature
desk symmetry, the store's ownership, hooks, the contract suite, a supersession of the ruling
that created it) and the standing duties would have to move into a run shape that does not fit
them. (1) keep the desk and leave the convergence item as it is — the user's words asked for the
engine; (1) would leave implement out of it.

**Accepted risk.** The engine is not designed here; until the convergence session lands, four
rule sets drift by ruling (the risk `specify-brainstorm-discovery` D5-specify-own-rules already accepted for three). Implement's design
phase keeps its own questioning until then.

**How it was decided.** The user's own steer ("should we just retire architecure then?"), the
lead's assessment put as three options with the cheaper shape named; user: "i like the idea of
thinking engine", re-asked as a one-word confirm in plain words; user: "question 1, yes" —
ratified (streak: 6).

**Changed at review (S14 — the user's own confirm; S15, batch; user: "ok", 2026-10-11).** (S14)
The deciding exchange, quoted. The lead's message that introduced the engine: "**The cheaper
version of your idea** is already on the table: the three-command convergence session
(brainstorm, specify, architecture sharing one thinking engine). We could widen it so
implement's design phase can run the same focus stage directly, without a desk visit first." Its
choices: "(1) Keep the desk, D1 to D5 stand (my pick). The convergence session later makes the
focus stage shared. (2) Keep the desk, and widen the convergence item now to say: implement's
design phase runs the focus stage itself when no visit file exists. One line in BACKLOG, no
change to this session's decisions. (3) Retire the desk. … Recommendation: (2)." The user: "i
like the idea of thinking engine." The re-put: "**Question 1: Keep the architecture command?** —
**Yes.** Keep it. Later, one shared "thinking stage" gets built that brainstorm, specify, the
architecture command, and implement all use. — **No.** Delete the architecture command. Move its
jobs into implement. We stop this session and start a new plan." The user: "question 1, yes".
Because that "Yes" bundled the keep with implement joining the engine, the implement half was
put again as its own one-line confirm at review; the user: "ok" — implement's design phase runs
the shared thinking stage. (S15) The hand-off target changes: the shared thinking-stage design
is judgment work — a stage the four commands run — and does not fit the near-dup convergence
item's charter ("boilerplate only, never judgment"); the landing opens a new BACKLOG item for
the shared stage, and the convergence item stays as it is.

### D7-diagram-set-index — The store gains a diagram set; the root index points at the diagrams and summarizes them — `Confident`

**Statement.** The architecture store gains a diagram set: a `diagrams/` sub-home under
`.mochiko/product/architecture/`, one small file per diagram — one Mermaid block, a one-line
caption, and a `Draws:` line naming the store elements, concern rows and data-model entities it
draws by their joined IDs. Any diagram kind is allowed — state machine, ER, sequence, flowchart,
data flow, deployment and others — and each diagram earns its place by the paragraph test
("would a reader learn more from this than from a well-written paragraph", F19) and the
plan-minimalism ladder; "one diagram, not a gallery" stays the spine's rule for its own container
view, and the spine's container diagram and key-flow sequences stay where they are. An ER or
state view draws the data-model baseline's entities by ID; the truth stays in that baseline, and
the view is flagged stale when a cited ID changes (the upkeep rule is Q7's). The derived root
`ARCHITECTURE.md` embeds no diagram: it is an index that points at each diagram file and
summarizes it in one line (the user's own addition), beside the spine thumbnail, the AX summary
table and the health view it carries today.

**Why.** The user's destination: diagrams that "explain to human and agents how the project
works", built and maintained by the architecture workflow (turn 4), as Mermaid text (F26). A set
of files with a `Draws:` line is the smallest shape that is maintainable (each diagram says what
it depends on) and diffable; the index as a pointer-and-summary keeps the readers' entry point
small for agents and gives humans one page to find the right picture.

**Rejected roads.** (a) the store keeps its two kinds; ER and state pictures drawn inside the
data-model file — a change to implement's design-phase skill and the data-model template, and
it leaves flowcharts and the other kinds without a home. (c) every diagram redrawn into the index
on every write — nothing is a maintained diagram in its own right, every write redraws every
diagram, and the index grows past what an agent should read first.

**Accepted risk.** A second place for data-model facts (an ER view beside the baseline's tables,
blind map 22), held in check by the `Draws:` line and the stale flag. A new sub-home and a
diagram-file template (one migration), and the index template gains a diagrams section. The
readability bar (F29) now reads "from the index and the diagrams it points at".

**How it was decided.** Q6, its own turn, three options with the cheapest named, the case
against the lead's lean given first (streak at 5), re-put in one-word form at the user's ask;
user: "B architecture.md is just index pointing to digagrams and summarizing them" — the lead's
lean with the user's own amendment on the index (streak reset: 0).

**Changed at review (S1, Critical; own turn; user: "as recommded", 2026-10-11).** No entity ID
family exists (artifact-format's IDs section lists none; kinako heads entities by name), so
"data-model entities by their joined IDs" could not be cited. The `Draws:` line now cites a
store element or concern row by its joined ID, and a data-model entity by file and heading —
`data-model.md § Vault` — and a citation that no longer resolves (a renamed or removed heading)
counts as stale, so a rename gets the diagram reviewed rather than silently orphaned. Minting an
entity ID family is handed to the later session that already has to touch
`patterns-entity-modeling` (the *two notations* thread), where the joined form would apply.
Rejected: (a) mint the family now — a template migration and a frame widening into implement's
design-phase skill; (c) no ER or state views until a key exists — parks half of the user's ask.

## Decision map — reshown after D7-diagram-set-index (2026-10-11)

**Decided.** D1-focus-stage-first · D2-part-direction-know · D3-visit-file-home ·
D4-blind-list-visits · D5-visit-file-input · D6-desk-stays-engine ·
D7-diagram-set-index.

**Askable now**
- Q7 — How the diagrams stay true: who flags a stale diagram, who redraws it, and when (hangs
  on D7-diagram-set-index).
- Q8 — Rendering for humans: the standing `architecture-design-primitive` D8-rendered-diagram-signoff ruling against Mermaid
  text in git (amends an existing ruling — the user's turn).

**Defaults — named, confirmed in one batch at wrap-up.** Half A's eleven (above) · half B: the
taste rules written into mochiko's diagram conventions from the named repo (fact work owed:
which survive Mermaid and GitHub's renderer) · the principal-architect draws every store diagram,
ER and state views included, from the baseline's IDs; the data-model truth stays with implement's
design phase · readers: the index summarizes and links, never embeds (ruled in D7-diagram-set-index) · two build
items, thinking and diagrams, so neither stalls on the other · the feature-level text notation
for ER and state against the product-level Mermaid views — handed to a later session · dogfood
criteria deferred to the first run (specify's precedent).

**Out of scope.** As changed at D5-visit-file-input.

### Q7 — How do the diagrams stay true? (put 2026-10-11, plain form)

Hangs on D7-diagram-set-index; unblocks the health-view and landing defaults. Options:
(a) stale flag only — any store or baseline write that touches an ID a diagram's `Draws:` line
names flags that diagram stale in the index's health view (a mechanical compare the store skill
runs when it regenerates the index); the next desk visit redraws it as a judgment write (cheaper
shape) · (b) redraw at the landing — implement's store landing redraws every touched diagram as
part of the fold (a second implement touch; the redraw is judgment, so it takes the review leg
inside a run that is closing) · (c) both — the flag is mechanical and immediate at any write; the
redraw is a judgment write at the next desk visit, or in a feature's design phase when its delta
touches the diagram, never inside the landing. Lead's pick: (c).
**User (2026-10-11): "I would like the architecture to be drawn first before implementation
happens, so implementation stays true to it"** — a direction, not a pick: diagrams are drawn
ahead of code and the build conforms to them. Fact put back to the user: implement already
draws a feature's delta before code and checks the build against it — the signed `architecture.md`
drawing in the run home, the deviation gate, the built-vs-signed diff (F34; `impl.midrun-refire`;
`architecture-design-primitive` D6-implement-honors-architecture) — so the want is met today for
the per-feature drawing and not for the store's maintained diagrams, which lag until a landing
or a visit. Re-put as two concrete shapes: (C1) the design phase updates the store's diagram
files in place before code, delta-styled (new · modified · removed), and that is what the user
signs; the run's own drawing shrinks to the checkpoint table and links; the deviation gate and the
built-vs-signed diff read the store diagrams; the landing flips the styling to plain (a second
implement touch — the design phase's output home gains the diagram files) · (C2) the per-feature
signed drawing stays as today, drawn before code in the run home; the store's diagrams are
redrawn from it at the landing (so they are true by the time the feature is built, never before).
**User: "c1"** (2026-10-11) — the user's own direction, shaped by the lead into two roads, the
user picking the one that matches their words (streak: 0, the user's design move) →
D8-draw-before-code.

**Frame change (the user's word, 2026-10-11).** Out of scope: implement's exception widens from
one line to three — the design-input line (D5-visit-file-input), the offer default, and the design phase drawing
a feature's change into the store's diagram files before code (D8-draw-before-code). Stamped here. Re-check:
D1–D7 unchanged by it; D7-diagram-set-index's `diagrams/` sub-home now has two writers across time (the architect
at the desk, the architect in a design phase) — one at a time, keyed by the lifecycle marker, as
the store's elements already are.

### D8-draw-before-code — A feature's change is drawn into the store's diagrams before code, signed there, and the build is held to it — `Confident`

**Statement.** Diagrams are drawn ahead of code, and the build conforms to them. For a
feature's structural change, implement's design phase edits the store's diagram files in place
before any code — the change delta-styled (new · modified · removed, the existing convention
F22) and the touched elements `proposed (<key>)` as the store already does — and that marked-up
diagram set is what the user signs at the design checkpoint; the run's own `architecture.md`
shrinks to the checkpoint table of changed elements and links to the store diagrams it signed.
The deviation gate and the built-vs-signed diff read the store diagrams; at the landing the
delta styling is flipped to plain and the elements to `built`, no redraw. A change made at the
desk with no feature redraws the touched diagrams during the visit, by the architect seat on the
ruled focus set, graded by tech-lead. The safety net: any store or baseline write that touches
an ID a diagram's `Draws:` line names, with no drawing behind it, flags that diagram stale in the
index's health view (a mechanical compare the store skill runs at index regeneration); a stale
diagram is redrawn at the next desk visit or in the next design phase that touches it, never
inside a landing. Agents reading a diagram read its stale flag with it.

**Why.** The user's words: "the architecture to be drawn first before implementation happens,
so implementation stays true to it." Implement already draws a feature's delta before code and
holds the build to it (F34, `impl.midrun-refire`, the built-vs-signed diff); the store's
maintained diagrams were the gap — true only after a landing or a visit. Editing them in place
with delta styling is the same pattern the baselines already follow (`delta-files-vs-direct-baseline-edits`
D1-delta-files-retire/D7-lifecycle-write-timing: written in place, flipped at sign-off).

**Rejected roads.** (C2) keep the per-feature drawing in the run home and redraw the store's
diagrams at the landing — true only once built. (A) flag only, redraw at the next visit — the
diagrams lag the code. (B) redraw inside the landing — judgment work at the busiest moment of a
closing run.

**Accepted risk.** A second and third implement touch (the design phase's output home gains the
diagram files; the run's drawing shrinks to a table and links) — the user's word widened the
frame for it. `architecture-design-primitive` D5-plan-run-artifact's "the drawing the user signs"
is amended in part: the signed object stays the run's `architecture.md`, now drawn from and
listing the store's delta-styled diagram files, which signing it signs (V9); recorded
as a supersession-by-ruling at landing. Two writers across time on `diagrams/`, serialized by
the lifecycle marker.

**How it was decided.** Q7, its own turn, three options in plain words; the user answered with a
direction of their own; the fact that implement already draws first was put back; two concrete
shapes re-put; user: "c1" (streak: 0).

**Changed at review (S1 own turn; S8 the user's word; S3, S4, S6, S7 batch; user: "ok",
2026-10-11).** (S1) The stale net compares cited targets — joined IDs for store elements and
concern rows, file and heading for data-model entities — and a citation that no longer resolves
reads stale (D7-diagram-set-index changed-at-review). (S3) Each diagram file is one entry carrying its own
`**Lifecycle:**` field, exactly as a product-baseline entry does
(`impl.baseline-entry-grammar`), so a design-phase hunk in it is a marked hunk under
`impl.fail.unmarked-baseline-write`; one open key per file — a second run that finds a diagram
`modifying (<other key>)` treats it as an in-flight collision and waits or routes to the user,
never a second marker on one file. (S4, repaired at verify — the S4 NOT CLEAN and V2) The stale net
defined: the compare runs at every regeneration of the derived index, and the index regenerates
at every store write (as today) and at every design checkpoint and every landing — structural
or not — of any run that wrote a product baseline or a diagram: implement's checkpoint and
landing each call the store skill's regeneration, while the fold itself still fires only when
structure was built. Its executor is the store skill, reading `Draws:` targets against the
entries and headings changed since the last regeneration — the index header carries the commit
it was last regenerated at, and the compare reads the diff since it (the reviewer's ungraded
note, lead-repaired) — a model, no code, an advisory
`mochiko-cli` check possible later and never required. "No drawing behind it" means a cited
target changed with no delta-styled change in that diagram under the same key or visit. The
flag has one home, the index's health view — outside `.mochiko/product/`, so never a run's
unmarked hunk — which counts the stale diagrams as a sixth count and names them
(`authoring-architecture-store.health-view-binding` amended from five counts to six); a diagram
file carries no flag, and an agent reading a diagram reads the health view's stale list with it.
(S6, repaired at verify V1) One signed object: the run's `architecture.md` — the neighborhood
delta view (the ~12-node rule of `patterns-system-design`) drawn from the delta-styled elements,
with the checkpoint table listing every store diagram file the change edited. Signing it signs
those in-place, delta-styled edits in `diagrams/` and, where a container changed, in the spine's
container view. The deviation gate anchors to the signed `architecture.md` — its view and its
table, the kinds per S7; a two-node change signs a neighborhood, never a wall. The statement's
"shrinks to the checkpoint table and links" reads as this. (S7) The deviation
gate holds the build to the container and flow views — today's kinds; ER, state, flowchart and
other views are checked at the landing diff, never as cycle stops. (S8) The implement touches
this card, D5-visit-file-input and D9-render-where-viewed reach are nine (V6), stamped as one frame change at review (the S8 list): the
design-input line · the offer · drawing into the store's diagram files before code ·
`impl.design-outputs-home` · `impl.deviation-gate` · the built-vs-signed diff · the landing flip
· gap-finding's first message · `impl.gate-design-checkpoint`'s rendered-diagram wording · and
a tenth, stamped at verify by the user's word (V7): implement rebuilds the derived index at every
design checkpoint and every landing, structural or not, so the stale compare runs there.

### Q8 — Rendering for humans: what does "the sign-off presents the rendered diagram" become? (put 2026-10-11)

Amends an existing ruling — `architecture-design-primitive` D8-rendered-diagram-signoff, "The
sign-off presents the *rendered* diagram, never raw source", with R5's degrade-with-record
exception (F23) — so the user's own turn. Facts: kinako's one live run signed un-rendered under
the exception (F23, F25); the plugin ships no renderer (clone-only install); Mermaid text renders
where it is viewed — GitHub, IDE previews — and this harness can publish a page that renders it
(the Artifact tool) when present. Options: (a) keep `architecture-design-primitive` D8-rendered-diagram-signoff as ruled — the sign-off must show a
rendered picture, the degrade-with-record path stays the fallback; the rendered primary path is
still unbuilt (BACKLOG, *Architecture-primitive dogfood*) · (b) amend: the sign-off surface is
Mermaid text that must parse, shown with a render when the session has a render surface (a
published page, a browser preview) and as text with a one-line degrade note when it has none;
"must render where viewed" replaces "must be seen rendered at sign-off"; an optional parse check
in `mochiko-cli` is an advisory exit-code signal, not kernel-class (CLAUDE.md) · (c) require a
renderer on the machine (`mmdc` or a browser) and halt the sign-off without one — the clone-only
install gains a developer-tool dependency, as it did for `mochiko-cli`. Lead's pick: (b);
cheapest: (a) unchanged.

#### Facts for the taste default (2026-10-11, live checks for blind map 26)

- F36 — Mermaid docs (context7 `/mermaid-js/mermaid`, `docs/syntax/flowchart.md` and
  `src/themes/theme-base.js`): the init directive sets a theme and its variables inline —
  `%%{init:{"theme":"base", "themeVariables": {"primaryColor":"#411d4e", "lineColor":"red",
  "darkMode":true}}}%%`; `fontFamily` rides the same directive; `classDef myStyle
  fill:#ff0000,stroke:#ffff00,stroke-width:4px` with `A:::myStyle` is "the recommended approach
  to style flowchart nodes"; the base theme derives `nodeBkg`, `nodeBorder`, `clusterBkg`,
  `defaultLinkColor`, the sequence actors' colours, `transitionColor` (state) and
  `relationColor` from `primaryColor`, `lineColor` and `textColor`. Lead's reading: one accent
  colour, hairline strokes and a font family are expressible per diagram in text; a 4px grid,
  shadows and exact geometry are not Mermaid's to control (it lays out), so those rules of the
  named repo do not carry.
- F37 — GitHub docs (`docs.github.com … creating-diagrams`, fetched 2026-10-11): "Add Mermaid
  syntax inside a fenced code block with the `mermaid` language identifier"; "Diagram rendering
  is available in GitHub Issues, GitHub Discussions, pull requests, wikis, and Markdown files";
  "To ensure GitHub supports your Mermaid syntax, check the Mermaid version currently in use"
  (a `mermaid` block containing `info`). The page says nothing about init directives, themes or
  size limits — whether GitHub honours an init directive stays unverified here and is checked by
  the build on a real file.

**Q8 ruling.** User: "as recommded" (2026-10-11) — ratified (streak: 1) → D9-render-where-viewed.

### D9-render-where-viewed — The sign-off surface is Mermaid text that parses, rendered where the session can — `Confident`

**Statement.** A diagram's sign-off surface is its Mermaid text, which must parse. When the
session has a render surface — a page it can publish, a browser preview — the user is shown the
rendered picture beside the text; when it has none, the text is shown with a one-line note that
it was not rendered here and renders where it is viewed (GitHub, an IDE preview). The rule
"must be seen rendered at sign-off" becomes "must render wherever it is viewed". An optional
parse check in `mochiko-cli` is admissible as an advisory exit-code signal, not kernel-class;
the build decides whether it is worth shipping.

**Why.** The plugin ships no renderer and the one live run signed un-rendered under the
exception (F23, F25) — the old rule held in text and not in practice. Mermaid text renders by
itself where it is read (F37), and the diagrams are now text in git by the user's word (F26).

**Rejected roads.** (a) keep the old rule with its fallback — the rendered primary path stays
unbuilt and the fallback stays the only path. (c) require a renderer on the machine — one more
developer-tool dependency for a clone-only install.

**Accepted risk.** The user may sign a text block they have not visualized; the taste rules
keep diagrams small, and the render shows up wherever a render surface exists.

**How it was decided.** Q8, the user's own turn (it amends `architecture-design-primitive`
D8-rendered-diagram-signoff — a supersession-by-ruling recorded at landing), three options with
the unchanged road named; user: "as recommded" — ratified (streak: 1).

**Changed at review (S9, batch; user: "ok", 2026-10-11).** The texts this ruling supersedes,
named: `architecture-design-primitive` D8-rendered-diagram-signoff;
`authoring-architecture-store.sign-off-flips-proposed`'s "on the rendered diagram";
`impl.gate-design-checkpoint`'s "signed on a rendered diagram"; `patterns-system-design`'s "must
render … never ship as a raw code block" line and its checklist item — each a
supersession-by-ruling recorded at landing. "Must parse" is the lead's duty before the
sign-off: checked by the render surface when one exists, else by the author's own read, the
degrade note saying which — a tool check stays optional. A published page sends product
architecture to a hosted service: the lead asks the user before publishing and prefers a local
preview.

## Defaults batch — named as they arose, put for one confirmation at the accept screen

Each default was named by heading in the turn it arose, with the offer "say pull X to open any
default on its own turn"; the user pulled none. The living text:

- **Who holds the conversation.** The lead, inline, in the main pane — the same seat and pane as
  the visit goal (`specify-brainstorm-discovery` D3-lead-holds-stage; the user's word "the lead is the manager of the
  teammates"). The lead carries the architect's craft as inputs — the shelf data, the store's
  index, the system-design altitude — puts the forks, and transcribes the user's rulings into the
  visit file (transcription — no sound loop). The principal-architect seat then plans and
  authors from the visit file, graded by tech-lead as today. No teammate holds a conversation
  with the user.
- **Mechanisms carried** from brainstorm and specify into the focus stage: one fork per turn · at
  least two real options, one the do-less shape (leave the row at its default, rule it `not-now`,
  or draw nothing) · the fact a fork hangs on in hand before it is put · a rough in-message mock
  — a one-item focus row as it would read, or a small Mermaid sketch when the question is about
  shape · a status line (ruled · askable · in fog) over the candidate parts · own-turn-or-batch: a
  part gets its own turn only when it is a real choice or costly to undo; the rest are default
  rows — the shelf's suggested default with its cost — named in one line as they arise and
  confirmed together on one screen, never by silence · the ratified-yes streak flag, counted over
  forks only, never over batch rows (blind map 18) · a vague answer re-asked as two concrete
  options · the stall clause: if the candidate set has not shrunk for three turns, say so and
  offer to step back to the visit goal · the stop rule: every candidate in or out, nothing open ·
  a provenance mark per item in the visit file — `user's words` · `lead's guess, ratified` ·
  `blind list` · `handed in`. Dropped: a frame card (the visit goal and its done condition are it)
  · decision cards and a record file (the visit file and the store rows are the record) · a blind
  angle map and a cold review (tech-lead's grade is the independent leg).
- **A `kind: fail`.** A desk visit's store write — baseline, stance batch, amendment, diagram —
  with no ruled focus set behind it in a visit file fails the visit (mirror of
  `spec.fail.story-outside-slate`); scoped to desk visits, never to an implement run. An element
  the set lacks is proposed to the user as a candidate, never authored before the user rules it
  in. *(S18, batch: scoped to desk visits; "delta" dropped — the desk writes no store delta,
  D5-visit-file-input.)*
- **Context inputs** *(added at review — S12, coverage, ruled inline)*: what the lead reads to
  propose a focus, and what the blind seat is given, per visit — the visit goal · the `Scope:`
  line · the derived index · any brainstorm record named · `FEATURES.md` and the capability
  entries in the visit's territory · the codebase analysis where one exists · for a feature's
  delta visit, the feature's spec; never the focus set, the visit file or the conversation
  (D4-blind-list-visits changed-at-review).
- **First visits.** Greenfield: the elicitation is the focus stage — the shelf rows and the blind
  list supply the candidates, the user's directions become the baseline's rulings, the batch
  confirms the rest. Brownfield: the reconstructed derivation (`Assumed`) is the fact base; the
  focus stage picks which derived elements get a real fork and which are confirmed in the batch;
  the confirm step becomes the set's close; nothing absorbed is silently discarded
  (`arch.dm-author-baseline` stands).
- **No size gate; a skip hands the set in.** A crisp ask is a focus set of one, provenance
  `handed in`, recorded in the visit file's header; the set's close, the visit file and the
  architect's graded write still happen; the blind list is skipped only per
  D4-blind-list-visits.
- **Inputs already in hand.** A brainstorm record named in `$ARGUMENTS` (the desk's
  `argument-hint` gains it — second-list fact), or directions the user hands in, open the stage
  as facts; only what they leave open is asked.
- **Where the rules live.** The desk's own schema: a new moment (the focus stage), the stage's
  duties and bindings, the reservation line (confirming the focus set is the user's, on a screen
  of its own), the fail rule, and a precedence clause stating that where the desk's rules and
  `mochiko:analysis-iterative` differ, the desk's rules win (as `brainstorm.own-rules-win` and
  `spec.own-rules-win` do). Convergence into the shared thinking stage is handed on
  (D6-desk-stays-engine).
- **Outside claims.** A load-bearing outside claim a focus item hangs on — a provider limit, a
  price, a platform behaviour — is live-checked before the item is ruled, its source in the visit
  file; a claim that cannot be checked is stated as unknown and the item carries it `open`;
  tech-lead's grade re-checks the claims the rows rest on (blind map 11) — the re-check rides the
  review-seat duty of `EXTERNAL-CLAIMS.md` (`review-brainstorm`'s reference), named in the
  grading brief, and the build adds that pointer to `arch.seat-tech-lead-grader` *(S21, batch)*.
- **Provenance per item** lives in the visit file; the AX row gains no field and links the visit
  file as its rationale home (D3-visit-file-home).
- **The implement offer.** When the sufficiency check names a structural gap and no desk visit
  file exists for the feature, the design phase offers a desk visit first or proceeds as today —
  the user's choice at run time (D5-visit-file-input).
- **Taste.** `patterns-system-design`'s `DIAGRAM-CONVENTIONS.md` gains a taste section drawn from
  the named repo, as far as Mermaid carries it (F36): one accent colour, reserved for the one or
  two things the reader should look at first (the delta styling keeps its three classes) ·
  hairline strokes, no fills beyond the accent · one font family, set by the init directive ·
  delete first, and the paragraph test before any diagram is drawn (F19) · a node budget, the
  existing ~12-node threshold · a title and a one-line caption on every diagram · a delta read
  as Before · Changes · After through the new / modified / removed classes. Rules that are
  SVG-only — the 4px grid, shadows, exact geometry — do not carry. Whether GitHub honours the
  init directive is checked by the build on a real file (F37). `DIAGRAM-CONVENTIONS.md` is the
  one home: the taste section lands first, and the trust-boundary diagram convention queued in
  the security-depth build (BACKLOG.md:907) extends it there *(S23, coverage, ruled inline)*.
- **Who draws.** The principal-architect draws every store diagram, ER and state views included,
  from the data-model baseline's entities cited by file and heading (S1; V4); the data-model
  truth stays with implement's design
  phase and the technical-analyst; tech-lead grades a drawing against the targets its `Draws:` line
  cites.
- **Readers.** The index summarizes and links each diagram, never embeds it
  (D7-diagram-set-index); agents read the diagram file — text and `Draws:` line — with the index's stale list (V8) —
  and humans see it rendered where it is viewed (D9-render-where-viewed).
- **Two build items.** (a) the focus stage, the visit-file home and template, implement's
  design-input line and offer; (b) the diagram sub-home and template, the index's diagrams
  section, the draw-before-code change in implement's design phase, the render rule and the
  taste section — so neither stalls on the other (blind map 32).
- **Two notations for one thing.** The feature-level text notation for ER and state machines
  (`patterns-entity-modeling`, F24) against the product-level Mermaid views, and the minting of
  an entity ID family (S1): handed to a later session, written as an open thread at landing.
- **The shared thinking stage** *(S15, batch)*: its design is a new BACKLOG item at landing —
  judgment work, a stage brainstorm, specify, the desk and implement's design phase all run —
  separate from the near-dup convergence item, which stays as it is.
- **Dogfood criteria.** What a run is judged on is deferred to the first desk run in ai-fileops —
  the greenfield baseline — judged by the user in use, as `specify-brainstorm-discovery` D7-dogfood-criteria-deferred did; `Deferred` on the
  user's word at the batch confirm.

## Open questions

- OQ1 — The evidence base: no desk run has shown the problem (F28); the diagnosis carries
  `Assumed` until the first ai-fileops desk run.
- OQ2 — The shared thinking stage's design (D6-desk-stays-engine): its own BACKLOG item at
  landing (S15), implement's design phase included; the near-dup convergence item stays as it
  is (V5).
- OQ3 — Whether GitHub honours a Mermaid init directive (F37): the build checks on a real file.
- OQ4 — How a `ruled`-not-built store element reads at the sufficiency check and flips at a
  landing (F35): not needed under D5-visit-file-input, noted for the convergence session.
- OQ5 — An entity ID family for the data model (S1): handed to the *two notations* session;
  until then ER and state views cite entities by file and heading.

## Untested bets (shown at the accept screen)

- The focus stage does not make a visit heavier than today's walk — tested on the first
  ai-fileops baseline run as turns and the user's time to the set's close, judged by the user
  (ai-fileops is desktop-scoped with no seeded shelf, so no shelf walk exists there to compare
  against — S16; a shelf-walk comparison waits for the first backend-scoped product or the
  Stage-2 desktop shelf; the open falsifier F5a still watches). A visit now has three agreed
  screens — goal, focus set, rows (S2) — which this bet carries.
- The defaults batch is not a rubber stamp — tested as pulls per batch: a batch that draws no
  pull across the first three visits is the warning sign (S22; this session's sixteen-row batch
  drew none).
- The blind list surfaces off-shelf concerns the user rules in — tested by counting visit-file
  items carrying provenance `blind list` that the user ruled in.
- The "how I'd know" sentence pins an item's meaning for the architect — tested by tech-lead's
  sameness grade at the rows' write finding no change of meaning.
- Diagrams drawn before code keep the build true — tested by the built-vs-signed diff on the
  first feature after the build showing no unsigned structural deviation.
- Mermaid carries enough of the named repo's taste — tested by the user's own read of the first
  drawn set.

## Freeze

Accept screen shown 2026-10-11 — the nine decisions by name and gist, the sixteen defaults, the
out-of-scope line as changed, the four open questions, the five untested bets with their tests;
the pull offer repeated; the user pulled none. **User: "confirm"** — the defaults batch is
confirmed as one block; the record is frozen for the cold read. The review seat (`review-seat`,
the blind-map seat) is resumed with this record for message two. Writers hold until the verdict
(`patterns-transport-floor.quiesce-before-cold-grade`).

## Cold review — landed 2026-10-11 (`reports/review.md`)

Solo cold read by `review-seat` (`mochiko:devils-advocate`, persona tier `opus`, no override), the
seat that drew the blind map; five cross-examination questions answered by the lead by message
(the row gate · the entity key · diagram hunks and two keys · D6-desk-stays-engine's exact exchange · the batch's
row set on later visits), the record untouched. Recommended status **critical-gaps**: 29 raised,
24 survived — 1 Critical · 15 Important · 8 Minor; coverage-marked S11, S12, S16, S23. Verified
first-hand by the seat: the architecture, store, implement, review-sufficiency, feature-map and
spine renders; the product and feature-desk homes; artifact-format's IDs section; `mochiko-cli
ids --check` (2 bare · 8 drift); kinako's spine line 8, `concerns.md` and `data-model.md`
headings; ai-fileops' `Scope:`; BACKLOG lines 37–41, 793, 907; `architecture.md` line 19; F19
and F37 live. Taken from the record unverified: F36, F18, F20, F24, F25, F31, F9–F17's lines.
Fence: the index never opened; no Explore spawn. The lead's reading of the survivors: the nine
cards stand in direction; the Critical one (S1) breaks a mechanism D7-diagram-set-index leans on; most of the
Important ones are intent the lead held and never wrote, plus floors D8-draw-before-code and D9-render-where-viewed reach into
without naming.

Routing (per `brainstorm.non-coverage-survivors` and `brainstorm.coverage-survivor-routing`):
own turns, one at a time, for S1 (the entity key — Critical, a real choice), S5 (the visit
file's home — challenges D3-visit-file-home, a user ruling), S13 (which visits fire the machinery — the cheaper
shape); the user's word on S8 (the full implement touch list as one frame stamp) and S14 (the
implement half of D6-desk-stays-engine as its own one-line confirm); the four coverage candidates S11, S12, S16,
S23 each put for a path (explore now · rule inline · defer) with the lead's lean; the rest — S2,
S3, S4, S6, S7, S9, S10, S15 and the Minor S17–S24 — as one named repair batch, the lead's
proposed fix per item, each a pull away from its own turn.

### Dispositions

| # | Sev | Finding (gist) | Disposition |
|---|---|---|---|
| S1 | Critical | ER and state views cite entity IDs that do not exist | own turn; user: "as recommded" — cite by file and heading, a dangling citation reads stale; entity IDs handed to the later *two notations* session (D7-diagram-set-index changed-at-review; D8-draw-before-code's stale net reads "cited targets") |

### S5 — The visit file's home (own turn, 2026-10-11)

The visit file (D3-visit-file-home) sits inside the store directory, which two rules hold to one writer through
the store skill (`arch.single-writer-store`; `authoring-architecture-store.one-home-one-writer`,
`class: floor`), and a run that pauses for a desk visit (D5-visit-file-input) would find the file under
`.mochiko/product/` in its pinned-base diff as an unmarked hunk (`impl.fail.unmarked-baseline-write`,
`class: floor`). Options: (a) home the visit directories outside `.mochiko/product/` — a new home
beside the store, as the feature desk's `.mochiko/features/desk/` sits beside the map's truth
files; the rows still link to the file; the build names the path (lead's pick) · (b) keep them
inside the store and record two floor amendments plus a run-pause exception (three floor touches)
· (c) keep them inside the store and forbid a desk visit while a run is open (the D5-visit-file-input pause path
dies). **User: "as recommded"** → (a); D3-visit-file-home changed-at-review.

| S5 | Important | the visit file inside the store collides with the single-writer floors and a paused run's unmarked-hunk fail | own turn; user: "as recommded" — visit directories homed beside the store, outside `.mochiko/product/` (D3-visit-file-home changed-at-review) |

### S13 — Which visits fire the stage, the visit file and the blind list (own turn, 2026-10-11)

As ruled, every visit gets a directory (D3-visit-file-home) and the stage (D1-focus-stage-first), and any multi-part set a blind
seat (D4-blind-list-visits) — so a store query writes a file for nothing ruled, and a drift visit with three
findings pays a seat over a closed three-way disposition
(`authoring-architecture-store.drift-user-disposition`). The user's word was "every visit"
(F27), so the change is the user's. Options: (a) the cheaper shape — the stage, the visit file
and the blind list fire only on visits that rule something: a baseline, a stance, an amendment,
a feature's delta thinking; a store query writes nothing; a drift visit records its dispositions
on the rows as today, with no stage, no file, no list; a fired-trigger routing writes nothing
(lead's pick) · (b) as ruled — every visit, literally · (c) the stage on every visit, the file
only when something was ruled, the blind list per D4-blind-list-visits. **User: "as recomded"** → (a); D1-focus-stage-first
changed-at-review.

| S13 | Important | machinery fires on visits that rule nothing | own turn; user: "as recomded" — the stage, the file and the list fire only on visits that rule something; a query writes nothing; a drift visit records dispositions on the rows as today (D1-focus-stage-first changed-at-review; F27 narrowed by the user's word) |

### S8 and S14 — the user's word (put 2026-10-11)

- S8 — the implement touches D5-visit-file-input, D8-draw-before-code and D9-render-where-viewed reach, beyond the three lines stamped: (1) the
  design-input line (D5-visit-file-input) · (2) the offer (D5-visit-file-input) · (3) the design phase draws into the store's
  diagram files before code (D8-draw-before-code) · (4) `impl.design-outputs-home` — the run's drawing changes
  shape (D8-draw-before-code) · (5) `impl.deviation-gate` reads the store diagrams (D8-draw-before-code; kinds narrowed by S7) ·
  (6) the built-vs-signed diff reads them (D8-draw-before-code) · (7) the landing flip clears the delta styling
  (D8-draw-before-code) · (8) gap-finding's first message carries the signed store diagrams in place of the
  signed drawing (D8-draw-before-code) · (9) `impl.gate-design-checkpoint`'s "signed on a rendered diagram"
  (D9-render-where-viewed). Put as one frame stamp: all nine as the implement touch; or narrow D8-draw-before-code to C2 (the run's
  drawing stays the signed surface; touches 4–8 fall away).
- S14 — the implement half of D6-desk-stays-engine rode a bundled "Yes"; put as its own confirm: implement's
  design phase runs the shared thinking stage (as D6-desk-stays-engine says), or the engine is brainstorm's,
  specify's and the desk's only, implement reading visit files alone.

### Coverage candidates S11, S12, S16, S23 — each put for a path (explore now · rule inline · defer)

- S11 (angles 1, 21) — the batch's row set on later visits. Lead's lean, rule inline: the batch
  covers only rows with no stance yet (`open`, or newly in scope); a `decided`, `not-now` or
  `n-a` row is never re-dealt by a batch and changes only as a focus item the user pulls in — an
  amendment demand, a fired trigger, a drift finding.
- S12 (angles 2, 9) — the context the focus and the blind list draw from. Lead's lean, rule
  inline, per visit kind: the lead reads, and the blind seat is given, the visit goal · the
  `Scope:` line · the derived index (spine thumbnail, AX summary, health) · any brainstorm
  record named · `FEATURES.md` and the capability entries in the visit's territory · the
  codebase analysis where one exists (brownfield) · for a feature's delta visit, the feature's
  spec; never the focus set or the conversation.
- S16 (angle 16) — the "not heavier" bet cannot be tested on ai-fileops (desktop, no shelf).
  Lead's lean, rule inline: the bet is read as turns and the user's time to the set's close on
  the first ai-fileops baseline, judged by the user; a shelf-walk comparison waits for the first
  backend-scoped product or the Stage-2 desktop shelf (both dogfoods are desktop).
- S23 (angle 29) — the queued trust-boundary diagram convention (BACKLOG.md:907) against the
  taste section. Lead's lean, rule inline: `DIAGRAM-CONVENTIONS.md` is the one home both
  extend; the taste section lands first and the trust-boundary convention extends it.

**User: "ok"** (2026-10-11) — all six on the lead's lean.

**Frame change (the user's word, 2026-10-11, S8; extended at verify, V7).** Out of scope:
implement's exception is the nine touches listed under S8 plus a tenth — the index rebuild at
every design checkpoint and every landing (V7, user: "yes") — stamped as one frame change in
place of the earlier "three touches". Re-check: D5-visit-file-input, D8-draw-before-code and D9-render-where-viewed are the decisions that reach them — consistent; D1–D4, D6-desk-stays-engine and D7-diagram-set-index
untouched.

| S8 | Important | implement touches exceed the stamped frame change | the user's word: all nine stamped as the implement touch (frame change above) |
| S14 | Important | D6-desk-stays-engine's implement half rode a bundled yes | the user's own confirm: implement's design phase runs the shared thinking stage (D6-desk-stays-engine changed-at-review; the exact exchange quoted there) |
| S11 | Important, coverage | the batch's row set on later visits undefined | ruled inline: no-stance rows only; ruled rows change only as pulled focus items (D1-focus-stage-first changed-at-review; batch default amended) |
| S12 | Important, coverage | the context the focus and the blind list draw from never bound | ruled inline: the input list per visit kind (D4-blind-list-visits changed-at-review; a new default *Context inputs*) |
| S16 | Important, coverage | the "not heavier" bet untestable on desktop ai-fileops | ruled inline: turns and the user's time to the set's close, judged by the user; a shelf comparison waits for a backend-scoped product or the Stage-2 desktop shelf (bets amended) |
| S23 | Minor, coverage | two conventions landing in one file unreconciled | ruled inline: `DIAGRAM-CONVENTIONS.md` is the one home; taste first, the trust-boundary convention extends it (taste default amended) |

### Repair batch — the lead's proposed fix per item (put 2026-10-11)

- S2 (D3-visit-file-home) — add: the desk's existing user gate on the architect's rows — after tech-lead's grade,
  `arch.user-reserved-rulings` and `arch.sound-loop-floor` unchanged — stands; the sameness grade
  is additive to it; the user sees the concrete row text (the chosen provider, the tenancy model)
  at that gate. A visit then has three agreed screens — goal, focus set, rows — noted on the "not
  heavier" bet.
- S3 (D7-diagram-set-index, D8-draw-before-code) — each diagram file is one entry carrying its own `**Lifecycle:**` field, exactly as
  a product-baseline entry does (`impl.baseline-entry-grammar`), so a design-phase hunk in it is a
  marked hunk; one open key per file — a second run that finds a diagram `modifying (<other
  key>)` treats it as an in-flight collision (waits, or routes to the user), never a second
  marker on one file.
- S4 (D8-draw-before-code) — the stale net defined: the compare runs at every index regeneration and is also
  called at a design phase's checkpoint and at a landing fold (both already go through the store
  skill); its executor is the store skill reading `Draws:` targets against the touched entries
  and headings — a model, no code (an advisory `mochiko-cli` check may follow, never required);
  "no drawing behind it" means a cited target changed with no delta-styled change in that
  diagram under the same key or visit; one home for the flag — a `Stale:` line in the diagram
  file, read by agents with the diagram — and the health view counts the stale diagrams as a
  sixth count (`health-view-binding` amended from five to six).
- S6 (D8-draw-before-code) — what the user signs: the diagram files in `diagrams/` the change touches, plus the
  spine's container view when a container changes, each edited in place with delta styling; the
  run's `architecture.md` keeps the neighborhood delta view (the ~12-node rule of
  `patterns-system-design`) drawn from the same elements as the sign-off picture, with the
  checkpoint table and links — so a two-node change signs a neighborhood, never a wall; D8-draw-before-code's
  "shrinks to the checkpoint table and links" is softened to that.
- S7 (D8-draw-before-code) — the deviation gate holds the build to the container and flow views (today's kinds);
  ER, state, flowchart and other views are checked at the landing diff, never as cycle stops.
- S9 (D9-render-where-viewed) — the superseded texts named: `architecture-design-primitive`
  D8-rendered-diagram-signoff; `authoring-architecture-store.sign-off-flips-proposed`'s "on the
  rendered diagram"; `impl.gate-design-checkpoint`'s "signed on a rendered diagram";
  `patterns-system-design`'s "must render … never ship as a raw code block" line and its
  checklist item — each a supersession-by-ruling at landing. "Must parse" is the lead's duty
  before the sign-off: by the render surface when one exists, else by the author's own read, the
  degrade note saying which. A published page sends product architecture to a hosted service —
  the lead asks the user before publishing and prefers a local preview.
- S10 (D1-focus-stage-first) — focus items and batch rows are each walked in retrofit-cost order; each batch row
  shows its suggested default, what it costs and what would argue against it (the shelf floor's
  three parts) — no supersession of `arch.breadth-invariant` or
  `patterns-architecture-shelves.recommend-then-arbitrate`.
- S15 (D6-desk-stays-engine) — a new BACKLOG item for the shared thinking-stage design (judgment work: a stage the
  four commands run), separate from the near-dup convergence item (boilerplate only), which is
  left as it is.
- S17 (D5-visit-file-input) — the desk writes no link on the feature's entry; implement's design phase fills the
  architecture link when it reads the visit file, and the slug finds the file until then.
- S18 (defaults) — the `kind: fail` is scoped to desk visits; "delta" dropped from its list.
- S19 (D2-part-direction-know) — where the know-sentence lands: in the visit file always; on a `not-now` row as the
  upgrade trigger; on a `decided` row as the first line of its `Rationale`; a target with a number
  only when the user gives the number — never derived from the sentence.
- S20 (D5-visit-file-input) — the design phase reads a visit file against its own base; where the store has moved
  past the file, the conflicts are listed at the checkpoint for the user.
- S21 (defaults) — tech-lead's re-check of outside claims rides the review-seat duty of
  `EXTERNAL-CLAIMS.md` (`review-brainstorm`'s reference), named in the grading brief; the build
  adds that pointer to `arch.seat-tech-lead-grader`.
- S22 (D4-blind-list-visits, bets) — D4-blind-list-visits keeps `Confident` with its why written: the case against was put first and
  the user ruled on it explicitly; "pulls per batch" joins the untested bets (this session's
  sixteen-row batch drew none).
- S24 (IDs) — owner code spans added to every cross-session cite; the five four-word slugs
  renamed to three words — `D1-focus-stage-first` · `D4-blind-list-visits` ·
  `D5-visit-file-input` · `D6-desk-stays-engine` · `D7-diagram-set-index` — every cite swept.

**User: "ok"** (2026-10-11) — the batch confirmed as one block; every fix applied to the one
card it touches, each under a changed-at-review line.

| # | Sev | Finding (gist) | Disposition |
|---|---|---|---|
| S2 | Important | where the user rules the architect's concrete row text | batch — the existing user gate stands, the sameness grade additive; three agreed screens (D3-visit-file-home changed-at-review; bet amended) |
| S3 | Important | diagram hunks unmarked; two open keys on one file | batch — one entry per diagram file with its own `**Lifecycle:**`; one open key per file, a second is an in-flight collision (D8-draw-before-code changed-at-review) |
| S4 | Important | the stale net has no trigger, executor or single home | batch — trigger at every index regeneration, which every checkpoint and every landing now calls; executor the store skill; the flag in the index's health view only, a sixth count (D8-draw-before-code changed-at-review; repaired at verify) |
| S6 | Important | what the user signs for a small change in a large system | batch — the touched diagram files plus the spine view when a container changes; the run keeps the neighborhood view as the sign-off picture (D8-draw-before-code changed-at-review) |
| S7 | Important | the deviation gate widens to every kind | batch — container and flow views only; the rest at the landing diff (D8-draw-before-code changed-at-review) |
| S9 | Important | D9-render-where-viewed names no superseded texts; "must parse" unenforced; publishing | batch — four texts named; parse the lead's duty; ask before publishing (D9-render-where-viewed changed-at-review) |
| S10 | Important | D1-focus-stage-first changes the order clause and drops the case-against | batch — retrofit-cost order in both tiers; each batch row with default, cost and case against; no supersession (D1-focus-stage-first changed-at-review) |
| S15 | Important | the convergence item's charter cannot carry the engine | batch — a new BACKLOG item for the shared stage; the convergence item unchanged (D6-desk-stays-engine changed-at-review; default added) |
| S17 | Minor | the desk writes the feature entry's link | batch — dropped; implement fills it (D5-visit-file-input changed-at-review) |
| S18 | Minor | the fail names "delta" and reads beyond desk visits | batch — scoped to desk visits, "delta" dropped (default amended) |
| S19 | Minor | the know-sentence's field on a decided row | batch — visit file always; trigger on `not-now`; first Rationale line on `decided`; numbers the user's (D2-part-direction-know changed-at-review) |
| S20 | Minor | a feature's visit file has no freshness rule | batch — read against the run's base; conflicts listed at the checkpoint (D5-visit-file-input changed-at-review) |
| S21 | Minor | the outside-claims re-check names no contract | batch — rides `EXTERNAL-CLAIMS.md`'s review-seat duty; pointer added at build (default amended) |
| S22 | Minor | the mark on D4-blind-list-visits; the batch mitigation untested | batch — `Confident` kept with its why; "pulls per batch" joins the bets (D4-blind-list-visits changed-at-review) |
| S24 | Minor | bare and drifting cites; five four-word slugs | batch — owner spans added; slugs renamed to three words, cites swept |

## Verify pass

Round 1 (2026-10-11, `reports/review.md` § Verify round 1): **21 CLEAN · 3 NOT CLEAN** (S4, S5,
S6), six fold-introduced defects V1–V6, status `needs-revision`; S1 CLEAN, no Critical left. All
nine lead-repaired the same round, each on the one card it touches:
- S4 and V2 — the compare's trigger widened to every index regeneration, with every checkpoint
  and every landing (structural or not) calling it; the flag moved back to the index's health
  view, outside `.mochiko/product/`, so it is never a run's unmarked hunk; diagram files carry no
  flag (D8-draw-before-code changed-at-review).
- V1 and S6 — one signed object: the run's `architecture.md` (neighborhood view plus the
  checkpoint table listing the edited diagram files); signing it signs the in-place delta-styled
  edits; the deviation gate anchors to it (D8-draw-before-code changed-at-review).
- V3 and S5 — "beside `.mochiko/product/architecture/`" corrected to outside `.mochiko/product/`
  altogether, working path `.mochiko/architecture/desk/<date-slug>/`; D3-visit-file-home's title
  and statement repaired.
- V4 — the *Who draws* default cites entities by file and heading.
- V5 — OQ2 names the new shared-stage BACKLOG item.
- V6 — D1-focus-stage-first's "two agreed screens" → three; D3-visit-file-home's "each desk
  visit" → each visit that rules something, and its "trigger or target" → per
  D2-part-direction-know; D8-draw-before-code's attribution of the nine touches now names
  D5-visit-file-input beside D9-render-where-viewed.
The reviewer's disclosure: its two reports carried the kernel-admission ID without its slug;
corrected to
`GI-019-kernel-tooling-admission` by the seat. Bounded round 2 requested over the repaired
cards only.

Round 2 (2026-10-11, `reports/review.md` § Verify round 2): **8 CLEAN · 1 NOT CLEAN** (V4, a
second time — to the user), one new blocking defect V7 and three Minor V8–V10, plus one
ungraded note outside the bound; status `needs-revision`. Per `brainstorm.fix-one-card-verify-once`
the second failed verify went to the user, who ruled (2026-10-11, "ok" · "yes, close"):
- V7 — stamped as the tenth implement touch by the user's word ("yes"): implement rebuilds the
  derived index at every design checkpoint and every landing (D8-draw-before-code
  changed-at-review; the S8 frame stamp extended).
- V4, V8, V9, V10 and the ungraded note — lead-repaired as text: "against the targets its
  `Draws:` line cites"; the *Readers* default reads the index's stale list with the diagram; D8-draw-before-code's
  accepted risk names the run's `architecture.md` as the signed object; the three bare IDs in
  this section rewritten; the index header carries the commit it was last regenerated at and
  the compare reads the diff since it.
- **Closed seat-unverified by the user's word ("close")**: the six repairs above were not
  re-read by the review seat; disclosed here and in the index entry. The ID check on the record
  reads 0 bare · 0 drift.

## Acceptance and landing

After-review screen shown 2026-10-11 — what changed per card, the disclosure of the six
seat-unverified repairs. **User: "accept."** The record is accepted.

Landing (KM close ritual, `.mochiko/memory/knowledge-management.md`), 2026-10-11:
- `DECISIONS.md` row appended (D1–D9 as review-amended, status `ruled`).
- `BACKLOG.md`: a new section *Architecture-desk brainstorm build* — the two build items (the
  focus stage and visit file; the diagram set and draw-before-code), the shared thinking-stage
  design session (S15), the *two notations + entity IDs* session (S1 hand-off), and the first
  ai-fileops desk run as the dogfood test; the existing *Three-command convergence session*
  item's trigger noted as fired.
- `ROADMAP.md`: folded into an existing Next row (caps held).
- `.mochiko/brainstorms/index.md`: this session's entry updated — status, review state,
  artifacts, landed.
- Build surface: two `plugin.json` bumps or one, the build's call; the supersessions-by-ruling
  this record names (`architecture-design-primitive` D5-plan-run-artifact and
  D8-rendered-diagram-signoff in part; the three rule texts under D9-render-where-viewed's
  changed-at-review; `authoring-architecture-store.health-view-binding`'s five counts; F27's
  "every visit" narrowed) are recorded at the build's strip entries, never silently.

Next-step offer (never a default): the build items above; the first ai-fileops desk run after
the build; a commit of this session's artifacts and operating-doc touches, suggested to the
user, never run by the session.
