# P2 — plugin side, wave 4: five command re-points, eleven strip entries, README wording

**Done.** All five commands fire from the migration log with no schema file read; eleven
`[v0.105.0]` strip entries recorded; the README's two halting sentences now read for all six.
Nothing committed. Plan: `p2-cmds-plan.md`, approved with four rulings folded in below.

Evidence taken on the host, `target/release/mochiko-cli` 0.1.0 · grammar 1 · plugin 0.104.0,
2026-09-04, and **re-taken after P1's migration `0002` landed** — the reworded `fail-conditions`
intents are in the renders below, so the numbers are against the log as it now stands.

## Per command

Each `.md` took the same three changes: the frontmatter gained `allowed-tools: Bash(mochiko-cli *)`
as a fourth key; `## Rules — load the schema first` was replaced whole by
`## Rules — delivered by mochiko-cli` (the wave-3 halt paragraph with the command name substituted
at its two occurrences, then seven `!` lines, then nothing); and step 3 took the wave-3 form,
citing the CLI-printed pin instead of a number. `implement` additionally took the three Entry
rewordings ruled at approval.

| command | `.md` diff | `!` lines | preamble pin | fail-conditions end line | agree | D13 checker |
|---|---|---|---|---|---|---|
| architecture | +24 −30 | 7 | `kind: fail · 1 rules` | `· 1 rules` | yes | 2 findings, 10 warnings |
| feature | +24 −30 | 7 | `kind: fail · 1 rules` | `· 1 rules` | yes | 2 findings, 8 warnings |
| implement | +32 −33 | 7 | `kind: fail · 15 rules` | `· 15 rules` | yes | 2 findings, 14 warnings |
| setup | +25 −33 | 7 | `kind: fail · 6 rules` | `· 6 rules` | yes | 2 findings, 5 warnings |
| specify | +24 −33 | 7 | `kind: fail · 9 rules` | `· 9 rules` | yes | 2 findings, 8 warnings |

**Section ids.** Every command's seven `!` lines are `preamble` plus the six live sections in the
preamble's own `sections` order — `<p>.sec.roles` · `reserved` · `tools` · `ways-of-working` ·
`boundaries` · `fail-conditions` — with `<p>` = `arch` · `feat` · `impl` · `setup` · `spec`. Each
line carries `--plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`.

**Render check — 35 of 35 pass.** Every `!` line was extracted from the shipped `.md` and executed
with `target/release` on `PATH`: all exit 0, all open with
`mochiko-cli rules <cmd> · section <id> · binary 0.1.0 · grammar 1 · plugin 0.104.0` and close with
`mochiko-cli rules end · <cmd> · <id> · <N> rules`, both in the halt clause's exact shape. The
`visit`/`run` word in step 3 matches the section intent P1's migration now prints: the desks read
"fails the visit", the runs "fails the run".

**Byte preservation.** Against `git show HEAD:`, the title line through `## Identity & Mission` is
byte-identical on all five (970 · 1177 · 1166 · 684 · 557 bytes), and Entry plus Goal is
byte-identical on four (1201 · 1202 · 1173 · 1901 bytes). `implement`'s Entry differs only in the
three ruled citations and the re-wrap they force; the `## Adaptive Goal Protocol` heading and its
lead-in line are untouched everywhere. No frontmatter key other than the added `allowed-tools` was
changed on any file.

**D13 checker**, per pair, `uv run --with pyyaml python3 scripts/check-command-schema.py --schema
plugins/mochiko/schemas/<cmd>.yaml --md plugins/mochiko/commands/<cmd>.md`. Each pair returns the
same two findings and its pre-edit warning count, unchanged:

```
FINDING: <cmd>.md: no Not-done line hard-coding the `kind: fail` count — want 'the N rules of `kind: fail`' (D7 C2 guard, re-keyed at build item 4)
FINDING: <cmd>.md: canonical heading `## Rules — load the schema first` absent (scaffold D2)
```

Both are the conversion-expected class, and I confirmed the delta rather than assuming it: the same
checker against each pair before the edit reported `0 findings` with warning counts 10 · 8 · 14 · 5
· 8, the same counts it reports now. These are exactly the two substitutions amended criteria 1 and
3 license.

## The eleven strip entries

All stamped `[v0.105.0]`, prepended above each file's `[v0.100.0]` block under one wave-context
comment per file, citing record D3 as amended and D9, the wave-3 Q-B ruling at `wave3-plan.md` §9,
the `DECISIONS.md` 2026-09-04 wave-3 row, and this wave's row. Pre-edit provenance on every
comment: `git show 9732de0:plugins/mochiko/commands/<cmd>.md`.

| file | entries | added |
|---|---|---|
| `.mochiko/strips/architecture.md` | the Rules block · the hand-pinned count | +81 |
| `.mochiko/strips/feature.md` | the Rules block · the hand-pinned count | +81 |
| `.mochiko/strips/implement.md` | the Rules block · **the Entry step's three `implement.yaml` citations** · the hand-pinned count | +113 |
| `.mochiko/strips/setup.md` | the Rules block · the hand-pinned count | +83 |
| `.mochiko/strips/specify.md` | the Rules block · the hand-pinned count | +88 |

Every fenced `Content:` block was machine-checked against `git show HEAD:` after the two-space list
indent is stripped: eleven of eleven reproduce the pre-edit text verbatim. Each entry carries
Disposition, `Tier failed: n/a — supersession by ruling`, verbatim Content, Kept deliberately, and
Consumers assessed. No entry has a shared consumer; the hand-pin entries name
`primitive-edits.md` criterion 3, already branched on a converted command at governance v3.0.1.

Two clauses are booked as losses in every Rules-block entry rather than passed over: the `labels:`
pointer at `command-labels.yaml`, and the "the `moments:` block … is unordered, never a sequence"
guard. The preamble's `legend` covers neither, so the conversion leaves them unsuccessored. They
are wave-wide, not per command, and went to P1 as legend lines on the lead's ruling.

## README

Two sentences, four lines changed, nothing else:

- Install step 2 — "a converted command's rules are rendered at fire by the binary, and a converted
  command **halts** when it is missing" becomes "every command's rules are rendered at fire by the
  binary, and a command **halts** when it is missing".
- Under "What `mochiko-cli` serves" — "A converted command proceeds only when both lines arrive in
  that exact shape" becomes "A command proceeds only when both lines arrive in that exact shape".

No survivor of "converted" remains in the file. Line 3 already named all six commands and was not
touched.

## The four named items, as ruled

1. **`implement` Entry — reworded, not left.** The sufficiency-check and absent-surface citations
   now read `(rules: `impl.sec.tools`, delivered above)`, and the attempt-bound defaults read
   "defaults carried by the `vars` block of the preamble delivered above". I checked the render
   before choosing the home: `impl.sec.tools` carries `impl.sufficiency-binding-verdict`,
   `impl.sufficiency-disputed-clause`, `impl.sufficiency-report` and `impl.absent-surfaces`, all
   live. The bounds themselves are `attempt_bound_cycle` and `gap_rework_bound` in the preamble's
   `vars` block. Both paragraphs keep every obligation word for word apart from the citations and
   the re-wrap; the eleventh strip entry records it against D3 as amended.
2. **`specify` Goal — byte-identical.** The template two-arm sentence stands untouched, and the
   `specify` Rules-block entry's *Kept deliberately* field records why: it is the template surface,
   not this command's rules, and belongs to the wave-6 migration.
3. **Legend gaps — P1's, not mine.** Recorded as losses in all five Rules-block entries so the
   trail carries them; no `.md` edit made.
4. **Section glosses — no action.** The render's section title and intent line replace them.

## Deviations

- **Two README edits, not one.** Flagged at plan time and accepted at approval. Section 3 of the
  wave plan names the Install sentence; the second halting sentence would otherwise still have read
  "a converted command" after all six converted.
- **Halt-paragraph line breaks kept, not re-flowed.** Accepted at approval. `architecture` is the
  only name longer than `brainstorm` and pushes two lines to 97 characters, inside the paragraph's
  own 99-character maximum.
- **Evidence re-taken mid-run.** P1's migration `0002` and its six snapshot lines landed while I was
  writing strips. Every render figure and checker line above is from the re-run against the current
  log, not the earlier pass. The counts did not move; the `fail-conditions` intent line did, and now
  reads "the .md Not-done line cites the count this render prints", which is what these five `.md`
  files do.
- **Nothing else touched.** My file set stayed disjoint from P1's and P3's: five commands, five
  strip files, `README.md`, and this report directory. No git mutation, no commit.
