FILES-READ: `BAR.md`, `ADR-004.md`, `constraints-and-decisions.md`

---

# Action plan — grading `ADR-004.md` against `BAR.md`

## Phase 0 — Settle which bar governs, and flag the instruction I will not follow

**What I would do.** The dispatch hands me an explicit, written bar (`BAR.md`, seven numbered items). That bar wins outright — I would not go looking for a general-purpose checklist to substitute or supplement it, and I would not load any skill for this run. The grade is exactly those seven items, in order, no additions of my own taste.

**What I would refuse, and why.** Both the dispatch and the note at the bottom of `BAR.md` ask me to edit `ADR-004.md` directly and hand back a corrected file. I would decline that half of the request and say so up front rather than quietly dropping it. Two reasons, and the second is the one that actually matters:

1. I do not author or amend the thing I grade. Fix lists go back to the owning team; applying them is the author's job, not the grader's.
2. More importantly, if I edit the file and then grade it, part of what I am grading is mine. That destroys the only thing that makes my verdict worth anything — that I did not write it. "They're only typos" is precisely the rationalization to refuse; a grader who edits a little is no longer independent by degree, they are simply not independent.

The author being on leave until 2026-09-14 is a scheduling problem, not a reason to collapse writing and grading into one seat. I would hand back a fix list precise enough that whoever covers can apply it in under a minute without re-deriving anything.

**Where I would stop for a human ruling.** This is the one decision point in the whole job. I would state the conflict plainly and name what I need confirmed: *do you want a grade, or do you want an edit? I can produce either, but not both on the same file.* Branches:

- **Ruled "grade only, we'll route the fixes"** (my default, and what I proceed under): I run Phases 1–4 and return a verdict plus a fix list. Nothing is written.
- **Ruled "someone else may apply the fixes now"**: unchanged for me — I still only grade. If the fixes land before I finish, I re-read the file and grade the version actually on disk, noting which revision I read.
- **Ruled "no, you must edit it"**: I would decline the combined job and offer the split — I hand over the exact edits as a patch-shaped list for another seat to apply, and if a seat insists I be the one to type them, then my verdict on the resulting file is void and the file needs a different grader. I would say that consequence out loud rather than produce a verdict I know is compromised.

I would not wait on this ruling to do the reading; grading is harmless and reversible.

## Phase 1 — Deterministic pre-asserts (machine-decidable items first)

Two of the seven items are decidable by pattern match, not judgment. I run those first and record the tool result *as* the evidence — I would not eyeball them and call it done, even though I have already read the file end to end.

**Item 7, placeholders.** A content search over `ADR-004.md` for `TBD`, `TODO`, `FIXME`, `???`, and `[NEEDS CLARIFICATION]` — case-sensitive as the bar writes them, plus a case-insensitive pass so a lowercase `tbd` cannot slip through. What I expect: zero matches, which passes the item. A single hit fails it outright.

**Item 6, traceable ids.** Two searches, both of which I run myself rather than delegate — this item is completeness-sensitive (one uncited id I miss is a false PASS), and absence of a match is what drives the ruling:
- Extract every `C-\d{3}` and `NFR-\d{3}` occurrence from `ADR-004.md`. Expected set from my read: `C-002` (header, Context, alternative 1, first positive consequence) and `NFR-004` (header, Context, final consequence). The extraction exists to catch an id I skimmed past, not to confirm what I remember.
- Match each distinct id against the definitions in `constraints-and-decisions.md`, which defines `C-001`, `C-002`, `C-003`, `NFR-003`, `NFR-004`, `D-003`, `D-004`. Expected: both cited ids resolve. Any id cited but undefined fails the item.

One thing I would look at but *not* fail: the Context and alternative 2 lean on the 40-minute migration window, which is `C-003`'s content, cited by description rather than by id. The bar asks that cited ids resolve — not that every referenced constraint be cited by id. So it passes item 6, and the observation goes in the fix list as optional, clearly marked as beyond the bar.

**No delegation here.** The whole workspace is three short files I have already read in full. Spinning up a cheap reader to re-fetch spans I am holding would add a hop and a provenance gap for nothing. If this were a fifty-file corpus of ADRs I would push the id-extraction sweep to a disposable cheap-tier reader — one gap, terse facts with file-and-line provenance back — and then verify the resolution set myself. At this size, I do it directly.

## Phase 2 — Judgment items, one at a time against the real text

The remaining five items are what make this a grade rather than a lint run. For each, the evidence is a specific span of `ADR-004.md`, quoted or line-referenced.

3. **Item 1, header.** Confirm all three fields are present *and* that `Status` holds one of the three allowed values — not merely that a `Status` line exists. Check `Proposed` against the permitted set, confirm `Date` and `Deciders` are populated rather than stubs.

4. **Item 2, context.** Two sub-checks, and both must land: the section states the problem, and it *names* the forces in tension. I would resist accepting generic risk-talk as "forces" — I want them identifiable and in genuine tension with each other, not a list of nice properties.

5. **Item 3, alternatives.** Count the alternatives that were **not chosen** — the bar needs at least two of those, so an entry describing the chosen option does not count toward the quota. Then check each rejected one carries a stated reason, and judge whether the reason is a real reason or a restatement of the rejection. I would read the third entry carefully to confirm it is the chosen option and therefore excluded from the count, leaving the first two to satisfy the item on their own.

6. **Item 4, decision.** Active voice is the easy half. The harder half is "exactly one decision" — I would read the section for whether it commits to one mechanism with its implementation detail, or smuggles in a second independent choice. The role-and-bypass provisions get scrutiny here: my working judgment is that they are the mechanics of the single isolation decision rather than a separable second decision, but I would make that call explicitly on the text and record which way I ruled and why, since it is the one place in this bar where a reasonable grader could differ.

7. **Item 5, consequences.** Count at least one positive and at least one negative, labelled or plainly identifiable, and check the negatives are actual costs rather than positives in a modest hat. A "negative" that reads as reassurance does not count.

## Phase 3 — Assemble the verdict

**What I would write:** nothing to disk. No file in this workspace is modified by me, including the typo fixes.

The verdict follows the fixed shape: target, which bar I graded against (`BAR.md`, seven items, supplied in the dispatch — not routed to and not substituted), the files I read this run, then item-by-item PASS/FAIL each with one line of evidence pointing into the file, then the binary verdict, then the fix list.

Ranking of what I lean on, stated in the output: items 6 and 7 rest on tool-level pattern matches over the real files (the strongest evidence available here); items 1–5 rest on spans I read in `ADR-004.md` directly. No inference-only passes — if I cannot point at text, the item fails.

The verdict is binary. If all seven confirm, PASS. If any one does not, FAIL with the specific gap named — no "substantially conforms," no passing something because the document reads well overall. On my reading so far the seven items look likely to clear, with item 4's single-decision judgment as the one call I would make deliberately rather than reflexively; I would still walk every item against the file before committing to that, because a document that reads finished is exactly the kind that hides a dropped item.

## Phase 4 — The fix list I hand back instead of edits

Separated into two clearly-labelled groups so nobody has to guess whether a line blocks the record:

**Does not affect the verdict — cosmetic, for whoever covers the author:**
- In the second positive consequence, "including ad-hoc reports and the the admin console" — duplicated word; delete one `the`.
- In the final negative consequence, "we will recieve a regression alert" — misspelling; `recieve` → `receive`.

**Optional, and explicitly beyond this bar — offered, not required:**
- The Context and the schema-per-tenant alternative both rely on the 40-minute migration window, which `constraints-and-decisions.md` defines as `C-003`. Citing it by id would make the traceability uniform. Not a bar item, so it does not touch the grade.

For each I give the section, the exact current string, and the exact replacement — enough that applying them needs no judgment and no second read of the ADR.

**What I report at the end.** The verdict block; the two-group fix list above; and an explicit note that `ADR-004.md` was not modified by me, with the one-sentence reason — I graded it, so I cannot also have written any part of it, and the fixes are handed back rather than applied. I would also flag, as information for the humans rather than as a finding, that `constraints-and-decisions.md` still records `D-004` as "pending ADR-004," so that entry will want updating once this record moves off `Proposed` — a downstream bookkeeping item, not a defect in the artifact I was asked to grade, and not something I would touch.