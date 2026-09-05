# V2 audit — wave 5, patterns family (9 pairs)

**Overall verdict: PASS.** Graded against `.claude/rules/mochiko/primitive-edits.md` as it now
stands, from the files themselves; P2's report was read only as a claim to be checked. Pre-edit
reference: `7d098b9`. Binary built from this worktree, 0.1.0, grammar 1, plugin 0.105.0.
Criterion (2) is the lead's to rule, so the family figure is reported below, not graded.

## Per-pair verdicts

- `patterns-adopt-first` — **PASS**
- `patterns-architecture-shelves` — **PASS**
- `patterns-code-minimalism` — **PASS**
- `patterns-map-minimalism` — **PASS**
- `patterns-model-tiering` — **PASS**
- `patterns-plan-minimalism` — **PASS**
- `patterns-sound-loop` — **PASS**
- `patterns-transport-floor` — **PASS**
- `patterns-vertical-tdd` — **PASS**

## 1. Frontmatter and diff scope

Scripted per pair against `git show 7d098b9:<path>`: the parsed frontmatter delta is the single
added line `allowed-tools: Bash(mochiko-cli *)` for all nine, with nothing else inserted, removed,
or reordered, so `name` and `description` are byte-identical and criterion 7 holds unchanged.
Excising the block from `## Rules` to the next `## ` heading and diffing the remainder, all nine
bodies are byte-identical outside the Rules section, with exactly one `## Rules` heading before
and after.

## 2. The Rules section

All nine heads read exactly `## Rules — delivered by mochiko-cli`. The halt paragraph matches the
plan's blockquote with `<skill>` substituted, machine-compared after whitespace normalisation, for
all nine. Seven `!` lines each, correctly prefixed and suffixed. The read-back sentence is verbatim
per plan §3 in all nine, and **no member carries any block after it** — consistent with a family
that has no obligated reference read and no body-scope clause.

The arguments are `preamble` plus the **patterns six-set** — `trigger · scope · discipline ·
inputs · disclosure · reserved`, a full swap-out rather than one slot — and each skill's order
equals its preamble render's `sections` list, which equals its schema's declared order. Each body
holds exactly six `.sec.` tokens, all on `!` lines. No residual mention of `schema.yaml`, a family
common file, or "load the schema" remains anywhere.

## 3. Delivery

All 63 `!` commands were run by hand with `target/release` on `PATH`. Every one of the 63 blocks
opens with its version-triple head line and closes with its matching end line in the exact shape;
zero failures, zero empty blocks. Each preamble's `- class: floor · N rules` pin equals the count
of ids on its `floors:` line, and the sequence is exactly **7 · 5 · 3 · 3 · 4 · 2 · 6 · 11 · 5** in
the brief's order.

## 4. The inline conditions lists the lead flagged

Confirmed by rendering the preambles, not by reading P2's claim. `patterns-transport-floor`'s
preamble prints both lanes with values, resolution mode, and an explanatory note apiece — the
`messaging` line reads "the run carries cross-seat or lead-relayed messaging", the
`shared_write_surface` line "two or more seats are able to write one artifact".
`patterns-vertical-tdd`'s prints its single condition the same way, with the note "the work opens a
genuinely new end-to-end path — greenfield, or a new path through the system". The render carries
strictly more than the dropped inline sentences did, since the old text named the dimensions
without their values or resolution modes. Both strips record this coverage explicitly rather than
letting the sentences vanish unremarked.

## 5. Strips

Each of the nine strip files carries exactly two `[v0.106.0]` entries as the top two in the file
(next stamp `v0.102.0`), and both entries in all nine carry every field the README demands of a
supersession-by-ruling entry. Content was machine-verified: entry one's fenced block dedents to a
string equal to the whole pre-edit Rules section from `7d098b9` for all nine, and entry two's is a
verbatim substring of that same section for all nine.

Because the family ships no common file, no stub resolution was at stake, and the strips say so —
each `Consumers assessed` field records that nothing shared leaves.

## 6. Criterion 9 — the deterministic checker

`uv run scripts/check-skill-schema.py --skill <name>` per pair. Each of the nine reports **exactly
two findings**, both conversion-expected: the missing `## Rules — load the schema first` heading
and the missing hand pin. No finding of any other class appears for any pair.

Warnings were compared against the pre-edit baseline by reconstructing each pair from `7d098b9`'s
`SKILL.md` plus the current schema and re-running the checker. The counts and texts are identical
before and after — 1 · 0 · 0 · 1 · 0 · 0 · 1 · 2 · 1 in the brief's order, all condition-coverage
notes. The extra findings the baseline fixture emits are artifacts of the temporary directory
lacking sibling skill directories and `../../schemas/`, not real.

## 7. Criterion 8 — the re-keyed payload

I recomputed all nine: body characters after the frontmatter plus the summed characters of the
seven rendered blocks, hook lines excluded. Every one reproduces P2's table exactly, and the family
total of **112,701** matches both the report and the figure the lead carries. Against the record's
F3 patterns baseline of 95,858 that is 17.6 % over. Criterion 8 as amended is satisfied — the
budget re-seeds to the measured payload with no headroom, which is what the ledger rows do —
and the family aggregate is the lead's call under criterion (2), not a per-pair failure.

P2's explanation of the overage checks out on its own terms: this family's baseline counts body
plus own schema only, because there is no common file to amortise, while the render adds a fixed
head and end line seven times over plus the preamble's five blocks. P2 is also candid that the
lead's §0 projection of +10.4 % was measured on the render alone and did not carry the body growth,
which is the honest way to report a gap against a projection.

## 8. IDs

`git diff 7d098b9 --stat -- plugins/mochiko/skills/*/schema.yaml` is empty, so no rule id and no
`<skill>.sec.*` id can have vanished and no tombstone is owed. Every id the `.md` files reference
resolves to a live node in the matching schema and appears in the render's `sections` list.

## 9. Report honesty

P2's patterns report survives checking. The diff-scope assertion, the floor-pin sequence, the
patterns six-set claim, the verbatim strip Contents, all nine budget rows, the family total and
its percentage, and the statement that no member carried a body-scope clause or an obligated
reference read all reproduce. Its D13 line ("50 findings across the 25 converted skills") described
the library at the moment it was written; the count has since moved as the last family landed,
which is not a defect in the report.

## Non-blocking observations

- **The preamble `legend` still prints `kind: … fail …` and two `enforces:` lines** on skill
  renders, though skill-pair criterion 11 makes both illegal in a skill schema. Third family this
  has been carried in; it is P1's surface, outside this unit, and still owed a ruling before the
  wave lands.
- **This family is the clean case for the conversion shape.** With no common file, no reference
  read, and no body-scope clause anywhere, all nine Rules sections are identical to one another
  once the skill name is substituted and line wrapping normalised, which I checked rather than
  assumed. Evidence the generic shape holds with no member carve-outs at all.

**Fix list: none.** Nothing blocks this family.
