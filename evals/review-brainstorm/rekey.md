# Re-key record — `evals/review-brainstorm/rules.json`

```json
{
  "rekeyed": {
    "at": "2026-09-08T23:23:18Z",
    "ruling": "primitive-eval-harness-v2 D12/C3 (R2: five skills)",
    "source": ".mochiko/schema-views/skills/review-brainstorm.yaml + mochiko-cli floors line"
  }
}
```

`rules.json` is a bare list the runner (`evals/run.py`) iterates directly (`for r in rules`, `len(rules)`, `r["id"]`, `r.get("class")`), so the stamp, the retired list, and the mapping live here rather than in the file. Every entry keeps `id` / `rule` / `class` / `source`; `section`, `kind`, `when`, `pointer`, `labels`, and `mapped_from` are additive.

## Counts

- Rules: 67 → 30 (log rules; every id in the derived view appears exactly once).
- Floors: 6 → 9 — equals the render's `floors:` line and its `class: floor · 9 rules` pin.
- Retired: 3. Added (no old counterpart): 4.

## Floors

Old (`SKILL.md`-keyed): `R-001`, `R-002`, `R-003`, `R-004`, `R-005`, `R-006`

New (log-keyed, = render `floors:`): `review-brainstorm.never-in-the-room`, `review-brainstorm.blind-map-before-record-contact`, `review-brainstorm.author-grader`, `review-brainstorm.contested-needs-new-angle`, `review-brainstorm.never-default-ready`, `review-brainstorm.unverifiable-claim-is-finding`, `review-brainstorm.evidence-floor`, `review-brainstorm.verdict-is-input`, `review-brainstorm.findings-through-leads-pen`

## Class changes (old entry → log rule)

| old id | old class | log rule | log class |
|---|---|---|---|
| R-055 | must | `review-brainstorm.never-in-the-room` | floor |
| R-064 | vocab | `review-brainstorm.never-in-the-room` | floor |
| R-010 | must | `review-brainstorm.blind-map-before-record-contact` | floor |
| R-059 | vocab | `review-brainstorm.blind-map-before-record-contact` | floor |
| R-008 | should | `review-brainstorm.lens-depth-never-jurisdiction` | must |
| R-066 | vocab | `review-brainstorm.reopen-born-verify-grade` | must |
| R-062 | vocab | `review-brainstorm.external-claims-binding` | must |
| R-067 | vocab | `review-brainstorm.cross-exam-binding` | must |
| R-063 | vocab | `review-brainstorm.hunt-classes-per-decision` | must |
| R-061 | vocab | `review-brainstorm.record-fitness-binding` | must |
| R-038 | format | `review-brainstorm.coverage-severity-mapping` | must |
| R-057 | vocab | `review-brainstorm.coverage-severity-mapping` | must |
| R-058 | vocab | `review-brainstorm.coverage-severity-mapping` | must |
| R-046 | format | `review-brainstorm.status-vocabulary-and-criteria` | must |
| R-051 | should | `review-brainstorm.status-vocabulary-and-criteria` | must |
| R-056 | vocab | `review-brainstorm.status-vocabulary-and-criteria` | must |
| R-050 | must | `review-brainstorm.never-default-ready` | floor |
| R-041 | format | `review-brainstorm.findings-formed-count-only` | must |
| R-065 | vocab | `review-brainstorm.findings-formed-count-only` | must |
| R-034 | format | `review-brainstorm.survivor-report-form` | must |
| R-044 | format | `review-brainstorm.survivor-report-form` | must |
| R-060 | vocab | `review-brainstorm.survivor-report-form` | must |
| R-053 | must | `review-brainstorm.evidence-floor` | floor |
| R-054 | must | `review-brainstorm.verdict-is-input` | floor |
| R-064 | vocab | `review-brainstorm.verdict-is-input` | floor |
| R-052 | must | `review-brainstorm.findings-through-leads-pen` | floor |

## Mapping (log rule ← old entries)

| log rule | class | mapped_from | note |
|---|---|---|---|
| `review-brainstorm.never-in-the-room` | floor | R-003, R-055, R-064 |  |
| `review-brainstorm.blind-map-before-record-contact` | floor | R-001, R-010, R-059 |  |
| `review-brainstorm.cold-read-before-counterpart` | must | — (added) |  |
| `review-brainstorm.author-grader` | floor | R-002 |  |
| `review-brainstorm.lens-depth-never-jurisdiction` | must | R-008, R-009 |  |
| `review-brainstorm.never-excess` | must | R-021 |  |
| `review-brainstorm.reopen-born-verify-grade` | must | R-066 |  |
| `review-brainstorm.contested-needs-new-angle` | floor | R-004 |  |
| `review-brainstorm.its-command-states-them` | must | R-054 |  |
| `review-brainstorm.grounding-excludes-session-artifacts` | must | R-012 |  |
| `review-brainstorm.verify-load-bearing-claims` | must | R-023 |  |
| `review-brainstorm.map-sample-audit` | must | — (added) |  |
| `review-brainstorm.external-claims-binding` | must | R-024, R-025, R-062 |  |
| `review-brainstorm.cross-exam-binding` | must | R-042, R-067 | when: pairing=pair — R-042/R-067's solo-skip clause is the rule's own gate, not a separate rule. |
| `review-brainstorm.hunt-classes-per-decision` | must | R-013, R-014, R-015, R-016, R-017, R-018, R-019, R-020, R-022, R-063 | R-014–R-020 (one rule per hunt class) and R-016 (class-2 boundary) collapse into one duty; per-class detail is SKILL.md Protocol prose the rule points at. |
| `review-brainstorm.excess-names-cheaper-shape` | must | R-021 |  |
| `review-brainstorm.record-fitness-binding` | must | R-026, R-027, R-028, R-029, R-030, R-031, R-032, R-033, R-061 | R-027–R-033 (the seven fitness items) are bound by pointer to references/RECORD-FITNESS.md, never restated; R-061's five-word confidence-mark enum has no log rule of its own. |
| `review-brainstorm.coverage-severity-mapping` | must | R-036, R-038, R-057, R-058 | R-036 (run the map-vs-record diff) is implied, not stated; R-057's Critical/Important/Minor trio is carried by this rule's parenthetical. |
| `review-brainstorm.dismissed-angle-is-ruling` | must | R-040 |  |
| `review-brainstorm.status-vocabulary-and-criteria` | must | R-046, R-047, R-048, R-049, R-051, R-056 | R-051 (too thin is the first finding) survives here only as the critical-gaps criterion 'a record too thin to review'; the first-finding duty is a floor in review-governance-intent, not in this log. |
| `review-brainstorm.unresolvable-is-commentary` | must | R-035 |  |
| `review-brainstorm.never-default-ready` | floor | R-005, R-050 |  |
| `review-brainstorm.unverifiable-claim-is-finding` | floor | R-006 |  |
| `review-brainstorm.verify-pass-grade` | must | — (added) |  |
| `review-brainstorm.synthesis-fidelity-sample` | must | — (added) |  |
| `review-brainstorm.findings-formed-count-only` | must | R-041, R-065 |  |
| `review-brainstorm.survivor-report-form` | must | R-034, R-037, R-043, R-044, R-045, R-060 | R-037's coverage evidence bar ('the diff plus materiality') is in this rule's text. |
| `review-brainstorm.evidence-floor` | floor | R-053 |  |
| `review-brainstorm.verdict-is-input` | floor | R-054, R-064 |  |
| `review-brainstorm.findings-through-leads-pen` | floor | R-052 |  |

## Retired (old entries with no log counterpart)

- **R-007** (must; was `SKILL.md:Overview (Spawned solo, your findings go to the lead undebated)`): cut — solo findings held to the cross-examination bar; no log rule, not in SKILL.md prose.
- **R-011** (format; was `SKILL.md:Phase 0 (Enumerate the angles ... actors, cost, failure modes, timescales, altern`): cut — angle-kind enumeration for the blind map; the log's blind-map-before-record-contact carries only the deliverable and its timing.
- **R-039** (must; was `SKILL.md:Phase 1 (Materiality — solo, apply this bar explicitly)`): cut — materiality gate (an angle whose absence changes nothing dies at the diff); nearest surviving text is the Minor rung of coverage-severity-mapping.

## Invariants verified

- `class: floor` id set in the file == render `floors:` line (order and membership) and == the `class: floor` pin count.
- Every rule id in `.mochiko/schema-views/skills/review-brainstorm.yaml` appears exactly once; the view's id list equals the live render's id list.
- No id appears twice; every old `R-XXX` is either mapped (possibly to more than one log rule) or retired, never both, never dropped.
- `evals/run.py` fields preserved: `id`, `class` (read), `rule` and the rest passed through to the judge as JSON.
