---
report: review
pass: cold
pairing: solo
lens: both (solo seat)
skill: mochiko:review-brainstorm
seat: cold-reviewer
reviewed: record.md (frozen 2026-09-19)
angle_map: reports/angle-map.md (56 angles, 8 classes)
raised: 25
survived: 18
critical: 4
important: 9
minor: 5
coverage_findings: 3
status: critical-gaps
---

## Failure narrative

The record is strong: it reconciles the standing rulings, sequences the waves so no wave
grades its own regime, and records its rejected road with reasons. The status is driven by
four repairable items, not by thinness.

C1: wave 1 cannot land as written — D3 shrinks the completeness rule at a gate site whose
grading skill carries the removed obligation as two `class: floor` rules, and the build
surface never touches that skill. C2: D6 opens a ship-with-open-finding path the release gate
does not admit. Two load-bearing claims do not hold as stated: F5's cost statement and D7's
attribution of the measured instantiation floor. Every Critical has a named repair.

## Notes of note

Verified against the tree at v0.110.0, not against the record's own summaries: the rendered
rule sets of `validation-constitution` and `patterns-sound-loop`, the F13 measurement table,
`setup`'s ratification ordering in the derived views, the GI-004 and GI-012 ledger blocks,
and `evals/run.py`'s cost fields.

Two of the record's own flags cleared on check: OQ5's "default FAIL" ambiguity, and D2's gate
classification of setup's validate step — ratification is of the synthesis, before any surface
is authored, so no user ruling sits downstream of that grade.

## Findings — Critical (4)

### C1 — D3's completeness shrink contradicts two `class: floor` rules at the setup gate

**Anchors:** D3 · D2 · Build surface items 1 and 2.

**Failure.** D2 makes setup's validate step a gate site, so D3 binds it. D3 shrinks the
completeness rule to judgment items confirmed once and states that "restart from the checklist"
leaves the grader's brief. The skill that grades at that site carries both obligations as floors:
`validation-constitution.verify-every-item` ("Verify every item — never skip one as 'obvious'")
and `validation-constitution.rationalization-stop`, whose text ends "means STOP and restart from
checklist assembly". `validation-constitution.every-set-must-pass` is a third. The build surface
edits `setup`'s own rule text and `primitive-edits.md`, and never touches
`validation-constitution`. The record's own Constraints section holds that floors leave only by
recorded supersession-by-ruling. As written, wave 1 lands a gate contract that the grading
skill's floors forbid the grader from following.

**Challenges a user ruling** — D3 was ruled "as recommended". The ruling need not change; its
build consequence is unhandled.

**Disposition — repair, at the build surface.** Either (a) scope D3's shrink to the
primitive-edit gate and state that setup's validate step keeps its floors unchanged, or (b) add
`validation-constitution` to build surface item 1 with a recorded supersession-by-ruling for
`verify-every-item` and `rationalization-stop`, plus the strip ceremony. Option (a) is the
smaller move and is consistent with D3's own rationale, which argues entirely from the
primitive-edit audit's mechanical pre-pass — a pre-pass that has no counterpart on the
governance surface set.

Suggested D3 fold, appended to the statement: "This shrink binds the primitive-edit gate only.
Setup's validate step keeps `validation-constitution`'s completeness floors unchanged — the
mechanical pre-pass this shrink relies on has no counterpart on the governance surface set."

### C2 — D6's "ship with the finding recorded" is a bump the release gate does not admit

**Anchors:** D6 · GI-012.

**Failure.** D6 gives the user three options after a second FAIL: fix again, ship with the
finding recorded, or drop the edit. GI-012 states that a `plugin.json` bump MUST NOT land
without audits PASS, alongside strips recorded, landing complete, the CHANGELOG entry and the
marketplace sync. A bump carrying a recorded-but-unfixed FAIL is a bump without audits PASS.
The record never routes that option through the ledger's waiver mechanism. GI-004's testability
is satisfied by it, since an audit trail exists, while GI-012's is not — so the conflict is
invisible to the one principle the record does check.

**Challenges a user ruling** — D6 was ruled "as recommended", and the three-option shape is the
substance of the ruling.

**Disposition — rule (user).** Three paths, in ascending cost: strike the middle option, leaving
fix-again or drop; keep it and route each use through a recorded GI-012 waiver at the ledger; or
keep it and amend GI-012's gate text to admit a user-ruled recorded exception. The second
preserves the user's authority without touching a release gate, and the waiver mechanism already
exists.

### C3 — At the gate D7 contemplates, the grading contract is composed by the author

**Anchors:** D7 · Build surface item 2.

**Failure.** D7 puts the whole gate contract in the spawn brief: the judgment items, the paths to
read, the pre-pass results, the tamper-proof clause, the fix-list form and the loop bound. D7 also
contemplates the case where the lead itself made the edit ("`opus` when the lead itself made the
edit"). In that case the seat composing the grader's entire contract is the author of the graded
work. Under `validator` the criteria lived in a primitive the author could not vary per spawn; a
brief is composed fresh each time. An under-specified brief yields a weaker grade with no signal,
and the grader has no independent source to fall back on. This also runs against the repo's own
delivery doctrine, which moved every rule set out of author-editable files into the migration log
rendered at fire, precisely so a run cannot read a softer source instead.

**Challenges a user ruling** — D7 was ruled "as recommended".

**Disposition — fold into D7.** Bind the brief's source rather than its author. Suggested wording,
appended to D7's statement: "The gate contract is not composed per spawn. It is rendered from the
migration log by `mochiko-cli` and pasted verbatim into the brief; only the unit, the paths and
the pre-pass output are written by the dispatcher. A brief whose contract section was hand-written
rather than rendered is a floor miss, on the same terms as an omitted `model:`."

This costs one migration entry and removes the whole class. It also makes C3, I3 and I6 a single
repair.

### C4 — D9's watch reads the same observation as success and as failure

**Anchors:** D9 · D10.

**Failure.** D9 compares the new form's outcome lines against the F3 baseline, in which roughly
half of waves were zero-blocking and one fix round was the norm. The path predicts the new form
produces the same catches more cheaply, so its success signal is a stable or lower blocking count
at lower cost. A grader weakened by D3's read shrink and D7's persona removal produces exactly
that: fewer blocking findings, fewer rounds. The record cites the seat-default dogfood watch as
its shape but does not carry that watch's own recorded caveat, which says a shallow PASS reads as
fewer findings and fewer rounds, and the cleared artifact is landed, not revertible. D9's fallback
fires only "if the lines disagree with the reasoning" — agreement is the ambiguous outcome, so the
fallback cannot fire on the failure it exists to catch. D10 compounds this: wave 1 runs the old
form and wave 2 the new, on different units, so no unit is ever graded both ways.

**Challenges a user ruling** — D9 and D10 were both ruled "as recommended".

**Disposition — fold into D9 and D10.** Add one differential: at wave 2, one unit is graded by
both forms — the new plain-seat gate form and `validator` on the old form — and the two finding
sets are compared in the D9 line. `validator` has not retired at wave 2, since D10 puts its
retirement at wave 3, so this costs one extra seat, once, and it is the only point in the sequence
where both instruments still exist. It converts D7's cost claim and D9's yield claim into one
observation. Suggested D9 wording: "One wave-2 unit is graded by both forms; its two finding sets
are recorded side by side and are the baseline the later lines are read against."

## Findings — Important (9)

### I1 — D3's judgment-item list omits the schema-content audit criteria

**Anchors:** D3 · Build surface items 2 and 5.

**Failure.** GI-004 as re-expressed at AM-2 keys the schema-content audit unit to the migration
file plus the regenerated view diff, graded on five criteria: intent stated, anchor present where
required, ID lifecycle right, floor and fail survival, register. D3's judgment items are the
markdown-pair set — preserved responsibilities, floor survival, independence, reserved-to-user,
the done-condition branch. Build item 2 rewrites the Check section "to D3/D6/D7/D9", which risks
dropping a set that traces to a ratified amendment. Wave 1's own edits are a migration file, and
build item 5 nonetheless specifies pair-form audits of the `setup` and `patterns-model-tiering`
pairs, with no schema-content audit of the migration itself.

**Disposition — repair.** Name both judgment sets in D3, keyed by unit: the pair set for markdown
primitives, the AM-2 five for schema content. Correct build item 5 to add the migration file plus
its regenerated view diff as its own graded unit.

### I2 — D7's cost rationale attributes a measured floor it was never measured against

**Anchors:** D7 · F5 · Evidence honesty.

**Failure.** The ~54K figure comes from three spawns, all of them personas, and its stated
composition is persona body plus preloaded skills plus their rule renders plus project context.
Project context is paid by any seat. No persona-less spawn was measured, so the avoidable share is
unknown, and D7's brief adds the entire gate contract back as new tokens. The evidence-honesty
section discloses that the figure was carried rather than re-measured, but not that it is being
attributed to a component it was never decomposed into.

**Disposition — repair, with a cheap probe.** One `general-purpose` spawn carrying the gate brief,
reading cache-creation input tokens the same way the three persona spawns were read. Until it
runs, rest D7's rationale on the prior user ruling — a persona whose only craft is checklist
grading is one persona too many — which carries the decision on its own, and mark the cost limb
`Assumed`.

### I3 — The pre-pass result reaches the grader as a report, which the tamper-proof clause forbids

**Anchors:** D3 clause 1 · D7.

**Failure.** D3 keeps the clause that a verdict with no evidence-read line is FAIL, and D7's brief
keeps "read the files, never the author's report". D3 clause 1 then has the CLI's mechanical
result cited in the brief as evidence, which the grader never redoes. A cited tool result is a
report. The grader is asked to trust exactly the channel the clause exists to distrust.

**Disposition — fold into D3 clause 1.** The grader runs the deterministic pre-pass itself. It is
one command with a printed report and seconds of wall time, and its output is then first-hand
evidence. Suggested wording: "The grader runs `mochiko-cli migrate validate --report` itself and
quotes its output; a pre-pass result quoted from the brief is not evidence."

### I4 — No wave grades a unit under both forms, so the sequence forecloses its own cheapest check

**Anchors:** D10.

**Failure.** D10's guarantee that no wave grades its own regime is sound, but it achieves it by
giving each form a different unit. The old form retires at the end of wave 1 and the new form
starts at wave 2, so the two instruments never touch the same artifact. That comparison is the one
thing that would settle whether the new form catches what the old one caught, and the sequence is
the only reason it is unavailable.

**Disposition — fold.** The same wave-2 double-grade named in C4's disposition. Recorded here
separately because it is a property of D10's sequencing, and a repair to D9's line alone would not
create it.

### I5 — The re-audit seat's identity is unspecified, and the delta read depends on it

**Anchors:** D6 · D3 clause 3.

**Failure.** D3 clause 3 has the re-audit read only what the fix touched plus anything the fix
could have broken. Judging what a fix could have broken requires the first read. If the re-audit
is a fresh spawn it lacks that context and must re-read the cluster, which reinstates the cost D3
removed; if it is the first grader resumed, the delta read works and independence is unaffected,
because that seat still authored nothing. The record never says which, and the two readings have
opposite costs.

**Disposition — fold into D6.** Suggested wording: "The re-audit is the same grader seat resumed,
never a fresh spawn — the delta read depends on the first read being in context, and independence
is unaffected because the seat authored nothing in either round."

### I6 — The primitive-gate grader runs no skill, which the axis-5 wording this record adopts requires

**Anchors:** D7 · Build surface item 3 · the wave-3 axis-5 reword.

**Failure.** The record adopts the axis-5 reword as already ruled: "a fresh seat that authored
nothing it grades, running a different skill from the author's". At the setup gate D7's plain seat
carries `validation-constitution`, so it runs a skill. At the primitive-edit gate it carries a
brief and no skill at all. It satisfies "authored nothing" but not "running a different skill",
and it inherits none of the grading floors a skill would deliver — the posture rules that
distinguish a skeptical grader from a capable seat reading a list survive only as prose the brief
restates. That also leaves producer and grader maximally alike: same model family, same tier by
D7's own pin, and now no differentiating instrument between them.

**Disposition — rule, at wave 3.** Either the axis-5 reword gains a clause admitting a rendered
contract in place of a skill, or the gate contract ships as a skill the plain seat mounts. The
second is the same repair as C3 and leaves axis 5 true as written.

### I7 — D9 names a comparison but no trigger, owner, or revert

**Anchors:** D9.

**Failure.** D9 collects five waves of lines and has the comparison "read at the next touch of
this record". There is no condition that fires, no one who owns the reading, and no named revert
if the comparison goes against the path. The watch it cites as its shape carries all three: two
observables, a revert trigger stated in advance, and a one-alias-flip revert. Here the revert is a
governance-adjacent rebuild, so the absence matters more, not less.

**Disposition — fold into D9.** A BACKLOG watch item with the trigger stated in advance and a
named revert. Suggested trigger: blocking findings per wave fall by more than half against the F3
baseline with no independent escape signal, or any defect reaches a release that a full-read audit
would have caught. Suggested revert: the gate form returns to a full read by a recorded ruling,
one migration entry.

### CG1 — The cost claim that motivates D9 is broader than the fact supports

**Anchors:** F5 · D9 · Evidence honesty. *(Coverage — map angle A20.)*

**Diff.** The map expected the record to establish whether cost is measurable before choosing an
instrument to measure it. F5 states the platform exposes no session-readable token totals, and D9
and the evidence-honesty section both rest on that. `evals/run.py` reads `total_cost_usd` off each
session result, sums it across a run, enforces a budget ceiling against the sum, and prints the
estimate. The claim holds for an in-session subagent spawn and does not hold for a session the
harness launches.

**Materiality.** A gate audit run as a launched session would yield the cost figure the record
says nobody has, in dollars rather than in seat counts. D9 chose a proxy without that option in
view, so its instrument choice would plausibly change, and the evidence-honesty line saying no
cost figure exists would be narrowed to the transport it is true of.

**Disposition — rule inline.** Narrow F5 to "no session-readable totals for an in-session spawn;
the SDK returns a per-session cost for a launched session, as `evals/run.py` uses", and add a cost
field to D9's line wherever the audit runs as a launched session.

### CG2 — Batching is never priced, and it is the lever the recorded waves most obviously expose

**Anchors:** D1 · D7 · Build surface. *(Coverage — map angle A15.)*

**Diff.** The map expected an enumerated menu of cheaper independence. D1 names four levers —
which seat, what tier, reading what, at what granularity — and the decisions rule the first three.
No decision rules how many units one seat grades. The recorded waves show one fresh seat per
cluster (13/13, 12/12, 8/8, 4/4) and F5 says each pays the instantiation floor, so a wave of N
clusters pays it N times. One seat grading N units saves N−1 floors, which is larger than anything
D7 claims and orthogonal to every lever the record does rule.

**Materiality.** A decision would plausibly be added, or D1's "granularity" lever would be ruled
rather than left gesturing. The independence question it raises is narrow and answerable: a seat
that authored none of N units is independent of all N, and the only real cost is context bleed
between units, which a per-unit verdict line already contains.

**Disposition — explore now.** It is one question with a small answer space, and it bears directly
on the driver the session was opened on.

## Findings — Minor (5)

### M1 — "Independence stays universal" overclaims against absences the repo has on record

**Anchors:** D1 · F10.

D1's claim is true of F1's five sites and is written as a claim about the library. Five stages
with no adversarial-critique seat are on record from 2026-08-04 — implement's built code,
specify's Intent synthesis, the live brainstorm before convergence, feature-close, and the lead's
own folds — and F10 defers a sixth case. **Disposition — repair:** scope the sentence to F1's
sites and point at the recorded absences, so a later reader does not read the consolidation as a
completeness claim.

### M2 — `author-grader-value-tiering` is claimed by two supersession sets

**Anchors:** Build surface item 6 · `producer-plan-enforcement` D3.

That record is superseded here at wave 1 and also sits in the four supersession rows D3 owes at
its own wave 2, which is this record's wave 3. **Disposition — repair:** note in build item 6 that
the wave-3 list drops it, so the row is written once.

### M3 — The landing set omits this session's own index entry

**Anchors:** Build surface item 6.

Item 6 updates the predecessor's Status line and index entry. The landing ritual also requires
this session's own entry to agree with its record and the decisions index. **Disposition —
repair:** add it to item 6.

### M4 — A repo rule file citing a migration-log rule id can go dead unnoticed

**Anchors:** D4 · OQ1.

`primitive-edits.md` is to cite `common.gate-loop-bound` by id, but nothing renders that rule into
that file, so a rename or tombstone leaves a dead pointer, which the record layer's invariants
call a defect. **Disposition — repair:** state the number in `primitive-edits.md` alongside the id,
or note the citation in the migration so an id change surfaces it.

### CG3 — OQ4 rides a case that may not arise, and inverts the one that does

**Anchors:** OQ4 · D7. *(Coverage — map angle A14.)*

**Diff.** The map expected the tier bound to be tested against the gate sites' actual producers.
OQ4 asks what happens when a `sonnet` seat produced a gate artifact. The gate sites are shipped
plugin primitives and the governance surface set, whose producers are the strong-class seats or
the lead; no `sonnet`-default seat produces either today. **Materiality:** completeness only — the
`Assumed` mark rides a case that may never occur, while the live question is the reverse, whether
a `sonnet` producer should ever be admitted on a gate artifact at all. **Disposition — defer** to
the same touch that reads the D9 lines.

## Raised and killed at cross-examination (7)

- **Setup's validate step is misclassified as a gate.** Killed: the derived views show
  ratification is of the synthesis, before any surface is authored, so no user ruling sits
  downstream of that grade. D2's criterion is applied correctly.
- **`validation-constitution` carries `verdict-is-input`, which D2 would contradict.** Killed: the
  render shows it does not; its verdict section is binary by floor and always was.
- **Sampling — grade k of n — is never considered.** Killed: foreclosed by D1, which keeps a
  grader at every site, and the record records the tiering road as rejected.
- **Independence and adversariality are conflated.** Killed: D2's job split does the work the
  distinction would have done, and F13 inventories the failure modes separately.
- **No escape or leak measure exists to give the yield a denominator.** Killed as a standalone: it
  is the same repair as C4, and reporting it twice would inflate the tally.
- **The seat-default key's thin independence limb is unexamined.** Killed under the new-angle bar:
  the user ruled that limb as is on 2026-09-19, and this record adds vocabulary, not a new argument
  against it.
- **Correlated failure across same-model seats is unnamed.** Killed as a standalone and folded into
  I6, where it bears on a decision rather than standing as commentary.

## Verdict

**Recommended status: `critical-gaps`.** This is input to the lead, never a clearing. The user
rules every survivor, and the four findings marked **challenges a user ruling** — C1, C2, C3, C4 —
are reserved entirely to the user.

Grounds: C1 blocks wave 1 as specified; C2 conflicts with a release-gate obligation; and two
load-bearing claims, F5's cost statement and D7's attribution of the instantiation floor, do not
hold as written.

Record fitness is otherwise high. Every decision carries a statement, a rationale, a confidence
mark and a user ruling. Adoption streaks are tracked and broken with recommendation-free forks at
Q5 and Q9. The rejected roads are recorded with reasons, and the evidence-honesty section is
candid about three of its four weak points. D1, D2, D5 and D8 survived the hunt with no finding
against them.

Tally: 25 raised, 18 survived — 4 Critical, 9 Important, 5 Minor, of which 3 are coverage findings
from the 56-angle blind map.

## Verify round 1

Bounded pass over the folded record: fold fidelity, record fitness, and D11's reopen-born check.
No fresh cold read, no new hunt; surface below is limited to fold-introduced contradictions and
the fitness items the round was asked for.

**Result: NOT CLEAN — 1 blocking, 5 nits.** All 18 dispositions landed at the anchors the user
ruled. The one blocking item is a self-contradiction the C4 fold introduced inside D10's own
statement.

### Fold fidelity — 18 of 18 landed

| Fold | Landed at | Carry-through |
|---|---|---|
| C1 (D3 narrowed to the primitive-edit gate) | D3 statement, closing sentence, naming all three `validation-constitution` floors | build item 1 setup bullet ("floors are untouched"); D2's two gate sites intact; D5's loop bound still spans both, correctly — see N1 |
| C2 (middle option struck) | D6 statement, with the ledger waiver path named | build item 2 ("fix-again or drop"); `common.gate-loop-bound` text carries no struck option; GI-012 conflict cleared |
| C3 (contract ships as `validation-primitive-edit`) | D7 statement: family, log-resident rules, two-way `review-seat-plan` delivery, dispatcher writes unit/paths/pre-pass only, hand-written = floor miss | build items 1 (new schema), 2 (render pasted verbatim), 5 (its pair audited) — see N2 |
| C4 (wave-2 double-grade) | D9 Baseline clause and D10 statement, sets side by side | build rider 7; D9 line gains `<seat>` — see B1 |
| I1 (two judgment sets keyed by unit) | D3 clause 2, pair set + AM-2 five, citing ledger GI-004 at v3.0.0 | build items 2 and 5; AM-2 wording verified verbatim against the ledger this round |
| I2 (cost limb unsupported) | D7 rationale rewritten onto the prior user ruling; Confidence split to `Assumed` on the cost limb | evidence honesty gains the F13-composition bullet; build rider 7; D9 names the spawn as the measurement |
| I3 (grader runs the pre-pass) | D3 clause 1, with "a pre-pass result quoted from the brief is not evidence" | build item 2; D7's dispatcher clause consistent (names the command, does not run it) |
| I4 (no unit graded both ways) | closed by C4 at D10 | same clause as C4 |
| I5 (re-audit seat identity) | D6 statement, same seat resumed | build item 2; D11 restates it consistently |
| I6 (grader runs no skill) | closed by C3, stated in D7 | true at both gates: `validation-primitive-edit` and `validation-constitution` |
| I7 (no trigger or revert) | D9 Watch clause, trigger and revert both stated in advance | build item 6 BACKLOG watch item |
| CG1 (cost claim narrowed) | F5 rewritten to separate in-session spawns from launched sessions | D9 `cost:` field; evidence honesty closing bullet |
| CG2 (batching) | new D11 via Q12 in the trail | build items 1 and 2 carry the one-seat-per-wave shape |
| CG3 (OQ4) | OQ4 re-marked `Deferred`, with the premise and the inverted live question stated | routed to the D9-line touch |
| M1 (universality overclaim) | D1 scope parenthetical naming the five seatless stages and F10 | none needed |
| M2 (double-claimed supersession) | build item 6, wave-3 list drops it | none needed |
| M3 (own index entry) | build item 6 | none needed |
| M4 (dead pointer) | build item 1, migration names its consumers | build item 2 cites by id, number in the migration |

No fold left a pre-fold position standing in another decision, the constraints, the build surface,
or the evidence-honesty section.

### Residuals

**B1 — blocking — D10 contradicts itself on `validator`'s last outing.** D10's statement calls
wave 1 the old form's "last outing" and, four sentences later, staffs `validator` on a wave-2 unit
for the double-grade. Both claims sit in one statement. The C4 fold introduced the second without
retiring the first. A builder reading the earlier sentence does not staff the wave-2 seat.
**Repair:** in D10, replace "(`validator`, full read): the old form's last outing" with
"(`validator`, full read): the old form's last full wave — its final outing is the single wave-2
unit below."

**N1 — nit — D3's "keeps two of three" is now gate-specific.** D3 opens "the gate keeps two" of
F13's three mechanisms. After the C1 fold that holds at the primitive-edit gate only; setup's gate
keeps all three, since its completeness floors are untouched. The closing narrowing covers the
shrink but not the opening framing. **Repair:** open D3 with "At the primitive-edit gate, of the
three mechanisms F13 found under one name, the gate keeps two".

**N2 — nit — `validation-primitive-edit`'s section set is unnamed.** Build item 1 names the
`SKILL.md`, the router row and the first-seed budget, but not the family section set the skill-pair
criteria require, nor the floor-count read-back. `validation-constitution` renders on the review
six-set, so the sibling reuses it. Fold-introduced and non-blocking; the primitive-edit ceremony
and the wave-1 audit both catch it. **Repair:** add to build item 1 "review six-set, as
`validation-constitution` uses, with explicit empty markers; floor-count pin and read-back per the
skill-pair criteria."

**N3 — nit — D4's confidence mark is stale.** It reads "Confident on the home; number and scope
pending", and D5 and D6 have since ruled the scope and the number. Pre-existing, not fold-induced.
**Repair:** "Confident on the home (user's own words); number ruled at D6, scope at D5."

**N4 — nit — D11 carries no adoption-streak mark.** Every other decision tracks the streak, and
Q12 followed a run of "as recommended" answers. **Repair:** add the streak mark to D11's confidence
line, consistent with D1–D10.

**N5 — nit — the review's transport is misstated.** The Review & dispositions section records the
cold-review seat as "subagent transport". The team config for this session lists `cold-reviewer`
with `backendType: in-process` and `tmuxPaneId: in-process` — an in-process teammate, not a
subagent. The agent type (`mochiko:devils-advocate`) and the `opus` pin are recorded correctly.
The distinction is one this record's own F7 lineage tracks, since the teammate leg and the subagent
leg carry different evidence. **Repair:** replace "subagent transport" with "in-process teammate
transport".

### Record fitness

Statement, rationale and confidence mark are present and intact on all of D1–D11. Every decision
names its user ruling. The four named consistency pairs check out:

- **D3 scope vs D2's two gate sites** — coherent. The loop bound (D4/D5) spans both gates; the
  completeness shrink (D3) binds one. The two are different mechanisms and the record keeps them
  apart.
- **D6 vs D9's watch** — coherent. The cap bounds rounds at two, and the watch trigger keys on
  blocking-finding counts rather than rounds, so the cap cannot mask the signal.
- **D7's skill vs the build surface and wave 3** — coherent. The skill is created at item 1,
  audited at item 5, pointed at from item 2, and named as the carrier producer-plan D3's wave 2
  waits on; CLAUDE.md's sweep stays at wave 3 as ruled.
- **D11 vs D6 and D9** — coherent. Per-unit verdict blocks and per-unit lines tagged with the
  seat, re-audits resuming that seat, and the context-fit split disclosed in the lines.

One consistency check worth recording as passed: build item 5 staffs wave 1 with two seats, which
D11 would forbid — correctly, because D11 ships in wave 1 and D10 audits wave 1 under the old form.
The record does not apply its own new rule to the wave that lands it.

### D11 reopen-born check

Internal consistency and fitness only, per `brainstorm.reopen-born-verify`; no coverage hunt, no
second reopen. D11 carries a statement, a rationale, a confidence mark, its user ruling, its
reopen-born marker and its verify-round note. Its independence argument is sound on the record's
own terms — independence keys on who authored the unit, not on how many units one reader holds —
and the per-unit line is a real containment for the context-bleed risk it names. It is consistent
with F5's per-spawn cost fact, with D6's same-seat resume, with D9's line grammar, and with the
build surface at items 1 and 2. One fitness nit only, N4 above. No contradiction found.

**Verdict: NOT CLEAN.** One blocking repair (B1) and five nits. None reopens a decision; all six
are wording or build-surface repairs inside rulings the user has already made.

## Delta-check

Bounded to the six verify-round repairs; nothing else in scope. All six landed as stated. Two
carry-through residuals, both nits, both the same stale word at a neighbouring site. **Nothing
blocks the landing.**

| Repair | Landed | Neighbour check |
|---|---|---|
| **B1** — D10 "last full wave — its final outing is the single wave-2 unit below" | yes, verbatim | Coherent with D10's own double-grade clause and with D7's retirement-at-wave-3 line. Build item 5's heading matched to "old form, last full wave". See R1 for the one place the old framing survives. |
| **N1** — D3 opens "At the primitive-edit gate, of the three mechanisms…" | yes | Does not collide with D3's closing C1 sentence, which now restates the same scope; redundancy only. See R2 for the heading. |
| **N2** — `validation-primitive-edit` bullet names the review six-set, empty markers, floor-count pin and read-back | yes, with all six section ids and `validation-constitution` cited as the precedent | Consistent with build item 5, which audits the new pair, and with D7's description of the skill. |
| **N3** — D4 confidence "number ruled at D6, scope at D5" | yes | D4's statement still points forward to Q6–Q7, which are the questions those two decisions answer. No contradiction. |
| **N4** — D11 confidence carries the streak mark | yes: "Adoption streak: 3 — flagged; no further question in the session, so no recommendation-free fork follows" | Streak arithmetic checks out against D9 (1) and D10 (2). The explanation is needed, because the record's own convention put a recommendation-free fork after each prior streak of 3 (Q5, Q9), and D11 has none. |
| **N5** — Review section says in-process teammate transport | yes, with the evidence cited (`backendType: in-process`) | Nothing else in the record names the transport. Agent type and the `opus` pin remain correct. |

### Residuals — 2 nits, 0 blocking

**R1 — nit — build item 5's closing sentence still calls wave 1 the last old-form sample.** The
heading was matched to "last full wave", but the item ends "The D9 line is written for these
audits too, as the baseline's last old-form samples." D9 makes the wave-2 double-grade part of the
baseline, and its `validator` half is an old-form sample that comes after wave 1. This is B1's
stale word surviving at a second site, inside item 5 rather than D10. Operationally harmless —
the instruction to write D9 lines for the wave-1 audits is unaffected. **Fix:** "…as the
baseline's first old-form samples, the wave-2 double-grade carrying the last."

**R2 — nit — D3's heading is still scope-generic.** The statement now opens "At the primitive-edit
gate", but the heading reads "At the gate, 'default FAIL' keeps the posture…". A reader scanning
headings takes the wider scope the C1 fold removed. **Fix:** open the heading "At the
primitive-edit gate, 'default FAIL' keeps…".

**Verdict: NOT CLEAN on a strict reading — 0 blocking, 2 nits.** Both are one-phrase wording fixes
at sites neighbouring a landed repair, and neither reopens a decision or changes what gets built.
The lead may land over them.
