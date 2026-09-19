# P2 build report — persona frontmatter + strips

**Seat:** P2 (`mochiko:staff-engineer`) · **Wave:** `wave1-build.md` P2 items 1–4 · **Date:** 2026-09-19
**Ruling executed:** `.mochiko/brainstorms/orchestrator-model-selection/record.md` D1–D3, D7 items 1/5, D8 (`DECISIONS.md` 2026-09-19)
**Worktree:** `seat-default-key` (branch `worktree-seat-default-key`, off `main` at v0.108.0) · **Target version stamp:** v0.110.0
**Status:** complete · not self-graded — V2 grades this

## Files touched (9)

| File | Change |
|---|---|
| `plugins/mochiko/agents/requirements-analyst.md` | frontmatter line 8 |
| `plugins/mochiko/agents/technical-analyst.md` | frontmatter line 9 |
| `plugins/mochiko/agents/product-engineer.md` | frontmatter line 9 |
| `plugins/mochiko/agents/product-manager.md` | frontmatter line 10 |
| `.mochiko/strips/requirements-analyst.md` | +33, −0 |
| `.mochiko/strips/technical-analyst.md` | +33, −0 |
| `.mochiko/strips/product-engineer.md` | +33, −0 |
| `.mochiko/strips/product-manager.md` | +33, −0 |
| `ARCHITECTURE.md` | 7 locations (see §3) |

Diffstat: 9 files, 151 insertions, 16 deletions.

## 1. Persona diffs

One line per file, `model: opus` → `model: sonnet`. Verified by `diff -U0`: each persona file shows
exactly one changed line, no other hunk.

```
plugins/mochiko/agents/requirements-analyst.md @@ -8 +8 @@
plugins/mochiko/agents/technical-analyst.md    @@ -9 +9 @@
plugins/mochiko/agents/product-engineer.md     @@ -9 +9 @@
plugins/mochiko/agents/product-manager.md      @@ -10 +10 @@
-model: opus
+model: sonnet
```

Post-state across the ten personas: six `model: opus` (devils-advocate · principal-architect ·
qa-engineer · staff-engineer · tech-lead · validator), four `model: sonnet` (the four above).
Matches D3's table exactly. The six strong persona files were never opened.

`## Delegating Cheap Reads` present and unmodified in all four (requirements-analyst:83,
product-engineer:76, product-manager:107, technical-analyst:119), per D7 item 2.

## 2. Strip entries

Four supersession-by-ruling entries, one per persona, appended newest-first at the top of each
file (for `technical-analyst.md`, below its frozen lineage-note comment). Each carries the
`strips/README.md` supersession shape: `Disposition: superseded`,
`Tier failed: n/a — supersession by ruling`, verbatim `Content: model: opus` with its line number,
`Kept deliberately`, `Consumers assessed`.

Stamp: `[v0.110.0]` on all four. Ruling cited on all four:
`.mochiko/brainstorms/orchestrator-model-selection/record.md` D3 (the ten-row seat class table),
with D1/D2 as mechanism and D8 as the alias rule, plus the `DECISIONS.md` 2026-09-19 row, and the
note that the same ruling supersedes `model-tiered-seats` D5 and fold F6.

Per-entry variation:

| Persona | D3 ground quoted | G7 fold cited | Benchmark-variant clause |
|---|---|---|---|
| requirements-analyst | "spec producer, graded by devils-advocate on `review-specifications`" | yes | variant exists, untouched |
| technical-analyst | "design producer, graded by feasibility + plan-artifacts reviews" | no — its graders emit verdicts | **none** — no variant file exists |
| product-manager | "feature-map producer; selection is the user's ruling" | yes | variant exists, untouched |
| product-engineer | "prototype producer, graded" | yes | variant exists, untouched |

`Kept deliberately` on each: every other byte of the file — the `description:` value (v0.63.0
protected prose), `name:`, `color:`, the `skills:` mount, and the whole body including
`## Delegating Cheap Reads`, whose `model: haiku` sentence is the reads rung and a different key
from the seat default. D8's alias-not-full-id rule named on each.

`Consumers assessed` on each: `plugin.json` agents list (path-only, unchanged) · the router's own
agent row (names no model) · `ARCHITECTURE.md`'s agents row (corrected this wave) ·
`patterns-model-tiering` and its floor set (superseded by migration `0007-seat-default-key`, which
records schema content by construction and takes no strip entry) · the persona eval kit and the
runner constant `ARM_MODEL` (both on `primitive-evals-v2`, not on `main`; noted, not edited) · the
benchmark variant where one exists.

## 3. ARCHITECTURE.md

### Disposition: hand-edit, legacy branch

The operating-docs derived-index rule does not bind. Evidence:

- `.mochiko/product/architecture/spine.md` is a header-only scaffold stub. Its own comment:
  "Scaffold stub written by the /mochiko:setup AM-2 run (2026-09-04) per the unconditional store
  scaffold rule: header only, no topology, no ruled content."
- `.mochiko/product/architecture/concerns.md`: "Empty by scaffold". No `AX-XXX` row exists.
- `.claude/rules/mochiko/operating-docs.md`: the derived rule "binds only once the store carries
  ruled content: where the store carries no ruled content (scaffold-only or absent),
  `ARCHITECTURE.md` stays hand-maintained legacy until the first `/mochiko:architecture` visit
  reconstructs the store from it" (D16).
- `ARCHITECTURE.md`'s own header: "updated at landings that change components, boundaries, or data
  flow" — a landing-time hand edit.

No `/mochiko:architecture` visit has run. The file is legacy, hand-maintained. It sits outside the
`plugins/mochiko/**` path set of the primitive-edits ceremony, so these edits take no strip entry.

### 3a. The agents row — the ruled edit (was line 60, now 61)

Before:

```
| **Agents** | [`plugins/mochiko/agents/`](plugins/mochiko/agents/) | 10 | Personas (all `model: opus`) that carry judgment and declare `skills:`. A persona contains no trace of any workflow — decoupling by absence; caller-side context rides the dispatch brief. |
```

After (lead-pinned option B wording, verbatim):

```
| **Agents** | [`plugins/mochiko/agents/`](plugins/mochiko/agents/) | 10 | Personas that carry judgment and declare `skills:`, each pinning a ruled default tier — six `model: opus`, four `model: sonnet` (seat default key, 2026-09-19). A persona contains no trace of any workflow — decoupling by absence; caller-side context rides the dispatch brief. |
```

### 3b. Dead-pointer repairs — the three lines assigned

**Commands row (was line 59).** Before, the two dead spans:

```
Identity & Mission · the obligated first read of the command's own schema · Adaptive Goal Protocol (…). Each ships as a `.md` + `plugins/mochiko/schemas/<cmd>.yaml` pair; the rule content — roles · reserved · tools · ways-of-working · boundaries · fail-conditions — lives in the schema, all six sections always present.
```

After:

```
Identity & Mission · `## Rules — delivered by mochiko-cli` · Adaptive Goal Protocol (…). Each ships as a `.md` alone; its rule content — roles · reserved · tools · ways-of-working · boundaries · fail-conditions, all six sections always present — lives in the migration log at [`plugins/mochiko/migrations/`](plugins/mochiko/migrations/) and is rendered at fire by `mochiko-cli`. No schema file ships (v0.107.0).
```

**Skills row (was line 61).** This row named no `schemas/` path and its count was already correct
(`ls plugins/mochiko/skills | wc -l` = 38; "the other 37" follows). Its defect was silence: it
described skills as prose-only while thirty of them ship a rules block. One clause added for
parity with the commands row; nothing removed.

Before: `…graded MUST/SHOULD triggers in their descriptions. Deterministic sub-checks ride as…`

After: `…graded MUST/SHOULD triggers in their descriptions. The thirty schema-bearing skills carry a `## Rules — delivered by mochiko-cli` block over rules held in the migration log, exactly as the commands do; the seven prose skills and the router carry no rule set. Deterministic sub-checks ride as…`

**Templates row (was line 62).** Before:

```
The **artifact schemas** re-homed to [`plugins/mochiko/schemas/`](plugins/mochiko/schemas/) as YAML data at v0.76.0 — the source of truth the `mochiko-cli` binary renders over and agents Read raw when it is absent (7 pipeline schemas after `plan.yaml` retired at v0.91.0, plus the two architecture-store schemas).
```

After:

```
The **artifact schemas** left `templates/` at v0.76.0 and now live in the migration log at [`plugins/mochiko/migrations/`](plugins/mochiko/migrations/), delivered at fire by `mochiko-cli` — no schema file ships and no agent Reads one (v0.107.0, the wave-6 end state).
```

The row's own count (`7 + constitution-modules/`) verified correct: seven `.md` files plus the
four-module directory.

### 3c. Three further repairs beyond the assigned lines — RULE ON THESE

Fixing the three assigned lines left the same dead pointer live in three other places in the same
file, which would have made the file contradict itself: the table saying no schema file ships while
the diagram and two paragraphs still showed `schemas/` as a plugin layer. I repaired all three
rather than ship a contradiction I had introduced. They are outside the lines assigned, so they are
flagged here for the lead's ruling and are individually revertible.

**(i) System-overview mermaid diagram (was lines 29, 33).** Before:

```
    schemas["schemas/ — artifact schemas (YAML)"]
    skills -->|"render via mochiko-cli,<br/>or Read raw when absent"| schemas
```

After:

```
    migrations["migrations/ — the rule + artifact-schema log"]
    commands -->|"rules rendered at fire<br/>by mochiko-cli"| migrations
    skills -->|"rules rendered at fire<br/>by mochiko-cli"| migrations
```

Net +1 line, which shifted every line number below it by one.

**(ii) Manifest paragraph (was lines 66–67).** Before:

```
outside the four layers (`templates/` and `schemas/` are referenced by commands and skills,
not registered; `schemas/` is data the Templates row above accounts for).
```

After:

```
outside the four layers (`templates/` is referenced by commands and skills, not registered;
`migrations/` is the rule log `mochiko-cli` reads, carried by the plugin and likewise not
registered).
```

**(iii) Command-form section (was lines 96, 99).** Before:

```
`## Identity & Mission` · `## Rules — load the schema first` · `## Adaptive Goal Protocol`,
… The rule-shaped content sits on the
pair's other surface, `plugins/mochiko/schemas/<cmd>.yaml`, in the six sections every schema
carries — roles · reserved · …
```

After:

```
`## Identity & Mission` · `## Rules — delivered by mochiko-cli` · `## Adaptive Goal Protocol`,
… The rule-shaped content sits on the
pair's other surface — the command's rules in the migration log, rendered at fire by
`mochiko-cli` — in the six sections every schema carries: roles · reserved · …
```

The heading string `## Rules — delivered by mochiko-cli` matches the scaffold criterion in
`.claude/rules/mochiko/primitive-edits.md`.

Post-edit sweep: `grep -niE "schema\.yaml|schemas/|load the schema|Read raw" ARCHITECTURE.md`
returns nothing.

### 3d. Left alone in ARCHITECTURE.md

Line 3's version stamp still reads `(v0.91.0, …)` against `main` at v0.108.0. Lead-owned; it moves
with the bump.

## 4. Repo-prose sweep — hits and dispositions

Terms swept: `model: opus` · `model: sonnet` · "all/every persona" · "all ten agents" · "rostered
seats" · "rostered personas" · "strong tier" · `model-tiered-seats` · bare `opus` / `sonnet`.
Excluded per brief: `.mochiko/brainstorms/`, `.mochiko/strips/`, `.mochiko/archive/`,
`.mochiko/decisions/`, `CHANGELOG.md`, `.git/`.

`ARCHITECTURE.md`'s agents row was the only prose in the repo asserting a uniform persona tier.

| Hit | Disposition |
|---|---|
| `ARCHITECTURE.md` agents row | **EDITED** — §3a |
| `plugins/mochiko/agents/*.md` frontmatter ×10 | four **EDITED**; six strong seats untouched |
| `plugins/mochiko/skills/patterns-model-tiering/SKILL.md:3, 13, 19` — "rostered seats never change model", "Rostered mochiko personas run on the strong tier and stay there", "Opus-cap headroom" | P1's unit. Not edited. |
| `plugins/mochiko/skills/mochiko/SKILL.md:69` — tiering router row, "rostered seats never change tier (model-tiered-seats D5)" | P1's unit. Not edited. |
| `plugins/mochiko/migrations/0001-genesis.yaml`, `0004-sonnet-worker-rung.yaml` | Log, append-only by construction; superseded by P1's `0006`. Not edited. |
| `.mochiko/schema-views/skills/patterns-model-tiering.yaml` | Regenerated, never hand-edited. P1's. Not edited. |
| `crates/mochiko-cli/tests/fixtures/genesis-corpus/…/patterns-model-tiering/schema.yaml` | Frozen test fixture, not README-class prose. Not edited. Flagged: it embeds the pre-`0006` floor text. If the crate re-derives from it, that is P1/V1's call. |
| `evals/contract/README.md:266` — floor-id counts incl. "`patterns-model-tiering` 4 → 6" | P1's `floor_ids`/`floor_pin` re-key. Not edited. |
| `DECISIONS.md:13, 14, 46` · `ROADMAP.md` token-epic row · `BACKLOG.md:661` | Lead-owned; already carry the new ruling. Not edited. |
| `CHANGELOG.md:27, 921, 940` | Lead-owned, and excluded by brief. Not edited. |
| `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/*.md` (6 files, `model: opus`) | Frozen benchmark archive, not shipped. Not edited; named in the strip entries. |
| `README.md` (repo root) | No hit — makes no model-tier claim. Not edited. |
| `evals/run.py`, `evals/commands/run.py` (`SESSION_MODEL = "sonnet"`) | Skill-eval session constants, not persona tiers. Not edited. |
| `plugins/mochiko/.claude-plugin/plugin.json:18–28` | Ten path strings, no model axis. Verified unchanged. |
| `plugins/mochiko/skills/mochiko/SKILL.md` agents table (rows at 139–149) | **Checked as the brief asked: no persona row names a model.** Role text only. Not edited. |

## 5. Not done, and why

- **D7 item 4 — persona-kit runner `ARM_MODEL` → per-persona default.** Out of reach on this
  branch: `evals/agents/` and `evals/commands/agents.py` do not exist in this worktree. Verified —
  `evals/commands/` holds `README.md`, `brainstorm-probe`, `implement`, `run.py`, `setup`,
  `wrapper.md`. Both live on `primitive-evals-v2`. Deferred to the branch merge, per the build
  plan's Deferred section.
- **Grading.** Not self-graded, per the author ≠ grader floor. V2 holds this.
- **Landing ritual** (`DECISIONS.md` row, `CHANGELOG.md`, `plugin.json` + `marketplace.json` bump
  to 0.110.0, ROADMAP/BACKLOG touches) — lead-owned, not P2's surface.

## 6. For V2

Grading surface: internal coherence of the four persona files, preserved responsibilities, the four
strip entries against `.mochiko/strips/README.md`'s supersession shape, and the untouched
`plugin.json` agents list.

Two items warrant explicit attention:

1. §3c — three ARCHITECTURE.md repairs beyond the lines the lead assigned, with the reason. If the
   lead rules them out of scope, each reverts independently.
2. The `technical-analyst` strip entry deliberately omits both the G7 fold and the benchmark-variant
   clause the other three carry. Neither applies to that seat: its graders emit verdicts, and it has
   no file under `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/`. The asymmetry is
   intentional, not an omission.
