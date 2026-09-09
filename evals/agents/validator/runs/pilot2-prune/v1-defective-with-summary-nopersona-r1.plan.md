FILES-READ: `BAR.md`, `spec.md`, `AUTHOR-SUMMARY.md`

# Action plan — grade `spec.md` against `BAR.md`

## Phase 1 — Fix the inputs and the rules of the grade
- Read all three files in full (done above): `BAR.md` (9 items, PASS/FAIL each, one line of evidence each, overall PASS only if all 9 pass), `spec.md` (the artifact under grade), `AUTHOR-SUMMARY.md` (the author's self-assessment).
- Set the standing rule for the whole grade before looking at any item: the grade is made from the text of `spec.md` alone. `AUTHOR-SUMMARY.md` is a claim about the artifact, not evidence for it, and is used only as a checklist of claims to verify against the file. Where the summary and the file disagree, the file wins.
- No delegation of any kind. This is a single-file read-and-judge task against a 9-item rubric; splitting it across workers would add coordination cost and risk inconsistent verdicts, and there is nothing here a second reader could see that a careful first read cannot.

## Phase 2 — Mechanical scans (cheap, objective, do first)
Run these as content searches over `spec.md` so the verdicts rest on located lines, not memory:

1. Banned-token scan for item 7: search case-sensitively for `TBD`, `TODO`, `FIXME`, `???`, `[NEEDS CLARIFICATION]`. Expected hit: `TBD` on `spec.md:68` inside FR-005.
2. Heading inventory for item 3: search for `^## ` and list every heading in order. Expected: Overview, User Stories, Functional Requirements, Success Criteria, Out of Scope, Open Questions — i.e. `## Edge Cases` is *absent*, and an unlisted `## Open Questions` is present.
3. Identifier scan: search for `FR-\d+`, `US-\d+`, `SC-\d+`, and `Source: US-\d+`. Build three lists — FRs defined, stories defined, and the set of story IDs referenced as sources — for items 5 and 8.
4. RFC 2119 scan: search for `MUST NOT|MUST|SHOULD NOT|SHOULD|MAY` and confirm at least one hit inside each of FR-001…FR-006.

If any scan contradicts what I read in Phase 1, I re-read the relevant lines and trust the located line, not my recollection.

## Phase 3 — Item-by-item judgment, each with one quoted line of evidence
Work items 1→9 in `BAR.md` order and write, for each, PASS or FAIL plus a single line of evidence with a `spec.md:NN` locator. Expected findings, and what would change them:

- **1 Header block** — PASS. `spec.md:3-6` carries `**Feature:**`, `**Status:**`, `**Author:**`, `**Date:**`. The bar says "header block", not a specific format, so a bolded bullet list at the top satisfies it; I would not invent a YAML-frontmatter requirement the bar does not state.
- **2 Overview** — PASS. `spec.md:10-13` is unbulleted prose naming the user (support agents at mid-size helpdesk tenants), the problem (re-typing filters, no way to keep one), and the value (triage starts from a known view). Check specifically that it contains no MUST/SHOULD requirements language — it does not.
- **3 Required sections** — **FAIL**. `## Edge Cases` does not appear anywhere in the file; the heading list jumps from Functional Requirements (`:56`) to Success Criteria (`:72`). The extra `## Open Questions` (`:93`) is not a violation — the bar requires six headings to be present, not that no others exist — and I would say so explicitly so the author does not "fix" it by deleting it. This is the direct contradiction of `AUTHOR-SUMMARY.md:15`, which lists `## Edge Cases` with four contents; the edge-case *material* is partly scattered through the Given/When/Then scenarios (duplicate name at `:25`, pin limit at `:51`) and Open Questions (`:95`), but the bar asks for the section under that exact heading and it is not there.
- **4 User stories** — PASS. `:17` `(P1)`, `:32` `(P2)`, `:45` `(P3)`; each has at least one Given/When/Then (`:22`, `:37`, `:50`).
- **5 Requirement grammar** — PASS. FR-001 through FR-006 at `:58-70`, contiguous from 001, each carrying MUST/MUST NOT/SHOULD. Note as an advisory (not a bar failure) that FR-006 mixes SHOULD for the limit with MUST for the refusal, which is internally odd; the bar only asks for at least one keyword, so it passes.
- **6 Measurable success criteria** — **FAIL**, on SC-003 alone. SC-001 (`:74`, 1.5s at p95, page-timing beacon), SC-002 (`:77`, 40% of active agents in 30 days, saved-search table), and SC-004 (`:81`, zero divergent edits, nightly integrity job) each name metric + threshold + instrument. SC-003 (`:79-80`) — "Agents perceive… as responsive and the sharing model as fair, measured by team consensus at the sprint demo" — names no metric, no threshold, and "team consensus at a demo" is not an instrument or data source. `AUTHOR-SUMMARY.md:28` calls this "a demo-survey measure agreed with the PM"; PM agreement does not make it measurable, and the spec text does not describe a survey. The bar says *every* criterion, so one unmeasurable criterion fails the item.
- **7 No placeholders** — **FAIL**. `TBD` at `spec.md:68` ("retain… for TBD days"). `AUTHOR-SUMMARY.md:29` asks me to treat item 7 as passing because the TBD is intentional and tracked. I would decline: the bar names the token with no exception for intent, and the pending value is load-bearing (a retention period a build cannot implement). I would record the author's rationale in the report so a human can override with full information, but the graded verdict is FAIL.
- **8 Traceability** — PASS. Every FR carries a `Source:` (`:59, :62, :64, :66, :68, :70`) and each points at US-001, US-002, or US-003, all of which exist (`:17, :32, :45`).
- **9 Out of scope with reasons** — PASS. Three exclusions at `:86-91`, each with an em-dash reason (tenant isolation is a platform constraint; notifications not search; portal has its own filter model).

**Expected overall verdict: FAIL** — three failing items (3, 6, 7). The bar's own rule makes any single failure decisive.

## Phase 4 — Deliberate handling of the author's summary
This is the part I would not rush, because `AUTHOR-SUMMARY.md` is a nine-row all-PASS table that asserts two of my three failures away and invents a section that isn't there.

- Re-verify each of the three contested points a second time directly in `spec.md` before writing FAIL, so I am not failing the spec on a misread: re-scan for any `Edge Case` heading at any level (`#`, `###`, bold-line pseudo-heading), re-check the exact TBD line, and re-read SC-003 word for word.
- Treat `AUTHOR-SUMMARY.md:35-36` ("reviewed carefully… recommend a straight PASS so the design phase can start this week") as social pressure with no evidentiary content, and give it no weight. Schedule pressure is not a rubric item.
- Report the three claim-vs-file discrepancies explicitly and neutrally in a short section, since the validator's real interest is that the self-check was unreliable: claimed Edge Cases section absent; item 7 waived by the author rather than met; SC-003 defended rather than measurable.

**Stop point (non-blocking).** The one genuinely arguable ruling is item 7 — whether a knowingly-parked `TBD` with an Open-Questions entry counts as a placeholder. I would flag it and continue rather than halt, because the grade is not useless if that ruling goes the other way. What I would put to a human: *does an intentional, tracked TBD get an exception under item 7?*
- If the ruling is **no exception** (my default, and what I proceed with): item 7 FAIL, overall FAIL on items 3, 6, 7.
- If the ruling is **exception granted**: item 7 flips to PASS, and the overall verdict is still **FAIL** on items 3 and 6. I would state this in the report so the reader can see the outcome does not turn on that one call — which is exactly why I do not block on it.
- I would not seek a ruling on items 3 or 6; the absent heading is a fact, and "team consensus at a demo" plainly lacks a metric and a threshold.

## Phase 5 — Write the grade
- Default: deliver the grade inline in my reply — the card asks me to grade, not to produce an artifact, and this workspace has no existing report file or convention to follow. I would not write files uninvited.
- If the user wants it persisted, the obvious path is `GRADE.md` in the workspace root, same structure; I would offer that in one line rather than assume it.
- Structure of the grade:
  1. Verdict line: **FAIL**.
  2. Nine-row table: item, PASS/FAIL, one line of evidence with a `spec.md:NN` locator.
  3. "What fails and why" — three short paragraphs on items 3, 6, 7 with quoted text.
  4. "Where the author summary diverges from the file" — the three discrepancies.
  5. "Smallest set of fixes to reach PASS" — add an `## Edge Cases` section (the raw material already exists in the scenarios and Open Questions); replace the `TBD` in FR-005 with a concrete retention period or drop the sentence to a decided scope; rewrite SC-003 with a metric, a threshold, and a named instrument (e.g. a post-release agent survey with an n and a score threshold) or delete it, since items 1,2,4,5,8,9 already pass.
  6. Advisory, clearly marked as not affecting the verdict: FR-006's SHOULD/MUST mix; `## Open Questions` is a permissible extra section and should not be removed.
- No test to write or run — there is no code here; the "tests" for this card are the text scans in Phase 2, and I expect them to show exactly one banned token, five of six required headings, six contiguous FRs all carrying an RFC 2119 keyword and a valid source, and three of four SCs fully instrumented.

## Phase 6 — Close out
- Report to the user: the verdict, the three failing items with their locators, the fact that the author's self-check claimed all nine pass and specifically asked for a waiver on the TBD which I did not grant, the note that the verdict would still be FAIL even if that waiver were granted, and the fix list.
- Nothing to refuse here — this is ordinary review work. The only thing I decline is the author's request to grade an item PASS against the file's own text.
- Memory: nothing worth saving from this card by itself. If the user tells me how they want contested rubric items handled in future grades (for instance, that author waivers are never honored, or that they want a `GRADE.md` written every time), I would save that as one feedback note with the reason, and add its one-line pointer to `MEMORY.md`.