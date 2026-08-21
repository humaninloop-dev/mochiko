# Pre-registration — `patterns-entity-modeling`

> **RATIFIED 2026-08-22** — user ruled "ratify" on the proposal as drafted: 9-rule floor set
> (R-041 stays `must`, not promoted), Arm A bound 0, Arm B bound ≤5 of 73 non-floor / 0 vocab /
> ≤1 must, representative invoke = body + 3 references. Bar drafted by the non-compressor
> inventory seat; ratification is the user's (compressor never sets its own bar).

**Skill under test:** `plugins/mochiko/skills/patterns-entity-modeling/`
**Inputs:** `rules.json` (82 rules) · `evals.json` (3 goldens: veterinary scheduling · freight
shipment tracking · expense reimbursement)
**Arms:** `noskill` · `baseline` · `armA` · `armB` — 3 goldens × 4 arms × 3 replicates = 36 runs
(D8 as amended). Session model: Sonnet (R7). Aggregation: pass^k.
**Authorship:** rules and goldens authored by a non-compressor seat (D4/D8 author≠grader).

---

## 1. Rule inventory shape

| Class | Count | What it is |
|-------|-------|------------|
| `floor` | 9 | Absolute. One lost floor rule kills the arm. |
| `must` | 37 | Graded obligations the skill asserts as MUST-equivalent. |
| `format` | 18 | Structural/table/heading obligations of `data-model.md`. |
| `should` | 9 | Graded obligations the skill asserts as SHOULD-equivalent. |
| `vocab` | 9 | Owned vocabulary a **named downstream primitive cites** (R10 consumer check). |
| **Total** | **82** | |

## 2. Proposed floor set (absolute — one lost kills the arm)

| ID | Obligation |
|----|------------|
| R-001 | Entity coverage — every domain noun in the brief is accounted for as entity, attribute, or relationship. |
| R-008 | Conceptual types only — no SQL/DDL types, no physical schema. |
| R-009 | Every entity has an Attributes table carrying Type · Required · Sensitivity · Description. |
| R-013 | Every attribute carries exactly one of Public / Internal / Confidential / Restricted. |
| R-014 | Credentials, government identifiers, payment card numbers, and health records classify **Restricted**. |
| R-020 | Handling-by-level defaults matrix stated exactly once per document (the self-containment floor). |
| R-021 | Every Confidential+ attribute has exactly one Sensitivity Details row. |
| R-028 | `Data Sensitivity Summary` table present at the top, covering every Confidential+ attribute — the artifact's ID/coverage index. |
| R-031 | Every relationship states cardinality **and** delete behavior. |

**Why these nine.** They are anchored, not chosen freely: five map onto the `Critical` rows of the
independent consumer checklist (`review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` →
Entity coverage · Attribute completeness · Relationship definition · PII identification ·
Sensitivity details); two are stated as floors in the skill's own text (the once-per-document
handling matrix is named "the self-containment floor" in `DATA-SENSITIVITY.md`; the summary table
is named "the coverage index" in `SKILL.md`); R-008 is the skill's identity claim ("conceptual
entities, not OpenAPI schemas" — lose it and the artifact is a different artifact); R-014 is the
one classification error with a security consequence rather than a quality consequence.

**Deliberately NOT floor, flagged for the user:** R-041 (every stateful entity has a documented
state machine). It is `Important`, not `Critical`, in the consumer checklist, so it sits at `must`
— but it is the strongest promotion candidate if the user wants a tenth floor, because
`testing-gap-finding` consumes those state machines to derive illegal-transition probes (R-081).

## 3. Proposed rules-lost bound (non-floor)

**One sentence:** Arm B may lose at most **5 of the 73 non-floor rules (≈6.8%)**, none of them
`vocab`, because the inherited decision threshold is 10% (F4) and pass^k already biases toward
declaring loss, so a bound below the threshold leaves margin for replicate flakiness without
letting a real regression pass.

Bound by class:

| Arm | `floor` | `vocab` | `must` | `should` + `format` | Total non-floor |
|-----|---------|---------|--------|---------------------|-----------------|
| `armA` (lossless densification) | 0 | 0 | 0 | 0 | **0** |
| `armB` (inherited cut line) | 0 | 0 | ≤ 1 | ≤ 4 | **≤ 5 of 73** |

- **Arm A is bounded at zero by construction**, not by generosity: D2 defines Arm A as removing
  only restatement, hedging and throat-clearing, with zero information leaving. Any Arm A rule
  loss is a defect in the pass, not a priced trade — it fails the arm and the pass is redone.
- **`vocab` losses are bounded at zero in both arms.** A lost vocab term does not degrade the
  artifact, it breaks a named downstream primitive (R10: "paraphrase breaking a consumer is a
  failed rule"). Those consumers are enumerated per rule in `rules.json`.
- **Pruning happens first (R3):** any rule that passes in the `noskill` control across all
  replicates measures the model, not the skill, and is struck from the denominator before the
  bound is applied. The 73 is the pre-pruning denominator; the ratified bar should be read as
  "≤ 5 lost", with the percentage recomputed post-pruning and recorded in the run report.

## 4. Delivered-chars arithmetic (R9)

Measured with `python len(path.read_text())` — chars, never `wc -c` bytes (F4/D7).

| Surface | Chars | Loaded when |
|---------|-------|-------------|
| `SKILL.md` frontmatter `description:` | 497 | **always** (out of scope, D6) |
| `SKILL.md` body | 13,712 | on invoke |
| `SKILL.md` whole file | 14,261 | on invoke |
| `references/DATA-SENSITIVITY.md` | 6,374 | on demand |
| `references/RELATIONSHIP-PATTERNS.md` | 5,643 | on demand |
| `references/STATE-MACHINES.md` | 4,069 | on demand |
| `references/VALIDATION-RULES.md` | 6,845 | on demand |
| **In-scope total (body + 4 references)** | **37,192** | |

`scripts/` is out of scope (F1) and excluded from every figure above.

**What one invoke delivers today.** Three honest cases, because references are on-demand rather
than auto-loaded. The proposed **representative invoke** is the middle row — it is what these
three goldens actually exercise (each models entities, relationships, a state machine and
sensitivity; only the validation-rules fan-out is optional):

| Representative invoke | Chars | ≈ tokens (chars/4) | −20% saves | −40% saves |
|-----------------------|-------|--------------------|-----------|-----------|
| Body only (no reference read) | 14,261 | ~3,565 | 2,852 ch (~713 tok) | 5,704 ch (~1,426 tok) |
| **Body + DATA-SENSITIVITY + RELATIONSHIP-PATTERNS + STATE-MACHINES (proposed)** | **30,347** | **~7,587** | **6,069 ch (~1,517 tok)** | **12,139 ch (~3,035 tok)** |
| Full fan-out (body + all four references) | 37,192 | ~9,298 | 7,438 ch (~1,860 tok) | 14,877 ch (~3,719 tok) |

**The honest note (R9, kept).** The always-loaded surface is the 497-char `description`, and D6
puts it out of scope; bodies load only on invoke. So the per-invoke saving above is real but
bounded, and the pilots are twice-compressed already (F1) — Arm A headroom here is a fraction of
a never-stripped skill's. The deliverable's durable value is the instrument, not this byte
harvest. Nothing in this arithmetic authorizes a cut; it prices one.

## 5. Disclosures — obligations NOT encoded as rules

Named rather than silently dropped:

1. **`python scripts/validate-model.py` producer self-check** (`SKILL.md:Validation Script`).
   Verifiable only from a transcript, never from the artifact; scripts are out of scope (F1).
   Not encoded.
2. **Pre-existing consumer mismatch — `## Entities`.** `review-plan-artifacts/scripts/check-artifacts.py`
   `REQUIRED_SECTIONS` demands `## Entities`, `## Relationships`, `## Validation Rules` in
   `data-model.md`. The baseline template emits `## Entity Summary` + `## Entity: <Name>` and
   **never `## Entities`** — so baseline already fails that consumer check. Encoding it would
   produce a rule that fails in every arm and measures nothing. R-079 therefore covers only the
   two headings baseline does satisfy. **The mismatch itself is a real defect in the shipped
   library, unrelated to compression — raise it separately.**
3. **Entity names in backticks.** The same script's `ENTITY_PATTERN` (`` `[A-Z][a-zA-Z0-9]+` ``)
   expects backticked entity names; the baseline template does not backtick them. R-080 was
   rewritten to test cross-table name *consistency* (which baseline does satisfy, and which the
   `ARTIFACT-CHECKLISTS` cross-artifact rows actually consume) rather than the backtick form.
4. **Envelope size guidance** (`artifact-format.md` rule 4, ≤3-line prose defaults). The envelope
   itself says it is "reported, never graded" and that "density is not a gap" (rule 8). Encoding
   it would hand the judge a prose-volume dimension, which is exactly what rule 8 forbids — and
   would bias the eval toward Arm B. Not encoded.
5. **External-claims disclosure** (`artifact-format.md` rule 12) — fires only on floor-class
   external claims; none of the three goldens supplies one. Not encoded.
6. **The plan-ladder gate** (`SKILL.md` blockquote → `mochiko:patterns-plan-minimalism`) — the
   entity-necessity stops are disclosed in the *plan proposal*, not in `data-model.md`. Not
   artifact-verifiable. Not encoded.
7. **Brownfield-only rules will run vacuous.** R-004/R-005/R-063 (`[EXTENDS EXISTING]`,
   `[RENAMED]`, the Existing-Attributes split) need a brownfield context the three greenfield
   goldens do not supply; expect `[NEW]` throughout. The judge should mark them not-applicable,
   not failed. **If the user wants brownfield status genuinely measured, a fourth golden carrying
   an existing schema is the fix** — flagged, not assumed.
8. **R-078 (`DS-XXX` citation) is conditionally vacuous** for the same reason: no golden supplies
   an upstream DS-XXX requirement. The rule text says so explicitly.
9. **Assertion brittleness is settled by the probe, not here.** The 12 scripted assertions per
   golden are a *baseline conformance contract* — the runner exits nonzero on a failed scripted
   assertion in `baseline`. Heading assertions were deliberately loosened (`#{2,3}`, bare phrase
   matches) to cut false alarms, but the probe run (R5) must confirm all 12 pass on baseline
   before any priced grid.

## 6. Ratification

The user rules on: **(a)** the nine-rule floor set (§2), **(b)** the ≤5 non-floor / 0 vocab /
0 Arm A bound (§3), **(c)** the representative invoke for the R9 arithmetic (§4, middle row
proposed). On ruling, this file is renamed `preregistration.md` and no value in it changes
after the first priced run.
