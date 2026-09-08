# Re-key record — `evals/review-plan-artifacts/rules.json`

```json
{
  "rekeyed": {
    "at": "2026-09-08T23:23:18Z",
    "ruling": "primitive-eval-harness-v2 D12/C3 (R2: five skills)",
    "source": ".mochiko/schema-views/skills/review-plan-artifacts.yaml + mochiko-cli floors line"
  }
}
```

`rules.json` is a bare list the runner (`evals/run.py`) iterates directly (`for r in rules`, `len(rules)`, `r["id"]`, `r.get("class")`), so the stamp, the retired list, and the mapping live here rather than in the file. Every entry keeps `id` / `rule` / `class` / `source`; `section`, `kind`, `when`, `pointer`, `labels`, and `mapped_from` are additive.

## Counts

- Rules: 113 → 36 (log rules; every id in the derived view appears exactly once).
- Floors: 11 → 11 — equals the render's `floors:` line and its `class: floor · 11 rules` pin.
- Retired: 2. Added (no old counterpart): 0.

## Floors

Old (`SKILL.md`-keyed): `R-001`, `R-002`, `R-003`, `R-004`, `R-010`, `R-011`, `R-012`, `R-060`, `R-063`, `R-082`, `R-093`

New (log-keyed, = render `floors:`): `review-plan-artifacts.author-grader`, `review-plan-artifacts.gap-list-scope`, `review-plan-artifacts.never-shrink`, `review-plan-artifacts.tier1-preassert`, `review-plan-artifacts.conformance-blocking`, `review-plan-artifacts.material-divergence-autofail`, `review-plan-artifacts.default-fail`, `review-plan-artifacts.letter-is-spirit`, `review-plan-artifacts.na-justified`, `review-plan-artifacts.critical-blocks`, `review-plan-artifacts.evidence-floor`

## Class changes (old entry → log rule)

| old id | old class | log rule | log class |
|---|---|---|---|
| R-104 | vocab | `review-plan-artifacts.author-grader` | floor |
| R-107 | vocab | `review-plan-artifacts.feasibility-handoff` | must |
| R-015 | format | `review-plan-artifacts.not-for` | must |
| R-016 | format | `review-plan-artifacts.not-for` | must |
| R-091 | must | `review-plan-artifacts.never-shrink` | floor |
| R-094 | must | `review-plan-artifacts.never-shrink` | floor |
| R-095 | must | `review-plan-artifacts.never-shrink` | floor |
| R-096 | must | `review-plan-artifacts.never-shrink` | floor |
| R-098 | must | `review-plan-artifacts.never-shrink` | floor |
| R-069 | format | `review-plan-artifacts.incremental-consistency-scope` | must |
| R-107 | vocab | `review-plan-artifacts.boundary-table` | must |
| R-019 | format | `review-plan-artifacts.completeness-supplied-sets` | must |
| R-103 | vocab | `review-plan-artifacts.completeness-supplied-sets` | must |
| R-061 | format | `review-plan-artifacts.tier1-preassert` | floor |
| R-102 | vocab | `review-plan-artifacts.cycle-card-checks` | must |
| R-109 | vocab | `review-plan-artifacts.cycle-card-checks` | must |
| R-051 | format | `review-plan-artifacts.cycle-card-check-mirror` | must |
| R-071 | format | `review-plan-artifacts.incremental-read-bound` | must |
| R-102 | vocab | `review-plan-artifacts.store-delta-checklists` | must |
| R-110 | vocab | `review-plan-artifacts.design-cross-checklists` | must |
| R-111 | vocab | `review-plan-artifacts.design-cross-checklists` | must |
| R-100 | vocab | `review-plan-artifacts.conformance-blocking` | floor |
| R-101 | vocab | `review-plan-artifacts.conformance-blocking` | floor |
| R-100 | vocab | `review-plan-artifacts.material-divergence-autofail` | floor |
| R-004 | floor | `review-plan-artifacts.adopt-first-lens` | must |
| R-105 | vocab | `review-plan-artifacts.adopt-first-lens` | must |
| R-007 | format | `review-plan-artifacts.rung-honesty-advisory` | must |
| R-101 | vocab | `review-plan-artifacts.rung-honesty-advisory` | must |
| R-106 | vocab | `review-plan-artifacts.rung-honesty-advisory` | must |
| R-108 | vocab | `review-plan-artifacts.rung-honesty-advisory` | must |
| R-081 | format | `review-plan-artifacts.mechanical-verdict` | must |
| R-097 | must | `review-plan-artifacts.na-justified` | floor |
| R-092 | must | `review-plan-artifacts.critical-blocks` | floor |
| R-058 | vocab | `review-plan-artifacts.severity-classification` | must |
| R-059 | format | `review-plan-artifacts.severity-classification` | must |
| R-081 | format | `review-plan-artifacts.verdict-criteria` | must |
| R-084 | vocab | `review-plan-artifacts.verdict-criteria` | must |
| R-059 | format | `review-plan-artifacts.issue-templates` | must |
| R-089 | format | `review-plan-artifacts.report-template` | must |
| R-099 | format | `review-plan-artifacts.report-template` | must |
| R-112 | vocab | `review-plan-artifacts.report-template` | must |
| R-076 | format | `review-plan-artifacts.incremental-report` | must |
| R-077 | format | `review-plan-artifacts.incremental-report` | must |
| R-078 | format | `review-plan-artifacts.incremental-report` | must |
| R-113 | vocab | `review-plan-artifacts.incremental-report` | must |
| R-061 | format | `review-plan-artifacts.tier1-forms-envelope` | must |
| R-062 | format | `review-plan-artifacts.tier1-forms-envelope` | must |

## Mapping (log rule ← old entries)

| log rule | class | mapped_from | note |
|---|---|---|---|
| `review-plan-artifacts.author-grader` | floor | R-104 |  |
| `review-plan-artifacts.gap-list-scope` | floor | R-001 | R-001's standard ('the approved artifact proposal') is superseded by 'the sufficiency report's gap list' (anchor 2026-08-26 plan-stage-utility). |
| `review-plan-artifacts.feasibility-handoff` | must | R-013, R-088, R-107 |  |
| `review-plan-artifacts.not-for` | must | R-014, R-015, R-016, R-017 |  |
| `review-plan-artifacts.never-shrink` | floor | R-011, R-091, R-094, R-095, R-096, R-098 | R-011's size/urgency limb and R-094–R-098 (the rationalization rows) fold here; R-011's letter/spirit limb is letter-is-spirit. |
| `review-plan-artifacts.incremental-consistency-scope` | must | R-064, R-066, R-067, R-069, R-070 |  |
| `review-plan-artifacts.no-prior-waiver` | must | R-091 |  |
| `review-plan-artifacts.escalate-full-reread` | must | R-072 |  |
| `review-plan-artifacts.boundary-table` | must | R-107 |  |
| `review-plan-artifacts.completeness-supplied-sets` | must | R-009, R-018, R-019, R-103 |  |
| `review-plan-artifacts.tier1-preassert` | floor | R-060, R-061, R-063 |  |
| `review-plan-artifacts.cycle-card-checks` | must | R-042, R-043, R-044, R-045, R-046, R-047, R-048, R-049, R-050, R-054, R-102, R-109 |  |
| `review-plan-artifacts.cycle-card-check-mirror` | must | R-051 | R-051's raw-Read fallback to schemas/tasks.yaml is gone — no schema file ships (v0.107.0). |
| `review-plan-artifacts.incremental-read-bound` | must | R-068, R-071 |  |
| `review-plan-artifacts.analysis-checklists` | must | R-020, R-021, R-022, R-023, R-024, R-025 | R-020–R-025's row-level checks are bound by pointer; the log states the reference 'carries no requirements.md checklist', so the FR→TR row naming may differ in the file. |
| `review-plan-artifacts.store-delta-checklists` | must | R-026, R-027, R-028, R-029, R-030, R-031, R-032, R-033, R-034, R-102 |  |
| `review-plan-artifacts.design-cross-checklists` | must | R-035, R-036, R-037, R-038, R-039, R-040, R-041, R-052, R-053, R-055, R-110, R-111 |  |
| `review-plan-artifacts.conformance-blocking` | floor | R-002, R-100, R-101 | R-002/R-100's 'approved proposal' vocabulary is rebased onto the gap list; BLOCKING survives. |
| `review-plan-artifacts.material-divergence-autofail` | floor | R-003, R-082, R-100 | R-003/R-082/R-100 — 'unproposed artifact' becomes 'an artifact no gap named'. |
| `review-plan-artifacts.adopt-first-lens` | must | R-004, R-005, R-105 | R-004 was a floor; the log carries the adopt-first lens as must (gate) at conformance strength. |
| `review-plan-artifacts.rung-honesty-advisory` | must | R-006, R-007, R-008, R-083, R-101, R-106, R-108 | R-108's 'hunt class 7' number is not in the log text; the excess hunt is named as review-feasibility's without a number. |
| `review-plan-artifacts.mechanical-verdict` | must | R-080, R-081 |  |
| `review-plan-artifacts.default-fail` | floor | R-093 |  |
| `review-plan-artifacts.letter-is-spirit` | floor | R-011, R-012 |  |
| `review-plan-artifacts.na-justified` | floor | R-097 |  |
| `review-plan-artifacts.critical-blocks` | floor | R-092 |  |
| `review-plan-artifacts.severity-classification` | must | R-058, R-059, R-090 |  |
| `review-plan-artifacts.verdict-criteria` | must | R-081, R-084 |  |
| `review-plan-artifacts.issue-templates` | must | R-059 |  |
| `review-plan-artifacts.report-template` | must | R-086, R-087, R-089, R-099, R-112 |  |
| `review-plan-artifacts.incremental-report` | must | R-076, R-077, R-078, R-079, R-113 |  |
| `review-plan-artifacts.evidence-floor` | floor | R-010 |  |
| `review-plan-artifacts.tier1-forms-envelope` | must | R-061, R-062 | R-062's five pre-assert check families ride the pointer to ARTIFACT-CHECKLISTS.md; the density clause is new. |
| `review-plan-artifacts.caller-names-sets` | must | R-057, R-065 | R-057 (sequencing is the lead's call) is generalized into the incremental-mode reservation. |
| `review-plan-artifacts.contradiction-routing` | must | R-073, R-074 |  |
| `review-plan-artifacts.unsure-targeted-review` | must | R-075 |  |

## Retired (old entries with no log counterpart)

- **R-056** (must; was `SKILL.md:Review Focus by Artifact Type (blockquote: Brownfield codebase-discovery review i`): cut — brownfield codebase-discovery review out of scope; not in the log's not-for list and not in SKILL.md prose.
- **R-085** (must; was `SKILL.md:Quality Checklist (Before finalizing the review, verify:)`): cut — the Quality Checklist as a pre-finalization self-check went with the section.

## Invariants verified

- `class: floor` id set in the file == render `floors:` line (order and membership) and == the `class: floor` pin count.
- Every rule id in `.mochiko/schema-views/skills/review-plan-artifacts.yaml` appears exactly once; the view's id list equals the live render's id list.
- No id appears twice; every old `R-XXX` is either mapped (possibly to more than one log rule) or retired, never both, never dropped.
- `evals/run.py` fields preserved: `id`, `class` (read), `rule` and the rest passed through to the judge as JSON.
