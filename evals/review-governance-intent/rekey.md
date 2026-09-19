# Re-key record — `evals/review-governance-intent/rules.json`

```json
{
  "rekeyed": {
    "at": "2026-09-08T23:23:18Z",
    "ruling": "primitive-eval-harness-v2 D12/C3 (R2: five skills)",
    "source": ".mochiko/schema-views/skills/review-governance-intent.yaml + mochiko-cli floors line"
  }
}
```

`rules.json` is a bare list the runner (`evals/run.py`) iterates directly (`for r in rules`, `len(rules)`, `r["id"]`, `r.get("class")`), so the stamp, the retired list, and the mapping live here rather than in the file. Every entry keeps `id` / `rule` / `class` / `source`; `section`, `kind`, `when`, `pointer`, `labels`, and `mapped_from` are additive.

## Counts

- Rules: 70 → 35 (log rules; every id in the derived view appears exactly once).
- Floors: 11 → 16 — equals the render's `floors:` line and its `class: floor · 16 rules` pin.
- Retired: 4. Added (no old counterpart): 0.

## Floors

Old (`SKILL.md`-keyed): `R-001`, `R-002`, `R-008`, `R-011`, `R-012`, `R-015`, `R-040`, `R-042`, `R-043`, `R-044`, `R-057`

New (log-keyed, = render `floors:`): `review-governance-intent.never-a-participant`, `review-governance-intent.author-grader`, `review-governance-intent.authored-surfaces-out`, `review-governance-intent.formulation-quality-excluded`, `review-governance-intent.its-command-states-them`, `review-governance-intent.default-fail`, `review-governance-intent.too-thin-first-finding`, `review-governance-intent.contested-audit-first`, `review-governance-intent.echo-rationales-outrank`, `review-governance-intent.declared-level-discipline`, `review-governance-intent.yardstick-never-taste`, `review-governance-intent.no-in-session-confirmation`, `review-governance-intent.evidence-floor`, `review-governance-intent.verdict-is-input`, `review-governance-intent.ratification-user-owned`, `review-governance-intent.findings-through-leads-pen`

## Class changes (old entry → log rule)

| old id | old class | log rule | log class |
|---|---|---|---|
| R-005 | must | `review-governance-intent.never-a-participant` | floor |
| R-062 | vocab | `review-governance-intent.never-a-participant` | floor |
| R-067 | vocab | `review-governance-intent.never-a-participant` | floor |
| R-070 | vocab | `review-governance-intent.author-grader` | floor |
| R-015 | floor | `review-governance-intent.sequestration` | must |
| R-068 | vocab | `review-governance-intent.sequestration` | must |
| R-001 | floor | `review-governance-intent.pre-ratification-timing` | must |
| R-058 | vocab | `review-governance-intent.pre-ratification-timing` | must |
| R-060 | vocab | `review-governance-intent.pre-ratification-timing` | must |
| R-070 | vocab | `review-governance-intent.pre-ratification-timing` | must |
| R-008 | floor | `review-governance-intent.lens-depth-never-jurisdiction` | must |
| R-061 | vocab | `review-governance-intent.lens-depth-never-jurisdiction` | must |
| R-063 | vocab | `review-governance-intent.lens-depth-never-jurisdiction` | must |
| R-008 | floor | `review-governance-intent.report-out-of-lens-trips` | must |
| R-061 | vocab | `review-governance-intent.solo-and-verify-routing` | must |
| R-013 | must | `review-governance-intent.authored-surfaces-out` | floor |
| R-014 | must | `review-governance-intent.formulation-quality-excluded` | floor |
| R-046 | format | `review-governance-intent.its-command-states-them` | floor |
| R-002 | floor | `review-governance-intent.frozen-window` | must |
| R-060 | vocab | `review-governance-intent.frozen-window` | must |
| R-017 | format | `review-governance-intent.read-set-binding` | must |
| R-069 | vocab | `review-governance-intent.read-set-binding` | must |
| R-026 | format | `review-governance-intent.cross-exam-binding` | must |
| R-064 | vocab | `review-governance-intent.cross-exam-binding` | must |
| R-064 | vocab | `review-governance-intent.substrate-bindings` | must |
| R-065 | vocab | `review-governance-intent.reality-facts-checked` | must |
| R-065 | vocab | `review-governance-intent.user-facts-flagged` | must |
| R-032 | format | `review-governance-intent.external-facts-binding` | must |
| R-065 | vocab | `review-governance-intent.external-facts-binding` | must |
| R-066 | vocab | `review-governance-intent.external-facts-binding` | must |
| R-059 | vocab | `review-governance-intent.status-vocabulary-and-criteria` | must |
| R-041 | must | `review-governance-intent.too-thin-first-finding` | floor |
| R-047 | must | `review-governance-intent.contested-audit-first` | floor |
| R-048 | must | `review-governance-intent.echo-rationales-outrank` | floor |
| R-050 | must | `review-governance-intent.declared-level-discipline` | floor |
| R-051 | must | `review-governance-intent.declared-level-discipline` | floor |
| R-052 | must | `review-governance-intent.declared-level-discipline` | floor |
| R-053 | must | `review-governance-intent.declared-level-discipline` | floor |
| R-054 | must | `review-governance-intent.yardstick-never-taste` | floor |
| R-069 | vocab | `review-governance-intent.yardstick-never-taste` | floor |
| R-056 | must | `review-governance-intent.no-in-session-confirmation` | floor |
| R-059 | vocab | `review-governance-intent.survivor-report-form` | must |
| R-062 | vocab | `review-governance-intent.survivor-report-form` | must |
| R-045 | must | `review-governance-intent.verdict-is-input` | floor |
| R-063 | vocab | `review-governance-intent.verdict-is-input` | floor |
| R-045 | must | `review-governance-intent.ratification-user-owned` | floor |
| R-067 | vocab | `review-governance-intent.findings-through-leads-pen` | floor |

## Mapping (log rule ← old entries)

| log rule | class | mapped_from | note |
|---|---|---|---|
| `review-governance-intent.never-a-participant` | floor | R-005, R-042, R-062, R-067 |  |
| `review-governance-intent.author-grader` | floor | R-012, R-057, R-070 |  |
| `review-governance-intent.sequestration` | must | R-015, R-068 | R-015 was a floor in the old inventory; the log carries it as must. |
| `review-governance-intent.pre-ratification-timing` | must | R-001, R-058, R-060, R-070 | R-001 was a floor; the log carries timing as must (the cost-asymmetry rationale is gone). |
| `review-governance-intent.lens-depth-never-jurisdiction` | must | R-008, R-061, R-063 | R-008 was a floor; split into this must and report-out-of-lens-trips (duty). |
| `review-governance-intent.report-out-of-lens-trips` | must | R-008 |  |
| `review-governance-intent.solo-and-verify-routing` | must | R-009, R-010, R-061 |  |
| `review-governance-intent.authored-surfaces-out` | floor | R-013 |  |
| `review-governance-intent.formulation-quality-excluded` | floor | R-014 |  |
| `review-governance-intent.never-excess` | must | R-022 |  |
| `review-governance-intent.its-command-states-them` | floor | R-046 | R-046 was a format pointer; the log carries it as a floor (routing) in this skill. |
| `review-governance-intent.frozen-window` | must | R-002, R-060 | R-002 was a floor; the log carries it as must. |
| `review-governance-intent.read-set-binding` | must | R-016, R-017, R-069 | R-016's brownfield limb is its own gated rule (brownfield-analysis-read, when: analysis=present). |
| `review-governance-intent.brownfield-analysis-read` | must | R-016 |  |
| `review-governance-intent.cross-exam-binding` | must | R-024, R-025, R-026, R-064 | when: pairing=pair — R-024's solo-skip clause is the rule's own gate. |
| `review-governance-intent.substrate-bindings` | must | R-027, R-028, R-064 |  |
| `review-governance-intent.reality-facts-checked` | must | R-029, R-065 |  |
| `review-governance-intent.user-facts-flagged` | must | R-030, R-049, R-065 |  |
| `review-governance-intent.external-facts-binding` | must | R-031, R-032, R-065, R-066 |  |
| `review-governance-intent.unresolvable-is-commentary` | must | R-019 |  |
| `review-governance-intent.over-governance-admissibility` | must | R-020, R-021, R-023 |  |
| `review-governance-intent.status-vocabulary-and-criteria` | must | R-036, R-037, R-038, R-039, R-059 |  |
| `review-governance-intent.default-fail` | floor | R-040 |  |
| `review-governance-intent.too-thin-first-finding` | floor | R-041 |  |
| `review-governance-intent.contested-audit-first` | floor | R-047 |  |
| `review-governance-intent.echo-rationales-outrank` | floor | R-048 |  |
| `review-governance-intent.declared-level-discipline` | floor | R-050, R-051, R-052, R-053 |  |
| `review-governance-intent.yardstick-never-taste` | floor | R-054, R-069 |  |
| `review-governance-intent.no-in-session-confirmation` | floor | R-056 |  |
| `review-governance-intent.finding-contract` | must | R-018, R-055 |  |
| `review-governance-intent.survivor-report-form` | must | R-033, R-034, R-035, R-059, R-062 |  |
| `review-governance-intent.evidence-floor` | floor | R-044 |  |
| `review-governance-intent.verdict-is-input` | floor | R-011, R-045, R-063 |  |
| `review-governance-intent.ratification-user-owned` | floor | R-011, R-012, R-045 |  |
| `review-governance-intent.findings-through-leads-pen` | floor | R-043, R-067 |  |

## Retired (old entries with no log counterpart)

- **R-003** (must; was `SKILL.md:Overview (closes on one artifact — `governance-intent.md`: the fact profile, floo`): prose — the artifact definition (six element families of governance-intent.md) is SKILL.md intro prose, not a log rule.
- **R-004** (must; was `SKILL.md:Overview (each element carrying a GI-ID and a confidence mark (`Confident / Assum`): prose — GI-ID plus the five-value lead-assigned confidence-mark enum is SKILL.md intro prose; the log carries no mark-vocabulary rule.
- **R-006** (must; was `SKILL.md:Overview (*coverage* (the agenda surface: missed dimensions, convergence-skip aud`): prose — the coverage-lens scope (four families) is SKILL.md intro prose; the log names the lens only (lens-depth-never-jurisdiction).
- **R-007** (must; was `SKILL.md:Overview (*coherence* (internal consistency: fact↔risk↔ruling alignment, the mark`): prose — the coherence-lens scope (four families) is SKILL.md intro prose; the log names the lens only (lens-depth-never-jurisdiction).

## Invariants verified

- `class: floor` id set in the file == render `floors:` line (order and membership) and == the `class: floor` pin count.
- Every rule id in `.mochiko/schema-views/skills/review-governance-intent.yaml` appears exactly once; the view's id list equals the live render's id list.
- No id appears twice; every old `R-XXX` is either mapped (possibly to more than one log rule) or retired, never both, never dropped.
- `evals/run.py` fields preserved: `id`, `class` (read), `rule` and the rest passed through to the judge as JSON.
