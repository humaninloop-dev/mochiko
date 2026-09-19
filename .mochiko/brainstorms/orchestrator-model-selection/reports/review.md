---
report: review
feature: orchestrator-model-selection
round: 1
lens: decision-quality + record-integrity (solo)
pass: cold
verdict: critical-gaps
verdict_basis: "D2's load-bearing precedence claim is broken by a documented env var the record never names (CLAUDE_CODE_SUBAGENT_MODEL_FORCE, live since v2.1.257), and D4's evidence gate carries no observable for the deviation lane D3 calls the larger cost lever."
strengths: "F13 dry run with sidechain model ids as proof; every decision names its rejected roads; OQ1-OQ6 honest about the open; F2's terminology-drift catch; D2 correctly refuses a PreToolUse gate under GI-019; D5's Haiku-never-a-seat bound; migration number and all ten persona line refs verified accurate."
raised: 22
survived: 15
coverage_gaps: 8
findings:
  - {id: G1, type: Contradiction, sev: Critical, at: "D2 Statement + Rationale; F5; F12",
     gap: "The file-default safety net is voided by CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1, which the record never names; it disables both the frontmatter and the spawn override on both transports, and also voids the shipped cheap/worker rungs.",
     fix: "Add the variable to Ground facts; name a preflight or resolved-model read-back that detects it; state whether mochiko declares the variable unsupported."}
  - {id: G2, type: Missing, sev: Critical, at: "D4 Statement + Revert trigger; D5 Statement; D3 Rationale",
     gap: "Both dogfood observables are producer-keyed, so the deviation lane ships with no observable; a deviated grader's failure mode is the shallow PASS, which moves both observables toward healthy, and its damage is a landed artifact, not a revertible alias.",
     fix: "Extend the watch to deviations (log every deviation and re-grade a sample on the class default), or bound D5 so judgment-of-record seats cannot deviate down until the lane has an observable. Challenges the user ruling at Q5 - disposition reserved."}
  - {id: G3, type: Missing, sev: Important, at: "D2 Statement; F12 second bullet",
     gap: "No minimum Claude Code version is named, and the record's own fact says the precedence D2 relies on is younger than the plugin's install base.",
     fix: "Name a version floor in the ruling, on the transport-floor precedent (agent-teams >= v2.1.224)."}
  - {id: G4, type: Assumption, sev: Important, at: "Constraints carried in, second bullet; F12 last bullet; OQ2",
     gap: "The teammate leg rests on a doc passage whose scope list is project, user, and managed - plugin scope is absent - so the file default is unconfirmed for mochiko personas on the transport the constraint names.",
     fix: "Probe before build: enable agent teams, seat one persona as a teammate with no override, read the sidechain model id - the F13 method on the other transport."}
  - {id: G5, type: Missing, sev: Important, at: "D2 Rationale; D3 table",
     gap: "An organization availableModels allowlist silently substitutes a blocked tier, running the seat on the lead's model, which defeats the file default without any error.",
     fix: "Name the allowlist channel in Ground facts and cover it with the same resolved-model read-back as G1."}
  - {id: G6, type: Assumption, sev: Important, at: "D2 Statement; F12 fourth bullet",
     gap: "D2 pins aliases while its own threat model is silent retiering, and F12 quotes both the fact that aliases float and the mechanism to pin - the record never weighs full model ids.",
     fix: "Rule the alias-versus-full-id question explicitly, with the eval-rebaseline cost (D7 item 4) on the same page."}
  - {id: G7, type: Assumption, sev: Important, at: "D3 Rationale; D1 Statement",
     gap: "The class criterion assumes a grader stands between output and verdict, but three of the four down-tiered seats are graded by a skill that disclaims a clearing verdict.",
     fix: "Re-read the three rows against what their grader actually gates, or state that gap-finding input is the accepted standard for the class."}
  - {id: G8, type: EdgeCase, sev: Important, at: "D5 Statement, upward deviation; F6",
     gap: "The upward Fable lane is opened without carrying F6's own Fable caveats into the bound, against the most prescriptive prompts in the library.",
     fix: "Bound or caveat the upward lane; name the refusal stop reason as a disclosed risk."}
  - {id: G9, type: Missing, sev: Important, at: "D6 Statement; F12 fifth bullet",
     gap: "Effort keeps inheriting silently on teammates - the exact failure D2 rules out for model - and a Sonnet seat at inherited xhigh is not the Sonnet seat a kit arm would measure, so it confounds D4's watch.",
     fix: "Either pin effort with the class default or record the confound in D4's watch terms."}
  - {id: G10, type: Missing, sev: Important, at: "D7 items 4 and 5; F7 last sentence",
     gap: "The runner's arm model is a module constant with a paired assert, not a flag, and neither it nor the falsified ARCHITECTURE.md agent row appears in the build surface.",
     fix: "Add both files to D7; restate F7's mechanism as a code edit."}
  - {id: G11, type: Assumption, sev: Important, at: "D1 Statement; D3 table",
     gap: "The three-class key has a one-member class with no distinct default and two rows assigned by stakes rather than by the stated criterion, so the key does not discriminate.",
     fix: "Cheaper shape: a two-value default column (strong / down) over the ground column D3 already carries, with the stakes rows argued in their own ground cell."}
  - {id: G12, type: Missing, sev: Important, at: "D2 Statement, disclosure duty; D7 item 3",
     gap: "Disclosure records the intended tier only; nothing reads back the tier that actually resolved, so every silent channel above is undetectable in a run.",
     fix: "Add a resolved-model line to each seat's report - the persona body reaches both transports - as the single detection for G1, G3, G4 and G5."}
  - {id: G13, type: Ambiguous, sev: Minor, at: "D7 item 3",
     gap: "Whether the roster-line duty is a per-command edit or rides brief-obligation is deferred to build with a precedent to check but no decision rule.",
     fix: "State the rule (single-source preferred unless the six pairs disagree) so the build has no open fork."}
  - {id: G14, type: Assumption, sev: Minor, at: "D5 Statement; F4",
     gap: "A floor-class prohibition rests on a user ruling the record itself marks as carrying no measurement.",
     fix: "Mark the Haiku bound Assumed on its evidence, or cite the 200K context limit as the load-bearing half."}
  - {id: G15, type: Missing, sev: Minor, at: "D2; D5",
     gap: "Nested spawns are unaddressed: a seat spawning a seat at depth 3 has no class default, no deviation owner, and no disclosure carrier.",
     fix: "One line: nested persona spawns take the class default and disclose in the spawning seat's report."}
coverage_gaps:
  - {id: CG1, sev: Important, angle: "A16 - the lead's own tier",
     diff: "F2 establishes lead tier differs from seat tier today; no decision rules on what the lead runs at, though the lead now holds every tier judgment and writes every brief.",
     materiality: "Plausibly changes D1 and D5 - a cheap lead makes the tier calls.", topic: "Lead-tier floor for orchestrating sessions"}
  - {id: CG2, sev: Important, angle: "A12 - the fact-checker and in-run dispatch roles",
     diff: "D5's preserved seed material named the fact-checker first; the table rules only the ten rostered personas, and this session's own fact-finding ran on a haiku dispatch.",
     materiality: "Plausibly changes D3 - an eleventh row, or an explicit out-of-scope line.", topic: "Tier of non-rostered in-run roles"}
  - {id: CG3, sev: Important, angle: "A17 - grader below producer",
     diff: "D5 lets any seat drop to sonnet and D3 defaults four producers there, so a deviated grader can sit at or below its producer; the record never asks whether tier asymmetry is part of independence.",
     materiality: "Plausibly changes D5's bounds.", topic: "Tier asymmetry as a producer-validator invariant"}
  - {id: CG4, sev: Important, angle: "A40 - rule-render discipline at a lower tier",
     diff: "Every seat must render mochiko-cli rules, read back the floor count, and halt on a malformed block; no fact or decision asks whether Sonnet holds that discipline.",
     materiality: "Plausibly changes D4 - this is the concrete content of the reliability gate being superseded.", topic: "Floor read-back conformance by tier"}
  - {id: CG5, sev: Important, angle: "A67 - consumer installs versus maintainer dogfood",
     diff: "D4's evidence base is kinako and in-repo runs; the plugin ships to third parties whose session tier, env vars, and allowlist differ, and who never see the watch.",
     materiality: "Plausibly changes D4's sufficiency.", topic: "Evidence base for shipped-plugin defaults"}
  - {id: CG6, sev: Minor, angle: "A19 - tier stability across a FAIL loop",
     diff: "Nothing binds a seat's tier across rounds of one loop; round 1 and round 3 verdicts may come from different tiers.",
     materiality: "Would not likely change a ruling; a one-line bound.", topic: "Tier stability within a review loop"}
  - {id: CG7, sev: Minor, angle: "A4, A5 - seat refusal, no-lead and nested spawns",
     diff: "No seat-side escalation path for a tier it judges too low, and no rule for a direct user invoke with no lead.",
     materiality: "Would not likely change a ruling.", topic: "Seat-side tier escalation"}
  - {id: CG8, sev: Minor, angle: "A45 - cost of a failed cheap grade",
     diff: "D3 sizes the saving (instantiation floors plus rounds); nothing sizes the loss side of a bad artifact or extra rounds.",
     materiality: "Would not likely change a ruling; sharpens D4's trigger.", topic: "Net cost model for a down-tiered seat"}
---

## Provenance and method

Cold seat, never in the room. Phase 0 blind angle map written to `reports/angle-map.md` before
record contact and frozen unedited; 74 angles in twelve groups. Record read cold from
`record.md`. Six hunt classes plus scenario stress per decision D1 through D7, then the coverage
diff against the map.

Fact sample-audit against the files: F1's ten persona line references, the `0006` migration
number, `evals/commands/README.md:97`, `evals/run.py:49`, the six rendered tiering floors, and
the graders of all four down-tiered seats - all confirmed. One miss: F7's persona-runner
`--model` flag (G10).

External claims verified live by this seat, per `references/EXTERNAL-CLAIMS.md`. F12's two
resolution orders and the v2.1.251 note are confirmed verbatim against the current pages.
**Source re-read clause: findings G1, G3, G4 and G5 rest on fetched external text. Solo review -
the lead re-reads the cited sources cold before those findings survive into the record.**

## Failure narratives

### G1 - the file default is not the top of the precedence chain

D2's Statement says each persona pins its class default as an alias "which sits above
`CLAUDE_CODE_SUBAGENT_MODEL` and above inheritance in both resolution orders (F12)", and its
Rationale concludes that "With the file default, forgetting costs only the deviation the lead
meant to make, never correctness." Both hold only in the absence of a variable the record never
names.

From `code.claude.com/docs/en/sub-agents`, fetched this review: "To apply one model to every
subagent, set `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` to `1`." · "While `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`
is on, Claude Code ignores the `model` field of every subagent definition, including the built-in
Explore and Plan subagents, and Claude can't pass a model when it starts a subagent." · "If you
set only `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, subagents run on the main conversation's model." ·
"Requires Claude Code v2.1.257 or later." The agent-teams page carries the same clause for the
other transport: "If you set `CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1`, the first two sources don't
apply."

So on an install with that variable set, every mochiko seat runs on one model chosen outside
mochiko, on both transports, with no error and no lead recourse - the spawn override is disabled
too. If that model is `haiku`, every persona is in the state D5 rules "has failed the floor",
and the two rungs the tiering floor already ships are voided with it: `model: haiku` on Explore
and `model: sonnet` on the worker are both ignored by name in the quoted text. This is wider than
the session's own topic - it is a live gap in the shipped floor - but it lands here because D2's
whole safety-net argument is the thing it falsifies. The repo is already past the version where
the variable exists: `evals/commands/agents.py:11` records CLI 2.1.258.

What resolves it: name the variable in Ground facts; decide whether mochiko declares it
unsupported the way PowerShell-only Windows is declared unsupported, or tolerates it with a
disclosed cost; and pair that with G12's resolved-model read-back so a run can tell.

### G2 - the evidence gate does not cover the lane the record calls the main event

D4 watches two observables: "review-round count per producer seat against the <=3 round cap, and
grader first-round FAIL findings on that seat's artifacts". Both are keyed to producers. D3's
own Rationale says "the larger cost lever is the lead's deviation lane on low-stakes runs (D2)",
and D5 opens that lane to the six strong-class seats, graders included.

For the four Sonnet defaults the instrument is sound: a weaker producer yields more grader
findings and more rounds, so the needle moves the right way. For a deviated grader it inverts.
The failure mode there is the one the prior record preserved as seed material and this record
quotes in F4 - "a plausible shallow PASS is the expensive-to-verify failure". A shallow PASS
produces fewer first-round FAIL findings and fewer rounds. Both observables read healthier
exactly as quality drops, and nothing else in the run looks at it.

The revert argument does not carry across either. The user's rationale at Q5 is "this is a simple
change to revert", and for D3's defaults that is exactly right: one alias flip plus a strip. A
deviation is not a file at all. It is a decision inside a run that has already happened, so what
needs reverting is a landed artifact that a weakened grader cleared - the spec that shipped, the
design the feasibility pass blessed. The cost of being wrong is asymmetric between the two things
D4 covers with one gate.

This finding challenges the user's ruling at Q5 and the reasoning given for it. Its disposition
is reserved to the user. The cheapest shape that keeps the ruling: leave D4 as ruled for the
class defaults, and either log every deviation with a sampled re-grade on the class default, or
hold judgment-of-record seats out of the downward lane until the lane has an observable of its
own.

### G4 - the second transport is still assumed

The Constraints section carries the session's own bar: "Whatever is ruled must resolve on both
transports - subagent and agent-team teammate - or it repeats the v0.77.0 explorer failure (ADR
2026-08-19)." F13 reaches the subagent transport only, and the record is honest about it.

The live agent-teams page confirms the mechanism in principle - "**`model`**: Claude Code uses the
definition's `model` in either display mode when your spawn prompt doesn't name one" - but the
section that mechanism belongs to opens by naming its scope: "you can reference a subagent type
from the project, user, or managed subagent scope". Plugin scope is not in that list, and the
restore clause names only "a project's `.claude/agents/` directory or an `--add-dir` directory".
Repo evidence points the other way - the 2026-08-16 ADR states that teammates "drop `skills:`
frontmatter but load the persona" - so the likely reading is a documentation gap rather than a
contradiction. F12's own last bullet already flags the scope question [could-not-verify].

The point is that the record ships D2's file-default mechanism on a leg that is doc-quoted,
scope-ambiguous, and cheap to settle. OQ2 defers the check to the first team-form dogfood run
after the build, and names as the fallback "spawn-time `model` on every teammate spawn as the
mandatory form" - which is the alternative D2 rejected in its own Rationale. Shipping first and
discovering in dogfood is the shape of the v0.77.0 failure this constraint exists to prevent. The
probe is one spawn: enable agent teams, seat one persona as a teammate with no override, read the
sidechain model id, which is exactly F13's method on the other transport.

## Record fitness

Graded per `references/RECORD-FITNESS.md`, evidence cited from the record.

| Item | Grade | Evidence |
|---|---|---|
| Self-contained | pass | F1-F13, D1-D7, OQ1-OQ6, Q1-Q8 reconstruct decision, reason and rejected road without the conversation |
| Decisions attackable | pass | every D carries a Rationale naming its driver; no bare assertions |
| Decision trail present | pass | the Q-trail maps each D to its question and the user's words, including Q3's dry-run condition and its result |
| Confidence marks honest | pass with note | adoptions were explicitly confirmed, and the record flags its own streak ("third consecutive unelaborated adoption; streak flagged"); D4 correctly carries `Assumed` on the revert trigger |
| Rejected roads recorded | pass | D1, D2, D4, D5, D6 each name what lost and why |
| Honest about the open | pass | OQ1-OQ6 present, including the two that undercut the ruling (OQ2, OQ4) |
| Provenance stated | pass with note | producer, dispatch and date given; no fact-checker seat was filled, and F12's load-bearing docs came from a single dispatch whose own last two bullets are [could-not-verify] - the gap this review's live check found (G1) sits in exactly that material |

## Recommended verdict

`critical-gaps`. Two criteria are met: a broken load-bearing claim (G1 falsifies D2's precedence
statement and the Rationale built on it) and, on G2, an uncovered risk on the lane the record
itself identifies as its main cost lever. The status is a statement about two decisions, not about
the record's craft - the fact layer is unusually well-tagged, the dry run is real evidence, and
every ruling names what it rejected.

Nothing here reopens D1's shape, D3's table as a table, D6's deferral, or D7's ritual, all of
which survive the hunt.

## Status

`critical-gaps` · 22 raised, 15 survived (2 Critical, 10 Important, 3 Minor) · 8 coverage gaps ·
cold pass complete, solo, no counterpart. Dispositions belong in `record.md` under the lead's pen;
this seat writes no record edits. G1, G3, G4 and G5 await the lead's cold re-read of the cited
sources before they survive into the record.

---

## Verify round 1

**2026-09-19 · same cold seat · folded record re-read from the file · nothing written to the record.**

Scope: each fold graded on (a) does it answer the finding it cites, (b) does it contradict
anything else in the record, (c) does its decision still reconstruct from Statement plus folds.
User rulings not re-litigated: G1 and G12 no action (`Contested`), G2 accept as is, the batch as
proposed, CG1–CG6 inline, CG7/CG8 deferred. Two new fold claims were checked against the files.

**Result: NOT CLEAN — 5 blocking, 7 nits.** Every finding below is a fold-introduced
contradiction or a fold-introduced claim that does not hold, except where marked pre-existing.
None asks for a ruling to change.

### Blocking

**V1 — three passages assert the resolved-model line the user declined.**
R3 records "**User: no.**" and G12's row reads "the resolved-model line declined after a plain-language
re-put; disclosure stays intent-only". Three places still carry it as live mechanism:
D8 Statement — "the resolved-model line in the seat's report makes the drift visible per run";
Review row G5 — "named in D2 as a channel the resolved-model line detects";
Review row G6 — "the resolved-model line makes drift visible".
D2's own G5 fold has it right ("the user ruled no read-back (G12), so the difference is not
detected in-run"), which is what makes the other three read as contradiction rather than nuance.
*Repair:* strike the resolved-model clause from D8 and from the two Review rows; D8's ruling stands
on the generation-freeze rationale alone.

**V2 — D3's Rationale was not folded with its table and now names a different exception set.**
The table (G11-reworded) marks `staff-engineer` and `principal-architect` as `strong (held:
criterion says down)`. The Rationale still reads "Six seats are the grader, the verifier, **the
build discipline**, or product truth by stakes" — the retired third class — and "The two rows the
lead expected to be contested — `principal-architect` … and `qa-engineer`". Table holds
staff-engineer; Rationale holds qa-engineer.
*Repair:* re-cut the Rationale's two sentences onto the two-value key and the table's actual held
rows.

**V3 — D2's Statement asserts a precedence the record's new facts contradict.**
Statement: the class default "sits above `CLAUDE_CODE_SUBAGENT_MODEL` and above inheritance in
both resolution orders (F12)". F14 (new): under FORCE "the persona file default, the lead's spawn
override, the `model: haiku` reads rung and the `model: sonnet` worker rung are all ignored". F15
(new): an allowlist substitutes. The exceptions are folded in below the Statement; the Statement
itself still reads unconditional.
*Repair:* qualify the Statement clause — "absent `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` and an
`availableModels` substitution (F14, F15)" — and cite F17 beside F12. No detection, no
declaration; this is wording, not a re-opening of G1 or G12.

**V4 — the G3 version floor is ruled but absent from the build surface.**
D2's G3 fold: "The floor is stated on the tiering skill beside the transport floor's `≥ v2.1.224`."
D7 item 2 enumerates migration `0006-seat-class-key.yaml`'s rule set and does not include it.
*Repair:* add the `≥ v2.1.251` version-floor rule to D7 item 2's list.

**V5 — D7 item 3's escape branch names a surface that no longer exists.**
Item 3 (G13 rule): "unless a rendered command pair already carries its own roster grammar, in
which case **that pair's schema is edited too**". `plugins/mochiko/schemas/` is absent from the
tree and no command `.md` references it [file-verified]; CLAUDE.md records the end state as "the
plugin carries **no schema file a run could read instead**", command rules living in
`plugins/mochiko/migrations/`.
*Repair:* re-point the branch at the command's migration section rather than a schema file.

### Nits

**V6 — D4's G9 fold claims both transports for effort inheritance on one transport's evidence.**
Fold: "seats inherit the lead's effort on both transports". F12 quotes only "Teammates inherit the
lead's effort level", and its own last sub-bullet marks thinking inheritance
`[could-not-verify]`. *Repair:* scope to teammates, or tag the subagent leg `[inferred]`.

**V7 — D4's CG4 evidence proves less than the fold claims.**
Fold: "the floor read-back and halt-on-malformed obligations held there". `evals/run.py:20`
[file-verified] reads: "A run whose load gate fails (plugin not loaded, skill never fired, rules
not delivered, wrong model, auth failure) is recorded `invalid` and excluded from every read; the
report counts it." A gate that discards failures evidences the runs that were read, not the rate.
*Repair:* cite the invalid-run count the report keeps, or scope the claim to valid runs.

**V8 — D3's Rationale points the round-count watch at the wrong decision** (pre-existing, not
fold-introduced): "that cost is measurable (round-count watch, D5)" — the watch is D4's.
*Repair:* D5 → D4.

**V9 — CG3 and D3's staff-engineer ground collide on the build pair.**
CG3: "the lead deviates the pair together or not at all." D3's staff-engineer ground holds it
strong because "a Sonnet seat above a Sonnet worker rung collapses the rung". So deviating
`qa-engineer` down forces `staff-engineer` down and collapses the worker rung by rule.
*Repair:* one clause in D5 — a build-pair deviation drops the worker rung for that run.

**V10 — D1 mints a name the build does not use.** D1: "a **seat default key**"; its own Rationale,
D7 item 2 and the migration filename all say "seat class key"; D3 is "The seat class table".
*Repair:* one name, everywhere.

**V11 — open-questions and trail are out of order.** OQ6 sits after OQ7/OQ8; R1 (the batch that
authorized R2–R4's framing) sits after them.
*Repair:* reorder OQ6 before OQ7, R1 before R2.

**V12 — the coverage header overstates its own routes.** Line above the CG table: "CG1–CG6 ruled
inline as folds on Constraints/D3/D4/D5, CG5 riding G1". CG5's row is "no action" and carries no
fold. *Repair:* "CG1–CG4 and CG6 as folds; CG5 no action."

### Fold-by-fold grade

| Fold | Answers its finding | Contradicts nothing | Decision reconstructs |
|---|---|---|---|
| D1 (G11) | yes | yes | yes — naming nit V10 |
| D2 (G3) | yes | yes | **no — V4** |
| D2 (G5) | yes | yes | yes |
| D2 (G1) | yes | yes | yes |
| D2 (G15) | yes | yes | yes |
| D2 Statement | n/a | **no — V3** | yes |
| D3 (G7) | yes | yes | yes |
| D3 (CG2) | yes | yes | yes — "fact-checker seat no longer exists" confirmed [file-verified] |
| D3 table/Rationale | n/a | **no — V2** | **no — V2** |
| D4 (G2) | yes | yes | yes |
| D4 (G9) | yes | yes | yes — V6 |
| D4 (CG4) | yes | yes | yes — V7 |
| D5 (G8) | yes | yes | yes |
| D5 (G14) | yes | yes | yes |
| D5 (CG3) | yes | yes | yes — V9 |
| D5 (CG6) | yes | yes | yes |
| D7 (G13) | yes | **no — V5** | yes |
| D7 (G10 items 4, 5) | yes | yes | yes |
| D8 (G6) | yes | **no — V1** | yes once V1 lands |
| Constraints (CG1) | yes | yes | yes |

### Record fitness, re-graded

All seven items pass, with the blocking repairs outstanding against the first and third.

| Item | Grade | Evidence |
|---|---|---|
| Self-contained | pass after V1–V5 | the folds carry their own reasoning; V1–V3 are the passages a cold reader would trip on |
| Decisions attackable | pass | every fold names what it answers and why |
| Decision trail present | pass | Review table with per-finding dispositions, R1–R4, and superseded proposals kept verbatim in the G1/G2 rows — an untouched decision is visibly untouched |
| Confidence marks honest | pass | G1 and G12 marked `Contested` at the divergence, not softened; D4's `Assumed` on the revert trigger survives; D8 `Confident (user-ruled, batch)` |
| Rejected roads recorded | pass | "Superseded proposal was:" and "Shapes offered recommendation-free were:" preserve what lost |
| Honest about the open | pass | G2's asymmetry recorded in D4 in terms; CG5 no-action stated; OQ2 narrowed to split-pane; OQ7/OQ8 added |
| Provenance stated | pass | Dispatch paragraph names seat, transport, map path, verdict, and the discharge of the source re-read clause |

### Status

**NOT CLEAN** · 5 blocking (V1–V5) · 7 nits (V6–V12) · no user ruling re-opened · every repair is
a wording or build-surface edit inside a ruling that stands. Verify round 1 complete; the record
re-reads CLEAN once V1–V5 land.

---

## Delta-check

**2026-09-19 · same cold seat · repaired spans only · nothing written to the record.**

Twelve repairs read. **Eleven land their finding cleanly. One (V9) lands but introduces a new
contradiction. One (V10) is partial.**

**Result: NOT CLEAN — 1 blocking, 2 nits.**

### Blocking

**D1 — the V9 repair narrows D5, and two passages say D5 was not narrowed.**

The repaired CG3 fold under D5 reads: "Where the producer is `staff-engineer`, the pair stays
`strong` (a Sonnet build seat collapses its worker rung — D3), so **the verifier does not deviate
down in an implement run**." `qa-engineer` is a grader, so that clause removes a strong-class seat
from the downward lane.

Two passages contradict it:

- D4's G2 fold — "The user ruled **accept as is**: … no extra re-grade, **no narrowing of D5**."
- R4 — "**User: accept as is.** → D4 fold; **D5 unchanged**."

And D5's own Statement still reads "The lead may send **any seat, including the six strong-class
seats**, down to `sonnet` for a run", which is now false for two of the six.

The substance matters: narrowing D5 so a grader cannot deviate down is option (ii) at R4, which the
user declined. The repair reaches the same place for two seats on a different rationale, without a
ruling.

*Repair (preferred):* scope the clause to the consequence, not the permission — "a build-pair
deviation drops the worker rung for that run" — which lands V9 and leaves D5 as ruled. *If the
bound is intended:* mark it lead-proposed and open, and fix "no narrowing of D5" and "D5
unchanged" to match.

### Nits

**N1 — D2's Statement miscategorises the allowlist and cites the wrong ruling.**
Statement: "absent `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` (F14) and an `availableModels` substitution
(F15) — both left as **the consumer's own environment** by ruling (G1, G12)." F15's own last
sentence says the allowlist is "Managed-settings only; **not a consumer-local setting**", and the
allowlist was ruled at G5, not G1.
*Repair:* "FORCE left as the consumer's own environment (G1); the allowlist an organization control
left undetected in-run (G5, G12)."

**N2 — V10 is half-landed: the key is renamed, the value vocabulary is not.**
D1 and D7 item 2 and the migration filename now agree on "seat default key" / `0006-seat-default-key.yaml`.
Still carrying the old word: D1's Rationale ("A class key keeps the default predictable"), D2's
Statement ("pins its **class default**", "the down class"), D3's heading ("The seat class table"),
D5's Statement ("the six strong-class seats").
*Repair:* one pass replacing "class default/class table/strong-class" with "default/seat default
table/strong-default", or one sentence in D1 declaring "class" the value vocabulary under the seat
default key.

### Repair-by-repair

| Residual | Lands its finding | New contradiction |
|---|---|---|
| V1 resolved-model line struck from D8 + rows G5/G6 | yes | no |
| V2 D3 Rationale re-cut (4 by criterion, 2 held, Q4 rows distinguished) | yes | no |
| V3 D2 Statement qualified | yes | no — see N1 |
| V4 version floor added to D7 item 2 | yes | no |
| V5 schema branch re-pointed at the migration log | yes | no |
| V6 effort scoped to teammates, subagent leg `[could-not-verify]` | yes | no |
| V7 CG4 evidence scoped to counted runs | yes | no |
| V8 round-count watch → D4 | yes | no |
| V9 build-pair clause in CG3 | yes | **yes — D1** |
| V10 key renamed "seat default key" | partial | no |
| V11 OQ1–OQ8 and R1–R4 in order | yes | no |
| V12 coverage header re-cut | yes | no |

### Status

**NOT CLEAN** · 1 blocking (D1) · 2 nits (N1, N2) · no user ruling re-opened, and D1 exists to keep
one from being re-opened by accident. The record re-reads CLEAN once D1 lands; N1 and N2 are
wording.
