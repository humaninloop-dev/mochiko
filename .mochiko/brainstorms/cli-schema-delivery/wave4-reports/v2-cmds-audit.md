# V2 — command-pair audit (wave 4, `cli-schema-delivery`)

**Overall verdict: PASS.** All five pairs pass every graded item, plus the README, the report-honesty
check and the budget pre-assert. Graded against `.claude/rules/mochiko/primitive-edits.md` criteria
1–11 as amended, `wave4-plan.md` §3 and §6, and the wave-3 referent `commands/brainstorm.md` with
`wave3-reports/v2-plugin-audit.md` as the bar — on the files, never on P2's report. Three findings;
none blocks the landing. Binary: `mochiko-cli 0.1.0 · grammar 1..1`, built here. HEAD is `9732de0`.

- `architecture` / `arch` — **PASS** (items 1–8, all)
- `feature` / `feat` — **PASS** (items 1–8, all)
- `implement` / `impl` — **PASS** (items 1–8, all; the three ruled Entry rewordings verified)
- `setup` / `setup` — **PASS** (items 1–8, all)
- `specify` / `spec` — **PASS** (items 1–8, all; see F2, an observation, not a defect)

**P1's unit, confirmed and otherwise ignored.** `git diff --numstat -- plugins/mochiko/schemas/`
returns `1 1` on each of six files, and `-U0` shows every one is the `fail-conditions` section
`intent`, `hard-codes this set's count` becoming `cites the count this render prints`. No other
schema line moved. `git diff --name-only -- plugins/` lists only the five commands, P1's
`migrations/README.md` and those six schemas — `hooks/`, `brainstorm.md`, `common.yaml`,
`command-labels.yaml`, templates and skills byte-untouched, and the provenance sidecar unchanged.

## 1. Scaffold (criterion 1 as amended) — PASS, all five

Parsed each frontmatter block: exactly four keys on every file — `description` · `argument-hint` ·
`disable-model-invocation` · `allowed-tools` — nothing else added or changed, the diffs showing a
single `+allowed-tools: Bash(mochiko-cli *)` line each. Headings, in file order on all five:
`# <Name> — <epithet>` · `## Identity & Mission` · `## Rules — delivered by mochiko-cli` ·
`## Adaptive Goal Protocol`. No extra top-level section; Entry → Goal → Not-done last.

## 2. Byte preservation — PASS, all five

Extracted the title-through-`## Identity & Mission` region and the Entry-plus-Goal region from each
worktree file and from `git show HEAD:`, compared as bytes. Title→Identity identical everywhere at
970 · 1177 · 1166 · 684 · 557 bytes; Entry+Goal identical on four at 1201 · 1202 · 1173 · 1901 bytes
(`specify` among them, so its template two-arm sentence stands as ruled), protocol lead-in on all five.

`implement`'s Entry (2144→2197 bytes) differs in the three ruled citations and nothing else: a
token-level `difflib` diff over the region returns exactly four opcodes, two ``implement.yaml`)`` →
``impl.sec.tools`, delivered above)``, one ``implement.yaml`'s `vars:`` → ``the `vars``, and the
insertion `of the preamble delivered above`. Every obligation word is otherwise untouched, the
run-open confirmation included. Each new citation names a home carrying the content, checked by
rendering, not by reading the schema: `impl.sec.tools` carries `impl.sufficiency-binding-verdict`,
`impl.sufficiency-disputed-clause`, `impl.sufficiency-report` and `impl.absent-surfaces`, all live;
the preamble's `vars` block prints `attempt_bound_cycle = 3` and `gap_rework_bound = 2`.

## 3. Halt paragraph and the seven `!` lines — PASS, all five

Took the referent's Rules section, substituted the command name for `brainstorm`, compared the
paragraph as bytes: **identical on all five**, so line breaks were kept rather than re-flowed, as P2
disclosed. Each section holds the heading, that ten-line paragraph and seven `!` lines; a scripted
check for any other non-blank line returns zero on every file. The seven are `preamble` plus the six
`<p>.sec.*` ids, each carrying `--plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`, ordered as the preamble's
`sections` block and the schema's `- id:` lines: roles · reserved · tools · ways-of-working ·
boundaries · fail-conditions. `setup` and `specify` keep `sec.harness` and `sec.bindings` tombstoned,
referenced by no `!` line and no `.md` token (criteria 2 and 4).

## 4. Step 3 — PASS, all five

Normalised whitespace and compared each step 3 to the referent's under two substitutions, the section
prefix and the `visit`/`run` word: **equal on all five**. Each cites "the `kind: fail` line under
`pins` in the preamble block", obliges "halt and surface it before closing" on an end-line
disagreement, and carries no hand-pinned count. `visit` on the two desks, `run` on the three runs,
matching the intent P1's migration prints (`feat.sec.fail-conditions` reads "fails the visit").

## 5. Delivery — PASS, 35 of 35

Extracted every `!` line from the shipped `.md` files and executed all thirty-five with
`target/release` on `PATH` and `CLAUDE_PLUGIN_ROOT=plugins/mochiko`. All exit 0. Every block opens
`mochiko-cli rules <cmd> · section <id> · binary 0.1.0 · grammar 1 · plugin 0.104.0` and closes
`mochiko-cli rules end · <cmd> · <id> · <N> rules`, both in the halt clause's exact shape. Each
preamble's `- kind: fail · N rules` pin equals its fail-conditions end-line count, at the pre-registered
1 · 1 · 15 · 6 · 9; `class: floor` pins read 22 · 13 · 34 · 18 · 16, per §0.

## 6. Strips — PASS, eleven entries

Every `[v0.105.0]` entry sits at the top of its file, above the `[v0.100.0]` block, under one
wave-context comment whose provenance pointer `git show 9732de0:` resolves — `9732de0` is HEAD. Two
entries each on `architecture`, `feature`, `setup` and `specify`; three on `implement`, the extra one
being the Entry citations. All eleven carry the supersession-by-ruling shape from
`.mochiko/strips/README.md`: Disposition, `Tier failed: n/a — supersession by ruling` with the ruling
cited, Content, Kept deliberately, Consumers assessed. I machine-compared every fenced Content block
against `git show HEAD:` — **eleven of eleven reproduce the pre-edit text verbatim.**

## 7. Criterion 9, the D13 checker — PASS, all five

Run per pair as `uv run --with pyyaml python3 scripts/check-command-schema.py --schema
plugins/mochiko/schemas/<cmd>.yaml --md plugins/mochiko/commands/<cmd>.md`. Each returns exactly two
findings, both of the conversion-expected class — `no Not-done line hard-coding the kind: fail count`
and ``canonical heading `## Rules — load the schema first` absent (scaffold D2)``. I confirmed the
delta rather than assuming it, running the same checker against each pre-edit `.md` from
`git show HEAD:`: `0 findings` on all five, warnings 10 · 8 · 14 · 5 · 8 — identical before and
after, and identical to P2's stated baselines.

## 8. Criterion 8 and protected content — PASS

Extracted every `id:` value from each schema at HEAD and now — 53 · 55 · 111 · 48 · 59 — with **none
vanished and none added**, so no tombstone was owed. Criterion 6's substance and criterion 7's
done-condition branches ride on schema text and the byte-identical Goal steps, both preserved; the
three DM-chartered commands still carry `arch.sound-loop-floor`, `feat.sound-loop-floor` and
`impl.sound-loop-floor`. Criterion 10 holds: `.mochiko/provenance.yaml` is unchanged. Criterion 11's
common-file co-Read is discharged by the render, whose legend reads "extends: is already resolved".

## README, honesty, budget

**README — PASS.** Both sentences reworded, four lines changed and nothing else. `grep -ic converted
README.md` returns **0**, so no survivor of "a converted command". No false install promise: the only
`cargo install` lines are the git-install line at 25 and the maintainer `--path` line at 52, never a
bare `cargo install mochiko-cli`. Wave 3's F6 is moot: the "Kernel-free" tagline is already gone.

**Report honesty — PASS.** `git diff --numstat` gives +24/−30, +24/−30, +32/−33, +25/−33, +24/−33 on
the commands and +81, +81, +113, +83, +88 on the strips, matching `p2-cmds.md`'s table, as do the
byte figures, the pins, the warning baselines and the disjoint file set. One cosmetic slip: it says
"Line 3 already named all six commands", where that sentence is line 5; the claim itself is true.

**Char-budget pre-assert — not applicable, verified.** `.mochiko/memory/primitive-cost-budgets.md`
records under "Unbudgeted primitives" that "all **commands** have no" per-primitive budget, and at
line 417 that the class was excluded by user ruling in both prior waves.

## Findings

1. **F1 — the D13 checker now fails advisory on all six commands (minor, lead item).** Both findings
   are permanent for a converted command: the checker still wants the old heading and a hard-coded
   count, neither of which any command now carries. Every shipped pair reports
   `2 findings — FAIL (advisory)` from here on, costing criterion 9 its value as a regression detector
   — a real new finding is a third line in a block that already fails. Worth a converted-command
   branch, or a ruling that the two are the expected floor. Not P2's to fix.
2. **F2 — `specify` Goal names a schema raw-Read while its own halt paragraph forbids one
   (observation).** Line 48 offers `plugins/mochiko/schemas/spec.yaml` Read raw when the binary is
   absent, under a paragraph reading "Never Read a schema file instead; there is no fallback". No
   obligation conflicts: `spec.yaml` is the template artifact schema, a different file from the rules
   schema `specify.yaml`, its own header declaring that degraded path first-class under D8, while the
   halt clause scopes to the rules render. P2 flagged it at plan time and the lead ruled the sentence
   byte-identical; recorded so the tension reaches wave 5 or 6 rather than going missing.
3. **F3 — two clauses convert without a successor (carried, not new).** The `labels:` pointer at
   `command-labels.yaml` and "the `moments:` block … is unordered, never a sequence" have no home in
   the preamble legend. P2 booked both as losses in all five Rules-block strip entries and routed them
   to P1 as legend lines, the right disposition; wave 3's F1 class, wave-wide, outside P2's unit.

**Fix list:** none blocking. F1 alone has a code consequence, and it belongs to the checker.
