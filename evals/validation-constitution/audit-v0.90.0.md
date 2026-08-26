# Author≠grader audit — `validation-constitution` v0.90.0 true-deletion body cut

**Verdict: PASS** (round 1, no fix list) · Grader: independent validator, authored none of the
graded material · Date: 2026-08-26 · Ceremony: `.claude/rules/mochiko/primitive-edits.md`

All 69 inventory rules survive, both budgets pass, and the VALIDATION RESULT block survives
field-complete as an enumeration clause — every field and every sub-item, which is the check most
at risk when a fenced template is prosified. This skill has `references/`, so the v0.87.0
reference-to-body defect class was live here; I swept both files directly and found it clean.
Three non-blocking advisories are recorded at the foot.

## 1. Deterministic char-budget pre-assert (D7)

| Class | Measured | Budget | Delivery cap | Result |
|---|---|---|---|---|
| Skill body | **5,103** | 6,379 | — | PASS |
| `description:` | **481** | 602 | 1,536 | PASS |

Baseline re-measured independently from `git show HEAD:` at **7,630** body / **481**
description, so 7,630 → 5,103 = **−33.12%**, matching the recorded −33.1%. The description is
byte-identical to baseline and sits exactly at its recorded winner figure. Budget arithmetic
checks: 5,103 × 1.25 = 6,378.75, rounded up to 6,379.

## 2. Preserved responsibilities — all 69 rules walked

**LOST: none. 69/69 have a home.**

- **Survives in the body: 66** — R-001 through R-018, R-020 through R-038, R-040 through R-038's
  neighbours and on through R-069, across the five paragraphs: identity and scope (L8), Inputs
  (L10), the four-leg Protocol (L12), the VALIDATION RESULT block (L14), and Floors (L16).
- **Survives in an untouched `references/` file or behind an explicit pointer: 3** — R-019
  (`references/QUALITY-CHECKLIST.md`, whose items are the graded surface and are correctly not
  restated), R-039 (`references/ANTI-PATTERNS.md`, carrying *Vague Principle*, *Generic
  Thresholds* and *Missing Enforcement* — all three verified present in that file), R-058 (the
  `mochiko:authoring-constitution` producer pointer).

Both reference files are confirmed untouched by `git status`, so they are valid single sources.

### The VALIDATION RESULT block — field-complete

This was the highest-risk conversion in the cut, so I checked it field by field rather than
sampling. The fenced template became one enumeration clause, and nothing was dropped:

- R-029 header verdict line `VALIDATION RESULT: PASS/FAIL` — present.
- R-030 checklist count with module-fragment accounting and names — present.
- R-031 **Surface integrity, all six sub-items** — region markers · index→home resolution · rules
  files paths-scoped · scope coverage · new-file read line present/absent/n-a ·
  universal-in-rules violations. Six of six.
- R-032 **Trace closure, all four sub-items** — manifest rows closed with the primary-home +
  companions qualifier · synthesis elements realized-or-flagged · waivers matched · modules
  matched. Four of four, qualifier intact.
- R-033 **Floor/module accounting, all three sub-items** — floor + declared level asserted with
  the region-stamp-equals-ledger equality check · modules matched to the fact profile · floor
  categories principled at the declared level or waived. Three of three, equality check intact.
- R-034 anti-patterns found (list or "none") · R-035 version bump · R-036 issues requiring fix
  with each failure listed · R-037 the advisory judgment-grade non-blocking line carrying
  suspected trace-fidelity mismatches — all present.

R-043 also holds: excess findings still land in the `Anti-patterns found` line rather than being
routed to a separate section or to the advisory line.

The one restoration the ADR and strip claim is present and verified: R-047, the red-flag
STOP-and-restart meta-rule with its named rationalization family. Its target was correctly
re-pointed — the baseline said "restart validation from Step 1", and since the numbered steps
were deleted the body now reads "STOP and restart from checklist assembly", which is what Step 1
was.

## 3. Internal coherence

Both `references/` pointers resolve. `templates/constitution-modules/*.md` is a real path with
four module files (`evolution-notes`, `knowledge-management`, `layer-rules`, `release-gates`).
The region marker strings survive verbatim — `<!-- mochiko:governance:begin -->` /
`<!-- mochiko:governance:end -->` — and match `QUALITY-CHECKLIST.md:47` and
`schemas/governance-surfaces.yaml:43` exactly, so R-012's literal-marker obligation is anchored
in real content on both sides.

The FAIL posture is present ("binary PASS or FAIL — no soft language, no middle ground", plus the
description's "defaults to FAIL"), as is author≠grader ("never co-mounted; the validator is a
different agent").

Two router-cited names deserve their working shown, since neither appears verbatim in the body:
**"three-part structure"** and **"placeholder scans"** (R-062, R-065). Both are homed. The
three-member set is named in the opening line and graded as one deliverable (R-001), and
`QUALITY-CHECKLIST.md` carries `## Structure Quality — universal core (the surface set)` and
`## Structure Quality — selected modules` as the graded items. The placeholder scan is named
twice in Floors ("placeholders = incomplete, return for completion" and "'I didn't use
placeholders' (the scan runs against the files regardless)") and owns a whole reference section,
`## No Placeholders Rule`. Neither router name is orphaned.

## 4. Stale pointers, both directions — the v0.87.0 defect class

**Clean, and this is the check that mattered most here**, since unlike the sibling passes this
skill has references that could point back at deleted steps.

A grep of both `references/QUALITY-CHECKLIST.md` and `references/ANTI-PATTERNS.md` for `SKILL.md`,
any `Step N`, `Core Process`, `Quantification Requirements`, `Red Flags`,
`Common Rationalizations`, `Common Mistakes`, `Related Skills`, `Overview` and `When NOT`
**returns zero hits**. The references never cited the body's step numbers or section names, so
deleting them stranded nothing. The defect that failed the v0.87.0 pass is genuinely absent, not
merely asserted absent.

Outbound, every citation of this skill elsewhere is skill-level with no section anchor:
`agents/validator.md:9,28`, `skills/mochiko/SKILL.md:31,43,45,146`,
`authoring-constitution/SKILL.md:34,130`, `analysis-codebase/SKILL.md:121`,
`testing-governance-injection/SKILL.md:3,23`,
`review-feasibility/references/FEASIBILITY-LENS.md:140,183`,
`review-plan-artifacts/SKILL.md:8`, `review-governance-intent/SKILL.md:10`, and
`schemas/governance-surfaces.yaml:4,14`. Each contract still holds: the validator persona's
mounted member issuing an authoritative binary grade (R-060, R-061, R-063); the producer↔validator
pair with grading an existing set routed here (R-064); trace-closure, structure and placeholder
grading staying this skill's against the injection probe's boundary clause (R-065); the
downstream Tier-2 jurisdiction against the intent reviewer, with enforceability graded here
(R-066); well-formed-governance as this skill's domain against both plan-side clauses (R-068);
and the schema's three bound facts — set graded as one deliverable, manifest as the validator's
grading surface, `--check` serving this checklist (R-069).

## 5. Record-layer consistency

Strip `[v0.90.0]`, the ADR, the `DECISIONS.md` row and the cost ledger all carry 7,630 → 5,103,
−33.1%, budget 5,103/6,379, re-seeded from 6,734/8,418, shipped v0.90.0. Every disposition-map
claim checks out against the body — notably the Step 9 fenced block mapping, which I verified
field by field above rather than accepting. The MANDATORY KEPT reconciliation is honest that the
v0.25.0 Red Flags / Common Rationalizations *table forms* end by this ruling while each distinct
rule survives as a Floors clause.

`plugin.json` is at 0.88.0 — correct pre-bump ordering for a v0.90.0 landing.

## Advisories — none blocking

1. **R-058 partial.** The producer pointer survives ("Producer side: `mochiko:authoring-constitution`
   (never co-mounted; the validator is a different agent)") but its greenfield-mode /
   brownfield-mode detail is dropped. That detail is single-sourced in the pointed-at skill's own
   description ("The single governance-authoring skill for BOTH greenfield and brownfield
   projects"), so it is homed, not lost.
2. **R-053 merged.** "Validation would be redundant since I wrote it carefully" no longer appears
   as its own clause; it was a near-duplicate of R-049, whose surviving clause ("authoring mode ≠
   validation mode: 'I reviewed it while writing' is not validation, fresh review catches blind
   spots") delivers R-053's evidence test that authoring care never discharges a check.
3. **Strip bookkeeping imprecision.** The declined-deeper-cut clause says ~−55% would kill "the 14
   VALIDATION-RESULT field rules". The VALIDATION RESULT block carries ten field rules
   (R-028–R-037); fourteen is the count of all format-class rules in the inventory. This describes
   a hypothetical that was declined, not the shipped state, so nothing in the record is false
   about what survives — but the figure would mislead a later reader of the strip.
4. **Still owed at the bump:** the `CHANGELOG.md` 0.90.0 entry (release gate 4) and the
   `marketplace.json` sync.

No fix list — nothing blocks this cut from shipping at v0.90.0.
