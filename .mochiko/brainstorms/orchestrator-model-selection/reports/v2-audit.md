# V2 audit — P2's persona frontmatter, strips, and ARCHITECTURE.md

**Verdict: PASS.** Five blocking nits: none. Five advisory notes: N1–N5, none blocking.

**Seat:** V2 (`mochiko:validator`) · author of none of the graded material · default FAIL
**Graded:** the nine files in P2's ownership, read directly; `reports/p2-report.md` used only as the claim list
**Worktree:** `seat-default-key` (branch `worktree-seat-default-key`, off `main` at v0.108.0)
**Ruling:** `record.md` D2, D3, D8 · `wave1-build.md` §P2 · lead's approved adjustments (option-B agents row; ownership widened to `ARCHITECTURE.md` 59/61/62; post-hoc acceptance of the §3c repairs)
**Ceremony:** `.claude/rules/mochiko/primitive-edits.md` (plain primitive form: char-budget pre-assert, internal coherence, preserved responsibilities, protected-content rule) · `.mochiko/strips/README.md`
**Register:** ultra

## Verdicts

| # | Item | Verdict |
|---|---|---|
| 1 | Persona files — sole change is the frontmatter alias | **PASS** |
| 2 | Four strip entries — shape and claim truth | **PASS** |
| 3 | `ARCHITECTURE.md` — legacy branch, truth, no new dead pointer, option-B wording | **PASS** |
| 4 | `plugin.json` untouched; nothing else of P2's under `plugins/mochiko/` | **PASS** |
| 5 | Scope — no file outside P2's nine | **PASS** |

## 1. Persona files — PASS

`git diff -U0 plugins/mochiko/agents/` returns exactly four hunks, one line each:

```
product-engineer.md    @@ -9 +9 @@   -model: opus  +model: sonnet
product-manager.md     @@ -10 +10 @@  -model: opus  +model: sonnet
requirements-analyst.md @@ -8 +8 @@   -model: opus  +model: sonnet
technical-analyst.md   @@ -9 +9 @@   -model: opus  +model: sonnet
```

`git diff --numstat` is `1 1` on each of the four and lists no fifth file. Because the numstat is
`1 1` per file, `description:`, `name:`, `color:`, `skills:` and the entire body are byte-identical
to `HEAD` by construction — no separate byte comparison is needed and none disagrees.

- **`description:` byte-identical** — no hunk touches it. Char-budget pre-assert therefore holds
  unchanged against `.mochiko/memory/primitive-cost-budgets.md:307–313`
  (requirements-analyst 303/379 · product-manager 438/548 · product-engineer 392/490 ·
  technical-analyst 402/503). `model:` is not a budgeted class. **No overage to argue.**
- **`skills:` untouched** — `requirements-analyst.md:10`, `technical-analyst.md:11`,
  `product-manager.md:12`, `product-engineer.md:11`, all outside the hunks.
- **`## Delegating Cheap Reads` untouched** — present at `requirements-analyst.md:83`,
  `product-engineer.md:76`, `product-manager.md:107`, `technical-analyst.md:119`, all outside the
  hunks. D7 item 2 satisfied.
- **Six strong personas untouched** — `git status --short` lists four agent files, no more.
- **Post-state matches D3 row for row.** `grep -n "^model:" plugins/mochiko/agents/*.md`:
  `opus` on devils-advocate:5 · principal-architect:15 · staff-engineer:8 · tech-lead:14 ·
  qa-engineer:7 · validator:7 — six. `sonnet` on requirements-analyst:8 · technical-analyst:9 ·
  product-manager:10 · product-engineer:9 — four. Exactly D3's table.
- **D8 (alias, never a full id)** — both values are family aliases. No `claude-*-5` string in any
  persona frontmatter.
- **Internal coherence.** `grep -niE "opus|sonnet|haiku|strong tier"` over the four returns only
  the frontmatter line and the `model: haiku` reads-rung sentence inside
  `## Delegating Cheap Reads` (`requirements-analyst.md:87`, `product-engineer.md:80`,
  `product-manager.md:111`, `technical-analyst.md:123`). No body text asserts the seat's own tier,
  so nothing in the four files now contradicts `sonnet`. The haiku reads rung is a different key
  and stays legal under D5 (Haiku is never a *seat*).
- **Preserved responsibilities.** Nothing removed. No protected content leaves; the `description:`
  values (v0.63.0 protected prose) and every `KEPT:`-class line survive intact.

## 2. Strip entries — PASS

One `[v0.110.0]` supersession-by-ruling entry per file, appended newest-first:

| File | Inserted at | Entry it precedes | Newest-first holds |
|---|---|---|---|
| `.mochiko/strips/requirements-analyst.md` | line 5 | `[v0.86.0]` | yes |
| `.mochiko/strips/technical-analyst.md` | line 12, below the frozen lineage comment | `[v0.91.0]` | yes |
| `.mochiko/strips/product-manager.md` | line 5 | `[v0.78.0]` | yes |
| `.mochiko/strips/product-engineer.md` | line 5 | `[v0.78.0]` | yes |

### Shape against `.mochiko/strips/README.md`

All five supersession fields present on all four, in the README's order:
`## [vX.Y.Z] <one-line description>` · `**Disposition:** superseded → …` ·
`**Tier failed:** n/a — supersession by ruling (<record + decision ID>)` · `**Content:**` ·
`**Kept deliberately:**` · `**Consumers assessed:**`. No tier number is asserted anywhere — the
README's category-error trap is avoided. `Content:` carries the verbatim superseded line
`model: opus` with its line number on each.

### Claim truth against the tree

| Claim | Evidence | True |
|---|---|---|
| `Content:` line numbers 8 / 9 / 10 / 9 | match both the `-U0` hunk headers and `grep -n "^model:"` | yes |
| D3 ground quoted per seat | `record.md` D3 table rows, verbatim: "spec producer, graded by devils-advocate on `review-specifications`" · "design producer, graded by feasibility + plan-artifacts reviews" · "feature-map producer; selection is the user's ruling" · "prototype producer, graded" | yes |
| G7 cited on three, omitted on technical-analyst | `record.md` D3 fold G7 itself scopes to "three of the four `down` seats"; the excluded seat is the one whose graders emit verdicts (`review-feasibility` → feasible/needs-revision/infeasible; `review-plan-artifacts` → ready/needs-revision/critical-gaps). The asymmetry is the record's own, not an omission | yes |
| `DECISIONS.md` 2026-09-19 row exists | `DECISIONS.md:13` | yes |
| the same ruling supersedes `model-tiered-seats` D5 + fold F6 | `DECISIONS.md:170` · `.mochiko/brainstorms/model-tiered-seats/record.md:166` | yes |
| description char figure 303 against budget 379 (requirements-analyst) | `.mochiko/memory/primitive-cost-budgets.md:307` reads `303 | 379` | yes |
| `plugin.json` agents list is path-only and unchanged | `plugins/mochiko/.claude-plugin/plugin.json:18–28` — ten `./agents/*.md` strings, no model axis; `git diff` on the file is empty | yes |
| the router's agents table names no model | `plugins/mochiko/skills/mochiko/SKILL.md:139–149` — ten rows, role text and `skills:` lists only, no model token | yes |
| benchmark variant exists for requirements-analyst / product-manager / product-engineer, carrying a frozen `model: opus` | `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/requirements-analyst.md:8`, `product-manager.md:10`, `product-engineer.md:9` | yes |
| technical-analyst has **no** benchmark variant, so the clause is omitted | `ls .mochiko/benchmarks/guardrails-vs-detail/variants/agents/` returns six files: devils-advocate · principal-architect · product-engineer · product-manager · requirements-analyst · validator. No technical-analyst | yes |
| the eval kit and `ARM_MODEL` are not on this branch | `evals/` has no `agents/` directory; `evals/commands/` holds `README.md`, `brainstorm-probe`, `implement`, `run.py`, `setup`, `wrapper.md` — no `agents.py` | yes |
| migration `0006-seat-default-key` records the schema content, so no strip entry there | `plugins/mochiko/migrations/0006-seat-default-key.yaml` present (untracked, P1's unit); `.mochiko/strips/README.md` "Schema content is recorded by the migration log, not here (from v0.107.0)" | yes |

No claim inside the four entries is false against this tree.

## 3. `ARCHITECTURE.md` — PASS

### Legacy-vs-derived branch — hand-edit was legal

`.claude/rules/mochiko/operating-docs.md:20` scopes the derived-index rule: it "binds only once the
store carries ruled content: where the store carries no ruled content (scaffold-only or absent),
`ARCHITECTURE.md` stays hand-maintained legacy until the first `/mochiko:architecture` visit
reconstructs the store from it (D16)."

The store is scaffold-only. `.mochiko/product/architecture/` holds two files and nothing else:
`spine.md` is a Scope line plus its own comment — "Scaffold stub written by the /mochiko:setup AM-2
run (2026-09-04) per the unconditional store scaffold rule: header only, no topology, no ruled
content"; `concerns.md` is a title plus "Empty by scaffold". **No `AX-XXX` row exists.** The legacy
branch applies; the hand edit is legal and takes no strip entry (the file sits outside the
`plugins/mochiko/**` path set of the primitive-edits ceremony).

### Truth of every edited line

| Edited claim | Check | Result |
|---|---|---|
| "No schema file ships (v0.107.0)" (line 60) | `ls plugins/mochiko/` → agents, commands, hooks, migrations, output-styles, skills, templates. **No `schemas/`** | true |
| commands "ship as a `.md` alone", 6 of them | `ls plugins/mochiko/commands/` → six `.md`, no sibling YAML | true |
| skills count 38, "the other 37" | `ls plugins/mochiko/skills \| wc -l` → 38 | true |
| "the thirty schema-bearing skills … the seven prose skills and the router carry no rule set" | `grep -rl "## Rules — delivered by mochiko-cli" plugins/mochiko/skills/ \| wc -l` → 30. 30 + 7 + 1 = 38 | true |
| templates "7 + `constitution-modules/`" | `ls plugins/mochiko/templates/` → seven `.md` plus `constitution-modules` | true |
| rules "live in the migration log at `plugins/mochiko/migrations/` … rendered at fire by `mochiko-cli`" | directory exists; CLAUDE.md GI-020 and `.claude/rules/mochiko/primitive-edits.md` state the same delivery | true |
| agents row "10 … six `model: opus`, four `model: sonnet`" | `ls plugins/mochiko/agents/ \| wc -l` → 10; the `grep` in §1 gives 6/4 | true |

### No new dead pointer

`grep -niE "schema\.yaml|schemas/|load the schema|Read raw" ARCHITECTURE.md` returns nothing
(exit 1). Both new markdown links resolve: `plugins/mochiko/migrations/` exists. The mermaid
diagram is coherent — the node id `schemas` was renamed to `migrations` at line 29 and both new
edges (lines 33, 34) target `migrations`; no dangling id survives. The two remaining uses of the
word "schema" (lines 100, 308) are conceptual — "the six sections every schema carries" — and name
no path; the six-section set is still the live grammar per `.claude/rules/mochiko/primitive-edits.md`
criterion 2.

### Option-B wording on the agents row

`ARCHITECTURE.md:61` is **byte-identical** to the "After" block P2 quotes as the lead-pinned option B
(`reports/p2-report.md:110`) — compared programmatically, `a == r` is `True`:

```
| **Agents** | [`plugins/mochiko/agents/`](plugins/mochiko/agents/) | 10 | Personas that carry judgment and declare `skills:`, each pinning a ruled default tier — six `model: opus`, four `model: sonnet` (seat default key, 2026-09-19). A persona contains no trace of any workflow — decoupling by absence; caller-side context rides the dispatch brief. |
```

See N4: the lead's own text is not on disk anywhere but inside P2's report, so this checks the
shipped row against P2's quote of it, not against an independent copy.

### Scope within the file

`git diff ARCHITECTURE.md` shows three hunks and nothing else: the diagram (lines 29, 33–34), the
layer table plus the manifest paragraph (lines 60–68), and the command-form section (lines 96–101).
Those are the three assigned lines (59/61/62 by the pre-edit numbering, shifted by the diagram's
net +1) plus the three §3c repairs the lead accepted post-hoc. **Each §3c repair is independently
true and removes a genuine contradiction**: had they been skipped, the table would have asserted
that no schema file ships while the diagram and two paragraphs still drew `schemas/` as a live
plugin layer. Nothing else in the file changed.

## 4. `plugin.json` and the rest of `plugins/mochiko/` — PASS

`git diff plugins/mochiko/.claude-plugin/plugin.json` is empty. The file still reads
`"version": "0.108.0"` and the ten-entry `agents` array is unchanged.

`git diff --numstat plugins/` returns six rows: P2's four agent files at `1 1` each, plus
`skills/mochiko/SKILL.md` (`1 1`) and `skills/patterns-model-tiering/SKILL.md` (`12 9`) — both P1's
unit, not graded here. **Nothing of P2's under `plugins/mochiko/` beyond the four frontmatter
lines.**

## 5. Scope — PASS

`git status --short` accounts for every change, each to a named owner:

| Owner | Files |
|---|---|
| **P2 (9)** | `plugins/mochiko/agents/{requirements-analyst,technical-analyst,product-manager,product-engineer}.md` · `.mochiko/strips/{requirements-analyst,technical-analyst,product-manager,product-engineer}.md` · `ARCHITECTURE.md` |
| P1 (7) | `plugins/mochiko/migrations/0006-seat-default-key.yaml` (untracked) · `.mochiko/schema-views/skills/patterns-model-tiering.yaml` · `evals/contract/expected-skills.json` · `plugins/mochiko/skills/patterns-model-tiering/SKILL.md` · `plugins/mochiko/skills/mochiko/SKILL.md` · `.mochiko/strips/patterns-model-tiering.md` · `.mochiko/strips/mochiko.md` |
| Lead | `.mochiko/brainstorms/index.md` · `.mochiko/brainstorms/model-tiered-seats/record.md` · `BACKLOG.md` · `DECISIONS.md` · `ROADMAP.md` · `.mochiko/brainstorms/orchestrator-model-selection/` (untracked; `reports/p2-report.md` inside it is P2's own report, expected) |

**No stray change.** `CHANGELOG.md`, `.claude-plugin/marketplace.json`, `crates/**` and
`.mochiko/archive/**` are all untouched.

## Fix list

**None blocking.** Nothing is required for this unit to land.

## Advisory notes

- **N1 (Minor, no fix required).** All four strips cite `ARCHITECTURE.md:60` in
  `Consumers assessed:`, quoting the pre-edit text "Personas (all `model: opus`)". After this
  wave's diagram edit that row is line 61 and no longer carries that phrase. The citation is a
  faithful quote of the state the ruling corrected, and `.mochiko/strips/**` is outside the
  governed dead-pointer scan, which `.mochiko/memory/knowledge-management.md:62` scopes to
  `ROADMAP.md` / `DECISIONS.md` / `BACKLOG.md`. If the lead wants it exact, the minimal edit in all
  four files is:
  `ARCHITECTURE.md:60` → `ARCHITECTURE.md` agents row (line 61 after this wave)
- **N2 (lead dependency).** The four stamps read `[v0.110.0]` while `plugin.json` still reads
  `0.108.0`. Correct per `wave1-build.md` (the lead bumps). If the bump lands at any version other
  than 0.110.0, all four stamps must move with it.
- **N3 (pre-existing, not introduced by P2).** The manifest paragraph (`ARCHITECTURE.md:65–68`)
  names `templates/` and `migrations/` as shipped-but-unregistered, and is silent on
  `plugins/mochiko/hooks/` and `plugins/mochiko/output-styles/`, which also ship. The pre-edit text
  was silent on both too, and neither sits on an assigned line. Not a regression; a candidate for a
  later `ARCHITECTURE.md` touch.
- **N4 (unverifiable from the tree).** The lead's option-B wording exists on disk only as P2's own
  quote at `reports/p2-report.md:110`. The shipped row is byte-identical to that quote, so the
  producer is internally consistent, but an independent copy of the lead's text does not exist for
  me to check against. **The lead should confirm the quote is their text.** This does not block.
- **N5 (lead-owned, already flagged by P2 §3d).** `ARCHITECTURE.md:3` still stamps the file
  "(v0.91.0)" against `main` at v0.108.0. Outside P2's lines; it moves with the bump.

## Two items P2 asked to have ruled

- **§3c — the three repairs beyond the assigned lines.** Graded and **sound**. Each removes a
  contradiction the assigned edits would otherwise have created, each is true against the tree, and
  each introduces no dead pointer. The lead's post-hoc acceptance is well founded.
- **The technical-analyst asymmetry.** **Correct, not an omission.** `record.md` D3 fold G7 scopes
  itself to "three of the four `down` seats", and the seat it excludes is the one whose graders emit
  clearing verdicts. The benchmark-variant clause is likewise absent because no variant file exists
  for that persona.
