---
report: review
round: 2
---

## Validate

VALIDATE: wave 1, unit "schema content" — `plugins/mochiko/migrations/0008-gate-form.yaml` plus its
regenerated derived-view diff (`.mochiko/schema-views/commands/setup.yaml`,
`.mochiko/schema-views/common/common.yaml`,
`.mochiko/schema-views/skills/patterns-model-tiering.yaml`, and the new
`.mochiko/schema-views/skills/validation-primitive-edit.yaml`). Round-2 seat, fresh, authored
nothing in the unit.

Checklist run: the AM-2 five for schema content (governance ledger GI-004 as re-expressed at
v3.0.0) — intent stated · anchor present where required · ID lifecycle right · floor and fail
survival · register. Ruling fidelity against
`.mochiko/brainstorms/author-grader-consolidation/record.md` D2, D3 (with its C1 fold), D4, D5,
D6, D7, D9, D11 and build-surface item 1.

Evidence read: `plugins/mochiko/migrations/0008-gate-form.yaml` (whole file) ·
`.mochiko/schema-views/skills/validation-primitive-edit.yaml` (whole file, 206 lines) ·
`git diff HEAD -- .mochiko/schema-views/` for `commands/setup.yaml`, `common/common.yaml`,
`skills/patterns-model-tiering.yaml` · `.mochiko/brainstorms/author-grader-consolidation/record.md`
(Decisions D1–D11, Build surface, Evidence honesty, Open questions, Session trail) ·
`.mochiko/memory/governance-ledger.md` (GI-004 block, AM-2 re-expression) ·
`plugins/mochiko/migrations/README.md` (grammar, anchor rule, change ops) ·
`.mochiko/schema-views/labels/skill-labels.yaml` · `.mochiko/schema-views/labels/command-labels.yaml`
· `.mochiko/schema-views/skills/validation-constitution.yaml` (six-set comparator) ·
`.claude/rules/mochiko/primitive-edits.md` (criteria provenance for the prose and overage items).
Neither `reports/p1-plan.md` nor `reports/a2-audit.md` was opened.

Pre-pass (run by this seat, output quoted):

`mochiko-cli migrate validate --report --plugin-root plugins/mochiko`

```
budget · skill/validation-primitive-edit · - · 21 rules · 5369 resolved characters of rule text
pointer resolution: 84 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
mochiko-cli migrate validate · 0 rejecting · 105 advisory
```

`mochiko-cli migrate status --plugin-root plugins/mochiko`

```
log plugins/mochiko/migrations · grammar 1 · sequences 1..8 (8 migrations)
state sha256:30333fa88897cbc9c8ab52c5f27d541cf616b477465a45593966a9f3355238af · 74 documents · 1067 rules
```

`mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views` then
`git status --short .mochiko/schema-views`

```
mochiko-cli views emit · 74 documents · .mochiko/schema-views
 M .mochiko/schema-views/commands/setup.yaml
 M .mochiko/schema-views/common/common.yaml
 M .mochiko/schema-views/skills/patterns-model-tiering.yaml
?? .mochiko/schema-views/skills/validation-primitive-edit.yaml
```

Emit was run twice. The second run left `git status` byte-identical and the diff hash unchanged
(`db97b5c02f058b992c30a98af454466f993b3759e3483a9edf6452cf1dd4e235` both times), so the four paths
above are the replay, and nothing in the tree disagrees with the log. Those four paths are the
unit's own deliverable; no fifth path moved.

`mochiko-cli rules validation-primitive-edit --section preamble --plugin-root plugins/mochiko`

```
pins
- class: floor · 11 rules
sections
- validation-primitive-edit.sec.independence · ... · 4 rules
- validation-primitive-edit.sec.scope · ... · 3 rules
- validation-primitive-edit.sec.inputs · ... · 3 rules
- validation-primitive-edit.sec.verdict · ... · 7 rules
- validation-primitive-edit.sec.output · ... · 3 rules
- validation-primitive-edit.sec.reserved · ... · 1 rules
```

All six sections rendered individually (21 rules total, matching the budget line). `setup
--section setup.sec.roles` renders 5 rules including `setup.validate-seat-form`; `--section
setup.sec.boundaries` renders 7 including `setup.gate-loop-bound` resolved from the common block.

AM-2 five:

- **Intent stated — PASS.** Header `intent:` is one folded line naming all four changes and their
  purpose: "Ship the primitive-edit gate form: the one-place FAIL loop bound in the command common
  block, setup's bound and validate-seat rule, the persona-less grader tier pin, and the
  validation-primitive-edit skill that carries the gate contract as rules."
- **Anchor present where required — PASS.** The grammar requires a header anchor only for a
  supersession, a tombstone of protected content, or a lowering of protection
  (`plugins/mochiko/migrations/README.md`, "The anchor rule"). This migration performs none — its
  ops are four `mint-rule` and one `import-document`. A well-formed header anchor is present
  anyway (`2026-09-19 author-grader-consolidation D7`), and fifteen of the twenty-five new rules
  carry rule-level anchors naming their deciding decision.
- **ID lifecycle right — PASS.** Every id is new and namespaced to its own document
  (`common.gate-loop-bound`, `setup.gate-loop-bound`, `setup.validate-seat-form`,
  `patterns-model-tiering.persona-less-grader-pin`, twenty-one `validation-primitive-edit.*`). No
  id is reused, moved, or retired. `import-document` names a document that did not exist, which the
  hard set would otherwise reject, and the four `extends:` targets in `skill-review-common` all
  resolve — the pre-pass reports 84 pointers checked and 0 rejecting.
- **Floor and fail survival — PASS.** No op removes, rewords, supersedes, tombstones or
  re-fields any existing rule, so no floor and no `kind: fail` rule can have been lost. The
  migration is purely additive: it adds two floors to `setup` and `patterns-model-tiering` and
  eleven to the new skill. A skill schema cannot carry `kind: fail` by grammar, and the new
  document correctly carries none.
- **Register — PASS.** Every label used is live in the right registry. `user-gate` and
  `independence` on `common.gate-loop-bound`, and `independence`/`seats` on
  `setup.validate-seat-form`, are in `command-labels`. `boundary`, `independence`, `binding`,
  `fence`, `evidence`, `verdict`, `reporting` and `user-gate` across the skill and the tiering pin
  are in `skill-labels`. No rule reaches for `seats`, which `skill-labels` does not carry. The
  pre-pass reports no unregistered-label rejection and no new zero-member-label advisory.

Ruling fidelity:

- **D2, gate job and its one site — PASS.** `validation-primitive-edit.gate-job` states the binary
  the lead cannot ship past, scopes it to a shipped `plugins/mochiko/` primitive before the
  `plugin.json` bump citing GI-004, and routes the input job to the review family and the
  governance surface set to `mochiko:validation-constitution`.
- **D3 posture, tamper-proof clause and pre-pass — PASS.** `default-fail` carries the posture by
  `extends:`; `tamper-proof-clause` states that a verdict with no evidence-read line is FAIL
  automatically; `pre-pass-first-hand` carries the I3 fold in the record's own terms, naming
  `mochiko-cli migrate validate --report` and the char-budget measurement as the grader's own run
  and adding that nothing the output asserts is re-derived by judgment.
- **D3 judgment sets keyed by unit — PASS.** `unit-keyed` keys the criteria by unit kind and
  `judgment-items-schema` carries the AM-2 five verbatim. The round-1 finding on
  `judgment-items-pair` is fixed correctly: scaffold headings and order now sit on the judgment
  side, and the mechanical list is confined to what `migrate validate` actually asserts. I checked
  that inventory rather than assuming it — the crate carries a `FailSegment` code, pointer
  resolution, id and ontology checks, and no scaffold check anywhere in
  `crates/mochiko-cli/src/`. Build-surface item 2 requires exactly this build-time inventory and
  forbids guessing the split. The two items beyond the record's five, scaffold conformance and an
  argued overage, are both pre-existing criteria in `.claude/rules/mochiko/primitive-edits.md`.
- **D3 C1 fold — PASS.** The shrink reaches the primitive-edit gate only.
  `setup.validate-seat-form` says the validate grader runs `mochiko:validation-constitution` "with
  its floors unchanged", and the migration touches no `validation-constitution` rule.
- **D4, one-place bound — FAIL.** The home is right: `common.gate-loop-bound` is a command-common
  block carrying the literal number in its own text, `setup` alone extends it, and the text names
  its two citing consumers so a rename sweeps them. But D4 also rules that skills never restate
  the bound, and the skill document restates its number in two places. See the failure narrative.
- **D5, gate loops only — PASS.** The bound is minted in the common block and extended by `setup`
  alone. No input loop and no build loop is touched; `implement`'s attempt bounds are untouched by
  every op in the file.
- **D6, one re-audit then the user — PASS on content.** `common.gate-loop-bound` carries the
  number, the same-seat-resumed clause from the I5 fold, the delta read, the halt, the C2 fold's
  "fix again or drop", and "no run raises this bound". `second-fail-user` reserves the disposition
  to the user in the record's own wording and adds the C2 reasoning the common rule omits.
- **D7, plain fresh seat pinned to a tier — PASS.** The round-1 finding is fixed:
  `setup.validate-seat-form` now says "a plain fresh seat", the record's phrase.
  `plain-seat-explicit-tier` and `patterns-model-tiering.persona-less-grader-pin` both make an
  omitted `model:` alias a floor miss whatever the verdict says, and `rendered-contract-only` puts
  a hand-written contract section on the same footing. `brief-carries-unit` holds the dispatcher to
  the unit, the paths and the pre-pass commands.
- **D9, outcome line — PASS.** `outcome-line` reproduces the grammar character for character,
  including the optional `cost:` field and its launched-session-only condition, and names both
  landing homes.
- **D11, one seat per wave — PASS.** `one-seat-per-wave` carries the shape, the per-unit verdict
  block and line, the seat tag, and the context-only split with its disclosure.
- **Build-surface item 1 — PASS on coverage.** All four bullets landed. The six-set matches
  `validation-constitution`'s ids and order exactly; no section is empty, so no empty marker is
  owed; the floor-count pin renders as 11 rules, derived by the renderer rather than declared.
- **Nothing added that no decision pays for — PASS.** I traced all twenty-one skill rules plus the
  three other mints to a deciding line. `judgment-items-prose` is the one that is not in a numbered
  decision; build-surface item 5 names "coherence + preserved responsibilities" as the prose unit's
  criteria, and the persona-grid advisory read is already ceremony in `primitive-edits.md`.

VERDICT: FAIL

Issues requiring fix:

1. **D4, the bound's number restated inside the skill (blocking).** D4 rules that the number lives
   in one place and that skills never restate it, and the test is that it appears in
   `common.gate-loop-bound`'s text and nowhere else in the log. Two sites in
   `plugins/mochiko/migrations/0008-gate-form.yaml` break that, and both render to the seat.
   - The `validation-primitive-edit.sec.verdict` section `intent` reads "The grading floors, the
     judgment items keyed by unit, and the one re-audit the loop allows." "The one re-audit" is the
     number. Fix: reword the intent to "The grading floors, the judgment items keyed by unit, and
     the bound the loop runs under."
   - `validation-primitive-edit.gate-loop-bound` opens "cited here, never restated" and then
     restates the mechanism, ending "a further FAIL halts the landing and goes to the user". With
     the bound at one re-audit that clause is true; at any other number it is false, so it fixes the
     number as surely as a digit would, and it duplicates `second-fail-user` in the same document.
     Fix: cut the clause, leaving the rule as the citation plus the re-audit read scope the skill
     legitimately owns — "The bound, its number, and the disposition after the terminal FAIL are
     `common.gate-loop-bound`'s, stated there and never here; what this skill adds is the read
     scope: the re-audit is the same grader seat resumed, reading only what the fix touched and what
     it could have broken."

## Failure narrative

The unit fails on one ruling-fidelity item, D4's one-place loop bound. Round 1 caught the number
written out in `validation-primitive-edit.gate-loop-bound`'s text and the fix removed the digits,
but the rule still paraphrases the bound's whole mechanism, and the enclosing section's `intent`
line now carries the number in words. Both go stale the moment the number changes, which is the
single failure D4 exists to prevent, and the rule asserts "never restated" immediately before
restating. The fix is two rewordings in
`plugins/mochiko/migrations/0008-gate-form.yaml`, both additive to the log, then a re-stamp and a
re-emit. Everything else on the AM-2 five and the other eight decisions holds.

## Notes of note

I ran `views emit` before capturing the tree's pre-emit state, so I cannot prove the author's tree
already matched the replay. Two consecutive emits produced an identical `git status` and diff hash,
and the four changed paths are exactly the unit's declared deliverable.

D3's prose lists scaffold headings as mechanical, which reads against where the migration puts
them. Build-surface item 2 settles it: the split is a build-time inventory of what `migrate
validate` asserts, and the crate carries no scaffold check. The judgment side is correct.

Not blocking: six minted skill rules carry no rule-level anchor although siblings anchor the same
decisions. The grammar owes none on a mint, so this is a provenance inconsistency, not a defect.

audit: schema content (migration 0008-gate-form + view diff) · validator · opus · 12 files · 2 rounds · 1 blocking
