# Producer-plan enforcement — decision record

**Topic:** how a producing seat's plan-first, read-only planning is enforced, and who QAs
that plan before work is spent — the author planning in native plan mode, the grader and the
Delivery Manager reviewing the plan. Driver ask (user, 2026-09-03): "how we can enforce the
author to use native plan mode that claude provides to plan and the grader and delivery
manager to QA the plan."

**Status:** accepted (2026-09-03) — build pending: wave 1 (plan-QA leg) · wave 2 (validator retirement, gated on wave-1 figures)
**Opened:** 2026-09-03
**Lead:** session lead (brainstorm charter, run inline in-conversation)

**Prior-session relations:** amends the plan-approval clause of
`command-architecture-realignment` D2/D3 (2026-08-02 — control moves to "native plan
approval", producer seats only) and the leg-1 shape of `charter-ritual-balance` D1
(2026-08-13, user-authored: "the producing seat plans first; the lead gives feedback,
approves the plan"), carried today by `patterns-sound-loop.leg-1-seat-produces`
(`class: floor`) and by the plan-approval clause each of the six command schemas carries in
its own shape (a `common.plan-approval-producers` stub in brainstorm/setup/specify; local
text in `impl.plan-approval-producers`; fused into `arch.seat-architect-producer` /
`arch.author-grader-separation` and `feat.author-grader` — no single rule ID spans the six). Composes with
`plan-structure-yagni` D2–D5 (plan-the-plan gate; `patterns-plan-minimalism` rung
justification for design seats), `teammate-message-races` D1–D7 (transport floor),
`model-tiered-seats` D5 (rostered seats never retier), and the kernel-class admission line
(GI-019; `schema-based-template-guidance` D11 the one admitted instance). The doctrine-only
enforcement posture restated in ADR `2026-08-19-explorer-retarget-native` ("no hooks,
nothing kernel-class") is in scope to be re-examined, not assumed.

---

## Ground facts

**Repo (whole-file reads, 2026-09-03):**

- **R1** — "plans first and works only on a plan you approved" is the entire mechanism in
  every command. No rule names plan mode, read-only tooling, a plan artifact path,
  plan-approval criteria, or a grader role on the plan. Grading and fact-finding seats are
  exempt (`impl.plan-approval-producers`, `impl.seat-sufficiency-independence`).
- **R2** — No skill grades a seat's plan. `review-plan-artifacts` grades the design-phase
  package after production; `patterns-plan-minimalism` gives design seats rung
  justification for design elements, not a plan-QA checklist.
- **R3** — Enforcement is doctrine-only: the plugin ships no hooks, no agent sets
  `permissionMode`; the `patterns-sound-loop` D5 rules-file leg stays deferred on its
  first-miss trigger (BACKLOG, open).
- **R4** — Plan rounds do happen in live waves (skill-content-schema, 2026-09-01: five
  plan-first producer seats, P1–P5 approvals lead-verified), but transport and mechanism
  went unrecorded; `teammate-message-races` F5, verbatim: "The main authoring phase (seats on
  approved plans, rulings at user gates) ran clean."
- **R5** — Maintainer-side precedent: `evals/commands` elicits a command lead's plan
  without execution and judges it against schema rules — advisory, and the lead's plan,
  not a seat's.

**Platform (code.claude.com/docs, fetched 2026-09-03; Claude Code 2.1.258):**

- **P1** — Agent teams: "A teammate that Claude spawns while the lead is in plan mode works
  in read-only plan mode until its plan is ready" and "Claude Code approves the plan in the
  lead's session as soon as the request arrives, without the lead reviewing it."
  Full sentence on modes: "You can change individual teammate modes after spawning, but you
  can't set per-teammate modes at spawn time" — the post-spawn half is user-mediated and
  per teammate, not a lead lever. Teams are experimental
  (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`, on in this repo's local settings).
- **P2** — Subagents: frontmatter `permissionMode: plan` exists but is "ignored when
  loading agents from a plugin" and ignored under auto mode; `ExitPlanMode` is stripped
  from a subagent unless its own `permissionMode` is `plan`; the Agent tool's `mode`
  parameter is deprecated ("Subagents inherit the parent session's permission mode").
- **P3** — Plan mode persists plans as files (`~/.claude/plans/`; `plansDirectory` setting,
  any scope); `ExitPlanMode` is a permission-required tool (user prompt, Ctrl+G edits the
  plan); `defaultMode: plan` is settable per project.
- **P4** — Hooks: plugins ship `hooks/hooks.json`; "Hooks from settings files, managed
  policy settings, and plugins also run inside subagents", input carrying `agent_id` /
  `agent_type`; `PreToolUse` matches any tool name (ExitPlanMode hookability implied, not
  documented); `SubagentStop` / `TeammateIdle` / `TaskCompleted` block with exit 2;
  `PermissionRequest` returns a decision object.
- **P5** — Costs: "Agent teams use approximately 7x more tokens than standard sessions when
  teammates run in plan mode." *Scope note (review, M6): this figure describes teammates in
  plan mode — the configuration D1/D7 reject; it is carried for provenance and is not the
  ruled shape's cost. OQ3 and the D8 watch own the cost question.* Teammates ignore an agent
  definition's `skills:` list.
- **P6** — Plugin agents honour `tools:` / `disallowedTools:` frontmatter; not `hooks`,
  `mcpServers`, or `permissionMode`.
- **P7** *(added at review, lead-verified against code.claude.com/docs/en/worktrees)* — While a
  session or subagent is isolated in a worktree, Claude Code blocks: an `Edit`, `Write`, or
  `NotebookEdit` targeting a path in the main checkout; a Bash command whose working
  directory resolves to the main checkout or cannot be verified to stay outside it; git
  redirects into the main checkout; and any command shape it cannot trace ("You can't turn
  this check off"). "Subagents that run in their own worktree carry the same checks."
  Worktree isolation is therefore native prevention of main-tree writes, not detection.
- **P8** *(added at review, lead-verified, same page)* — "Subagent worktrees use the same base
  branch as `--worktree`, so they branch from your repository's default branch unless
  `worktree.baseRef` is set to `"head"`"; `"head"` is documented for "isolating subagents
  that need to operate on in-progress work". A worktree is a checkout of committed state:
  uncommitted changes in the main tree are invisible inside it under either setting. A
  worktree left unchanged is removed when the subagent finishes; a changed one stays under
  `.claude/worktrees/`; the sweep leaves it in place while it still holds work, so it
  clears only by `git worktree remove`.
- **P9** *(added at review)* — In-repo precedent for a structural tool fence:
  `evals/commands/run.py:280` runs headless with `--allowedTools Read,Grep,Glob` (its probe
  report: "unlisted tools are denied at call time… Structural denial holds"). The seat
  analog is static `tools:` frontmatter; the Agent tool carries no per-spawn tools
  parameter. No agent file in the library declares `tools:` or `disallowedTools:`, so F1
  tested permission-mode inheritance only.

**Probe (this session, 2026-09-03; auto-mode start, agent teams on, scratch writes only,
repo tree clean throughout):**

- **F1 — writes NOT blocked.** Three seats spawned while the lead sat in plan mode all
  created their scratch file with the Write tool and again with Bash: a
  `mochiko:staff-engineer` unnamed subagent, a `mochiko:staff-engineer` named teammate
  (confirmed in the team config), and a built-in `general-purpose` subagent. The subagents
  received no plan-mode text at all and had no `EnterPlanMode` / `ExitPlanMode`. The
  teammate received the "Plan mode is active… you MUST NOT make any edits" reminder with
  its own plan-file path — after both writes had already succeeded; no plan-approval
  request or response occurred. "Lead in plan mode ⇒ producers read-only" is false on both
  transports.
- **F2 — no seat plan file.** Only the lead's plan appeared under `~/.claude/plans/`; a
  seat's plan exists only as its returned text.
- **F3 — resume keeps context.** A seat resumed by agent ID repeated its earlier results
  verbatim; a two-dispatch plan/approve/execute shape is viable.
- **F4 — the native user gate is lead-level.** `ExitPlanMode` gated only the lead's own
  plan; nothing a seat did passed through it.

## Problem — why this session (driver: F1, user-ruled)

The library's only control over producer work before it is spent — plan first, lead approves
— is a sentence, and the native mechanism the 2026-08-02 ruling relied on for teeth does not
bite: teammate plan approval is auto-granted (P1), subagents never enter plan mode (F1), and a
plan-mode lead does not constrain its seats on either transport (F1). Two consequences:
"enforce" today means "instruct"; and no seat reviews a plan before the lead approves it,
so the approval is one context's judgment on a plan whose review it will also adjudicate.

## Decisions

*(D1… as ruled — statement · rationale · confidence mark; review amendments marked X-n)*

### D1 — Enforcement class: detection plus review, no hook gate, no worktree — `Contested` *(reclassified at review X1; worktree removed at acceptance, user-ruled)*

**Statement:** a producer seat's planning is enforced by detection and review, never by a
write-blocking hook and never by worktree isolation. The planning dispatch spawns the seat
in the main tree with a plan-only brief; the seat returns its plan as text and stops; the
lead compares `git status --porcelain` against the snapshot it took before the dispatch,
and any new or changed path is an automatic FAIL of that planning round (D6); a non-author
grader QAs the plan before the lead approves. No plugin-shipped hook, no kernel-class
admission ruling, no `isolation: worktree`.

**Roads seen and declined:** (1) a hook gate — prevention that is executable and gating,
across the GI-019 bright line — offered at Q1 as A and C; the user ruled B alone, and no
escalation road is recorded: a future hook gate takes its own ruling from zero. (2)
Read-only planner persona variants via `tools:` / `disallowedTools:` frontmatter (honoured
for plugin agents and teammates, P6/P9) — surfaced at review, declined by the user (X1):
one duplicate persona per producer, and Bash cannot be dropped without breaking planning
reads. (3) Worktree isolation of the planning seat — adopted at Q7, shown at review to be
native prevention of main-tree writes (P7), **removed at acceptance by the user**: with
native plan mode out of the design, the user would rather not carry the worktree's
machinery (base-branch setting, in-flight-path briefs, left-behind trees, three probes)
for the planning step; detection covers the failure at the cost of a user cleanup on a
miss.

**Rationale:** F1 shows no permission-mode lever constrains a seat's writes during
planning. The record's original claim — "the only prevention available is executable and
gating" — was false (P7, P9): two native preventions exist, and both were seen and declined
on cost. What stands is the user's B: an instruction the seat follows, a detector that
catches the miss, a grader and the lead who never approve an ungraded plan, and n=0 live
misses to price a stronger lever against.

**Confidence:** Contested — the lead recommended C at Q1 (B now, hook road recorded); the
user ruled B alone, deliberately, and at acceptance removed the worktree the lead's Q7
recommendation had added.

### D2 — Plan home: transient — verbatim text in the grader's brief and the lead's approval; the verdict survives, the plan does not — `Confident`

**Statement:** a seat's plan is not persisted. The seat returns it as text; the lead passes
that text verbatim — never a summary — in the plan grader's brief and holds it for the
approval; the close report's disclosure line carries the outcome per seat in one grammar
(X14): `plans: <seat>:PASS|FAIL(n)`, `n` the re-plan rounds consumed on one shared counter
(a grader FAIL or a lead bounce each consume it; with D6's bound of one, `n ≤ 1` before
escalation — M4), with `dirty` appended when the seat changed the tree during planning (X7 as amended:
the main tree, since no worktree is used). No new artifact folder, no gitignored plans directory, no scaffold change.

**Rationale:** the plan is transient by nature and the grade is what must survive. Verbatim
text in a brief is the authored surface itself, so author≠grader's "never the author's
report" holds. The token epic found the reporting layer nobody reads outweighed the design
layer; persisting one plan per seat per round would rebuild that layer. Two homes was the
cost of option a; a scaffold change the cost of option c.

**Confidence:** Confident — ruled "as recommended".

### D3 — Plan grader: a fresh spawn of the author's own persona type; `validator` retires library-wide, timing deferred to a second wave; devils-advocate stays out of the plan loop for now — grader `Contested` · retirement `Confident` on intent, `Deferred` on timing (X3) · DA rider `Deferred`

**Statement:** a seat's plan is graded by a fresh spawn of the same persona type as its
author — a `staff-engineer` plan by a fresh `staff-engineer`, a `technical-analyst` plan by
a fresh `technical-analyst` — never the author's own context, never the lead. A producer
spawned without a mochiko persona is graded by a fresh generic seat carrying the criteria
block in its brief (X3, closes OQ2). The `validator` persona retires library-wide, not only
here — wherever it grades today (primitive-edit audits of command pairs and converted skill
pairs, the setup loop's validate step via `validation-constitution`) — **but in a second
wave** (X3 a), after the first wave's plan-grade figures are in and only once the
replacement carrier is in place: the default-FAIL posture and the audit criteria ride the
grader brief, and the grader is a fresh seat that authored nothing it grades. CLAUDE.md's
axis 5 is reworded to that standard — "graded by a structurally independent grader: a
fresh seat that authored nothing it grades, running a different skill from the author's" —
and the router's mount doctrine line becomes "never mount producing and grading skills for
the same artifact on one seat" (the persona may hold both; the seat never grades what it
produced). `devils-advocate` is not a seat in the plan loop; it keeps its existing surfaces
(specs, design packages, records, governance intent, gap-finding). "For now" — the user's
words — so re-adding it is a future ruling, not a standing trigger.

**Rationale (user):** a domain peer reads a plan with the judgment the plan needs, and a
fresh spawn keeps the independence; a persona whose only craft is checklist grading is one
persona too many. Devils-advocate gives opinions rather than a grade, the wrong instrument
for a plan gate. **Timing (review, X3):** retiring the only generic default-FAIL grader
before its replacement has graded once would leave the NON-NEGOTIABLE GI-004 audits with an
undefined grader and make the retiring wave grade its own landing under an untested
regime; the split lets the first wave produce the figures the retirement waits on.

**Consumers the retirement reaches (enumerated at record time; exact strip inventory owed
at the second wave):** `plugins/mochiko/agents/validator.md` + its `plugin.json` entry + the
router's three validator-bearing lines in `skills/mochiko/SKILL.md` (row, mount doctrine
line, checklist routing) · `CLAUDE.md` (three `mochiko:validator` ceremony mentions; GI-004
pointer; **axis 5 of the skill-library conventions**, X6) · `.claude/rules/mochiko/primitive-edits.md`
(grader named for command and skill pairs) · `validation-constitution` (validator-side
skill: grader identity re-worded, procedure unchanged) · `authoring-constitution` schema +
`governance-surfaces.yaml` / `governance-intent.yaml` ("the validator" as a role word,
re-worded where it names the persona) · `.mochiko/memory/primitive-cost-budgets.md`
(validator agent budget row retires) · `.mochiko/memory/governance-ledger.md` (GI-004
detail) · repo-level `compressing-skills` skill · `.mochiko/provenance.yaml` anchors · eval
rules naming the validator. Prior rulings partly superseded: `validator-scope-and-verbosity`,
`validator-worktree-isolation`, `author-grader-value-tiering`, ADR
`2026-08-26-validator-router-indexed-checklists` — each gets its supersession row at that
landing.

**Confidence:** grader identity `Contested` (lead recommended `validator` + mirror
checklist; user ruled the peer form) · retirement `Confident` on intent (user-authored),
`Deferred` on timing (second wave, X3 ruled "as recommended") · devils-advocate exclusion
`Deferred` ("drop for now").

### D4 — Criteria carrier: a new review skill, `review-seat-plan`, pair form — `Confident` *(delivery clause added at review)*

**Statement:** the plan-QA criteria live in one new skill, working name `review-seat-plan`,
shipped in the review-family pair form (`SKILL.md` + in-directory `schema.yaml`, the family
six-set, `skill-review-common.yaml` stubs where the near-dup bar licenses them). Delivery
runs two ways at once: the skill is model-invoked — trigger phrases in its `description`
within the 1,536-character budget, for description-triggered delivery in normal runs — and
every grader brief names it besides, because the grader persona varies (D3) and a
persona's preloaded `skills:` cannot carry it. Pre-install path (X15): until the install
refresh of D8 item 10 lands, a grader Reads the repo pair by path
(`plugins/mochiko/skills/review-seat-plan/SKILL.md` + `schema.yaml`); the wave records
which path each grade used. Criteria are referenced, never restated, in command schemas and
briefs.

**Rationale:** the grader is any persona type, so the checklist cannot ride a persona's
preloaded `skills:`; a schema block relayed through briefs makes the brief the home and
restates the criteria six times; folding into `patterns-sound-loop` puts a grading
procedure in a discipline skill and blurs the patterns-versus-review split the census
families rest on. One skill, one budget seed, one pair build.

**Confidence:** Confident — ruled "as recommended".

### D5 — The plan-QA criteria, ratified as proposed; item 4 renamed at review (X4) — `Confident`

**Statement:** `review-seat-plan` grades a plan default-FAIL against seven items; a FAIL
cites the item and gives the fix.

1. **Scope fidelity** — the plan covers exactly the brief's assigned scope (named gap, card,
   or deliverable); anything beyond, or anything owed and missing, fails it outright.
2. **Write set declared** — every path the seat will create or change is listed, disjoint
   from other seats' declared sets, none outside the brief's surfaces.
3. **Reads named** — where the plan claims something already exists or should be reused, it
   names what it read to know that; no on-trust claims.
4. **Rung claims present** — design seats: each design element carries its
   simplest-execution rung claim per `patterns-plan-minimalism` (blocking on presence; the
   claim's honesty stays advisory, per `plan-structure-yagni` D5 as ruled 2026-08-12 —
   nothing superseded); builders: each task names the test it will write first.
5. **Stops and hand-offs named** — user-reserved questions appear as questions, never as
   decisions; the verification seat or `**TEST:**` gate the work hands to is named; the
   attempt bound is acknowledged.
6. **No self-clearing step** — no step where the seat grades, accepts, or lands its own
   output.
7. **Size bound, advisory** — a plan the grader cannot read in one sitting is a finding,
   never a FAIL.

Items 1–6 blocking, 7 advisory. The plan grade is **additive** to sound-loop leg 2, never a
substitute: the produced artifact is still graded by a non-author seat before the user's
gate (X9).

**Rationale:** items 1, 2 and 6 are the three failure modes the floor exists for (scope
creep, write collisions, self-clearing); 3 and 4 import the read obligation and the
rung-claim presence the plan-minimalism ruling already binds on design work — honesty
stays advisory there and here; 5 keeps the user gate and the verification hand-off visible
before work is spent; 7 guards against plan bloat without turning length into a gate.

**Confidence:** Confident — "ratify"; item 4's presence reading ruled "as recommended" at
review.

### D6 — Approval and rework semantics — `Confident` *(three clauses added at review)*

**Statement:**
- The lead approves only on a PASS. It may send a PASSed plan back on judgment, with
  feedback; that counts as a re-plan round. It never approves a FAIL.
- Re-plan bound: one, on a single shared counter (grader FAIL or lead bounce). After a FAIL
  the seat revises against the fix list and the same grader seat re-grades (resumed — it
  owns the fix list; fresh only if it is gone). A second consumption goes to the user with
  both verdicts: re-plan again, re-staff, or narrow the scope.
- A dirtied tree is its own FAIL: the lead snapshots `git status --porcelain` before the
  planning dispatch and diffs after the seat returns; any new or changed path fails the
  round whatever the plan says, and the seat is told why. A second dirty round goes to the
  user as a seat that will not plan read-only. **Cleanup (X7 as amended at acceptance):**
  the lead never reverts (`common.no-git-mutations`); it lists the paths, suggests the
  revert to the user, and marks the seat `dirty` in the disclosure line. Until the user
  rules, no execute dispatch runs on that tree.
- No new user prompt per plan. Verdicts reach the user through the disclosure line and the
  gates that already exist (run-open confirmation, card confirm, design checkpoint); a plan
  that raises a reserved-to-user question rides the existing escalation-batching rule.
- **No size threshold (X5, user-ruled as recommended):** the planning dispatch inherits the
  floor's kind test. A one-line judgment-authored write on a governing surface takes the
  planning dispatch, the peer grade and the approval like any other; the cost is priced by
  the D8 watch, never carved — a carve would be the size gate rejected at
  `charter-ritual-balance` D2.
- On the three commands with no chartered Delivery Manager (`brainstorm` · `setup` ·
  `specify`) every schema text says "the lead" (M5); the DM wording belongs to
  `architecture` · `feature` · `implement` only.

**Rationale:** the approval stays the lead's (leg 1 as ruled 2026-08-13) but can no longer
clear a plan no peer has passed; one bounded re-plan mirrors the per-grade attempt economy
the build already runs under, and the second miss is the user's call, never the lead's; the
dirty-tree FAIL is D1's detector, independent of plan quality; adding a user prompt per plan would refight the 2026-08-02 choreography ruling and
the escalation batching rule.

**Confidence:** Confident — "ratify"; review clauses "batch as recommended".

### D7 — Dispatch shape: one seat plans, stops, and is resumed to execute on the approved plan — `Confident` *(re-shaped at acceptance, user-ruled: no worktree)*

**Statement:** the producing seat is spawned once, as a subagent by default, in the main
tree, with a plan-only brief. It returns its plan as text and its turn ends — on the
subagent transport it cannot act again until the lead resumes it, so the wait for approval
is structural (F3: a resumed seat keeps its context verbatim). The peer grades (D3–D5),
the lead approves (D6), and the lead resumes the same seat with "approved" and the plan or
fix-list text quoted (transport-floor leg 4). A teammate planning seat is the lead's call
for cause; it holds by mesh hold only, disclosed. Grader seats spawn fresh. Transport stays
the lead's per-seat call (`command-architecture-realignment` D5 as narrowed 2026-08-14).

**Rationale:** with no worktree there is nothing to merge back, so the planning seat can
execute; keeping it saves the discarded context per producer that the two-spawn shape cost
(OQ3) and drops the base-branch setting, the in-flight-path brief line, the left-behind
trees and the three worktree probes. Two prices, both disclosed: read-only planning is
now instruction plus detection, the enforcement class the user ruled at Q1; and the plan
is no longer the sole contract — the resumed seat executes carrying everything it thought
while planning, written down or not, so the peer grade binds the plan, not the seat's
whole intent.

**Roads recorded:** the worktree two-spawn shape (Q7 a, review-amended with
`worktree.baseRef: "head"`) stands as the recorded alternative if live misses show the
detector is not enough — it is native prevention, no ruling on machinery needed to adopt
it, only this record's amendment.

**Confidence:** Confident — the shape has no unverified platform premise (return-until-
resume and context retention are session-evidenced, F3; `git status` detection is
deterministic); the residual unknown is the live miss rate, which the D8 watch measures.

### D8 — Landing set — `Confident` *(amended at review: items 0, 9, 10 added; items 1, 2, 4, 5, 7 amended; item 8 struck at acceptance with the worktree)*

**Statement:** the ruling lands in two waves (D9), each move under the primitive-edit
ceremony (strip → record → author≠grader audit, the grader a fresh seat that authored
nothing it grades):

0. **KM landing ritual** (I7) — `DECISIONS.md` rows for this record and the supersession
   annotations below; a `BACKLOG.md` item minted for the build and moved to the trail at
   landing; `ROADMAP.md` Now/Next touched; statuses agreeing across the brainstorms index,
   this record and the decisions index.
1. **Sound-loop leg 1 amended** — `patterns-sound-loop.leg-1-seat-produces` (`class: floor`,
   user-authored 2026-08-13) becomes: "the producing seat plans first, read-only, in a
   plan-only dispatch, and stops; a fresh peer of its persona type grades the plan per
   `review-seat-plan`; the lead approves only a passed plan and resumes the seat to work.
   The plan grade is additive to leg 2, never a substitute."
   Supersession-by-ruling strip on the floor rule; the ID survives; the floor pin count
   (6) is unchanged.
2. **Command schemas swept** (I8, I1, M5) — every site restating the obligation:
   `common.plan-approval-producers` (three stubs), `impl.plan-approval-producers` (its
   exemption set keeps `verification`), `impl.design-gaps-only`,
   `impl.builder-decompose-disclose`, `arch.seat-architect-producer`,
   `arch.author-grader-separation`, `feat.author-grader`, and the three pointer rules
   `arch.sound-loop-floor` / `feat.sound-loop-floor` / `impl.sound-loop-floor` — reworded to
   name the mechanism; "the lead", never DM, on brainstorm/setup/specify; identical 3+
   texts converge through `common.yaml` (near-dup R1–R6), the detector residue accounted
   in `scripts/similar-rules-allowlist.yaml`. One strip per command.
3. **2026-08-02 basis superseded** — `command-architecture-realignment` D2's "native plan
   approval" clause gets its supersession annotation in `DECISIONS.md`; D3 (producers
   only) stands.
4. **Disclosure line** — `patterns-sound-loop.disclosure-line` gains one segment after the
   seats: `plans: <seat>:PASS|FAIL(n)` with `dirty` appended for a tree changed during planning;
   `floor: clear` stays bare (X14).
5. **`validator` retired — second wave** (X3): agent file, `plugin.json` entry, the
   router's three validator-bearing lines; grader wording in `CLAUDE.md` (three ceremony
   mentions + axis 5), `.claude/rules/mochiko/primitive-edits.md`, `validation-constitution`,
   `authoring-constitution` schema, the budgets and governance ledgers; supersession rows for
   the four validator rulings named in D3. Gated on the first wave's figures and the named
   carrier.
   5′. **First wave:** `primitive-edits.md` criterion 6 is rewritten from "Plan approval
   before any producing seat works" to the peer-graded standard (I9), so audits never grade
   against the retired rule.
6. **New `review-seat-plan` pair** — review-family form, router row, first-seed budget,
   provenance anchors for D5's seven criteria, the D4 two-way delivery clause.
7. **First-live-run watch with a stop condition** (X10) — the first wave records plan
   verdicts per seat, re-plan rounds, dirty-tree events and added grader seats; it **halts
   and reports to the user** when plan FAILs exceed half the wave's producers, any seat
   reaches a second re-plan, or any planning dispatch dirties the tree.
8. *(struck at acceptance — the `worktree.baseRef: "head"` scaffold went with the worktree;
   X2 is closed by the same ruling: a seat planning in the main tree sees the tree it will
   execute in.)*
9. **`patterns-plan-minimalism.grading-routing` re-pointed** (I2/X13) — seat-plan rung-claim
   presence routes to `review-seat-plan`; design-package rung honesty stays with
   `review-plan-artifacts`, advisory.
10. **Install refresh gate** (X15) — before the wave's first dispatch the plugin is
    reinstalled from the local marketplace at the repo version; until then graders Read the
    repo pair by path (D4), and the wave's evidence names which path each grade used.

**Rationale:** every move is a consequence already ruled (D1–D7 as amended); listing them
here makes the landing reconstructible (GI-006) and gives the build wave its inventory.
Item 1 is the one line the lead could not amend without the user's word.

**Confidence:** Confident — "ratify"; amendments "batch as recommended".

### D9 — Rollout: two waves, the first running under the leg it ships — `Assumed (n=0)` *(split at review, X3; re-marked X6; abort gate re-keyed at acceptance)*

**Statement:** **Wave 1** lands D8 items 0–4, 5′, 6–10 — the plan-QA leg — and runs under
it: its producer seats plan in plan-only dispatches, fresh peers grade their plans per D5, the lead
approves on PASS (D6), and its audits are graded by fresh seats that authored nothing they
grade. Wave 1 is therefore the first live run under the new leg, and item 7's watch starts
with its own figures. **Abort gate:** the install refresh (item 10) lands before wave 1's first
planning dispatch; the worktree probes are struck with the worktree (OQ1 closed).
**Wave 2** lands D8 item 5, the `validator` retirement, after wave 1's figures are in and
the replacement carrier is in place (D3).

**Rationale:** the peer-grade leg is cheap to run inside the wave; a single wave would have
retired the only generic default-FAIL grader before its replacement had graded once, on the
wave touching a `class: floor` rule and the GI-004 ceremony (X3); the split is the repo's
own staged-rollout pattern (skill-content-schema's sequenced waves, command-content-schema
D10). Precedent for building under the ruling being built: `charter-ritual-balance` v0.70.0.

**Confidence:** Assumed (n=0) — "yes a" at Q9 (sixth straight adoption), split "as
recommended" at review; no live figure exists yet for the leg's cost or its miss rate.

## Session trail

- 2026-09-03 — session opened after the probe; the user ruled the driver: F1.
- **Q1** enforcement class — A prevention (hook gate, kernel ruling) · B detection plus review · C = B now + A as recorded escalation road. Lead recommended C. **A1 (user): B** — D1, `Contested`.
- **Q2** plan home — a run artifact folder (persisted, committed) · b transient (verbatim text in brief + approval, disclosure line carries the verdict) · c gitignored `.mochiko/plans/`. Lead recommended b. **A2 (user): "as recommended"** — D2, `Confident`.
- **Q3** plan grader — a `validator` mirror checklist (new `review-seat-plan` pair) · b `devils-advocate` critique · c extend `review-plan-artifacts`. Lead recommended a. **A3 (user):** "get rid of validator; same persona type, fresh spawn; but devils advocate gives opinions too" — lead asked one clarification (DA in or out of the loop); **user: "drop devils advocate for now"** — D3, split marks.
- **Q4** criteria carrier — a new `review-seat-plan` pair · b shared `common.plan-gate` block relayed in briefs · c fold into `patterns-sound-loop`. Lead recommended a. **A4 (user): "as recommended"** — D4, `Confident`.
- **Q5** the seven plan-QA criteria proposed (1–6 blocking, 7 advisory). **A5 (user): "ratify"** — D5, `Confident`.
- **Q6** approval + rework package (DM approves only on PASS · one re-plan round · dirty-worktree FAIL · no new user prompt). **A6 (user): "ratify"** — D6, `Confident`. *Adoption-streak flag: Q4–Q6 three straight recommendations adopted; raised to the user at Q7.*
- **Q7** dispatch shape — a separate plan seat (worktree, discarded) + fresh execute seat with the approved plan verbatim · b same seat, no worktree, `git status` detector. Lead recommended a; streak flagged. **A7 (user): "as recommended"** — D7, `Confident`.
- **Q8** landing set, seven moves. **A8 (user): "ratify"** — D8, `Confident` (fifth straight adoption, Q4–Q8).
- **Q9** rollout — A one wave, build under the leg it ships · B pilot on implement. Lead recommended A. **A9 (user): "yes a"** — D9, `Confident`.
- **Convergence** — record frozen for cold review; review sizing put to the user as a named gate: pair lens-split (recommended) · single · none (waiver). **User: "as recommended"** — pair.
- **Cold review dispatch (2026-09-03)** — transport floor invoked (message lane fires: legs 3, 4, 6, 7 held; no shared write surface; platform 2.1.258 ≥ 2.1.224). Two `devils-advocate` seats spawned as named teammates, blind two-message dispatch: message 1 = topic statement + goal line + lens only, fence on all of `.mochiko/brainstorms/`; `review-dq` (decision-quality) · `review-ri` (record-integrity). Record frozen at D1–D9 + OQ1–OQ6 from this line on until verdicts.
- **Acceptance-stage amendment (2026-09-03, user-ruled):** before accepting, the user asked for the workflow by example and whether native plan mode had any role (it has none at seat level — P1, P2, F1); ruled **no worktree** for the planning seat ("I would rather not use worktree if we don't plan to use native plan mode"). Folded: D1 back to detection plus review (worktree recorded as a declined road with its cost), D6 dirty-tree detector via `git status` snapshot with user cleanup, D7 re-shaped to one seat that plans, stops, and is resumed (`Confident`), D8 item 8 struck, D9 abort gate re-keyed to the install refresh, OQ1/OQ7 closed, X2 closed. One bounded verify round owed on the touched decisions.
- **Accepted (2026-09-03, user: "accept")** — after the pair review, the verify/delta-check rounds, and the acceptance-stage amendment's bounded verify + delta-check, all CLEAN. Landing per the KM close ritual: `DECISIONS.md` row + supersession annotations on the 2026-08-02 and 2026-08-13 rows · `BACKLOG.md` build item (waves 1 and 2) · `ROADMAP.md` Next "Floor builds" line touched (no new Next item — the horizon cap is at 7) · index entry updated. Sound-loop D5 first-miss trigger: **not fired** — this session's probe writes were scratch files, and no judgment-authored governing-surface write ran without the loop. Disclosure: `floor: tripped · seats: lead (record, by charter) / review-dq + review-ri`; operating-doc edits are transcription.
- **Cold review closed (2026-09-03)** — both survivor reports in (see Review + disposition trail); 23 merged survivors dispositioned: X1–X4 one by one, X5 and the rest "batch as recommended"; folds applied by the lead's pen; record re-frozen for the verify pass.

## Review + disposition trail

**Sizing gate (2026-09-03):** lead recommended a lens-split pair; user: "as recommended".
**Dispatch:** two `devils-advocate` teammates, blind two-message dispatch (message 1 topic +
goal line + lens, fence on all of `.mochiko/brainstorms/`; message 2 the record path after
each map arrived): `review-dq` (decision-quality, 9-cluster map A–I) · `review-ri`
(record-integrity, 34-angle map A–F). Transport floor: message lane fired (legs 3, 4, 6, 7
held); no shared write surface; findings entered only through the lead's pen.
**Cold reads:** review-ri 18 formed (4 C / 9 I / 5 M), fitness 7/7 then revised to 6/7;
review-dq 13 formed (4 C / 6 I / 3 M), fitness 6/7. Counts crossed first, contents held.
**Cross-examination:** four messages by role. review-ri withdrew its C2 in full (worktree
main-checkout writes are harness-blocked — the fact reversed into C1's second limb),
narrowed I10 from Critical, corrected one inventory slot and found three more unswept
sites, withdrew half of M1, adopted review-dq's fitness grade; review-dq withdrew M1 and M3
outright, withdrew the 7x limb of I5, downgraded C2 to Important. Neither seat conceded
anything it was not shown wrong on.
**Survivor reports:** review-ri 18 raised / 17 survived, `critical-gaps`; review-dq 13 raised
/ 11 survived, `critical-gaps`. Both reached the status independently. Lead-verified the two
new load-bearing facts before dispositions (P7–P9 added).

**Merged survivor set (28 raw → 23 merged) and dispositions.** The user opened the four
Criticals one by one and ruled "batch as recommended" for the rest.

| # | Sev | Source | Finding (short) | Disposition |
|---|---|---|---|---|
| X1 | C | ri C1 + dq C1 | D1's "only prevention is a hook" false twice (P6/P9 tools frontmatter; P7 worktree = prevention); Q1 never saw the planner-persona road | **User: a** — D1 reclassified; planner-persona road recorded declined; B stands |
| X2 | C | dq C3 | planning worktree cut from `origin/main`; plan formed against a different tree | **User: a** at review; **closed at acceptance** by the user's removal of the worktree — the seat plans in the tree it executes in |
| X3 | C | dq C4 + ri I3 + dq I1 + dq I3 | validator retirement outruns its rationale; GI-004 grader undefined; wave grades its own landing; axis 5 contradicted; pilot dismissed on undefended premise | **User: a** — wave split (D9); retirement deferred on timing with carrier named (D3); axis 5 reworded + added to consumers |
| X4 | C | ri C3 | D5 item 4 "rung honesty" blocking flips `plan-structure-yagni` D5 (advisory) | **User: a** — renamed "Rung claims present", presence blocking, honesty advisory; no supersession |
| X5 | I | dq I6 + ri I5 | no-size-threshold inheritance prices a one-line edit at two spawns + grader | **User: batch as recommended** — inherit, no carve (D6) |
| X6 | I | dq C2↓ + ri I10 + fitness | D7 `Confident` on unread docs / unverified premises; D9 `Confident` at n=0; no retreat recorded | batch — D7 `Assumed`, D9 `Assumed (n=0)`; fallback Q7 b recorded; OQ1 = abort gate |
| X7 | I | dq I2 | dirty-worktree FAIL leaves git litter the lead may not clean | batch — user cleans via suggested `git worktree remove`; `dirty` in disclosure line (D6, D2) — mechanism amended at acceptance: dirty main tree, revert suggested to the user, not `git worktree remove` |
| X8 | I | dq I3 | (folded into X3) | — |
| X9 | I | dq I4 | plan grade could be read as discharging leg 2 | batch — additivity sentence in D5 and D8 item 1 |
| X10 | I | dq I5 + ri M6 | recurring cost unbounded; P5 figure irrelevant to ruled shape | batch — stop condition on the watch (D8 item 7); P5 scoped (ground facts) |
| X11 | I | ri I5 | (merged into X5) | — |
| X12 | I | ri I1 | D8 item 2 would drop `verification` from implement's exemptions | batch — retained (D8 item 2) |
| X13 | I | ri I2 | `patterns-plan-minimalism.grading-routing` points at the wrong grader; not in landing | batch — D8 item 9 |
| X14 | I | ri I4 | two disclosure grammars (D2 vs D8 item 4) | batch — one grammar `plans: <seat>:PASS|FAIL(n)` |
| X15 | I | ri I6 (corrected) + dq M2 | skill absent from installed plugin during the wave; Read-by-path unruled; D4 "model-invoked" vs brief-naming | batch — D4 two-way delivery clause + pre-install Read-by-path; D8 item 10 install refresh gate |
| X16 | I | ri I7 | D8 omits the KM landing ritual | batch — D8 item 0 |
| X17 | I | ri I8 | leg-1 substance restated at five unswept sites; detector residue | batch — D8 item 2 inventory extended; `common.yaml` convergence |
| X18 | I | ri I9 | `primitive-edits.md` criterion 6 never swept | batch — D8 item 5′ (wave 1) |
| X19 | M | ri M1 | header names a nonexistent six-schema rule ID | batch — header reworded |
| X20 | M | ri M2 | R4 paraphrase presented as quotation | batch — verbatim quote |
| X21 | M | ri M3 | P1 truncated the post-spawn-modes sentence | batch — sentence restored |
| X22 | M | ri M4 | re-plan counter shared but unstated | batch — stated (D2, D6) |
| X23 | M | ri M5 | DM prose on three non-DM commands | batch — "the lead" on those three (D6, D8 item 2) |

Withdrawn before disposition: ri C2 · dq M1 · dq M3 · dq I5's 7x limb · ri M1's second
half. Fitness: "confidence marks honest" restored by X6's re-marks.

**Verify pass (review-ri, record-integrity lens, one bounded round):** NOT CLEAN — 4 items,
all Minor, none blocking: P8 dropped the sweep's "without losing work" qualifier (fold-introduced) ·
one present-tense DM echo in the Problem section · session-trail close/dispatch lines out of
order · index status word differing from the record. All four lead-repaired the same round;
all 17 record-integrity survivors graded landed; P7, P9 and the floor-pin count spot-checked
against sources; fitness 7/7 restored. Observation carried: X8 and X11 are cross-reference
rows, so the merged table holds 21 substantive dispositions across 23 rows.
**Delta-check (review-ri, bounded to the four repairs):** CLEAN — each repair verified at its
site; status words agree across record and index. Observation taken: the index About line
now says "the lead" too. The record is ready for the user's acceptance.
**Bounded verify on the acceptance-stage amendment (review-ri):** NOT CLEAN — 2 Minor: OQ3
still billed the discarded planning context; the X7 table row still named `git worktree
remove`. Both lead-repaired; the observation (resumed seat carries unwritten intent) folded
into D7's rationale as a disclosed price. Worktree references, two-spawn assumptions, D7's
`Confident`, and status agreement all graded CLEAN.
**Delta-check (review-ri, bounded to the two repairs):** CLEAN. The record is consistent as it
stands; acceptance is the user's.

## Open questions

*(elicited unknowns land here as they arise; none silently dropped; status after review)*

- **OQ1 — worktree mechanics:** *closed at acceptance* — the user removed the worktree
  from the design (D1/D7); the recorded alternative in D7 would reopen it.
- **OQ2 — peer for persona-less producers:** *resolved at review (X3)* — a fresh generic seat
  carrying the criteria block in its brief; carried into D3.
- **OQ3 — cost delta:** one added grade per producer seat per run (the discarded planning
  context went with the worktree, D7). Unmeasured; the D8 item 7 watch records it and now carries a stop
  condition (X10). P5's 7x figure does not describe this shape (M6).
- **OQ4 — devils-advocate re-entry:** excluded "for now" (D3). No trigger recorded; a future
  ruling re-adds it if plan grades prove too lenient.
- **OQ5 — installed-plugin skew:** *resolved at review (X15)* — D8 item 10's install refresh
  gate plus D4's pre-install Read-by-path; the wave's evidence names the path used.
- **OQ6 — `ExitPlanMode` hookability:** implied by the hooks matcher grammar, undocumented;
  moot under D1 (no hook gate), kept for any future prevention ruling.
- **OQ7 — `isolation: worktree` as agent-definition frontmatter on plugin agents:** *moot at
  acceptance* with OQ1; kept for the recorded alternative.
