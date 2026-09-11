FILES-READ: `.mochiko/specs/time-and-attendance/spec.md`, `FEATURES.md`, `README.md`, `BACKLOG.md`, `.mochiko/specs/index.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`

---

# Plan — feature-map delta for the time-and-attendance spec

## Phase 0 — Establish the rule the map is built by (already done from reading)

The three existing entries settle what a map row is here, and it is not what the engineering lead proposed. FEAT-001/002/003 are each a user-visible capability with an actor, a hook sentence, an `Extent` that says what is deliberately excluded, `Relations` to other capabilities, and a `Story trace` back to a spec's story IDs. Sprint boundaries appear nowhere. README also states plainly that the backlog holds defects/tooling/process and the map holds capabilities.

So I would reject both options the engineering lead offered, and say so to the founder in one line each rather than quietly ignoring them:

- The "Sprint 14 / Sprint 15 / Nice-to-haves" carve is a delivery schedule. It would put a database migration and a rota timezone bug on the capability map, and it would name rows after sprints that stop meaning anything the moment the plan slips. Sequencing belongs in the sprint plan; it can still be honoured without the map recording it.
- The single "Time & attendance" row is too coarse: it would fold an SMS reminder and a deferred Xero integration into one capability, and give the row an `Extent` nobody can write honestly.

This is a judgment I would make and state, not a question I would hold the work for.

## Phase 1 — Classify every story before writing anything

I would work through all eleven story IDs and assign each to exactly one destination — a capability, an amendment to an existing capability, the backlog, or an explicit exclusion. This is the actual derivation; the file edits are bookkeeping afterwards.

| Story | Destination | Reasoning |
|---|---|---|
| US-001 clock-in/out | new capability A | staff-side attendance capture |
| US-002 breaks | new capability A | same actor, same record, changes worked hours |
| US-003 manager fixes a time | new capability B (flagged — see Phase 3) | manager amends the record before approval |
| US-004 CSV export | new capability B | output of the weekly hours |
| US-005 SMS shift reminder | new capability C | not attendance at all; a notification to staff about the *rota* |
| US-006 Manchester UTC bug | `BACKLOG.md` | a defect in delivered FEAT-001, not a capability |
| US-007 copy last week | amendment to FEAT-001 | FEAT-001's `Extent` currently says "Not: copying a previous week into a draft" — this story overturns that line |
| US-008 shifts → Postgres | `BACKLOG.md` (merge into the existing sidecar item) | infrastructure; invisible to any user |
| US-009 overtime flag | new capability B | it is a state on the week's hours view, pre-export |
| US-010 sign-off before export | new capability B | it is the gate that makes the export trustworthy |
| US-011 Xero | new capability D, status `proposed`, out of this release | the ops lead asked for it to be recorded; `proposed` is the map's mechanism for exactly that |

Two classifications I would call out to the founder because they change what engineering thinks it is building:

- **US-006 and US-008 are not features.** Together they are two of the five items the engineering lead put on the map. US-008 is already half-recorded in `BACKLOG.md` as the SQLite sidecar item dated 2026-05-30; I would extend that item rather than open a second one, so the two do not drift apart.
- **US-007 does not belong to this spec's capability at all.** It is a rota-building change that happens to be sitting in the time-and-attendance spec. It gets traced from this spec but lands on FEAT-001.

## Phase 2 — The proposed delta

Four new rows, all `proposed` (nothing is built), plus one amendment to a delivered row.

**FEAT-004 — Shift clock-in** · `.mochiko/features/FEAT-004-shift-clock-in.md`
Hook: *a staff member taps in at the start of a shift and out at the end, with breaks, producing a recorded worked time against the shift.*
Extent to write: button appears only within 30 minutes of a scheduled shift (US-001); break start/end while clocked in, deducted from worked time (US-002); a record is always tied to a scheduled shift. Not: clocking in with no scheduled shift. Not: geofencing or photo verification. Not: correcting a recorded time — that is FEAT-005.
Relations: composes-with FEAT-001 (a clock-in attaches to a published shift). Depends on the `shifts` table move (backlog item), which I would note as a delivery dependency inside the file, not as a map row.
Story trace: `time-and-attendance: US-001, US-002`.

**FEAT-005 — Weekly hours review and export** · `.mochiko/features/FEAT-005-weekly-hours-export.md`
Hook: *a manager reviews the week's worked hours per person, corrects and approves them, and exports the approved week for the accountant.*
Extent: per-person weekly totals; a manager can amend a clock-in or clock-out with the original value still shown (US-003); hours over contracted hours are flagged on the view (US-009); per-person approval is required before export (US-010); CSV with name, site, role, hours, overtime hours (US-004). Not: pushing to payroll — see FEAT-007. Not: holiday or entitlement accrual (consistent with FEAT-003's existing "Not"). Not: pay rates or cost — the CSV carries hours only.
Open question to record in the file: US-009 says "contracted hours" and the product has no such field today — FEAT-001 tracks role and times only. I would flag this as a gap for the analyst rather than invent the data model.
Story trace: `time-and-attendance: US-003, US-004, US-009, US-010`.

**FEAT-006 — Shift reminders** · `.mochiko/features/FEAT-006-shift-reminders.md`
Hook: *staff get an SMS an hour before a shift starts.*
Extent: SMS one hour before shift start, naming time and site; delivery via a third-party gateway (the spec names Twilio — I would keep the vendor out of the capability text and note it as an implementation choice). Not: push notification. Not: notification on publish — that remains excluded by FEAT-001. Not: a reply channel.
Relations: composes-with FEAT-001 (reminders are driven by published shifts). I would note that this is the first thing in the product that sends anything outbound to staff, which carries a cost and a consent question worth surfacing to the founder.
Story trace: `time-and-attendance: US-005`.

**FEAT-007 — Payroll export to Xero** · `.mochiko/features/FEAT-007-payroll-export-xero.md`
Status `proposed`, with the file stating explicitly it is out of this release, recorded so it is not lost. Extent kept deliberately thin.
Story trace: `time-and-attendance: US-011`.

**Amendment — FEAT-001 Rota building** (status stays `delivered`)
Remove the `Extent` line "Not: copying a previous week into a draft — every week starts empty" and replace it with a positive line: a manager can populate an empty draft by copying the previous week's shifts. Add `time-and-attendance: US-007` to its story trace alongside the existing `rota-basics` line. I would not touch its `since` date or status — the capability is delivered, its extent grew.

**`FEATURES.md`** — append four rows in ID order with the hooks above, all `proposed`.

**`BACKLOG.md`** — add the Manchester timezone defect under Open (attributed to the spec, US-006, since it is a live customer-facing bug in a delivered capability); extend the existing SQLite sidecar item with a note that the clock-in work depends on completing the move (US-008), rather than adding a duplicate line.

**`.mochiko/specs/index.md`** — update the `time-and-attendance` row: capabilities touched becomes `FEAT-001, FEAT-004, FEAT-005, FEAT-006, FEAT-007`; status becomes derivation-complete-pending-story-review. `Closed` stays empty.

**Delta summary for the founder** · `.mochiko/specs/time-and-attendance/feature-delta.md` — one page: the four new rows, the FEAT-001 amendment, the two stories sent to the backlog, why the sprint carve was not used, and the open questions. This is the artefact the founder actually asked for; the file edits are what makes it real.

## Phase 3 — The stops

**Stop 1 — the date discrepancy. I would raise this before writing anything.** The spec is dated 2026-06-08 and books the analyst's story review for 2026-06-15. Today is 2026-09-09, and the brief describes that review as happening *next week*. The review date in the spec is three months past. Either the spec is stale and stories have moved since, or the review slipped and the document was never updated. What I would confirm: is `spec.md` as I read it the current text, and is the review now around 2026-09-16?

- If the spec is current and the review merely moved: proceed exactly as planned.
- If there is a newer version of the stories: re-run Phase 1 against it before writing; the classification table is the only thing that would change.
- **Default if I get no answer:** proceed on the text as written, and put a line at the top of the delta noting which version of the spec it was derived from and the date gap, so the analyst can spot a mismatch immediately.

**Stop 2 — where US-003 (manager fixes a wrong time) sits.** It is genuinely arguable: it amends the attendance record, so it could sit with FEAT-004; but the manager does it in the weekly hours view, before sign-off, and it is meaningless without the approval flow. I would place it in FEAT-005 and cross-reference it from FEAT-004's `Extent`, and put the choice in front of the founder as a one-line note rather than a blocking question. If they rule the other way, the fix is moving one story ID and swapping two `Extent` lines.

**Stop 3 — whether the map should carry a `proposed` row for work nobody will start (FEAT-007/Xero).** The map's status vocabulary includes `proposed`, so I would use it. If the founder prefers the map to hold only work that is actually queued, FEAT-007 collapses into a single "Not:" line in FEAT-005 and a note in the delta.

**What I would not do:** I would not mark anything `in-flight` or `delivered`, would not touch FEAT-002 or FEAT-003, and would not write acceptance criteria or estimates into the feature files — those are the analyst's and engineering's, and the map deliberately does not carry them.

## Phase 4 — Consistency checks before handing over

There is no test suite here; this is a document set, so the checks are re-reads I would perform against what I wrote:

1. Every one of US-001…US-011 appears exactly once as a destination — in a `Story trace`, in `BACKLOG.md`, or (none, on this plan) as an explicit exclusion. Expect: eleven accounted for, nine on the map across five features, two on the backlog.
2. Every `FEAT-00X` id is unique and every link in `FEATURES.md` points at a file that exists. Expect: seven rows, seven files.
3. Every status used is one of `proposed` / `in-flight` / `delivered` / `retired`.
4. No `Extent` line in a new feature contradicts an `Extent` line in FEAT-001/002/003 — specifically that FEAT-001's publish-notification exclusion still stands after FEAT-006 is added, and that the FEAT-001 copy-week line is flipped in exactly one place.
5. New feature files follow the shape of the existing three: title, status block, `Capability`, `Extent`, `Relations`, `Story trace`. Note the existing files carry a `sticky — live rows may still be visible below` marker under a delivered status; I would not copy that onto `proposed` rows.
6. The spec index row's capability list matches the features that actually trace to this spec.

## Phase 5 — Delegation

None. Eight files, all read in full, all short; the work is a classification judgment rather than volume. Spawning a worker would add a review burden larger than the task. I am also not running any cloud review here — that is user-triggered and billed.

## Phase 6 — What I would report

A short note to the founder: the map gains four proposed capabilities and one amendment to a delivered one; two of the eleven stories are not capabilities and went to the backlog; one story (copy last week) belongs to rota building, not to attendance. Plus the three things needing a human: the June/September date discrepancy on the spec, the missing "contracted hours" data the overtime flag assumes, and the placement of the manager time-correction story. And an explicit line that the analyst's story review has not happened, so every new row is `proposed` and the derivation should be re-checked against whatever the review changes.