---
name: compressing-skills
description: Maintainer-side compression pass over one mochiko plugin skill — rule inventory, protected-content reconciliation, Arm A/B variants, eval dispatch, ratification gate. Invoke on "compress skill <name>" or "run a compression pass".
---

# Compressing Skills

Repo-level maintainer tooling. **Never shipped** — this skill lives at `.claude/skills/` and
must never move under `plugins/`. It compresses exactly one mochiko plugin skill per pass and
hands the result to the before/after eval before anything may land.

Provenance: `.mochiko/brainstorms/skill-compression-tooling/record.md` (D1–D8 as amended,
folds R1–R16). That record is the authority; this skill carries the procedure.

## Scope and fences (non-waivable)

- **One skill per pass** (D3). Input: one skill directory under `plugins/mochiko/skills/`.
- **Bodies + `references/*.md` only** (D6). `description:` frontmatter is out of scope — never
  edit it here (its instrument is the fire-rate watch, not this pass). Scripts, YAML, and
  `templates/` are out of scope.
- **Topology fenced** (D5): no file renames, no merging references into `SKILL.md`, no new
  files, no relocating content anywhere (including `templates/` — banned destination). Densify
  or delete in place only.
- **True-reductions-only**: a cut that reappears elsewhere is a sham cut and forbidden.
- **Never write into `plugins/`** during the pass. Variants are staged under
  `evals/<skill>/variants/`; the plugin tree changes only at the landing step, through the full
  edit ceremony.
- **Arm A runs once per skill per landing** — no iterative re-densification loops.
- Measure in **chars, never `wc -c` bytes** (ledger accounting law).

## Procedure

### 1. Measure

Record the current char count of `SKILL.md` and each in-scope reference (Python
`len(open(f).read())`, the ledger-canonical measure). Check
`.mochiko/memory/primitive-cost-budgets.md` for the skill's body budget and any deliberate
stance (e.g. the `mochiko` router body is deliberately unbudgeted — the body IS the router
index; the router rides **Arm A only**, never Arm B).

### 2. Rule inventory (the eval's checklist — author ≠ grader)

Enumerate every behavioral rule the **baseline** skill asserts: each MUST/SHOULD, each floor,
each anti-pattern, each format obligation, each owned-vocabulary definition. One entry per
rule: `{id, rule, class: floor|must|should|format|vocab, source: <file:section>}`. Write it to
`evals/<skill>/rules.json`.

**Consumer-side check (R10):** grep `plugins/mochiko/` for references to this skill's owned
vocabulary and single-source claims ("owns the X grammar", "single source of Y", skill-name
citations). Each consumer-cited term becomes a rule entry (`class: vocab`) — paraphrase that
breaks a consumer is a failed rule.

**Independence:** the inventory must be built or reviewed by a seat that is not the compressing
author. An unreviewed inventory blocks the eval — a rule the compressor never noticed is a rule
the eval never tests.

### 3. Protected-content reconciliation (R2 — before any cut is drafted)

Read `.mochiko/strips/<skill>.md` end to end (if the file does not exist — a never-stripped
skill — record that in the pass report and the protected set is empty). Enumerate the protected set: every `KEPT:`
survivor ruling and its enumerated elements, plus any `DECISIONS.md`-traceable line. Write the
list into the pass report. Arm B may touch a protected element **only** as a recorded
supersession-by-ruling (the v0.64.0 reconciliation entries are the worked precedent). If the
whole remaining body is a `KEPT:` survivor, say so in the report and scope Arm B to what a
supersession could honestly cover — or recommend Arm A only.

### 4. Arm A — lossless densification

Same headings, same rules, same MUST/SHOULD grading, same example count. Remove only:
restatement, hedging, throat-clearing, duplicated framing, prose that repeats an adjacent
table. Zero information leaves. Write the variant as a full skill-directory copy at
`evals/<skill>/variants/armA/`.

### 5. Arm B — cut-line strip

Apply the inherited cut line (`validator-scope-and-verbosity` D4): **keep** goal + output
contract, non-waivable floors, anti-patterns and rejections, hard reference data; **drop**
step-by-step procedure, worked examples, restatement — subject to the
protected set from step 3. Whole sections may vanish; information loss is by design and is what
the eval prices. Write to `evals/<skill>/variants/armB/`. For each dropped span, note the rule
IDs (step 2) it carried — those are the rules the eval is expected to stress.

### 6. Eval dispatch

Preconditions, each blocking:

- **Goldens commissioned:** `evals/<skill>/evals.json` (3 per skill) authored by a
  non-compressor seat — missing or compressor-authored goldens block the eval, same as an
  unreviewed inventory.
- **Probe run done (R5):** one cheap probe (`run.py probe`) has settled invocation mechanics
  before any priced grid.
- **Pre-registration recorded** (below).

Hand off to the runner (`evals/README.md` for usage): four arms — no-skill control, baseline,
Arm A, Arm B — 3 replicates each, pass^k aggregation, Haiku checklist judge, plus the Sonnet
pairwise read — blind, position-swapped, low-replicate, secondary only (both judges advisory).
Rules that pass under the no-skill control are **pruned** — they measure the model, not the
skill (R3). The **pre-registered ship bar** (floor rules absolute; rules-lost bound) and the
**delivered-chars arithmetic** live in `evals/<skill>/preregistration.md`; the bar is proposed
by a non-compressor seat and ratified by the user — the compressor never sets the bar it is
graded against. Runs are metered API spend (`ANTHROPIC_API_KEY`).

### 7. Pass report

One report per pass at `evals/<skill>/pass-report.md`: measurements (before/after chars per
arm) · rule inventory pointer + reviewer · protected set + any supersession need ·
dropped-span/rule map · eval results table (rules lost per arm, pass^k; rules pruned by the
no-skill control; pairwise verdicts with position-consistency; judge parse failures; costs) ·
recommendation per arm against the pre-registered bar.

### 8. Ratification gate

The user rules per variant: ship Arm A, ship Arm B, ship neither. No variant lands without
this ruling. The eval report is evidence, never the decision.

### 9. Landing hand-off

A ratified variant lands through the **standard edit ceremony, unchanged**
(`.claude/rules/mochiko/primitive-edits.md`): per-cut `.mochiko/strips/<skill>.md` entries
(supersession-by-ruling entries for any protected content) **carrying the rule inventory
(`evals/<skill>/rules.json`) and the dropped-span/rule map as evidence**, the independent
author≠grader audit with the eval report attached as evidence, the `plugin.json` bump,
CHANGELOG. At landing,
**re-seed the skill's body budget downward** in `.mochiko/memory/primitive-cost-budgets.md`
(R11) — headroom does not survive the cut. Post-ship degradation re-adds content via the strips
README re-add entry type, not by silent edit.

## Common mistakes

| Mistake | Fix |
|---|---|
| Compressing several skills in one pass | One skill, one pass, one report |
| Editing `plugins/` directly while drafting | Variants live under `evals/<skill>/variants/` |
| Relocating a paragraph to survive the cut | Sham cut — delete or keep, never move |
| Arm B silently dropping a `KEPT:` element | Supersession-by-ruling or the element stays |
| Grading with the inventory you authored, unreviewed | Non-author seat builds or reviews it |
| Running the grid before the bar is pre-registered | `preregistration.md` first, then runs |
| Treating judge output as the gate | Judges are advisory; the user's ratification gates |
