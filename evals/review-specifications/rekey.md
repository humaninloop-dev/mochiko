# Re-key record — `evals/review-specifications/rules.json`

```json
{
  "rekeyed": {
    "at": "2026-09-08T23:23:18Z",
    "ruling": "primitive-eval-harness-v2 D12/C3 (R2: five skills)",
    "source": ".mochiko/schema-views/skills/review-specifications.yaml + mochiko-cli floors line"
  }
}
```

`rules.json` is a bare list the runner (`evals/run.py`) iterates directly (`for r in rules`, `len(rules)`, `r["id"]`, `r.get("class")`), so the stamp, the retired list, and the mapping live here rather than in the file. Every entry keeps `id` / `rule` / `class` / `source`; `section`, `kind`, `when`, `pointer`, `labels`, and `mapped_from` are additive.

## Counts

- Rules: 81 → 30 (log rules; every id in the derived view appears exactly once).
- Floors: 7 → 8 — equals the render's `floors:` line and its `class: floor · 8 rules` pin.
- Retired: 2. Added (no old counterpart): 0.

## Floors

Old (`SKILL.md`-keyed): `R-001`, `R-026`, `R-030`, `R-043`, `R-060`, `R-062`, `R-071`

New (log-keyed, = render `floors:`): `review-specifications.author-grader`, `review-specifications.no-scope-creep`, `review-specifications.map-git-baseline`, `review-specifications.check-existing-first`, `review-specifications.density-never-gap`, `review-specifications.evidence-floor`, `review-specifications.gap-bound`, `review-specifications.input-not-verdict`

## Class changes (old entry → log rule)

| old id | old class | log rule | log class |
|---|---|---|---|
| R-077 | vocab | `review-specifications.author-grader` | floor |
| R-005 | format | `review-specifications.not-for` | must |
| R-073 | vocab | `review-specifications.not-for` | must |
| R-079 | vocab | `review-specifications.not-for` | must |
| R-078 | vocab | `review-specifications.feature-layer-same-report` | must |
| R-069 | must | `review-specifications.no-scope-creep` | floor |
| R-026 | floor | `review-specifications.external-claims` | must |
| R-027 | format | `review-specifications.external-claims` | must |
| R-080 | vocab | `review-specifications.external-claims` | must |
| R-029 | format | `review-specifications.map-mirror` | must |
| R-043 | floor | `review-specifications.serve-and-click` | must |
| R-076 | vocab | `review-specifications.serve-and-click` | must |
| R-077 | vocab | `review-specifications.serve-and-click` | must |
| R-070 | must | `review-specifications.check-existing-first` | floor |
| R-017 | format | `review-specifications.gap-taxonomy` | must |
| R-072 | vocab | `review-specifications.gap-taxonomy` | must |
| R-081 | vocab | `review-specifications.gap-taxonomy` | must |
| R-075 | vocab | `review-specifications.feature-critical-checks` | must |
| R-075 | vocab | `review-specifications.feature-important-checks` | must |
| R-076 | vocab | `review-specifications.sf-critical-checks` | must |
| R-076 | vocab | `review-specifications.sf-important-checks` | must |
| R-056 | vocab | `review-specifications.severity-grammar` | must |
| R-072 | vocab | `review-specifications.severity-grammar` | must |
| R-059 | format | `review-specifications.density-never-gap` | floor |
| R-061 | must | `review-specifications.density-never-gap` | floor |
| R-014 | format | `review-specifications.clarifications-shape` | must |
| R-057 | format | `review-specifications.report-structure` | must |
| R-058 | format | `review-specifications.report-structure` | must |
| R-072 | vocab | `review-specifications.report-structure` | must |
| R-067 | must | `review-specifications.gap-bound` | floor |
| R-068 | should | `review-specifications.gap-bound` | floor |
| R-074 | vocab | `review-specifications.input-not-verdict` | floor |

## Mapping (log rule ← old entries)

| log rule | class | mapped_from | note |
|---|---|---|---|
| `review-specifications.author-grader` | floor | R-071, R-077 |  |
| `review-specifications.what-not-how` | must | R-002, R-010, R-011 |  |
| `review-specifications.not-for` | must | R-003, R-004, R-005, R-006, R-007, R-008, R-009, R-073, R-079 | R-003–R-009 (one rule per exclusion) collapse into the routing list; 'performance specs' and 'implementation planning' survive verbatim. |
| `review-specifications.excess-admissibility` | must | R-024 |  |
| `review-specifications.never-excess` | must | R-024 |  |
| `review-specifications.feature-layer-same-report` | must | R-028, R-078 |  |
| `review-specifications.no-scope-creep` | floor | R-069 |  |
| `review-specifications.complete-coverage` | must | R-064, R-065, R-066 |  |
| `review-specifications.external-claims` | must | R-026, R-027, R-080 | R-026 was a floor; the log carries the external-claims binding as must. |
| `review-specifications.map-git-baseline` | floor | R-030 |  |
| `review-specifications.map-mirror` | must | R-029 |  |
| `review-specifications.sf-legal-shapes` | must | R-042 |  |
| `review-specifications.serve-and-click` | must | R-043, R-076, R-077 | R-043 was a floor; the log carries serve-and-click as must (duty), gated when: manifest-present=present. |
| `review-specifications.check-existing-first` | floor | R-070 |  |
| `review-specifications.gap-taxonomy` | must | R-016, R-017, R-018, R-019, R-020, R-021, R-022, R-023, R-072, R-081 | R-018–R-023 (one rule per defect class) collapse into the taxonomy binding; class detail is SKILL.md Procedure prose; R-016's five user-facing categories are the rule's parenthetical. |
| `review-specifications.smuggled-posture` | must | R-025 |  |
| `review-specifications.feature-critical-checks` | must | R-031, R-032, R-033, R-036, R-037, R-038, R-075 |  |
| `review-specifications.feature-important-checks` | must | R-034, R-035, R-039, R-040, R-075 |  |
| `review-specifications.authority-split` | must | R-044 |  |
| `review-specifications.sf-critical-checks` | must | R-045, R-046, R-047, R-048, R-049, R-076 |  |
| `review-specifications.sf-important-checks` | must | R-050, R-051, R-052, R-076 |  |
| `review-specifications.severity-grammar` | must | R-053, R-054, R-055, R-056, R-072 |  |
| `review-specifications.density-never-gap` | floor | R-059, R-060, R-061 | R-061's advisory-overage clause is in the rule text; R-059's envelope framing rides it. |
| `review-specifications.question-format` | must | R-013, R-015 |  |
| `review-specifications.no-presupposed-mechanism` | must | R-012 |  |
| `review-specifications.clarifications-shape` | must | R-014 |  |
| `review-specifications.report-structure` | must | R-057, R-058, R-072 |  |
| `review-specifications.evidence-floor` | floor | R-062 |  |
| `review-specifications.gap-bound` | floor | R-067, R-068 | R-068 was a should; the log carries the 5–7 bound as a floor and folds R-067 (related gaps grouped) into it. |
| `review-specifications.input-not-verdict` | floor | R-001, R-074 |  |

## Retired (old entries with no log counterpart)

- **R-041** (must; was `SKILL.md:The Screens & Flows section (Graded with the spec — same reviewer, same report)`): prose — Screens & Flows graded same reviewer / same report is SKILL.md Procedure prose ('widen to the spec's other layers'); the feature layer has feature-layer-same-report, Screens & Flows has no equivalent log rule (its grading is sf-legal-shapes and the sf-* check sets).
- **R-063** (must; was `SKILL.md:Quality Checklist (Before finalizing the review, verify:)`): cut — the Quality Checklist as a pre-finalization self-check went with the section.

## Invariants verified

- `class: floor` id set in the file == render `floors:` line (order and membership) and == the `class: floor` pin count.
- Every rule id in `.mochiko/schema-views/skills/review-specifications.yaml` appears exactly once; the view's id list equals the live render's id list.
- No id appears twice; every old `R-XXX` is either mapped (possibly to more than one log rule) or retired, never both, never dropped.
- `evals/run.py` fields preserved: `id`, `class` (read), `rule` and the rest passed through to the judge as JSON.

## Re-key 2026-09-24 — setup-product-agnostic

```json
{
  "rekeyed": {
    "at": "2026-09-24T13:32:56Z",
    "ruling": "setup-product-agnostic D1/D2, 2026-09-24; lead rulings H4 and the addendum re-plan round 1",
    "source": ".mochiko/schema-views/skills/review-specifications.yaml (stubs: .mochiko/schema-views/common/skill-review-common.yaml) + mochiko-cli floors line"
  }
}
```

Field-scoped: only the `rule` text of the entries named below moved, copied from the landed view (the `mochiko-cli rules` render cross-checked per id, `${var}` substituted from the skill view's `vars:`); every other field and entry is byte-identical, `git diff` the proof.

### Counts

- Rules: 30 → 30. Floors: 8 → 8 — equals the render's `floors:` line and its pin.
- Retired: 0. Added: none.

### Text re-keyed

- `review-specifications.never-excess` — an `extends: review-common.never-excess` stub, text resolved from `.mochiko/schema-views/common/skill-review-common.yaml`: no longer names compliance-module-derived obligations (`0019`, D1). The stub's `labels: [boundary]`, carried from the common block, is unchanged. (A § 7 build defect by lead ruling: the common block's retired clause was graded here.)

### Invariants verified

- Field-scoped to the one entry: it equals the resolved common-view text and the render's text; `git diff` shows that one `rule` line and nothing else.
- `class: floor` set equal to the render's `floors:` line; every `tempts` id resolves.
- Not asserted here: file-wide id-set parity. At this re-key the view carries 31 ids and this file 30 — `review-specifications.sf-direction-checks` (minted by `0013`, 2026-09-19) is absent. Pre-existing, out of this field-scoped re-key, raised to the wave lead (S3 plan H6); not repaired.

## Re-key 2026-10-08 — human-readable-ids

```json
{
  "rekeyed": {
    "at": "2026-10-08T03:34:34Z",
    "ruling": "human-readable-ids D6 (DECISIONS.md 2026-10-08), migration 0049; wave 2 test plan r1 §8",
    "source": ".mochiko/schema-views/skills/review-specifications.yaml (stubs: .mochiko/schema-views/common/skill-review-common.yaml) + mochiko-cli floors line"
  }
}
```

Field-scoped, as the 2026-10-06 re-key was: the one minted id is added at its view position with
`mapped_from: []` (no pre-cut `R-XXX` entry maps to it), its `rule`, `labels` and `pointer`
resolved from the `review-common.joined-ids` block it extends; every other field and entry is
byte-identical, `git diff` the proof. Script: one insert pass over the landed view and the common
view; the file round-trips byte for byte through the script's writer before the insert.

### Counts

- Rules: 30 → 31. Floors: 8 → 8 — equals the render's `floors:` line and its pin.
- Retired: 0. Added: 1.

### Added

| log rule | class | section | when / pointer | note |
|---|---|---|---|---|
| `review-specifications.joined-ids` | must | `sec.verdict` | `../../templates/artifact-format.md` | D6 — an `extends: review-common.joined-ids` stub; labels `verdict`, `artifact-grammar` carried from the block |

### Invariants verified

- Id set and order equal to the view's but one: `review-specifications.sf-direction-checks`
  (minted by `0013`, 2026-09-19) is still absent, as the 2026-09-24 re-key recorded. Pre-existing,
  out of this field-scoped re-key, raised to the wave lead; not repaired.
- `class: floor` set equal to the render's `floors:` line; every `tempts` id resolves.
- `review-specifications.joined-ids` sits in no golden's `tempts` — the wave ruled no
  change to `tempts` for the new stub — so the run reads it `untempted`, disclosed and never read,
  until a golden plants an ID to grade.

## Re-key 2026-10-10 — specify-brainstorm-discovery

```json
{
  "rekeyed": {
    "at": "2026-10-10T05:45:22Z",
    "ruling": "specify-brainstorm-discovery (DECISIONS.md 2026-10-10), migration 0053; wave 1 plan §3.9",
    "source": ".mochiko/schema-views/skills/review-specifications.yaml + mochiko-cli floors line"
  }
}
```

Field-scoped, as the 2026-09-24 and 2026-10-08 re-keys were: the one minted id is added at its
view position with `mapped_from: []` (no pre-cut `R-XXX` entry maps to it), and the one reworded
rule's `rule` text is copied from the landed view; every other field and entry is
byte-identical, `git diff` the proof. Script: one pass over the landed view; the file round-trips
byte for byte through the script's writer before the edit. The `when` value is written in the
file's own `<condition>=<value>` string form.

### Counts

- Rules: 31 → 32. Floors: 8 → 8 — equals the render's `floors:` line and its pin.
- Retired: 0. Added: 1. Text re-keyed: 1.

### Added

| log rule | class | section | when / pointer | note |
|---|---|---|---|---|
| `review-specifications.slate-checks` | must | `sec.verdict` | `slate-present=present` | the four slate checks (Slate home and binding, as repaired N2/N3); kind `duty`, label `verdict` |

### Text re-keyed

- `review-specifications.complete-coverage` — the coverage duty now names every slate row beside
  every user story (`0053`). Its class, kind, section, labels and `mapped_from` are unchanged.

### Invariants verified

- Id set and order equal to the view's but one: `review-specifications.sf-direction-checks`
  (minted by `0013`, 2026-09-19) is still absent, as the 2026-09-24 and 2026-10-08 re-keys
  recorded. Pre-existing, out of this field-scoped re-key; disclosed by the wave lead's ruling,
  not repaired.
- `preregistration.md` still reads "30 rules — 8 floor · 22 must"; that figure has been stale
  since the 2026-10-08 re-key. Pre-existing; disclosed by the same ruling, not repaired.
- `class: floor` set equal to the render's `floors:` line; every `tempts` id resolves.
- `review-specifications.slate-checks` sits in no golden's `tempts`, so the run reads it
  `untempted`, disclosed and never read. It is also gated off on every golden: no fixture spec
  carries the slate header line (`> Slate: …`) its `slate-present` condition reads.
