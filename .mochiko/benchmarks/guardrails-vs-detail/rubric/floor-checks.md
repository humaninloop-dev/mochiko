# Floor checks — binary, per stage

The non-waivable floor list for each benchmark stage. Distinct from the scoring rubrics
(`setup-artifacts.md`, `specify-artifacts.md`), which grade quality on a 0–100 gradient.

**A floor is binary: PASS or VIOLATION.** Per record D6, any floor violation in any
guardrails replicate is an **automatic flag** for that stage's cluster regardless of the
rubric score — it forces "detail pays" for the floor's section, score notwithstanding. A
high rubric score never buys a floor back.

These floors are the secondary floor-conformance check the record's D5 keeps alongside the
comparator rubric — never the comparator itself. They are derived from the record's D4
keep-set (goal / output contract · non-waivable floor · anti-patterns and rejections · hard
reference data) and from each command's own Goal and "Not done — default FAIL" lines — not
from any skill's internals.

## Checking protocol

- **Artifacts only.** Check the run's produced artifacts, exactly as the rubric scorer does.
  Never read a transcript, a report, or the variant that produced them.
- Each floor is checked independently. Mark PASS or VIOLATION with a one-line evidence note.
- One VIOLATION flags the stage. Report every violation found; do not stop at the first.

## Setup floors

Derived from `commands/setup.md` Goal + "Not done — default FAIL", and the governance floor.

| ID | Floor | VIOLATION when |
|----|-------|----------------|
| F-S1 | **No secrets in the repo.** | Any credential, token, key, or secret appears in an authored surface. |
| F-S2 | **Three-Part principles.** Every principle carries enforcement + testability + rationale. | Any principle is missing enforcement, a testable condition, or a rationale. |
| F-S3 | **No silent floor gap.** Every floor category has a principle or a recorded waiver. | A floor category is absent with neither a governing principle nor a recorded waiver. |
| F-S4 | **Version bumped.** The governance region carries a bumped semver / ratified stamp. | No ratified stamp, or the semver was not bumped for this run. |
| F-S5 | **Feature map exists at close.** | No `FEATURES.md` — brownfield reconstruction absent, or greenfield scaffold absent. |
| F-S6 | **Intent ratified before surfaces.** The intent synthesis exists and the region carries the ratified stamp keyed to it. | Governance surfaces exist with no ratified intent synthesis behind them. |

## Specify floors

Derived from `commands/specify.md` Goal + "Not done — default FAIL".

| ID | Floor | VIOLATION when |
|----|-------|----------------|
| F-SP1 | **Every story has acceptance scenarios.** | A story file carries no acceptance scenario (and is not marked `rejected`). |
| F-SP2 | **Out-of-scope list present.** | The spec has no out-of-scope list. |
| F-SP3 | **Screens & Flows section present.** The SCR/FLOW manifest, or the explicit "No UX surface — prototype waived at intent." line. | The section is absent entirely (neither manifest nor waiver line). |
| F-SP4 | **Feature Selection section present, with the user's selection recorded.** | The section is absent, or the selection was made without the user's ruling recorded. |
| F-SP5 | **Every story homed or rejected.** Each story maps to exactly one FEAT-ID or is marked `rejected` with a why. | A story is neither homed to exactly one feature nor rejected with a recorded reason. |
| F-SP6 | **No placeholder tokens.** `spec.md` conforms to the template with no unresolved placeholders. | Any template placeholder token survives in `spec.md`. |
| F-SP7 | **UX-bearing implies a prototype with no drift.** | Ruled UX-bearing but no clickable prototype, or manifest↔prototype drift is unresolved. |

## Both stages

| ID | Floor | VIOLATION when |
|----|-------|----------------|
| F-X1 | **Independently graded.** The artifact set shows evidence of an independent grade (a stress-test / review pass), not author self-clearance. | The only grade of record is the author's own. |
