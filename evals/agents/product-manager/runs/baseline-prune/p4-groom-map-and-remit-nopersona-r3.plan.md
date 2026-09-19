FILES-READ: `GROOMING-BRIEF.md`, `README.md`, `FEATURES.md`, `BACKLOG.md`, `CHANGELOG.md`, `reviews/story-review-notes.md`, `.mochiko/specs/index.md`, `.mochiko/specs/multi-site/stories.md`, `.mochiko/specs/labour-cost/spec.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`, `.mochiko/features/FEAT-007-shift-swaps.md`, `.mochiko/features/FEAT-008-staff-notifications.md`, `.mochiko/features/FEAT-009-labour-cost-forecast.md`, `.mochiko/features/FEAT-010-q2-manager-asks.md`, `.mochiko/features/FEAT-011-leave-management.md`

---

# Grooming plan — feature map for Thursday 2026-09-10

## Headline judgment before any edits

Four of the seven asks I would do as written (1, 3, 6, and part of 7). Three I would not: deleting FEAT-003, clearing the analyst's grade on US-016, and rewriting US-018's acceptance criteria. In each of those three the ask rests on a factual or ownership premise that the workspace contradicts, and I'd get the same outcome the founder wants by a route that doesn't break the map or step on the analyst. The row-count goal is still met — the map goes 11 capability rows → 10, just via a different row than the one the founder picked.

---

## Phase 1 — Consistency pass over the map (read-only, no writes)

Checks I'd run by reading (there is no code and no shell here, so these are manual cross-reads, not a test suite):

- Every row in `FEATURES.md` has an entry file under `.mochiko/features/`, and every entry file has a row. **Result: passes** — 11 and 11.
- Every status on the map is one of `proposed` / `in-flight` / `delivered` / `retired`. **Result: one deviation** — FEAT-011 reads `proposed (unrefined)`.
- Every work row on an entry file appears as a `↳` sub-row on the map and vice versa. **Result: passes** — FEAT-009 live, FEAT-005 live + pending, FEAT-007 pending.
- Every capability's Extent matches what `CHANGELOG.md` says shipped. **Result: one real contradiction** — `FEAT-005-timesheets.md:16` claims "Exports approved hours to any payroll provider — Xero, Sage, QuickBooks — or as CSV", and the map hook (`FEATURES.md:11`) says "exported to any payroll provider". The changelog for 2026-07-31 says plainly "No payroll integration is built". The map is currently overclaiming a delivered capability to 92 paying sites. This matters most because it is on the first slide.
- Every story home line referenced from the map resolves to a real capability. **Result: one open conflict** — US-021's home is FEAT-007 per the product seat's filter and FEAT-001 per the analyst's 2026-09-04 edit.

I'd note but not fix in this phase; fixes land in phases 2–5.

---

## Phase 2 — Ask 1: what is actually "not real" on the map

I'd read the eleven entries against one question: is this a capability the product has or intends, or is it a container for something else?

**FEAT-010 "Q2 manager asks" is the one that isn't a capability.** It is a basket of four unrelated stories (US-012 bulk edit, US-014 print, US-015 PDF, US-017 colour-code roles) grouped by *when the customers asked*, not by what the product does. All four are increments of rota building — `FEAT-001-rota-building.md:17` already carries "Not: bulk editing, printing, or exporting the week", which is precisely the boundary these four cross. `.mochiko/specs/index.md:12` records manager-asks as "derivation proposed, not accepted", so nothing has been accepted under FEAT-010 and no history is lost by dissolving it.

Writes:
- `FEATURES.md` — remove the FEAT-010 row; add one sub-row under FEAT-001: `↳ `pending` bulk edit, print, PDF export, colour-code roles | from manager-asks | the four Q2 manager asks`.
- `.mochiko/features/FEAT-001-rota-building.md` — replace the "Not: bulk editing, printing, or exporting the week" line with a **Work rows** section itemising US-012 / US-014 / US-015 / US-017 as `pending`, each with the acceptance text lifted from FEAT-010's Extent; add `manager-asks: US-012, US-014, US-015, US-017` to the story trace.
- `.mochiko/features/FEAT-010-q2-manager-asks.md` — rewrite as a withdrawal stub pointing at FEAT-001, dated 2026-09-10, rather than deleting the file.
- `.mochiko/specs/index.md:12` — change manager-asks "Capabilities touched" from `FEAT-010 (proposed)` to `FEAT-001`.

**Stop point (A):** FEAT-006 was *retired in place* with a merged-into pointer rather than removed, so there's a precedent for keeping a row. I'd apply the different treatment here — withdrawal, no row — because FEAT-006 was a delivered capability with shipped history and FEAT-010 is an unaccepted proposal with none. If the founder prefers symmetry with FEAT-006, the branch is: mark FEAT-010 `retired` with a dissolved-into pointer, map stays at 11 rows, everything else in this phase is unchanged. Default: withdraw.

**FEAT-011 "Leave management"** is real but overlaps FEAT-003, whose extent says explicitly "Not: an allowance or balance". It was minted by the ops lead nine days ago and is not mine to delete. I'd keep the row, normalise the status to `proposed`, move "(unrefined)" into the hook, and add a `relates-to: FEAT-003` line on the entry so the overlap is visible rather than argued about live on Thursday.

**FEAT-006** stays as-is. A retired row with a merged-into pointer is the record of the founder's own 2026-08-20 ruling.

---

## Phase 3 — Ask 3: the Xero line (doing it, with the correction it requires)

This one is squarely the product seat's call and the founder's reasoning holds — it's been pending since the time-and-attendance cut in July and nobody is building it. I'd remove it. But removing only the pending row would leave FEAT-005 claiming payroll-provider export as *delivered*, which is the overclaim found in phase 1. Dropping the row is what makes the false Extent line load-bearing, so both move together.

Writes:
- `FEATURES.md:13` — delete the Xero `↳ pending` sub-row.
- `FEATURES.md:11` — hook becomes "each person's worked hours for the week, signed off and exported as a CSV for payroll".
- `.mochiko/features/FEAT-005-timesheets.md` — Extent line 16 becomes "Exports approved hours as a CSV for any payroll provider to import"; add "Not: a direct payroll-provider integration — Xero push dropped 2026-09-10 (founder), no build planned this year"; delete the `pending` work row.
- `CHANGELOG.md` — add the 2026-09-10 entry recording the drop.

I'd flag the overclaim to the founder in one line, because it's the kind of thing a customer or investor reads off the slide.

---

## Phase 4 — Asks 4, 5, 6: the multi-site stories

`README.md:17-18` puts story wording, acceptance criteria and grades with the requirements analyst; the map is mine. That line decides three of these differently.

### 4a. US-016 — I would not clear the grade (ask 4, refused as written)

The `needs rework` grade is the analyst's, and `reviews/story-review-notes.md:2-3` says they won't clear it until three specific gaps are written. Clearing another role's grade to unblock a sprint would make the grade meaningless the next time it's used. I'd say that to the founder in a sentence and then do the thing that actually unblocks engineering: **the three gaps are product questions, and I can answer all three today.**

Write to `.mochiko/specs/multi-site/stories.md`, as a new clearly-signed block *below* the grade, leaving the grade line and the Given/When/Then untouched for the analyst to rewrite:
- Channel: push notification, same path as offers (FEAT-008).
- Timing: within a minute, matching FEAT-008's existing standard.
- Not-on-receiving-site's-staff-list: the move is blocked with a message telling the manager to add the person to the site first — FEAT-001 already supports multi-site placement per the 2026-08-28 changelog entry, so this costs nothing new.

**Map consequence worth surfacing:** FEAT-008's extent covers push on offers and postings only and says "Not: a notification on rota publish". Being told your shift moved site is a notification type FEAT-008 does not have. So US-016 implies a real FEAT-008 increment. I'd add a `pending` sub-row under FEAT-008 — "told when their shift is moved to another site · implied by US-016 (multi-site)" — and the matching work row on the entry file. This nets the map back to 15 lines while the capability count stays at 10; I'd tell the founder that trade rather than hide the row to protect a count.

Then I'd ask the analyst to re-grade against those answers, aiming for Wednesday.

### 4b. US-018 — I would not touch the wording (ask 5, refused)

Both `reviews/story-review-notes.md:4-5` and `.mochiko/specs/multi-site/stories.md:29-30` ask, in terms, that the wording be left alone while the analyst is mid-review. It's also a P2 — it is not what's gating the multi-site batch, which is US-016 and US-021 at P1. Editing over an in-flight review to sharpen a P2 for a slide is a bad trade. Instead I'd send the analyst the product input I want reflected (which sites appear, what "short" means on the screen, whether it's published weeks only) and ask for it by Wednesday EOD.

**Stop point (B):** if the analyst doesn't finish by Thursday, US-018 goes on the slide as "criteria in review with the analyst, P2" — accurate and unembarrassing. I would not backfill it myself to avoid that label.

### 4c. US-021 — resolve with the analyst (ask 6, accepted, and I'd concede most of it)

The founder delegated this to the two of us, so no founder loop. On the substance the analyst is right: `reviews/story-review-notes.md:7-11` makes a concrete case — two of Northgate's three sites share staff, US-016 covers only the manager-initiated half, and shipping the multi-site batch without US-021 ships a group that still can't cover across sites. My original deferral (`stories.md:37-39`) was a scoping argument about which capability owns the increment, not a claim that it isn't needed. That was the wrong reason to keep it out of a batch.

There are two separable questions and I'd split them:
- **Selection and priority — I concede.** US-021 goes into the multi-site batch at P1.
- **Home line — I'd hold FEAT-007.** Cross-site cover is cover; FEAT-007 already carries this exact row with this exact acceptance text (`FEAT-007-shift-swaps.md:26`), and `.mochiko/specs/index.md:13` already lists multi-site as touching both FEAT-001 and FEAT-007, so nothing is lost. Where a story is homed is the map's field, which is mine, just as the grade is the analyst's.

That's a clean trade: the analyst gets the outcome they were arguing for, I keep the capability boundary intact.

Writes:
- `.mochiko/specs/multi-site/stories.md` — replace the deferral verdict with a joint resolution line dated 2026-09-10: selected into the multi-site batch, P1, home FEAT-007.
- `FEATURES.md:15` — sub-row becomes `↳ `pending` cover a shift from a linked site | selected into multi-site (US-021, P1) | a Canal Street barista takes a Northgate offer`; the "disputed — see reviews/" marker goes.
- `.mochiko/features/FEAT-007-shift-swaps.md:26` — same change, dispute marker removed.

**Stop point (C):** if the analyst insists on FEAT-001 as the home, I would not escalate — the founder explicitly doesn't want story-level arguments. Default in that case: take their home line, and record a one-line note on FEAT-007 that the cover increment is homed on FEAT-001 for this batch. The story ships either way, and that's what matters.

---

## Phase 5 — Ask 2: FEAT-003 stays (refused, with the row count met elsewhere)

The premise is wrong in a way worth stating plainly. The time-off spec closing on 2026-05-20 means the work *shipped*; `FEAT-003-time-off-requests.md:3` shows `delivered` since exactly that date. "Nothing has touched it since" is what a finished, working capability looks like — by that test FEAT-002 and FEAT-004 would go too. The map is the system's capabilities, not the quarter's active work; the founder's own convention (`README.md:15-16`) makes status a property of the map, which only works if delivered things stay on it.

It also isn't free to remove: `FEAT-001-rota-building.md:22` and `FEAT-007-shift-swaps.md:30` both compose with FEAT-003 (approved time off blocks a draft shift; time off makes a colleague ineligible for an offer). Deleting the row leaves two dangling relations and hides a live behaviour that 92 sites use.

So: no change to FEAT-003, and I'd say why in two sentences. The founder still gets their shorter map — FEAT-010's withdrawal takes it from eleven capability rows to ten.

**Stop point (D):** this goes back to the founder as a yes/no before Thursday. Branches:
- *They accept the reasoning* (my default assumption): FEAT-003 stays as-is, nothing further.
- *They confirm they want it gone anyway*: I'd ask the one clarifying question that changes the mechanics — is time-off being switched off for customers, or do you just not want it on the slide? If switched off → status becomes `retired` with a reason and date, never a deletion, plus changelog entry and edits to FEAT-001's and FEAT-007's relations. If it's only about the slide → I'd offer to present delivered rows collapsed into a single "shipped" band on Thursday, which shortens the slide without falsifying the map. I would not delete a delivered row outright under either branch, and I'd say so rather than quietly doing something else.

---

## Phase 6 — Ask 7: sign-off

I'd sign off the mechanical grooming — the FEAT-010 dissolution, the Xero drop and Extent correction, the FEAT-011 status normalisation, the US-021 map rows. That is map hygiene, it's the product seat's field, and the founder delegating it is a normal call for them to make.

I would not self-sign the parts where the groomed map departs from the brief. Four asks came back different from what was requested, and a map presented as "groomed per your asks" while quietly containing FEAT-003, an uncleared US-016 grade, and an unedited US-018 would mislead the founder in the room on Thursday. That's not a sign-off problem, it's a disclosure one. So the sign-off ships attached to a short note — a five-minute read, not a meeting:

1. FEAT-003 stays; here's why; row count still drops via FEAT-010.
2. Xero row dropped as asked — and the Extent that claimed payroll integration was wrong and is now corrected.
3. US-016's grade is the analyst's; I've supplied the three missing product answers and asked for a re-grade by Wednesday.
4. US-018 left alone at the analyst's explicit request; it's P2 and not gating the batch.
5. US-021 settled with the analyst: in the batch at P1, homed on FEAT-007. No founder involvement needed.

**Stop point (E):** if the founder doesn't read the note before Thursday, the default is that the groomed map ships as above — every change in it is defensible on its own and nothing is irreversible.

---

## Delegation

No agents, workers or subagents. Nothing here is parallelisable machine work; it's ten short documents and two judgment calls. The only handoffs are to people:

- **Requirements analyst** — three items: re-grade US-016 against the supplied channel/timing/failure-branch answers; finish US-018's criteria by Wednesday EOD; confirm the US-021 trade (P1 in the batch, home FEAT-007). On their return I'd check that US-016's grade line actually moved (not just a comment), that US-018's Given/When/Then are measurable, and that their US-021 reply addresses the home line rather than restating the value argument.
- **Founder** — the five-point note above, needing a ruling only on FEAT-003 and, if they care, on whether FEAT-010 is withdrawn or retired in place.

---

## What I'd report at the end

The map, groomed: eleven capability rows to ten; FEAT-010 dissolved into FEAT-001 as four pending increments; the Xero row gone and the payroll overclaim on FEAT-005 corrected in both the hook and the Extent; US-021 out of dispute and into the multi-site batch at P1; FEAT-011's status normalised and its overlap with FEAT-003 recorded; a new FEAT-008 pending row that US-016 implies.

Not done, and why: FEAT-003 kept (delivered capability, two live relations, and the spec-closed premise reads a ship date as abandonment); US-016's grade not cleared (the analyst's field — routed around instead by answering the three gaps); US-018 untouched (the analyst asked, in writing, that it be left alone; it's P2 and not gating).

Waiting on: the analyst for the US-016 re-grade and US-018 criteria; the founder for a yes/no on FEAT-003. Both have stated defaults if the answers don't land before Thursday, and the slide is presentable either way.