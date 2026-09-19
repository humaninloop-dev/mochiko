FILES-READ: `.mochiko/specs/time-and-attendance/spec.md`, `FEATURES.md`, `README.md`, `BACKLOG.md`, `.mochiko/specs/index.md`, `.mochiko/features/FEAT-001-rota-building.md`, `.mochiko/features/FEAT-002-staff-availability.md`, `.mochiko/features/FEAT-003-time-off-requests.md`

---

# Action plan — feature-map delta for `time-and-attendance`

## Phase 1 — Establish the axis the map is carved on

**What I'd do:** Settle the shape question before writing anything, because the spec offers two carvings and I'd take neither.

**Read (done):** `FEATURES.md` header and the three existing entries; `README.md` working conventions.

**Finding that drives everything else:** the map is "the system as capabilities," each row a durable thing the product can do with a user-facing hook (`FEATURES.md:4`, and the three delivered rows). Measured against that:

- The engineering lead's **Sprint 14 / Sprint 15 / Nice-to-haves** carving groups by delivery schedule. Sprint names go stale the moment the plan slips, "Nice-to-haves" is a priority bucket rather than a capability, and it puts a database migration and a timezone defect on a capability map. I'd decline this carving and say so plainly to the founder — with the note that the *sequencing* in it is useful and I'd preserve it as ordering on the proposed rows, not as row identity.
- The **single "Time & attendance" row** is the opposite failure: it hides that clocking (staff-side), timesheet correction and sign-off (manager-side), and payroll export (accountant-side) have different audiences, can ship independently, and would be flagged `delivered` at different dates. FEAT-001/002/003 are each finer-grained than that.

**Default I'd proceed under:** derive rows from the stories, on the capability axis, at roughly the grain of FEAT-001..003.

## Phase 2 — Classify every story, one disposition each

**What I'd do:** Walk US-001..US-011 and assign each exactly one destination. Not everything in a spec is a capability — `README.md:17` is explicit that a story is not a capability until the map says so, and `BACKLOG.md:3` reserves defects/tooling/process for the backlog.

| Story | Disposition | Why |
|---|---|---|
| US-001 clock in/out | FEAT-004 Shift clocking | the core capture capability |
| US-002 breaks | FEAT-004 | same capability, same surface, same record |
| US-003 manager fixes a time | FEAT-005 Timesheet review | manager-side correction with the original retained |
| US-004 CSV export | FEAT-006 Payroll export | different audience (the accountant), leaves the product |
| US-005 SMS shift reminder | FEAT-007 Shift reminders (`proposed`) — **flagged** | a real capability, but about the rota, not attendance; FEAT-001 currently says "Not: any notification to staff" (`FEAT-001:17`), so this is the first notification the product would send. Twilio is implementation detail and stays off the map. |
| US-006 UTC times, Manchester | `BACKLOG.md`, defect against FEAT-001 | a bug in delivered behaviour, not a new capability |
| US-007 copy last week | **Amends FEAT-001**, not a new row | FEAT-001:16 explicitly excludes it; this flips a stated boundary of an existing capability |
| US-008 shifts → Postgres | `BACKLOG.md`, folded into the existing sidecar item | enabling work; `BACKLOG.md:7` already tracks exactly this SQLite/Postgres split, so I'd extend that line rather than open a second one |
| US-009 overtime flag | FEAT-005 | it is a signal on the week's hours view the manager reviews |
| US-010 sign-off before export | FEAT-005, gating FEAT-006 | the approval act is manager review; the gate is stated on both rows |
| US-011 Xero | No row | out of this release by the author's own words; recorded as a boundary line on FEAT-006 so it is not forgotten without pretending it is planned |

**Coverage check I'd run before writing:** all eleven stories appear exactly once; nothing in two features; four map rows, one amendment, two backlog lines, one boundary line.

## Phase 3 — Gaps I'd flag rather than paper over

These go to the founder now and into the spec's open-questions list for the analyst's review; none of them block the delta.

1. **US-009 needs data the product does not have.** Overtime is defined against "contracted hours," and nothing in FEAT-001..003 stores contracted hours per person — FEAT-003:15 is explicit that the product tracks no entitlement or balance. US-004's `overtime hours` CSV column inherits the same gap. I'd write FEAT-005 and FEAT-006 with the overtime element marked as depending on a contracted-hours field that must be specified.
2. **US-010 has an undefined edge:** what happens when a manager edits a time (US-003) *after* signing off the week. Re-open, re-approve, or block? I'd state a default on the row — sign-off is voided by a later edit — and mark it as the analyst's call.
3. **US-001 leaves two things open:** no clock-in path for someone who works an unscheduled shift (the button only appears near a scheduled one), and no stated behaviour when someone never taps clock-out. I'd write both as explicit `Not:` / open lines rather than invent them.
4. **The spec is three months old.** It is dated 2026-06-08 and books the analyst review for 2026-06-15; today is 2026-09-09 and the review is described as next week. The churn-risk framing ("two of our three-site groups") and the 41-site count are June figures. I'd ask the founder to confirm the spec still reflects current priority — and proceed on the assumption that it does, since the delta is wanted this week.
5. **Two convention gaps** with no precedent in the workspace, which I'd raise and default past rather than stall on:
   - *Amending a `delivered` feature (FEAT-001) with behaviour that is not built yet.* Default: leave the status `delivered`, replace the "Not: copying a previous week" line with an in-scope line marked pending, and add `time-and-attendance: US-007` to its story trace. Branch — if the founder says a delivered row must stay frozen, US-007 becomes its own small `proposed` row instead and FEAT-001's exclusion line stays until it ships.
   - *Relation vocabulary.* Existing files only use `composes-with`. FEAT-005→FEAT-004 and FEAT-006→FEAT-005 are genuine dependencies. Default: reuse `composes-with` and state the gating in the Extent prose, rather than invent `depends-on` unilaterally. Branch — if the founder is happy to extend the vocabulary, I'd switch those two lines to `depends-on`, which reads more truthfully.

## Phase 4 — Write the delta

Files I would write, following the exact shape of the existing entries (title, status line with `since`/`sticky`, `## Capability`, `## Extent` with `Not:` boundaries, `## Relations`, `## Story trace`):

- `.mochiko/features/FEAT-004-shift-clocking.md` — status `proposed`. Staff tap to clock in, out, and around breaks against a scheduled shift; the app records actual times and break deductions. Extent: button visible within 30 minutes of a scheduled shift; break time comes off worked hours. Not: clocking without a scheduled shift; not location or geofence checks; not automatic clock-out. Relations: composes-with FEAT-001. Trace: `time-and-attendance: US-001, US-002`.
- `.mochiko/features/FEAT-005-timesheet-review.md` — status `proposed`. A manager reviews the week's worked hours per person, corrects wrong times with the original preserved, sees an overtime flag, and signs each person off. Extent covers the edit trail and the sign-off state; the overtime flag is marked as pending a contracted-hours field. Not: an hours dispute flow for staff. Relations: composes-with FEAT-004. Trace: `US-003, US-009, US-010`.
- `.mochiko/features/FEAT-006-payroll-export.md` — status `proposed`. An approved week leaves the product as a CSV the accountant opens (name, site, role, hours, overtime hours). Extent: export blocked until sign-off. Not: pushing hours into Xero or any payroll system — parked, US-011. Relations: composes-with FEAT-005. Trace: `US-004`, boundary note `US-011`.
- `.mochiko/features/FEAT-007-shift-reminders.md` — status `proposed`, carrying the scope flag from Phase 3. Staff get a text an hour before a shift starts. Not: publish notifications; not confirm-or-decline replies. Relations: composes-with FEAT-001. Trace: `US-005`.

Files I would edit:

- `FEATURES.md` — four rows appended after FEAT-003, each `proposed`, each with a one-line hook in the voice of the existing three.
- `.mochiko/features/FEAT-001-rota-building.md` — the copy-last-week amendment and the added story-trace line (per the Phase 3 default).
- `.mochiko/specs/index.md` — the `time-and-attendance` row: capabilities touched becomes `FEAT-001, FEAT-004, FEAT-005, FEAT-006, FEAT-007`; status left at derivation-pending until the founder accepts the delta, then flipped.
- `BACKLOG.md` — extend the existing sidecar line to record that the clock-in work depends on the `shifts` move landing first (US-008); add a new open line for the Manchester timezone defect (US-006) noting it is a FEAT-001 defect.

Files I would **not** touch: `.mochiko/specs/time-and-attendance/spec.md`. It is the ops lead's draft with a review booked; my open questions go in the delta summary and to the founder, not as edits into someone else's document. I'd confirm before writing into it.

- `.mochiko/specs/time-and-attendance/feature-map-delta.md` — the founder-facing summary: the eleven-row disposition table above, the reasoning for rejecting both proposed carvings, the five flags, and the two decisions I want ruled on.

## Phase 5 — Consistency checks before I hand it over

No code, so the checks are on the artefacts:

- Every story US-001..US-011 resolves to exactly one destination, verified against the Phase 2 table — expect eleven, no duplicates, no orphans.
- Each new `FEAT-00X` ID is unused; each `FEATURES.md` link path matches a file that exists; each new file's `## Story trace` names `time-and-attendance` and only stories in that spec.
- Every status used is one of `proposed`/`in-flight`/`delivered`/`retired` (`FEATURES.md:5`) — all four new rows should read `proposed`, none `delivered`.
- No sprint name, ticket number, Twilio, SQLite, or Postgres appears in any feature file — those belong to the backlog and the plan, not the capability map.
- `.mochiko/specs/index.md` capabilities-touched list matches the set of files actually written.

**Delegation:** none. This is a small, judgement-dense reading task over eight short files; a subagent would add a summarisation layer over exactly the text I need to read verbatim, and the disputed calls are the whole deliverable.

## Phase 6 — What I'd report

A short note to the founder: four new `proposed` capabilities (shift clocking, timesheet review, payroll export, shift reminders), one amendment to rota building for copy-last-week, two backlog lines for the timezone defect and the Postgres move, and Xero recorded as an explicit boundary rather than a row. Stated plainly: I did not adopt either carving in the spec, and why. Then the two rulings I need — whether payroll export stays its own row or folds into timesheet review, and whether the SMS reminder belongs in this spec's delta at all given it is a rota capability — with my recommendation on each (keep export separate; keep the reminder as its own row but note the theme mismatch). Plus the five flags, with the contracted-hours gap called out as the one that will actually block US-009 and half of US-004's CSV if the analyst does not close it on the 16th.