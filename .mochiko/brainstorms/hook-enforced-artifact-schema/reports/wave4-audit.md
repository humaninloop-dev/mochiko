---
report: review
feature: hook-enforced-artifact-schema
round: 4
seat: validator (independent — authored none of the graded surfaces)
date: 2026-09-15
graded_tree: working tree at HEAD 794cea8 (nothing committed)
units: 7
verdicts_first_pass: 1 FAIL · 2 FAIL · 3 PASS · 4 PASS · 5 PASS · 6 11/11 HOLDS
verdicts_re_audit: 1 PASS · 2 PASS · 7 FAIL, then PASS on re-check — see the Re-audit section
blocking: 0 — every unit PASS as the tree stands; unit 6 rules 11/11 HOLDS
budget_pre_assert: run on every edited primitive with the canonical snippet; figures below
deterministic_pre_pass: mochiko-cli migrate validate --report --plugin-root plugins/mochiko — PASS, 0 rejecting
---

## Failure narrative

Units 1 and 2 fail on the rule text, not on the ceremony. Both added sections name a single
home for an artifact the migration log declares in two, and both hand the seat a literal
`mochiko-cli home` command whose placeholder path resolves to the wrong home. Run verbatim,
the command prints `specs-index`, the line `` `<slug>/` is NOT a declared sub-directory of
this home ``, no `data-model.md`/`api.yaml` deliverable and no bound — the opposite of what
the section promises. The ruled mint for the co-located sibling artifact
(`authoring-technical-requirements.artifact-home`, on `constraints-and-decisions.md`) names
all four candidate homes and uses a placeholder-free command form; these two sections do
neither. Both are fixable in a few lines each; the ceremony work around them (additive edits,
nothing removed, both bodies well under budget) is sound.

## Method

Every figure below was produced by this seat against the working tree, never read from the
cycle report. Character counts use the canonical snippet in
`.mochiko/memory/primitive-cost-budgets.md` (characters of the parsed value, never `wc -c`).
Rendered-rule figures come from `mochiko-cli rules <name> --section <id> --plugin-root
plugins/mochiko` over the seven `!` lines each `SKILL.md` enumerates. Home claims were checked
by running `mochiko-cli home <path>` and against the derived views under
`.mochiko/schema-views/homes/`. Diffs are `git diff` against HEAD `794cea8`.

---

## Unit 1 — `plugins/mochiko/skills/patterns-entity-modeling/SKILL.md` — FAIL

Prose skill, no rule set (`mochiko-cli rules patterns-entity-modeling --section preamble` →
`error: no command or skill named 'patterns-entity-modeling' in the log`; zero `!` lines; no
`.mochiko/schema-views/skills/patterns-entity-modeling.yaml`). Graded on internal coherence
plus preserved responsibilities, as the ceremony directs for a prose primitive.

**Preserved responsibilities — PASS.** `git diff --numstat` reports `14 0`: fourteen insertions,
zero deletions. Nothing was removed, re-worded, or re-homed. No `KEPT:` line, protected-set
line, or `DECISIONS.md`-traceable line is touched.

**Char budget — PASS.** Body 13,726 → 14,302 chars against a budget of 16,835. Under by 2,533.

**Rule text against D1a — FAIL, two findings.**

**F1 (blocking) — the home is wrong in half the runs the skill serves.** The section asserts:

> `data-model.md` is a declared deliverable of the spec home, `.mochiko/specs/<slug>/`.

`data-model.md` is a declared deliverable of **two** homes. `.mochiko/schema-views/homes/spec.yaml`
carries it at 150 lines whole-file, and `.mochiko/schema-views/homes/feature.yaml` carries it at
150 lines whole-file as well. Verified by running the binary:

```
$ mochiko-cli home .mochiko/features/FEAT-003/data-model.md --plugin-root plugins/mochiko
home: feature — One capability's work home
resolved: `data-model.md`, a declared deliverable
```

The design phase of an `/mochiko:implement` run writes into the feature home — that is why the
feature home declares `data-model.md`, `constraints-and-decisions.md` and `architecture.md` side
by side. A seat in that run, reading this section, is told its artifact belongs in the spec home.

The ruled mint for the artifact sitting immediately beside it in both homes gets this right,
and is the standard this section is held to:

```
authoring-technical-requirements.artifact-home:
  `constraints-and-decisions.md` is a declared file in whichever home the run owns — the
  spec, the feature, the epic or the product baseline: render `mochiko-cli home <that
  path>` before the first write and hold the bound it returns.
```

**F2 (blocking) — the literal command resolves to the wrong home.** The section's fenced block is:

```
mochiko-cli home .mochiko/specs/<slug>/data-model.md
```

Run verbatim, that is not a near-miss, it is a different answer:

```
home: specs-index — The specification index
directory: .mochiko/specs/
resolved: `<slug>/` is NOT a declared sub-directory of this home
deliverables:
  - index.md · no template · no bound declared
reports: this home opens no reports/ directory
```

`mochiko-cli home --help` takes "A repository-relative path, or an absolute one under the
working directory" — there is no placeholder mode. The section's whole value is that the seat
runs the command instead of copying a sibling; a command that returns the wrong home and an
explicit "NOT a declared sub-directory" line teaches the opposite lesson. The ruled mints avoid
this in both available ways: `<that path>` as an instruction (technical-requirements,
vertical-tdd, executing-tdd-cycle) or a placeholder-free literal
(`authoring-feature-map`: `mochiko-cli home .mochiko/features`).

**F3 (minor) — token inconsistency inside the edited file.** The new section spells the spec
directory `<slug>`, matching the home. Line 284 of the same file still reads
`python scripts/validate-model.py .mochiko/specs/<feature>/data-model.md`. One file, one home,
two tokens. The seat's own path sweep flagged this exact class in `commands/specify.md` and
called it cosmetic, then left it unfixed in a file it was editing anyway.

**Not owed, stated for the record.** D1a's third clause — "a templated file's section budgets
come from `mochiko-cli template <kind>` before drafting" — does **not** bind here. `data-model.md`
is declared `no template · 150 lines, whole file` in both homes, so the clause has no templated
subject in this skill. Its absence is correct, not a gap.

**Also correct.** "the file set and the budget are the migration log's" · "a file already on disk
may itself predate the declared shape" (the D1b sibling-copying failure mode, well put) · "A write
to a name the home does not carry is refused at write time, and so is a body past the bound"
(matches the gate's two checks) · "A new deliverable kind takes a migration in the plugin's log,
never a local exception" (matches D2's upstream route and its no-consumer-local-surface clause).
Heading style and placement — immediately before `## data-model.md Structure`, the template the
producer drafts from — are right.

## Unit 2 — `plugins/mochiko/skills/patterns-api-contracts/SKILL.md` — FAIL

Prose skill, no rule set (zero `!` lines, no schema view). Same two blocking findings.

**Preserved responsibilities — PASS.** `git diff --numstat` reports `15 0`. Additive only.

**Char budget — PASS.** Body 11,036 → 11,680 chars against a budget of 13,412. Under by 1,732.

**F1 (blocking) — single home asserted for a dual-home artifact.** The section says `api.yaml`
and the per-endpoint contract files "are declared deliverables of the contracts home,
`.mochiko/specs/<slug>/contracts/`". `.mochiko/schema-views/homes/feature-contracts.yaml`
declares the same three deliverables — `<slug>.md`, `api.yaml`, `README.md` — at
`.mochiko/features/<FEAT-ID>/contracts/`, with the identical `bounds_cite`. Confirmed:

```
$ mochiko-cli home .mochiko/features/FEAT-003/contracts/api.yaml --plugin-root plugins/mochiko
home: feature — One capability's work home     # <FEAT-ID> unresolved, see F2
$ mochiko-cli home .mochiko/specs/user-auth/contracts/api.yaml --plugin-root plugins/mochiko
home: spec-contracts — Interface contracts under a spec
resolved: `api.yaml`, a declared deliverable
```

**F2 (blocking) — both fenced commands resolve to `specs-index`.** Same defect as unit 1, twice:

```
mochiko-cli home .mochiko/specs/<slug>/contracts/api.yaml   → specs-index, not a declared sub-directory
mochiko-cli home .mochiko/specs/<slug>/quickstart.md        → specs-index, not a declared sub-directory
```

**F3 (minor) — token inconsistency inside the edited file.** New section uses `<slug>`; line 194
still reads `.mochiko/specs/<feature>/contracts/api.yaml`, as does line 18 of
`scripts/validate-openapi.py`.

**F4 (advisory) — one paragraph mixes a correct single-home fact with an incorrect one.**
"`quickstart.md` is a deliverable of the spec home one level up" is right: the spec home declares
it, the feature home does not. The `api.yaml` sentence beside it is wrong for the same reason it
is right. A reader has no way to tell the two apart, which is what makes the single-home form
unsafe rather than merely incomplete.

**Not owed.** The `mochiko-cli template <kind>` clause does not bind: `api.yaml`, `<slug>.md`,
`README.md` and `quickstart.md` all render as `no template`.

**Also correct.** "whose bound is the contract's own interface rather than a line count" matches
`bounds: elsewhere` / `bounds_cite: the contract's own interface, per mochiko:patterns-api-contracts`
exactly, with no number restated. "Read each home before the first write rather than copying a
sibling file" is the D1a obligation, correctly worded.

## Unit 3 — `plugins/mochiko/skills/executing-tdd-cycle/SKILL.md` — PASS

Pair audit: `SKILL.md` plus the seven blocks `mochiko-cli` renders.

**The edit.** `git diff --numstat` reports `1 1` — one line, the frontmatter `description:`.
`.mochiko/specs/<feature>/tasks.md` → `.mochiko/features/<FEAT-ID>/tasks.md`. Nothing else moved.

**Re-point is correct.** `.mochiko/schema-views/homes/feature.yaml` declares `tasks.md` as its
first deliverable, bound to the `tasks` template, at path `[.mochiko, features, <FEAT-ID>]`. The
new text uses the home's own token verbatim. The old text was also already contradicting the
skill's own body, which reads "the feature's `tasks.md`" at the card-reading step — the edit
closes that, so internal coherence improves.

**Char-budget pre-assert.**

| class | before | after | budget | headroom |
|---|---|---|---|---|
| `description:` value | 498 | 501 | 623 (ledger) / 1,536 (delivery cap) | 122 |
| delivered-at-invoke payload | 20,380 | 20,380 | 20,063 | **−317, see unit 6** |

The payload figure is byte-unchanged by this edit — the body is identical to HEAD and the log
was not touched this wave. The overage is wave 3's mint and is ruled in unit 6. The
`description:` class, the only class this edit moves, has 122 characters spare after it.
The seat's own reported figures reproduce exactly against my independent measurement.

**Rendered rules unchanged, floor read-back intact.** The preamble prints:

```
pins
- class: floor · 11 rules
floors: … · executing-tdd-cycle.artifact-home · …
```

and the `SKILL.md` read-back sentence is "state back the floor count the preamble's `class: floor`
pin prints and the ids its `floors:` line lists" — cites both, hard-codes neither. Criterion 3
satisfied. Section enumeration is the review six-set plus preamble, which is the ruled form for
the dense five. No floor removed; `artifact-home` is present as `class: floor`.

**Strip entry — `.mochiko/strips/executing-tdd-cycle.md`, PASS.** Supersession-by-ruling form,
all five fields present, stamped `[v0.109.0]` matching `plugin.json`, inserted newest-first above
the `[v0.107.0]` entry. `Content:` matches HEAD verbatim (checked against
`git show 794cea8:…`). `Consumers assessed` claims the fully-qualified path appears nowhere else
in the plugin — **verified**: `grep -rn "specs/<[A-Za-z-]*>/tasks\.md" plugins/mochiko/` returns
nothing, and 16 files mention the bare name with no directory prefix.

**One advisory.** The entry cites record **D3** (source of truth). D2 is the ruling that makes
`tasks.md` a feature-home deliverable and is the closer anchor; both sit in the same record, so
reconstruction is unharmed. Not a defect, worth a word at the landing.

## Unit 4 — `plugins/mochiko/templates/analyst-report-template.md` — PASS

**The edit.** `git diff --numstat` reports `4 2` — Usage Note 5 only, two lines out, four in.
Nothing else in the template changed.

**Re-point is correct.** `.mochiko/specs/<slug>/reports/analyst-report.md`. The spec home opens a
`reports/` directory with `by_type: {}` — any name, envelope-bound. The template's own skeleton
already carries `report: disclosure`, and `disclosure` is a member of the envelope's closed enum
(`[cycle, verification, final-validation, review, feasibility, disclosure]` in
`.mochiko/schema-views/templates/report-envelope.yaml`). So the re-point genuinely needed no sixth
migration, exactly as the strip claims. The token moves `<feature>` → `<slug>`, matching the home.

**Added text is accurate and restates no number.** "A report lands in its home's `reports/`
directory under any name, and the `report: disclosure` type above is what admits this one;
`mochiko-cli home <path>` prints the home that governs a given path" — `<path>` is an instruction,
not a literal placeholder command, so unit 1's F2 does not recur here.

**Strip entry — PASS.** All five supersession fields, `[v0.109.0]`, newest-first, `Content:`
verbatim against HEAD, ruling cited as record D2 plus the `spec` home's `reports:` block in
`0005-artifact-homes.yaml`. `Consumers assessed` claims the path appears in this template only —
**verified**: no other occurrence of `analyst-report.md` under `plugins/`.

## Unit 5 — `plugins/mochiko/templates/techanalyst-report-template.md` — PASS

Identical shape and identical evidence. `git diff --numstat` reports `4 2`, Usage Note 7 only.
Re-point to `.mochiko/specs/<slug>/reports/techanalyst-report.md`, `report: disclosure` already in
the skeleton, `<feature>` → `<slug>`. Strip entry carries all five fields, `[v0.109.0]`,
newest-first above `[v0.91.0]`, `Content:` verbatim against HEAD, ruling cited as D2 plus the
`spec` home's `reports:` block. `Consumers assessed` verified — no other occurrence under
`plugins/`. `Kept deliberately` names both surviving clauses.

## Unit 6 — budget-overage ruling, 11 rows — 11 HOLDS, 0 FAIL

**The eleven, identified independently** from the `mint-rule` ops in
`plugins/mochiko/migrations/0005-artifact-homes.yaml` (17 mints: 6 commands, 11 skills). Each
minted rule is `class: floor · kind: binding`, anchored
`2026-09-13 hook-enforced-artifact-schema D1` — the ruled D1a floor.

**How the ruling was decided.** For each row I measured the current delivered payload, then
re-rendered the same seven sections against a log directory holding `0001`–`0004` only. The
difference is the mint's exact cost. The test applied: **remove the mint — is the row under
budget?** If yes, the mint is the overage's cause and the rule HOLDS. All eleven pass that test.

| skill | payload | budget | over | mint cost | payload − mint | ruling |
|---|---|---|---|---|---|---|
| analysis-codebase | 13,954 | 13,776 | +178 | 523 | 13,431 | HOLDS |
| authoring-architecture-store | 19,955 | 19,733 | +222 | 558 | 19,397 | HOLDS |
| authoring-constitution | 29,824 | 29,614 | +210 | 546 | 29,278 | HOLDS |
| authoring-epic | 14,323 | 14,062 | +261 | 498 | 13,825 | HOLDS |
| authoring-feature-map | 22,437 | 22,323 | +114 | 522 | 21,915 | HOLDS |
| authoring-prototype | 15,176 | 14,980 | +196 | 499 | 14,677 | HOLDS |
| authoring-requirements | 12,633 | 12,373 | +260 | 497 | 12,136 | HOLDS |
| authoring-technical-requirements | 21,038 | 20,775 | +263 | 508 | 20,530 | HOLDS |
| authoring-user-stories | 13,727 | 13,444 | +283 | 519 | 13,208 | HOLDS |
| executing-tdd-cycle | 20,380 | 20,063 | +317 | 512 | 19,868 | HOLDS |
| patterns-vertical-tdd | 15,935 | 15,775 | +160 | 507 | 15,428 | HOLDS |

Overage range 114–317, reproducing the seat's sweep exactly, re-run twice against the quiesced
tree with identical results. Mint cost 497–558 per skill — larger than every overage, because
three separate v0.107.0 edits shrank each row underneath the mint.

**Method check.** The differential render is sound, not inferred: the `0001`–`0004` render of
`authoring-requirements.sec.artifact` contains zero occurrences of `artifact-home`, the full-log
render exactly one. The delta is the mint and nothing else.

**Full decomposition of every overage, all four components ruled and recorded.**

| component | size | ground |
|---|---|---|
| D1a mint (wave 3) | +497 to +558 | `class: floor`, anchored `2026-09-13 hook-enforced-artifact-schema D1` |
| v0.107.0 render-format change | −237, exactly, every skill | verified constant, see below |
| v0.107.0 two-arm retirement | a further −8 to −189, seven rows | `cli-schema-delivery` D9 |
| v0.107.0 body reword | +1 to +42, **seven** rows | `[v0.107.0]` strip entries |

**The −237 is a verified constant, not an average.** I measured six skills the mint never touched
— `review-feasibility`, `review-brainstorm`, `patterns-sound-loop`, `brownfield-integration`,
`review-sufficiency`, `testing-end-user`. Every one renders exactly 237 chars below its recorded
v0.106.0 figure. Five of the six sit exactly −237 on payload; `testing-end-user` sits −204 because
its body carries the same +33 reword drift.

**The residual in seven rows is also ruled.** Four of the eleven shrink exactly −237; seven shrink
between −245 and −426. `git diff 62aa99d 32c1ed5 --stat` shows the v0.107.0 wave appended 151
lines to `0003-two-arm-to-cli.yaml` — "Retire the two-arm delivery phrasing and every schema-file
path from the corpus", anchored `2026-09-03 cli-schema-delivery D9`, retiring rules such as
`analysis-codebase.deliverable-two-arm-binding` and touching `patterns-vertical-tdd`. Those seven
are exactly the skills that carried a two-arm rule. `0005` itself contains only the mint for each
of the eleven — two id mentions per skill, no `reword-rule` or tombstone — so it is not the source.

**No row is restored playbook prose.** The body deltas trace to the same v0.107.0 wave rewording
"the schema's `<id>` rules" to "delivered by `mochiko-cli` as the `<id>` rules", a recorded
supersession, never a re-add. Every component of every overage is a ruled edit. The
`patterns-model-tiering` precedent the seat cites (standing +3,844 ruled-HOLDS from the
sonnet-worker-rung mints) is the right form.

**Owed at the landing, not fixable here.** Eleven ledger rows in
`.mochiko/memory/primitive-cost-budgets.md` need a standing-overage line stamped at the version
that ships them, in the `patterns-model-tiering` form. **Stamp them from re-measurement, never by
adding the mint to the recorded figure** — seven of the eleven recorded render figures are stale
beyond the uniform −237, so the arithmetic would carry the staleness forward. Name the v0.107.0
format change, the D9 two-arm retirement and the body reword in the same line, so the next sweep
does not re-discover them as new drift.

---

## Fix list

**Blocking — unit 1, `patterns-entity-modeling/SKILL.md`:**

1. Name both homes. `data-model.md` is a declared deliverable of the spec home **and** the feature
   home; say so, in the form `authoring-technical-requirements.artifact-home` already uses for the
   file beside it.
2. Make the fenced command runnable, or stop presenting it as one. Either write it with a real
   path and mark the placeholder as the reader's substitution, or use the instruction form
   `mochiko-cli home <that path>`. A copy-pasteable `<slug>` returns `specs-index` and an explicit
   "NOT a declared sub-directory" line.
3. Optional, same edit: spell the spec directory `<slug>` at line 284 so one file carries one token.

**Blocking — unit 2, `patterns-api-contracts/SKILL.md`:**

4. Name both contracts homes — `.mochiko/specs/<slug>/contracts/` and
   `.mochiko/features/<FEAT-ID>/contracts/` carry the identical deliverable set.
5. Fix both fenced commands, same remedy as item 2.
6. Optional: `<slug>` at line 194 and in `scripts/validate-openapi.py` line 18.

**Non-blocking — carried to the landing:**

7. Stamp the eleven standing overages into `.mochiko/memory/primitive-cost-budgets.md` at the
   shipping version, **from re-measurement, not arithmetic** — seven recorded render figures are
   stale beyond the uniform −237. Name the D1a mint, the v0.107.0 render-format change, the D9
   two-arm retirement and the body reword in the same line.
9. Two figures in the cycle report need correcting before they are carried into the ledger: the
   mint costs 497–558 gross, not 350–550, and the ~237 is a v0.107.0 format constant, not prior
   headroom — these budgets were re-seeded at v0.106.0 with none.
8. Consider citing record D2 beside D3 in the `executing-tdd-cycle` strip entry.

Re-audit scope on the fix: units 1 and 2 only. Units 3, 4, 5 and the unit-6 ruling stand.

## Notes of note

The single-home wording is the interesting failure, because the migration log made it newly
wrong. Before wave 3 these artifacts had one conventional location and naming it was fine;
`0005` declares them in two homes, and the same sentence is now a hazard. The ruled mints were
authored against that fact and read home-agnostically; the two markdown sections were authored
against the older habit. Worth watching in wave 5's violator pass, where the same assumption
would re-home files into the wrong tree.

`mochiko-cli home` refusing placeholder paths is correct behaviour and probably should stay,
but it means any documentation showing a literal command must use a placeholder-free path. That
is a house rule worth writing down once rather than catching per-primitive.

---

# Re-audit

Bounded second pass at the lead's open, on the current tree. Default FAIL, nothing carried over
from the first pass. Verdicts: **unit 1 PASS · unit 2 PASS · unit 7 FAIL.**

## Unit 1 — `patterns-entity-modeling/SKILL.md` — PASS

Both blocking findings are closed, and closed correctly rather than papered over.

**F1 resolved, and the claim is now complete.** The section reads "`data-model.md` is a declared
file in whichever home the run owns — the spec, the feature, the epic or the product baseline".
I resolved the deliverable against the whole homes set rather than checking the four asserted:
`grep -l "data-model.md" .mochiko/schema-views/homes/*.yaml` returns exactly four files —
`spec.yaml`, `feature.yaml`, `epic.yaml`, `product.yaml`. Four claimed, four declared, none
missed. The wording now matches the ruled mint
`authoring-technical-requirements.artifact-home` almost phrase for phrase, which is the right
model to have copied.

**F2 resolved.** No fenced command survives. `grep -n 'mochiko-cli home'` returns one line, the
instruction form "Render `mochiko-cli home <that path>` for the file you are about to write". A
seat now substitutes a real path instead of pasting a placeholder that resolves to `specs-index`.

**F3 dissolved.** The section no longer spells a spec-directory token at all, so it cannot
disagree with line 284. The wider `<feature>`/`<slug>` inconsistency elsewhere in the plugin is
untouched and stays advisory, as the seat's own sweep classified it.

**Preserved responsibilities — PASS.** `git diff --numstat` reports `11 0`. Still additive; the
fix shrank the section by three lines rather than adding to it.

**Char budget — PASS.** Body 14,355 against a budget of 16,835, under by 2,480. The seat's
figure reproduces exactly.

**`mochiko-cli template <kind>` still not owed.** `data-model.md` renders `no template ·
150 lines, whole file` in all four homes, checked in each. The clause binds templated files and
has no subject here. Its continued absence is correct.

## Unit 2 — `patterns-api-contracts/SKILL.md` — PASS

**F1 resolved, both artifacts.** "declared files in the `contracts/` home under whichever scope
the run owns — the spec, the feature, the epic or the product baseline". `grep -l "api.yaml"`
returns exactly four homes — `spec-contracts`, `feature-contracts`, `epic-contracts`,
`product-contracts` — each declaring the same three deliverables.

**The bound claim holds across all four.** "each of those homes declares its bound as the
contract's own interface rather than a line count" — I checked every one:

```
spec-contracts · feature-contracts · epic-contracts · product-contracts
bounds: elsewhere
bounds_cite: the contract's own interface, per mochiko:patterns-api-contracts
```

**The `quickstart.md` claim is right, and right in the harder direction.** The section says it is
"a declared file one level up, in the spec, the epic or the product home; the feature home does
not carry it." `grep -l "quickstart.md"` returns exactly `spec.yaml`, `epic.yaml`, `product.yaml`
— three homes, and `feature.yaml` is absent from the list. So both halves check out: the three
named homes are the three that declare it, and the stated exclusion is real. This was the claim
most likely to have been asserted from habit, and it was resolved against the log.

**F2 resolved.** One `mochiko-cli home` line, the `<that path>` instruction form. Both former
fenced commands are gone.

**F4 dissolved.** The paragraph no longer mixes a verified single-home fact with an unverified
one, because no single-home claim remains.

**Preserved responsibilities — PASS.** `git diff --numstat` reports `13 0`. Additive.

**Char budget — PASS.** Body 11,877 against 13,412, under by 1,535. Seat's figure reproduces.

**Not owed.** `api.yaml`, `<slug>.md`, `README.md` and `quickstart.md` all render `no template`,
so the `mochiko-cli template <kind>` clause has no subject in this skill either.

## Unit 7 — the eleven standing-overage lines — FAIL

Six of the seven criteria are met, precisely. One is missing from every row, and it is the one
that lets a row be checked.

**What is correct.** Every figure in all eleven rows reproduces against my own measurement of
the quiesced tree — payload, body, render, overage and mint cost, thirty-three numbers, no
discrepancy. Each row names the `artifact-home` floor, its `0005-artifact-homes.yaml` origin,
the record anchor `2026-09-13`, the measured mint cost, and wave-4 audit unit 6 as the ruling.
Prior history is preserved intact on every row, including
`authoring-feature-map`'s dissolved +562 note and `patterns-vertical-tdd`'s absorbed +294 trail.
**No budget number moved**: all eleven still read their seeded figure and "(no headroom)".
The figures were plainly re-measured, not derived — `authoring-requirements` and
`authoring-technical-requirements` carry an unchanged body, which arithmetic from the recorded
figure would not have produced.

**What is missing — the `0003` two-arm residual, absent from all eleven rows.** No row mentions
`0003`, the two-arm retirement, `cli-schema-delivery` D9, or the v0.107.0 render-format change.
The consequence is not cosmetic: **as written, no row's arithmetic closes.** A reader
reconciling `analysis-codebase` takes the recorded render of 9,088, adds the stated mint cost of
523, expects 9,611, and finds 9,228 — a 383-char hole the row does not account for. That breaks
the GI-006 reconstruction property the ledger exists to carry.

| skill | recorded render + mint | actual render | unexplained | of which v0.107.0 format | of which `0003` two-arm |
|---|---|---|---|---|---|
| analysis-codebase | 9,611 | 9,228 | −383 | −237 | −146 |
| authoring-feature-map | 16,909 | 16,483 | −426 | −237 | −189 |
| patterns-vertical-tdd | 10,386 | 10,027 | −359 | −237 | −122 |
| authoring-constitution | 22,465 | 22,128 | −337 | −237 | −100 |
| authoring-architecture-store | 14,900 | 14,564 | −336 | −237 | −99 |
| authoring-prototype | 10,878 | 10,556 | −322 | −237 | −85 |
| authoring-technical-requirements | 17,245 | 17,000 | −245 | −237 | −8 |
| authoring-requirements | 9,431 | 9,194 | −237 | −237 | — |
| authoring-user-stories | 9,434 | 9,197 | −237 | −237 | — |
| executing-tdd-cycle | 13,235 | 12,998 | −237 | −237 | — |
| authoring-epic | 11,431 | 11,194 | −237 | −237 | — |

**Attribution is evidence-backed, not inferred.** The −237 is an exact constant, verified on six
skills the mint never touched. The residual beyond it lands on exactly the skills the v0.107.0
append to `0003-two-arm-to-cli.yaml` names: `analysis-codebase`, `authoring-feature-map`,
`authoring-technical-requirements`, `patterns-vertical-tdd` directly, plus the three authoring
skills that bound the retired `authoring-common.two-arm-template` common stub. The four rows with
no residual are the four that bound neither. Retired ids visible in that diff include
`analysis-codebase.deliverable-two-arm-binding`, `authoring-feature-map.feature-entry-two-arm`,
`patterns-vertical-tdd.tasks-binding-two-arm` and `authoring-common.two-arm-template`.

**Fix — one clause per row, no number changes.** Add to each of the eleven the v0.107.0
render-format change of −237, and to the seven the `0003` two-arm residual with its size and the
`cli-schema-delivery` D9 anchor. Nothing else in these rows needs touching.

## Settled — seven body deltas, not six

**The seat is right and my first-pass report was wrong.** Non-zero body deltas against the
v0.106.0 seeded figures: `analysis-codebase` +38 · `authoring-feature-map` +18 ·
`authoring-prototype` +19 · `patterns-vertical-tdd` +12 · `executing-tdd-cycle` +42 ·
`authoring-constitution` +1 · `authoring-user-stories` +1. That is seven. I undercounted by
dropping the two +1 rows when I wrote the sentence; my own measurement table in unit 6 carried
them correctly all along, so the error was in the prose, not the data. The eleven ledger rows
name a body delta on exactly those seven, which is right. Unit 6's decomposition table is
corrected above.

## Unit 7 re-check — PASS

Third pass over the eleven ledger rows, on the tree as it stands. All four checks clear.

**The closing identity holds on all eleven.** I recomputed `overage = mint + body delta − 237 −
0003 reduction` from my own measured figures, not from the row's:

| skill | mint | body | format | `0003` | computed | stated |
|---|---|---|---|---|---|---|
| analysis-codebase | 523 | +38 | −237 | −146 | 178 | 178 |
| authoring-constitution | 546 | +1 | −237 | −100 | 210 | 210 |
| authoring-feature-map | 522 | +18 | −237 | −189 | 114 | 114 |
| authoring-requirements | 497 | — | −237 | — | 260 | 260 |
| authoring-user-stories | 519 | +1 | −237 | — | 283 | 283 |
| authoring-prototype | 499 | +19 | −237 | −85 | 196 | 196 |
| authoring-technical-requirements | 508 | — | −237 | −8 | 263 | 263 |
| executing-tdd-cycle | 512 | +42 | −237 | — | 317 | 317 |
| patterns-vertical-tdd | 507 | +12 | −237 | −122 | 160 | 160 |
| authoring-architecture-store | 558 | — | −237 | −99 | 222 | 222 |
| authoring-epic | 498 | — | −237 | — | 261 | 261 |

Eleven of eleven close exactly. Every payload, body and render figure in the rows also matches
my measurement of the quiesced tree.

**Each row names only the limbs it carries.** Seven rows carry the `0003` render limb —
`analysis-codebase`, `authoring-architecture-store`, `authoring-constitution`,
`authoring-feature-map`, `authoring-prototype`, `authoring-technical-requirements`,
`patterns-vertical-tdd` — which is exactly the set my differential measurement found with a
residual beyond the constant. A different seven carry the body limb, matching my seven non-zero
body deltas. `authoring-requirements` and `authoring-epic` carry neither, correctly: both measure
zero on both. Each row's stated component count — Two, Three or Four — matches the limbs it then
lists, on all eleven.

**The two limbs are now separately and correctly sourced.** The body limb reads "from the
v0.107.0 two-arm body reword, strip-recorded in that wave's entry"; the render limb reads "from
the two-arm rule retired by `0003-two-arm-to-cli.yaml` (record `cli-schema-delivery` D9, anchor
2026-09-03)". That distinction is the one that matters for GI-006: the migration changed log
rules and the render, never a `SKILL.md` body, and the body edit has its own record. I confirmed
all seven body-limb skills carry a `## [v0.107.0]` entry in their strip file, so the pointer
resolves in every case.

**No budget number moved, prior history intact.** Eleven rows changed and nothing else in the
ledger. Every row's right-hand column is byte-identical to HEAD and still reads "(no headroom)",
and each old row's entire parenthetical survives verbatim as a substring of the new one —
checked mechanically, not by eye. `authoring-feature-map`'s dissolved +562 note and
`patterns-vertical-tdd`'s absorbed +294 trail are both present.

**One note on what I graded.** An intermediate version of these rows, read earlier in this same
pass, attributed the body delta to `0003` — including on `executing-tdd-cycle` and
`authoring-user-stories`, two skills that migration does not mention at all. The current text
corrects that. The verdict above is against the current text.

**Unit 7: PASS.** No fix list.

## Unit 7, third stamp — PASS (confirms the re-check)

The frozen state named in this pass is byte-identical to the text graded in the re-check above.
The re-stamp had already landed when that verdict was taken, so nothing new arrived; the
intermediate version noted there — body delta credited to `0003` — precedes both. Re-graded
anyway, from a fresh live measurement rather than the prior pass's numbers, and with the
render-form identity the lead put.

**All three legs close on all eleven.** The payload identity alone could hide an offsetting pair
of errors, so this pass checks each side separately against a live re-measure:

- **render leg** — recorded render − 237 − `0003` reduction + mint = measured render
- **body leg** — recorded body + body delta = measured body
- **overage leg** — measured payload − budget = the stated overage

Eleven of eleven pass all three. The worked example reproduces exactly:
`analysis-codebase` 9,088 − 237 − 146 + 523 = 9,228 measured, body 4,688 + 38 = 4,726 measured,
payload 13,954 against budget 13,776 = +178 stated.

**Limb sets are right, including the swap.** Seven rows carry the `0003` render limb; a different
seven carry the body limb, dropping `authoring-architecture-store` and
`authoring-technical-requirements` and picking up `authoring-user-stories` and
`executing-tdd-cycle`. Both sets match my independent measurement element for element, and
`authoring-requirements` and `authoring-epic` carry neither, correctly. Every row's stated
component count agrees with the limbs it lists.

**Anchors, budgets and history, checked mechanically.** The `cli-schema-delivery` D9 anchor with
its 2026-09-03 date is present on every one of the seven render limbs and on no row that lacks
one. The −237 clause is on all eleven. Every budget cell is byte-identical to HEAD, and each
old row's full parenthetical survives verbatim inside its replacement. Eleven rows changed;
nothing else in the ledger did.

**Unit 7: PASS.** No fix list. The ledger is landable as it stands.
