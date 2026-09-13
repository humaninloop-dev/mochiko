# Template readiness for a conformance block — wave-1 reconnaissance for the wave-3 census

**Status:** fact table, not a design. Wave 1 measured; the D2/D5 census rules on every row.
**Produced:** 2026-09-13, wave 1 (staff-engineer seat), from the log at
`plugins/mochiko/migrations` at commit `35c2c5b`. **Consumers:** D11 wave 3 (the census migration
and the budget table), D6 (OQ1 per-section budgets), D4a/D4c (heading grammar, placeholder scope).

---

> ## Constraint wave 3 must not miss
>
> **The conformance keys and the `home` `import-document` ops must land in the same migration
> file.** `schema::Section` ignores unknown keys by design (`crates/mochiko-cli/src/schema.rs`
> doc comment: "Unknown/extra keys are ignored rather than rejected, so a template can grow fields
> without breaking an older binary"). A migration that added `max_lines`, `heading`, or
> `conformance` **alone**, after the first `mochiko-cli-v*` publish, would therefore be read by an
> older binary as a template with no budgets: the producer view would omit them and `check` would
> pass a file that violates a budget it cannot see. That is a silent degradation, not a halt.
>
> Wave 3 is safe only because the same migration also carries `import-document` ops of
> `kind: home`, and an unknown document kind is **rejected loudly** — `crates/mochiko-cli/src/migration.rs:619`,
> "`kind: {kind_token}` is not a document kind". The loud rejection on the `home` op is what
> protects the silently-ignorable conformance keys travelling beside it.
>
> **If wave 3 splits them into two migrations, or if any later migration adds a conformance key on
> its own, that migration takes a grammar bump** (`GRAMMAR_RANGE` in
> `crates/mochiko-cli/src/migration.rs:16`, currently `(1, 1)`). Wave 1 deliberately does not bump:
> no `mochiko-cli-v*` tag exists yet, so the range freezes at wave 1's own publish with `home` and
> the conformance keys already inside it.

---

## Method

Every figure below is read from the replayed log, not from a shipped file. Reproduce with:

```
cargo run -q --manifest-path crates/mochiko-cli/Cargo.toml -- \
  --log-dir plugins/mochiko/migrations views emit --out <scratch>
```

then compare each `<scratch>/templates/<name>.yaml`'s `sections[].name` list against the `##`
headings inside its own `skeleton:` block. "Complete `##` set" means the skeleton's headings cover
every section that should be a heading, so a heading-order check has something to bind to.

## The table

| template | sections | skeleton `##` | match | `heading:` overrides needed | complete `##` set | placeholder spelling | what wave 3 must author or rule first |
|---|---|---|---|---|---|---|---|
| `spec` | 11 (2 optional) | 11 | **exact** | none | yes | `{{…}}` ×20, `[…]` ×4, `FEAT-XXX` | nothing — ready for a conformance block as it stands |
| `governance-intent` | 14 (6 optional) | 13 | all but `Header` | none; mark `Header` as heading-less | yes | `[…]` ×36 | rule that `Header` governs no heading (it describes the title line) |
| `feature-entry` | 8 (5 optional) | 7 | all but `Header & Status` | none; mark `Header & Status` heading-less | yes | `{{…}}` ×17, `AX-XXX` | same ruling as above for `Header & Status` |
| `codebase-analysis` | 4 | 3 | **none** | **3** — em dash vs colon | yes, once overridden | `{{…}}` ×45 | three `heading:` overrides (`Part 1 — Inventory (Factual)` → `Part 1: Inventory (Factual)`, same for Part 2 and `Appendix`), plus the `Header` heading-less ruling |
| `tasks` | 4 | 2 | **none** | 1 or a skeleton fix | **no** | `[…]` ×12, `FEAT-XXX` | rule whether `## Cycle Cards` is a heading the artifact carries; the skeleton omits it and puts cards as `### - [ ] Cycle 1:` directly under `## Cycle Format` (see note 1) |
| `features-index` | 2 | 0 | **n/a** | n/a | **no headings at all** | `{{…}}` ×8, `<slug>`, `FEAT-XXX` | the artifact (`FEATURES.md`) is a title, a blockquote and one table — heading order is vacuous; rule it to frontmatter + whole-file bound + placeholders only |
| `architecture-store` | 5 (1 optional) | 10 | **none** | n/a | n/a | `[…]` ×8 | **the sections name files, not headings** (`Store layout`, `Concern ledger (AX rows)`, `Derived root index`) and the skeleton concatenates three files' headings; split into per-file templates or bind conformance at the `home` level (see note 2) |
| `governance-surfaces` | 5 (2 optional) | 0 | **n/a** | n/a | **no headings at all** | `<concern>` | its own skeleton says it "is a SET of five distinct surfaces, not a single fill-in document"; each section's contract carries the fenced shape. Split into five templates, or declare its homes `bounds: elsewhere` (see note 3) |

**Summary:** 1 of 8 is ready as it stands (`spec`). 3 more are ready after a heading-less ruling on
a leading meta section (`governance-intent`, `feature-entry`) or that plus three `heading:`
overrides (`codebase-analysis`). 4 need a wave-3 authoring decision before any heading-level
conformance block can bind (`tasks`, `features-index`, `architecture-store`,
`governance-surfaces`).

## Notes

1. **`tasks` — the template's own skeleton may be a source of the drift D1 is built to stop.**
   `## Cycle Cards` is a declared section, but the skeleton places the cards as
   `### - [ ] Cycle 1: …` after a `---` rule directly under `## Cycle Format`, with no
   `## Cycle Cards` line. Record F10 reports kinako's `FEAT-002/tasks.md` as having "no
   `## Header`, no `## Cycle Cards`, cards under `## Cycle Format`" — which is the skeleton
   followed faithfully, not a producer inventing a shape. Whichever way wave 3 rules, the
   template's section list and its skeleton must be made to agree, or the conformance block will
   deny artifacts that match the skeleton the producer was handed.

2. **`architecture-store` is a multi-file template.** Its sections describe the store's layout
   (`spine.md`, the `AX-XXX` ledger, a graduated concern file, the derived root index) and its
   skeleton shows three of those files in sequence: `## Container diagram` / `## Elements` /
   `## Key flows` (the spine), then `## AX-001 …` rows, then `## Spine` / `## Concerns` /
   `## Health` (the derived index). A single conformance block cannot bind one file here. The
   `home` document already binds a template per file (`deliverables[].template`), so the shape
   wave 3 needs is one template per file rather than one per store.

3. **`governance-surfaces` is a set, and two of its five surfaces are outside the gate already.**
   Its five shapes are the `CLAUDE.md` governance region, the `.claude/rules/mochiko/<concern>.md`
   files, the ledger, the trace manifest, and the output-style rules file. Record D5 already puts
   `CLAUDE.md` **outside the write-time gate entirely** and the `.claude/rules/mochiko/*.md` files
   under **location and file name only**, so at most three of the five could ever carry a shape
   check.

4. **Placeholder spellings are not uniform, and one literal token looks like a placeholder.**
   Four templates use `{{token}}` (`codebase-analysis` 45, `spec` 20, `feature-entry` 17,
   `features-index` 8); three use `[token]` (`governance-intent` 36, `tasks` 12,
   `architecture-store` 8); two carry `<token>`. D4c's "exact token spelling the template declares"
   is therefore the right rule and must be honored literally — **a pattern-based check would
   misfire.** The concrete trap: `tasks` uses `[P]` as *real* cycle-card syntax ("`[P]` marks
   parallel-eligible cards"), sitting beside genuine placeholders `[N]` and
   `[Simple | Split | Merge]`. Any `\[[A-Z]\]`-shaped rule would deny every conforming `tasks.md`.
   Wave 3 enumerates tokens per template; it must not derive them.

5. **No frontmatter exists in any of the eight skeletons.** None opens with a `---` block, so
   `conformance.frontmatter` (required fields, enums) binds nothing among these eight today. Its
   live consumers are the report envelope (`plugins/mochiko/templates/report-format.md` line 23
   carries the `report:` enum — `cycle | verification | final-validation | review | feasibility |
   disclosure`) and whichever deliverables wave 3 decides should carry frontmatter. The
   wave-1 mechanism supports it either way; the census decides where it applies.

6. **Optional sections are already declared and need no new field.** Sixteen of the 53 sections
   across the eight carry `required: false`, and the producer view already renders them
   `## Name — optional`. The conformance check reads the same flag: an optional section may be
   absent but must hold its relative order when present.

## What wave 1 does not claim

This table measures the eight templates the log carries. It says nothing about the roughly 22
homes with no template at all (record F9/F13), which take location, file set, and a whole-file
bound under D4f until their templates land. It also sets no budget: every `max_lines` figure is
the census's to propose and the user's to ratify (D6).
