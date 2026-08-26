# Author≠grader audit — `review-specifications` v0.88.0 true-deletion body cut

**Verdict: PASS** (round 1) · Grader: independent validator, authored none of the graded material
· Date: 2026-08-26 · Ceremony: `.claude/rules/mochiko/primitive-edits.md`

All 81 inventory rules survive, both budgets pass, every consumer contract holds, and the
v0.87.0 stale-pointer defect class is genuinely absent rather than merely asserted absent — I
checked it rather than taking the strip's word for it. Four non-blocking advisories are recorded
at the foot.

## 1. Deterministic char-budget pre-assert (D7)

Measured with the canonical snippet in `.mochiko/memory/primitive-cost-budgets.md` (characters of
the parsed value, never `wc -c` bytes).

| Class | Measured | Budget | Delivery cap | Result |
|---|---|---|---|---|
| Skill body | **6,187** | 7,734 | — | PASS |
| `description:` | **490** | 613 | 1,536 | PASS |

Baseline re-measured independently from `git show HEAD:` at **12,184** body / **490**
description. So 12,184 → 6,187 = **−49.22%**, matching the −49.2% recorded in the strip, ADR,
`DECISIONS.md` row and cost ledger. The description is byte-identical to baseline, untouched by
this cut, and sits at exactly its recorded winner figure — no ledger drift on this row.

## 2. Preserved responsibilities — all 81 rules walked

Source of truth: `evals/review-specifications/rules.json`, authored independently of the
compressor from the pre-cut baseline.

**LOST: none. 81/81 have a home.**

This skill has **no `references/` directory**, so there is no reference-homed bucket. Every rule
had to land in the body or behind an explicit single-source pointer:

- **Survives in the body: 76** — R-001 through R-026, R-028 through R-058, R-060, R-062 through
  R-081. The body's six paragraphs carry them: identity and routing (L8), Method with the
  question craft, five user-facing categories and the six-class defect taxonomy (L10), the
  feature layer with the git-baseline rule and all ten checks with severities (L12), Screens &
  Flows with both legal shapes, the serve-and-click floor, the authority split and all eight
  checks (L14), severity plus output (L16), and Floors (L18).
- **Survives behind an explicit single-source pointer: 5** — R-027
  (`../review-brainstorm/references/EXTERNAL-CLAIMS.md` for the external-claim trigger and
  mechanics), R-014 and R-057 (`templates/advocate-report-template.md` for the Clarifications
  shape and the report structure), R-029 (`mochiko:authoring-feature-map` for the map
  machinery), R-059 (`templates/artifact-format.md` for the deliverable envelope).

The five inventory-driven restorations the ADR and strip claim are all present and verified:
R-012 (`never presupposing a mechanism ("should we cache?" assumes caching)`), R-015 (`specific,
never vague`), R-064 (`every user story reviewed for completeness`), R-065 (`every success
criterion checked for measurability`), R-066 (`edge cases hunted per main flow`).

Two rules deserve their working shown, since both could have been mishandled:

- **R-059** is fully homed despite the body dropping the baseline's spec-specific enumeration.
  `templates/artifact-format.md` rule 4 carries the generic default ("list entries (acceptance
  criteria, edge cases, scenarios, impacts, consequences) are one line each"), and the "2–3
  scenarios per story" count lives at `plugins/mochiko/schemas/spec.yaml:11` under
  `authoring-user-stories`. The reviewer-side obligation — hold the spec to the envelope, never
  fault it for conforming — survives in Floors.
- **R-061** cites `artifact-format.md` **rule 8**, and rule 8 is still rule 8 after the v0.82.0
  envelope rewrite: line 54, "Density is not a gap; excess is (the review rule)", carrying the
  advisory-overage clause the body cites. The citation did not go stale.

## 3. Internal coherence

Every outbound pointer in the body resolves. The full set is `mochiko:review-code-minimalism`,
`mochiko:authoring-requirements`, `mochiko:analysis-iterative`, `mochiko:authoring-feature-map`,
`templates/advocate-report-template.md` (twice), `templates/artifact-format.md`, and the relative
link `../review-brainstorm/references/EXTERNAL-CLAIMS.md` — which I resolved through the actual
relative path from the skill directory, not just by checking the file exists elsewhere.

The report-template pointer is honest about what it points at: `verdict:`, the one-line
`strengths:`, `sev: Critical | Important | Minor`, and the
`type: Missing | Ambiguous | EdgeCase | Assumption | Contradiction` enum all exist in the
template, so R-057, R-081 and R-056 are anchored in real content.

No contradiction with the `description:`, which was not touched: it promises gap review of an
already-drafted spec including the feature layer and Screens & Flows, gap-finding input rather
than a clearing verdict, and the `analysis-iterative` routing — all four match the body. The
default independence posture ("Independent reviewer, never the author") and the input-not-verdict
posture ("**input, never a clearing PASS/FAIL verdict of its own**") are both present in the
opening line.

## 4. The v0.87.0 lesson — stale pointers into this skill

This is the defect class that failed the sibling audit, so I checked it directly rather than
accepting the strip's assertion.

**Structurally absent, and confirmed empirically.** A plugin-wide grep for every citation of
`review-specifications` returns eleven hits, and **not one names a section or anchor of this
skill**:

- `agents/devils-advocate.md:52` — the canonical-home anchor. It names three limbs (gap taxonomy,
  severity rubric, structured output format) and points at the skill as a whole, with no section
  reference. All three limbs are resident: the six-class taxonomy at L10, the spec-specific
  severity rubric at L16, the output-format binding at L16. **R-072 holds.**
- `agents/devils-advocate.md:19` and `skills/mochiko/SKILL.md:141` — roster lines, scope
  unchanged.
- `skills/mochiko/SKILL.md:60` — the router row, the densest citation. All eight feature-layer
  names survive (derivation honesty, filter rejections justified, dedup against the actual map at
  the run-open git baseline, entry well-formedness, delta legality, SC re-homing, selection-card
  deferred-SC honesty, specs-index agreement), including the run-open git baseline qualifier, and
  all five Screens & Flows names survive with the served-prototype walk (FLOW walkability, SCR
  reachability, P1 scenario coverage, drift, waiver second-guess). **R-074, R-075, R-076 hold.**
- `skills/authoring-prototype/SKILL.md:41,142`, `skills/authoring-feature-map/SKILL.md:31,106`,
  `skills/analysis-iterative/SKILL.md:3,16`,
  `skills/review-brainstorm/references/EXTERNAL-CLAIMS.md:94`,
  `skills/review-plan-artifacts/SKILL.md:8` — all grader-identity or routing pointers, all still
  true. **R-077, R-078, R-079, R-080 hold.**

I also swept for the deleted section names themselves. Every "Screens & Flows section" hit in
`commands/specify.md`, `commands/plan.md`, `authoring-prototype` and the router refers to the
**spec's** Screens & Flows section, which still exists — not to this skill's deleted heading.
Every "Quality Checklist" hit belongs to a different skill's own section. No citation anywhere
depends on a heading this cut removed.

## 5. Record-layer consistency

The numbers agree across every surface — strip `[v0.88.0]`, the ADR, the `DECISIONS.md` top row
and the cost ledger all carry 12,184 → 6,187, −49.2%, budget 6,187/7,734, shipped v0.88.0, with
the ledger's re-seed paragraph deriving 6,187 × 1.25 = 7,734 correctly.

Every disposition-map claim I checked is true against the actual body, including the ones most
likely to be aspirational: the five category names survive, class 6 keeps its full v0.67.0
calibration clause, the canonical-home clause survives verbatim ("the canonical hunt taxonomy
`devils-advocate` leans on"), the feature layer keeps all ten checks with severities and the R13
git-baseline rule, Screens & Flows keeps all eight checks plus both legal shapes and the
serve-and-click obligation, and the three Related-Skills bullets survive as inline pointers.

The MANDATORY KEPT reconciliation is likewise true. Notably the v0.82.0 entry's
`Kept deliberately: "Density is never itself a gap"` survives verbatim in Floors, and the
v0.88.0 entry is honest about what it supersedes — the v0.26.0 severity and Core Principle
*table forms* and the v0.63.0 keep-set *section forms*, substance intact — rather than claiming
the forms survived.

**The strip's "No dead pointers created" claim is TRUE this time**, verified against the sweep in
section 4 rather than taken on trust. This is the exact sentence that was false in the v0.87.0
sibling pass, so it earned an independent check.

`plugin.json` is at 0.87.0 while the record layer stamps v0.88.0 — correct pre-bump ordering,
since the audit precedes the bump.

## Advisories — none blocking

1. **R-017 partial: the "remove-shaped" label is dropped.** The rule's core survives — the body
   is the canonical home and carries the taxonomy detail the persona leans on — but the
   baseline's internal relation ("the first five are what the product questions above surface;
   the sixth is the remove-shaped excess class") is only half stated. "Hunt the user-facing
   categories … for the six requirement-defect classes" carries the first half; the
   *remove-shaped* label is gone, though class 6's own clause conveys the substance ("no user
   need or ratified driver pays for it … or the cheaper shape"). **No consumer depends on the
   label from this skill** — `review-feasibility` (class 7), `review-governance-intent` and
   `patterns-adopt-first` each carry their own remove-shaped wording. The strip does not falsely
   claim the lead-in survives. Restoring the label costs roughly 15 characters if the lead wants
   the taxonomy's shape relation explicit.
2. **R-032 thinning.** Disposition completeness reads "filter-rejected with the why recorded";
   the baseline said the why is recorded *in the story file*. The location qualifier is gone, the
   obligation is not.
3. **R-063 form change.** The Quality-Checklist-as-section meta-rule dies; all eleven items
   survive individually across Method, the check paragraphs and Floors, and the strip discloses
   the form's death by ruling. Same disposition as R-085 in the v0.87.0 sibling audit.
4. **R-073 wording.** The body says "a drafted spec" and never the literal `spec.md`. No
   consumer contradiction — `devils-advocate:19` and router row 60 name the file themselves, and
   the scope statement does not re-scope.

**Still owed at the bump** (not this audit's gate): the `CHANGELOG.md` 0.88.0 entry (release
gate 4) and the `marketplace.json` sync.

No fix list — nothing blocks this cut from shipping at v0.88.0.
