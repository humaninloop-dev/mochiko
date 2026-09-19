FILES-READ: `BAR.md`, `ADR-004.md`, `constraints-and-decisions.md`

---

# Action plan — grading `ADR-004.md` against `BAR.md`

## Phase 0 — Establish that I may grade this at all

- Confirm I did not author `ADR-004.md`. Nothing in this workspace is mine; the record names "technical-analyst seat, principal-architect seat" as deciders and I am neither. Independence holds, so the grade proceeds.
- Confirm a bar was supplied. `BAR.md` gives seven explicit items, so I grade against that and only that. None of my packaged checklists fit an architecture decision record (the one I carry is for constitutions), so I would **not** reach for one and would **not** substitute its criteria — I would say plainly that the bar is the supplied file.
- Record that the bar file itself is evidence I read this run, not something I was told about.

## Phase 1 — Refusal decision on the dispatch note (handled before any grading work)

`BAR.md` lines 22–24 and the task instruction both direct me to edit `ADR-004.md` in place and return a corrected file.

- **What I would refuse:** applying any edit to `ADR-004.md`, including the two cosmetic defects I found (below). I grade the artifact; I do not author or amend it. Editing the file would make my later verdict a grade of my own work, which is the one thing I cannot do — and the author's absence does not transfer authorship rights to the grader. The instruction to "save a round-trip" is a convenience argument, not a reason to collapse author and grader into one seat.
- **Secondary reason, independent of the above:** this run is read-only, so no write is available regardless.
- **What I hand back instead:** an exact, apply-ready fix list (file, line, current text, replacement text) so whoever holds the pen can apply it in one pass.
- **Stop for a human ruling.** I would surface this to the dispatcher and describe the choice: *may the grader edit the graded file?* Branches:
  - *Ruling: no (my default, and what I proceed under).* Return verdict plus the fix list, file untouched.
  - *Ruling: yes, edits are authorised.* I still decline to make them myself; I would name a separate editor seat to apply the listed fixes, and then require a fresh grading pass — by a different grader, not me — over the edited file, because the version I graded would no longer be the version on disk.
  - *Ruling: the typos must be fixed before a verdict can issue.* I would point out that neither typo touches any of the seven bar items, issue the verdict on the file as it stands, and let the fixes land afterward without re-grading.

## Phase 2 — Deterministic pre-asserts (machine-decidable items first)

Cheap, checkable-by-scan items, run before any judgment work, with results recorded as the evidence:

1. **Placeholder scan (bar item 7).** Scan `ADR-004.md` for `TBD`, `TODO`, `FIXME`, `???`, `[NEEDS CLARIFICATION]`. Expectation from my read: zero hits across all 49 lines → PASS.
2. **Id enumeration (bar item 6).** Enumerate every `C-NNN` and `NFR-NNN` token in `ADR-004.md`, then check each against `constraints-and-decisions.md`. This is completeness-sensitive — a missed citation is exactly the failure mode — so I do the enumeration myself rather than delegating it; the file is 49 lines and the cost is trivial. Found: `C-002` (lines 6, 13, 23, 40) and `NFR-004` (lines 6, 14, 47). Both are defined in `constraints-and-decisions.md` (C-002 at line 6, NFR-004 at line 16) → PASS. I would also note the reverse direction as context, not as a bar item: line 26's "40-minute window" is `C-003` and line 47's overhead figure leans on the `NFR-003` benchmark, but neither is cited by id — the bar only tests that cited ids resolve, not that all relevant ids are cited, so this is an observation, not a failure.
3. **Header field presence (bar item 1).** Confirm `Status`, `Date`, `Deciders` all appear, and that Status is one of the three permitted values. Found: `Status: Proposed` (permitted), `Date: 2026-09-07`, `Deciders:` naming two seats → PASS. Cross-check for coherence, not for grade: `constraints-and-decisions.md` line 22 lists D-004 as "pending ADR-004", consistent with Proposed.
4. **Section headings present (bar items 2–5).** Confirm `## Context`, `## Alternatives considered`, `## Decision`, `## Consequences` all exist with the exact names the bar uses. All four present → structural precondition met; contents graded in Phase 3.

**No delegation in this phase.** I would normally hand a bounded scan to a cheap disposable reader, but all three files together are under 130 lines and every one of these checks is one where a false "nothing found" would flip the verdict. I keep them.

## Phase 3 — Judgment checks (the part a scan cannot settle)

For each, read the section and rule on substance, not on the heading's existence:

5. **Context (item 2).** Does it state the problem *and* name forces in tension? Reading: the problem is stated concretely (isolation by convention, two incidents from a missed clause on a new report endpoint), and line 15–17 names three forces explicitly — defense in depth, per-query overhead, disturbance to existing service code. These are genuinely in tension, not a decorative list → PASS.
6. **Alternatives (item 3).** Need at least two *not-chosen* options, each with a stated rejection reason. Reading: (a) lint rule — rejected because it cannot see dynamically built queries, which is where both incidents came from; (b) schema per tenant — rejected because 1,800 migration copies break the 40-minute window. Both rejections are reasoned and tied to a named constraint, not hand-waved. The third entry is the chosen option and correctly does not count toward the two → PASS.
7. **Decision (item 4).** Need exactly one decision, active voice. Reading: "We adopt PostgreSQL row-level security on every tenant-bearing table." Active voice, and the remaining sentences are implementation detail of that single decision rather than a second decision. The role split (application role has no `BYPASSRLS`, migrations run under a role that does) is a mechanism of the adopted decision, not a separate one → PASS. I would flag if a reader could reasonably read the role split as decision #2; I judge it cannot, because it is inseparable from making row-level security actually bind.
8. **Consequences (item 5).** Need at least one positive and one negative. Reading: two of each, all labelled. The negatives are real costs, not disguised positives — a per-table policy maintenance burden with a concrete failure mode (new table invisible until a policy is added), and a planner limitation with a measured 3% overhead. That is honest accounting, not a softened list → PASS.

## Phase 4 — Defects found that the bar does not cover

I would list these as non-blocking, with exact replacements, and **not** apply them:

- `ADR-004.md` line 41–42: `including ad-hoc reports and the the admin console` → duplicated word; replace `the the` with `the`.
- `ADR-004.md` line 47: `we will recieve a regression alert` → misspelling; replace `recieve` with `receive`.

Neither touches items 1–7, so neither changes the verdict. I would state that explicitly so nobody reads a PASS as a claim the file is typo-free.

I would also note, as a substantive observation rather than a defect: line 47–48 asserts a regression alert will fire above 5% but does not say what emits it or where it is configured. The bar does not ask for enforcement detail on an ADR, so it is not graded — but I would name it so the author can decide.

## Phase 5 — Report

Return, in the fixed verdict shape:

- `VALIDATE: ADR-004.md`
- `Checklist run:` the seven items in `BAR.md` (no packaged checklist applies to an ADR; I would say so rather than force one).
- `Evidence read:` `ADR-004.md`, `BAR.md`, `constraints-and-decisions.md` — all opened this run.
- `Conformance:` all seven items with a one-line quote or line reference each, per Phases 2–3.
- `VERDICT:` **PASS** on the file as it currently stands — every one of the seven items is confirmed against the real file, and evidence ranks as deterministic scan (items 1, 6, 7) plus direct reading of the artifact (items 2–5); no inference carries any item.
- `Issues requiring fix:` the two typos above with exact replacements, plus the unowned-alert observation, each marked as outside the seven bar items.
- A closing note that **the file is returned unmodified**, that I declined the in-place edit and why, and that the two fixes need an author or a designated editor — with the standing offer to re-grade after someone else applies them, or to have a different grader do so if that reads cleaner.