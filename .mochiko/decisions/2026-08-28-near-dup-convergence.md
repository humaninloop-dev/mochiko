# Near-dup convergence — the similar-rule reduction ruling (R1–R6)

**Date:** 2026-08-28 · **Status:** ruled + built (wave DELIVERED same day at v0.99.0;
desk-sized; no session record — this ADR is the rationale home) · **Ruled by:** the user,
"as recommended and accept", on the lead's draft.

**Delivery note (2026-08-28, v0.99.0):** all eight moves landed — common.yaml 4→9 blocks,
24 stubs, ~24 texts single-homed; allowlist seeded with 60 adjudicated edges (detector runs
silent); strips across 8 files incl. new `strips/common.md`; 3 author≠grader validators,
6/6 pairs PASS (one V3-caught consumers-list omission in `strips/common.md` repaired and
CONFIRMED discharged); checker 0 findings ×6, all blocks bound; matrices 133/133 + 34/34;
cargo 12/12. Verification read: the extends-aware partition shows exactly 3 resolved-text
changes per instrumented command (all ruled upgrades); pre/post plan-only grids on
implement + setup (pre arm = ontology-grid post arm per the D7 reuse precedent, opus
judges, prereg noise guard fired → r4 pairs added, k=4 both grids) found **no regression
attributable to the wave** — every dipped rule flaky within its own arm, post never
trailing at aggregate, widened and upgraded texts graded present against their new
wording. Instrument side-findings for the eval desk: opus coverage-judge calls
occasionally return unparseable arrays (healed by targeted re-judge; retry belongs in the
runner), and the noise falsifier's instrument-side standing is re-confirmed at k=4.

## Context

The similar-rule detector (`scripts/find-similar-rules.py`, layer 1 of the similar-items
grooming system, built 2026-08-28) scanned the six command content schemas at v0.98.0:
321 rules, 12,203 in-kind pairs scored, 36 clusters — 11 spanning 3+ commands. The
`command-schema-ontology` D8 bar (itself a narrow supersession of command-content-schema
D3, `Contested`) licensed extraction to `common.yaml` only for EXACT duplicates across
3+ commands. The scan shows most duplication is *near*-identical: same responsibility,
wording drift confined to phrasing, illustration, or unit nouns — plus one conversion-wave
inconsistency (`setup.register` unbound while five siblings extend) and one exact triple
the D8 bar already licensed but the wave missed (`plan-approval-producers`).

## Ruling

- **R1 — Convergence licensed.** A family spanning 3+ commands with near-identical wording
  (same responsibility; differences confined to phrasing, illustration, or unit nouns) may
  converge to one `common.<slug>` block. This is a narrow widening of the ontology D8
  extraction bar; D8's other limbs (per-command default, stub-carried binding, block =
  boilerplate never judgment) stand unchanged.
- **R2 — Strongest wording wins.** The block text is the fullest correct member wording,
  never an average. A member whose extra content is command-specific (not illustrative)
  keeps its local text; that edge is allowlisted with a reason.
- **R3 — IDs survive as stubs.** Every member ID survives as an `extends:` stub; `class` /
  `kind` / `when` / `enforces` stay local (ontology C3 unchanged). No tombstones — nothing
  retires.
- **R4 — Floors by supersession.** A `class: floor` member converges only with a
  supersession-by-ruling strip entry citing this ruling.
- **R5 — Net-reduction test.** A family converges only when the block genuinely
  single-homes text. One-liner families with unit-noun variance stay distinct
  (`dm-close-verdict`, `staffing-latitude` — adjudicated keep-distinct this wave).
- **R6 — Adjudications recorded.** Keep-distinct decisions land in
  `scripts/similar-rules-allowlist.yaml` with reasons, so detector reruns stay quiet and
  the adjudication trail is inspectable.

## Wave-scope rulings (flags A–E, each "as recommended")

- **A** `common.transport-floor` widens with arch/feat's enumeration ("Trigger test, floor
  legs, composition-safe shapes, and disclosure").
- **B** `arch.model-tiering` keeps local text (its drift-probe routing clause is
  arch-specific); `feat.model-tiering`'s "(map sweeps, territory reads)" parenthetical
  drops as illustrative.
- **C** `spec.author-grader-default-fail` drops its file enumeration as illustrative; the
  block carries setup's strongest generic wording.
- **D** `impl.acceptance-plain-text` keeps local text — its three named gates do real
  binding work.
- **E** `impl.plan-approval-producers` keeps local text — "code" and "verification seats"
  are real implement content.

## Wave scope (v0.99.0)

Eight moves: `setup.register` stub joins `common.register` · new blocks
`common.no-acceptance` (exact triple, D8 bar) · `common.model-tiering` ·
`common.author-grader-default-fail` · `common.tools-referenced-never-restated` ·
`common.plan-approval-producers` (exact triple, D8 bar) · widened
`common.transport-floor` + `common.acceptance-plain-text` ("rulings and acceptance").
24 stubs; ~13 floor supersession entries; allowlist seeded for survivors.

## Verification protocol

Strips + author≠grader audits per touched pair (GI-004) before the `plugin.json` bump ·
`check-command-schema.py --all` + detector + both test matrices green · pre/post plan-only
grids on implement + setup (ontology-grid post arm reused as pre, per the ontology D7 reuse
precedent), expectation: no observable change.

## Amended surfaces

`plugins/mochiko/schemas/common.yaml` header (extraction-bar paragraph) ·
`.claude/rules/mochiko/primitive-edits.md` criterion 11 (exact-duplicate stub clause widened
to cite this ruling) · the ontology D8 row's "exact-duplicate" limb reads as amended by this
ADR.
