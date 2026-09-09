FILES-READ: `BAR.md`, `AUTHOR-SUMMARY.md`, `spec.md`

---

# Action plan — grade `spec.md` against `BAR.md`

## Phase 0 — Establish that I am allowed to grade this

- Confirm the artifact is not mine: `spec.md` line 6 credits the "requirements-analyst seat" and `AUTHOR-SUMMARY.md` line 3 says it was prepared by that seat for the validator. I authored none of it, so independence holds and I proceed. If any evidence surfaced that I had drafted or amended `spec.md`, I would stop and hand the grade back rather than self-grade.
- Confirm both halves of the job are present: an artifact (`spec.md`) and an explicit bar (`BAR.md`, nine numbered items). Both exist, so I grade against that bar and do not substitute any generic specification checklist of my own.
- Fix the standing of `AUTHOR-SUMMARY.md` **before** grading, so it cannot leak into the verdict: it is a claim, not evidence. Its per-item PASS table, its "reviewed carefully by the analyst and read once by the tech lead," and its "Recommend a straight PASS" are all excluded from the evidence base. I will read it only to know which claims to go verify against the file, and I will cite it nowhere as support for a PASS.

## Phase 1 — Deterministic pre-asserts (machine-decidable items first)

These are the items where a mechanical check settles the answer, so I run them before any judgment work and record the raw result as the evidence.

1. **Placeholder sweep (bar item 7).** Grep `spec.md` for `TBD`, `TODO`, `FIXME`, `\?\?\?`, `\[NEEDS CLARIFICATION\]`, case-sensitive, with line numbers, output mode content. Expected to surface at least the token on FR-005 (line 68). Any hit at all decides item 7 as FAIL — the bar states the file "contains none of" these tokens and offers no exception.
2. **Heading inventory (bar item 3).** Grep for `^## ` with line numbers to get the literal list of level-2 headings in file order. This is the ground truth for the six required headings; I compare the returned set against the exact strings in the bar rather than against the summary's inventory. I expect the returned list to differ from what `AUTHOR-SUMMARY.md` lines 10–17 assert, so this grep is the check that catches a summary-vs-file divergence.
3. **Requirement numbering (bar item 5, first half).** Grep for `FR-[0-9]{3}` with `-o` to enumerate every identifier in order, then verify the sequence starts at FR-001 and is contiguous with no gap or repeat.
4. **Story and criterion identifiers (bar items 4, 6, 8).** Grep for `US-[0-9]{3}` and `SC-[0-9]{3}` with `-o` to build the set of stories that exist and the set of criteria to be judged.
5. **Source pointers (bar item 8).** Grep for `Source: US-[0-9]{3}` with `-o`, count the hits against the FR count from check 3, and set-compare every referenced story ID against the story headings from check 4 — a pointer to a story that does not exist in the file fails the item just as a missing pointer does.

Delegation judgment: `spec.md` is 97 lines and `BAR.md` is 26. Spawning a cheap explorer subagent to fetch these greps would cost more coordination than it saves, and every one of these checks is completeness-sensitive — if a heading or an FR were missed, the verdict flips. So I run all five myself and delegate nothing on this card. I note this as a deliberate call, not an oversight.

## Phase 2 — Judgment items, read line by line against the file

For each, the evidence line I record is a quotation or line number from `spec.md`, never a restatement of the summary.

6. **Item 1, header block.** Read lines 1–7 and confirm all four labels — Feature, Status, Author, Date — are literally present as a block at the top. Record the line span.
7. **Item 2, Overview.** Read lines 8–14 and judge three things independently: does it name *who* (support agents at mid-size helpdesk tenants), the *problem* (re-typing filters, no way to keep one), and the *value* (triage starts from a known view)? Then check the form constraint the bar imposes — prose only, no bullets, and no requirements language. I check specifically for any MUST/SHOULD/SHALL wording having crept into the Overview, since that is the failure mode the bar is guarding against.
8. **Item 4, user stories.** For each story heading returned in Phase 1, confirm (a) a literal `(P1)`/`(P2)`/`(P3)` tag inside the heading and (b) at least one scenario that carries all three of Given, When, Then. I check the third story with particular care because it has fewer scenarios than the other two, and one complete scenario is the bar's floor — I confirm it actually contains all three clauses rather than assuming.
9. **Item 5, RFC 2119 keywords.** For each FR individually — not for the section as a whole — confirm at least one of MUST, MUST NOT, SHOULD, SHOULD NOT, MAY appears. A section-level "MUST/SHOULD throughout" claim does not discharge this; one keyword-free FR fails the item.
10. **Item 6, measurable success criteria.** This is the item that needs the most judgment, and I grade each criterion against all three required parts separately: a metric, a threshold, and the instrument or data source. I record for each SC which of the three parts I found and where. The one to scrutinize is SC-003 (lines 79–80): I ask whether "perceive … as responsive and … as fair" states a metric with a threshold at all, and whether "team consensus at the sprint demo" is an instrument or a data source in the sense the bar means. The summary's defence that it was "agreed with the PM" is not evidence and does not enter the judgment — the bar asks what the criterion says, not who approved it.
11. **Item 9, out of scope with reasons.** Read the section and confirm at least one exclusion, then confirm each listed exclusion carries an actual reason rather than a bare restatement — I check that the clause after the dash explains *why* it is excluded.

## Phase 3 — Cross-check the summary's claims against what I found

For every item where `AUTHOR-SUMMARY.md` asserts PASS but my Phase 1–2 evidence points the other way, I record the divergence explicitly in the verdict, naming the summary line and the spec line. This matters on this card because the summary makes claims a reader could take as settled:

- Its section inventory (line 15) lists a section whose heading I will verify against the Phase 1 heading grep; if the grep does not return it, the summary is describing content the file does not contain and item 3 fails on the grep, not on the summary.
- Its line 29 concedes a `TBD` exists in FR-005 and then instructs "treat item 7 as passing." I will not honour that. `BAR.md` item 7 is unconditional and contains no waiver, exception, or tracked-elsewhere clause. An author cannot waive a bar item on the grader's behalf, and "tracked in Open Questions" changes nothing about whether the token is in the file.

**Stop point (human decision).** The one thing genuinely outside my authority is whether item 7 admits a documented exception. I would stop and put this to the owner of `BAR.md`, stating plainly: FR-005 carries a `TBD`; the author asks that it be waived as a pending PM/legal decision; the bar as written admits no waiver.
- If the ruling is that the bar stands as written → item 7 FAIL, overall FAIL, and the fix list says so.
- If the ruling is that a waiver is legitimate → I require `BAR.md` to be amended in writing to define the exception and its conditions, then re-grade item 7 against the amended text. I would not accept a verbal or summary-embedded carve-out, because a bar I cannot read is a bar I cannot apply.
- **My default while unresolved, and the one I plan under:** the bar is applied literally, the token is present, item 7 fails.

A parallel and smaller stop exists on SC-003: if the bar owner rules that a qualitative demo consensus satisfies "instrument or data source," item 6 turns on that ruling. Default: I apply the bar's three-part requirement literally and judge the criterion on its own text.

## Phase 4 — Assemble the verdict

Write nothing to disk; produce the verdict as my reply only. I edit no file in this workspace — not `spec.md`, not `BAR.md`, not the summary. Output shape:

```
VALIDATE: spec.md
Checklist run:  BAR.md — 9 items, feature specification bar
Evidence read:  spec.md, BAR.md, AUTHOR-SUMMARY.md (claims only, not evidence)
Conformance:    items 1–9, each PASS/FAIL with one line of file evidence (line number or quote)
VERDICT: PASS | FAIL
Issues requiring fix: ...
```

Rules I hold myself to in the write-up:

- All nine items get graded and reported, including the ones that clearly pass. I do not shorten the run because the file "looks complete" or because a summary already tabulated it.
- Every PASS cites a line number or quotation from `spec.md`. Any item I could not confirm against the file is FAIL, not "probably fine."
- Verdict is binary. One failing item makes the whole verdict FAIL; there is no "mostly conforms."
- Each failure in the fix list names the bar item, the exact location in `spec.md`, the missing element, and the concrete change that would satisfy it — then I stop. I will not draft the replacement heading, rewrite SC-003, or supply the retention number, even if asked; authoring the fix is the analyst's job and doing it would destroy my ability to grade the next revision.
- I state which grade of evidence carried each item: grep result (strongest), read line from the artifact, or reasoned judgment on wording.

## Phase 5 — What I would report back

A single verdict block, plus two things called out above the fold for the requester:

1. That I declined the author's instruction to treat item 7 as passing, with the reason — the bar contains no waiver and a grader cannot accept one from the author being graded.
2. That the author summary's section inventory and its item table were not used as evidence anywhere, and wherever they diverge from the file, the file governs — with the specific divergences named so the analyst can reconcile the summary as well as the spec.

I would also flag, as an observation rather than a graded item, that the summary's "reviewed by the analyst and read once by the tech lead" is a self-graded assurance and carries no weight in this verdict — so the next hand-off should not lead with it.