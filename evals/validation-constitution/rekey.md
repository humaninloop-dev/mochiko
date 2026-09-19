# Re-key record — `evals/validation-constitution/rules.json`

```json
{
  "rekeyed": {
    "at": "2026-09-08T23:23:18Z",
    "ruling": "primitive-eval-harness-v2 D12/C3 (R2: five skills)",
    "source": ".mochiko/schema-views/skills/validation-constitution.yaml + mochiko-cli floors line"
  }
}
```

`rules.json` is a bare list the runner (`evals/run.py`) iterates directly (`for r in rules`, `len(rules)`, `r["id"]`, `r.get("class")`), so the stamp, the retired list, and the mapping live here rather than in the file. Every entry keeps `id` / `rule` / `class` / `source`; `section`, `kind`, `when`, `pointer`, `labels`, and `mapped_from` are additive.

## Counts

- Rules: 69 → 26 (log rules; every id in the derived view appears exactly once).
- Floors: 9 → 14 — equals the render's `floors:` line and its `class: floor · 14 rules` pin.
- Retired: 4. Added (no old counterpart): 0.

## Floors

Old (`SKILL.md`-keyed): `R-001`, `R-002`, `R-004`, `R-006`, `R-011`, `R-015`, `R-016`, `R-017`, `R-059`

New (log-keyed, = render `floors:`): `validation-constitution.author-grader`, `validation-constitution.from-file-floor`, `validation-constitution.set-not-file`, `validation-constitution.every-set-must-pass`, `validation-constitution.letter-is-spirit`, `validation-constitution.verify-every-item`, `validation-constitution.binary-verdict`, `validation-constitution.default-fail`, `validation-constitution.rationalization-stop`, `validation-constitution.placeholders-incomplete`, `validation-constitution.missing-parts-fail`, `validation-constitution.satisfaction-verifies-nothing`, `validation-constitution.evidence-floor`, `validation-constitution.skip-documented`

## Class changes (old entry → log rule)

| old id | old class | log rule | log class |
|---|---|---|---|
| R-049 | must | `validation-constitution.author-grader` | floor |
| R-053 | must | `validation-constitution.author-grader` | floor |
| R-063 | vocab | `validation-constitution.author-grader` | floor |
| R-064 | vocab | `validation-constitution.author-grader` | floor |
| R-067 | vocab | `validation-constitution.author-grader` | floor |
| R-043 | format | `validation-constitution.excess-governance` | must |
| R-069 | vocab | `validation-constitution.input-set` | must |
| R-015 | floor | `validation-constitution.missing-input-fails` | must |
| R-016 | floor | `validation-constitution.missing-input-fails` | must |
| R-017 | floor | `validation-constitution.missing-input-fails` | must |
| R-019 | format | `validation-constitution.quality-checklist` | must |
| R-063 | vocab | `validation-constitution.quality-checklist` | must |
| R-069 | vocab | `validation-constitution.set-not-file` | floor |
| R-022 | must | `validation-constitution.verify-every-item` | floor |
| R-023 | must | `validation-constitution.verify-every-item` | floor |
| R-039 | format | `validation-constitution.vague-language` | must |
| R-045 | must | `validation-constitution.binary-verdict` | floor |
| R-060 | vocab | `validation-constitution.binary-verdict` | floor |
| R-061 | vocab | `validation-constitution.binary-verdict` | floor |
| R-005 | must | `validation-constitution.default-fail` | floor |
| R-061 | vocab | `validation-constitution.default-fail` | floor |
| R-005 | must | `validation-constitution.rationalization-stop` | floor |
| R-047 | must | `validation-constitution.rationalization-stop` | floor |
| R-048 | must | `validation-constitution.rationalization-stop` | floor |
| R-051 | must | `validation-constitution.rationalization-stop` | floor |
| R-052 | must | `validation-constitution.rationalization-stop` | floor |
| R-054 | must | `validation-constitution.rationalization-stop` | floor |
| R-055 | must | `validation-constitution.rationalization-stop` | floor |
| R-044 | must | `validation-constitution.placeholders-incomplete` | floor |
| R-057 | must | `validation-constitution.missing-parts-fail` | floor |
| R-050 | must | `validation-constitution.satisfaction-verifies-nothing` | floor |
| R-028 | format | `validation-constitution.validation-result-block` | must |
| R-029 | format | `validation-constitution.validation-result-block` | must |
| R-030 | format | `validation-constitution.validation-result-block` | must |
| R-031 | format | `validation-constitution.validation-result-block` | must |
| R-032 | format | `validation-constitution.validation-result-block` | must |
| R-033 | format | `validation-constitution.validation-result-block` | must |
| R-034 | format | `validation-constitution.validation-result-block` | must |
| R-035 | format | `validation-constitution.validation-result-block` | must |
| R-036 | format | `validation-constitution.validation-result-block` | must |
| R-037 | format | `validation-constitution.validation-result-block` | must |
| R-062 | vocab | `validation-constitution.validation-result-block` | must |
| R-065 | vocab | `validation-constitution.validation-result-block` | must |
| R-056 | must | `validation-constitution.skip-documented` | floor |

## Mapping (log rule ← old entries)

| log rule | class | mapped_from | note |
|---|---|---|---|
| `validation-constitution.author-grader` | floor | R-049, R-053, R-059, R-063, R-064, R-067 | R-064's 'never co-mounted' clause is SKILL.md intro prose; R-049's 'reviewed it while writing' is in the rule text verbatim. |
| `validation-constitution.not-for` | must | R-007, R-008, R-009, R-010, R-046 |  |
| `validation-constitution.excess-governance` | must | R-040, R-041, R-043 |  |
| `validation-constitution.never-excess` | must | R-042 |  |
| `validation-constitution.from-file-floor` | floor | R-011 |  |
| `validation-constitution.input-set` | must | R-012, R-013, R-014, R-069 |  |
| `validation-constitution.missing-input-fails` | must | R-015, R-016, R-017 | R-015–R-017 were floors; the log carries the three missing-input FAILs as one must (missing-parts-fail stays a floor). |
| `validation-constitution.superseded-artifact-flag` | must | R-018 |  |
| `validation-constitution.checklist-assembly` | must | R-020, R-021 |  |
| `validation-constitution.quality-checklist` | must | R-019, R-063 |  |
| `validation-constitution.anti-patterns-scan` | must | R-051, R-052 |  |
| `validation-constitution.set-not-file` | floor | R-001, R-069 |  |
| `validation-constitution.every-set-must-pass` | floor | R-002 |  |
| `validation-constitution.letter-is-spirit` | floor | R-004 |  |
| `validation-constitution.verify-every-item` | floor | R-022, R-023 | R-022 was a must; the log carries it as a floor and folds R-023 (unselected fragments) into it. |
| `validation-constitution.vague-language` | must | R-038, R-039 |  |
| `validation-constitution.version-bump` | must | R-024, R-025, R-026, R-027 |  |
| `validation-constitution.binary-verdict` | floor | R-045, R-060, R-061 | R-045 was a must; the log carries the binary verdict as a floor. |
| `validation-constitution.default-fail` | floor | R-005, R-061 |  |
| `validation-constitution.rationalization-stop` | floor | R-005, R-047, R-048, R-051, R-052, R-054, R-055 | R-047 (stop and restart) plus the six named rationalizations; 'restart from Step 1' became 'restart from checklist assembly'. |
| `validation-constitution.placeholders-incomplete` | floor | R-044 |  |
| `validation-constitution.missing-parts-fail` | floor | R-057 |  |
| `validation-constitution.satisfaction-verifies-nothing` | floor | R-050 |  |
| `validation-constitution.validation-result-block` | must | R-028, R-029, R-030, R-031, R-032, R-033, R-034, R-035, R-036, R-037, R-062, R-065 | R-028–R-037 (one rule per output line) collapse into one binding; every line survives in the rule text. |
| `validation-constitution.evidence-floor` | floor | R-006 |  |
| `validation-constitution.skip-documented` | floor | R-056 |  |

## Retired (old entries with no log counterpart)

- **R-003** (must; was `SKILL.md:Overview (Governance validation ensures the surface set is enforceable, testable,`): prose — the purpose statement (enforceable · testable · trace-closed · anti-pattern-free, before finalization) is SKILL.md intro prose.
- **R-058** (format; was `SKILL.md:Related Skills (**OPTIONAL:** mochiko:authoring-constitution - Core authoring for`): prose — the Related Skills pointer to mochiko:authoring-constitution is SKILL.md intro prose ('Producer side: … never co-mounted').
- **R-066** (vocab; was `consumer: plugins/mochiko/skills/review-governance-intent/SKILL.md:33; SKILL.md:Overview (`): seam — the downstream Tier-2 jurisdiction line is held by review-governance-intent.authored-surfaces-out in that skill's log; this log carries no rule for it.
- **R-068** (vocab; was `consumer: plugins/mochiko/skills/review-feasibility/references/FEASIBILITY-LENS.md:140,183`): seam — 'governance well-formedness is this skill's domain' is held by review-plan-artifacts.not-for and review-feasibility's reference; this log carries no rule for it.

## Invariants verified

- `class: floor` id set in the file == render `floors:` line (order and membership) and == the `class: floor` pin count.
- Every rule id in `.mochiko/schema-views/skills/validation-constitution.yaml` appears exactly once; the view's id list equals the live render's id list.
- No id appears twice; every old `R-XXX` is either mapped (possibly to more than one log rule) or retired, never both, never dropped.
- `evals/run.py` fields preserved: `id`, `class` (read), `rule` and the rest passed through to the judge as JSON.
