FILES-READ: `record.md`

---

# Review plan — offline-mode design session record

The deliverable here is a written review returned to the room plus a question list, not code. Only `record.md` exists in this workspace, so the review is bounded by what the record itself says; I note below what I'd ask for that isn't here.

## Phase 1 — Build a decision inventory and cross-reference

**Do:** Re-read `record.md` end to end and tabulate all twelve decisions against three axes: what each one puts *on the device*, what each one assumes about *connectivity*, and what each one assumes about *who acts* (technician / office / server). Most contradictions in a record like this surface only when decisions written in separate parts of a session are put on the same row.

**Read:** `record.md` — the Context block (lines 12–16) treated as evidence, not preamble; it is where the session's own counter-examples live.

**Write:** nothing yet; working table held in the draft.

**Expect to show:** that the record contains at least one pair of decisions that cannot both be built.

## Phase 2 — Contradiction pass (the blocking findings)

These are the four I would lead the review with. Each is internal to the record — I'm not importing outside requirements.

**B1 — D4 and D10 (and D9) cannot both be built.**
D4 says no customer personal data on the device: name, phone, address "fetched live and never persisted." D10 says the offline job card shows name, phone and site address, and its stated reason is calling ahead and navigating *while driving between jobs, often without signal* — precisely when a live fetch is impossible. D9 puts the name and site address on the sign-off screen, which D1 insists must work offline. So D4's closing claim, "this is what lets us keep the device out of the data-protection assessment," rests on a behaviour the same document rules out three decisions later. Note also that D4's alternative (a) — encrypt at rest and store everything — was rejected *only* because the assessment would still be needed. If the assessment is needed anyway, that rejection has lost its reason and should be re-run.

**B2 — D6 is contradicted by the record's own Context.**
D6 sizes the queue for one shift because "technicians are back in coverage by the end of every shift, since vans return to the depot," and therefore drops compaction and overflow handling. Context line 16 says some regional contracts run multi-day jobs at sites with no coverage at all. Both can't hold. Worse, the record never says what the device *does* when the queue is exceeded — silently drop, refuse writes, or crash — which means the failure mode of the feature is undefined in exactly the scenario the feature exists for.

**B3 — D5, D7 and D12 leave the technician in a dead end.**
D5: a failed replay stops the queue and surfaces to the technician. D12: the indicator shows "blocked." D7: the device never shows a merge screen and conflicts resolve server-side. So "blocked" is a state with no defined action available to the person it's shown to. Two consequences to raise: a strictly ordered halting queue means one bad write freezes the sync of every later job, including other customers' closures; and since D7 has the server *resolve* rather than reject, the record never enumerates what actually causes a replay failure (validation, expired auth, job deleted or reassigned, server error) — so nobody can implement D5's error path.

**B4 — Offline sign-in and session lifetime are undecided, and can hard-lock the feature.**
D3 pulls jobs "on sign-in." Sign-in normally needs the network. Nothing in the record says how long a device can remain offline before the session expires, or what a technician does on day two of a multi-day no-coverage job if it does. This gap can lock a technician out of the local data the whole design exists to give them. I'd rate it blocking because it's cheap to decide now and expensive to discover in the field.

## Phase 3 — Second-order gaps (resolve before build, not necessarily before starting)

- **The record's own open question, sharpened.** D3 evicts jobs outside the 7-day window; the mobile lead believes the queue guarantees a dirty job is kept. It doesn't — the queue holds *writes*, eviction happens in the *job store*; they're different tables. The rule needs stating explicitly: never evict a job that has queued writes or is inside D11's 24-hour edit window, and never run eviction while offline (otherwise a long offline stretch can expire and wipe the store with no way to refetch).
- **D3 vs multi-day work:** a job that starts on day six of the window and runs three days. Is the window anchored to the refresh, or to job end date?
- **D11 vs D5 and D7:** a post-closure edit syncs after the closure has already synced. Is it an ordinary queued write, and does D7's "technician wins on notes and photos" extend to edits arriving after the office has seen the job closed? Also, 24 hours measured on which clock — device clock is user-settable.
- **D8 is unquantified:** no resolution, no per-job photo cap, no storage budget — and photos are by far the largest thing D6 assumed was small. "Over any connection" also means metered cellular data and battery on the technician's device; someone should confirm that cost is accepted. If photos are evidence in disputed jobs, someone should confirm reduced resolution is acceptable as a record.
- **D2 names SQLite but takes no position on encryption at rest, or on what happens to the store at sign-out** — relevant if vans or devices are shared between technicians, and unavoidable if B1 resolves toward D10.
- **D7's rule table needs to cover lifecycle, not just fields:** job cancelled by the office while the technician completes it offline, job reassigned to another technician while the first holds unsynced work, job deleted. "Technician wins on status" as written would resurrect a cancelled job.
- **D12 will lie under B3:** with an ordered halting queue, a job shows "pending" merely because an *earlier* job blocked. One icon is fine; it needs to distinguish "waiting on me" from "waiting on something else."
- **No success criteria anywhere.** The 40 lost or duplicated closures a week is the only number in the record; I'd propose it become the acceptance metric before build rather than after rollout.

**Not blocked, safe to start:** D1 and D2, and the local job store and read path generally. I'd say so explicitly so the review doesn't read as "stop everything."

## Phase 4 — Draft the review document

**Write:** `review-offline-mode.md` in this workspace, structured as: one-paragraph summary → Blocking (B1–B4, each stating which decisions collide and what breaks) → Resolve before build (Phase 3 list) → Clear to proceed (D1, D2) → Questions with named owners → What I'd need to go further.

**Would not do without asking:** edit `record.md` itself. It's the room's concluded artefact; changing its status line or annotating decisions in place is their call, so I'd offer it and leave the record untouched by default.

## Phase 5 — The questions I'd put back, with owners

1. **Product + support + whoever owns the data-protection assessment — which stands, D4 or D10?** This is the one that must be answered before any job-card or sign-off work starts.
2. **Backend + mobile — what does a technician actually do when the indicator reads "blocked"?** And should a blocked write halt that job's sync only, or the whole device's?
3. **Backend — what causes a replay to fail at all, given D7 has the server resolve conflicts by rule?** An enumerated list, since D5's error path can't be built without it.
4. **Mobile — what is "one shift" in megabytes and rows, and what happens on overflow?** Asked jointly with: how does that survive the multi-day no-coverage contracts in your own Context section?
5. **Mobile — how long can a device stay offline and still open its jobs?** (Sign-in, token lifetime, refresh.)
6. **Mobile + product — the eviction rule for jobs with unsynced work, written down** (the record's open question, which I'd argue is not answered by the queue).
7. **Product + legal/support — photo resolution, per-job cap, and whether reduced-resolution photos hold up as evidence in a disputed job.**
8. **Backend — does the D7 table cover cancellation, reassignment and deletion, not just field merges?**
9. **Mobile + security — is the SQLite store encrypted, and what happens to it at sign-out or device handover?**
10. **Support + product — what number replaces the 40 a week, and how is it measured?**

## Phase 6 — The stop, and the branches

**Stop:** I would not start job-card or sign-off implementation until the room rules on question 1. It's a product-and-privacy call with an external dependency (the six-week security review queue named in D4), not a call I should make for them.

- **If D10 holds** (technicians keep name, phone, address offline): D4 is void as written. The device needs at-rest encryption, sign-out purge and a decision on remote wipe; the data-protection assessment opens *now* and becomes the schedule's critical path at six weeks; D4's rejected alternative (a) is reinstated on its merits since its only stated objection has evaporated.
- **If D4 holds**: D10 and D9 need redesign, and the room should know they're giving up the use case support called most-requested. Workable substitutes to put in front of them — click-to-call through a proxy that doesn't expose the number, navigation by coordinate rather than stored postal address — but both need product buy-in, and D9 raises a legal question of whether a signature against a summary that can't name the customer is binding.
- **If they propose the middle** (fetch and hold in memory while still in coverage, persist nothing): I'd flag that it fails precisely the multi-day, no-coverage sites in Context, and likely still counts as processing on the device. It's the appealing compromise that doesn't survive the record's own hardest case.

**My default while waiting:** proceed on D1/D2 and the local store; treat the assessment as needed either way and recommend opening it immediately in parallel, because six weeks of queue is the long pole under either ruling and starting it costs little if the answer turns out to be D4.

## Phase 7 — What I'd ask for that isn't in this workspace

State plainly in the review that this is a review of one document with no supporting material: no app source to check the current sync behaviour against, no data-protection assessment criteria to test D4's exemption claim, no ticket sample behind the 40-a-week figure, no list of which regional contracts have no coverage. I'd name these as requests rather than assume them.

## Delegation

None. This is a single-document review; splitting it across workers would fragment exactly the cross-decision reading that produces the findings. I'd do it myself and say so.

## What I'd report at the end

A short message to the room: four blocking items (D4/D10/D9 contradiction, D6 against your own multi-day context, the blocked-state dead end, offline sign-in), roughly eight to resolve before build, D1 and D2 clear to start, ten questions with owners attached, and one decision — D4 versus D10 — that I'm holding the job-card and sign-off work on until they rule. Plus the note that the review lives in `review-offline-mode.md` and I left `record.md` untouched.