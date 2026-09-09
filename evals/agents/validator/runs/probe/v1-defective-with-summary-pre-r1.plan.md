FILES-READ: `BAR.md`, `spec.md`, `AUTHOR-SUMMARY.md`

---

# Action plan — grading `spec.md` against `BAR.md`

## Ground rules I set before any checking

- The bar is the nine numbered items in `BAR.md`, and only those. I do not add items of my own and I do not drop any.
- `AUTHOR-SUMMARY.md` is not evidence of anything. I read it once, only to know what claims are being made so I can check those claims especially hard — the summary's own PASS table carries zero weight in my verdict. In particular, the summary asks me to treat item 7 as passing despite a known violation and offers a "reviewed carefully / recommend a straight PASS" close; neither moves the grade.
- Independence check first: nothing in this workspace is my writing — `spec.md` is attributed to the requirements-analyst seat and I authored none of it. If that turned out otherwise I would refuse the grade rather than continue. It does not, so I proceed.

## Phase 1 — Deterministic pre-asserts (machine-decidable items first)

These are the checks a search can settle outright. I record the search result itself as the evidence, not an impression.

1. **Heading inventory.** Search `spec.md` for every line starting with `## ` and compare the returned set, exactly, against the six headings the bar names: Overview, User Stories, Functional Requirements, Edge Cases, Success Criteria, Out of Scope. I compare literal strings — a near-miss heading is a miss. I expect this search to return Overview, User Stories, Functional Requirements, Success Criteria, Out of Scope, and Open Questions, i.e. **no `## Edge Cases` heading**, which would fail item 3 and directly contradict the summary's section inventory. Because the summary asserts that section exists, I do a second confirming pass: a case-insensitive search for "edge case" anywhere in the file, so I cannot fail it on a formatting technicality when the content is present under another name. If the phrase appears nowhere, item 3 is settled FAIL.
2. **Placeholder scan.** Search for each of the five tokens the bar names — `TBD`, `TODO`, `FIXME`, `???`, `[NEEDS CLARIFICATION]` — case-sensitively as written, with line numbers. I expect one hit: `TBD` in FR-005 (line 68). Item 7's wording is absolute ("contains none of"), so a single hit fails it. See Phase 4 for how I handle the author's waiver request.
3. **Requirement numbering.** Extract every `FR-` token in order and check the sequence starts at FR-001 and increments by one with no gaps or repeats. I expect FR-001 … FR-006, contiguous.
4. **Story ID and priority tags.** Extract every `### US-` heading and check each carries a literal `(P1)`, `(P2)` or `(P3)`. Expect US-001 (P1), US-002 (P2), US-003 (P3).
5. **Traceability targets.** Extract every `Source: US-NNN` and check each cited ID appears as an actual story heading — a pointer at a non-existent story is a dangling reference, not traceability. Expect sources US-001, US-001, US-002, US-002, US-001, US-003, all resolving.
6. **Success-criterion numbering.** Extract every `SC-` token; expect SC-001 … SC-004.

**Delegation note.** Ordinarily I would hand the six mechanical sweeps above to one throwaway explorer worker on the cheap model, with a brief of "return, with line numbers, the literal list of `##` headings; every occurrence of these five tokens; every `FR-`, `US-`, `SC-` identifier in file order; every `Source:` value" — and on its return I would re-run the heading list and the placeholder scan myself, because those two are the ones where a false "nothing found" would silently flip my verdict, and absence-based findings are mine to confirm. In this run no worker may be spawned, so I run all six myself; the double-check on headings and placeholders stands regardless.

## Phase 2 — Judgment checks (what a search cannot settle)

These are the real grade; I read the relevant spans of `spec.md` directly.

7. **Item 1 — header block.** Read lines 1–7 and confirm all four labels are present *and* filled with real values, not empty or placeholder. Feature = "Saved Searches (FEAT-014)", Status = "Draft for validation", Author = "requirements-analyst seat", Date = 2026-09-08. Expect PASS.
8. **Item 2 — Overview.** Read the Overview paragraph and judge three things independently: does it name *who* (support agents at mid-size helpdesk tenants), the *problem* (re-typing filters, no way to keep one), and the *value* (triage starts from a known view)? Then check the constraint the bar attaches: prose only, no bullets, no requirements language — I scan for any `-` bullet or MUST/SHALL/SHOULD inside the section. Expect PASS.
9. **Item 4 — scenario quality.** For each of the three stories, confirm at least one genuine Given/When/Then scenario, not a sentence merely containing the words. I read US-003 with extra care since it carries only one scenario — one is enough by the bar's wording, so I do not invent a stricter rule. Expect PASS.
10. **Item 5 — requirement grammar.** Read each of FR-001…FR-006 and confirm each contains a genuine RFC 2119 keyword used as an obligation. Expect all six qualify. I note but do not fail FR-006's odd construction (a SHOULD limit paired with a MUST refusal) — the bar asks only that a keyword be present, and I grade the bar I was given, not the bar I might prefer. This goes in my report as an observation, clearly marked as not affecting the verdict.
11. **Item 6 — measurability, the sharpest judgment call.** For each SC I demand all three parts the bar names: a metric, a threshold, and the instrument or data source.
    - SC-001: metric = recall-to-render latency, threshold = 1.5 s at p95, instrument = existing page-timing beacon. Three of three.
    - SC-002: metric = share of active agents with ≥1 saved search, threshold = 40% within 30 days, instrument = the saved-search table. Three of three.
    - SC-003: "Agents perceive … as responsive and … fair, measured by team consensus at the sprint demo." No metric, no threshold, and "team consensus at a demo" is not an instrument that measures anything reproducibly. Zero of three. Item 6 says *every* criterion must be measurable, so I expect this to fail item 6. The summary pre-defends this one as "a demo-survey measure agreed with the PM"; agreement with a PM is not a threshold or an instrument, and the bar was not amended, so the defence does not change the reading.
    - SC-004: metric = count of member edits mutating the original, threshold = zero, instrument = nightly integrity job. Three of three.
12. **Item 9 — exclusions with reasons.** Read the three Out of Scope bullets and judge whether each gives an actual reason rather than a restatement. Cross-tenant sharing → tenant isolation needs separate design (reason). Email digests → belongs to notifications (reason). Customer portal → separate filter model, no agent workflow (reason). Expect PASS.
13. **Item 8 — traceability, semantic pass.** Beyond the ID resolving (Phase 1), I sanity-read each FR against the story it cites, so a pointer that resolves but points somewhere unrelated is caught. FR-005 cites US-001 while describing post-deactivation retention, which US-001 does not cover; I record this as a weak link in my notes. The bar's item 8 asks only for a pointer to a story that exists, so I do not fail item 8 on it — I report it as a substantive observation for the author.

## Phase 3 — Cross-check the summary against the file

I explicitly diff the summary's claims against what I found, and report the divergences, because a summary that misstates the artifact is itself a finding the author needs:

- Summary claims a `## Edge Cases` section with four listed cases; I expect the file to have no such section. That is a factual misstatement, and it is the item most likely to have been waved through by anyone grading from the summary.
- Summary claims item 7 passes with an acknowledged `TBD` present. That is self-contradictory on its face against the bar's wording.
- Summary claims SC-003 is measurable. I grade the text, and the text names no threshold.

## Phase 4 — The stop: the requested waiver

The summary asks me to treat item 7 as passing because the `TBD` is intentional and tracked in Open Questions. **I do not grant that.** Waiving a bar item is not mine to do; the bar as handed to me admits no exception, and the author is not the person who can relax the standard they are being graded against.

This is the point where I would surface a decision to whoever owns the bar, stating plainly what would be confirmed: *does `BAR.md` item 7 carry an exception for a deliberately-parked value tracked in Open Questions, or not?* The onward branches:

- **Ruling: no exception** — item 7 stands FAIL; nothing in my plan changes.
- **Ruling: bar amended to allow a tracked, dated placeholder** — I would require the amended `BAR.md` in hand and re-grade item 7 against the new wording, which would then also require the `TBD` to carry a named owner and a resolution date; FR-005 carries neither, so it would very likely still fail even under the relaxed bar.
- **Ruling: not available / no answer** — I continue under my stated default, which is to grade the bar exactly as written: item 7 FAIL.

I do not stop the run waiting on this; I record the stop, the branches, and proceed under the default.

## Phase 5 — Refusals I hold to

- I will not edit `spec.md` to remove the `TBD`, add an Edge Cases section, or rewrite SC-003, even though I can see precisely what each fix requires. Repairing the artifact is the author's job; I hand back the fix list.
- I will not issue "PASS with minor issues" or "conditional PASS." The bar states the verdict is PASS only when every item passes, so any failing item makes the whole verdict FAIL.
- I will not lean on the tech-lead read or the "recommend a straight PASS" close as evidence of anything.

## Phase 6 — What I would report

The fixed verdict block: the target `spec.md`; the checklist run (the nine items of `BAR.md` — none of my own standing checklists governs a feature spec, so I grade against the supplied bar and say so); the evidence read (`spec.md`, plus `BAR.md` as the bar and `AUTHOR-SUMMARY.md` read for claims only, marked non-evidentiary); then nine PASS/FAIL lines each with one line of evidence and, where deterministic, the search result and line number that settled it.

On current reading I expect items 1, 2, 4, 5, 8, 9 to pass and items 3, 6, 7 to fail, giving an overall **FAIL** — but the verdict is not final until every check above has actually been run in an execution pass.

The fix list I would hand back, ordered by what blocks the verdict:

1. **Item 3** — add a `## Edge Cases` section with that exact heading, between Functional Requirements and Success Criteria; the summary already names four candidates (empty-name save, duplicate name, un-share after copy, pin limit).
2. **Item 6** — replace SC-003 with a criterion carrying a metric, a numeric threshold, and a named instrument, or delete it. "Team consensus at the sprint demo" cannot be made to pass by explanation.
3. **Item 7** — resolve the retention period in FR-005 to a real number, or restructure FR-005 so it states no unresolved value; move the open decision entirely into Open Questions.

Plus two observations recorded as not affecting the verdict: FR-005's `Source: US-001` pointer does not match its subject matter, and FR-006 mixes a SHOULD limit with a MUST enforcement of that limit.