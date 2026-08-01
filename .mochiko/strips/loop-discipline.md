# Strip notes — `skills/loop-discipline/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave-1 open — the R4b
≥3-consumer queue ruling, user-approved 2026-07-25 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`).

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

## [v0.44.0] Keystone-test doctrine pointer
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(Full doctrine + the grep-checkable deny-list: `.mochiko/brainstorms/agent-decoupling/synthesis.md`.)
```
- **Kept deliberately:** the keystone test itself, whole — the any-job question, the intrinsic-traits carve-out, the this-loop-machinery list, and decoupling-proven-by-absence.

## [v0.44.0] Archived-ROADMAP source-techniques pointer (Reference Files list)
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
- See the archived mochiko `ROADMAP.md` ("The sound-loop doctrine", `.mochiko/archive/ROADMAP.md` in the mochiko repo) for the source techniques
```
- **Consumers assessed:** the two live reference rows above it (`workflow-contract` and `agent-dispatch` templates) are untouched — both point at shipped files.

# v0.40.0 — the D6(a) rewrite, ratified on its own acceptance card

**Wave context:** `lead-owned-process-flexibility`
(`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`), **D6(a) ratified at
acceptance A1 on its own card** — R31 gave it one because doctrine surgery on the skill
carrying mochiko's whole loop discipline deserves its own yes, and the card shows the internal
`Contested` element (U1-C's tier-ranking decline). Wave note: `.mochiko/strips/command-shape.md`
[v0.40.0]. Body **12,291 → 15,124 B** (words 1,946 → 2,367, +22%), measured after the repair
round. A growth revision, and
nothing was loosened: requirements 1 and 4 are untouched, and 2 and 3 gain clauses.

**Scope, and it is narrower than the build brief's compression.** R28 scoped D6(a) to
**command-supervised loops**; *"non-command loops keep the four requirements exactly as written
today"*. So the skill did **not** narrow to command-scope — it gained a **two-scope** framing
paragraph, and each amendment below is marked as command-supervised. An agent loop or a skill's
own produce → check still reads all four exactly as before, including guard 1's static
deterministic ceiling (F11's text, unchanged).

**Additions this revision** — recorded for the decision row, not as strips: the two-scope
paragraph under the sound-loop box · requirement 2's U1-B floor clause — carrying the waiver
qualifier **"at the weight card"** verbatim, added at the repair round so the three files that
state invariant 2 (`command-shape.md` Layer 1, this skill, `sized-end-stage-review.md`) state it
at one strength rather than three · requirement 3's command-supervised carrier paragraph. **Preserved untouched, checked line by line:** the
`A LOOP IS ONLY SOUND WHEN` box · the letter/spirit aphorism (a `KEPT:` survivor, [v0.25.0]) ·
requirement 1 in full · requirement 4 in full · the validator trust ranking (which U1-C
**declined** to elevate to floor — a `Contested` element, so touching it here would encode the
option the user rejected) · the tamper-proof clause · the whole *Routing a FAIL by gap type*
section and its corollary · the keystone test and the decoupling-by-absence material · the Red
Flags list · every Common-Rationalizations row but the one logged below.

## [v0.40.0] The single-carrier claim superseded — a command supervisor is its own contract
- **Disposition:** superseded → rewritten in place at four sites, all naming one carrier rule:
  the Overview sentence · the *How to apply* section (retitled "name the carrier, then fill it",
  now a three-branch list) · the Common-Rationalizations row · the Related pointer.
- **Tier failed:** n/a — supersession by ruling (**OQ-2**, adopted verbatim at acceptance **A2**;
  raised as **R17**, which observes that D1-as-amended falsifies F30's constant-at-authoring
  premise for a departing run). The paired shape-side entry is
  `.mochiko/strips/command-shape.md` [v0.40.0] "The per-run-contract prohibition retired".
- **Content (verbatim, all four rewritten sites — an auditor without `git` sees everything that
  left):**
  1. Overview — "the discipline is carried by this skill (the rules) plus a
     [`workflow-contract`](../../templates/workflow-contract.md) that each workflow fills in."
  2. *How to apply: fill in the contract* (the section heading itself is part of what left) —
     "Instantiate [`workflow-contract`](../../templates/workflow-contract.md) for the workflow.
     The filled-in contract is the inspectable proof that all four requirements are met — a
     reviewer can read it and see whether the validator is genuinely independent and where the
     human gate sits. **A workflow without a filled contract has not met this skill's bar.**"
  3. Common Rationalizations, the contract row — "| \"This loop is too simple to need a
     contract\" | Then the contract takes 2 minutes to fill. Simple loops with no contract are
     how unsound loops ship. |"
  4. Related, the contract pointer — "- [`workflow-contract`
     template](../../templates/workflow-contract.md) — the fill-in form this skill governs"
- **The defect this closes, on record since the fact map:** **F21** recorded that
  `command-shape.md`:24–27 already superseded the *How to apply* sentence for commands, and
  **F25** that no command in the six writes a contract — so the skill has been stating a bar its
  largest consumer class provably did not meet, for two shape versions. The revival gives the
  sentence a true reading rather than deleting it.
- **Kept deliberately:** the bar itself — a loop whose requirements are written down nowhere has
  not met it — now reaching the real failure: *"a loop with no filled carrier, or one whose
  composed bounds and gates live only in the lead's context, has not met this skill's bar."* That
  second clause is R18's measured hazard (F88's resume tax) turned into the rule. Also kept: the
  contract as the inspectable proof a reviewer reads for independence and human-gate placement.
- **Consumers assessed** (5 files reference `workflow-contract`; all checked this wave):
  `templates/workflow-contract.md` — revised, entry `.mochiko/strips/workflow-contract.md`
  [v0.40.0] · `templates/command-shape.md` — revised, entry above · `skills/mochiko/SKILL.md` —
  the router carried the stale claim in **two** places ("a command supervisor stitches … to a
  goal **under a workflow-contract**"; the reach-when cell "instantiating the contract for a
  specific workflow") — **corrected in this same wave**, entry `.mochiko/strips/mochiko.md`
  [v0.40.0] · `skills/validation-command-shape/SKILL.md` — its only mention is the new check 22,
  authored this wave · `templates/agent-dispatch.md` — a bare `Pairs with:` pointer that names no
  fill rule, **unaffected and byte-verified unchanged**. One further hit outside that set:
  `templates/slices-template.md:101` lists "filled contracts" among per-slice artifacts — still
  correct, since a departing run's contract lands beside its deliverable.

## [v0.40.0] Requirement 3's cap re-read as the command's *stated default bounds*
- **Disposition:** superseded **in place — guard 1's text stands, its reach does not.** A
  command-supervised paragraph is added below the four guards; the guards themselves, including
  guard 1's "deterministic ceiling counted by the supervisor … not judged by the model", are
  carried across verbatim and still govern every non-command loop.
- **Tier failed:** n/a — supersession by ruling (**D6(a)**, ratified at **A1**: *"Requirement 3's
  carrier: the command's **stated default bounds** plus recorded departures plus U1-D's
  counter/no-silent-re-declaration rules — not command-static-only, and never
  declaration-only"*). U1-D closes the R16/R20 cluster; A3 adds that a declared **cost range** is
  a bound.
- **Content (the reading that was retired, the line itself unchanged):** guard 1 read as
  requiring a *command-static* cap for every loop, which under D1-as-amended would forbid the
  composition U2 grants — and, read the other way, a declaration-only bound would be the
  re-declaration loophole R20 names (raise the bound just before busting it).
- **Kept deliberately, and this is the whole point of the "never declaration-only" clause:** the
  cap is still deterministic, still counted by a supervisor who is not the model deciding it is
  done, still escalating rather than silently dying. What the amendment adds is that a composed
  bound is a bound *in the same sense* — lead-counted, rising only at a user checkpoint,
  re-declared only on the record — and one sentence naming the failure it prevents: "a cap that
  lives only in the lead's head, or one raised quietly just before it would have been busted, is
  the LLM-controlled exit wearing a number." The two Common-Rationalizations rows that price this
  exact excuse ("The model will stop when it's done" · "Separate agent is overkill here") are
  **untouched**, and F23/F24's Red Flags with them.
- **Consumers assessed:** the six commands take the obligated read and each states its own bounds
  — **unchanged and still conformant**, since under U2 their caps *are* stated defaults
  (`.mochiko/strips/command-shape.md` [v0.40.0], P7/P8 entry). `workflow-contract.md` §3 gains the
  matching declared-cost-range and named-counter fields, logged in its own note. No `review-*` or
  `validation-*` skill restates guard 1 — grep for `Hard round cap` returns this skill and the
  contract template only.

## [v0.40.0] KEPT: requirement 2's never-self-grade absolute, under the U1-B addition
- **Tier-2 evidence:** the U1-B clause added at this revision gives the *lead's own pen* a
  non-discretionary cold grade, which is an addition on top of the absolute — never a trade
  against it. Recorded as a survivor because a reader meeting "whether a review runs is the
  lead's call" for the first time could mistake it for a softening: it is not, and the skill now
  says so in the same breath ("Composition reaches which reviews run; it never reaches the
  never-self-grade rule above"). The session's own trail is the evidence — two lead bookkeeping
  overstatements (R25, B8) and one reviewer over-reach (B9), **every one caught by the other
  side's read, none by its author** (record, verify pass round 3).

## [v0.39.0] Requirement 2's unqualified verdict absolute → qualified for the devolved clean branch

- **Disposition:** superseded → rewritten in place, same sentence position in requirement 2
  ("External, independent validation"). The absolute gains a narrow exception clause citing
  `templates/command-shape.md` **Layer 2 — Clearing under the mesh** as the exception's home; the
  conditions that devolve a unit are **not** restated here, and the qualifier is vacuous for any
  consumer whose shape devolves nothing.
- **Tier failed:** n/a — supersession by ruling (team-method **D3**, `DECISIONS.md` 2026-07-25
  row → `.mochiko/brainstorms/team-method-vs-command-shape/record.md`; raised as an escalation by
  `.mochiko/decisions/2026-07-30-layer-2-mesh-rewrite-executed.md` and **closed by**
  `.mochiko/decisions/2026-07-31-team-method-escalations-closed.md`). D3 devolved the
  deterministic-clean unit's advance to the verifying seat at shape v4; read literally, this
  sentence forbade what the shape then required. The line was not verbose — it was made partly
  wrong by a ruling.
- **Content (verbatim):** "The lead/referee owns the verdict."
- **Kept deliberately:**
  - **The whole of requirement 2** — the never-self-grade rule, the different-agent /
    different-skill structure, the artifact-not-say-so clause, the three-rank validator ladder,
    the tamper-proof clause, and the route-to-the-human fallback: untouched, and the exception
    reaches none of them (a devolved unit is still graded by a *different* seat; what devolves is
    the lead's read, never independence).
  - **The lead's done-condition verdict** — restated in the amendment's closing clause precisely
    so the exception cannot be read as reaching requirement 1: the workflow's done-condition
    verdict is the lead's either way. This is the clause that keeps the qualifier narrow.
  - The `A LOOP IS ONLY SOUND WHEN` block, the Red Flags, and the Common Rationalizations table —
    none of which asserts lead-read-every-result; nothing there needed the same qualifier.
- **Consumers assessed:** all **22** referencing files, per the ≥3-consumer all-consumer rule
  (D9 / R4b queue). Commands (6): `brainstorm` · `implement` · `plan` · `setup` · `slice` ·
  `specify` — each takes the obligated read only; `implement` is the sole devolved-branch binder
  and the qualifier is what makes its Cycle-checkpoint line consistent with this skill, the other
  five being unaffected (their non-deterministic check surfaces put them outside the exception).
  Skills (12): `authoring-commands` · `authoring-slices` · `loop-discipline` (self) · `mochiko` ·
  `patterns-technical-decisions` · `review-brainstorm` · `review-feasibility` ·
  `review-governance-intent` · `review-plan-artifacts` · `review-slices` · `review-task-artifacts` ·
  `validation-command-shape`. Templates (4): `agent-dispatch` · `command-shape` ·
  `sized-end-stage-review` · `workflow-contract`. **No consumer restates the superseded sentence**,
  so none inherits a stale copy; the `review-*` "your status is input, the lead owns the clearing
  verdict" lines are each scoped to a **judgment** artifact — the exception is deterministic-CLI-only
  and cannot reach them, which is why none was touched. The per-file disposition table for all 22
  was produced in this pass and returned with the wave handoff.

## [v0.25.0] Sound-loop provenance clause + dead reference pointer (0 lines, ~230 chars)
- **Disposition:** deleted (provenance clause: "Derived from the sound-loop technique cluster (…) plus open question #11"; the axis-mapping sentence kept) · repaired (Related pointer to `agent-skills-research/synthesis/my-framework.md` — **dead since the submodule was removed 2026-07-21** — now points at `ROADMAP.md` alone)
- **Tier failed:** 2 (provenance, not procedure) · dead-pointer repair (D2 reference scope)
- **Content:** the technique-cluster name list; the removed-submodule path
- **Consumers assessed:** 21 consuming files enumerated at wave open — none reference either line; the four requirements, gap-type routing, and keystone test (what consumers actually point at) are untouched

## [v0.25.0] KEPT: the entire remaining body (whole-core survivor ruling)
- **Tier-2 evidence:** contested as a whole at the under-band pass (1.9% by chars, 0 lines — deeply
  under the 30–70 band by ruling) and kept: this is the doctrine home 21 files point at; the four
  requirements each name their failure mode, the gap-routing corollary names the crossed-wires
  defect, the keystone test names the coupling failure, and the scoped aphorism instance is
  consequence-anchored. Cutting the doctrine home trades dead-pointer risk across the library for
  zero altitude gain. Session ruling: batch-3 ratification 2026-07-25.

## [v0.25.0] KEPT: the letter/spirit aphorism set — 11 skill copies, library-wide ruling
- **Tier-2 evidence:** ruled keep-and-survivor-log (R4b item 4, user-approved 2026-07-25).
  Strip-to-reference has no home: this skill's own line ("Violating the letter of THESE rules…")
  is a differently worded, loop-scoped sibling, not a generic canonical home — the queue's
  canonical-home premise was corrected at ruling time. A pointer line costs exactly what the
  motto line costs (net savings ≈ 0), and the motto's mechanism is presence at the point of
  temptation; the 11 copies are ~0.2% of library body mass — the 30–70 bands never need them.
  **Rider (applies per-wave):** a bare, free-floating copy (e.g. `authoring-user-stories:8`,
  `executing-tdd-cycle:8`) gets a one-line named consequence attached in its skill's wave —
  earning Tier-2 status in place, net-zero lines — never stripped; consequence-anchored copies
  (`testing-end-user`, `validation-constitution`) already qualify. Census at ruling: review-slices,
  validation-constitution, brownfield-integration, authoring-slices, authoring-user-stories,
  review-task-artifacts, testing-end-user, review-plan-artifacts,
  authoring-technical-requirements, patterns-vertical-tdd, executing-tdd-cycle.
