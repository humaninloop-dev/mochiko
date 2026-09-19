# P1 build report — migration 0007, the tiering skill pair, the suite re-key

**Seat:** P1 (`mochiko:staff-engineer`) · **Wave:** `wave1-build.md` P1 items 1–14 · **Date:** 2026-09-19
**Ruling executed:** `.mochiko/brainstorms/orchestrator-model-selection/record.md` D1–D5, D7 item 2, D8 as review-amended (`DECISIONS.md` 2026-09-19)
**Worktree:** `seat-default-key` (branch `worktree-seat-default-key`, off `main` at v0.108.0) · **Target version stamp:** v0.110.0
**Status:** complete, V1 fix round applied · every planned item landed, nothing deferred · not self-graded — V1 confirms this pair
**V1 round 1:** FAIL on two fidelity defects in floor text, overage ruled HOLDS, all twelve criteria pass. Fixes F1–F4 applied in place on the untracked migration (no new migration, no new strip, no `expected-skills.json` change) — see §9.
**Plan authority:** the lead's approval message of 2026-09-19, which ruled flags A (make the third `class-key-session-tier` edit), B (keep `seat-version-floor` at `class: must`) and C (edit no command section)

## Files touched (7)

| File | Change |
|---|---|
| `plugins/mochiko/migrations/0007-seat-default-key.yaml` | new, 150 lines, 11 ops |
| `.mochiko/schema-views/skills/patterns-model-tiering.yaml` | regenerated, +87 −18 |
| `plugins/mochiko/skills/patterns-model-tiering/SKILL.md` | +12 −9 (description · tagline · Overview) |
| `plugins/mochiko/skills/mochiko/SKILL.md` | +1 −1 (the tiering router row) |
| `evals/contract/expected-skills.json` | +3 −2 (`floor_ids`, `floor_pin`) |
| `.mochiko/strips/patterns-model-tiering.md` | +61 −0 |
| `.mochiko/strips/mochiko.md` | +23 −0 |

Diffstat over the six tracked files: 187 insertions, 30 deletions; plus the new migration.
`views emit` walked 50 documents and rewrote one — every other view is byte-identical, which is
the replay agreeing with the committed tree everywhere the migration did not reach.

No file outside P1's ownership was opened for writing. The four persona files, their strips and
`ARCHITECTURE.md` are P2's and were never touched.

## 1. Migration 0007 — 11 ops

`grammar: 1` · `id: 0007-seat-default-key` · `sequence: 7` · header `anchor: 2026-09-19
orchestrator-model-selection D1` · `hash: sha256:0287aa5d0681d9ebac8a370c7092556e5c9b4a32142063d2ff3773f3856d4ef3`
(the post-fix-round value; §9 carries the trail).
Every op names `schema: skill/patterns-model-tiering`.

| # | op | node | class · kind · labels | per-rule anchor |
|---|---|---|---|---|
| 1 | `reword-section` | `sec.scope` (title + intent) | — | header |
| 2 | `supersede-rule` | `rostered-seats-never-retier` | was floor | `…D1` (op-level, required) |
| 3 | `mint-rule` → `sec.scope` | `seat-default-key` | floor · binding · boundary | `…D3` |
| 4 | `mint-rule` → `sec.discipline` | `seat-deviation-lane` | must · latitude · reporting | `…D2` |
| 5 | `mint-rule` → `sec.discipline` | `seat-deviation-bounds` | floor · bound · boundary | `…D5` |
| 6 | `mint-rule` → `sec.discipline` | `seat-version-floor` | must · bound · binding | `…D2` |
| 7 | `mint-rule` → `sec.disclosure` | `seat-roster-disclosure` | must · duty · reporting | `…D2` |
| 8 | `reword-rule` | `override-is-the-pin` | floor, unchanged | header covers |
| 9 | `reword-rule` | `class-key-session-tier` | floor, unchanged | header covers |
| 10 | `reword-rule` | `worker-seat-set-reserved` | floor, unchanged | header covers |
| 11 | `reword-section` | `sec.reserved` (note) | — | — |

**Sequence 7, renumbered mid-wave.** The file was authored, stamped and gated at sequence 6, then
renumbered to 7 when the lead re-checked the `primitive-evals-v2` branch and found both 0005
(`0005-artifact-homes`) and 0006 (`0006-leads-pen-no-patched-copy`) already claimed there. A
collision is a rejection, not a merge conflict, so the rename was applied and the file re-stamped:
the hash covers `{id, sequence, anchor, changes}`, so it moved from
`sha256:9c6f31ca…54677270` to the value above. Not one byte of rule content changed with it, and
both gates were re-run after the rename — the results in §2 are the post-rename ones.
`mochiko-cli migrate status` reads `log plugins/mochiko/migrations · grammar 1 · sequences 1..7
(5 migrations)`; the holes at 5 and 6 are the other branch's, and gaps are legal per the log
README's "Sequence allocation".

**ID continuity.** One id leaves, through `supersede-rule` with its own anchor — the only exit the
grammar allows for protected content. Three floor rewords keep their ids. Two sections are
reworded, neither tombstoned. No surviving rule text or section note references the superseded id:
the only two references were `sec.reserved`'s note (op 11 rewrites it) and the skill's own
`description:` (rewritten in §3). Verified by search across `plugins/mochiko/` and
`.mochiko/schema-views/` — the remaining hits are the append-only log itself
(`0001-genesis.yaml`, `0004-sonnet-worker-rung.yaml`), which is history by construction.

**Flag B, as ruled.** `seat-version-floor` ships `class: must · kind: bound`, while its sibling
`patterns-transport-floor.version-floor` is `class: floor · kind: bound`. The asymmetry is
deliberate, ruled by the lead at plan approval, and keeps the floor set at seven.

**Flag A, as ruled, plus V1's F1.** Op 9 makes four word-level edits to `class-key-session-tier`
and nothing else: `Session tier —` → `Seat tier —`; `or a strong seat does` → `or another rostered
seat does`; `stays on the strong tier` → `stays with the seat` (F1); `(model-tiered-seats D5;` →
`(orchestrator-model-selection D1/D3;`. The two middle edits are one defect the ruling creates and
V1 caught the half I missed: under D3 four rostered seats are `down`-class, so "strong" no longer
names the seat set, and the rule used the collided vocabulary twice. Flag A fixed the first
occurrence; F1 fixed the second.

## 2. Deterministic gates

Every figure below was taken after the V1 fix round (F1–F3), against the final tree — which is
also after the sequence-6 → sequence-7 rename.

### `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`

```
mochiko-cli migrate validate · 0 rejecting · 105 advisory
```

Exit 0. Zero rejecting findings. The 105 advisories are the log's standing set (condition
coverage, unused moments, enforces coverage, budget lines, zero-member labels) and the migration
adds none of its own: the tiering skill's only advisory is its budget line,
`budget · skill/patterns-model-tiering · - · 21 rules · 9039 resolved characters of rule text`.
No `condition-coverage` or `zero-member-label` finding names this skill — the five minted rules
declare no `when:` (the schema declares no `conditions:` block) and every label they carry is live
in the skill-labels registry.

Similarity sweep: `none — no pair clears the threshold`, so no new near-duplicate edge and no
allowlist entry is owed.

### `python3 evals/contract/run.py --host-only`

**Exit code 0.** `contract suite: 4/4 cases passed, 4 ran, 4 measurement(s) recorded and not asserted`

Per-case verdicts: `ok hook-input` · `ok converted-shape` · `ok render-ceiling` · `ok deliverables`.

The two decisive lines inside `converted-shape`, which is where criterion (1)'s frozen set is
cross-checked against the render in both directions:

```
ok    the pre-registered floor set matches the patterns-model-tiering render (7 ids)
ok    patterns-model-tiering: the `floors:` line agrees with the section renders
```

`render-ceiling` decisive lines:

```
ok    no rendered rule still names a shipped schema file (when the binary is absent · plugins/mochiko/schemas/)
rec   largest render — implement · impl.sec.tools — 15,332 chars / 15,499 bytes, 51.1% of the ceiling
```

The tiering skill's largest section after the migration is `sec.discipline` at 4,452 characters,
about 15% of the ~30,000-character inline ceiling, so the growth costs no headroom there.

### `mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views`

```
mochiko-cli views emit · 50 documents · .mochiko/schema-views
```

Regenerated, never hand-edited. The CI view ≡ replay comparison therefore holds.

## 3. The rendered pair

### Floor pin and index, read back from the preamble

```
pins
- class: floor · 7 rules

floors: patterns-model-tiering.class-key-session-tier · patterns-model-tiering.seat-default-key · patterns-model-tiering.override-is-the-pin · patterns-model-tiering.seat-deviation-bounds · patterns-model-tiering.worker-return-is-a-claim · patterns-model-tiering.brief-obligation · patterns-model-tiering.worker-seat-set-reserved
```

Seven, in render order, exactly the set the plan predicted. `rostered-seats-never-retier` is gone;
`seat-default-key` and `seat-deviation-bounds` are in.

Section counts after the migration: `trigger` 3 · `scope` 1 · `discipline` 8 · `inputs` 2 ·
`disclosure` 6 · `reserved` 1 — 21 rules. The `scope` section is retitled
`Scope — dispatch tier and the seat default`; the other five titles are unchanged, so the `!`
lines in `SKILL.md` still enumerate every section in the render's own order (asserted by
`converted-shape`).

### `SKILL.md`

- `description:` **1,208 characters** of the parsed value by the canonical snippet in
  `.mochiko/memory/primitive-cost-budgets.md`, up from 998, under the 1,536 delivery cap. This
  skill has no measured `description:` budget row, so the hard cap is the only bar.
- Tagline and the Overview's opening sentence reworded; the Overview's economics sentence is
  word-for-word what it was, only re-wrapped so the paragraph does not sit ragged.
- The `## Rules — delivered by mochiko-cli` block is **byte-identical** — halt clause, seven `!`
  lines, read-back sentence. It was never in an edit's `old_string`.

### Router row

`plugins/mochiko/skills/mochiko/SKILL.md`, the `patterns-model-tiering` row: three spans changed,
the rest of the row byte for byte. The WHEN clause gains spawning a rostered seat; `stay session
tier` becomes `stay seat tier` to track op 9's rename; the D5 jurisdiction clause becomes the seat
default key clause. The row's two other uses of "session tier" mean spawn inheritance, not the
class-key value, and stay.

### Contract-suite pre-registration

`evals/contract/expected-skills.json`, the `patterns-model-tiering` entry only: `floor_ids` 6 → 7
(sorted, as `freeze_expectations.py:136` writes them) and `floor_pin` 6 → 7. `family`,
`schema_bytes` 7232, `common` null, `common_bytes` null, `baseline_bytes` 7232, `baseline_source`
and `body_bytes_pre` 2337 are untouched — the field-scoped diff the README's "Criterion (1)"
demands of a replacement. The replacement is a new pre-registration by ruling, not an edit to
match a changed render: the ids were derived from the migration's own ops before the render was
read back.

## 4. Budget — measured, over, and argued

| | body | render | payload |
|---|---|---|---|
| before | 2,873 | 11,823 | 14,696 |
| after | 3,112 | 14,752 | **17,864** |

Measured with the canonical snippet for the body and the sum of the seven rendered blocks, taken
against the quiesced tree after the V1 fix round. The before figures reproduce the standing
`[v0.108.0]` row exactly, which is what makes the after figure comparable. The fix round moved the
payload by +5 against the pre-fix 17,859: F1 −5, F2 +62, F3 −52.

Standing budget row: **10,852, no headroom**, already carrying a ruled `+3,844` HOLDS overage at
v0.108.0 for the measured 14,696.

**Overage: +7,012 over the 10,852 budget; +3,168 over the v0.108.0 ruled payload.** V1 ruled this
overage HOLDS at round 1, on the argument below.

Justification offered to V1: a genuine new obligation set, never restored prose. The ruling gives
this skill a second jurisdiction it did not have — the tier every rostered seat runs at — and the
growth is that jurisdiction's own obligations, each stated once:

| new obligation | chars of rule text |
|---|---|
| the key, its criterion, the ten-row table, alias-not-id, no `inherit` | 784 |
| the deviation lane and who owns it | 371 |
| the four bounds on that lane, with G8's three `fable` caveats | 619 |
| the platform version floor and the two consumer-environment overrides | 381 |
| the run-report roster-line duty | 243 |

2,398 characters of new rule text, read off the render rather than off the migration source; the
supersession removes 207. The rest of the render delta is the per-rule headers the render prints,
the `override-is-the-pin` extension to seat spawns (261 → 429, `+168`), and the preamble's longer
`floors:` line. No clause restates a sibling skill, and nothing previously stripped came back.

## 5. G13 — the command surfaces, checked and left alone (flag C, as ruled)

All six command schemas were rendered section by section through `mochiko-cli rules <cmd>
--section <id>` and searched for `roster`, `seat roster`, `run report`, `model:`, `tier`, `haiku`,
`sonnet` and `opus`.

**No command carries roster grammar.** `roster`, `seat roster` and `run report` appear in no rule
of `brainstorm` · `specify` · `implement` · `setup` · `feature` · `architecture`. Each carries
exactly one tiering rule — `brainstorm.model-tiering`, `spec.model-tiering`,
`impl.model-tiering`, `setup.model-tiering`, `feat.model-tiering`, `arch.model-tiering` — and
every one of them points at this skill and restates nothing, so the single-source branch of G13
holds and **no command section is edited in migration 0007.**

Two `implement` rules name what a report must contain and were read and ruled out as not roster
grammar: `impl.sufficiency-report` (what the sufficiency verdict states) and
`impl.reports-envelope` (where reports land and which format they follow). Neither describes a
seat roster line.

The roster-line duty therefore lives only at `patterns-model-tiering.seat-roster-disclosure`,
whose text carries the conditional so a command that later grows roster grammar knows it owes the
tier column there too.

## 6. Notes of note

- **The `sec.reserved` note is view-only.** `mochiko-cli rules` prints a section's title and
  intent but not its `note:`, so op 11 changes the derived view and the log, and costs the render
  nothing. It still had to happen: the note named `rostered-seats-never-retier`, and leaving it
  would have left a dead pointer in the committed view.
- **Frontmatter is not strict YAML here.** The `description:` value contains `model: haiku` and
  `model: sonnet` inside backticks, which a strict plain-scalar parser would reject. That is the
  pre-existing convention in this file and across the library, and the new value follows it
  unchanged; the canonical measurement snippet reads the value by regex, not by a YAML parser.
- **`migrate stamp` reflowed the file.** The stamped migration is rewritten in the log's own
  layout, so its line wrapping differs from what was authored. No content moved.
- **Four of P2's strip files carried the pre-rename migration name; P2 repaired them in the same
  wave.** `.mochiko/strips/requirements-analyst.md`, `product-engineer.md`,
  `technical-analyst.md` and `product-manager.md` each cited migration `0006-seat-default-key`
  after the rename, which would have left four dead pointers — a defect under the
  knowledge-management invariants. P1 reported them to the lead rather than editing, those files
  being P2's owned surface under the wave's single-writer split, and P2 made the one-word fix.
  All four now cite `0007-seat-default-key`, verified by re-reading the four files.

## 7. Not done, and why

Nothing in P1 items 1–14 was left out. Items outside P1's ownership and deliberately untouched:

- the `plugin.json` / `marketplace.json` / `CHANGELOG.md` bump to 0.110.0 — the lead's
- the budgets row for the argued overage in `.mochiko/memory/primitive-cost-budgets.md` — the
  lead's, after V1 rules on the overage
- the four persona frontmatter flips, their strips, and `ARCHITECTURE.md` — P2's
- `evals/commands/agents.py`'s `ARM_MODEL` (D7 item 4) — deferred by the wave plan; the runner
  lives on `primitive-evals-v2`, not on `main`
- the full contract suite in the Docker sandbox — the maintainer-side gate at the bump, not P1's;
  `--host-only` is green and is the cheapest gate on delivery shape

## 8. For V1

Graded unit is the pair: `plugins/mochiko/skills/patterns-model-tiering/SKILL.md` plus the seven
blocks `mochiko-cli` renders for it, against the skill-pair criteria block of
`.claude/rules/mochiko/primitive-edits.md`. Reproduce with:

```
mochiko-cli migrate validate --report --plugin-root plugins/mochiko
mochiko-cli rules patterns-model-tiering --section preamble --plugin-root plugins/mochiko
python3 evals/contract/run.py --host-only
```

The char-budget pre-assert is §4 above, overage named and argued. The router row and the two
strip files are prose-primitive edits and take the matching-skill routing, not the pair block.

## 9. V1 round 1 — the fix round

V1 returned FAIL on two fidelity defects in floor text, ruled the overage HOLDS, and passed all
twelve skill-pair criteria. Four fixes were pinned by the lead and applied in place on the
untracked migration — no new migration file, no new strip entry, no `expected-skills.json` change,
since the migration had not yet landed and the log's history is what a landed edit would owe.

| fix | node | change |
|---|---|---|
| F1 | `class-key-session-tier` | `stays on the strong tier` → `stays with the seat` — the second occurrence of the vocabulary collision flag A fixed at the first |
| F2 | `seat-deviation-bounds` | inserted `, that thinking is always on with effort the only depth control,` after "prompted for Opus", so the `fable` caveat names all three of G8's items |
| F3 | `override-is-the-pin` | dropped the limb `, or a persona file carrying `inherit` or no `model:`,` — the `inherit` bar is single-homed in `seat-default-key` |
| F4 | `reports/p1-report.md` | confirmed every 0006 reference now reads 0007 / sequence 7 / the current hash |

All three schema fixes are confirmed in the render, not merely in the source: the trigger section
prints `mislead a producer stays with the seat`, and the discipline section prints `prompted for
Opus, that thinking is always on with effort the only depth control, and that a `refusal`` and
`an undisclosed override has failed it too`.

**Hash trail.** `sha256:9c6f31ca…54677270` (sequence 6, pre-rename) → `sha256:f07c75a8…ade43e1`
(sequence 7, pre-fix) → **`sha256:0287aa5d0681d9ebac8a370c7092556e5c9b4a32142063d2ff3773f3856d4ef3`**
(sequence 7, post-fix — the current file). The hash covers `{id, sequence, anchor, changes}`, so
both the rename and the fix round moved it.

**F4 check.** Every 0006 reference in this report is gone. The heading, the files table, §1's
`id`, `sequence`, and hash, and §5 all read 0007 / sequence 7. Two strings containing "0006"
survive deliberately and name something other than this migration: the other branch's
`0006-leads-pen-no-patched-copy` in §1's renumber paragraph, and §6's record of the four citations
in P2's strip files that briefly pointed at the pre-rename name. Those four were repaired by P2 in
the same wave and now cite `0007-seat-default-key`.

Gates re-run after the fix round and reported in §2 as the current figures: `migrate validate
--report` 0 rejecting, `views emit` 50 documents with no view drift beyond this skill,
`run.py --host-only` exit 0 at 4/4 with both `converted-shape` floor lines green, payload
re-measured at 17,864.
