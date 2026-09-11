FILES-READ: `README.md`, `FEATURES.md`, `BACKLOG.md`, `notes/pilot-northgate.md`, `.mochiko/specs/index.md`, `.mochiko/specs/shift-cover/spec.md`, `.mochiko/specs/timesheets-export/spec.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`, `.mochiko/features/FEAT-004-time-clock.md`, `.mochiko/features/FEAT-005-timesheets.md`, `.mochiko/features/FEAT-006-open-shifts.md`

---

# Action plan — shift-cover: filter, map delta, pilot advice

## Phase 0 — Two things I would raise before writing anything

**0a. The target date has already passed.** Today is 2026-09-09. Northgate's date is 2026-08-24 — sixteen days ago — and the spec is still at `specify` with nothing built and no in-flight run. The pilot note (2026-07-12) describes the date as "six weeks from the shift-cover kick-off"; the spec opened 2026-07-13, so the six weeks ran out around 2026-08-24. Nothing in the workspace records a slip, a re-plan, or a conversation with Northgate.

I would not block on this. The scope advice below — what to build first and why — is the same content whichever way the calendar resolves, so I would produce it in full against a six-week budget and label the budget as "six weeks from whatever date you confirm."

**What I would ask the founder to confirm:** is 2026-08-24 a missed date, a date that has since moved, or a typo for a later date?
- *If missed / moved:* I re-date the advice to six weeks from the new commitment, and add a line to the advice note recording that the original date passed undelivered — Northgate should hear that from the founder, not discover it.
- *If the real date is further out (e.g. autumn term 2027):* the ordering is unchanged; the "cut" list (US-005) can come back into scope and swaps stop competing with open shifts for the same window.
- *Default I proceed under if no answer:* the date is missed, the six-week budget starts now (2026-09-09, landing ~2026-10-20), and scope is cut as if the window were still tight — because a customer already waiting is the case for shipping less, sooner, not more.

**0b. One map decision is the founder's, not mine.** FEAT-006 "Open shifts" was minted by the founder on 2026-06-30 with the hook *"a manager sends an unfilled shift to a staffing agency and the agency fills it."* The spec's US-002 uses the same words for a different thing: a manager posts an unfilled shift to **their own staff**, who claim it. These are not the same capability — different fillers, different money, different integrations.

**What I would confirm:** does FEAT-006 get refilled as internal open shifts, with the agency route recorded separately, or does the agency idea keep FEAT-006 and internal claiming get a new ID?
- *If refill (my recommendation, and the default I proceed under):* FEAT-006's capability text is rewritten to internal posting and claiming, and the agency route is preserved as a `pending` work row on the same entry so the founder's intent is not silently deleted. The convention supports this — the entry is explicitly marked `unrefined`, "name and hook only; a spec's derivation fills it," and this is that derivation. Nothing in the pilot evidence or in any reviewed story supports agency fill: Northgate's problem is 20–30 WhatsApp messages a week between people who already work there.
- *If the founder wants agency to keep FEAT-006:* internal open shifts becomes FEAT-007, staff swaps FEAT-008, push FEAT-009, and FEAT-006 stays `proposed (unrefined)` and out of the pilot. Ten minutes of renumbering; everything else below is unchanged.

I would not touch a founder-minted entry's meaning without that ruling landing one way or the other, but I would draft both and carry on.

---

## Phase 1 — Filter the nine stories

I would work through each story and put it in exactly one destination, writing the reasoning down rather than just the verdict, because three of these do not go where their priority label suggests.

| Story | Verdict | Destination |
|---|---|---|
| US-001 Offer my shift for swap (P1) | New capability | **FEAT-007 Shift swaps** — `live` row |
| US-002 Post an open shift (P1) | Fills the existing unrefined entry | **FEAT-006** — capability text + `live` row |
| US-003 Eligibility (P1) | **Not a capability** — a rule on both | Extent lines + `composes-with` on FEAT-006 and FEAT-007 |
| US-004 Push when an eligible shift is posted (P2) | **New capability** — the product has none today | **FEAT-008 Push notifications** — `live` row for cover alerts |
| US-005 Weekly cover summary (P3) | Too small for an entry | `pending` work row on FEAT-007 |
| US-006 Safari PDF blank (P1) | **Defect, not capability** | `BACKLOG.md` |
| US-007 Publish the week (P1) | **Already delivered** | No map change — FEAT-001 already covers it |
| US-008 Per-site switch (P2) | Rollout mechanics, not a capability | Extent line + `live` row on FEAT-006/FEAT-007 |
| US-009 Swapped shift on the taker's timesheet (P2) | Real, and **mis-prioritised** | Extent + relation on FEAT-005; rides with FEAT-007 |

Three of these need their reasoning stated on the record, since they contradict the labels the analyst put on them:

**US-004 is not a P2.** Northgate's staff open the app **twice a week, almost always on publish day**. Both US-001 and US-002 are claim-races: someone posts a shift on Wednesday for Saturday, and the first eligible person to see it takes it. A staff member who opens the app on publish day will see that post on the following Monday. Without push, the manager posts an open shift into a room nobody is in, and goes back to WhatsApp — which is the exact behaviour the release is meant to replace. Push is not an enhancement to shift cover at Northgate; it is the delivery mechanism that makes it work at all. It also has no existing home: FEAT-001's extent says outright *"Not: any notification to staff on publish or change — staff open the app to see the week."* Building it opens a genuinely new capability that publish-notifications and time-off decisions will later want, which is why it gets its own entry rather than an extent line on swaps.

**US-009 is not a P2 either, and it is the sharpest finding in this pass.** The in-flight timesheets-export spec, FR-003, states: hours are counted against the shift the person was **scheduled** on, and a clock-in with no matching published shift is marked `unscheduled`, **excluded from the approved total**, and kicked back to the manager. FEAT-004's extent is harder still: *"Not: clocking in without a scheduled shift — the button does not appear."* So if an approved swap does not rewrite who holds the published shift, the colleague who covers a Saturday cannot clock in at all — and if they somehow do, their hours are excluded from the CSV the accountant pays from. Northgate has 38 staff, mostly students, and has accepted the CSV as their payroll path. Shipping swaps without US-009 means shipping unpaid shifts.

The good news is that US-001's own second acceptance criterion already says the fix: *"the published rota shows the colleague on the shift and I am off it."* If approval genuinely reassigns the published shift, US-009 may cost very little beyond a test. I would not assert that — I would ask the engineers whether approval is planned as a reassignment of the existing shift record or as a separate cover-assignment layer, and record the answer. If it is a separate layer, US-009 is real work and swaps cannot ship without it.

**US-006 and US-007 should not have been in this spec.** US-006 is a support defect with no story shape; the workspace convention puts defects in `BACKLOG.md`. US-007 is already live — FEAT-001's extent covers publishing, and FEAT-001 already traces a US-007 from the time-and-attendance run, so leaving this one here would corrupt the trace. Neither belongs on the map. I would mention both to the requirements analyst as a filtering note, not file it as a complaint — a customer's requirements list is a reasonable place for both to have come from.

---

## Phase 2 — Write the map delta

Everything here is proposed; I would put it in front of the founder as a diff before it lands, given Phase 0b and the FEAT-001 amendment.

### 2a. Rewrite `.mochiko/features/FEAT-006-open-shifts.md`

Status `proposed (unrefined)` → `proposed`. Capability: *A manager posts an unfilled published shift to eligible staff at the site; the first to claim it gets it, subject to the manager's approval.* Extent lines to cover: only published shifts with nobody on them; only staff eligible by role, declared availability, and approved time off; claims ordered, first on top, manager approves one and the others are told; enabled per site. Relations: `composes-with` FEAT-001 (the claim lands on the published week), FEAT-002, FEAT-003 (both gate eligibility). Work rows: `live` — post and claim, acceptance from US-002's independent test; `pending` — the agency route, carrying the founder's original hook and a note that no story or pilot evidence supports it yet.

### 2b. New `.mochiko/features/FEAT-007-shift-swaps.md`

Capability: *A staff member offers a published shift of theirs to eligible colleagues at the same site; a colleague takes it and the manager approves, which moves the shift to them.* Extent: same eligibility gate as FEAT-006; a declined swap leaves the shift with the original holder and the decline is visible to them; an approved swap reassigns the published shift, so the taker can clock in and the hours land on the taker's timesheet; enabled per site. Relations: `composes-with` FEAT-001, FEAT-002, FEAT-003; **`depends-on` FEAT-005 and FEAT-004** — the reassignment is what makes the hours payable. Work rows: `live` — offer/take/approve, acceptance from US-001's test; `pending` — the weekly cover summary (US-005), acceptance "after three swaps and one claim the week view shows 3 and 1."

### 2c. New `.mochiko/features/FEAT-008-push-notifications.md`

Status `proposed`. Capability: *The app pushes a notification to a staff member's phone when something needs them.* Extent: named site, day, and time in the body; delivered within a minute; only to eligible recipients; per-person opt-out. Relations: `composes-with` FEAT-006 and FEAT-007. Work rows: `live` — cover alerts (US-004); `pending` — notify on publish, which FEAT-001 explicitly excludes today and which is the obvious next customer for this capability.

### 2d. Amend `.mochiko/features/FEAT-001-rota-building.md`

This is a delivered entry and the amendment is deliberate. The line *"Not: a manager changing who holds a published shift other than by re-opening the week"* stops being true the moment either FEAT-006 or FEAT-007 ships. I would replace it with a line saying the holder of a published shift changes only through an approved swap or claim, and add `composes-with` relations to FEAT-006 and FEAT-007. The status stays `delivered`; the map's live rows sit on the new entries.

### 2e. Amend `.mochiko/features/FEAT-005-timesheets.md`

Add an extent line: hours for a shift that changed hands through an approved swap or claim count for the person who worked it. Add `composes-with` FEAT-007. This is the map-level record of the FR-003 collision.

### 2f. `FEATURES.md`

Add three rows in the map's format — FEAT-006 restated and no longer `(unrefined)`, FEAT-007 and FEAT-008 as `proposed`, each with its indented `live`/`pending` work rows and hooks. FEAT-001 and FEAT-005 rows keep their status; FEAT-005's hook is untouched.

### 2g. `BACKLOG.md`

Add under Open: `- [ ] Published week prints blank when the rota PDF is printed from Safari; Chrome is fine (support, 2026-07-15, two customers)`. I would also flag, without acting on it, that FEAT-001's extent does not mention a printable/PDF view at all — either the map understates what shipped or the PDF lives somewhere unrecorded. Worth one question to engineering; not mine to guess at.

### 2h. `.mochiko/specs/index.md`

Update the shift-cover row: status `specify` → whatever the convention's next state is once selection is done (I would match how `timesheets-export` was phrased at the same point rather than invent a word), and fill "Capabilities touched" with FEAT-006, FEAT-007, FEAT-008, FEAT-001, FEAT-005.

### 2i. `.mochiko/specs/shift-cover/spec.md`

Update the header (status, and the `Next:` line, which describes exactly this task and is now done), and append a selection section: each story to its destination with the reason, the three re-priorities called out, and US-006/US-007 marked as not capability work. I would not rewrite the analyst's story text.

### 2j. New `.mochiko/specs/shift-cover/release-advice.md`

The founder-facing note from Phase 3.

---

## Phase 3 — The advice for Northgate

**Recommendation: build the manager's open-shift flow, its eligibility gate, push, and the per-site switch. Add staff swaps only if the shift-reassignment mechanism lands cleanly. Cut the summary.**

**Ship (the pilot minimum):**
1. **US-002 open shifts** — two-thirds of Northgate's 20–30 messages per site per week are a manager asking "can anyone do Saturday 2–8?" With three sites that is roughly 45 messages a week, ~30 of them the manager chasing. Managers told us the single thing that would change their week is not having to chase people. This is the largest measurable win in the note.
2. **US-003 eligibility** — not optional. Without it a manager approves someone on approved leave or in the wrong role, and the feature creates work instead of removing it.
3. **US-004 push** — promoted from P2. Twice-a-week app opens, concentrated on publish day, means an unnotified post is not seen before the shift happens. Items 1 and 2 do not function at this customer without it.
4. **US-008 per-site switch** — the founder wants one café at a time across three sites, and a pilot that can't be turned off site by site can only be turned off for everyone. If the window tightens, this can be a config flag rather than an admin screen.

**Ship if the reassignment holds:**
5. **US-001 swaps + US-009 timesheet attribution, as one unit.** Swaps are one-third of the traffic but produce *all* of the disputes that reach the owner — "I thought Sam was covering me." A recorded offer/take/approve chain is what kills those, and no amount of open-shift tooling touches them. They cost more than they look: the eligibility gate is shared with item 1, but the shift must genuinely change hands or the taker cannot clock in and is not paid. **US-009 does not slip separately from US-001** — shipping swaps without it means Northgate's staff work shifts that never reach the accountant's CSV.

**Cut:** US-005, the weekly summary (P3, spotting a chronically short site, no value inside a six-week pilot with three cafés a manager already knows). **Route elsewhere:** US-006 to the backlog, US-007 answered directly — publishing is live today, FEAT-001, since 2026-03-14; Northgate needs a reply, not a build.

**The tradeoff I would put to the founder plainly:** open shifts fix the *volume*; swaps fix the *disputes*. Shipping only open shifts means the owner keeps getting "I thought Sam was covering me" escalations, and the staff-side half of the WhatsApp habit survives, which weakens the whole pilot's story. My recommendation is to sequence rather than choose — open shifts + eligibility + push first as the demonstrable win, swaps immediately behind — and if only one can land, land open shifts, and tell Northgate's owner explicitly that staff-to-staff cover is next and why.

**Coordination I would flag as urgent, not a footnote:** `timesheets-export` is in-flight, cycle 2 of 3, and owns FEAT-005's live row. Its FR-003 assumes hours attach to the scheduled shift and that anything else is `unscheduled` and excluded. Shift cover changes what "scheduled" means. That conversation should happen this week, with that build's engineer, before either side hardens — not after cycle 3 closes.

---

## Phase 4 — Checks before I hand it over

There is no code and no test suite here; this is a docs workspace, so there is nothing to run and I would say so rather than imply a green build. What I would verify by re-reading:

- All nine stories appear exactly once in the selection table, none dropped, none in two places.
- Every new `FEATURES.md` row links to a file I actually created, and every new entry file has a row on the map.
- Statuses drawn only from `proposed` / `in-flight` / `delivered` / `retired`; new entries are `proposed`, not `in-flight` — nothing is being built yet.
- The FEAT-001 "Not:" line and FEAT-005's extent no longer contradict FEAT-006/FEAT-007.
- FEAT-006's story trace and the specs index agree with each other, and FEAT-001's existing time-and-attendance US-007 trace is untouched.
- Each `live` and `pending` row carries an acceptance line taken from the story's own independent test, not one I invented.

**No delegation.** I would not spawn any subagent or worker for this — it is one pass over thirteen short files, and the judgment calls (re-prioritising US-004 and US-009, redefining a founder-minted entry, amending a delivered capability) are the substance of the task, not overhead to farm out. The two hand-offs are to people: the engineers on timesheets-export, and the requirements analyst on US-006/US-007.

**What I would report at the end:** the map delta as a diff for approval; the three stories whose priority I moved and why; the timesheets collision, stated as a blocking dependency between swaps and payroll rather than a nice-to-have; the FEAT-006 agency-vs-internal ruling still open with my recommendation and the default I proceeded under; the passed 2026-08-24 date with a request for the real target; and the one-line answer for Northgate on US-007.