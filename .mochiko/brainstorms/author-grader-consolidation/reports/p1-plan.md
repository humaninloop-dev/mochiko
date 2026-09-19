---
report: disclosure
round: 1
seat: P1 (schema producer)
wave: wave 1 — the gate form
unit: plugins/mochiko/migrations/0008-gate-form.yaml + the contract-suite pre-registration
---

## Plan — migration `0008-gate-form.yaml` and its pre-registration

Baseline read from the tree at v0.110.0: `migrate validate --report` 0 rejecting · 104 advisory;
`migrate status` sequences 1..7 · 73 documents · 1043 rules; similarity sweep scanned 1043 ·
scored 150489 · clusters 0 · suppressed 168.

### 1. Header

`grammar: 1` · `id: 0008-gate-form` · `sequence: 8` · `intent:` one line ·
`anchor: 2026-09-19 author-grader-consolidation D7` · `hash:` written by `migrate stamp`.

### 2. Change list, in order

**C1 — `mint-rule` on `command-common/common`.** `id: common.gate-loop-bound`,
`labels: [user-gate, independence]`, `anchor: 2026-09-19 author-grader-consolidation D6`. No
`class:`/`kind:`/`when:`/`enforces:` — the validator rejects any of those on a common block
(`extends-class-local`; `validate.rs` around line 1790). Text, verbatim from the wave plan (the
number lives here and nowhere else):

> A gate verdict of FAIL allows one fix and one re-audit — by the same grader seat resumed,
> reading only what the fix touched and what it could have broken; a second FAIL halts the landing
> and goes to the user with both fix lists, fix again or drop; no run raises this bound. Beyond the
> extending command, `.claude/rules/mochiko/primitive-edits.md` and
> `validation-primitive-edit.gate-loop-bound` cite this id — a rename or tombstone sweeps them.

**C2 — `mint-rule` on `command/setup`, section `setup.sec.boundaries`.**
`id: setup.gate-loop-bound`, `class: floor`, `extends: common.gate-loop-bound`,
`anchor: 2026-09-19 author-grader-consolidation D6`. No labels and no text — both inherit
(`setup.transport-floor` is the shape; `setup.model-tiering` is the precedent for a stub carrying
an anchor). `class:` is local by rule and must be declared.

**C3 — `mint-rule` on `command/setup`, section `setup.sec.roles`.**
`id: setup.validate-seat-form`, `labels: [independence, seats]`, `class: must`, `kind: binding`,
`anchor: 2026-09-19 author-grader-consolidation D7`. Text:

> The validate step's grader is a fresh seat that authored no surface, running
> `mochiko:validation-constitution` with its floors unchanged, spawned with an explicit `model:`
> alias never below the producer's tier; its FAIL→fix→re-grade loop is bounded by
> setup.gate-loop-bound.

**C4 — `mint-rule` on `skill/patterns-model-tiering`, section
`patterns-model-tiering.sec.discipline`.** `id: patterns-model-tiering.persona-less-grader-pin`,
`labels: [boundary]`, `class: floor`, `kind: bound`,
`anchor: 2026-09-19 author-grader-consolidation D7`. Text, reworded off the wave plan's draft to
break the shingle overlap with `patterns-model-tiering.override-is-the-pin` (risk R3):

> A persona-less grader or reviewer spawn carries an explicit `model:` alias, never omitted — the
> tier the graded work was produced at, never below, and `opus` where the lead itself produced it.
> An omitted alias is a floor miss whatever the verdict says: the seat took whatever tier the
> dispatching session happened to run at, which is not a pin.

**C5 — `import-document`, `kind: skill`, `name: validation-primitive-edit`.** Content:
`kind: skill` · `skill: validation-primitive-edit` · `vars: {verdict: PASS}` (required — the
`default-fail` stub inherits text carrying `${verdict}`, and an unbound placeholder is a rejecting
`var-unbound`) · the six review-family sections, every one carrying rules. Ids fixed by the wave
plan; labels all live in the skill registry already, so no `registry-add` is owed.

`sec.independence` — *Independence — who grades, and at what tier*:

1. `.author-grader` — `class: floor`, `extends: review-common.author-grader`.
2. `.plain-seat-explicit-tier` — `[independence, binding]`, `class: floor`, `kind: binding`,
   anchor D7. *"The gate grader is a persona-less fresh seat that authored nothing in the unit it
   grades, spawned with an explicit `model:` alias — the tier the graded work was produced at,
   never below, and `opus` where the lead itself made the edit. A spawn whose alias was omitted has
   missed this floor whatever the verdict says."*
3. `.rendered-contract-only` — `[independence, fence]`, `class: floor`, `kind: binding`, anchor D7.
   *"The gate contract is what `mochiko-cli rules validation-primitive-edit` renders. The
   dispatching brief carries only the unit, the file paths, and the pre-pass command to run; a
   contract section hand-written into a brief instead of rendered is a floor miss, on the same
   terms as an omitted `model:` alias. The grade runs against the render, never a paraphrase of
   it."*
4. `.one-seat-per-wave` — `[independence, binding]`, `class: must`, `kind: binding`, anchor D11.
   *"One grader seat takes every unit of a wave; independence holds because that seat authored none
   of them. Each unit keeps its own verdict block and its own outcome line, tagged with the seat,
   so no verdict hides in the batch. The seat splits in two only when the units' files would not
   fit its context, and the lines say so."*

`sec.scope` — *Scope — the gate job and its one site*:

5. `.gate-job` — `[boundary, verdict]`, `class: must`, `kind: routing`, anchor D2. *"The job is a
   gate — a binary the lead cannot ship past — and it applies at the primitive-edit gate alone: a
   shipped `plugins/mochiko/` primitive, before the `plugin.json` bump that ships it (GI-004).
   Reviews whose findings a user rules downstream are the review family's input job, and setup's
   governance surface set is `mochiko:validation-constitution`'s."*
6. `.unit-keyed` — `[boundary, binding]`, `class: must`, `kind: binding`. *"The unit is keyed by
   kind, and the criteria follow the unit: a command pair, its `.md` with its render · a skill
   pair, its `SKILL.md` with its render · a prose primitive · schema content, which is the
   migration file together with its regenerated view diff."*
7. `.never-excess` — `class: must`, `extends: review-common.never-excess`.

`sec.inputs` — *Inputs — read from the files, and the pre-pass run first-hand*:

8. `.from-file-floor` — `[fence, evidence]`, `class: floor`. *"Inputs are read from the files
   themselves — the edited primitive, its render, the migration and its regenerated view diff, and
   the strip entry where one is owed — never from the author's report."*
9. `.pre-pass-first-hand` — `[evidence]`, `class: floor`, `kind: duty`, anchor D3 (the record's own
   wording). *"The grader runs `mochiko-cli migrate validate --report` and the char-budget
   measurement itself and quotes their output as first-hand evidence; a pre-pass result quoted from
   the brief is not evidence. Nothing that output already asserts is re-derived by judgment."*
10. `.brief-carries-unit` — `[binding]`, `class: must`, `kind: binding`. *"The brief carries the
    unit's kind and name, its file paths, and the pre-pass commands to run — and nothing else of
    the contract."*

`sec.verdict` — *Verdict — binary, evidenced, and bounded*:

11. `.binary-verdict` — `[verdict]`, `class: floor`. Text deliberately unlike
    `validation-constitution`'s one-liner (risk R3): *"The verdict is PASS or FAIL and nothing
    between — a gate the lead cannot ship past, so a hedged or partial clearing is not a verdict at
    all."*
12. `.default-fail` — `class: floor`, `extends: review-common.default-fail`.
13. `.tamper-proof-clause` — `[verdict, evidence]`, `class: floor`, anchor D3. *"A verdict carrying
    no evidence-read line that names the run's own files is FAIL, automatically: the unit is not
    done until the grader has read it."*
14. `.judgment-items-pair` — `[verdict]`, `class: must`, `kind: binding`, anchor D3. *"For a command
    pair or a skill pair the grader confirms each judgment item once, with one evidence line:
    preserved responsibilities · floor survival · independence, no seat grading its own row ·
    reserved-to-user content in the reserved section · the matching done-condition branch · an
    argued overage where the pre-pass shows one. The mechanical items — scaffold headings and
    order, the section-set enumeration, `kind: fail` against its fail segment, id continuity and
    tombstones, ontology grammar, `extends:` conformance, pointer resolution — belong to the
    pre-pass and are read from its output, never re-derived."*
15. `.judgment-items-schema` — `[verdict]`, `class: must`, `kind: binding`, anchor D3. *"For schema
    content — a migration file with its regenerated view diff — the judgment items are the AM-2
    five: intent stated · anchor present where the exit requires one · ID lifecycle right · floor
    and fail survival · register."*
16. `.judgment-items-prose` — `[verdict]`, `class: must`. *"For a prose primitive the items are
    coherence and preserved responsibilities. An edit to a kitted persona also reads the advisory
    grid the brief cites; the grid informs the grade and never gates it."*
17. `.gate-loop-bound` — `[verdict, user-gate]`, `class: floor`, `kind: bound`, anchor D6. *"FAIL
    allows one fix and one re-audit, taken by the same grader seat resumed and reading only what
    the fix touched and what it could have broken. A second FAIL halts the landing and goes to the
    user. No run raises the bound; its number and its command-side home are
    `common.gate-loop-bound`."*

`sec.output` — *Output — the verdict block and the outcome line*:

18. `.verdict-block` — `[reporting, binding]`, `class: must`, `kind: binding`. *"Each unit lands one
    block: `VALIDATE:` with the unit · `Checklist run:` · `Evidence read:`, whose absence is FAIL ·
    `Pre-pass:` with the command and its quoted output · one PASS or FAIL line per judgment item
    with its evidence · `VERDICT: PASS | FAIL` · `Issues requiring fix:`, each entry naming the
    item, the missing thing, and one concrete fix."*
19. `.outcome-line` — `[reporting, evidence]`, `class: must`, `kind: duty`, anchor D9. *"Every gate
    audit writes one line — `audit:` with the unit, the seat, the tier, the file count, the round
    count, the blocking-finding count, and a `cost:` field where one applies — into the record of
    the landing it belongs to: the wave's `build-log.md` entry, or the `.mochiko/decisions/` record
    when the edit closes an ad-hoc defect. The `cost:` field is present only where the audit ran as
    a launched session, read from that session's own reported total."*
20. `.evidence-floor` — `class: floor`, `extends: review-common.evidence-floor`.

`sec.reserved` — *Reserved — the second FAIL is the user's*:

21. `.second-fail-user` — `[user-gate]`, `class: floor`, `kind: reservation`, anchor D6. *"The
    disposition after a second FAIL is the user's — fix again, or drop the edit. A bump carrying an
    unfixed FAIL is a bump without audits PASS; overruling a grader the user judges wrong rides the
    ledger's waiver path, never a run's own call."*

### 3. Command sequence

```
mochiko-cli --plugin-root plugins/mochiko migrate stamp plugins/mochiko/migrations/0008-gate-form.yaml
mochiko-cli --plugin-root plugins/mochiko migrate validate --report
mochiko-cli --plugin-root plugins/mochiko views emit --out .mochiko/schema-views
mochiko-cli --plugin-root plugins/mochiko rules validation-primitive-edit --section preamble
mochiko-cli --plugin-root plugins/mochiko migrate status
```

Gate between steps 2 and 3: **0 rejecting**, and the similarity block still reading
`clusters: 0`. Step 4 reads the floor pin back. Step 5 yields P3's figures.

### 4. Expected post-0008 figures (P3's re-key; P1 confirms from `migrate status`)

| figure | now | after |
|---|---|---|
| sequences | 1..7 | 1..8 |
| documents | 73 | 74 |
| rules (command + skill) | 1043 | 1067 |
| live command rules | 327 | 329 |
| live skill rules | 716 | 738 |
| skill floors | 240 | 252 |
| declared command floors | 116 | 117 |
| command fail nodes | 36 | 36 |
| similarity scanned | 1043 | 1067 |

`common.gate-loop-bound` is a common block and falls outside the 1043 count (327 + 716 = 1043
today). `scored`, `suppressed_hits` and the `(327, 12_421, 0, 60)` row in `matrix_similar.rs` are
re-baselined by P3 from the run, not predicted here.

### 5. `evals/contract/expected-skills.json` diff

Derived under the README's "When a migration legitimately moves a floor set" ruling.

- **`families.review.members`** — insert `"validation-primitive-edit"` after
  `"review-sufficiency"` and before `"validation-constitution"` (the list is alphabetical).
- **New member row `validation-primitive-edit`** — `"family": "review"`, `"floor_ids"` the eleven
  ids below sorted, `"floor_pin": 11`, `"schema_bytes": 0`, `"common": "schemas/skill-review-common.yaml"`,
  `"common_bytes": 0`, `"baseline_bytes": 0`, `"body_bytes_pre": 0`, and
  `"baseline_source": "post-freeze member, 2026-09-19 author-grader-consolidation D7 — no pre-conversion baseline exists, never measured"`.
- **`patterns-model-tiering`** — `floor_ids` gains
  `"patterns-model-tiering.persona-less-grader-pin"` (sorted position: after `override-is-the-pin`,
  before `seat-default-key`), `floor_pin` 7 → 8. The four byte columns and `baseline_source` stay
  byte-identical.
- Nothing else moves. Every other row is untouched.

### 6. `evals/contract/run.py` diff

- **`EXPECTED["setup"]`** — add `"setup.gate-loop-bound"` to the frozenset.
  `setup.validate-seat-form` is `class: must` and does **not** belong in it. `baseline_bytes`
  (`20_245`) untouched.
- **`PROBE_ARGUMENTS`** — add a `"validation-primitive-edit"` row in the review-family block, after
  `"review-sufficiency"`: argument `plugins/mochiko/skills/mochiko/SKILL.md`, note *"Grades a
  shipped primitive edit at the gate; the argument names the edited primitive's own file. A path
  the sandbox does not carry takes the skill's own missing-input branch, after the read-back."*
  **Required** — `probe_prompt` does `PROBE_ARGUMENTS[name][0]` and `converted-shape` asserts every
  converted primitive has a row (risk R2).
- **The unguarded percentage at `run.py:2985`** — `(delivered - baseline) / baseline` divides by
  the frozen baseline with no guard, so a `baseline_bytes: 0` row raises `ZeroDivisionError` and
  takes the whole delivery case down. Guard it the way the summary printer at line 5141 already
  does, printing `no pre-conversion baseline` in place of a percentage when `baseline` is falsy
  (risk R1). Scope addition — lead's call before I execute.

### 7. `evals/contract/README.md` prose

- **Lines 94–95** — `×30` → `×31` on both the `<skill>-delivery` and `<skill>-absence` rows.
- **Lines 98–103** — "eighty-two cases and a hundred and fifty-one sessions" → "eighty-four cases
  and a hundred and fifty-five sessions"; "thirty skill delivery cases" and "thirty single-session
  skill absence cases" → "thirty-one" each; the sentence "The session count is wave 5's unchanged,
  because wave 6 adds only a host case" gains the v0.111.0 clause — one skill added by the
  2026-09-19 `author-grader-consolidation` ruling, four sessions.
- **Line 266** — commands "116 across six" → "117 across six (… plus `setup.gate-loop-bound` by the
  2026-09-19 author-grader-consolidation ruling)"; skills "240 across thirty" → "252 across
  thirty-one", with the two new clauses: `patterns-model-tiering` 7 → 8, and
  `validation-primitive-edit`'s eleven as a post-freeze member with no pre-conversion baseline.
- **Lines 706–711** — "A hundred and fifty-five metered sessions" → "a hundred and fifty-nine";
  "A hundred and fifty-four of them are cases" → "a hundred and fifty-eight"; "ninety across thirty
  skills" → "ninety-three across thirty-one skills"; "thirty-six single-session absence cases" →
  "thirty-seven".
- **Line 538** (the per-family delivered-at-invoke table) is **not** touched: its byte columns are a
  measurement of a past run, and the new member has none.

### 8. Floor count

**Confirmed: 11**, by construction — `author-grader` · `plain-seat-explicit-tier` ·
`rendered-contract-only` · `from-file-floor` · `pre-pass-first-hand` · `binary-verdict` ·
`default-fail` · `tamper-proof-clause` · `gate-loop-bound` · `evidence-floor` · `second-fail-user`.
Three in `sec.independence`, none in `sec.scope`, two in `sec.inputs`, four in `sec.verdict`, one in
`sec.output`, one in `sec.reserved`. Re-read from the preamble's `pins` line before
`expected-skills.json` is written; the freeze is never taken from my count.

### 9. Risks

- **R1 — `baseline_bytes: 0` crashes the delivery case.** `run.py:2985` divides by it unguarded.
  Needs the one-line guard in §6. The alternatives are worse: `null` raises `TypeError` on the
  same line, and a fabricated non-zero baseline would assert a pre-conversion size that never
  existed, against the README's own "nothing can re-derive them" clause.
- **R2 — no `PROBE_ARGUMENTS` row.** `probe_prompt` raises `KeyError` and `converted-shape` fails
  its "every converted primitive has a pre-registered probe argument" check. The wave plan's P1 row
  names only the `EXPECTED["setup"]` re-key, so this is a scope addition inside a file P1 already
  owns a slice of.
- **R3 — new similar-rule edges, and the allowlist is not in my ownership.** `matrix_similar.rs`
  asserts `clusters.len() == 0`, so any surviving edge fails `cargo test`. Three sources: (a)
  `validation-primitive-edit.evidence-floor` against `review-sufficiency.evidence-floor` — the same
  stub-versus-local shape the allowlist already suppresses six times; (b)
  `validation-primitive-edit.default-fail` against `review-brainstorm.never-default-ready` — the
  same shape, five existing rows; (c) the in-document pair
  `patterns-model-tiering.persona-less-grader-pin` against `override-is-the-pin`, whose drafted tail
  was near-verbatim. (c) is handled by the C4 rewording above. (a) and (b) almost certainly need two
  new rows in `scripts/similar-rules-allowlist.yaml`, a file the wave plan assigns to nobody.
- **R4 — anchor on a common block has no precedent.** No `command-common` or `skill-common` block in
  the corpus carries an `anchor:` today. The grammar permits it (only `class`/`kind`/`when`/
  `enforces` are rejected on a block) and it is what makes D6's number protected content. If
  `migrate validate` rejects it, I fall back to the stub-only anchor on `setup.gate-loop-bound` and
  record the fallback rather than improvising.
- **R5 — `enforces-coverage` advisory grows.** `setup.gate-loop-bound` is a floor no `kind: fail`
  node enforces, so setup's advisory line gains one uncovered id. Advisory, exit 0, and the record
  minted no fail node for it — reported, not fixed.
- **R6 — `cite-foreign` advisory on setup.** The inherited `common.gate-loop-bound` text names
  `validation-primitive-edit.gate-loop-bound`, scanned at the binding stub. Command-side citation
  prefixes exclude skill names, so I expect no finding; any that appears is advisory and gets
  reported, not reworded away — the wave plan pins that consumer-naming sentence.
- **R7 — `${verdict}`.** Confirmed needed: `review-common.default-fail` carries `${verdict}` and an
  unbound placeholder is a rejecting `var-unbound`. `vars: {verdict: PASS}` is in the import.

## Execution — halted at change C1

The migration is written whole, with all five changes, and is **unstamped**. It is held at
`inputs/0008-gate-form.draft.yaml`, **not** in the log: left under
`plugins/mochiko/migrations/`, an unstamped file makes `migrate validate` reject repo-wide and
nothing can be rendered from the state, which would have blocked P2 and P3 mid-run. The log
directory is back to 0 rejecting. Nothing else in the tree was touched: no view re-emit, no
`expected-skills.json`, no `run.py`, no README, no allowlist row, no git.

| command | decisive output |
|---|---|
| `migrate stamp`, whole file | `0008-gate-form.yaml: changes[0]: 'section:' missing or not text` |
| probe: `mint-section` + `mint-rule` on `command-common/common` | `section-set · command-common/common · a common library carries blocks at the top level, never sections` — 1 rejecting |
| probe: `replace-document` on `command-common/common` | `op-inapplicable · a 'command-common' document changes one node at a time — wholesale replacement is reserved for templates and shelf data` — 1 rejecting |
| probe: changes C4 + C5 alone, real allowlist in place | **0 rejecting**, 105 advisory · `74 documents · 1065 rules` · `clusters: 3` |
| probe: preamble pin for the new skill | `pins — class: floor · 11 rules`, the eleven ids exactly as §8 lists them |
| probe: `import-document` a **second** `command-common` document | **0 rejecting**; `setup.gate-loop-bound` renders its inherited text and labels |

**Figures from the C4 + C5 probe.** Documents 73 → 74, rules 1043 → 1065, sequences 1..8,
similarity scanned 1065 · scored 156109 · suppressed 168. With C2 and C3 landed these become 74
documents and 1067 rules, as §4 predicted.

**Advisories against the 104 baseline:** one added — `budget · skill/validation-primitive-edit ·
21 rules · 5357 resolved characters of rule text`. None removed. R5's `enforces-coverage` growth
and R6's `cite-foreign` never appeared, because both ride C2, which is blocked.

**The three clusters the sweep actually reports** (R3 as ruled — rows only for reported edges):
`review-sufficiency.evidence-floor` against `validation-primitive-edit.evidence-floor`, best 0.93 ·
`review-code-minimalism.diff-and-report-both-read` against
`validation-primitive-edit.from-file-floor`, 0.62, which §9 did not predict ·
`review-brainstorm.never-default-ready` against `validation-primitive-edit.default-fail`, 0.61. All
three are the stub-versus-local shape the allowlist already suppresses eleven times. The rows are
drafted and unwritten, because they belong in the same landing as the migration.

## Execution 2 — resumed and landed after the grammar widening

P4's widening installed: `mint-rule` with the `section:` key omitted on a common document appends
a top-level block. All five changes landed in one file, `plugins/mochiko/migrations/0008-gate-form.yaml`,
stamped `sha256:698cb30be991e32cdef656e9e046cc0e7b42553b791524e0794b0561f78a5d91`. The `inputs/`
draft stays as the record of the halt.

| step | decisive output |
|---|---|
| `migrate stamp` | stamped; the log's own layout rewritten in place |
| `migrate validate --report` | `0 rejecting · 105 advisory` |
| `views emit` | `74 documents · .mochiko/schema-views`; a second emit produced no further diff, so views ≡ replay |
| `rules validation-primitive-edit --section preamble` | `pins — class: floor · 11 rules`, the eleven ids matching §8 exactly |
| `migrate status` | `grammar 1 · sequences 1..8 (8 migrations)` · `74 documents · 1067 rules` |
| similarity, before the allowlist rows | `clusters: 3 (CROSS-PAIR 3)` · suppressed 168 |
| similarity, after the allowlist rows | `none — no pair clears the threshold` · `clusters: 0` · suppressed 171 |

**Figures for P3.** Documents 74 · rules 1067 · sequences 1..8 · state hash
`sha256:8950f9b90e1fc2bc3d964097bc36b462aee29d0135b74dbc67110c1cca929ddb` · similarity scanned
1067, scored 156764, clusters 0, suppressed 171. Every §4 prediction held, including the command
and skill rule splits.

**R4 — resolved in favour of keeping it.** The `anchor:` on `common.gate-loop-bound` was accepted:
`migrate validate` returns 0 rejecting and the emitted view carries the anchor on the block. The
corpus's first anchored common block, so D6's number is protected content and leaves only by
recorded ruling. No fallback was needed.

**Advisory delta against the 104 baseline: one added, none removed.** The added line is
`budget · skill/validation-primitive-edit · 21 rules · 5357 resolved characters of rule text`.
R5 landed as predicted but as a changed line rather than a new one — setup's `enforces-coverage`
went from sixteen uncovered floor/gate rules to seventeen, now naming `setup.gate-loop-bound`.
R6 never fired: no `cite-foreign` on setup, as §9 expected.

**Files I changed.** `plugins/mochiko/migrations/0008-gate-form.yaml` (new) ·
`.mochiko/schema-views/skills/validation-primitive-edit.yaml` (new, emitted) ·
`.mochiko/schema-views/commands/setup.yaml`, `common/common.yaml` and
`skills/patterns-model-tiering.yaml` (emitted) · `evals/contract/expected-skills.json` ·
`evals/contract/run.py` · `evals/contract/README.md` · `scripts/similar-rules-allowlist.yaml` ·
this plan file · `inputs/0008-gate-form.draft.yaml`. No git mutations. Nothing under `crates/`,
`plugins/mochiko/skills/`, or the manifests — those are P2's and P3's.

## Execution 3 — A2 fix round, amended in place

Three rule texts reworded inside `0008-gate-form.yaml` — no 0009, since the migration is unshipped
and uncommitted. Re-stamped to
`sha256:80f48ff15495644c32c979f5bda3b6beb08534e46abbf3ee6e7a247c7f45225a`.

1. **`validation-primitive-edit.gate-loop-bound`** — the restated number is gone. The rule now
   opens "The bound and its number are `common.gate-loop-bound`'s — cited here, never restated",
   then keeps every other clause: same grader seat resumed, delta read, a further FAIL halting to
   the user, no run raising it. One home for the number, as D4 rules.
2. **`validation-primitive-edit.judgment-items-pair`** — "scaffold headings and order" moved from
   the mechanical list to the judgment list, which now reads scaffold headings and order ·
   preserved responsibilities · floor survival · independence · reserved-to-user · the matching
   done-condition branch · an argued overage. The mechanical list lost that item and gained
   nothing, so it is now the section-set enumeration, the fail-segment check, id continuity and
   tombstones, ontology grammar, `extends:` conformance and pointer resolution. The binary has no
   scaffold check, so the old placement pointed the grader at an assertion nothing makes.
3. **`setup.validate-seat-form`** — "a fresh seat" became "a plain fresh seat", D7's persona-less
   default made explicit.

| gate | result |
|---|---|
| `migrate validate --report` | `0 rejecting · 105 advisory` — the delta against 104 is unchanged, still the new skill's budget line alone |
| similarity | `clusters: 0 (none)` · scanned 1067 · scored 156764 · suppressed 171 |
| `migrate status` | `74 documents · 1067 rules` · sequences 1..8 · state `sha256:30333fa88897cbc9c8ab52c5f27d541cf616b477465a45593966a9f3355238af` |
| preamble pin | `class: floor · 11 rules`, the same eleven ids — so `expected-skills.json` needs no change and was not touched |
| views | re-emitted, 74 documents; a second emit left the working tree identical, so idempotent |

**Note fix 1 — the allowlist comment.** It called all three edges stub-versus-local. Corrected to
say which is which: the first two rows are CROSS-PAIR + EXTEND-GAP, the stub-versus-local shape,
where the new skill binds a review-common block and the counterparty keeps local text; the third
is a plain CROSS-PAIR, both sides carrying local text and neither binding a block, so it is an
ordinary keep-distinct rather than an extraction candidate. Row order matches the description.

**Note fix 2 — the absolute case total.** The builder declares **89** cases
(`python3 evals/contract/run.py --list`), not the 84 I wrote. My +2 cases / +4 sessions delta was
right; the base I applied it to was stale on HEAD by more than the wave-6 prose admitted — three
host cases and the two wave-4 hook cases had been added since it was written. The line now reads
eighty-nine cases and a hundred and fifty-eight sessions, enumerates seven host cases and the two
hook cases, and names `--list` as the figure it must agree with. Counted back: 7 host + 2 fixture
+ 6 command delivery + 6 command absence + 3 mechanism + 31 skill delivery + 31 skill absence +
preload + 2 hook = 89 cases, and 18 + 6 + 3 + 93 + 31 + 2 + 2 + 3 = 158 sessions. The later
"hundred and fifty-nine metered sessions" line was already correct at 158 cases plus the preflight
probe, so it needed no second correction.

**Note fix 3 — the stale comment in the runner, fix-on-sight.** `evals/contract/run.py:5087`, the
comment explaining why the host cases run first, said the sandbox run costs "a hundred and
fifty-one metered sessions". Corrected to a hundred and fifty-nine, the figure the README's own
metered-sessions line declares — the comment counts metered sessions, so it takes 158 case
sessions plus the preflight authentication probe, not the 158 of the case-enumeration line.
Comment text only: `python3 -m py_compile` clean, and no behavior or assertion changed.

Files touched this round: `plugins/mochiko/migrations/0008-gate-form.yaml` · the four emitted
views · `scripts/similar-rules-allowlist.yaml` · `evals/contract/README.md` ·
`evals/contract/run.py` (one comment line) · this plan file. `expected-skills.json` was not
touched. No git mutations.

## Execution 4 — A2 round 2, D4 fidelity

Two rewordings in `0008-gate-form.yaml`, in place. Re-stamped to
`sha256:f62b192214589baed96d6bf4a537a1dc7c1424fadb789d8f9b37352894954c54`.

1. **`validation-primitive-edit.sec.verdict` intent** — "the one re-audit the loop allows" became
   "the re-audit the loop allows". The section's own summary line no longer fixes the count.
2. **`validation-primitive-edit.gate-loop-bound`** — the closing clause "a further FAIL halts the
   landing and goes to the user" is gone. It fixed N=1 by implication and duplicated
   `second-fail-user`, which carries the halt as its ruled subject. The rule now reads: the bound
   and its number are `common.gate-loop-bound`'s, cited never restated; after a FAIL the fix is
   re-audited by the same grader seat resumed, reading only what the fix touched and what it could
   have broken; no run raises the bound.

| gate | result |
|---|---|
| `migrate validate --report` | `0 rejecting · 105 advisory` — delta against 104 still the new skill's budget line alone |
| similarity | `clusters: 0 (none)` · scanned 1067 · scored 156764 · suppressed 171 |
| `migrate status` | `74 documents · 1067 rules` · sequences 1..8 · state `sha256:f7b3ab2de8d102fbeee1c86f71780058609799007f9a612d081bf1b1a3c90028` |
| preamble pin | `class: floor · 11 rules`, the same eleven ids — `expected-skills.json` untouched again |
| views | re-emitted, 74 documents; a second emit left the tree identical, so idempotent |

**The count sweep, over the whole emitted corpus.** Searched every view for `one re-audit`,
`one fix and`, `a second FAIL`, `second FAIL`, `one round`, `single re-audit`, `re-audit once`.
Four hits, no others anywhere in the 74 documents:

- `common/common.yaml` lines 50–51 — `common.gate-loop-bound` itself. The one home of the number,
  which is the point.
- `skills/validation-primitive-edit.yaml` line 202 — inside `second-fail-user`, the rule whose
  ruled subject the second FAIL is. Carved out by the fix brief, correctly.
- `skills/validation-primitive-edit.yaml` line 194 — the `sec.reserved` **title**, "Reserved — the
  second FAIL is the user's". Borderline and left alone deliberately: it is a section label naming
  the subject of the single rule the section holds, not a second home for the number, and changing
  a reviewed title is wider than the two rewordings the round scoped. Flagged for the lead rather
  than changed unilaterally — if the verdict is that a section title may not imply the count
  either, the fix is one word ("Reserved — a repeat FAIL is the user's") and I will take it.

No command schema, and no other skill, carries a count-fixing phrase.

**Retitle taken, lead-ruled.** `validation-primitive-edit.sec.reserved` title became "Reserved — a
repeat FAIL is the user's". Re-stamped to
`sha256:b9356c807ebdc076c6c37c7af384a3718f4c6630bf1ae2f187f6985f695ae96f`; `0 rejecting · 105
advisory`; `74 documents · 1067 rules` at sequences 1..8, state
`sha256:7b58e8c16be44a8c8b22f64de5ba70bbb50d1db2e3da0c65ca9e56add4988343`; pin `class: floor · 11
rules`; views re-emitted and idempotent; clusters 0. The sweep re-run now returns two hits only —
`common.gate-loop-bound` and `second-fail-user` — so the count lives in its one home and its one
ruled subject, nowhere else in the corpus.

Files this round: the migration · the emitted `validation-primitive-edit` view · this plan file.
No git mutations.

## Execution 5 — fix-on-sight, `case_reminder_spawn` NameError

Pre-existing on `main` since 5d8fc69c (2026-09-15), not from this wave: the full suite passed 88 of
89 and crashed on the last case with `NameError: name 'golden' is not defined`. The name is never
bound in that function; the value meant is the local `marker`, which the line two above already
reports as "the line both arms looked for". One token, nothing else:

```diff
-                  {"shape": "2 sessions", "reminder_line": golden, "arms": findings})
+                  {"shape": "2 sessions", "reminder_line": marker, "arms": findings})
```

`python3 -m py_compile evals/contract/run.py` — clean. The remaining occurrences of "golden" in the
file are all either comments or the module-level `REMINDER_GOLDEN` constant, which is a different
thing: the frozen line the freeze asserts from both ends. Nothing else referenced the unbound name.

`python3 evals/contract/run.py --case reminder-spawn`, Docker sandbox up, release binary fresh:

```
ok    reminder-spawn
        ok    R-SPAWN-UNNAMED — the reminder reached the subagent before its first turn
        ok    R-SPAWN-NAMED — the reminder reached the subagent before its first turn
        rec   the line both arms looked for — mochiko gate: artifacts under declared homes take their shape from `mochiko-cli home <path>`
        evidence: evals/.work/contract-reminder-spawn-17b34d3b

contract suite: 1/1 cases passed, 1 ran, 1 measurement(s) recorded and not asserted
FILTERED — 1 of 89 declared cases. Not a gate run.
```

Both arms assert green and the third line is the recorded measurement the fix restored — it is what
`reminder_line` writes into the verdict, so the crash was in the evidence write, after both
assertions had already passed. The runner's own closing line names this a filtered run and not a
gate run, so gate 6 still needs the full 89.

Files this round: `evals/contract/run.py` (one token) · this plan file. No git mutations.

## Failure narrative

Change C1 cannot be written under grammar 1: no op adds a block to an existing common library.
`mint-rule` needs a live section and a common library carries none; minting a section into one is a
rejecting `section-set`; `replace-document` is reserved to templates, shelves and homes. Both
alternatives were probed, not reasoned about, and both reject. C2 depends on C1, whose `extends:`
would dangle, and C3 cites C2, so three of the five changes block together. `0001-genesis` is the
only migration that ever wrote a common library, so nothing has exercised adding a block since.
Two ways out, both above my seat. **A — widen the grammar:** one op, or `mint-rule` taking an absent
`section:` on a common document; general, but crate production code, which §7 names as a halt.
**B — import a second `command-common` document:** probed green, since `common_prefix_of` returns
`common` for every such document, but it splits what D4 calls the log's one shared command library
and nothing catches one block id minted twice. I recommend A; B reads against D4's own words.

## Notes of note

Two ownership gaps the wave plan did not assign were ruled to me at approval and are carried in the
plan above: the `PROBE_ARGUMENTS` row and the similar-rule allowlist rows.

`freeze_expectations.py` cannot regenerate the freeze on this tree (both its guards fail), so the
new member row is hand-derived under the README's replacement ruling, as that paragraph provides.

The `.outcome-line` rule carries D9's literal line grammar verbatim in its text, per the wording
fix ruled at approval; rule 19 in §2 of this plan still paraphrases it, and the migration file is
the authority.
