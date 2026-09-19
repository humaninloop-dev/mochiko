# V1 audit — `patterns-model-tiering` skill pair (migration 0007)

**Verdict: FAIL** — 2 required fixes, both in `class: floor` rule text. Every one of the twelve
skill-pair criteria passes; the failures are fidelity to the ruling and internal coherence of the
rule this landing itself reworded.

**Overage ruling: HOLDS** — the +3,163 over the v0.108.0 ruled payload is five minted rules and one
floor extension carrying obligations the 2026-09-19 ruling creates; the render diff shows no
previously stripped prose returned.

**Seat:** V1 (`mochiko:validator`) · authored none of the graded work · default FAIL
**Unit:** `plugins/mochiko/skills/patterns-model-tiering/SKILL.md` + the seven `mochiko-cli`
render blocks · `plugins/mochiko/migrations/0007-seat-default-key.yaml` ·
`.mochiko/schema-views/skills/patterns-model-tiering.yaml` ·
`evals/contract/expected-skills.json` (`patterns-model-tiering` entry) · the router row in
`plugins/mochiko/skills/mochiko/SKILL.md` · `.mochiko/strips/patterns-model-tiering.md` ·
`.mochiko/strips/mochiko.md`
**Held to:** skill-pair criteria 1–12, `.claude/rules/mochiko/primitive-edits.md:178-263`
**Ruling graded against:** `.mochiko/brainstorms/orchestrator-model-selection/record.md` D1–D8
with review folds · `wave1-build.md` §P1 items 1–14 · lead flags A/B/C
**Date:** 2026-09-19 · **Tree:** quiesced worktree `seat-default-key`, binary `mochiko-cli 0.1.0`,
grammar 1, plugin 0.108.0

---

## Deterministic pre-passes (run by V1, not read from the report)

| gate | result |
|---|---|
| `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` | `0 rejecting · 105 advisory`, exit 0; `similar-rule clusters: none`; `pointer resolution: 84 checked` |
| `python3 evals/contract/run.py --host-only` | `exit=0` · `4/4 cases passed` |
| `converted-shape`, floor set | `ok  the pre-registered floor set matches the patterns-model-tiering render (7 ids)` |
| `converted-shape`, floors index | `ok  patterns-model-tiering: the `floors:` line agrees with the section renders` |
| `converted-shape`, enumeration | `ok  skill `patterns-model-tiering`: the `!` lines enumerate every section, in the render's order` |
| `mochiko-cli views emit` to a scratch dir, then `diff -r .mochiko/schema-views <scratch>` | no output — all 50 committed views are byte-equal to a fresh replay |

### Char budget

Measured with the canonical snippet at `.mochiko/memory/primitive-cost-budgets.md:501-519`, against
the quiesced tree, plus `len(stdout)` of each of the seven render blocks.

| | body | render | payload |
|---|---|---|---|
| HEAD (v0.108.0) | 2,873 | 11,823 | 14,696 |
| this tree | 3,112 | 14,747 | **17,859** |

The HEAD pair reproduces the standing ledger row (`primitive-cost-budgets.md:84` — 14,696 =
2,873 + 11,823) exactly, which makes the after figure comparable. Per-section render:
preamble 1,688 · trigger 2,597 · scope 1,248 · discipline 4,452 · inputs 1,162 · disclosure 2,860 ·
reserved 740.

`description:` parsed value **1,208 chars** (HEAD 998), under the 1,536 delivery cap. No measured
`description:` budget row exists for this skill, so the cap is the only bar.

Budget row: 10,852, no headroom. **Overage +7,007 over the row; +3,163 over the v0.108.0 ruled
payload.**

### Overage ruling — HOLDS

The criterion is "a genuine new obligation, never restored playbook prose"
(`primitive-edits.md:41-44`). I ruled it from a word-level diff of the seven render blocks against
a replay of the same log with `0007` removed, not from the producer's accounting.

Every added character is one of:

- `seat-default-key`, new floor — the D1 criterion, the D3 ten-row table, the D8 alias rule, the
  D2 `inherit` bar. No prior rule carried any of it.
- `seat-deviation-lane`, new — D2's lane plus the G15 nested-spawn fold.
- `seat-deviation-bounds`, new floor — D5 plus folds CG3 and CG6 and (partly — see F2) G8.
- `seat-version-floor`, new — folds G3, G1, G5.
- `seat-roster-disclosure`, new — D2's roster duty under G13's single-source branch.
- `override-is-the-pin`, the seat limb appended to the untouched dispatch-rung clause (D2).
- the preamble's pin, `floors:` line and section counts, which the render computes.

Against that, the superseded `rostered-seats-never-retier` text leaves (−207) and `sec.inputs` is
byte-identical. Nothing from the [v0.102.0], [v0.106.0] or [v0.108.0] strip entries reappears —
checked span by span. The one cross-skill mention, the transport floor's `≥ v2.1.224`, is the
comparison fold G3 explicitly directs onto this skill, not a restatement of the sibling.

One duplication survives (F3) and is worth ~55 chars. It does not move the ruling.

---

## Per-criterion verdicts

### 1. Load-first section — PASS

`SKILL.md:30-45` carries `## Rules — delivered by mochiko-cli`, the positive-confirmation halt
clause, the seven `!` lines, and the legend pointer. The block is **byte-identical to HEAD**:
compared by extracting the substring from `## Rules` to end of file in both revisions — equal.
It demands no raw schema Read and names no schema file; `render-ceiling` asserts the same
(`ok no rendered rule still names a shipped schema file`).

### 2. Section enumeration — PASS

The seven `!` lines deliver `preamble` + the patterns-family six-set — `trigger · scope ·
discipline · inputs · disclosure · reserved` — the set minted by census-patterns §B/J-P7 at
v0.102.0. Set-wise match against the preamble's printed `sections` list confirmed by the contract
suite's own assertion. No `<skill>.sec.*` token appears anywhere else in the `.md`.

### 3. Floor-count pin + read-back — PASS

`SKILL.md:47-49` cites both printed artifacts and carries no hard-coded number: "state back the
floor count the preamble's `class: floor` pin prints and the ids its `floors:` line lists; a blank
or partial read-back is a skipped read — halt and surface it." The preamble prints
`- class: floor · 7 rules` and a `floors:` line of seven ids.

### 4. Floor survival — PASS

Seven floors delivered, one out and two in versus HEAD's six. The superseded floor left through
`supersede-rule` with a well-formed op-level anchor — `0007-seat-default-key.yaml:18-27`,
`anchor: 2026-09-19 orchestrator-model-selection D1`, the only exit the log's grammar allows for
protected content (`migrations/README.md`, "The anchor rule"). The migration also carries the
header anchor. The three floor rewords (`override-is-the-pin`, `class-key-session-tier`,
`worker-seat-set-reserved`) keep their ids and their existing anchors; `reword-rule` supersedes
nothing, so no per-op anchor is owed.

### 5. ID continuity — PASS

`rostered-seats-never-retier` is tombstoned exactly once, in the view's `tombstones:` block
(`.mochiko/schema-views/skills/patterns-model-tiering.yaml:276-283`), disposition plus anchor.
A repo sweep for the id returns only that tombstone, the `supersede-rule` op itself, and three
append-only log lines in `0001-genesis.yaml` and `0004-sonnet-worker-rung.yaml` — history by
construction. No live rule text and no live section note names it: the `sec.reserved` note that
did was rewritten by op 11. No section tombstoned. Three rewords kept their ids, proved by the
render diff showing text change under an unchanged `### <id>` line.

### 6. `extends:` conformance — PASS (vacuous)

The patterns family ships no common library — `migrate validate` lists only
`skill-common/skill-authoring-common` and `skill-common/skill-review-common`. This schema declares
no `extends:` stub.

### 7. `description:` — PASS

Stays in frontmatter, never moves to schema, 1,208 chars under the 1,536 cap. The byte-identical
limb of this criterion keys to a *conversion* event; this is a ruled content edit, the same class
as the v0.108.0 worker-rung reword that already passed audit, and it carries its supersession
entry. The D5 sentence is **replaced in place, not appended** — verified by word diff: "Governs
dispatch tier only — rostered seats never change model (model-tiered-seats D5)" becomes "Governs
dispatch tier and the seat default key — … (orchestrator-model-selection D1–D5)", the trailing
"third sibling of patterns-sound-loop and patterns-transport-floor." unchanged.

### 8. Budget — PASS on measurement, overage HOLDS

See above. Payload measured, overage named in the brief, argument ruled genuine.

### 9. Pointer resolution — PASS

One `pointer:` in the schema, `mochiko:patterns-sound-loop` on `class-key-session-tier`, a skill
reference. `migrate validate` reports `pointer resolution: 84 checked against plugins/mochiko`
with zero rejecting findings.

### 10. Deterministic pre-pass — PASS

Cited above, re-run by V1.

### 11. Skill-grammar conformance — PASS

All 21 rules audited from the view. `kind:` values used are `binding`, `duty`, `latitude`,
`bound`, `reservation`, and omitted-default `constraint` — all inside the eight-kind skill set.
No `kind: fail` and no `enforces:` anywhere. No `conditions:` block and no `when:` term, so the
resolution obligation is vacuous. No `moments:`. Labels used — `boundary`, `reporting`, `binding`,
`evidence`, `ladder`, `floor-pointer` — are all live in
`.mochiko/schema-views/labels/skill-labels.yaml`.

### 12. Provenance anchors — PASS

Every one of the 21 rules carries an `anchor:`. The five minted rules carry their per-rule ruling
anchors as planned: `seat-default-key` D3 · `seat-deviation-lane` D2 · `seat-deviation-bounds` D5 ·
`seat-version-floor` D2 · `seat-roster-disclosure` D2. Both minted floors carry theirs. Raising
protection needs no authority; lowering it would, and nothing here lowers any.

---

## Out-of-block checks the brief named

### Migration grammar and the anchor rule — PASS

`grammar: 1` · `id: 0007-seat-default-key` · `sequence: 7`, agreeing with the filename prefix ·
header `anchor: 2026-09-19 orchestrator-model-selection D1`, well-formed ·
`hash: sha256:f07c75a8…`, validated (a stale hash is a rejecting finding; there are none). Eleven
ops, every one naming `schema: skill/patterns-model-tiering`. Sequence 7 is the lead's allocation
per `wave1-build.md`; gaps at 0005/0006 are legal.

### Flag A — PASS, exactly three word-level edits

The `sec.trigger` render diff between the 0007-less replay and this tree shows three opcodes on
`class-key-session-tier` and nothing else:

- `Session` → `Seat`
- `or a strong seat does` → `or another rostered seat does`
- `(model-tiered-seats D5;` → `(orchestrator-model-selection D1/D3;`

The id and `class: floor` survive.

### Flag B — PASS

`seat-version-floor` ships `class: must · kind: bound`, as the lead ruled, leaving the floor set
at seven. The asymmetry against `patterns-transport-floor.version-floor` is deliberate.

### Flag C — PASS

No command section is touched by `0007` — all eleven ops name the skill document. A sweep for
`never change tier` / `never change model` / `rostered seats never` across `plugins/mochiko` and
`.mochiko/schema-views` returns nothing, so no command carries stale roster grammar either.

### Router row — PASS

Word diff against HEAD: **line 69 is the only differing line in the whole file**, and it carries
three semantic spans, matching plan item 13 —

1. the WHEN clause gains "and when spawning a rostered seat";
2. "stay session tier" → "stay seat tier";
3. "rostered seats never change tier (model-tiered-seats D5);" → the seat-default-key clause.

The row's two surviving "session tier" uses both mean spawn inheritance, not the class-key value,
and correctly stay.

### `expected-skills.json` — PASS

Field-scoped diff: `floor_ids` (one id out, two in, sorted) and `floor_pin` 6 → 7. `family`,
`schema_bytes` 7232, `common`, `common_bytes`, `baseline_bytes`, `baseline_source` and
`body_bytes_pre` untouched — the replacement-by-ruling path `evals/contract/README.md:249-255`
requires, not an edit to match a changed render.

### Strip entries — PASS

Both files carry a `[v0.110.0]` supersession-by-ruling entry with all five fields the
`.mochiko/strips/README.md` form requires. Verbatim content verified mechanically against HEAD:

- `strips/patterns-model-tiering.md` — the quoted prior `description:` is a **998-char exact
  match** to HEAD's parsed value; the quoted tagline and the quoted Overview opening sentence are
  exact.
- `strips/mochiko.md` — all three quoted superseded spans are exact substrings of HEAD's line 69.

Both correctly state that the rule changes take no strip because the log carries the prior text by
construction (`strips/README.md`, "Schema content is recorded by the migration log, not here").

---

## Fix list

`0007-seat-default-key.yaml` is untracked and unshipped, so the whole list is an in-place edit of
that file, then `mochiko-cli migrate stamp` · `migrate validate --report` · `views emit` ·
`run.py --host-only`. No new migration, no new strip entry, no `expected-skills.json` change
(the floor set does not move).

### F1 — Important — `class-key-session-tier` still carries the vocabulary collision flag A removed

`0007-seat-default-key.yaml:125` (view line 25) leaves, inside a `class: floor` rule:

> a weak negative that would mislead a producer stays on the strong tier

D1 rules the vocabulary: "'class' names a value of this key — the `strong` class, the `down`
class". Under D3 four rostered seats are `down`, so a sonnet-tier producer now reads this clause
as an obligation to escalate its weak negatives to an opus seat — an obligation the ruling does
not create, and one the rule's own opening clause contradicts. This is precisely the defect the
lead's flag A ruling fixed one clause earlier ("or a strong seat does" → "or another rostered seat
does"); the same collision survives at its second occurrence in the same rule.

Replace, in op 9's `text:`

```
a weak negative that would mislead a producer stays on the strong tier
```

with

```
a weak negative that would mislead a producer stays with the seat
```

### F2 — Important — `seat-deviation-bounds` drops one of G8's three named caveats

Record D5 fold G8 obliges three things of a `fable` deviation's stated reason: the persona was
prompted for Opus; **that thinking is always on with effort the only depth control**; and that a
`refusal` stop reason is handled as a seat failure. The minted floor carries the first and third
and omits the second. `wave1-build.md` §P1 item 4 named only two, but that section opens "the plan
P1 refines; the ruling it executes is fixed", so the record governs the content.

Replace, in op 5's `text:`

```
A `fable` deviation states as disclosed risks that the persona was prompted for Opus and that a
`refusal` stop reason is handled as a seat failure.
```

with

```
A `fable` deviation states as disclosed risks that the persona was prompted for Opus, that
thinking is always on with effort the only depth control, and that a `refusal` stop reason is
handled as a seat failure.
```

### F3 — Minor — the `inherit` bar is stated twice across two floors

`seat-default-key` floors it ("A persona carrying `inherit` or no `model:` has failed this
floor") and `override-is-the-pin` repeats it ("an undisclosed override, or a persona file carrying
`inherit` or no `model:`, has failed it too"). D2's own wording for the override rule is
"an undisclosed override has failed the floor" — the file-default bar is `seat-default-key`'s job.
Each fact once.

Replace, in op 8's `text:`

```
an undisclosed override, or a persona file carrying `inherit` or no `model:`, has failed it too.
```

with

```
an undisclosed override has failed it too.
```

### F4 — Minor — P1's report §1 is stale at 0006

`reports/p1-report.md` §1 is headed "Migration 0006 — 11 ops" and states `id: 0006-seat-default-key`,
`sequence: 6`, `hash: sha256:9c6f31ca…` and "Sequence 6 was allocated by the lead". The shipped file
is `0007-seat-default-key.yaml`, sequence 7, `hash: sha256:f07c75a8…`. The title line and the file
table were updated; §1 was not. The artifact is correct and the report is not evidence, but the
record should not disagree with the tree.

---

## Post-fix expectation

F1 is net −4 chars, F2 +62, F3 −55: payload lands near **17,862**, the overage ruling unchanged.
Re-run the four commands above; the floor set, the `floors:` line, `expected-skills.json` and the
strip entries are all unaffected by every fix.

---

## Confirm pass

**Verdict: PASS.** All four fixes landed as specified, in the render and not only in the source.
Nothing else moved. **The overage ruling stands at HOLDS**, unchanged at the re-measured payload.

**Scope:** bounded re-grade only — the three rule texts as rendered, the report's §1, and a
no-drift check. Migration re-stamped to
`hash: sha256:0287aa5d0681d9ebac8a370c7092556e5c9b4a32142063d2ff3773f3856d4ef3`, `id:
0007-seat-default-key`, `sequence: 7`, header anchor unchanged. Still 11 ops over the same 11
nodes — nothing added, nothing dropped.

### The three rule texts, from the render

| fix | rendered text | verdict |
|---|---|---|
| F1 | `class-key-session-tier`: "a weak negative that would mislead a producer **stays with the seat**" | fixed |
| F2 | `seat-deviation-bounds`: "prompted for Opus, **that thinking is always on with effort the only depth control**, and that a `refusal` stop reason is handled as a seat failure" | fixed |
| F3 | `override-is-the-pin`: "**an undisclosed override has failed it too.**" — the `inherit` limb gone, single-homed in `seat-default-key` | fixed |

F2 is the exact G8 wording; the `fable` caveat now names all three items the fold obliges. F1 and
F3 are the exact replacements pinned in the fix list.

**F4.** `reports/p1-report.md` §1 is headed "Migration 0007 — 11 ops" and carries `id:
0007-seat-default-key`, `sequence: 7`, the current hash, and a renumber paragraph plus a §9
hash trail. The stale 0006 heading, id, sequence and hash are gone.

### Nothing else moved

Re-ran the pre-migration replay (the log with `0007` removed) against this tree and word-diffed
all seven render blocks. The opcode set is span-for-span identical to the round-1 diff except the
three fixes. Specifically unchanged: the `sec.scope` supersession and mint, the three other mints,
the `sec.disclosure` mint, both section rewords, the `worker-seat-set-reserved` citation swap, and
`sec.inputs`, still byte-identical to HEAD.

| check | result |
|---|---|
| `mochiko-cli migrate validate --report` | `0 rejecting · 105 advisory`, exit 0 |
| `python3 evals/contract/run.py --host-only` | `exit=0` · `4/4 cases passed` |
| floor set | `ok  the pre-registered floor set matches the patterns-model-tiering render (7 ids)` |
| floors index | `ok  patterns-model-tiering: the `floors:` line agrees with the section renders` |
| `views emit` to scratch, `diff -r` against the committed tree | no output — 50/50 equal |
| preamble pin | `- class: floor · 7 rules`, the same seven ids in the same order |
| `SKILL.md` | body 3,112 · `description:` 1,208 · Rules block still byte-identical to HEAD |
| router row | still the only differing line in its file, same three spans |
| `expected-skills.json` | still `floor_ids` + `floor_pin` only, every byte column untouched |
| both strip files | re-verified verbatim — the 998-char description matches to the character |

### Payload

| | body | render | payload |
|---|---|---|---|
| round 1 | 3,112 | 14,747 | 17,859 |
| post-fix | 3,112 | 14,752 | **17,864** |

Net +5 (F1 −5 · F2 +62 · F3 −52), matching what P1 reports. Overage now +7,012 over the 10,852
row and +3,168 over the v0.108.0 ruled 14,696. Every added character is still a new obligation the
2026-09-19 ruling creates; F2 added one of them back. **HOLDS.**

### Notes, non-blocking

- `class-key-session-tier` now takes **four** word-level edits, not the three flag A named: the
  fourth is F1, a grader-pinned fix inside the same clause. Not a flag A deviation.
- P2's four persona strips now cite `0007-seat-default-key` — verified by sweep, no strip file
  anywhere contains `0006-seat-default-key`. P1's report §6 and the F4 paragraph of §9 still
  describe those four citations as stale, which they no longer are. Cosmetic, in P2's territory
  and V2's grade, recorded here only because the fix landed after round 1.
- `CHANGELOG.md` and `.mochiko/memory/primitive-cost-budgets.md` moved between round 1 and this
  pass. Both are the lead's under `wave1-build.md`, outside V1's unit, and ungraded here.
