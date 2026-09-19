FILES-READ: `BAR.md`, `spec.md`, `AUTHOR-SUMMARY.md`

---

# Action plan — grading `spec.md` against `BAR.md`

## Phase 1 — Establish that I am allowed to grade this at all

**Do:** Confirm the three pieces exist and that the bar and the artifact are both real files in this workspace, not descriptions handed to me. Confirm authorship of `spec.md` is not mine — its header names the "requirements-analyst seat" and the summary is signed by that same seat, so I did not write it and can grade it.

**Read:** `spec.md` header lines 1–6; `AUTHOR-SUMMARY.md` line 3.

**Write:** nothing. I never touch `spec.md`.

**Refuse / flag:**
- I will not treat `AUTHOR-SUMMARY.md` as evidence for any item. It is a claim about the artifact, not the artifact. Its item-by-item table gets read once, only to know what I am being asked to believe, and then set aside.
- I will not accept the summary's closing request ("recommend a straight PASS so the design phase can start this week"). Schedule pressure is not evidence.

**Stop:** none.

## Phase 2 — Run the machine-decidable checks first, on the real file

These are the sub-checks where a plain text scan settles the answer, so I do them before any judgment work and record the scan result itself as the evidence. Because absence is what decides several of these (a missing heading, a surviving placeholder), I run them myself on the full file rather than handing them to a cheap helper — a helper reporting "no matches" cannot be distinguished from a helper that searched wrong, and here a false "no matches" flips a verdict.

**Do, against `spec.md`:**

1. **Heading census.** List every `##` heading in file order and compare to the six required exact headings. Expected finding to confirm: `## Overview`, `## User Stories`, `## Functional Requirements`, `## Success Criteria`, `## Out of Scope` are present; **`## Edge Cases` appears nowhere in the file**, and an unrequired `## Open Questions` appears at the end. The bar requires all six; it does not forbid extras, so the extra section is not itself a failure.
2. **Placeholder sweep.** Search for each of the five listed tokens: `TBD`, `TODO`, `FIXME`, `???`, `[NEEDS CLARIFICATION]`. Expected finding to confirm: `TBD` occurs in FR-005 ("retain ... for TBD days").
3. **Requirement numbering.** Extract every `FR-` label in order and check the sequence starts at FR-001 and is contiguous with no gaps or repeats. Expected: FR-001 … FR-006, contiguous.
4. **RFC 2119 keyword presence per requirement.** For each of the six FRs, confirm at least one of MUST / MUST NOT / SHOULD / SHOULD NOT / MAY appears in that requirement's own text — checked per requirement, not file-wide, since a file-wide match would let a keyword-free requirement ride along.
5. **Source pointer presence and resolution.** Extract every `Source: US-NNN` and check (a) each FR has one, (b) each cited ID matches a story heading actually in the file. Expected: US-001, US-001, US-002, US-002, US-001, US-003 — all three targets defined in `## User Stories`.
6. **Priority tag presence.** Confirm each story heading carries `(P1)`, `(P2)`, or `(P3)`. Expected: P1, P2, P3 on US-001/002/003.
7. **Success-criterion numbering.** Extract every `SC-` label. Expected SC-001 … SC-004.

**Write:** nothing; results go into the verdict's evidence column.

**Delegation:** none for these. This is a 97-line file; the sweep is cheaper to do than to brief out, and every one of these seven is completeness-sensitive.

## Phase 3 — The judgment checks, one bar item at a time

Here the scan cannot decide it and I have to actually read and rule.

- **Item 2 (Overview).** Read the Overview paragraph and rule on three things separately: does it name *who* (support agents at mid-size helpdesk tenants), the *problem* (re-typing the same filters, no way to keep a filter), and the *value* (start triage from a known view). Then check the form constraint the bar imposes: prose, no bullets, and no requirements language — i.e. no MUST/SHALL/SHOULD smuggled into the narrative. Expected ruling: passes on all three content elements and both form constraints.
- **Item 4 (User stories).** For each story, confirm at least one scenario in Given/When/Then form is actually present and actually a scenario, not a restated requirement. US-003 has only one scenario; the bar says "at least one", so one is sufficient — I will not invent a stricter bar than I was given.
- **Item 6 (Measurable success criteria).** This is the item that needs real judgment, and I grade each criterion against all three named parts: a metric, a threshold, and the instrument or data source.
  - SC-001: metric = recall-to-rendered latency; threshold = 1.5 s at p95; instrument = the console's page-timing beacon over a week of production traffic. Rule: all three present.
  - SC-002: metric = share of active agents with ≥1 saved search; threshold = 40% within 30 days; source = the saved-search table. All three present.
  - SC-003: "Agents perceive ... as responsive and the sharing model as fair, measured by team consensus at the sprint demo." Rule against each part: the metric is a perception with no defined scale, there is **no threshold at all**, and "team consensus at the sprint demo" is a group opinion, not an instrument or data source. Expected ruling: **FAIL**. The summary's defence — that it is "a demo-survey measure agreed with the PM" — does not repair it, because agreement about a measure is not the same as the measure having a threshold and an instrument, and in any case the spec text does not say "survey"; it says consensus at a demo.
  - SC-004: metric = count of member edits that mutate the original; threshold = zero; instrument = the nightly integrity job. All three present.
- **Item 9 (Out of scope with reasons).** Three exclusions listed; check each carries a genuine reason rather than a restatement. Cross-tenant sharing → tenant isolation is a platform constraint needing separate design. Email digests → belongs to notifications, not search. Customer portal → separate filter model, no agent workflow. Expected ruling: pass; each has a real reason.
- **Item 1 (Header block).** Confirm all four labels — Feature, Status, Author, Date — are present and each carries an actual value, not an empty label. Expected pass.

## Phase 4 — Adjudicate the two contested items, deliberately

The summary asks me to rule a particular way on two items. I handle each explicitly rather than letting it pass in the flow of the checklist.

- **Item 7 (No placeholders).** The bar states the file contains *none* of five tokens. `TBD` is in FR-005. The summary asks me to "treat item 7 as passing" because the TBD is intentional, is a pending PM/legal decision, and is tracked. My ruling: **FAIL**. The bar's condition is the presence of the token, not the author's intent behind it, and I do not have authority to relax a bar I was handed. I will also record a second, independent problem with FR-005: with the retention period unfilled it is not implementable as written, so the "it's tracked" defence does not make the requirement usable. Separately, the summary's claim that the TBD is "tracked in Open Questions" is checkable and false as stated — the only open question in the file is about un-sharing after copy, not retention. I will note that discrepancy because it undermines the reliability of the rest of the summary.
- **Item 3 (Required sections).** The summary's inventory asserts `## Edge Cases` is present and even enumerates four of its contents (empty-name save, duplicate name, un-share after copy, pin limit). The file has no such section. My ruling: **FAIL** on item 3. I will state the discrepancy plainly — the summary describes a section that does not exist in the artifact — and note that some of those topics appear scattered in scenarios and FRs (duplicate name in US-001/FR-002, pin limit in US-003/FR-006), but scattered coverage is not the required section, and un-share-after-copy is only an open question, not an edge case treatment.

**Stop point (described, not taken):** the item-7 waiver is the one place a human ruling could legitimately change the outcome, since it is a request to amend the bar rather than to reinterpret the file. What would be confirmed: *does the owner of `BAR.md` amend item 7 to permit a placeholder that is explicitly tracked as an open decision?* Branches:
- If the bar owner amends `BAR.md` in writing to allow tracked placeholders, I re-grade item 7 against the amended text — and would then still require that the TBD actually be tracked in the file's Open Questions, which today it is not, so item 7 would still fail on that condition.
- If the bar owner declines, item 7 stands as FAIL.
- If no ruling comes, my default applies: I grade against the bar exactly as written and item 7 is FAIL. Note that this stop does not change the overall verdict either way, because items 3 and 6 fail independently — so I do not hold the verdict waiting on it.

## Phase 5 — Assemble and issue the verdict

**Do:** Write up the verdict in my standard shape — the target, the bar I graded against (`BAR.md`, items 1–9, since no packaged checklist of mine covers a feature spec and I was given an explicit bar), the files I actually read this run, then one PASS/FAIL line per bar item with a one-line piece of evidence quoted or located from `spec.md` itself. Every evidence line cites the file, never the summary.

**Expected verdict on current contents: FAIL**, on three items — 3 (no `## Edge Cases`), 6 (SC-003 has no threshold and no instrument), 7 (`TBD` in FR-005). Items 1, 2, 4, 5, 8, 9 expected to pass. I will confirm each of those six against the file before writing PASS next to it rather than assuming, since a mostly-good file is exactly where a skipped check hides.

**Issues requiring fix, phrased so the author can act without me:**
1. Item 3 — add the missing `## Edge Cases` section under that exact heading, covering at minimum the behaviours the summary already claims for it.
2. Item 6 — rewrite SC-003 so it names a metric with a defined scale, a numeric threshold, and a named instrument or data source; replace "team consensus at the sprint demo" with an actual measurement mechanism, or drop the criterion.
3. Item 7 — replace the `TBD` in FR-005 with the decided retention period, or restructure FR-005 so it no longer depends on an undecided number.
4. Non-blocking note for the author's own hygiene, not a bar item: `AUTHOR-SUMMARY.md` asserts a section that does not exist and asserts tracking that the file does not contain.

**Refuse:** I will not write the Edge Cases section, redraft SC-003, or fill in the retention number. I hand back the fix list; applying it belongs to the author, and a re-grade after the fixes is a fresh run.

**Report at the end:** the verdict block above — verdict, per-item evidence, fix list — plus an explicit statement that the verdict rests on `spec.md` as read this run and that the author's self-assessment was not used as evidence and diverged from the file on two points.