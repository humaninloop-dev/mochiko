---
report: disclosure
round: 1
---

# P2 plan — prose: the new skill, the router, the budget row, the rules file, CHANGELOG, manifests

**Seat:** P2 (prose producer) · **Wave:** `wave1-gate-form.md` §2 P2 row, §3.2, §3.4 · **Date:** 2026-09-19
**Ruling planned against:** `.mochiko/brainstorms/author-grader-consolidation/record.md` D2, D3, D6, D7, D9, D11 and build-surface items 1, 2, 6, 7 (`DECISIONS.md` 2026-09-19)
**Target version stamp:** v0.111.0 · **Status:** plan only — nothing outside this file was written
**Depends on:** P1's `0008-gate-form.yaml` landing before §3's payload figure can be measured; the section ids are pinned in the wave plan §3.1 and are used verbatim below.

Files this seat will own at execute (six):

1. `plugins/mochiko/skills/validation-primitive-edit/SKILL.md` (new)
2. `plugins/mochiko/skills/mochiko/SKILL.md` (two edits)
3. `.mochiko/memory/primitive-cost-budgets.md` (two rows + one rule sentence — see risk R2)
4. `.claude/rules/mochiko/primitive-edits.md` (Check section replaced)
5. `CHANGELOG.md` (one entry prepended)
6. `plugins/mochiko/.claude-plugin/plugin.json` + `.claude-plugin/marketplace.json` (version only)

---

## 1. `skills/validation-primitive-edit/SKILL.md` — full draft

**Frontmatter `description` character count: 730** (characters of the parsed value, canonical
snippet; cap 1,536). Trigger phrases carried verbatim: `gate audit` · `primitive-edit audit` ·
`grade the pair` · `author≠grader audit`. Graded MUST + SHOULD both present.
**Body: 3,449 characters** (measured on the draft below; `validation-constitution` is 3,263 for
comparison). Payload = body + the seven renders, measured at execute per §3.

````markdown
---
name: validation-primitive-edit
description: This skill MUST be invoked when running the primitive-edit gate audit — the binary PASS/FAIL grade a shipped `plugins/mochiko/` primitive takes before the `plugin.json` bump that ships it (GI-004). SHOULD also invoke on 'gate audit', 'primitive-edit audit', 'grade the pair', or 'author≠grader audit'. The graded unit is keyed by kind — command pair · skill pair · prose primitive · schema content — and the criteria follow the unit; the deterministic pre-pass is run first-hand by the grader and quoted, never from the brief. Defaults to FAIL; run by a plain fresh seat that authored nothing in the unit, never the editor. Not the input-job `review-*` families, and not setup's governance surface set (`validation-constitution`).
allowed-tools: Bash(mochiko-cli *)
---

# Validating a Primitive Edit

The primitive-edit gate: one binary grade over one edited unit, standing between a
`plugins/mochiko/` edit and the `plugin.json` bump that ships it. Nobody rules after this grade,
which is what makes it a gate rather than input — the `review-*` family carries the input job.
You are a plain fresh seat that authored nothing in the unit, and the rules below, exactly as
`mochiko-cli` renders them, are the whole contract: your dispatch brief carries the unit, its
file paths, and the pre-pass command, and nothing else of the bar you hold. Maintainer-side
ceremony: `.claude/rules/mochiko/primitive-edits.md`.

## Rules — delivered by mochiko-cli

Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
carries — one block per section. Every block opens with a version-triple line
(`mochiko-cli rules validation-primitive-edit · section <id> · binary <v> · grammar <g> · plugin <p>`) and
closes with an end line (`mochiko-cli rules end · validation-primitive-edit · <id> · <N> rules`). **Proceed
only when every block carries both lines in that exact shape, from whichever channel delivered
it — this slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an
error, an empty block, the placeholder `[shell command execution disabled by policy]`, a
file-path-plus-preview stub — is a failure to deliver: surface `mochiko-cli rules not
delivered: <what was seen>` and halt. Never Read a schema file instead; there is no fallback.
The `legend` in the preamble block is the reading grammar; a `pointer:` binds you to that
file's or skill's procedure, referenced never restated.

!`mochiko-cli rules validation-primitive-edit --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.independence --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.scope --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.inputs --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.verdict --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.output --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules validation-primitive-edit --section validation-primitive-edit.sec.reserved --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`

Before the first procedural step, state back the floor count the preamble's `class: floor` pin
prints and the ids its `floors:` line lists; a blank or partial read-back is a skipped read —
halt and surface it.

## Procedure

Run the pre-pass yourself before anything else — the commands your brief names — and keep the
output to quote. Then read the unit's own files: the edited primitive, its rules as
`mochiko-cli` renders them, the migration and its regenerated view diff, the strip entry where
one is owed. Walk the judgment items for this unit's kind, confirming each once against what
you read. Close by emitting the verdict block in full, and write the outcome line into the
record of the landing this edit belongs to. On a FAIL you hand back the fix list and stop:
you are resumed for the one bounded re-audit rather than respawned.
````

Conformance notes for A1: the `!` lines match the contract suite's `BANG_LINE` regex exactly and
enumerate `preamble` then the six family ids in the wave plan's pinned order; `allowed-tools:
Bash(mochiko-cli *)` is present (the suite asserts the literal string); no rule text is restated
in the body; the delivery paragraph and the read-back sentence are the `validation-constitution`
text with the skill name substituted.

---

## 2. Router edits (`plugins/mochiko/skills/mochiko/SKILL.md`) — exact old → new

### 2.1 The `validation-*` paragraph (currently lines 33–38)

OLD:

```
**The two review-skill families** (the `validation-*`/`review-*` split, 2026-07-18): the prefix encodes **who owns the
clearing**. `validation-*` = the skill **issues the authoritative grade** — a binary PASS/FAIL
checklist grade, default FAIL, on the `validator` persona (today: `validation-constitution`;
a PASS is still human-gated downstream). `review-*` = the skill **produces
```

NEW:

```
**The two review-skill families** (the `validation-*`/`review-*` split, 2026-07-18): the prefix encodes **who owns the
clearing**. `validation-*` = the skill **issues the authoritative grade** — a binary PASS/FAIL
checklist grade, default FAIL, on the `validator` persona (today: `validation-constitution`;
a PASS is still human-gated downstream), joined by `validation-primitive-edit`, the
maintainer-side member carried by a plain fresh seat rather than a persona
(`author-grader-consolidation` D7). `review-*` = the skill **produces
```

The persona wording is left exactly as it stands — the `validator` sweep is wave 3.

### 2.2 New cluster table, inserted between the Brainstorm cluster's trailing blockquote and `### Entry point`

OLD:

```
> The questioning engine is `analysis-iterative` (registered under Specify, above — a general/shared skill); it is not brainstorm-specific.

### Entry point (user-invoked — you run it)
```

NEW:

```
> The questioning engine is `analysis-iterative` (registered under Specify, above — a general/shared skill); it is not brainstorm-specific.

### Primitive-edit gate (model-invoked — reached by the maintainer ceremony in `.claude/rules/mochiko/primitive-edits.md`)
| Skill | Reach when |
|-------|------------|
| `validation-primitive-edit` | running the **primitive-edit gate audit** — the binary PASS/FAIL a shipped `plugins/mochiko/` primitive takes before the `plugin.json` bump that ships it (GI-004). The unit is keyed by kind (command pair · skill pair · prose primitive · schema content) and the judgment items follow it; the deterministic pre-pass is run first-hand by the grader and quoted, never relayed from the brief; the verdict block carries the evidence-read line (absent ⇒ FAIL) and one outcome line goes into the record of the landing. Carried by a **plain fresh seat with an explicit `model:` alias**, never the editor, one seat per wave; a FAIL allows one fix and one re-audit by the same seat resumed, a second FAIL goes to the user. Not the input-job `review-*` families, and not setup's governance surface set (`validation-constitution`) |

### Entry point (user-invoked — you run it)
```

No other router edit. The `validator` agent row (`skills: validation-constitution`) is wave 3.

---

## 3. Budget ledger (`.mochiko/memory/primitive-cost-budgets.md`) — rows and measurement

### 3.1 How the payload is measured, once P1's render exists

Run from the repo root at execute, after `0008-gate-form.yaml` has landed and `views emit` is
clean. This is the ledger's canonical snippet with the render concatenation the D10.6 re-seed
rows describe (body + the seven rendered blocks, one number, characters of the parsed value):

```
python3 - <<'PY'
import re, subprocess
p = "plugins/mochiko/skills/validation-primitive-edit/SKILL.md"
text = open(p).read()
fm, body = re.match(r'^(---\n.*?\n---\n)(.*)$', text, re.S).groups()
sections = ["preamble"] + ["validation-primitive-edit.sec." + s for s in
    ("independence", "scope", "inputs", "verdict", "output", "reserved")]
render = "".join(subprocess.run(
    ["mochiko-cli", "rules", "validation-primitive-edit", "--section", s,
     "--plugin-root", "plugins/mochiko"], capture_output=True, text=True).stdout
    for s in sections)
m = re.search(r'^description:\s*(.*(?:\n  .*)*)', fm, re.M)
print("body", len(body), "render", len(render), "payload", len(body) + len(render),
      "desc", len(" ".join(l.strip() for l in m.group(1).splitlines() if l.strip())))
PY
```

The hook lines are excluded as the harness's, not the primitive's. No `references/` or
`scripts/` directory ships with this skill, so the exemption clause is inert. The figure is
taken at the release gate against the quiesced tree, not mid-fix-round — the standing rule that
caught `authoring-architecture-store` at v0.81.0.

### 3.2 Row text — skill bodies table (append after `testing-gap-finding`)

```
| validation-primitive-edit | PAYLOAD (first budget row at birth, seeded [v0.111.0] — payload: body 3449 + render RENDER; the argued-overage path applies from here) | PAYLOAD (no headroom) |
```

`PAYLOAD` and `RENDER` are filled from §3.1 at execute; `3449` is re-measured then rather than
carried, in case the fix round moves the body.

### 3.3 Row text — skill descriptions table (append after `review-code-minimalism`)

```
| validation-primitive-edit | 730 | 730 (no headroom) |
```

730 is above the ~500 family norm and is disclosed rather than trimmed: it carries the entry
site and the GI-004 gate boundary, the four SHOULD trigger phrases, the unit-keyed grading unit,
the first-hand pre-pass clause, the default-FAIL and plain-seat-independence lines, and the two
negative boundaries that keep it off `review-*` and `validation-constitution`. Precedent for a
description above the norm: `testing-gap-finding` 709, `review-sufficiency` 686,
`patterns-model-tiering` 643. Under the 1,536 delivery cap with 806 to spare.

### 3.4 One rule sentence — see risk R2

The ledger's rule paragraph admits three seeding paths (benchmark winner · ruled editorial cut ·
ruled schema conversion), none of which is a birth seed. The wave plan and record build item 1
direct a first-seed row at birth. Proposed addition to that paragraph, pending the lead:

```
or **a ruled birth seed** (`author-grader-consolidation`, build item 1, 2026-09-19: a skill born
inside a ruling that names its budget seeds to its measured delivered-at-invoke payload with no
headroom, and takes the argued-overage path from its first edit)
```

Alternative if the lead declines: write §3.2 and §3.3 in the precedent form instead —
"unbudgeted at birth, hard-cap-only; measured at birth at body X / desc 730" — matching
`testing-gap-finding`, `review-sufficiency`, `authoring-epic` and `authoring-architecture-store`.

---

## 4. CLI-asserted vs judgment inventory (wave plan §3.4, derived — not guessed)

Derived by reading `crates/mochiko-cli/src/validate.rs` (the `Code` enum plus `severity()`,
`REJECTING` 51 codes, `ADVISORY` 14) and `evals/contract/run.py` (`case_converted_shape`).

**[CLI] — rejecting, asserted by `mochiko-cli migrate validate`:** grammar parse/header/version ·
sequence collision and mismatch · hash mismatch · op unknown/malformed/inapplicable · log file
name · kind discriminator · **section set** · id format/prefix/duplicate · **mint-once** ·
**tombstone integrity** · label unknown/missing/retired · var unbound · **extends**
unresolved/cross-family/class-local · when undeclared/value · condition declaration · moment
undeclared/declaration · **enforces** unresolved/required/misplaced · **fail segment** · **skill
grammar** · class unknown · rule kind unknown · text missing · **protected exit** · **anchor
format** · depth exceeded · cite unresolved · **pointer unresolved** · superseded field · unknown
field · flat rules · document empty · home shape/pattern/duplicate/binding/bounds.

**[CLI] — advisory, reported and noted, never gating:** deixis · unused var/condition/moment ·
enforces coverage · condition coverage · **budget** (prints `N rules · M resolved characters of
rule text` — a measurement, never a comparison against the ledger) · cite foreign · labels
inherited · retired selector · pointless override · orphan block · zero-member label · skeleton
sigil. The similar-rule cluster sweep is the same class (`MOCHIKO_FULL_SIMILAR=1`).

**[suite] — asserted by the contract suite's `converted-shape` host case, and only these three:**
every `!` line renders its own primitive · the `!` lines enumerate every section the render
declares, in the render's order · the literal `allowed-tools: Bash(mochiko-cli *)` is in the
frontmatter. A fourth check in the same case asserts every converted primitive has a
pre-registered floor set and baseline in `expected-skills.json` / `EXPECTED`.

**[judgment] — everything else**, notably: the `.md` scaffold's headings and their order, the
frontmatter key set beyond the Bash grant, `<cmd>.sec.*` / `<skill>.sec.*` tokens appearing in
`.md` prose, the Not-done citation and its halt clause, the floor-count read-back sentence, the
`description:` byte-identity diff, the budget comparison and any overage argument, whether a
recorded ruling actually covers a protected exit, the near-dup extraction bar, the MOVE/DECLARE
single-homing call, D3's pair set, the AM-2 five for schema content, and coherence plus
preserved responsibilities for prose.

**Correction to the wave plan §3.4.** It states that "the `.md` scaffold headings/order and the
`!`-line enumeration are checked by the contract suite's `converted-shape` host case". The
`!`-line enumeration is; the scaffold headings and order are not. `case_converted_shape` makes
the three checks listed above and nothing else, and no case anywhere in `evals/` greps a
primitive for `## Identity & Mission`, `## Rules — delivered by mochiko-cli`, or `## Adaptive
Goal Protocol`. Command criterion 1 is therefore tagged **[judgment]** below, with its
`allowed-tools` limb **[suite]**. See risk R1.

---

## 5. `.claude/rules/mochiko/primitive-edits.md` — Check section, full replacement text

Replaces the current lines 37–277, from `- **Check** —` through the skill-pair block's closing
Rulings paragraph. The frontmatter, the opening ceremony paragraphs, the schema-content
paragraph, the Record bullet, the persona-grid paragraph, the protected-content paragraph and
the closing pure-additions line are untouched. Every numbered item is preserved and tagged.

````markdown
- **Check** — one binary gate audit over one unit, run by **the gate grader**: a plain fresh seat
  (`general-purpose`, no persona) that authored nothing in the unit, spawned with an **explicit
  `model:` alias** equal to the tier the edit was produced at and never below — `opus` when the
  lead made the edit. An omitted alias is a floor miss
  (`patterns-model-tiering.persona-less-grader-pin`). The seat's contract is
  `mochiko:validation-primitive-edit`'s render pasted verbatim into the brief; the dispatcher
  writes only the unit, its file paths, and the pre-pass commands, and a hand-written contract
  section is a floor miss on the same terms as an omitted alias. **One seat takes every unit of a
  wave** (`author-grader-consolidation` D11): each unit keeps its own verdict block and its own
  outcome line tagged with the seat, and the seat splits into two only when the units' files
  would not fit its context, saying so in the lines. The editor never grades their own edit.

  **The deterministic pre-pass runs first, and the grader runs it.** `mochiko-cli migrate
  validate --report --plugin-root plugins/mochiko` (0 rejecting to proceed; advisory findings
  noted, never gating) and the char-budget measurement are executed by the grader itself and
  quoted in the verdict block. A pre-pass result quoted from the brief is not evidence, and
  nothing that output asserts is re-derived by judgment.

  **Char-budget pre-assert (D7).** The grader counts the edited primitive's budgeted classes —
  skill body, skill `description:` value, agent `description:` value — as **characters of the
  parsed value, never `wc -c` bytes** — against `.mochiko/memory/primitive-cost-budgets.md`
  (canonical measurement snippet lives there). Over budget = FAIL, unless the editor named the
  overage in the audit brief with a justification the grader rules holds (a genuine new
  obligation — never restored playbook prose). `references/` files are exempt. Primitives
  without a measured budget fall back to hard caps only (skill `description:` ≤ 1,536 delivery
  cap); budgets are never invented.

  **The graded unit is keyed by kind, and the criteria follow the unit.** For a **command**, the
  graded unit is the command's own **pair** — `plugins/mochiko/commands/<cmd>.md` + the command's
  rules as `mochiko-cli` renders them from the log — held against the canonical-scaffold criteria
  below. (This supersedes the "the command's own text" bar of ADR
  `2026-08-02-doctrine-purge-wave-1` decision 4; ruling: `command-md-scaffold-standardization`
  D1, C1 fold.) For a **schema-bearing skill** the graded unit is likewise the pair — `SKILL.md`
  + that skill's rendered rules — held against the skill-pair criteria block below
  (skill-content-schema D8/I6; the matching-skill routing never applies to the pair). For
  **schema content** — a migration file plus its regenerated view diff — the unit is that pair
  and the items are the AM-2 five: intent stated · anchor present where required · ID lifecycle
  right · floor and fail survival · register. For every other primitive — the seven prose skills
  and the router included — the unit is the file, graded on internal coherence plus preserved
  responsibilities, with the matching `validation-*` / `review-*` skill reached as the domain
  lens where one exists (`templates/command-shape.md` was deleted at v0.46.0; the dedicated
  `validation-command-shape` skill at v0.45.0).

  **The loop is bounded, and the bound has one home.** A FAIL allows one fix and one re-audit —
  by the **same grader seat resumed**, reading only what the fix touched and what it could have
  broken, never the whole cluster again. A second FAIL halts the landing and goes to the user
  with both fix lists; the user rules — fix again, or drop the edit. No run raises this bound.
  The number lives in `common.gate-loop-bound` and nowhere else; this file and
  `validation-primitive-edit.gate-loop-bound` cite that id, so a rename or tombstone of it
  sweeps them both. Overruling a grader the user judges wrong rides the governance ledger's
  existing waiver path.

  **The landing carries the audit's outcome line.** Each gate audit writes one line — `audit:
  <unit> · <seat> · <tier> · <n> files · <n> rounds · <n> blocking[ · cost: $<x>]` — into the
  record of the landing it belongs to: the wave's `build-log.md` entry when the edit ships from a
  session, the `.mochiko/decisions/` ADR when it is an ad-hoc defect close. The `cost:` field is
  present only for an audit run as a launched session, read from the SDK's per-session
  `total_cost_usd`; an in-session spawn carries none. A landing whose audits left no lines is
  incomplete.

  Each criterion below is tagged for where its answer comes from: **[CLI]** — asserted by
  `mochiko-cli migrate validate`, read from the pre-pass output and never re-derived by
  judgment · **[suite]** — asserted by the plugin contract suite's `converted-shape` host case ·
  **[judgment]** — the grader's own, confirmed once with one line of evidence. A criterion with
  two tags has a mechanical limb and a judgment limb, named in the item.

  **Canonical-scaffold criteria — every pair-form command, all six commands.** A command ships as
  a `.md` whose rules `mochiko-cli` renders from the log, and is graded across **both surfaces** on
  one criteria set. There is no second block and no per-form exception: the library has one
  scaffold (`command-md-scaffold-standardization` D1/D2), and the only branch is the
  done-condition class at the end of this list.

  1. **Scaffold conformance** — **[judgment]**, except the `allowed-tools` grant (**[suite]**).
     The `.md` carries the canonical headings in the canonical
     order — frontmatter (`description` · `argument-hint` ·
     `disable-model-invocation: true` · `allowed-tools: Bash(mochiko-cli *)` — a required
     key set; YAML key order is not graded, though all six ship in this order) ·
     `# <Name> — <epithet>` · `## Identity & Mission` (one tight
     section, never materially delaying the Rules block) ·
     `## Rules — delivered by mochiko-cli` · `## Adaptive Goal Protocol` with its three
     steps **Entry** → **Goal** → **Not done — default FAIL** (last). `$ARGUMENTS` is
     handled in Entry; the Not-done line cites the CLI-printed pin rather than carrying a
     count (criterion 3). No `**Goal:**` opener line, no `Harness` / `Bindings`
     sections, no per-command extra top-level section. Nothing mechanical reads these headings:
     the suite asserts only the literal `allowed-tools: Bash(mochiko-cli *)` string.
  2. **Rules-block enumeration** — **[suite]** for the enumeration, **[CLI]** for the schema's own
     set, **[judgment]** for tokens in prose. The section IDs enumerated in the Rules block match the
     schema's section IDs **set-wise** — the six-set `<cmd>.sec.roles` · `reserved` ·
     `tools` · `ways-of-working` · `boundaries` · `fail-conditions`, all six present in
     every schema, a section with no rules carrying its explicit empty marker (D4/D5).
     Every `<cmd>.sec.*` token anywhere in the `.md`, inside the Rules block or outside
     it, resolves to a live node.
  3. **FAIL survival** — **[CLI]** (`fail-segment`, `tombstone-integrity`, `mint-once`),
     **[judgment]** for the `.md`'s citation and halt clause. It keys to **`kind: fail`**
     (ontology D1, build item 4): every
     `kind: fail` rule survives (a reword keeps its ID), and the correspondence
     between the `<cmd>.fail.*` ID segment and `kind: fail` holds in both directions —
     `kind:` is never defaulted on a `.fail.*` ID. The
     hand-pinned count is gone by ruling (`cli-schema-delivery` D3: the counts are
     computed and printed by the CLI, never hand-pinned): the pin is the
     `- kind: fail · N rules` line the render prints under `pins` in the preamble block,
     and the `.md`'s Not-done line cites that pin and obliges a halt-and-surface when a
     delivered section's end-line count disagrees with it. Grade the citation and the
     halt clause; a hard-coded number there is the defect, not its absence.
  4. **ID continuity (D11/D14)** — **[CLI]** (`tombstone-integrity`, `mint-once`, `id-format`,
     `id-prefix`, `id-duplicate`, `cite-unresolved`), **[judgment]** for a reference carried in
     prose rather than in a structured field. No `<cmd>.*` ID — rule **or** `<cmd>.sec.*` section —
     vanishes without a tombstone. A reword keeps its ID, a split mints children recording
     the parent, a merge tombstones the losers; no surviving rule text references a
     tombstoned or re-homed node.
  5. **`class: floor` = must-survive (M3)** — **[CLI]** for the exit and its anchor
     (`protected-exit`, `anchor-format`), **[judgment]** for whether the cited ruling covers it. A
     `floor`-class rule leaves only by recorded
     supersession-by-ruling; an `advisory`-class rule may change without the ceremony.
  6. **Substance across the pair** — **[judgment]**. Plan approval before any producing seat works ·
     author ≠ grader independence (no self-grading seat row) · decisions reserved to the
     user, carried in `<cmd>.sec.reserved` · bindings complete — paths, templates, entry
     condition — in `<cmd>.sec.tools` and the Entry step · the non-waivable floor in
     `<cmd>.sec.boundaries`. The floor includes the sound-loop pointer line
     `mochiko:patterns-sound-loop` on the three DM-chartered commands — `architecture` ·
     `feature` · `implement` (`charter-ritual-balance` D3) — counted on whichever surface
     carries it; the scaffold does not extend that pointer to `brainstorm` · `setup` ·
     `specify`, and an audit must not demand it there. Where the command is DM-chartered,
     the DM's bare-minimum responsibilities are present as owned responsibilities.
  7. **Done-condition class — grade the branch that matches the command, and only that
     branch** — **[judgment]**.
     - **Desk commands — `architecture` · `feature` — per-visit contract.** The protocol
       converges *each visit*, with the user, to a one-line goal **and its explicit done
       condition**, then runs to it and closes with a verdict against it. A visit ending
       with no stated done-condition verdict is a defect. Do **not** demand a fixed done
       condition here.
     - **Run commands — `brainstorm` · `implement` · `setup` · `specify` — fixed
       contract.** The **Entry** step carries the entry gating and, where the command
       routes, the neither-source routing; the **Goal** step states a **fixed** done
       condition; **Not done** defaults it to FAIL and is count-pinned. Do **not** demand
       a negotiated per-run goal — that is the desk's form, not the run's.
     - **`implement` additionally** (ADR `2026-08-13-charter-plan-implement` ruling 3):
       convergence at a named EXISTING user gate — run-open confirmation naming batch,
       scope type, attempt bounds (redeclarable there and only there), and the fixed done
       condition stated — closing at the existing acceptance gate. No new ceremony: the
       gate must already exist in the run.
  8. **Preserved responsibilities** — **[judgment]**, as for any primitive: protected content
     leaves only by recorded supersession-by-ruling, and strips + budgets apply unchanged.
  9. **Deterministic pre-pass** — **[CLI]**. `mochiko-cli migrate validate --report --plugin-root
     plugins/mochiko` is **run by the grader** and its output quoted, beside the char-budget
     measurement. (The Python checkers it replaces retired at v0.107.0.)
  10. **Provenance anchors (D16 as re-keyed)** — **[CLI]** for the anchor's presence and format,
      **[judgment]** for whether the ruling covers the exit. Decision anchors live on the log's own
      rules, carried by the migration that writes them — a supersession or tombstone of
      protected content carries its anchor in the migration, and the binary enforces that
      at apply. An anchored rule still leaves only by recorded supersession-by-ruling. The
      former sidecar is frozen at `.mochiko/archive/provenance-frozen-2026-09-05.yaml`,
      for provenance queries only.
  11. **Ontology-grammar conformance (D1–D8)** — **[CLI]** for the grammar (`rule-kind-unknown`,
      `class-unknown`, `condition-declaration`, `when-undeclared`, `when-value`,
      `moment-undeclared`, `moment-declaration`, `enforces-required`, `enforces-unresolved`,
      `enforces-misplaced`, `extends-unresolved`, `extends-class-local`, `extends-cross-family`),
      **[judgment]** for the extraction bar and the MOVE/DECLARE call. Across the pair: every
      `kind:` value comes from the nine-kind closed set — `constraint` · `duty` · `gate` ·
      `reservation` · `binding` · `bound` · `routing` · `fail` · `latitude` — with `constraint`
      the omitted default (an absent `kind:` reads `constraint` and is never written) · the schema
      declares the `conditions:` and `moments:` blocks its own rules use, and every `when:`
      term and every moment-resolved resolution point resolves against them · a rule-level
      activation guard is single-homed in `when:` and has left the `text` (MOVE), except
      where the condition rides the rule's subject noun and extraction would falsify the
      text or strand a referent, where `when:` is added and the text stands unchanged
      (DECLARE) · a `class: floor` rule is always read and always delivered whatever its
      `when:` — `when:` gates when the obligation applies, never whether it is delivered ·
      every `kind: fail` node carries `enforces:`, each listed ID resolving to a live local
      rule, an empty list legal only with its one-line reason · an `extends: common.<slug>`
      stub inherits `text` / `labels` / `pointer` only, declares `class:` locally (C3), and
      binds only where the block carries the command's responsibility under the extraction
      bar — exact duplicate across 3+ commands (ontology D8), or a 3+-command near-identical
      family converged under strongest-wording-wins (near-dup convergence ruling R1/R2,
      `.mochiko/decisions/2026-08-28-near-dup-convergence.md`; a member whose extra content
      is command-specific keeps local text, the edge recorded in
      `scripts/similar-rules-allowlist.yaml`). No co-Read of a common file is demanded: the
      render resolves every stub before the model sees it.

  Rulings: `.mochiko/brainstorms/command-md-scaffold-standardization/record.md`
  D1–D7 (`DECISIONS.md` 2026-08-27 — the canonical scaffold; supersedes the charter-form /
  goal-form split and this block's former dual-block shape, clause inventory in that
  record's Appendix A) · `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`
  D10 · `.mochiko/decisions/2026-08-13-charter-plan-implement.md` ·
  `.mochiko/brainstorms/charter-ritual-balance/record.md` D3 (`DECISIONS.md` 2026-08-13) ·
  `.mochiko/brainstorms/command-content-schema/record.md` D9 · D11 · D14 · D16
  (`DECISIONS.md` 2026-08-26) ·
  `.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 (`DECISIONS.md`
  2026-08-27 — the run-shape grammar; amends command-content-schema D6, and D3 narrowly) ·
  `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 (`DECISIONS.md` 2026-08-28 —
  widens the D8 extraction bar to 3+-command near-identical families,
  strongest-wording-wins).

  **Skill-pair criteria — every schema-bearing skill (the thirty); the review family from
  v0.100.0.** The seven prose skills and the router carry no rule set, never had a schema,
  and take the plain primitive ceremony above, not this block. A schema-bearing skill
  ships as `SKILL.md` whose rules `mochiko-cli` renders from the log
  (the skill directory stays the self-contained shipping unit) and is graded
  across **both surfaces** on this criteria set. The grader is the gate grader defined above,
  exactly as for a command pair; the matching `validation-*` / `review-*` skill routing never
  applies to the pair (no validator-for-skills exists, and a pilot member never
  grades itself). This block is a **sibling** of the command block above, never a fork of
  it: skills are their own grammar family — no `moments:`, no `$ARGUMENTS` protocol, no
  Not-done count-pin — under the same governance envelope.

  1. **Load-first section** — **[suite]**. The `SKILL.md` body carries a
     `## Rules — delivered by mochiko-cli` section whose seven `!` lines are the
     enumeration. No raw Read of a schema or of a family common file is demanded: the
     render resolves every `extends:` stub and every `${var}` before the model sees it, and
     prints the reading grammar — `when:` interpretation, floors always delivered, stub
     inheritance limits — as the preamble's `legend`. A member's own obligated reference
     read (e.g. `review-feasibility`'s lens) sequences in that section.
  2. **Section enumeration** — **[suite]** for the `.md` ↔ render match, **[CLI]**
     (`section-set`) for the schema's own set, **[judgment]** for tokens in prose. The section IDs
     enumerated in the load-first block match the
     schema's section IDs **set-wise** — the skill's **family section set**, minted once
     by that family's census-backed rollout ruling, uniform within the family, every
     section present in every member schema, a section with no rules carrying its
     explicit empty marker. Sets minted so far: the review family's
     `<skill>.sec.independence` · `scope` · `inputs` · `verdict` · `output` · `reserved`
     (census §H, v0.100.0); the authoring family's, swapping `verdict` for `artifact` —
     `independence` · `scope` · `inputs` · `artifact` · `output` · `reserved`
     (census-authoring J-1, v0.101.0); the patterns family's — a full swap-out, not one
     slot — `trigger` · `scope` · `discipline` · `inputs` · `disclosure` · `reserved`
     (census-patterns §B/J-P7, v0.102.0). The small families' dense five —
     `testing-end-user` · `testing-gap-finding` · `executing-tdd-cycle` ·
     `brownfield-integration` · `analysis-codebase` — mint no set of their own: they
     REUSE the review six-set with explicit empty markers, by the 2026-09-01
     small-families door ruling (census-small-families §B fit table, v0.103.0).
     Every `<skill>.sec.*` token
     anywhere in the `.md`, inside the load-first block or outside it, resolves to a live
     node. "The load-first block" reads as the delivered section's
     `--section` arguments — the six family ids in the preamble's printed order, behind the
     `preamble` line — and the set-wise match is graded against those.
  3. **Floor-count pin + read-back** — **[CLI]** for the printed pin, **[judgment]** for the
     sentence that cites it, **[suite]** for the frozen floor set in `expected-skills.json`.
     The load-first block obligates stating the floor
     count back before the first procedural step (the delivery read-back,
     skill-content-schema D6 as amended). The hand-pinned count is
     gone by ruling: the pin is the `- class: floor · N rules` line the render prints under
     `pins` in the preamble block together with the `floors:` index line beneath it, and the
     read-back sentence cites both — a hard-coded number there is the defect, not its
     absence.
  4. **Floor survival** — **[CLI]** (`protected-exit`, `anchor-format`), **[judgment]** for
     whether the cited ruling covers it. A `class: floor` rule leaves only by recorded
     supersession-by-ruling; an `advisory`-class rule may change without the ceremony.
  5. **ID continuity** — **[CLI]** (`tombstone-integrity`, `mint-once`, `id-duplicate`,
     `cite-unresolved`). No `<skill>.*` ID — rule or `<skill>.sec.*` section — vanishes
     without a tombstone. A reword keeps its ID, a split mints children recording the
     parent, a merge tombstones the losers; no surviving rule text references a tombstoned
     or re-homed node.
  6. **`extends:` conformance** — **[CLI]** (`extends-unresolved`, `extends-cross-family`,
     `extends-class-local`), **[judgment]** for the near-dup bar. An
     `extends: <family>-common.<slug>` stub binds only the
     skill's own family library (cross-family sharing forbidden, D5), inherits
     `text` / `labels` / `pointer` only, declares `class:` locally, and binds only under the
     near-dup bar — near-identical across 3+ members, strongest-wording-wins (R1/R2,
     `.mochiko/decisions/2026-08-28-near-dup-convergence.md`); a member whose extra content
     is skill-specific keeps local text, the edge recorded in
     `scripts/similar-rules-allowlist.yaml`. The stub's `<skill>.*` ID stays the citable
     ID. No co-Read of a family common file is demanded: the render resolves every stub
     before the model sees it.
  7. **`description:` untouched** — **[judgment]** (a diff the grader reads; the cap is measured
     with the canonical snippet). The frontmatter `description:` value is byte-identical
     across the conversion and ≤ 1,536 chars (the delivery cap); it never moves to schema. For a
     skill born in this wave there is no prior value to be identical to: the item reads
     "≤ 1,536 chars" only.
  8. **Budget = delivered-at-invoke payload** — **[CLI]** for the measurement (the render plus the
     ledger's canonical snippet), **[judgment]** for any overage argument. The budgeted quantity is
     the `SKILL.md` body plus the seven rendered blocks the `!` lines deliver, one number,
     characters of the parsed value (`cli-schema-delivery` D10 clause 6 — no schema file is read at
     invoke, so none is part of the payload). The budget re-seeded to that measured figure at
     conversion with **no +25% headroom** (the ledger's third seeding path — the conversion
     is a relocation, never a measured winner); content growth takes the normal
     argued-overage path, named in the brief. The hook lines are excluded as the harness's,
     not the primitive's; `references/` and `scripts/` stay exempt. `migrate validate`'s
     `budget` finding is advisory and counts rule text only — it is never the ledger comparison.
  9. **Pointer resolution** — **[CLI]** (`pointer-unresolved`). Every `pointer:` resolves
     base-dir-relative from the skill
     directory, cross-directory climbs included (`../<other-skill>/references/...` is
     legal; the Single-source convention governs the pointed-at files).
  10. **Deterministic pre-pass** — **[CLI]**. `mochiko-cli migrate validate --report
      --plugin-root plugins/mochiko` is **run by the grader** and its output quoted, beside the
      char-budget measurement. (The Python checkers it replaces retired at v0.107.0.)
  11. **Skill-grammar conformance** — **[CLI]** (`skill-grammar`, `rule-kind-unknown`,
      `when-undeclared`, `moment-declaration`). Every `kind:` value comes from the **eight-kind**
      skill set — `constraint` · `duty` · `gate` · `reservation` · `binding` · `bound` ·
      `routing` · `latitude` — with `constraint` the omitted default; **`kind: fail` and
      `enforces:` are illegal in a skill schema** (census-evidence retirement,
      skill-content-schema D9/M2). Every `when:` term resolves against the schema's
      declared `conditions:`; a `class: floor` rule is always read and always delivered
      whatever its `when:`; no `moments:` block exists (procedure stays prose, D3).
  12. **Provenance anchors** — **[CLI]** for the anchor mechanics, **[judgment]** for whether the
      ruling covers the exit. Decision anchors live on the log's own rules, carried by the
      migration that writes them and enforced by the binary at apply; a rule carrying a
      supersession-transfer (a `KEPT:`-protected or `DECISIONS.md`-traceable line relocated
      into schema content, skill-content-schema D8/C4) inherits protected status through
      that anchor and leaves only by recorded supersession-by-ruling. The former sidecar is
      frozen at `.mochiko/archive/provenance-frozen-2026-09-05.yaml`.

  Rulings: `.mochiko/brainstorms/skill-content-schema/record.md` D1–D9 as amended
  (`DECISIONS.md` 2026-09-01) · the census inventory
  `.mochiko/brainstorms/skill-content-schema/research/census.md` (§E kind retirement · §H section
  set · J-7 cross-directory pointers) · the authoring-family census
  `.mochiko/brainstorms/skill-content-schema/research/census-authoring.md` (§I labels · J-1
  section set · J-6 budget · J-7 first-seeds) · the patterns-family census
  `.mochiko/brainstorms/skill-content-schema/research/census-patterns.md` (§B section proposal ·
  §ROAD rejection · §I labels · J-P2 first-strips · J-P5 two-arm/overage) · the
  small-families census
  `.mochiko/brainstorms/skill-content-schema/research/census-small-families.md` (§B six-set-reuse
  fit table · §C zero common blocks · §D abort-tripped — the dense five convert on the
  B/C drivers · J2-8 dual-homing twins · J2-9 ruled repair) ·
  `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 ·
  `.mochiko/brainstorms/author-grader-consolidation/record.md` D2–D7, D9, D11 (`DECISIONS.md`
  2026-09-19 — the gate form: the unit-keyed judgment items, the first-hand pre-pass, the plain
  grader seat, the bounded loop, the outcome line).
````

---

## 6. `CHANGELOG.md` — entry draft (prepended under the file header)

````markdown
## [0.111.0] — 2026-09-19

**The primitive-edit gate form** (record
`.mochiko/brainstorms/author-grader-consolidation/record.md` D2–D7, D9, D11; `DECISIONS.md`
2026-09-19; supersedes `author-grader-value-tiering`). Author≠grader was ruled in pieces across
five sessions and one governance principle; this wave lands the gate half as one contract.

Two grading jobs are now ruled rather than grown (D2). A **gate** — a binary the lead cannot ship
past — applies only where the artifact leaves the repo with no human ruling behind the grade:
shipped plugin primitives before the `plugin.json` bump (GI-004), and setup's validate step.
**Input** — severity-ranked findings the user rules — applies everywhere a user ruling already
sits downstream. At the primitive-edit gate the completeness rule shrinks to judgment items keyed
by unit, each confirmed once with one evidence line (D3); the mechanical tier is the deterministic
pre-pass, which **the grader runs itself and quotes** — a pre-pass result relayed in the brief is
not evidence, and a re-audit reads only the fix delta. The posture (default FAIL) and the
tamper-proof clause (a verdict with no evidence-read line is FAIL) are unchanged, and setup's
validate step keeps `validation-constitution`'s completeness floors untouched (C1).

New skill **`validation-primitive-edit`** (D7/C3) — the gate contract as rules on the review
six-set, eleven floors. It is carried by a **plain fresh seat**, no persona, spawned with an
explicit `model:` alias equal to the tier the edit was produced at and never below, `opus` when
the lead made it; an omitted alias is a floor miss. The dispatcher pastes the render verbatim and
writes only the unit, its paths, and the pre-pass command — a hand-written contract section is a
floor miss on the same terms. One seat takes every unit of a wave (D11), each unit keeping its own
verdict block and its own outcome line.

Migration **`0008-gate-form.yaml`** (sequence 8): `common.gate-loop-bound` minted in the command
common block, its number in its own text (D4/D6) — a FAIL allows one fix and one re-audit by the
same seat resumed, a second FAIL halts the landing and goes to the user, fix again or drop, and no
run raises it — extended by `setup.gate-loop-bound`; `setup.validate-seat-form`;
`patterns-model-tiering.persona-less-grader-pin` (floor); and the `validation-primitive-edit`
document. `.claude/rules/mochiko/primitive-edits.md` Check section rewritten to the same form,
every criterion tagged CLI-asserted / contract-suite / judgment from a build-time inventory
against `crates/mochiko-cli/src/validate.rs` and `evals/contract/run.py` rather than guessed.
Every gate audit now leaves one outcome line (D9) in the record of its landing, the baseline the
wave-2 double-grade is read against.

Router gains the primitive-edit gate table and names the new member beside
`validation-constitution`. Budget first seed for the new skill: payload PAYLOAD (body BODY +
render RENDER), no headroom; description 730. Contract suite pre-registers the new member.
Audits: A1 (pairs) · A2 (schema content + pre-registration) · A3 (prose + crate) — verdicts in
`.mochiko/brainstorms/author-grader-consolidation/reports/`. Gates: `mochiko-cli migrate validate`
0 rejecting · views ≡ replay · `cargo test -p mochiko-cli` green · contract suite full sandbox run
green.
````

`PAYLOAD` / `BODY` / `RENDER` are filled at execute from §3.1; the audit and gate figures are
filled once §4 and §5 of the wave plan have run. No other CHANGELOG line is touched.

## 7. Manifest diffs

`plugins/mochiko/.claude-plugin/plugin.json` — one line:

```
-  "version": "0.110.0",
+  "version": "0.111.0",
```

`.claude-plugin/marketplace.json` — one line, inside `metadata`:

```
-    "version": "0.110.0"
+    "version": "0.111.0"
```

MINOR is right: one new skill, schema additions, no protected exits, no removed capability.
Both files are governed by `.claude/rules/mochiko/rust-cli.md` paths, not `primitive-edits.md`;
neither takes a strip entry.

## 8. Strips owed

None. The new skill and the router table are pure additions and ride the decision row.
`primitive-edits.md` is a repo rule, not a plugin primitive (record build item 4), so its rewrite
takes no `.mochiko/strips/` entry. `CHANGELOG.md` and both manifests are not plugin primitives.
Asserted, not assumed: the only `plugins/mochiko/` files this seat touches are the new
`SKILL.md` and the router, and the router edit removes nothing.

## 9. Risks

**R1 — the wave plan's [suite] claim for scaffold headings is wrong, and A3 grades my tags.**
§3.4 says the contract suite's `converted-shape` case checks the `.md` scaffold headings and
order. It does not. That case makes exactly three per-primitive checks — each `!` line renders its
own primitive, the `!` lines enumerate every declared section in the render's order, and the
literal `allowed-tools: Bash(mochiko-cli *)` is present — plus one suite-wide check that every
converted primitive has a pre-registered floor set. No file under `evals/` greps a primitive for
`## Identity & Mission` or `## Adaptive Goal Protocol`. I have tagged command criterion 1
**[judgment]** with its `allowed-tools` limb **[suite]**, against the wave plan's text, because
the record says the split is a build-time inventory and not to be guessed. If the lead wants §3.4
followed literally instead, say so and I will retag — but the tag would then assert a check that
does not exist.

**R2 — a first-seed budget at birth has no seeding path in the ledger.** The ledger admits three
(benchmark-measured winner · ruled editorial cut · ruled schema conversion) and states budgets are
never invented. Every new skill on record was **unbudgeted at birth, hard-cap-only**:
`testing-gap-finding` (v0.79.0), `review-sufficiency` (v0.91.0), `authoring-epic` (v0.72.0),
`authoring-architecture-store` (v0.81.0). The wave plan §3.2 and record build item 1 direct a
first-seed row. Writing one without touching the rule paragraph leaves the ledger contradicting
itself on its own first page. §3.4 above proposes the one-sentence fourth path anchored to this
record; the alternative is the precedent form. Lead's call — I will not write either silently.

**R3 — the prose-primitive route gains a second statement.** The new skill's
`.judgment-items-prose` covers a prose primitive (coherence + preserved responsibilities), while
this file and CLAUDE.md both say "the matching `validation-*` / `review-*` skill otherwise", and
CLAUDE.md is untouched this wave (record build item 3). My §5 text keeps the existing sentence and
re-points it — the gate grader's bar is `validation-primitive-edit` for every unit kind, and the
matching skill is reached as the domain lens where one exists. That is the smallest change that
does not strand CLAUDE.md, but it is a judgment call and the CLAUDE.md reconciliation belongs to
wave 3.

**R4 — trigger-phrase collision with `validation-constitution` is semantic, not literal.** None of
the four exact phrases appears in `validation-constitution`'s description. The residual risk is
that both descriptions open with "MUST be invoked to grade …", both say "defaults to FAIL", and
both say "never the author"; a model holding a governance-surface artifact could reach for the
wrong one. Mitigation is the explicit negative boundary in my description, which names both
`validation-constitution` and the `review-*` family. Verifying it needs a routing probe, which
this wave has no budget for; disclosed rather than assumed.

**R5 — description at 730 chars becomes the library's largest.** Prior high is
`testing-gap-finding` at 709. Under the 1,536 delivery cap with 806 to spare, and every clause is
routing-load-bearing (entry site · GI-004 boundary · four trigger phrases · unit keying ·
first-hand pre-pass · default FAIL · plain-seat independence · two negative boundaries). Disclosed
in §3.3 rather than trimmed, on the `testing-gap-finding` precedent.

**R6 — protected lines leaving `primitive-edits.md`, and the ruling for each.** Three, all under
`author-grader-consolidation` D3/D7 and record build item 2:

- "graded by `mochiko:validator`" (the schema-bearing-skill sentence, traceable to
  skill-content-schema D8/I6) becomes "the gate grader". The D8/I6 citation is preserved in place.
- "The grader is **`mochiko:validator`**, exactly as for a command pair" (skill-pair block
  preamble, same ruling) becomes "The grader is the gate grader defined above, exactly as for a
  command pair". The rest of the sentence is verbatim.
- "is cited in the audit brief" in command criterion 9 and skill criterion 10
  (`cli-schema-delivery`, the v0.107.0 Python-checker retirement) becomes "is **run by the
  grader** and its output quoted". The retirement parenthetical is preserved verbatim.

Nothing else is removed. "Restart from the checklist" and the full-cluster re-read are **not** in
this file — they live in `plugins/mochiko/agents/validator.md` and in `validation-constitution`'s
schema, and both retire at wave 3, outside this wave's scope. The record's phrasing ("leaves the
grader's brief") is satisfied here by the bounded delta re-audit the new text states.

**R7 — the router will read inconsistently at v0.111.0, deliberately.** The `validation-*`
paragraph will still say the family runs "on the `validator` persona" while its newest member runs
on a plain seat, and the `validator` agent row will still mount `validation-constitution` only.
That is §3.2's instruction (persona wording untouched, wave 3 sweeps it). The added clause names
the exception explicitly so a cold grader reads it as scoped rather than stale.

**R8 — the payload figure cannot exist until P1 lands.** §3.2's row and the CHANGELOG's budget
sentence carry placeholders until `0008-gate-form.yaml` is applied and `mochiko-cli rules
validation-primitive-edit --section …` renders. The body figure (3,449) is re-measured at the same
moment rather than carried forward, since the A1 fix round may move it.

---

## Execution

Executed 2026-09-19 on the lead's approval message, which ruled R1 (my tags stand, wave plan §3.4
corrected), R2 (write the fourth seeding path, then the first-seed rows), R3 (as drafted), and R6
(the three protected lines leave under D3/D7, recorded in the build log, no strip).

### Files changed (7)

1. `plugins/mochiko/skills/validation-primitive-edit/SKILL.md` — created, exactly the §1 draft.
2. `plugins/mochiko/skills/mochiko/SKILL.md` — the two §2 edits: the `validation-*` paragraph
   gains the new member, and the primitive-edit gate table is inserted ahead of `### Entry point`.
3. `.mochiko/memory/primitive-cost-budgets.md` — four edits: the fourth seeding path in the rule
   paragraph, the skill-bodies row, the skill-descriptions row, and a disclosure paragraph for
   the 730-char description.
4. `.claude/rules/mochiko/primitive-edits.md` — Check section replaced per §5; 371 lines.
5. `CHANGELOG.md` — the `## [0.111.0] — 2026-09-19` entry prepended above 0.110.0.
6. `plugins/mochiko/.claude-plugin/plugin.json` — `0.110.0` to `0.111.0`.
7. `.claude-plugin/marketplace.json` — `0.110.0` to `0.111.0`.

### Measurements

- Description: **730** characters of the parsed value, measured on the landed file with the
  ledger's canonical snippet. Cap 1,536.
- Body: **3,449** characters, measured on the landed file.
- Payload: **not measured at first execution.** `mochiko-cli rules validation-primitive-edit
  --section preamble --plugin-root plugins/mochiko` returned `error: no command or skill named
  'validation-primitive-edit' in the log`. P1's `0008-gate-form.yaml` parsed by then (the earlier
  `op-malformed · changes[0]: `section:` missing or not text` was gone) but did not yet carry the
  `import-document` for this skill. `PAYLOAD` / `BODY` / `RENDER` stood as placeholders in exactly
  two spots: the skill-bodies row in the budget ledger, and the budget sentence of the CHANGELOG
  entry. Both were filled in the resume pass below.

### Payload fill (resume pass, 2026-09-19, after migration 0008 landed)

Measured with §3.1 against the quiesced tree, plugin 0.111.0, all seven renders exiting 0:

| Quantity | Characters |
|---|---|
| Body | 3,449 |
| Render (seven blocks) | 11,573 |
| Payload | 15,022 |
| Description | 730 |

Per-section render: `preamble` 1,968 · `independence` 1,974 · `scope` 1,383 · `inputs` 1,279 ·
`verdict` 2,729 · `output` 1,566 · `reserved` 674. The preamble prints `class: floor · 11 rules`
with the eleven ids the wave plan §3.1 named, so the body's read-back sentence has a real pin to
cite. Body and description both re-measured on the landed file rather than carried from the plan;
both are unchanged from first execution.

Two placeholders filled, no other edit: the skill-bodies row in
`.mochiko/memory/primitive-cost-budgets.md` (15,022, no headroom, `[v0.111.0]` ruled birth seed,
the measurement's provenance stated in the row) and the budget sentence of the CHANGELOG's
0.111.0 entry. The CHANGELOG's audit and gate figures remain placeholders for the lead to fill at
landing.

### A1 fix — `patterns-model-tiering` row restamped (2026-09-19)

A1 round 1 returned FAIL on the `patterns-model-tiering` pair, item 8: this wave's
`persona-less-grader-pin` floor grew the render, and the row still carried its `[v0.110.0]` stamp
while the sibling rows were current. The fix is mine because the ledger is my file.

Re-measured independently rather than copied from the audit — the ledger's canonical snippet,
against the quiesced tree, with the section list read from the skill's own seven `!` lines:

| Quantity | Characters |
|---|---|
| Body | 3,112 |
| Render (seven blocks) | 15,246 |
| Payload | 18,358 |
| Budget | 10,852 |
| Overage | +7,506 |
| Description | 1,208 |

Per-section render: `preamble` 1,737 · `trigger` 2,592 · `scope` 1,248 · `discipline` 4,907 ·
`inputs` 1,162 · `disclosure` 2,860 · `reserved` 740. My figures agree with A1's to the character,
arrived at independently.

The delta over the `[v0.110.0]` payload is **+494, entirely render, and entirely one component**:
the `persona-less-grader-pin` floor minted on `patterns-model-tiering.sec.discipline` by
`0008-gate-form.yaml` under `author-grader-consolidation` D7, which closes that record's F12 by
obliging an explicit `model:` alias on every persona-less grader or reviewer spawn. The floor pin
moved 7 to 8. Body and description are unchanged at 3,112 and 1,208, so nothing in this stamp is
body or frontmatter growth, and no strip-recorded prose returned.

The row is restamped as a `[v0.111.0]` ruled-HOLDS overage at 18,358 with the justification named,
and the prior `[v0.110.0]` figures are kept inside the same row as history, in the row's existing
newest-first style. Nothing else in the ledger changed, and no other file was touched for this fix.

## A3 fix round (2026-09-19)

Five blocking findings across three of my files. All five applied, nothing else.

**1. Loop-bound paragraph, `primitive-edits.md`.** The paragraph stated the count ("one fix and
one re-audit") and then claimed the number lives in `common.gate-loop-bound` and nowhere else —
it restated in the same breath as it disclaimed. Rewritten: a FAIL sends the edit back for a fix
and a re-audit by the same seat resumed on the delta, and how many such rounds a landing gets is
the rule id's to say, cited rather than restated. The second-FAIL-to-the-user clause and the
no-raise clause stand as D6's ruled subject.

**2. Skill-pair criterion 1, same file.** Was wholly **[suite]**. Split like its command twin: the
`!`-line enumeration and the `allowed-tools` grant are **[suite]**, while the
`## Rules — delivered by mochiko-cli` heading itself, the no-raw-Read clause, and the
reference-read sequencing are **[judgment]** — the suite reads the `!` lines and the grant string,
not the heading or the prose around it.

**3. Birth-seed row re-measured.** Figures below.

**4. R5 disclosure paragraph.** "The library's largest description" was false:
`patterns-model-tiering` measures **1,208** on the current tree, its 643 being the v0.77.0 birth
figure that the seat-default-key ruling superseded. Reworded to "well above the ~500 family norm",
and the precedents now carry their current figures — `patterns-model-tiering` 1,208 (named as the
actual largest), `testing-gap-finding` 709, `review-sufficiency` 686, each re-measured this round.

**5. CHANGELOG.** One paragraph added: `mint-rule` takes `section:` as optional this wave — still
`op-malformed` on a command or skill, appends a block on a family common library — with no
grammar-version bump (additive, no `mochiko-cli-v*` tag released), and the plugin now requires
`mochiko-cli` 0.2.0 or later because a 0.1.x binary rejects `0008-gate-form.yaml` at parse and
halts rather than degrading. Binary verified in the tree: `mochiko-cli 0.2.0 · grammar 1..1`.

### Re-measurement — `validation-primitive-edit`

| Quantity | Characters |
|---|---|
| Body | 3,449 |
| Render (seven blocks) | 11,519 |
| Payload | 14,968 |
| Description | 730 |

Per-section render: `preamble` 1,966 · `independence` 1,974 · `scope` 1,383 · `inputs` 1,279 ·
`verdict` 2,679 · `output` 1,566 · `reserved` 672. Run twice, identical both times.

**The log is still moving, and the row says so.** Three readings of this payload now exist, each
correct at its moment: **15,022** at my fill pass (landing sha `8950f9b9…`), **15,031** as A3 read
it, and **14,968** now, against log state `sha256:7b58e8c1…` · 74 documents · 1067 rules · floor
pin 11, binary 0.2.0. The drift is P1's schema fix rounds, not measurement noise. I stamped the
figure I measured, as instructed, and recorded the two superseded readings plus the state hash in
the row itself — with the explicit note that the release-gate sweep must re-confirm it against the
quiesced tree. That sweep rule is the ledger's own, and it is what caught
`authoring-architecture-store` at v0.81.0. Flagging it rather than presenting 14,968 as final.

## A3 round 2 — the disposition limb (2026-09-19)

One finding: skill-pair criterion 5 was tagged wholly **[CLI]**, which overstates what the binary
proves. Verified against the crate before writing the tag rather than taking the finding on
report — the tombstone op in `crates/mochiko-cli/src/migration.rs` carries `disposition: String`,
plain free text, and the `tombstone-integrity` arm in `crates/mochiko-cli/src/validate.rs` raises
on exactly three conditions: an id tombstoned twice, an id both live and tombstoned, and an empty
disposition. It never reads what the disposition says. So "a reword keeps its ID", "a split mints
children recording the parent" and "a merge tombstones the losers" are judgment reads of free
text, not machine checks. A3 is right.

Three tags widened, nothing else touched:

- **Skill criterion 5** now carries **[CLI]** for the mechanics and **[judgment]** for the
  lifecycle each disposition claims, with the three conditions the binary actually checks named so
  a grader can see where the machine stops.
- **Command criterion 4**, which carries the same clause set, takes the same limb in the same
  words.
- **Command criterion 3** takes the limb for its own "every `kind: fail` rule survives (a reword
  keeps its ID)" clause, pointing at criterion 4 rather than repeating the statement.

Criteria count still 23 (11 command + 12 skill). No other file was touched in this round, and no
git command was run.

### Verification run

- `grep` over `primitive-edits.md`: no `mochiko:validator`, no "dispatch a separate validator", no
  "cited in the audit brief" survives.
- Criteria count: 23 numbered items (11 command + 12 skill), matching the pre-edit file.
- Tag occurrences: 36 across the two blocks.
- Both manifests read `0.111.0`.

### Deviations from the plan

Two, both additive and both inside the approved rulings.

1. **A disclosure paragraph was added to the budget ledger** beneath the skill-descriptions table,
   carrying the §3.3 justification for the 730-char description. The plan listed only the table
   row. The ledger's own convention is that every above-norm description carries its reasoning in
   prose (`patterns-system-design`, `testing-gap-finding`, `review-sufficiency` all do), and a
   bare row would have left R5 undisclosed in the file that the char-budget pre-assert reads.
2. **The CHANGELOG entry names the §3.4 inventory correction** in one clause, which the §6 draft
   did not. The lead's R1 ruling made the correction part of the shipped change rather than a
   plan-local note, so the release record states it.

Nothing was removed beyond the three lines named in R6. No git command was run.
