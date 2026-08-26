---
name: review-sufficiency
description: This skill MUST be invoked when grading a capability-batch's guidance sufficiency at `/mochiko:implement` entry — the ten-clause check per selected work row, collapsing to a three-clause form per delta card under delta scope, over the spec, the architecture store, and the product baselines, emitting a binding per-row `sufficient` verdict or the gap list that scopes the in-run design phase. SHOULD also invoke on 'sufficiency check', 'enough guidance', 'sufficiency verdict', or 'gap list'. Never reads code, `tasks.md`, `**TEST:**` cases, or cycle reports. Defaults to FAIL — a row is insufficient until every clause is graded; run by a seat that authored none of the graded sources.
---

# Grading Guidance Sufficiency

Binding pre-build gate over one unit of selected work: **does the guidance that already exists
carry enough for a builder to build it?** A `sufficient` verdict licenses cards and build
directly; any gap scopes the in-run design phase to exactly the named gaps, nothing else. You
grade sources you did not author, and you never author the fix — the design phase is a
different seat.

**Fence.** Read the feature's `spec.md` (its Screens & Flows manifest included), the product
architecture store, the product baselines (`data-model.md`, `contracts/`,
`constraints-and-decisions.md`), and the capability map entries at
`.mochiko/features/FEAT-XXX-<slug>.md`. Never the code, `tasks.md`, `**TEST:**` cases, cycle
reports, or **this batch's own** `FEAT-XXX/` run-output directory — all of those are downstream
of this verdict, and reading them makes the check circular. **One carve:** a clause-10 in-flight
collision licenses reading *the colliding feature's* design-phase deltas and owning spec — the
sole run-output read this fence admits, scoped to the colliding surface, and never extended to
that feature's code, cards, or cycle reports.

**Unit.** Selection scope grades **per selected work row** — the work row is the map's unit of
scope, which makes the check size-adaptive by construction. Delta scope grades **per delta
card**, and only three clauses apply: criteria testable (1) · touched surfaces identified
(2, 3) · store consult and trip check run (4). Clause 9 does not apply under delta scope — the
desk's delta card is itself the `[MODIFY]` instrument, carrying the marked delta on the entry
and folding at landing; a delta fix discovered structural re-fires the design phase rather
than clearing here.

## The ten clauses

A unit is *sufficient* only when every applicable clause holds. Each clause names its own gap
form; a clause that cannot be graded is a gap, never a pass.

1. **Testable criteria** — every acceptance scenario and SC-XXX has a stateable oracle. No
   stateable oracle = gap.
2. **Contract exposure** — every touched API surface named, then graded against baseline
   `contracts/`: *named-and-locatable* (the baseline publishes a continuation point the
   surface attaches to) = no gap; *named-and-unattachable* (no seam exists to attach it) = gap.
3. **Data exposure** — every touched entity named, then graded against baseline
   `data-model.md` on the same locatable / unattachable split.
4. **Structural trigger** — store consulted, trip check run, and either a no-delta claim
   recordable or a delta needed (gap). Spine elements whose `Derived from` cites the row's own
   feature or epic delta are **excluded** from the no-delta evidence: a row never satisfies
   this clause with its own planned structure.
5. **NFR targets** — the applicable store concern rows identified *and their targets stated*.
   Targets absent = gap. A row plausibly bearing NFR load (user-facing latency, data volume,
   auth surface, availability) with no identifiable concern row = gap.
6. **Commodity exposure** — any storage, queueing, caching, auth, search, or serialization
   need named and adopt-first answerable. A stated mechanism with **no weighed alternatives is
   not resolved**; unresolved = gap.
7. **Dependency order** — in-batch row dependencies resolvable. Unresolvable = gap.
8. **UX trace** — where the spec carries a Screens & Flows manifest: every FEAT-tagged
   SCR-XXX's data shown has a nameable serving contract surface, and every FLOW-XXX action a
   mutation path. Existing = cited; new = gap. No manifest = n/a.
9. **Delivered-feature exposure** (*selection scope only*) — keyed on the **row's** status,
   never the capability's. A touched surface owned by a delivered row is never zero-gap: it
   auto-fires the design phase, its `[MODIFY]` amendment is named in this report, and the
   amendment is written as the marked delta on the affected feature's map entry.
10. **In-flight exposure** — keyed on the **row's** status. A touched surface owned by an
    in-flight row obliges reading that feature's deltas and owning spec: need covered → cite
    the planned contract, no gap · adjacent → **gap**, the design phase authors the proposed
    delta sequenced behind that delivery · conflicting → reserved to the user at run-open.
    **No locks** — conflict routing is a question to the user, never a hold on the touched
    feature; only silent contradiction is prohibited.

## Branches

**Absent baselines.** An absent baseline file grades its touched surfaces new (gap), never
n/a. The design phase's first duty is then the seed: empty scaffolds where no code is
delivered, reconstruct-and-confirm with the user at the design checkpoint where delivered code
exists.

**Trips are not gaps.** A store trip — a touched row standing `open` or `not-now` — never
becomes a gap. It rides the verdict report and is dispositioned by the user at run-open:
warn and record; a recorded deferral is a legal escape, a silent skip is not.

## Verdict and output

Verdict per unit: `sufficient`, or the gap list. Binding at entry — a gap list routes to the
design phase, zero gaps routes to cards and build. A **disputed clause defaults to gap and the
dispute goes to the user**; the grader never clears alone.

The verdict lands as **`sufficiency-report.md`** in the feature dir
(`.mochiko/features/FEAT-XXX/`), under the `templates/report-format.md` envelope — the durable
record of whether this batch was buildable on the guidance that already existed. It carries:
the per-unit verdicts · the gap list, each gap keyed to its clause · the store-consult result
and any no-delta claim (clause 4) · store trips awaiting the user's disposition at run-open ·
in-flight conflicts routed to the user · any `[MODIFY]` amendment naming (clause 9) · the
`quickstart.md` null-path record where no real external-integration surface exists.

## Floors — non-waivable

- **Defaults to FAIL** — a unit is insufficient until every applicable clause is graded.
  Absence of looking is never evidence of sufficiency.
- **Never author what you grade** — you authored none of the three sources, and the design
  phase that closes a gap is a different seat.
- **Every clause graded, or flagged n/a with its justification** — never silently dropped.
  Clause 8's no-manifest n/a and clause 9's delta-scope n/a are the only structural ones.
- **Verdict and dispositions land in the report** — evidence living only in conversation is a
  floor violation.
- **Your verdict is input to routing, never a clearing** — the lead routes; the user rules
  trips, in-flight conflicts, and any disputed clause.
