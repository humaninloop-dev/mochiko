FILES-READ: `.mochiko/specs/time-and-attendance/spec.md`, `FEATURES.md`, `README.md`, `BACKLOG.md`, `.mochiko/specs/index.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`

---

# Plan — feature-map delta for the time-and-attendance spec

## Phase 0 — Framing, and two things I'd flag up front

**Read (done):** all eight files above. Nothing further to read; this workspace is entirely docs.

Two things I'd state to the founder before showing the delta, then keep working:

1. **The dates don't line up.** The spec is authored 2026-06-08 with a story review booked 2026-06-15, and the README quotes customer counts as of June 2026. Today is 2026-09-09, so "next week" is three months after the booked review. Either the spec sat, or the review moved. This doesn't change the derivation, so I'd derive against the spec as written and ask in the report whether anything happened to these stories in the intervening quarter that isn't in the file.
2. **I would not use either carve the engineering lead proposed.** The sprint carve ("Sprint 14", "Sprint 15", "Nice-to-haves") groups by delivery batch, not by what a user can do — a map built that way goes stale the moment the sprint plan moves, and "Nice-to-haves" is a bucket, not a capability. The single "Time & attendance" entry is the opposite failure: it hides that capturing attendance and approving/exporting hours are separately shippable capabilities with different users, and it swallows three stories that aren't this spec's capability at all. I'd say this plainly and briefly, credit the intent (saving the product seat time), and show the capability carve instead.

**No delegation.** This is a small, judgment-heavy derivation over eight short files; splitting it across workers would cost more in briefing and re-checking than doing it. No subagents, no workers.

**No test suite exists here.** Verification in Phase 5 is consistency checking against the map's own conventions, not test runs.

## Phase 1 — Sort every story into capability / not-a-capability

The rule I'd apply, from `README.md` and `FEATURES.md`: an entry on the map is a *capability* — something the product lets someone do. Defects and infrastructure are not; `BACKLOG.md` says so explicitly.

I'd produce this classification as working notes (not a file yet):

| Story | Verdict |
|---|---|
| US-001 clock-in/out button | capability — new |
| US-002 break start/end | capability — new, same one |
| US-003 manager fixes a wrong time | capability — new, boundary call (see Phase 2) |
| US-004 CSV export | capability — new |
| US-009 overtime flag | capability — new, same one as US-004 |
| US-010 sign-off before export | capability — new, same one as US-004 |
| US-005 SMS reminder before shift | capability — but **not this spec's capability** (see Phase 3) |
| US-007 copy last week | **not new** — amends delivered FEAT-001 |
| US-006 rota shows UTC in Manchester | **defect** → `BACKLOG.md`, no map entry |
| US-008 shifts table to Postgres | **infrastructure** → `BACKLOG.md`, no map entry |
| US-011 Xero payroll push | explicitly deferred → boundary line, not an entry |

Three of the eleven stories produce no new map row. That is the headline of the delta and I'd lead the report with it.

## Phase 2 — Draft the two new capability entries

**Write `.mochiko/features/FEAT-004-attendance-capture.md`** — status `proposed`, following the exact section shape of FEAT-001/002/003 (`## Capability`, `## Extent` with `Not:` lines, `## Relations`, `## Story trace`).

- Capability: a staff member records the start and end of the shift they actually worked, and their breaks, from the staff app; a manager can correct a recorded time and the original stays visible.
- Extent lines: clock-in offered only within 30 minutes of a scheduled shift; break time comes off worked hours; manager edits preserve the original.
- `Not:` lines I'd write from what the stories are silent on: not location or geofence verification of the clock-in; not clocking in against no scheduled shift; not staff editing their own recorded times.
- Relations: `composes-with: FEAT-001` — the 30-minute window and the join both depend on the published rota.

**Write `.mochiko/features/FEAT-005-hours-approval-and-export.md`** — status `proposed`.

- Capability: a manager reviews the week's worked hours per person, sees when someone is over their contracted hours, approves each person, and exports the approved week as a CSV for the accountant.
- Extent: sign-off gates the export (US-010); columns name, site, role, hours, overtime hours.
- `Not:` lines: not a payroll push — sending approved hours to Xero is named in the spec as a later release (US-011); not holiday entitlement (already FEAT-003's boundary).
- Relations: `composes-with: FEAT-004` — hours derive from captured attendance.

**The one boundary call I'd stop and name:** US-003 (manager fixes a wrong time) could sit on FEAT-004 with the record it corrects, or on FEAT-005 with the manager's other week-end work. I'd default to **FEAT-004**, because the audit trail ("keep the original visible") is a property of the attendance record itself and has to hold whether or not anyone ever exports. If the founder rules the other way, it moves one line of Extent and one story ID from FEAT-004's trace to FEAT-005's — no other change.

## Phase 3 — The SMS reminder, and the FEAT-001 amendment

**US-005 (SMS reminder).** This is a real capability, but it is not time and attendance — it's a notification sent before a shift, and it directly reverses a boundary the map already holds: FEAT-001 says "Not: any notification to staff on publish — staff open the app to see the week." Adding outbound messaging is a product decision with cost and consent consequences, not a rider on a clock-in spec. I'd also strip "set up Twilio" — the map records capability, not vendor.

**Stop and confirm with the founder.** Default if no answer: write it as **`.mochiko/features/FEAT-006-shift-reminders.md`**, status `proposed`, with a note that it arrived via this spec but stands outside its capability line and wants its own spec before build. Branches: if the founder says it belongs in this release, I keep the file and add the row to this delta unqualified; if they say defer it, I drop the file and instead record the story in the spec's own notes so it isn't lost, leaving the delta at two new rows plus one amendment.

**US-007 (copy last week)** amends FEAT-001. I'd edit `.mochiko/features/FEAT-001-rota-building.md`: remove the `Not: copying a previous week into a draft — every week starts empty` line, add an Extent line for copying the previous week into an empty draft, and add `- time-and-attendance: US-007` to its story trace. The file's status stays `delivered` because the capability is delivered; the new extent line I'd mark inline as proposed and sourced to US-007, since the map has no per-line status convention. **This is the second thing I'd surface for a ruling** — if the founder prefers amendments not to touch a delivered entry until built, the alternative is to hold US-007 in the spec and amend FEAT-001 on delivery. I'd note that the first way makes the map honest about intent and the second keeps it honest about reality, and recommend the first with the inline marker.

## Phase 4 — Edit the index files and the backlog

- **`FEATURES.md`** — append rows for FEAT-004, FEAT-005, and (default) FEAT-006, status `proposed`, each with a one-line hook in the same voice as the existing three ("a staff member taps in at the start of a shift and out at the end; breaks come off the hours").
- **`.mochiko/specs/index.md`** — update the `time-and-attendance` row: capabilities touched becomes `FEAT-001, FEAT-004, FEAT-005, FEAT-006`; status from `specify — derivation pending` to `specify — features derived`. Leave `Closed` empty.
- **`BACKLOG.md`** — add two Open items: the Manchester UTC defect (US-006), and the `shifts`-to-Postgres migration (US-008), the latter cross-referenced to the existing 2026-05-30 sidecar item, since they are the same underlying problem and shouldn't be worked twice.
- **The spec file** — I would *not* rewrite the ops lead's stories or delete the engineering lead's proposal. I'd append a short derivation note under the proposal recording what the map took, what it routed to the backlog, and why the sprint carve wasn't used, so the disagreement is visible at the review rather than discovered there.

## Phase 5 — Check the delta before reporting

No tests to run; these checks stand in for them, and I'd do each by re-reading the files I wrote:

- Every one of US-001…US-011 appears exactly once across the story traces, the backlog, or the explicit deferral — none dropped, none double-counted.
- Every ID referenced resolves: FEAT-004/005/006 files exist at the paths `FEATURES.md` links; relations name features that exist.
- New files match the section headings and `Not:`-line style of FEAT-001/002/003, and the `FEATURES.md` table still parses with three columns of content plus ID.
- No vendor or sprint names leaked into capability text (no "Twilio", no "Sprint 14", no "SQLite").

Expected result: eleven stories accounted for, three new rows, one amended entry, two backlog items, zero broken links.

## Phase 6 — Report, plus the questions the analyst review should settle

I'd report: the delta (2 new capabilities, 1 flagged as out-of-line, 1 amendment to FEAT-001, 2 backlog items), the files changed, why the sprint carve was set aside, and the two rulings I'm holding (US-003's home, and how to mark an unbuilt extent on a delivered feature) with my defaults already applied.

Then a short list of gaps for the 2026-06-15 story review — these are the analyst's to settle, not mine to invent answers for, but the founder should see them now because two affect scope:

- **Contracted hours don't exist in the product.** US-009 flags "over their contracted hours", but nothing in FEAT-001/002/003 stores a contract. Someone has to add that, and it isn't in any story.
- **US-004 is P1 and needs an overtime column; US-009, which computes overtime, is P2.** As prioritised, the P1 export ships with a column it can't fill.
- The 30-minute window means work outside a scheduled shift can't be captured at all — deliberate, or a gap?
- What happens when someone never clocks out; does the shift auto-close, or wait for a manager edit?
- Are breaks paid or unpaid, and does that vary by site?
- US-006's timezone bug and the clock-in timestamps are the same underlying problem; the fix should land before attendance capture, not alongside it.
- Export scope for the two three-site groups — per site, or per group in one file for the accountant?