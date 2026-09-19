---
report: review
round: 1
---

## Failure narrative

Unit 1 (schema content) is FAIL on three blocking findings. The migration is grammar-clean and
the AM-2 five hold, but three obligations of the ruling it executes are not delivered as the
record phrases them. First, D4 and build-surface item 1 put the loop bound's number in one place;
`validation-primitive-edit.gate-loop-bound` restates it while asserting its number lives
elsewhere. Second, D3's mechanical split is a build-time inventory of what `migrate validate`
actually checks; `validation-primitive-edit.judgment-items-pair` assigns scaffold headings and
order to the pre-pass, which never reads a primitive's markdown. Third, build-surface item 1
phrases setup's grader as "a plain fresh seat"; `setup.validate-seat-form` drops "plain", so D7's
persona-less default is unstated at that gate site. Each fix is a reword in a follow-on migration
and none touches an id. Unit 2 is PASS with no blocking findings.

## Unit 1 — VALIDATE

VALIDATE: schema content — `plugins/mochiko/migrations/0008-gate-form.yaml` together with its
regenerated view diff.

Checklist run: the AM-2 five from governance ledger GI-004 as re-expressed at v3.0.0, then
fidelity to record decisions D2, D3, D4, D5, D6, D7, D9, D11 and build-surface item 1.

Evidence read: the migration file in full; the three changed view diffs
(`common/common.yaml`, `commands/setup.yaml`, `skills/patterns-model-tiering.yaml`); the new
`.mochiko/schema-views/skills/validation-primitive-edit.yaml` in full; the record's Decisions and
Build-surface sections; `plugins/mochiko/migrations/README.md`; the GI-004 ledger block;
`crates/mochiko-cli/src/validate.rs` for the check set; the corpus views for collision hunting.

## Unit 1 — pre-pass, run first-hand

`mochiko-cli migrate validate --report --plugin-root plugins/mochiko`
`mochiko-cli migrate validate · 0 rejecting · 105 advisory`
`pointer resolution: 84 checked against plugins/mochiko`
`rules scanned: 1067 · in-kind pairs scored: 156764 · clusters: 0 (none)`

`mochiko-cli migrate status --plugin-root plugins/mochiko`
`log plugins/mochiko/migrations · grammar 1 · sequences 1..8 (8 migrations)`
`state sha256:8950f9b90e1fc2bc3d964097bc36b462aee29d0135b74dbc67110c1cca929ddb · 74 documents · 1067 rules`

## Unit 1 — pre-pass, views equal replay

`mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views`
`mochiko-cli views emit · 74 documents · .mochiko/schema-views`
then `git status --short .mochiko/schema-views` printed the same four lines as before the emit,
and a recursive checksum over the tree was unchanged at
`f1675927633026c943883e5db0153ba6b9b7709c22b2f6bd19125ecbb96ec2fe`. Views are the replay.

## Unit 1 — pre-pass, the render pin

`mochiko-cli rules validation-primitive-edit --section preamble --plugin-root plugins/mochiko`
`pins` / `- class: floor · 11 rules` — the pin is 11, as expected, and the `floors:` line names
eleven ids. The six sections render as `independence` · `scope` · `inputs` · `verdict` · `output`
· `reserved`, which is `validation-constitution`'s set exactly, with no empty section needing an
explicit marker.

## Unit 1 — the AM-2 five

1. Intent stated — PASS. The header `intent:` names all four changes (the common-block bound,
   setup's bound and validate-seat rule, the tiering pin, the new skill), and every one of the
   six sections carries its own `intent:`.
2. Anchor present where required — PASS. Header anchor `2026-09-19 author-grader-consolidation
   D7` is well-formed; thirteen rule-level anchors are present and each names the decision its
   rule executes. No `supersede-rule` or `tombstone-rule` op appears, so no protected exit owes
   one. `common.gate-loop-bound` is the corpus's first anchored common-block rule — no anchor
   exists anywhere under `.mochiko/schema-views/common/` at HEAD — and the binary accepted it
   (`anchor-format` is a rejecting code; validate reports 0 rejecting).

## Unit 1 — the AM-2 five, continued

3. ID lifecycle right — PASS. `mint-once`, `id-duplicate` and `id-prefix` are rejecting codes and
   validate reports none. No tombstone or supersession was owed: no loop-bound rule existed
   anywhere in the corpus; setup carried no validate-step grader rule; and
   `patterns-model-tiering.override-is-the-pin` covers the dispatch rungs and persona seat spawns,
   not the persona-less grader spawn the new floor pins.
4. Floor and fail survival — PASS. The migration removes nothing. New floors match the ruling:
   `persona-less-grader-pin` (D7's floor-miss clause), `setup.gate-loop-bound` (D6), and the
   skill's eleven. `validation-constitution`'s three completeness floors are untouched, which is
   the C1 fold.
5. Register — PASS. Zero `deixis` findings, and every rule text is obligation-shaped with no
   restated procedure. One wording note is carried below, not blocking.

## Unit 1 — ruling fidelity, what holds

D2 — PASS. `gate-job` states the binary and names the one site, routing the input job to the
review family and the governance set to `validation-constitution`.

D5 — PASS. `common.gate-loop-bound` opens "A gate verdict of FAIL", so build and input loops are
untouched.

D6 — PASS. The number is one, the re-audit is the same seat resumed, the delta read is
"what the fix touched and what it could have broken", and "fix again or drop" is the C2 fold.

## Unit 1 — ruling fidelity, what holds, continued

D9 — PASS. The `outcome-line` grammar is character-for-character the record's, and the two
landing homes match.

D11 — PASS. `one-seat-per-wave` carries the per-unit block, the per-unit line and the split
condition.

## Unit 1 — ruling fidelity, what fails

D3 — FAIL. The judgment sets and the first-hand pre-pass duty are right, but
`judgment-items-pair` assigns "scaffold headings and order" to the pre-pass. `migrate validate`
has no scaffold check: its code set is section-set, id and tombstone integrity, kind and fail
discriminators, extends conformance, pointer resolution, deixis and budget, and the only `.md`
it ever touches is a `pointer:` target. The `.md` scaffold criterion would then be checked by
neither leg.

## Unit 1 — ruling fidelity, what fails, continued

D4 — FAIL. `validation-primitive-edit.gate-loop-bound` restates the number and the human-gate
disposition, the mechanics `review-common.its-command-states-them` names as never restated,
while its own closing clause says the number lives in `common.gate-loop-bound`.

D7 — FAIL at setup only. `setup.validate-seat-form` says "a fresh seat that authored no surface"
where the record phrases it "a plain fresh seat". The skill's own `plain-seat-explicit-tier`,
`rendered-contract-only` and `brief-carries-unit` are right.

## Unit 1 — verdict

VERDICT: FAIL

Issues requiring fix:

1. `validation-primitive-edit.gate-loop-bound` (D4, build item 1) — the number is stated twice in
   the log. Reword the rule to cite the bound without restating it. A skill cannot `extends:` a
   command-common rule, so a citation is the available shape.
2. `validation-primitive-edit.judgment-items-pair` (D3, build item 2) — "scaffold headings and
   order" names a check the pre-pass does not run. Move it to the judgment list, or name the
   check that does assert it.
3. `setup.validate-seat-form` (D7, build item 1) — insert "plain" before "fresh seat", so the
   persona-less default is stated where the record states it.

## Unit 2 — VALIDATE

VALIDATE: contract-suite pre-registration — `evals/contract/expected-skills.json`,
`evals/contract/run.py`, `evals/contract/README.md`, `scripts/similar-rules-allowlist.yaml`.

Checklist run: the README's own bar at "When a migration legitimately moves a floor set" and
"A replacement moves `floor_ids` and `floor_pin`, and nothing else", then each of the four
field-scoped items the unit is keyed to.

Evidence read: `git diff` of all four files; the whole `PROBE_ARGUMENTS` review block and the
`EXPECTED["setup"]` block in `run.py`; the delivery-case builder and the summary printer around
both baseline divisions; the family aggregation; the allowlist header and its existing rows; the
rule texts on both sides of all three new allowlist edges.

## Unit 2 — pre-pass, run first-hand

A field-scoped diff of `expected-skills.json` against `HEAD` reported exactly three moves:
`patterns-model-tiering.floor_ids` (one addition), `patterns-model-tiering.floor_pin` (7 to 8),
and the new `validation-primitive-edit` row. Top-level `families` and `provenance` also moved;
no other row and no other field changed.

`mochiko-cli rules patterns-model-tiering --section preamble` prints `class: floor · 8 rules`
and its eight ids are the JSON's eight. `mochiko-cli rules setup --section preamble` prints
`class: floor · 20 rules` including `setup.gate-loop-bound`; `EXPECTED["setup"]` now holds
exactly twenty ids.

Summed: 117 command floor ids across six, 252 skill floor ids across thirty-one, and every
`floor_pin` equals its own list length.

## Unit 2 — checklist

1. `patterns-model-tiering` moved only the two floor fields — PASS. `schema_bytes` stays 7232 and
   `common`, `common_bytes`, `baseline_bytes`, `baseline_source` and `body_bytes_pre` are
   byte-identical, per the field-scoped diff above.
2. The new row's floor fields match the render — PASS. Its eleven `floor_ids` are the eleven the
   preamble renders, as a set, and `floor_pin` is 11.
3. Byte columns are 0 with a stated reason — PASS. All four are 0 and `baseline_source` reads
   "post-freeze member … no pre-conversion baseline exists, never measured". The field shape
   otherwise matches its review-family siblings.
4. `families.review.members` lists it — PASS, appended after `validation-constitution`.

## Unit 2 — checklist, continued

5. `EXPECTED["setup"]` gained exactly `setup.gate-loop-bound` — PASS. One id, one comment in the
   existing `0005-artifact-homes` style, and the set is a `frozenset`, so position is inert. It
   is a floor in the render.
6. The `PROBE_ARGUMENTS` row follows the table's shape — PASS. A two-tuple of argument and prose,
   placed in the review block after `validation-constitution`, naming a real shipped primitive.
7. The zero-baseline guard mirrors the existing one — PASS. It reuses the summary printer's
   `if baseline` test at `run.py:5160`; those two are the only divisions by a baseline in the
   file. Output for a non-zero baseline is unchanged string-for-string, and the new row's
   `invoke_old` resolves to `None`, so the family aggregation skips it rather than dividing.
8. Nothing else in `run.py` changed — PASS.

## Unit 2 — checklist, the allowlist

9. Each of the three rows names a genuine live edge, and nothing else moved — PASS, proved by
   running the detector with no allowlist in scope. A copy of the log under the scratchpad has no
   `scripts/` ancestor, and `find_allowlist` walks up from the log directory, so
   `mochiko-cli migrate validate --report --log-dir <copy> --plugin-root plugins/mochiko` reports
   `clusters: 3 (CROSS-PAIR 3)` and `allowlist-suppressed edges: 168` against 171 in the repo run.
   The three clusters are exactly the three added rows. Both sides of each were read: the
   evidence-floor and default-fail edges are stub-versus-local, and the third is a real
   near-duplicate of read-boundary framing whose `reason:` correctly argues keep-distinct. The
   diff is purely additive.

## Unit 2 — verdict

VERDICT: PASS

Issues requiring fix: none.

## Notes of note

Two non-blocking observations, neither changing a verdict.

The migration writes the second AM-2 criterion as "anchor present where the exit requires one"
where the record and GI-004 write "anchor present where required". The substitution sharpens
rather than narrows, since the log's anchor rule is defined on protected exits, but it is a
deviation from the record's wording.

The new allowlist comment block calls all three edges "the stub-versus-local shape". The
detector marks two of them EXTEND-GAP and the third CROSS-PAIR only, so that sentence
over-generalizes; the rows themselves are correct.

## Notes of note, continued

The README's absolute case total is stale, and this diff inherits rather than causes it.
`python3 evals/contract/run.py --list` declares eighty-nine cases; the README's table lists
eighty-four. The gap is three host cases (`gate-input`, `reminder-input`, `if-placement`) and the
two wave-4 hook cases (`gate-live`, `reminder-spawn`) that were never added to the table, which
is also why the same page says "four host cases" in one paragraph and "seven host cases" in
another. At `HEAD` the figures were eighty-two against a builder-declared eighty-seven, the same
five. The change this diff makes — two cases and four sessions for one new skill — is
arithmetically right on both figures, and the second figure's 159 exceeds the first's 155 by the
three hook sessions plus the preflight probe, as it did before.

## Outcome lines

audit: schema content (migration 0008-gate-form + view diff) · validator · opus · 5 files · 1 rounds · 3 blocking

audit: contract-suite pre-registration · validator · opus · 4 files · 1 rounds · 0 blocking
