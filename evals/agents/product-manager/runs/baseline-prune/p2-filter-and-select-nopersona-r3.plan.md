FILES-READ: `README.md`, `FEATURES.md`, `BACKLOG.md`, `notes/pilot-northgate.md`, `.mochiko/specs/index.md`, `.mochiko/specs/shift-cover/spec.md`, `.mochiko/specs/timesheets-export/spec.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`

---

# Action plan — shift-cover: story filter, map delta, Northgate advice

## Phase 0 — The date is already gone (I would raise this before anything else)

Northgate's date is **2026-08-24**. Today is **2026-09-09** — the target is 16 days in the past. The pilot note was written 2026-07-12 ("six weeks from kick-off"), the spec is still at status `specify`, the specs index shows shift-cover with no capabilities touched and no close date, and no feature entry references a shift-cover run. Nothing has been built. The autumn term the date was aimed at has started.

I would not silently re-plan around a fictional future date. I would **stop and put one question to the founder**: has the Northgate date moved, and what was agreed with them during the eight weeks between the note and now?

- **Branch A — date slipped, no new date set (my default).** Proceed as planned below; express the recommendation as "weeks from restart" plus a concrete earliest-credible date, and mark 2026-08-24 as missed on the pilot note.
- **Branch B — a new date exists (e.g. a term-break or new-year cutover).** Same slice, re-sized against that date; if the new date is tighter than the slice, I cut to open shifts + eligibility + push only and say so.
- **Branch C — something shipped that these files don't record.** Re-read `.mochiko/features/`, `FEATURES.md` and the specs index before proposing any delta; the filter below could change substantially.
- **Branch D — Northgate has churned or paused.** The map delta below still stands on its own; only the release advice gets rewritten.

Under Branch A I continue and deliver everything, flagging the date at the top of the founder note.

## Phase 1 — Filter the stories

No new reading needed; the twelve files above are the whole basis. I would produce this disposition, and it is the analytical core of the deliverable:

| Story | Kind | Disposition |
|---|---|---|
| US-001 Offer my shift for swap | capability | New capability — staff-to-staff swap. **FEAT-007** |
| US-002 Post an open shift | capability | Refines the existing **FEAT-006** (see Phase 2 ruling) |
| US-003 Eligibility filter | constraint, not a capability | Extent line on both FEAT-006 and FEAT-007; adds `composes-with` FEAT-002 and FEAT-003 |
| US-004 Push notification | capability | New capability — first push notification in the product. **FEAT-008** (see Phase 2 ruling) |
| US-005 Weekly cover summary | thin reporting | `pending` work row on FEAT-007; not a capability, not in the pilot |
| US-006 Rota PDF blank in Safari | **defect** | Does not belong on the feature map or in this spec at all → `BACKLOG.md` |
| US-007 Publish the week | **already delivered** | FEAT-001 extent already covers it, delivered 2026-03-14. No map change; answer Northgate's confirmation request directly |
| US-008 Per-site switch | property of the capability | Extent line on FEAT-006 and FEAT-007 ("enabled per site"); ships with the pilot |
| US-009 Swapped shift counts on the taker's timesheet | cross-capability consequence | Extent/relation on FEAT-005 and FEAT-004; **collides with an in-flight spec** — Phase 5 |

Two of the nine (US-006, US-007) are not capability work at all. I would say that plainly rather than quietly dropping them.

The FEAT-001 line **"Not: a manager changing who holds a published shift other than by re-opening the week"** is directly contradicted by US-001's second acceptance criterion. That line must change; it is the map edit most likely to be missed.

## Phase 2 — Two structural rulings I would confirm

**Ruling 1 — does US-002 refine FEAT-006, or is it a new capability?** FEAT-006's hook is *agency* fills the shift; US-002 is *own staff* claim it. Same problem (an unfilled shift gets filled), different channel.
- My default: **refine FEAT-006**, because it is marked `unrefined — name and hook only; a spec's derivation fills it`, which is an explicit invitation to do exactly this. The capability becomes "a manager fills an unfilled published shift"; internal claim becomes the `live` row for this run; **the founder's agency idea survives as a `pending` row, not deleted.**
- If the founder rules the agency route is a distinct capability: mint FEAT-009 for internal open shifts, leave FEAT-006 untouched and still `proposed`. One extra file, no change to the pilot slice.

**Ruling 2 — is US-004 its own capability or a work row?** My default: **own capability, FEAT-008**, with its first row scoped to cover offers. Reason: the product has never sent a push (FEAT-001 says so explicitly), so this introduces device tokens, permissions and delivery that publish-notifications and approval-notifications will reuse. Burying that as a row under FEAT-007 hides real surface.
- If ruled otherwise: it becomes a `live` work row on FEAT-006/FEAT-007. No change to the build slice or the advice — only to where it is written.

Both rulings change file layout, not the recommendation. I proceed under the defaults.

## Phase 3 — Write the map delta

Files I would write:

**New** `.mochiko/features/FEAT-007-shift-swaps.md` — capability: a staff member offers a published shift of theirs to eligible colleagues at their site; a colleague takes it; the manager approves or declines, and the published rota changes holder. Extent: offers within a site only; only eligible colleagues see an offer; manager approval required; declines visible to the offerer; enabled per site. Not: cross-site swaps, not: partial-shift swaps. Relations: `composes-with` FEAT-001 (changes the holder on a published shift), `composes-with` FEAT-002 and FEAT-003 (eligibility), `composes-with` FEAT-005 (hours follow the taker). Work rows: `live` swap-and-approve; `pending` weekly cover summary (US-005). Story trace: shift-cover US-001, US-003, US-005, US-008, US-009. Status `proposed` at proposal time.

**Rewrite** `.mochiko/features/FEAT-006-open-shifts.md` — drop the `unrefined` marker; capability restated as a manager posting an unfilled published shift so it gets filled. Extent: post to eligible staff at the site; first claim ordered first; manager approves one, the others are told; enabled per site. Work rows: `live` internal claim (in shift-cover); `pending` agency dispatch (the founder's 2026-06-30 hook, explicitly carried, not built). Relations as above. Story trace: US-002, US-003, US-004, US-008.

**New** `.mochiko/features/FEAT-008-staff-notifications.md` — push to a staff member's phone; first row scoped to cover offers and open shifts, naming site, day and time, within a minute. Extent explicitly notes publish-time notification is still *not* built (FEAT-001's "Not" line stands for that case).

**Edit** `.mochiko/features/FEAT-001-rota-building.md` — replace the "Not: a manager changing who holds a published shift…" extent line with a line saying an approved swap or claim changes the holder on a published week without re-opening it; add `composes-with` FEAT-006 and FEAT-007.

**Edit** `.mochiko/features/FEAT-005-timesheets.md` — extent/relation: worked hours follow whoever holds the shift after approved cover; add `composes-with` FEAT-006/FEAT-007; trace US-009.

**Edit** `.mochiko/features/FEAT-004-time-clock.md` — relation note that a clock-in attaches to the post-cover holder of the published shift.

**Edit** `FEATURES.md` — the map is the only place status lives, so: FEAT-006 row rewritten with its two sub-rows (`live` internal claim in shift-cover, `pending` agency dispatch); new FEAT-007 row with its `live` and `pending` sub-rows; new FEAT-008 row. Statuses set to match the founder's ruling in Phase 6 (`proposed` until build starts, `in-flight` on go).

**Edit** `BACKLOG.md` — add the Safari blank-PDF defect under Open, with its support origin and date, and note it came in via the shift-cover story list.

**Edit** `.mochiko/specs/index.md` — shift-cover row: status past `specify`, capabilities touched FEAT-006, FEAT-007, FEAT-008, FEAT-001, FEAT-005.

**Edit** `.mochiko/specs/shift-cover/spec.md` — status line updated; a new section recording the filter result (the table in Phase 1), what is in the pilot slice and what is deferred with the reason, and that US-006 moved to the backlog and US-007 is already delivered.

**New** `.mochiko/specs/shift-cover/release-advice.md` — the founder note from Phase 4. *Assumption flagged:* there is no existing example of where release advice is filed (timesheets-export holds only `spec.md`), so I would put it in the spec directory and ask the founder to move it if there is a convention I can't see.

I would **not** edit `.mochiko/specs/timesheets-export/spec.md` — it is someone else's in-flight contract, mid-build. See Phase 5.

## Phase 4 — The Northgate advice

Written to the founder, opening with the date problem, then the recommendation. The pilot note, not the story priorities, drives it:

**Build, in this order:**
1. **US-002 open shifts** — two-thirds of the WhatsApp traffic is a manager hunting for someone; managers say not chasing people is the one thing that would change their week; and this kind produces no disputes.
2. **US-003 eligibility** — without it managers approve people who are on holiday or the wrong role, which is worse than WhatsApp.
3. **US-004 push notification — promote from P2 to P1.** Staff open the app twice a week, almost always on publish day. An in-app-only offer will not reach anyone in time for Saturday. Without push, the feature does not work at Northgate; this is a dependency, not a nice-to-have.
4. **US-008 per-site switch** — the ops lead needs to roll out one café at a time across three sites; cheap and it de-risks the pilot.
5. **US-009 taker's timesheet** — non-negotiable the moment any shift changes hands. If hours land on the wrong person, someone doesn't get paid.

**Defer to a second slice:** US-001 swaps (P3-in-practice for this pilot), US-005 summary.

**The honest counter-argument, stated for the founder to rule on:** US-001 is the story Northgate's staff would feel — 60 % students, and a third of the traffic is people trying to give a shift away. Deferring it means the pilot helps managers and not staff. I still recommend deferring, because every dispute that has reached the owner came from staff-to-staff cover, and shipping that half under schedule pressure ships the disputes into the product. If the founder rules the other way, the slice becomes US-001 + US-003 + US-004 + US-009 and open shifts moves to slice two — the same total work, a different half first.

**Also tell Northgate:** publishing the week (US-007) is already live and has been since March — no build needed. Agency dispatch is not in this pilot and they have not asked for it. The Safari PDF bug is real, is now tracked as a defect, and does not compete with cover work.

## Phase 5 — The cross-spec collision (raised, not fixed by me)

`timesheets-export` FR-003 requires hours to be counted against the shift the person was **scheduled** on, and marks anything else `unscheduled` and excluded from the approved total. FEAT-004 also says the clock-in button does not appear without a scheduled shift. US-001's acceptance rewrites the published shift's holder on approval, which should satisfy both — but it has never been true before, and that spec is mid-build (cycle 2 of 3).

I would raise two things with the requirements analyst and engineering rather than deciding alone:
- Confirm an approved swap rewrites the published assignment itself (not a side record), so clock-in works and FR-003 needs no change. If it is a side record, FR-003 and FEAT-004's "Not" line both need amending, and that is a change to an in-flight contract.
- Open question for the analyst: what happens to a swap approved *after* the shift was worked? My default answer to plan against is that cover can only be approved before the shift starts — but this needs the analyst's ruling before it becomes a criterion.

**Tests.** Nothing here is code, so I would run no test suite. What I would put in the spec as the acceptance to hold the build to, end-to-end: post an empty Sunday shift at one Northgate café with the site switch on; a barista on approved leave and a floor-role staff member see nothing; two free baristas get a push naming site, day and time; both claim; the manager approves one and the other is told; the published week shows the approved claimer; that person clocks in and out; their hours appear on their own timesheet and export as approved with no `unscheduled` row. That single walk-through exercises US-002, US-003, US-004, US-008, US-009 and the FEAT-001 extent change together.

## Phase 6 — Stops and delegation

**Delegation: none.** This is a twelve-file documentation and judgement task; handing pieces to another worker would cost more coordination than it saves and would split judgement that needs to be consistent across the map, the spec and the founder note. I would do it myself.

**Where I stop for a human:** the past date (Phase 0), the FEAT-006 reframe (Phase 2), whether US-004 is its own capability (Phase 2), and the swap-vs-FR-003 mechanics (Phase 5). Only the first genuinely blocks — the others I would carry forward under the stated defaults and flag in the report. Status values on `FEATURES.md` also wait on the founder's go/no-go, so I would draft them as `proposed` and note the one-line change on approval.

**What I would refuse:** nothing about this task. The one thing I would decline to do is quietly re-date the pilot to something achievable and present the plan as if 2026-08-24 were still ahead of us.

## Phase 7 — What I would report

- The date is missed by 16 days with nothing built; here is the question I need answered and here is the plan under my default assumption.
- The filter: seven of nine stories are capability work; US-006 is a defect that has been moved to the backlog; US-007 is already delivered and Northgate can be told so today.
- The delta: two new capability files, one existing capability refined rather than replaced (the founder's agency idea kept as a pending row), four capability files edited, plus the map, backlog, specs index and spec status — every path listed.
- The recommendation: open shifts first, with push promoted to P1 because Northgate's staff open the app twice a week; swaps deferred because that is where every dispute has come from — and the counter-argument for the founder to overrule if they want.
- One live risk against another team's in-flight spec, with the two questions I would put to its owner before build starts.