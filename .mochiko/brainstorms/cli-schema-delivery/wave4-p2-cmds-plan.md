# P2 — plan: five command re-points, ten strip entries, README wording

Referent: `commands/brainstorm.md` at HEAD plus its two `[v0.104.0]` strip entries; bar `wave3-reports/v2-plugin-audit.md`. Figures from `mochiko-cli` 0.1.0 · grammar 1 · plugin 0.104.0.

| command | prefix `<p>` | `kind: fail` pin | `class: floor` | step-3 word | D13 baseline |
|---|---|---|---|---|---|
| architecture | `arch` | 1 | 22 | visit | 0 findings, 10 warnings |
| feature | `feat` | 1 | 13 | visit | 0 findings, 8 warnings |
| implement | `impl` | 15 | 34 | run | 0 findings, 14 warnings |
| setup | `setup` | 6 | 18 | run | 0 findings, 5 warnings |
| specify | `spec` | 9 | 16 | run | 0 findings, 8 warnings |

## 1. The uniform edit

- **Frontmatter** gains `allowed-tools: Bash(mochiko-cli *)` as a fourth key after
  `disable-model-invocation: true`; nothing else in it changes.
- **The Rules section** is replaced whole: heading `## Rules — delivered by mochiko-cli`, the wave-3 halt paragraph
  with `brainstorm` substituted at both occurrences, the seven `!` lines, nothing else. Line breaks stay at the
  referent's word positions rather than re-flowing — `architecture`, the longest name, pushes two lines from 95 to
  97 characters, inside the paragraph's existing 99-character maximum — so V2's diff stays a two-token substitution.
- **The seven `!` lines** run in the preamble's `sections` order, each ``!`mochiko-cli rules <cmd> --section
  <id> --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1` ``: `preamble` · `<p>.sec.roles` · `<p>.sec.reserved` ·
  `<p>.sec.tools` · `<p>.sec.ways-of-working` · `<p>.sec.boundaries` · `<p>.sec.fail-conditions`. All five
  preambles list exactly those six live sections in that order.
- **Step 3** takes the plan §3 wrap verbatim, `<p>` and `<visit|run>` substituted per the table.
- **Byte-identical:** the three existing frontmatter keys, `# <Name> — <epithet>`, `## Identity & Mission`, the
  `## Adaptive Goal Protocol` heading and its lead-in, steps 1 and 2 — checked by a `git diff` touching nothing else.

## 2. Command-specific content the generic shape does not carry

1. **Section glosses** — the render replaces each block's six glosses with the section's own title and intent line, at least as informative. No action.
2. **The `labels:` gloss** and **"the `moments:` block … is unordered, never a sequence"** have no successor:
   the render prints bare label names, and the legend covers neither moments nor an empty `enforces:` list.
   Inherited from wave 3, same class as its F1 — the fix is a legend line in a future migration, P1's surface.
3. **`implement` Entry cites `implement.yaml` three times** (sufficiency rules, absent-surface rules, the attempt-
   bound `vars:` block). Entry stays byte-identical, so they stay, but now point at a file the run is told never to
   read; values still arrive, since the render substitutes every `${var}` and prints `vars`. Leave and record —
   changing them needs a ruling and an eleventh strip entry.
4. **`specify` Goal offers a schema fallback** — "`mochiko-cli template spec`, or its schema
   `plugins/mochiko/schemas/spec.yaml` Read raw when the binary is absent". That is the template path, not the
   rules path, so GI-020 stands, but it sits three sections under a halt paragraph saying there is no fallback.
   Byte-identical this wave; flagged for wave 5.

## 3. Strips and README

Two `[v0.105.0]` supersession-by-ruling entries per `.mochiko/strips/<cmd>.md`, prepended above the `[v0.100.0]`
block under one wave-context comment per file, both citing record D3 as amended and D9, the wave-3 Q-B ruling at
`wave3-plan.md` §9, and the `DECISIONS.md` 2026-09-04 wave-3 row plus this wave's.

- **"the Rules block — raw schema Read superseded by CLI delivery"** — content verbatim from `git show HEAD:`;
  Kept deliberately names Identity & Mission and steps 1–2; no shared consumers, the block was its own text.
- **"the hand-pinned `kind: fail` count in Not-done"** — the superseded number is the table's pin column; Kept
  deliberately names the out-of-sync halt, re-keyed from the pair to the delivery; Consumers assessed:
  `primitive-edits.md` criterion 3, already branched at v3.0.1.
- **README** — two halting sentences re-worded so both read for all six: the Install step-2 sentence ("a
  converted command's rules are rendered at fire … **halts** when it is missing") and the line under "What
  `mochiko-cli` serves" ("A converted command proceeds only when both lines arrive in that exact shape"). The
  brief names the first; I read "reads for all six" as covering the second, and flag it rather than assume.

## 4. Check list before I report

1. **35 renders** — with `target/release` on `PATH`, all seven `!` commands per command exit 0 and carry both
   the version-triple head line and the `rules end · <cmd> · <id> · <N> rules` line.
2. **Pin versus end line** — each preamble's `kind: fail` pin equals its fail-conditions end-line count: 1 · 1 · 15 · 6 · 9.
3. **D13 checker per pair** — `uv run --with pyyaml python3 scripts/check-command-schema.py --schema
   plugins/mochiko/schemas/<cmd>.yaml --md plugins/mochiko/commands/<cmd>.md`, cited in my report. Against the
   table's baselines: exactly two findings per pair, both conversion-expected (the absent hard-coded Not-done
   count, the absent `## Rules — load the schema first` heading), warnings unchanged.
4. **Byte preservation** per §1, and `git status --short -- plugins/` showing only the five `.md`s.
