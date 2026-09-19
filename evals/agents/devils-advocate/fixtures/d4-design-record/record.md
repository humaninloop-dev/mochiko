# Design session record — Offline mode for the technician app

- **Session:** offline-mode
- **Status:** concluded, awaiting review
- **Date:** 2026-09-08
- **In the room:** product lead · mobile lead · backend lead · support lead
- **Purpose:** decide how the technician app behaves without connectivity, so that a job can be
  started, worked, and closed in the field and reach the office when the device reconnects.

## Context

Technicians on Fieldline visit customer sites — basements, plant rooms, rural properties —
where the app has no signal for minutes to hours. Today the job card, the parts list, and the
sign-off form all need a live connection, so technicians take photos and notes on paper and
re-enter them at the van. Support logs about 40 lost or duplicated job closures a week traced to
this. Some regional contracts run multi-day jobs at sites with no coverage at all.

## Decisions

### D1 — Offline is the default working mode, not an error state

The app treats no-connectivity as normal. Every screen a technician uses on site works from
local data; sync happens when it can.

Alternatives: (a) read-only offline with a banner — rejected, sign-off is the step most often
done offline. (b) offline only for the sign-off form — rejected, partial coverage confuses more
than it helps.

### D2 — The local store is SQLite through the platform's standard binding

Alternatives: (a) an embedded document store — rejected, adds a dependency for no query the app
needs. (b) in-memory with file dumps — rejected, loses work on a crash.

### D3 — The device holds the technician's assigned jobs for the next 7 days

Jobs are pulled on sign-in and refreshed whenever the app is online. Jobs outside the window
are not on the device.

Alternatives: 1 day — rejected, multi-day jobs would fall off mid-job. 30 days — rejected, most
of it is never used and the store grows.

### D4 — No customer personal data is stored on the device

Offline jobs reference the customer by ID only. Name, phone, and address are fetched live and
never persisted, so a lost or stolen device exposes nothing that identifies a customer. This is
what lets us keep the device out of the data-protection assessment.

Alternatives: (a) encrypt at rest and store everything — rejected, the assessment would still
be needed and the security review queue is six weeks. (b) store the name only — rejected as a
half-measure.

### D5 — Writes are queued locally and replayed in order on reconnect

Every change on the device appends to a local queue; on reconnect the queue replays to the
server oldest first. A failed replay stops the queue and surfaces to the technician.

Alternatives: (a) send each change independently — rejected, out-of-order arrival breaks job
state transitions.

### D6 — The queue is sized for one shift

Technicians are back in coverage by the end of every shift, since vans return to the depot, so
the queue never holds more than one shift of work and needs no compaction or overflow handling.

Alternatives: (a) unbounded queue with compaction — rejected, work for a case that does not
occur.

### D7 — Conflicts resolve by server-side rule, never on the device

When the office edits a job the technician also changed offline, the server applies field-level
rules: office wins on schedule and price; technician wins on status, notes, photos, and
sign-off. The device never shows a merge screen.

Alternatives: (a) last writer wins — rejected, an office reschedule would silently overwrite a
completed sign-off. (b) the technician resolves conflicts on the device — rejected, the support
lead vetoed adding decisions to the technician's day.

### D8 — Photos are stored on the device at reduced resolution and uploaded on reconnect over any connection

Alternatives: (a) full resolution over Wi-Fi only — rejected, technicians rarely have Wi-Fi
before the next job.

### D9 — Sign-off captures the customer's signature on the device

The sign-off screen shows the job summary with the customer's name and site address for
confirmation, captures the signature, and marks the job closed locally. The closure syncs like
any other write.

Alternatives: (a) sign-off requires connectivity — rejected, see D1.

### D10 — The offline job card shows the customer's name, phone, and site address

Technicians call ahead when running late and navigate to the site from the card; both happen
while driving between jobs, often without signal.

Alternatives: (a) show the address only — rejected, calling ahead is the most-requested use.

### D11 — A job stays editable on the device for 24 hours after closure

Technicians add a forgotten photo or note the same evening. After 24 hours the job is read-only
on the device.

Alternatives: (a) read-only at closure — rejected, support sees a steady stream of "forgot to
attach the photo" tickets.

### D12 — Sync status is one indicator on the job list

One icon: synced, pending, or blocked. Tapping it lists the pending items. No per-field
indicators.

Alternatives: (a) per-field markers — rejected as clutter.

## Open questions

- Whether a job that falls out of the 7-day window (D3) while it holds unsynced changes is kept
  until it syncs — the mobile lead believes the queue guarantees this, but it was not walked
  through.

## Next steps

- Mobile lead writes the sync design against D5–D7.
- Backend lead specifies the D7 conflict rules as a table.
