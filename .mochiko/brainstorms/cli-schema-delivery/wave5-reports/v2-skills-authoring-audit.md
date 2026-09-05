# V2 audit — wave 5, authoring family (8 pairs)

**Overall verdict: PASS.** Graded against `.claude/rules/mochiko/primitive-edits.md` as it now
stands, from the files themselves; P2's report was read only as a claim to be checked. Pre-edit
reference: `7d098b9`. Binary built from this worktree, 0.1.0, grammar 1, plugin 0.105.0.

## Per-pair verdicts

- `authoring-architecture-store` — **PASS**
- `authoring-constitution` — **PASS**
- `authoring-epic` — **PASS**
- `authoring-feature-map` — **PASS**
- `authoring-prototype` — **PASS**
- `authoring-requirements` — **PASS** (one disclosed deviation, below)
- `authoring-technical-requirements` — **PASS**
- `authoring-user-stories` — **PASS**

## 1. Frontmatter and diff scope

Scripted per pair against `git show 7d098b9:<path>`: the parsed frontmatter delta is the single
added line `allowed-tools: Bash(mochiko-cli *)` for all eight, with nothing else inserted, removed,
or reordered, so `name` and `description` are byte-identical and criterion 7 holds unchanged.
Excising the block from `## Rules` to the next `## ` heading and diffing the remainder, all eight
bodies are byte-identical outside the Rules section. Each file carries exactly one `## Rules`
heading before and after.

## 2. The Rules section

All eight heads read exactly `## Rules — delivered by mochiko-cli`. The halt paragraph was
compared machine-wise against the plan's blockquote with `<skill>` substituted and whitespace
normalised: identical for all eight. Seven `!` lines each, correctly prefixed and suffixed. The
read-back sentence is verbatim per plan §3 in all eight.

The arguments are `preamble` plus the **authoring six-set** — `independence · scope · inputs ·
artifact · output · reserved`, with `artifact` in the review family's `verdict` slot — and each
skill's order equals its preamble render's `sections` list, which equals its schema's declared
order. Each of the eight bodies holds exactly six `.sec.` tokens, all on `!` lines, so no dangling
reference survives, and no residual mention of `schema.yaml`, `skill-authoring-common`, or "load
the schema" remains anywhere. Only `authoring-requirements` carries a block after the read-back.

## 3. Delivery

All 56 `!` commands were run by hand with `target/release` on `PATH`. Every one of the 56 blocks
opens with its version-triple head line and closes with its matching end line in the exact shape;
zero failures, zero empty blocks. Each preamble's `- class: floor · N rules` pin equals the count
of ids on its `floors:` line, and the sequence is exactly **9 · 12 · 10 · 16 · 4 · 4 · 8 · 4** in
the brief's order.

## 4. The two member items the lead flagged

**`authoring-requirements`'s script-pointer sentence survives**, as its own block after the
read-back: "A `pointer:` here may bind you to a script's content as well as a file's or skill's
procedure — referenced, never restated." The obligation it protects is real — the halt paragraph's
generic wording reaches only a file's or skill's procedure, and this skill's pointers reach a
script — so keeping it is right. It is a rewrite, not a byte-identical carry of the old "A
`pointer:` rule binds you to that file's or script's content, referenced never restated"; the old
text is verbatim in the strip. Same shape as `review-feasibility`'s lens sentence last family.

**The "empty by design" markers are covered, confirmed by rendering the empty sections** rather
than by reading P2's claim. `authoring-prototype`'s `inputs` and `reserved` and
`authoring-requirements`'s `independence` and `inputs` all render at 0 rules with a `note:` line
carrying the reason and its census citation, for example "Deliberately empty — no decision is
reserved at this seat; selection and story rulings live upstream of the prototype (census:
reserved 0)". The render therefore says more than the dropped marker did.

## 5. Strips

Each of the eight strip files carries exactly two `[v0.106.0]` entries as the top two in the file
(next stamp `v0.101.0`), and both entries in all eight carry every field the README demands of a
supersession-by-ruling entry. Content was machine-verified: entry one's fenced block dedents to a
string equal to the whole pre-edit Rules section from `7d098b9` for all eight, and entry two's is
a verbatim substring of that same section for all eight.

The dropped "this body carries identity and teaching only" clause is named, with its reason, in
each of the three strips whose members carried it — `authoring-feature-map`,
`authoring-prototype`, `authoring-requirements`.

## 6. Criterion 9 — the deterministic checker

`uv run scripts/check-skill-schema.py --skill <name>` per pair. Each of the eight reports
**exactly two findings**, both conversion-expected: the missing `## Rules — load the schema first`
heading and the missing hand pin. No finding of any other class appears for any pair.

Warnings were compared against the pre-edit baseline by reconstructing each pair from `7d098b9`'s
`SKILL.md` plus the current schema and re-running the checker. The counts and texts are identical
before and after — 3 · 3 · 0 · 1 · 2 · 0 · 1 · 1 in the brief's order, a mix of condition-coverage
notes and inherited-label absences on `letter-is-spirit`. The extra findings the baseline fixture
emits are artifacts of the temporary directory lacking `../../templates/`, not real.

## 7. Criterion 8 — the re-keyed payload

I recomputed all eight: body characters after the frontmatter plus the summed characters of the
seven rendered blocks, hook lines excluded. Every one reproduces P2's table exactly, and the
family total of **147,304** matches both the report and the figure the lead carries. Against the
record's F3 authoring baseline of 150,576 that is 2.2 % under, as claimed.

## 8. IDs and the family common file

`git diff 7d098b9 --stat -- plugins/mochiko/schemas/` is empty, so
`plugins/mochiko/schemas/skill-authoring-common.yaml` is byte-unchanged, and the per-skill
`schema.yaml` diff is likewise empty. No rule id and no `<skill>.sec.*` id can have vanished, and
no tombstone is owed. Every id the `.md` files reference resolves to a live node.

## 9. Report honesty

P2's authoring report survives checking. The diff-scope assertion, the floor-pin sequence, the
"six ids are the family set with `artifact` in the `verdict` slot" claim, the verbatim strip
Contents, all eight budget rows, the family total and its percentage, and the naming of the three
members that carried the body-scope clause all reproduce. Its D13 line ("32 findings across the 16
converted skills") described the library at the time it was written; the library has since moved
on as later families landed, which is not a defect in the report.

## Non-blocking observations

- **The script-pointer sentence is a rewrite, not a byte-identical carry**, and neither P2's
  report nor the strip says so explicitly. The substance is preserved and the old text is in the
  strip, so this is disclosure, not damage. It is the second instance of the pattern; the lead may
  want plan §3's "byte-identical" wording relaxed to "substance preserved, old text stripped"
  before the remaining families are graded against it.
- **The preamble `legend` still prints `kind: … fail …` and two `enforces:` lines** on skill
  renders, though skill-pair criterion 11 makes both illegal in a skill schema. Carried forward
  from the review-family audit; it is P1's surface, outside this unit, and still worth a ruling
  before the wave lands.

**Fix list: none.** Nothing blocks this family.
