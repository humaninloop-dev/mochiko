---
report: disclosure
round: 1
seat: P2 (prose producer)
wave: wave 2 — the plan-QA leg
unit: skills/review-seat-plan/SKILL.md + router + budgets + primitive-edits.md criterion 6 + CHANGELOG + manifests
depends_on: P1's render of `review-seat-plan` (section ids pinned in wave2-plan-qa.md §3.1 item 6)
---

## Scope

The P2 row of `wave2-plan-qa.md` §2, detailed in §3.2: the new `review-seat-plan` SKILL.md, the
router's new Seat-plan QA table, the birth-seed budget rows, the one clause of
`primitive-edits.md` criterion 6 named by PPE D8 item 5′, the 0.112.0 CHANGELOG entry, and the
two manifest version bumps. Nothing else moves. The `validator`-bearing router lines and the
`Operating rules` mount-doctrine line are wave 3 and stay untouched; the migration, the schema
views, the contract-suite files and the crate fixtures belong to P1 and P3.

Deliberately left alone: the router's two-review-families paragraph at lines 28–43 (see Risks),
`references/` files for the new skill (none — the contract is the render), and any `plugin.json`
skill entry (skills are directory-globbed by `"skills": "./skills/"`, so a birth needs no
manifest row beyond the version).

## Write set

Seven repo paths plus this plan file. None outside the P2 row.

1. `plugins/mochiko/skills/review-seat-plan/SKILL.md` — new file, new directory.
2. `plugins/mochiko/skills/mochiko/SKILL.md` — one inserted table block after line 125.
3. `.mochiko/memory/primitive-cost-budgets.md` — two rows plus one disclosure paragraph.
4. `.claude/rules/mochiko/primitive-edits.md` — criterion 6, one clause.
5. `CHANGELOG.md` — one new `[0.112.0]` entry at the head.
6. `plugins/mochiko/.claude-plugin/plugin.json` — `"version"` 0.111.0 → 0.112.0.
7. `.claude-plugin/marketplace.json` — `metadata.version` 0.111.0 → 0.112.0.

## Write set — strips owed

None. The skill is a birth, the router edit is a pure addition, the budget ledger and
`primitive-edits.md` are repo surfaces rather than shipped primitives, and schema content takes
no strip entry by construction (`primitive-edits.md`, "Schema content"). This matches §5's
assertion. If P1's render forces a wording change that removes shipped text, I stop and raise it
rather than writing a strip outside my row.

## Reads

- `wave2-plan-qa.md` whole — my row in §2, the section ids in §3.1 item 6, the deliverables in §3.2, the gate form in §4, the gates in §5.
- `producer-plan-enforcement/record.md` lines 169–425 — D2 (transient plan, verdict grammar), D3 (fresh peer), D4 (two-way delivery), D5 (the seven items verbatim), D6 (approval, bound), D7 (dispatch shape), D8 (items 5′ and 6).
- `plugins/mochiko/skills/validation-primitive-edit/SKILL.md` whole — the mirrored shape: frontmatter keys, the Rules paragraph, the seven `!` lines, the read-back sentence, the Procedure.
- `plugins/mochiko/skills/validation-constitution/SKILL.md` whole — the second sibling; confirms `allowed-tools: Bash(mochiko-cli *)` and the same Rules boilerplate.
- `ls` of both skill directories — no `schema.yaml` ships; only `validation-constitution` carries `references/`. Basis for the no-`references/` claim above.
- Router `skills/mochiko/SKILL.md` whole — lines 122–127 give the exact Primitive-edit gate block and the Entry-point anchor; lines 28–43 the families paragraph; lines 158–163 the wave-3 mount-doctrine line.
- `.claude/rules/mochiko/primitive-edits.md` lines 1–110 and 160–169 — the criterion 6 clause verbatim and its surrounding wrap.
- `.mochiko/memory/primitive-cost-budgets.md` lines 1–96 (rule paragraph, fourth seeding path, `validation-primitive-edit` body row), 266–313 (descriptions table and the above-norm disclosure), 518–535 (canonical snippet).
- `CHANGELOG.md` lines 1–70 — the 0.111.0 entry as the shape; both manifests whole.
- `mochiko-cli home <this path>` and `mochiko-cli template report-envelope --plugin-root plugins/mochiko` — this file's envelope and the brainstorm-session home.

## SKILL.md draft — frontmatter

```
---
name: review-seat-plan
description: This skill MUST be invoked when grading a producing seat's plan — the binary PASS/FAIL a plan-only dispatch takes before the lead approves it and resumes that seat to work (sound-loop leg 1). SHOULD also invoke on 'grade the plan', 'seat plan', 'plan-only dispatch', 'plan QA', or 're-plan'. Seven items are walked — scope fidelity · write set declared · reads named · rung claims present · stops and hand-offs named · no self-clearing step · size bound — 1 through 6 blocking, 7 advisory; a FAIL cites the item and gives the fix. Carried by a fresh peer of the author's persona type, never the author's own context and never the lead; a persona-less producer's plan goes to a fresh generic seat. Defaults to FAIL. Additive to sound-loop leg 2, never a substitute — the artifact the seat then produces is still graded by a non-author seat.
allowed-tools: Bash(mochiko-cli *)
---
```

**Description: 839 characters** of the parsed value, measured with the canonical snippet in
`.mochiko/memory/primitive-cost-budgets.md` against this exact text. Under the 1,536 delivery
cap with 697 to spare. Body as drafted below: 3,534 characters.

## SKILL.md draft — body, opening

```
# Grading a Seat Plan

The plan gate of sound-loop leg 1: one binary grade over one producing seat's plan, standing
between a plan-only dispatch and the lead's approval to resume that seat. The grade is additive
to leg 2, never a substitute — the artifact the seat goes on to produce is still graded by a
non-author seat before the user's gate. You are a fresh peer of the author's persona type, or a
fresh generic seat where the producer carries no mochiko persona, and the rules below, exactly as
`mochiko-cli` renders them, are the whole contract: your dispatch brief carries the seat's plan
verbatim, its assigned scope, and the other seats' declared write sets, and nothing else of the
bar you hold. The floor this grade serves: `mochiko:patterns-sound-loop`.
```

## SKILL.md draft — Rules boilerplate, 1 of 2

Heading line `## Rules — delivered by mochiko-cli`, a blank line, then:

```
Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules review-seat-plan · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · review-seat-plan · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
```

## SKILL.md draft — Rules boilerplate, 2 of 2

```
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.
```

Character-identical to the same paragraph in `validation-primitive-edit/SKILL.md` and
`validation-constitution/SKILL.md`, with the skill name substituted at its three occurrences.

## SKILL.md draft — the seven `!` lines

```
!`mochiko-cli rules review-seat-plan --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules review-seat-plan --section review-seat-plan.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
```

Blank line before and after the block in the file. Order and ids exactly as §3.1 item 6 pins
them: `preamble`, then independence · scope · inputs · verdict · output · reserved.

## SKILL.md draft — read-back sentence

```
Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.
```

Verbatim from both siblings. §3.1 item 6 expects six floors by construction; the sentence names
no number, so P1's confirmed pin needs no edit here.

## SKILL.md draft — Procedure

Heading line `## Procedure`, a blank line, then:

```
Read the plan as the brief carries it — the seat's own verbatim text, never a summary and never
its account of what it means. Then check the tree for every claim the plan makes about what is
already there: what the plan says it read, you read. Walk the seven items in order, confirming
each once against the plan and what you read, blocking on items 1 through 6 and recording item 7
as a finding. Close by emitting the verdict in full and returning it to the lead, who carries the
outcome into the run's disclosure line; the plan itself is never persisted. On a FAIL you hand
back the fix list and stop — you are resumed to grade the revision rather than respawned, and the
approval is the lead's, never yours.
```

## Router edit

Pure insertion, no text removed. Anchor: `plugins/mochiko/skills/mochiko/SKILL.md` line 125 is
the `validation-primitive-edit` row, line 126 is blank, line 127 is the Entry-point heading.
**Old:** nothing between them. **New:** after line 125, a blank line then the four lines below,
leaving line 126's blank and the Entry-point heading in place.

```
### Seat-plan QA (model-invoked — every producing seat's plan-only dispatch, all six commands)
| Skill | Reach when |
|-------|------------|
| `review-seat-plan` | grading a **producing seat's plan** before the lead approves it and resumes that seat to work — the plan-only dispatch of sound-loop leg 1. The plan arrives verbatim in the grader's brief, never a summary and never a persisted file; every claim it makes about the tree is checked against the tree. Seven items: scope fidelity · write set declared and disjoint · reads named · rung claims present (design seats) or the test named first (builders) · stops and hand-offs named · no self-clearing step · size bound (advisory) — 1–6 blocking, and a FAIL cites the item and gives the fix. Carried by a **fresh peer of the author's persona type** — a fresh generic seat for a persona-less producer — never the author's context, never the lead; after a FAIL the same grader seat is resumed to re-grade. The verdict returns as `PLAN GRADE: <seat> · PASS|FAIL · <items failed: fixes>` and the lead carries it into the disclosure line's `plans:` segment. Approval is the lead's, and the grade is additive to sound-loop leg 2, never a substitute |
```

## Budget rows

Both rows are birth seeds on the ledger's fourth path — the **ruled birth seed** paragraph at
lines 20–24, opened last wave — so the measured figure is the budget with **no headroom**, and
the argued-overage path applies from the first edit. Measurement is the canonical python3
snippet at lines 518–535, characters of the parsed value, run by me against the quiesced tree
**after** P1's `0009-plan-qa-leg.yaml` lands and the wave's fix rounds close, then re-confirmed
by the release-gate sweep. Body **3,534** and description **839** are measured against the draft
above; the render is P1's and is written `<R>`, the payload `<P>`, until it exists.

Row appended to the `Skill bodies` table, after `validation-primitive-edit` at line 93:

```
| review-seat-plan | <P> (first budget row at birth, seeded [v0.112.0] ruled birth seed, `producer-plan-enforcement` D8 item 6 — payload: body 3,534 + render <R>, measured with the canonical snippet against the quiesced tree after `0009-plan-qa-leg.yaml` and the wave's fix rounds, log state `sha256:<…>` · <N> documents · <N> rules · floor pin <N>, binary 0.2.0; to be re-confirmed by the release-gate sweep; the argued-overage path applies from here) | <P> (no headroom) |
```

## Budget rows — description and disclosure

Row appended to the `Skill descriptions` table, after `validation-primitive-edit` at line 300:

```
| review-seat-plan | 839 | 839 (no headroom) |
```

839 sits above the ~500 family norm, so it is disclosed rather than trimmed, in a short
paragraph beside the existing `validation-primitive-edit` (730) one at lines 306–313. The
load-bearing clauses: the entry site and the leg-1 gate boundary, the five SHOULD trigger
phrases D4 obliges for description-triggered delivery, the seven-item enumeration that tells a
model whether this is the right grade, the fresh-peer independence and its persona-less
fallback, the default-FAIL posture, and the additive-to-leg-2 boundary. No playbook prose.
Precedent: `validation-primitive-edit` 730, `patterns-system-design` 649.

## `primitive-edits.md` criterion 6 — old

One clause of one criterion, per PPE D8 item 5′. Nothing else in the file moves — not the Check
section, not the pre-pass paragraph, not the unit-keying paragraph, not criterion 7. Lines
160–161 today, the `·` chain continuing unchanged onto line 161 and beyond:

```
  6. **Substance across the pair** — **[judgment]**. Plan approval before any producing seat works ·
     author ≠ grader independence (no self-grading seat row) · decisions reserved to the
```

## `primitive-edits.md` criterion 6 — new

Three lines: the second wraps the new clause, the third is the untouched remainder.

```
  6. **Substance across the pair** — **[judgment]**. A producing seat's plan graded by a fresh peer
     per `mochiko:review-seat-plan` and approved by the lead only on PASS before it works ·
     author ≠ grader independence (no self-grading seat row) · decisions reserved to the
```

Lines 162 onward are byte-identical; the file grows by exactly one line.

## CHANGELOG entry draft — heading and the leg reword

Entry heading line `## [0.112.0] — 2026-09-19`, a blank line, then:

```
**The plan-QA leg** (record `.mochiko/brainstorms/producer-plan-enforcement/record.md` D1–D9,
landing set D8 items 0–4, 5′, 6–10; `DECISIONS.md` 2026-09-03 and 2026-09-19; sequenced by
`author-grader-consolidation` D10 as its wave 2). Sound-loop leg 1 said the producing seat plans
first and the lead approves, which let the lead clear a plan nobody else had read. It now reads:
the seat plans read-only in a plan-only dispatch and stops, a fresh peer of its persona type
grades the plan, and the lead approves only a passed plan and resumes the seat to work. The plan
grade is **additive** to leg 2, never a substitute — the artifact the seat then produces is
still graded by a non-author seat before the user's gate.
```

## CHANGELOG entry draft — the new skill

```
New skill **`review-seat-plan`** (D4) — the plan-QA criteria as rules on the review six-set.
Seven items (D5): scope fidelity · write set declared and disjoint · reads named · rung claims
present for design seats, or the test named first for builders · stops and hand-offs named · no
self-clearing step · size bound; 1–6 blocking, 7 advisory, and a FAIL cites the item and gives
the fix. The plan is transient (D2) — it reaches the grader verbatim in the brief and is never
persisted; the verdict returns as `PLAN GRADE: <seat> · PASS|FAIL · <items failed: fixes>` and
the lead carries it into the disclosure line's new `plans:` segment. Delivery runs two ways
(D4): the description triggers the skill, and every grader brief names it besides, because the
grader's persona varies and a persona's preloaded `skills:` cannot carry it. After a FAIL the
same grader seat is resumed to re-grade; the re-plan bound and the escalation stay the command's.
```

## CHANGELOG entry draft — migration, rules, budget

```
Migration **`0009-plan-qa-leg.yaml`** (sequence 9) rewords `patterns-sound-loop.leg-1-seat-produces`
(floor; id and pin unchanged, the verbatim prior text and the ruling anchor carried in the log as
the supersession record), the disclosure-line grammar, `common.plan-approval-producers` and its
three stubs, the command-local restatements on `architecture` · `feature` · `implement`, and
`patterns-plan-minimalism.grading-routing`; it mints the `review-seat-plan` document.
`.claude/rules/mochiko/primitive-edits.md` criterion 6 moves to the peer-graded standard (D8 item
5′), so gate audits never grade against the retired rule. The router gains a Seat-plan QA table.
Budget: birth-seed rows for the new skill on the ledger's fourth path — payload <P> (body 3,534 +
render <R>), no headroom; description 839, disclosed above the ~500 family norm.
```

## CHANGELOG entry draft — the wave's own run

```
This wave ran **under the leg it ships** (D9): every producer seat was spawned once with a
plan-only brief, the tree snapshotted before and diffed after, a fresh peer graded each plan
default-FAIL against D5's seven items, and the lead approved only a PASS before resuming the
seat. Audited under the new gate form (`author-grader-consolidation` D3/D6/D7/D11) — one plain
`general-purpose` seat with an explicit `model:` alias over every unit — and the
`review-seat-plan` pair double-graded by a fresh `mochiko:validator` under the old form, both
finding sets and both first-turn cache-creation figures recorded (D9/I2). Gates: <filled at the
bump from §5 — migrate validate, views ≡ replay, cargo test, similarity sweep, contract suite
91/91, char-budget pre-assert, CHANGELOG, manifests synced>.
```

## Manifest diffs

Two one-line version bumps, both written at the bump, after every audit passes.

```
plugins/mochiko/.claude-plugin/plugin.json line 3
-  "version": "0.111.0",
+  "version": "0.112.0",

.claude-plugin/marketplace.json line 9
-    "version": "0.111.0"
+    "version": "0.112.0"
```

MINOR is right: one new skill, floor rewords that keep their ids, no protected exits.

## Stops and hand-offs

- **Waits on P1.** Every `!` line's section id and the floor count the read-back names are P1's
  to land. I write the SKILL.md with the ids §3.1 item 6 pins, but the render does not exist
  until `0009-plan-qa-leg.yaml` is stamped and views are emitted. I do not stamp, validate, or
  emit anything — that is P1's row, and touching it would be a write-set collision.
- **Budget figures wait twice.** `<P>` and `<R>` stay placeholders until the render exists; I
  measure after P1 lands and the fix rounds close, then the release-gate sweep re-confirms
  against the quiesced tree. A figure taken mid-fix-round is a snapshot, not the row.
- **The version bump is not mine to fire.** CHANGELOG and both manifests are drafted here and
  written at the bump, which §5's gates and the user's leg-3 ruling gate.
- **Who reads this.** A fresh generic peer grades this plan default-FAIL against PPE D5's seven
  items; the lead approves only a PASS and resumes me with the plan quoted. One re-plan round on
  the shared counter, a second consumption to the user. Later the wave's single gate grader takes
  my files as five units, one verdict block each: the pair, router, budgets, rules file, release.

## Risks

1. **The render is not mine and the body is written against it.** If P1's rule texts land with
   different section ids or a floor count that contradicts the read-back sentence, the SKILL.md
   is wrong in a way no local read catches. Mitigation: execute after P1's landing and run
   `mochiko-cli rules review-seat-plan --section preamble` myself before declaring done, rather
   than trusting §3.1's pin. If they diverge I raise it instead of editing P1's row.
2. **`allowed-tools: Bash(mochiko-cli *)` versus the no-on-trust rule.** The skill obliges the
   grader to check tree claims against the tree, but the frontmatter grants only the CLI. Both
   siblings ship exactly this line and §3.2 pins it, so I mirror it and flag the tension. If a
   grader raises it, the fix belongs in a ruling, not a quiet frontmatter widening by me.
3. **A `review-*` name on a gate-shaped grade.** The router's families paragraph says
   `validation-*` issues the authoritative grade and `review-*` produces lead-adjudicated input;
   `review-seat-plan` is named by D4, yet the lead cannot approve past its FAIL. My row forbids
   touching that paragraph, so the tension ships visible. Named here so the grader rules on it.

## Execution

Seven files, exactly the declared write set, nothing outside it. Measured on the landed file
with the canonical snippet: body **3,534**, description **839**, seven `!` lines present.

1. `plugins/mochiko/skills/review-seat-plan/SKILL.md` — new, as drafted, no edits to the draft.
2. `plugins/mochiko/skills/mochiko/SKILL.md` — Seat-plan QA table at lines 127–130, between the
   intact Primitive-edit gate table (122–125) and the Entry-point heading (132). One row.
3. `.mochiko/memory/primitive-cost-budgets.md` — body row at line 94 (after
   `validation-primitive-edit`), description row at 302, disclosure paragraph at 319–327.
4. `.claude/rules/mochiko/primitive-edits.md` — criterion 6 only; `+2 −1`, so the file grows by
   exactly one line as planned, and lines 162 onward are byte-identical.
5. `CHANGELOG.md` — `## [0.112.0] — 2026-09-19` at line 8, above the 0.111.0 entry.
6. `plugins/mochiko/.claude-plugin/plugin.json` — line 3, 0.111.0 → 0.112.0.
7. `.claude-plugin/marketplace.json` — line 9, 0.111.0 → 0.112.0.

## Execution — deviations and notes

- **No deviations from the approved plan.** Every file landed as drafted; nothing widened, no
  file touched outside the write set, no git mutation run.
- **Placeholders left, per the lead's sequencing.** The budget body row carries `<P>`, `<R>`,
  `sha256:<…>` and three `<N>` fields; the CHANGELOG budget sentence carries `<P>` and `<R>`;
  the CHANGELOG gates sentence carries its angle-bracket fill-at-bump note. All await P1's
  render and the release-gate sweep. I did not poll for the migration.
- **Floor-pin discrepancy to settle at the read-back.** The wave plan §3.1 item 6 reasons to six
  floors by construction; the resume brief says the pin is expected to be 5. The read-back
  sentence in the SKILL.md names no number, so nothing in my files depends on which is right.
  Flagging it so the `--section preamble` check settles it rather than passing silently.
- **Strips: none owed**, as the plan asserted — birth, pure addition, and two repo surfaces.

## Execution — figures filled

Read-back run: `mochiko-cli rules review-seat-plan --section preamble --plugin-root
plugins/mochiko` → `class: floor · 5 rules`, ids `author-grader` · `fresh-peer-grader` ·
`plan-verbatim` · `default-fail` · `approval-is-the-leads`, plugin 0.112.0, binary 0.2.0.
**Pin is 5**, settling the flag I raised: §3.1 item 6 listed exactly these five and ruled
`evidence-floor` out, so "six by construction" was a miscount in the wave plan, not a divergence.

The render's section set and order match my seven `!` lines exactly: `preamble`, then
independence · scope · inputs · verdict · output · reserved. No SKILL.md edit needed.

Measured figures — body **3,534** · render **9,167** · payload **12,701** · description **839**.
Per block: preamble 1,623 · independence 1,531 · scope 1,034 · inputs 1,080 · verdict 2,309 ·
output 894 · reserved 696. State at measurement: `sha256:0c6b1460…` · 75 documents · 1,082
rules · sequences 1..9 · binary 0.2.0 · plugin 0.112.0.

## Execution — measurement convention and deviations

- **Render convention calibrated, not assumed.** The ledger names no convention for the render
  half of a payload, so I re-measured `validation-primitive-edit` under both candidates against
  the current tree: raw block text with each block's trailing newline gives **11,519**, the exact
  figure in its [v0.111.0] row, and stripping them gives 11,512. `0009` does not touch that
  skill, so the row reproduces and the raw convention is confirmed. Body and description use the
  ledger's canonical snippet unchanged. Per-block breakdown added to the row, as wave 1 did.
- **One wording deviation in the budget row.** My plan drafted "after `0009-plan-qa-leg.yaml` and
  the wave's fix rounds"; the audits have not run yet, so the row reads "immediately after
  `0009-plan-qa-leg.yaml` landed and before the wave's audits". Accuracy, not scope — the
  re-confirmation clause is unchanged and the release-gate sweep still owns the final figure.
- **One hook deny, surfaced not worked around:** my first append used a shell heredoc; the
  artifact-home check refused it and I re-did it with Edit. No second attempt on that path.

## Execution — re-measure after P1's re-stamp

Re-ran the full measurement against the re-stamped tree, not assumed from the prior reading.
**Every figure is unchanged:** body 3,534 · render 9,167 · payload 12,701 · description 839, and
the per-block split is identical to the digit (preamble 1,623 · independence 1,531 · scope 1,034
· inputs 1,080 · verdict 2,309 · output 894 · reserved 696). The preamble still prints
`class: floor · 5 rules` over the same five ids, binary 0.2.0, plugin 0.112.0.

Only the log's state hash moved, from `sha256:0c6b1460…` to `sha256:d5cb369a…`, at unchanged 75
documents · 1,082 rules · sequences 1..9. That is the signature of an anchor repair — anchors
are migration metadata and never reach the rendered rule text — so the stable payload is the
expected result, and doubles as the check that the re-stamp changed no rule content.

One file changed this round: the budget body row's cited state hash. The CHANGELOG budget
sentence carries no hash and needed no edit; the release-gate sweep still owns the final figure.

## Execution — gate fix, `patterns-sound-loop` overage

Re-measured independently of the grader's figures and they agree: body **2,812** · render
**9,769** · payload **12,581** against the 12,388 budget, a standing **+193** overage. Floor pin
stays 6, body is unchanged, and the description (505) is untouched, since a schema reword never
reaches the frontmatter. Restamped the row as a **[v0.112.0] ruled-HOLDS overage** in the
`patterns-model-tiering` [v0.111.0] shape: prior figures kept as history, the justification
named, budget column unmoved.

Attribution was measured, not asserted. I replayed the log without `0009` into a scratchpad
root — control state `sha256:7b58e8c1…` · 74 documents · 1,067 rules, the same baseline the
`validation-primitive-edit` row cites — and rendered all seven sections against both logs. The
version-triple line is the same length either way (`plugin unknown` and `plugin 0.112.0` are
both seven characters), so no correction was needed.

## Execution — the three components, and what they prove

Five of the seven sections are byte-identical across the control, so the entire delta is the two
floor rewords `0009` names, and the three components sum to the overage exactly:

- `discipline` **+177** — `leg-1-seat-produces`, the plan-QA leg (D8 item 1, anchor 2026-09-03).
- `disclosure` **+253** — `disclosure-line`, the new `plans:` segment (D8 item 4, same anchor).
- the v0.107.0 render-format change **−237**, the constant the other rows already carry.

+177 +253 −237 = +193. The control also explains why the row's recorded render of 9,576 did not
match anything I measured: the pre-`0009` render is 9,339, exactly 237 below it, so the stale
half of this row was the format change, not the new obligation. That is what makes "a genuine
new obligation, never restored prose" a measured claim here rather than an assertion. One file
changed: the `patterns-sound-loop` skill-bodies row.
