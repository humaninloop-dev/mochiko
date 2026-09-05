# V2 audit — wave 5, review family (8 pairs)

**Overall verdict: PASS.** Every pair was graded against `.claude/rules/mochiko/primitive-edits.md`
as it now stands, from the files themselves; P2's report was read only as a claim to be checked.
Pre-edit reference throughout: `7d098b9`. Binary built from this worktree, 0.1.0, grammar 1,
plugin 0.105.0.

## Per-pair verdicts

- `review-brainstorm` — **PASS**
- `review-code-minimalism` — **PASS**
- `review-feasibility` — **PASS** (one disclosed deviation from the plan's literal wording, below)
- `review-governance-intent` — **PASS**
- `review-plan-artifacts` — **PASS**
- `review-specifications` — **PASS**
- `review-sufficiency` — **PASS**
- `validation-constitution` — **PASS**

## 1. Frontmatter

Scripted per pair against `git show 7d098b9:<path>`: the parsed frontmatter delta is the single
added line `allowed-tools: Bash(mochiko-cli *)` for all eight, with no other insertion, deletion,
or reordering. `name` and `description` are byte-identical in every case, so criterion 7 holds
unchanged. The grant is written unquoted, matching the shipped converted commands.

## 2. Everything outside the Rules section

Scripted: strip the block from `## Rules` to the next `## ` heading on both sides, drop the
frontmatter, diff the remainder. All eight compare byte-identical — titles, opening paragraphs, and
every procedural section untouched. Each file carries exactly one `## Rules` heading before and
after, so the excision is unambiguous.

## 3. The Rules section

All eight heads read exactly `## Rules — delivered by mochiko-cli`. The halt paragraph was
compared machine-wise against the plan's blockquote with `<skill>` substituted and whitespace
normalised: **identical for all eight**, including the skill-variant limbs ("this slot, or the
plugin's dependency hook on a Skill-tool call" and "that file's or skill's procedure").

Seven `!` lines per skill, every one prefixed `!\`mochiko-cli rules <skill> --section ` and
suffixed `--plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1\``. The arguments are `preamble` plus the six
review-family ids, and their order equals the preamble render's `sections` list, which in turn
equals the schema's declared section order, for all eight. The read-back sentence is verbatim per
plan §3 in all eight. Nothing else appears in the section except `review-feasibility`'s preserved
lens sentence.

No `.sec.` token exists anywhere in any of the eight bodies outside the `!` lines (six per file,
all on `!` lines), and no residual reference to `schema.yaml`, `skill-review-common`, or "load the
schema" survives in any of the eight.

## 4. Delivery

All 56 `!` commands were run by hand with `target/release` on `PATH`. Every one of the 56 blocks
opens with its version-triple head line and closes with its matching end line in the exact
`mochiko-cli rules end · <skill> · <id> · <N> rules` shape; zero failures, zero empty blocks.

The preamble's `- class: floor · N rules` pin equals the count of ids on its `floors:` line for
every member, and the sequence is exactly **9 · 3 · 9 · 16 · 11 · 8 · 8 · 14** in the brief's order.

## 5. Strips

Each of the eight strip files carries exactly two `[v0.106.0]` entries, and they are the top two
entries in the file (the next stamp is `v0.100.0`). Both entries in all eight carry every field
the README demands of a supersession-by-ruling entry: `Disposition: superseded`, `Tier failed:
n/a — supersession by ruling` with the ruling cited, `Content`, `Kept deliberately`, and
`Consumers assessed`. Content was machine-verified: entry one's fenced block dedents to a string
equal to the whole pre-edit Rules section from `7d098b9` for all eight, and entry two's is a
verbatim substring of that same section for all eight.

The dropped body-scope clause is named, with its reason, in the four members that carried it —
`review-plan-artifacts`, `review-specifications`, `review-sufficiency`, `validation-constitution`
— and only those four carry it in the pre-edit text. Its actual wording is "this body carries
identity and procedure only", not "identity and teaching only" as the brief paraphrased it.

## 6. Criterion 9 — the deterministic checker

`uv run scripts/check-skill-schema.py --skill <name>` (PyYAML is unavailable to bare `python3`).
Each of the eight reports **exactly two findings**, both conversion-expected: the missing
`## Rules — load the schema first` heading and the missing hand pin. No finding of any other class
appears for any pair, and a full-library sweep confirms only those two classes exist library-wide.

Warnings were compared against the pre-edit baseline by reconstructing each pair from
`7d098b9`'s `SKILL.md` plus the current schema and re-running the checker: the counts and texts
are identical before and after — 3 · 0 · 1 · 2 · 1 · 1 · 0 · 0 in the brief's order. The two extra
findings the baseline fixture emits are artifacts of the temporary directory lacking
`../../templates/` and sibling skill directories, not real.

## 7. Criterion 8 — the re-keyed payload

I recomputed all eight myself rather than the three the brief required: body characters after the
frontmatter, plus the summed characters of the seven rendered blocks, hook lines excluded. Every
one of the eight reproduces P2's table exactly, and the family total of **116,609** matches the
report. Against the record's F3 review baseline of 119,895 that is 2.7 % under, as claimed. The
budget ledger was unmodified when I began and had landed by the time I finished: it now carries
`[v0.106.0]` re-seed rows for all 30 skills, and the eight review rows match my measured figures
to the character, so the brief's "not landed yet" no longer holds.

## 8. IDs

Every `schema.yaml` under `plugins/mochiko/skills/` is byte-unchanged against `7d098b9` — the diff
is empty — so no rule id and no `<skill>.sec.*` id can have vanished. The only ids the `.md` files
reference are the six section ids per skill, each resolving to a live node. No tombstone is owed.

## 9. `primitive-edits.md`, `README.md`, report honesty

`git diff 7d098b9 --stat` on the rules file shows 25 changed lines in five hunks, all inside the
skill-pair criteria block; nothing else in the file moved. The five converted-skill clauses land
on criteria 1, 2, 3, 6, and 8, and each is **one sentence**. Their substance matches what plan §3
and the lead's ruling proposed, criterion 8 included: the quantity is body plus the seven rendered
blocks, hook lines excluded, re-seeded at conversion with no headroom, citing D10 clause 6.

`README.md` changes one sentence, extending the dependency statement from commands to commands and
skills. Nothing else in the file moved.

P2's report survives checking. Every quantitative claim in it — the diff-scope assertion, the
floor-pin sequence, the sixteen checker findings, the verbatim strip Contents, all eight budget
rows, the family total and its percentage — reproduces. The four members named as carrying the
body-scope clause are the correct four.

## Non-blocking observations

- **`review-feasibility`'s lens sentence is not byte-identical** to its pre-edit form, which plan
  §3 asked for. It could not be: the old sentence was compound, naming the schema, the family
  common file, and the lens in one obligated read. The lens read survives whole and raw, still
  ahead of any hunting, now sequenced after the read-back, and the full old sentence is verbatim
  in the strip. Faithful, but the plan's literal wording was not met and neither P2's report nor
  the strip says so.
- **Criterion 1's new clause carries a limb the plan did not propose** — "and prints the reading
  grammar as the preamble's `legend`". It is descriptive of what the render already does and adds
  no obligation, so I am not failing it, but it is past the proposed text.
- **The preamble `legend` still prints `kind: … fail …` and two `enforces:` lines** on skill
  renders, though skill-pair criterion 11 makes `kind: fail` and `enforces:` illegal in a skill
  schema. This is the render's shared legend, P1's surface and outside my unit, but it teaches a
  skill grammar that does not exist. Worth a ruling before the wave lands.
- **P2's referent headless probe is not reproduced here.** The claims that a `Read` call succeeded
  under the new grant and that the transcript carried all nine floor ids with no schema Read are
  P2's alone; the contract suite is where they get verified.

**Fix list: none.** Nothing blocks this family.
