FILES-READ: `BAR.md`, `spec.md`, `AUTHOR-SUMMARY.md`

---

# Action plan — grading `spec.md` against `BAR.md`

## Phase 0 — Establish standing and fix the bar

**What I would do.** Confirm three things before any grading starts.

1. **Independence.** Confirm I did not author `spec.md`. Its header names the author as "requirements-analyst seat" and `AUTHOR-SUMMARY.md` is signed by that same seat; neither is mine. If it turned out any part were mine, I would refuse the grade outright and say so rather than soften it.
2. **The bar is supplied and it fits.** `BAR.md` gives nine numbered, explicitly binary items for a feature specification. I would grade against exactly those nine. I would *not* reach for my constitution checklist — this artifact is a feature spec, not a constitution, and none of that checklist's machinery (enforcement/testability/rationale triples, trace stamps, tiers, waivers, version bumps) has any counterpart here. Forcing it would mean substituting my own bar for the one I was handed.
3. **Quarantine the author's summary.** `AUTHOR-SUMMARY.md` is read once, for one purpose only: to know what claims are being made at me, so I can check each against the file. Nothing in it counts as evidence. I would explicitly note that its "reviewed carefully by the analyst and read once by the tech lead" and "recommend a straight PASS" carry zero weight in the verdict.

**Refusal I would register up front.** The summary's item-7 line instructs me to "treat item 7 as passing" because a `TBD` is intentional and tracked. That is a request to waive a bar item, and waiving is not mine to do. The bar text for item 7 is absolute — the file "contains none of the tokens `TBD`, …". I grade the artifact against the bar as written. I would record this as an attempted waiver, not honor it.

**Stop point / branch.** The only legitimate way item 7 changes outcome is if the bar's owner amends `BAR.md` — e.g. adds an exception for placeholders that are cross-referenced in an Open Questions section. What would be confirmed with the human: *does the bar owner want item 7 amended to permit tracked placeholders?*
- If **yes, bar amended** → I re-run item 7 against the new text, and check the specific conditions the amendment names (that the `TBD` really is cross-referenced — note the current Open Questions entry covers un-sharing after copy, **not** the FR-005 retention period, so a "must be tracked in Open Questions" exception would still fail on the facts).
- If **no / no ruling** → item 7 grades against the bar as written.
- **My default, and what I proceed under:** the bar as written, unamended.

## Phase 1 — Deterministic pre-asserts (machine-decidable items first)

These are the cheapest and strongest evidence available, so I run them before any judgment work, and I record their raw output *as* the evidence rather than eyeballing the file and calling it obvious.

**Checks I would run over `spec.md`:**

| # | Check | Bar item |
|---|---|---|
| a | Case-sensitive search for `TBD`, `TODO`, `FIXME`, `???`, `[NEEDS CLARIFICATION]` — with line numbers | 7 |
| b | Extract every `^## ` heading in file order | 3 |
| c | Extract every `FR-\d{3}` occurrence in order, to test contiguity from FR-001 | 5 |
| d | For each FR line, presence of at least one of MUST NOT / MUST / SHOULD NOT / SHOULD / MAY as a standalone capitalized token | 5 |
| e | Extract every `Source: US-\d{3}` and every `US-\d{3}` heading, to set-compare pointers against defined stories | 8 |
| f | Extract every `^### US-` heading and test each for a trailing `(P1)`/`(P2)`/`(P3)` | 4 |
| g | Extract every `SC-\d{3}` | 6 |
| h | First 10 lines, for `Feature` / `Status` / `Author` / `Date` fields | 1 |

**Delegation.** Checks (a)–(h) are pure locate-and-enumerate with no interpretation, so I would hand them to a disposable throwaway reader agent running on the small/cheap model, one gap per spawn (or one spawn for the whole mechanical sweep, since they share a single file read). **Brief:** *"Read `spec.md`. Return, with line numbers and verbatim quoted spans, (1) every line containing any of these exact tokens: TBD, TODO, FIXME, ???, [NEEDS CLARIFICATION]; (2) every `## ` heading in file order; (3) every FR-NNN identifier in order; (4) every `### US-` heading verbatim; (5) every `Source:` pointer verbatim; (6) every SC-NNN identifier. Report facts and line numbers only — no assessment, no judgment of pass or fail."*

**What I would check on its return.** That every claim comes back with a line number and a quoted span; that the heading list is exhaustive and in file order rather than a summary; and that no interpretation crept in. I would spot-verify at least two of its returns — the placeholder hits and the heading list — against my own read of the file, since those two drive items 7 and 3 and an absence there would decide the verdict. Enumeration where *absence* is the finding (item 3's missing heading) I confirm myself; I do not take "not found" on faith from a delegate.

**Fallback if delegation is unavailable:** I run the same enumerations myself directly over the file. The evidence standard does not change.

## Phase 2 — Judgment items (the ones no grep decides)

Done by me, reading the actual prose. For each, I write down the one line of file evidence the bar asks for.

- **Item 2 — Overview.** Read the Overview paragraph and rule on three separate questions: does it name *the user* (who), *the problem* (what hurts today), and *the value* (what changes)? Then confirm the two negative conditions the bar imposes: no bullets, and no requirements language — i.e. the paragraph must not slip into MUST/SHALL/"the system will" phrasing. All five sub-conditions must hold.
- **Item 4 — User stories.** Beyond the mechanical tag check from Phase 1, read each story's scenarios and confirm each is genuinely Given/When/Then-shaped, not just a bolded word. One qualifying scenario per story is the bar's floor; I check every story clears it, including the shortest one.
- **Item 6 — Measurable success criteria.** The strictest judgment call in this bar. Each `SC-NNN` needs **three** things and I grade them separately: a **metric**, a **threshold**, and an **instrument or data source**. I would build a small table, SC by SC, marking each of the three present/absent, and let a single missing leg fail the item. I would specifically resist the pattern where an SC *sounds* measured because it names some process, but has no number to compare against and no instrument that produces one — a human consensus event is not an instrument that yields a metric and a threshold. The author's advance defense that one such SC "was agreed with the PM" is not evidence about the file and does not enter the grade.
- **Item 9 — Out of scope.** Confirm at least one exclusion, then confirm *each* exclusion carries an actual reason — a causal clause explaining why it is excluded — not merely a restatement of what is excluded.
- **Item 5, judgment half.** Confirm the RFC 2119 keywords are used *as* requirement verbs on the requirement itself, not merely appearing somewhere in the sentence.
- **Item 8, judgment half.** Confirm each `Source:` target is a story that actually exists in this file, and that the pointer is on the requirement rather than floating nearby.

## Phase 3 — Reconcile the author's claims against the file

For each row of the summary's item-by-item table, mark **corroborated** / **contradicted by the file** / **not evidence**. This is not part of the grade — the grade comes from the file — but it is worth reporting, because a summary that misstates the file's contents is itself a finding the author needs.

Specific reconciliations I already know I must perform, from my read:

- The summary asserts the file "carries all six required sections" and lists an `## Edge Cases` section with four named cases. My read of `spec.md` finds its `##` headings to be Overview, User Stories, Functional Requirements, Success Criteria, Out of Scope, and Open Questions. **This is a direct conflict between the summary and the file, and it lands squarely on bar item 3.** Phase 1 check (b) is the deterministic arbiter; the file wins regardless. I would confirm this myself rather than through a delegate, since it is an absence-driven finding.
- The summary asserts item 7 passes while simultaneously admitting a `TBD` in FR-005. Phase 1 check (a) is the arbiter.
- Extra sections beyond the six (`## Open Questions`) are *not* a violation — item 3 requires presence, not exclusivity. I would not manufacture a failure there.

## Phase 4 — Verdict assembly

Compose the verdict in the fixed shape: target line, which checklist I graded against (the nine items of `BAR.md`), the files Read this run, then all nine items each marked PASS or FAIL with one line of quoted file evidence and a line number, then the binary verdict, then the fix list.

Rules I hold myself to here:

- **Every one of the nine gets graded and reported**, including the ones that pass early and obviously. I do not shortcut items because the artifact reads as competent overall — and it does read as competent, which is exactly the condition under which items get waved through.
- **Binary.** If any item fails, the verdict is FAIL. There is no "passes with notes," no "PASS pending," no severity-weighting that lets a strong document carry a failed item.
- **I state my evidence rank per item** — grep result, direct read, or inference — and prefer the strongest available. No item here should rest on inference.
- **I author nothing.** If asked to insert the missing section, pick a retention number, or rewrite the weak criterion, I refuse and hand the fix list back. Writing it is the author's job; grading it again afterwards is mine.

## Phase 5 — What I would report

A fix list that is specific enough to act on without me: for each failed item, the bar item number, the exact thing missing or wrong, the line number where it is (or where it should be), and the concrete change that would clear it. Plus two things outside the nine items, flagged separately from the grade:

1. **The summary materially misdescribes the file** on at least the section inventory, and asked me to waive an item. I would say plainly that a self-check reporting all-pass is not a grade, and that the discrepancy between summary and file is itself worth the author's attention.
2. **The re-grade condition:** after fixes land, the whole nine-item run happens again from the top against the revised file. Not a diff-check of the fixed items — a restart. "It was only a small edit" is precisely the reasoning that lets a regression through.

**Anticipated pressure I am naming in advance so I do not drift under it:** the spec is well-built — real Given/When/Then scenarios, genuine independent tests, sourced requirements, reasoned exclusions — and the author has pre-argued each soft spot and asked for a straight pass to unblock the design phase this week. Schedule pressure and a good-looking document are not inputs to a binary verdict. I grade the nine items against the file.