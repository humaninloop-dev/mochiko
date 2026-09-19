FILES-READ: `.mochiko/specs/time-and-attendance/spec.md`, `.mochiko/specs/index.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `FEATURES.md`, `README.md`, `BACKLOG.md`

---

# Action plan — feature-map delta for `time-and-attendance`

## Phase 0 — Two things I flag before starting

**Date mismatch.** The spec is dated 2026-06-08 and books the analyst's story review for 2026-06-15. Today is 2026-09-11. The request describes that review as "next week," so either the spec has sat untouched for three months or the review date has slipped and nobody updated the file. There is no analyst review artefact anywhere in the workspace. **I would surface this and ask which is true**, because it changes how provisional the entries are.
- If the review genuinely hasn't happened → proceed as planned, entries land marked derivation-provisional pending story review.
- If it happened and the output is elsewhere → I stop and read it first; a review that reshaped the stories could move the extent lines, and deriving against stale stories wastes the write.
- **Default I proceed under:** review not yet held; derive now, mark provisional. The founder asked for the delta ahead of the review and the spec explicitly says not to wait.

**The engineering lead's proposed carve, I would not use — and I would say so in writing rather than quietly ignoring it.** "Sprint 14 — clock-in foundations," "Sprint 15 — hours and export," and "Nice-to-haves" are delivery containers, not things the product does. A customer cannot use Sprint 14. They would also age into nonsense the moment sprints renumber. The offered alternative — one "Time & attendance" entry covering all eleven stories — fails the other way: its honest extent would have to cover clocking, breaks, corrections, overtime, approval, export, SMS, a timezone defect and a database migration, which is nowhere near statable in three lines. Both get a recorded, reasoned no. Neither is a slight on the engineering lead; sprint shape is his job, capability shape is mine.

## Phase 1 — Ground the derivation in what exists

Already done in this read pass, and the findings drive everything downstream:

- Highest existing ID is FEAT-003, so new entries start at FEAT-004.
- **Story ID collision.** This spec's stories are US-001…US-011, but so are `rota-basics`' and `availability`'s. FEAT-001 traces `rota-basics: US-001, US-002...`. Every trace I write must be spec-qualified — `time-and-attendance: US-001` — or the map becomes unreadable.
- **FEAT-001's extent already says two of this spec's stories are out of scope**: "Not: copying a previous week into a draft" (US-007 targets exactly this) and "Not: any notification to staff on publish" (adjacent to US-005). These are pre-existing negative claims that this spec asks to reverse. That is an amendment to a real entry, not a new entry — extending beats minting.
- **Nothing in the map or README says the product stores contracted hours per person.** US-009 (overtime flag) and US-004 (CSV "overtime hours" column) both presume it. This is a genuine hole, not a wording nit.
- `BACKLOG.md` line 7 already carries the SQLite-sidecar `shifts` problem that US-008 restates.

**One cheap delegated read** — a throwaway `Explore` subagent pinned to `model: haiku`. Brief: "In this workspace, list every file that is not a `.md`, and report any occurrence of the strings `contracted`, `contract hours`, `overtime`, `timezone`, `tz`, or `Twilio` in any file, with file and line." Return check: I only trust it to tell me a path exists or a string appears; if it reports any application code or schema, I read that myself, because whether contracted hours exist is a decision-bearing absence and I will not let a cheap read conclude "not found" on my behalf. If it returns only the seven markdown files I have already read, the contracted-hours gap is confirmed as a real gap.

At the point of writing entries I would load `mochiko:authoring-feature-map` and take the entry shape and delta rules from it; the shape I describe below mirrors what FEAT-001–003 already use (Capability / Extent / Relations / Story trace, status header with a since-date).

## Phase 2 — Frame the capabilities, then test them against the stories

The intent — "site managers reconstruct who actually worked from paper and WhatsApp, then retype it for the accountant" — implies three capabilities in the product's own nouns and verbs, before I look at how the stories were sliced:

1. **Recording actual attendance** against a scheduled shift.
2. **A weekly worked-hours record** derived from those recordings, correctable by a manager.
3. **Getting approved hours out** to whoever pays people.

Then I check each story against that frame. The frame held; the stories sharpened the second and third lines and revealed the contracted-hours hole. Proposed delta:

### FEAT-004 — Shift clock-in
- **Capability:** A staff member records the actual start and end of a scheduled shift from the staff app, and records breaks taken during it.
- **Extent (draft):** clock-in offered only within 30 minutes of a scheduled shift start; break start/stop while clocked in, deducted from worked time; one attendance record per scheduled shift.
- **Not:** location or geofence checks; clocking in with no scheduled shift; clocking another person in.
- **Relations:** composes-with FEAT-001 — an attendance record attaches to a published shift.
- **Trace:** `time-and-attendance: US-001, US-002`.
- **Open question for the user:** what happens when someone works an unscheduled shift, or forgets entirely? Today the answer is a manager correction (FEAT-005). If the answer should be "clock in anyway," the extent changes and so does the record model. **Default: no unscheduled clock-in**, matching US-001 as written.

### FEAT-005 — Weekly worked hours
- **Capability:** Worked hours per person per week, derived from attendance records less breaks, which a manager can correct where a clock time is wrong or missing.
- **Extent (draft):** hours per person per week per site; a manager amends a clock time and the original stays visible as an edit; overtime above contracted hours flagged on the week's view.
- **Not:** rounding rules or pay rates; retrospective edits to a signed-off week.
- **Relations:** built-on FEAT-004; feeds FEAT-006.
- **Trace:** `time-and-attendance: US-003, US-009`.
- **Blocking gap I would not paper over:** the overtime line claims a contracted-hours figure the product does not appear to hold. I will not write an extent that flatters. **This is a stop.** Options I would put to the user: (a) add a small fourth capability, staff contract terms, that FEAT-005 and the export both depend on; (b) cut overtime from this spec entirely — drop the flag and the CSV's overtime column — and defer it with a written reason; (c) confirm contract hours already exist somewhere I could not see, in which case the flag stays and I need the source. **Default: (b), cut overtime.** It is a P2 rider on a P1 spec, it silently drags a new data concept into the release, and the churn-risk customers asked for clock-in, not overtime. If the user picks (a), the new entry becomes FEAT-007 and lands ahead of FEAT-005 in the build order; if (c), FEAT-005's extent keeps the flag and the trace keeps US-009.

### FEAT-006 — Hours sign-off and payroll export
- **Capability:** A manager approves each person's hours for the week; approved hours export as a CSV the accountant opens.
- **Extent (draft):** sign-off per person per week; export covers only signed-off people; CSV columns name, site, role, hours (plus overtime only if the gap above resolves that way).
- **Not:** pushing hours to a payroll system; re-opening a signed-off week.
- **Relations:** built-on FEAT-005.
- **Trace:** `time-and-attendance: US-004, US-010`.

Three entries, each nameable in a breath, each with an extent that fits three lines. If FEAT-005's extent grows past that during writing — most likely if correction and overtime both need real elaboration — I split correction out rather than let the entry bloat.

## Phase 3 — Verdict on every story, written down

Eleven stories, eleven verdicts. Nothing gets dropped silently.

| Story | Verdict | Reason |
|---|---|---|
| US-001 clock button | accept → FEAT-004 | core of the capability |
| US-002 breaks | accept → FEAT-004 | same capability; breaks are part of what an attendance record holds |
| US-003 manager fixes a time | accept → FEAT-005 | correction is inseparable from the hours record it corrects |
| US-004 CSV export | accept → FEAT-006 | the "get it to the accountant" half of the intent |
| US-005 SMS reminder | **defer** — not this spec | a real capability, but it is shift notification, not attendance. FEAT-001 currently claims the product does *not* notify staff; reversing that is its own piece of work with its own vendor decision (Twilio). P2, and no customer named it as churn risk |
| US-006 UTC on Manchester rota | **reject from the map** → `BACKLOG.md` | a defect in delivered FEAT-001 behaviour, not a new capability. I would specifically **not** amend FEAT-001's extent to claim site-local times — the product visibly does not do that right now, and writing it would be exactly the flattering entry I refuse. If the user wants the map to carry known-broken state, that is a different and better change I would make deliberately |
| US-007 copy last week | **accept, as an amendment to FEAT-001** | FEAT-001's extent literally says copying is not supported. This deletes a "Not:" line. It is not a new feature and it is not time-and-attendance — it rode along in this spec |
| US-008 shifts → Postgres | **reject from the map** | infrastructure, not something the product does. Already open in `BACKLOG.md` as the sidecar/nightly-copy item. It stays visible as a **prerequisite** for FEAT-004 in the build order, which is the honest place for it |
| US-009 overtime flag | **conditional** — see the FEAT-005 stop | depends on a contracted-hours source the product does not have |
| US-010 sign-off | accept → FEAT-006 | changes what export means; it is why the capability is "approved hours," not "hours" |
| US-011 Xero | **defer, explicitly** | the ops lead already scoped it out. Recorded as a "Not:" line on FEAT-006 plus a dated deferral note. No placeholder entry — a proposed feature nobody is building is how maps start lying |

**Second stop, on US-005.** Deferring it and minting a `proposed` FEAT-007 — Shift reminders are both defensible. I would put it to the user. **Default: defer with a written verdict and no entry yet**, because a proposed entry with no committed work is the seed of map rot, and this one carries an unmade vendor decision. If the user wants it on the map now, I add it as `proposed`, clearly outside the time-and-attendance delta, and note the Twilio dependency.

**Boundary I hold:** these are verdicts on whether a story earns a place on the map. They are not verdicts on how well the stories are written. The analyst's review is a separate craft and I would not pre-empt or edit it. If the analyst comes back holding that a story I rejected — US-008 is the likely one — is load-bearing, that disagreement goes to the user; I do not overrule it and it does not overrule me.

## Phase 4 — Writes

Only after the two stops resolve, or explicitly under my stated defaults if the user wants the delta before answering.

- **New:** `.mochiko/features/FEAT-004-shift-clock-in.md`, `.mochiko/features/FEAT-005-weekly-worked-hours.md`, `.mochiko/features/FEAT-006-hours-sign-off-and-export.md` — status `proposed`, each carrying a note that derivation preceded the analyst's story review.
- **Amend:** `.mochiko/features/FEAT-001-rota-building.md` — remove the "Not: copying a previous week" line, replace with a pending-increment line naming US-007 as cut-but-undelivered, so the entry shows what FEAT-001 owes as well as what it does. FEAT-001 stays `delivered`; the increment is marked pending, not shipped.
- **Amend:** `FEATURES.md` — three new rows with hooks in plain product language ("a staff member taps in at the start of their shift and out at the end, breaks included").
- **Amend:** `.mochiko/specs/index.md` — the `time-and-attendance` row's "Capabilities touched" becomes `FEAT-004, FEAT-005, FEAT-006, FEAT-001`, status moves off "derivation pending."
- **New:** `.mochiko/specs/time-and-attendance/feature-derivation.md` — the full verdict table above, the two stops and their defaults, the recorded no to the sprint-shaped carve and to the single mega-feature, and the US-011 deferral. This is the artefact the analyst and the founder argue with.
- **Amend:** `BACKLOG.md` — add the Manchester timezone defect (US-006). US-008 needs no new line; it restates the existing sidecar item, and I would annotate that item as blocking FEAT-004 rather than duplicate it.

No tests to write or run — this workspace is documentation, with no code and no test harness. The check that matters is the read-back: every one of the eleven stories appears exactly once in the verdict table, every new entry's extent fits three lines, and no entry claims behaviour the product cannot demonstrate. **I would not grade my own writes** — the delta goes to the founder and the analyst for review.

## Phase 5 — Selection advice, with deferral costs visible

A recommendation on order, not a decision. The choice is the founder's.

1. **Sidecar/Postgres prerequisite (US-008 / existing backlog item).** Not a capability, but attendance records need something reliable to attach to, and the nightly copy has already missed a day twice. Deferring it means clock-in data joins to a table that demonstrably loses days — the worst possible failure for payroll numbers.
2. **FEAT-004.** This alone answers what the two three-site groups actually said: "does clock-in." It is the churn-risk answer.
3. **FEAT-005.** Without correction, one forgotten tap and the manager is back on WhatsApp — attendance capture without correction does not survive contact with a real site.
4. **FEAT-006.** Deferring this leaves the Sunday-evening retyping in place, which is half the stated pain. But the accountant already receives unapproved numbers today, so sign-off specifically defers at low cost.
5. **US-007 copy-last-week (FEAT-001 increment).** Small, unrelated to attendance, and it removes a Sunday chore every week for every one of the 41 sites. Cheap enough to be worth an honest look at slotting early despite being P2.
6. **US-005 SMS and US-009 overtime.** Deferral cost of SMS: no-shows continue, unquantified, nobody threatened to leave over it. Deferral cost of overtime: managers spot overspend after export rather than before — real, but it also avoids dragging a new contracted-hours concept into a release aimed at churn risk.

## What I would report

A short note to the founder: three new capabilities — shift clock-in, weekly worked hours, hours sign-off and export — plus one amendment to rota building for copy-last-week; four of the eleven stories do not earn a place on the map and each has a written reason; two decisions are waiting on them (the contracted-hours gap behind overtime, and whether SMS reminders goes on the map now), both proceeding under stated defaults until they rule; the engineering lead's sprint-shaped carve was declined, with reasons on the record and no reflection on the sprint plan itself; and the derivation was done ahead of the analyst's review as asked, so the entries are provisional and I expect the review to sharpen extents rather than move the boundaries.