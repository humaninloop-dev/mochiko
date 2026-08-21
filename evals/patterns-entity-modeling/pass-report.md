# Compression pass report — `patterns-entity-modeling`

> **Status: SUSPENDED (user-ruled 2026-08-22) after grid stage 1** — noskill+baseline run
> (`runs/staged-001/`, $8.56): baseline 36/36 scripted assertions ×3 PASS; 3 rules pruned by
> control; checklist judge single-shot arrays ~9% unparseable → judge re-chunked, re-judge of
> stored artifacts pending resume. armA/armB sessions not yet run.
>
> *(Original status line:)* variants staged, grid stage 1 run. Steps 3–5 of `.claude/skills/compressing-skills/SKILL.md`
> are complete; eval results and the per-arm recommendation are **pending grid**.

| | |
|---|---|
| Skill under test | `plugins/mochiko/skills/patterns-entity-modeling/` |
| Variants | `evals/patterns-entity-modeling/variants/armA/` · `evals/patterns-entity-modeling/variants/armB/` |
| Rule inventory | `evals/patterns-entity-modeling/rules.json` — 82 rules, authored by the non-compressor inventory seat (author ≠ grader, D4/D8) |
| Goldens | `evals/patterns-entity-modeling/evals.json` — 3, same non-compressor seat |
| Ship bar | `evals/patterns-entity-modeling/preregistration.md` — ratified 2026-08-22 (9-rule floor set; Arm A bound 0; Arm B ≤ 5 of 73 non-floor / 0 `vocab` / ≤ 1 `must`) |
| Provenance | `.mochiko/brainstorms/skill-compression-tooling/record.md` (D1–D8 as amended, folds R1–R16) |
| Prior compression waves | v0.23.0 (workflow-token-reduction wave 2) · v0.27.0 (skill-succinctness wave 3) · v0.64.0 (guardrails-vs-detail wave 2) — this skill is **thrice-touched**, not twice |

---

## 1. Measurements

Chars via Python `len(open(f).read())` — never `wc -c` bytes (ledger accounting law, F4/D7).
`scripts/validate-model.py` (16,530 ch) is out of scope (F1) and excluded from every figure;
it was copied byte-identical into both variants and is not counted.

| File | baseline | Arm A | Δ A | Arm B | Δ B |
|---|---:|---:|---:|---:|---:|
| `SKILL.md` (whole file) | 14,261 | 13,364 | −897 (−6.3%) | 14,261 | 0 (0.0%) |
| `references/DATA-SENSITIVITY.md` | 6,374 | 5,996 | −378 (−5.9%) | 5,880 | −494 (−7.8%) |
| `references/RELATIONSHIP-PATTERNS.md` | 5,643 | 5,487 | −156 (−2.8%) | 3,525 | −2,118 (−37.5%) |
| `references/STATE-MACHINES.md` | 4,069 | 3,955 | −114 (−2.8%) | 1,722 | −2,347 (−57.7%) |
| `references/VALIDATION-RULES.md` | 6,845 | 6,793 | −52 (−0.8%) | 4,412 | −2,433 (−35.5%) |
| **In-scope total** | **37,192** | **35,595** | **−1,597 (−4.3%)** | **29,800** | **−7,392 (−19.9%)** |

**Representative invoke** (the pre-registered middle row — body + `DATA-SENSITIVITY` +
`RELATIONSHIP-PATTERNS` + `STATE-MACHINES`, what the three goldens actually exercise):

| Arm | Chars | ≈ tokens (chars/4) | Saved vs baseline |
|---|---:|---:|---:|
| baseline | 30,347 | ~7,587 | — |
| Arm A | 28,802 | ~7,201 | 1,545 ch (~386 tok, −5.1%) |
| Arm B | 25,388 | ~6,347 | 4,959 ch (~1,240 tok, −16.3%) |

### Fence checks (both arms)

- `description:` frontmatter **byte-identical to baseline in both arms** (D6 — out of scope).
- `scripts/` **byte-identical to baseline in both arms** (F1 — out of scope, copied unmodified).
- No file renamed, merged, split, or relocated; no new file (D5). Every cut is a delete-in-place.
- Nothing written into `plugins/` during the pass.
- **Arm A structural invariants verified mechanically** against baseline, per file: heading count
  and heading text identical · code-fence count identical · table-row count identical ·
  checklist-item count identical. Arm A changed prose only.

| Arm A file | headings | fences | table rows | checklist items |
|---|---|---|---|---|
| `SKILL.md` | 35 → 35 | 8 → 8 | 81 → 81 | 10 → 10 |
| `DATA-SENSITIVITY.md` | 10 → 10 | 4 → 4 | 36 → 36 | 11 → 11 |
| `RELATIONSHIP-PATTERNS.md` | 31 → 31 | 18 → 18 | 88 → 88 | 8 → 8 |
| `STATE-MACHINES.md` | 21 → 21 | 12 → 12 | 39 → 39 | 8 → 8 |
| `VALIDATION-RULES.md` | 35 → 35 | 26 → 26 | 98 → 98 | 9 → 9 |

---

## 2. Protected-content reconciliation (step 3 — run before any cut was drafted)

Source read end to end: `.mochiko/strips/patterns-entity-modeling.md` (4 entries). The v0.64.0
guardrails cut already ran a reconciliation against the v0.27.0 survivor ruling; that entry is
the worked precedent this pass follows.

**18 protected elements, in three tiers.**

### Tier 1 — the v0.27.0 `KEPT:` survivor ruling (hardest protection)

`## [v0.27.0] KEPT: the remaining body (under-band survivor ruling, 17% vs 30–70)` — a
survivor-provenance entry with Tier-2 evidence, ratified as wave-3 batch-2 on 2026-07-25. It
enumerates the protected core; the entry groups these as 8 items, atomized here to 10.

| # | Element | Where it lives | Why it was ruled a survivor |
|---|---|---|---|
| P-01 | The ~100-line canonical `data-model.md` template | `SKILL.md:## data-model.md Structure` | "the single canonical template, this wave's untouchable core" |
| P-02 | Four-level sensitivity taxonomy table | `SKILL.md:### Classification Levels` | ownership declared by `DATA-SENSITIVITY.md`'s own header — the v0.27.0 drift repair pointed *into* the SKILL |
| P-03 | Classification decision tree | `SKILL.md:### Classification Decision Tree` | same drift-repair ruling |
| P-04 | PII-maps-onto-the-levels paragraph | `SKILL.md:## Data Sensitivity Classification` | same drift-repair ruling |
| P-05 | The five-step annotation procedure | `SKILL.md:### Annotating Sensitivity` | named explicitly; **re-affirmed in bold** at the v0.64.0 reconciliation |
| P-06 | Conceptual-type vocabulary | `SKILL.md:### Conceptual Types` | `patterns-api-contracts` Type Mapping maps *from* it |
| P-07 | Entity-extraction heuristics | `SKILL.md:### Identification Heuristics` | no other legal residence |
| P-08 | Entity-vs-attribute rules | `SKILL.md:### Entity vs. Attribute Decision` | no other legal residence |
| P-09 | Brownfield status table | `SKILL.md:### Brownfield Entity Status` | no other legal residence |
| P-10 | Validation-script scope paragraph | `SKILL.md:## Validation Script` | names the producer-self-check-vs-independent-review boundary |

*(P-02, P-03 and P-04 are one grouped item in the entry — "the sensitivity taxonomy + decision
tree + PII mapping". Split here so the map below can name a single carrier per rule.)*

### Tier 2 — the v0.64.0 `Kept deliberately` keep-set

`## [v0.64.0] Guardrails cut` is a supersession-by-ruling entry (`DECISIONS.md` 2026-08-11 build
row Wave 2 residual + user rulings 2026-08-10/11). Its **Kept deliberately** field records what
survived a ruled cut. Per `.mochiko/strips/README.md`, that field exists *"because a doctrine
reversal usually spares part of what it touches, and an unrecorded survivor reads to the next
auditor as an oversight"* — so removing a named survivor now would read as exactly the oversight
the field exists to prevent. Treated as protected, at section granularity, one tier below a
`KEPT:` entry.

Beyond the Tier-1 elements it re-names:

| # | Element |
|---|---|
| P-11 | `SKILL.md:## Overview` |
| P-12 | `SKILL.md:## When NOT to Use` |
| P-13 | `SKILL.md:### Standard Attributes` |
| P-14 | `SKILL.md:## Relationship Modeling` / `## State Machine Modeling` / `## Validation Rules` (the three reference pointers) |
| P-15 | `SKILL.md:## Quality Checklist` |
| P-16 | `SKILL.md:## Common Mistakes` |

### Tier 3 — the v0.23.0 ruled artifact form

Not sections but a ruled *form*, binding both `SKILL.md`'s canonical template and
`references/DATA-SENSITIVITY.md` (workflow-token-reduction wave 2, R2 self-containment floor,
ratified 2026-07-24). Reverting either clause — in particular restoring the deleted 7-row
per-attribute aspect table — is a supersession, not a compression.

| # | Element |
|---|---|
| P-17 | Handling-by-level defaults stated **once per document** (the self-containment floor) |
| P-18 | One **Sensitivity Details** row per Confidential+ attribute, columns Level · Retention · Access · Deviations · Compliance |

### What the protected set leaves for Arm B

**Tiers 1 and 2 together name every section of the current `SKILL.md` body.** There is no
unprotected section left in it. Independently of protection, applying the inherited cut line to
`SKILL.md` reaches for only two spans anyway — and **both are Tier-1 hard-protected**:

| Span the cut line would take | Protection | Disposition |
|---|---|---|
| `### Annotating Sensitivity` (step-by-step procedure) | P-05 | **supersession would be required — not attempted** |
| `## Validation Script` (procedure + invocation) | P-10 | **supersession would be required — not attempted** |

Every other `SKILL.md` section the cut line **keeps on its own terms**: `## Overview` is the goal;
`## When NOT to Use` and `## Common Mistakes` are the rejections and anti-patterns;
`## data-model.md Structure` is the output contract; the heuristics, type vocabulary, taxonomy,
decision tree, brownfield table and standard attributes are hard reference data;
`## Quality Checklist` is the obligation list.

**Conclusion, stated plainly: Arm B has zero legal cuts in `SKILL.md`.** Arm B's `SKILL.md` is
therefore **byte-identical to baseline**, and Arm B differs from baseline in the references only.
Arm A's densification was deliberately *not* folded into Arm B — blending the two would destroy
the per-arm attribution D2 exists to buy.

Arm B's legal room is entirely in the references: `RELATIONSHIP-PATTERNS.md`,
`STATE-MACHINES.md` and `VALIDATION-RULES.md` carry **no strip entries at all** (never stripped,
no protected set), and `DATA-SENSITIVITY.md` carries no `KEPT:` ruling of its own — only the
Tier-3 form binds it.

**Supersessions required: 2. Supersessions taken: 0.**

---

## 3. Arm A — lossless densification

Full skill-directory copy at `evals/patterns-entity-modeling/variants/armA/`. Same headings, same
rules, same MUST/SHOULD grading, same example count (verified mechanically, §1). Removed only
restatement, hedging, throat-clearing, duplicated framing, and prose repeating an adjacent table.

| # | File · span | Kind | Chars | Why nothing leaves |
|---|---|---|---:|---|
| A-01 | `SKILL.md:## Overview` sentence 2 | self-TOC | ~180 | Enumerated the section headings that immediately follow. |
| A-02 | `SKILL.md` plan-ladder blockquote, trailing clause | restatement | ~15 | "this skill models the entities that survive it" restated the clause opening the same sentence. |
| A-03 | `SKILL.md` "Look for entities in:" | throat-clearing | ~22 | Lead-in to a table under the heading *Identification Heuristics*. |
| A-04 | `SKILL.md` "When modeling in brownfield projects:" | throat-clearing | ~38 | Lead-in to a table under the heading *Brownfield Entity Status*. |
| A-05 | `SKILL.md:## Data Sensitivity Classification` opening, 2nd half | duplicated framing | ~135 | The DS-XXX declaration boundary is stated in full in `## When NOT to Use` 80 lines above; the ownership claim is kept and re-pointed at that in-file home. |
| A-06 | `SKILL.md:### Annotating Sensitivity` step 2 | restatement | ~15 | "once per document" and "a single time" stated one rule twice inside one sentence. |
| A-07 | `SKILL.md:## Validation Rules` lead-in | throat-clearing | ~55 | "Constraints and validation rules ensure data integrity." — carries no rule. |
| A-08 | `SKILL.md:## data-model.md Structure` preamble, middle sentence | prose repeating an adjacent template | ~255 | Restated `### Annotating Sensitivity` steps 1–3 and the template rendered directly below it; the canonical-template claim, the envelope pointer and "Density is not a gap" all kept. |
| A-09 | `SKILL.md:## Validation Script` parenthetical shape list | in-sentence duplication | ~120 | The same coverage list is stated authoritatively in the closing paragraph of the same section. |
| A-10 | `SKILL.md` "Before finalizing entity model, verify:" | throat-clearing | ~40 | Lead-in to a list under the heading *Quality Checklist*. |
| A-11 | `DATA-SENSITIVITY.md` header sentence 2, 2nd half | self-TOC | ~125 | Enumerated this file's own headings; the load-bearing half — "the taxonomy and decision tree live in the SKILL", cited by the v0.27.0 drift repair — is kept verbatim. |
| A-12 | `DATA-SENSITIVITY.md:## Sensitivity Annotation` preamble | prose repeating an adjacent table | ~95 | Spelled out in prose exactly what the table's own **Carried by** column encodes per row. |
| A-13 | `DATA-SENSITIVITY.md:## Sensitivity Details Row Format` preamble tail | restatement | ~45 | The level-default split is already stated in the Density paragraph and the field-definitions table. |
| A-14 | `DATA-SENSITIVITY.md:## Traceability to Analysis` closing sentence | duplicated framing | ~115 | Third statement of the same ownership split (stated in the sentence above it, and twice in `SKILL.md`). |
| A-15 | `RELATIONSHIP-PATTERNS.md` header enumeration | self-TOC | ~62 | Named the file's own headings. |
| A-16 | `RELATIONSHIP-PATTERNS.md` 1:1 lead-in | prose repeating an adjacent table | ~57 | "optional extensions or large attribute groups" are the first two rows of the *When to Use 1:1* table below it. |
| A-17 | `RELATIONSHIP-PATTERNS.md` self-reference lead-in | restates its heading | ~35 | "When an entity relates to itself." under *Self-Referential Relationships*. |
| A-18 | `STATE-MACHINES.md` ×2 "Each state/transition should document:" | throat-clearing | ~62 | Lead-ins to Column/Purpose tables; the glosses themselves are kept in full. |
| A-19 | `STATE-MACHINES.md` "Include state machines in the data-model.md file:" | restates its heading | ~50 | Under the heading *State Machine in data-model.md*. |
| A-20 | `VALIDATION-RULES.md` header | redundancy | ~11 | "Reference documentation for documenting…". |
| A-21 | `VALIDATION-RULES.md` compound-unique lead-in | restates its heading | ~40 | "When uniqueness spans multiple fields:" under *Compound Unique Constraints*. |

**Arm A rules-lost expectation: 0**, by construction — no heading, table row, code fence,
checklist item, example, or MUST/SHOULD grading changed. The pre-registered Arm A bound is 0, and
any loss the grid finds is a defect in this pass, not a priced trade.

### Honest statement of Arm A headroom found

**−1,597 chars in-scope (−4.3%); −5.1% on the representative invoke.** That is the real ceiling
here, and it is small on purpose.

- The skill is **thrice-touched, not twice**: v0.23.0 collapsed the per-attribute aspect blocks,
  v0.27.0 stripped in-file second copies and repaired reference drift (and then had to record a
  survivor ruling because the body came in *under* the 30–70% band at 17%), and v0.64.0 found
  only one droppable section and said so in writing — *"honest small yield… no forced percentage"*.
  Arm A is the fourth pass over the same prose.
- Densification found **21 spans**, of which the four largest (A-08 ~255, A-01 ~180, A-05 ~135,
  A-11 ~125) are 45% of the whole yield. The remaining 17 average ~50 chars each. The tail is
  genuinely exhausted — what is left is table cells, checklist lines, and code-fenced templates,
  none of which Arm A may touch.
- The reference files are the flattest: `VALIDATION-RULES.md` yields **−0.8%** because it is
  almost entirely tables and fenced format blocks with two-word lead-ins. Its real redundancy
  (`## Validation in data-model.md` re-showing formats already given, and the three worked
  pattern sets) is *example* redundancy, which Arm A's same-example-count fence forbids and Arm B
  is the instrument for.
- **No cut was forced to hit a number.** Two candidates were examined and rejected as
  information-bearing rather than restatement: "Every entity typically needs:" above the standard
  attributes table (carries the R-006 obligation the table's *Required* column alone would not
  assert), and `STATE-MACHINES.md:## State Documentation Components` (looks like restatement of
  the format block, but is the sole *statement* — as opposed to demonstration — of the backticked-
  state and `*`-for-any-state conventions behind R-042).

---

## 4. Arm B — cut line, and the dropped-span → rule map

Full skill-directory copy at `evals/patterns-entity-modeling/variants/armB/`. Cut line inherited
from `validator-scope-and-verbosity` D4: **keep** goal + output contract, non-waivable floors,
anti-patterns and rejections, hard reference data; **drop** step-by-step procedure, worked
examples, restatement — subject to the protected set in §2.

**13 spans dropped, −7,392 chars (−19.9% in-scope), 0 of them in `SKILL.md`.**

| # | Dropped span | Chars | Rule IDs the span carried | Surviving carrier after the cut |
|---|---|---:|---|---|
| B-01 | `DATA-SENSITIVITY.md:## Data Sensitivity Summary` (worked rollup example) | 494 | **R-028 (floor)**, **R-077 (vocab)** | `SKILL.md:### Annotating Sensitivity` step 4 + the canonical template's `## Data Sensitivity Summary *(the coverage index)*` heading and table — both byte-identical to baseline in Arm B |
| B-02 | `RELATIONSHIP-PATTERNS.md:### Common 1:N Examples` | 358 | R-033, R-034 | `### Documentation Format` 1:N block (Foreign Key + On Delete rows) · `## Delete Behavior` table · `## Validation Checklist` |
| B-03 | `RELATIONSHIP-PATTERNS.md:### Common N:M Examples` | 393 | R-036, R-037 | `## Many-to-Many (N:M) Pattern` + its Documentation Format block + `### Join Entity Definition` (kept — see note) |
| B-04 | `RELATIONSHIP-PATTERNS.md:## One-to-One (1:1) Pattern` (whole section) | 687 | R-031 (floor, 1:1 instance only), R-033, R-035, R-038 | `## Relationship Types` table names 1:1 · the 1:N Documentation Format block carries the identical field set · `## Relationship Attributes Table` · `## Delete Behavior` |
| B-05 | `RELATIONSHIP-PATTERNS.md:## Self-Referential Relationships` (whole section) | 680 | **R-039 (`should`) — sole teaching carrier** | `## Validation Checklist` row "Self-referential relationships are clearly marked" only. **The priced loss: taught + checklisted → checklisted only.** |
| B-06 | `STATE-MACHINES.md:## State Documentation Components` | 586 | **R-042 (`format`) — sole statement of the backticked-state and `*`-for-any-state conventions**, R-043 | `## State Machine Format` block *demonstrates* both tables with exact column headers, backticked state values and a `*` row — demonstrated, no longer stated |
| B-07 | `STATE-MACHINES.md:## Common State Patterns` (lifecycle / approval / order-payment) | 952 | none uniquely | R-048 via the format block's `### Diagram` + checklist |
| B-08 | `STATE-MACHINES.md:## Guards and Side Effects` (two example tables) | 613 | R-046, R-047 | the format block's Transitions table carries live Guard and Side Effects values ("User is owner", "Is admin", "Set startedAt", "Log action") + two checklist rows |
| B-09 | `STATE-MACHINES.md:## State Machine in data-model.md` | 196 | none uniquely | R-049 is sourced from `SKILL.md`'s template, unchanged in Arm B |
| B-10 | `VALIDATION-RULES.md:### Compound Unique Constraints` | 416 | R-051 | `### Validation Column Patterns` row `` `Unique(scope)` `` with the `Unique(projectId)` example + checklist |
| B-11 | `VALIDATION-RULES.md:### Custom Patterns` | 355 | none uniquely | R-052 via `### Common Formats` + the `Pattern: regex` row |
| B-12 | `VALIDATION-RULES.md:## Validation in data-model.md` | 780 | R-054, **R-057 (`format`) — partial** | `## Entity-Level Constraints` block + `### Cross-Field Rules`. **Note:** this span held the only `fk_` exemplar; after the cut R-057's uk_/chk_/fk_ convention is exemplified for `uk_` and `chk_` only |
| B-13 | `VALIDATION-RULES.md:## Common Validation Patterns` (3 worked sets) | 882 | none uniquely | R-052, R-053 via Validation Column Patterns + Common Formats + checklist |

### What the grid is expected to stress

Ranked by exposure against the ratified bound (0 floor / 0 `vocab` / ≤ 1 `must` / ≤ 4
`should`+`format`):

1. **B-01 — the only span touching a floor and a vocab rule at once (R-028, R-077).** Both are
   bounded at zero. Judged safe *only* because Arm B leaves `SKILL.md` verbatim, so each keeps a
   first-class carrier there. If the grid loses either, this is the span to revert first.
2. **B-05 — R-039** (`should`), the one deliberate, priced information loss in the arm.
3. **B-06 — R-042** (`format`), demoted from stated to merely demonstrated.
4. **B-12 — R-057** (`format`), partial: the `fk_` naming exemplar is gone.
5. B-02/B-03/B-04/B-08/B-10 carry rules that keep an explicit surviving carrier; expected to hold.
6. B-07/B-09/B-11/B-13 carry no rule uniquely; expected to hold.

Consumed against the bound if 2–4 all land: 0 floor · 0 `vocab` · 0 `must` · 3 of 4
`should`+`format`. **No span was kept in order to protect the bound** — `### Join Entity
Definition` was kept because it is the canonical join-entity *shape* (output contract / hard
reference data), which the cut line keeps on its own terms, not because R-037 sits in a bounded
class.

---

## 5. Eval results

**Pending grid.** To be filled from the runner (`evals/README.md`): four arms — `noskill` control ·
`baseline` · `armA` · `armB` — 3 goldens × 4 arms × 3 replicates = 36 runs, pass^k aggregation,
Haiku checklist judge plus the blind position-swapped Sonnet pairwise read (both advisory).

Preconditions still owed before any priced run: probe run (R5) confirming all 12 scripted
baseline assertions pass. Rules that hold under `noskill` across all replicates are pruned from
the denominator first (R3), and the post-pruning percentage is recorded here.

| Arm | Rules lost (pass^k) | floor | vocab | must | should + format | Pruned by control | Pairwise verdict (position-consistent?) | Judge parse failures | Cost |
|---|---|---|---|---|---|---|---|---|---|
| `noskill` | — | — | — | — | — | — | — | | |
| `baseline` | | | | | | | — | | |
| `armA` | | | | | | | | | |
| `armB` | | | | | | | | | |

## 6. Recommendation per arm

**Pending grid.** Neither arm is recommended until the grid runs against the ratified bar; the
user's ratification (step 8) is the gate, the report is evidence.
