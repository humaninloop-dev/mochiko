FILES-READ: `record.md` (the only file in the workspace — confirmed via a glob of `**/*`)

---

# Action plan — review of the concluded offline-mode session

## Phase 1 — Set the deliverable and what I will not touch

- Deliverable: one new file, `review-offline-mode.md`, containing a findings section ordered by severity and a closing list of questions for the room, each tagged with an owner and marked blocking / non-blocking.
- I would **not** edit `record.md`. It is a signed-off record of what the room decided on 2026-09-08; a reviewer overwriting it destroys the audit trail and makes it unclear what was decided versus what was later reinterpreted. Any amendment is the room's to make. I'd say this explicitly at the top of the review.
- I'd re-read `record.md` once more in full before drafting, specifically to check whether each contradiction I think I've found is actually stated or is me filling a gap — the difference decides whether an item is a "conflict" or a "silence."

## Phase 2 — Cross-check every decision against every other decision

Method: build a small matrix of D1–D12 plus the Context section, and walk each pair asking "can both be true on the same device at the same time?" What I expect this to surface, and what I've already found on the first read:

**Blocking contradiction — D4 vs D9 and D10.** D4 says no customer personal data is stored on the device and that name, phone and address are "fetched live and never persisted." D10 says the offline job card shows name, phone and site address, for use *without signal while driving*. D9 says the offline sign-off screen shows the customer's name and site address. These cannot all hold: under D4 the two screens the design exists to make work would render blank exactly when they're needed. This is not a detail to be resolved during implementation — D4's stated payoff is staying out of the data-protection assessment and off a six-week security-review queue, so the schedule assumption rests on the side of the contradiction that D9/D10 already overrule.

**Second-order problem with D4's premise.** Even if D10 were dropped, the device still holds captured signatures (D9), job photos of customer premises (D8), and technician notes (D7). I'd flag — not rule on — that these plausibly identify a customer or their property, so "a lost or stolen device exposes nothing that identifies a customer" likely fails on its own terms, and the rejected alternative (a) may be the honest position. This goes to whoever owns privacy sign-off, not to me.

**Blocking contradiction — D6 vs the Context and D3.** D6 sizes the queue for one shift because "vans return to the depot." The Context says some regional contracts run multi-day jobs at sites with *no* coverage at all, and D3 exists to keep 7 days of jobs precisely for those. So the design's own worst case is days of queued work, and D6 rejected overflow handling as "work for a case that does not occur" — a case the record documents two paragraphs earlier.

**Head-of-line blocking — D5, D7, D12.** A failed replay stops the whole queue, and the queue is per-device, not per-job. One unreplayable write freezes closures for every other job on the device. The record never says what a "failure" is (does a server-side conflict resolution under D7 count?), who can clear it, or whether the technician can skip past it. D12 gives a single "blocked" icon with no stated action behind it.

**The original problem may not be solved.** The motivation is ~40 lost *or duplicated* closures a week. Ordered replay addresses ordering; nothing in the record addresses idempotency, so a write that the server processed but couldn't acknowledge gets replayed and duplicated on reconnect — the failure mode most likely to have produced the duplicate half of those 40.

**Silences I'd list separately:** pull-refresh (D3) versus pending local writes — ordering unspecified and a refresh could clobber local edits; session/token expiry across a multi-day offline job, and whether sign-out wipes an unsynced store; the clock source for D11's 24-hour window on a device that has been offline; jobs assigned or changed after the technician left coverage; a job reassigned to a second technician while the first holds unsynced work; a storage bound for D8 photos and whether reduced resolution survives a warranty dispute; upload "over any connection" and roaming cost for rural work; support-side visibility so support can diagnose a blocked queue rather than absorb it as another ticket.

## Phase 3 — Scenario traces (the substitute for tests; no code exists)

I'd write four end-to-end traces into the review and expect each to fail at a named decision:

1. **Two-day rural job, no coverage at all.** Expect: fails at D6 (queue sized for one shift) and at D3 refresh-on-sign-in, and at session expiry if tokens are short-lived.
2. **Technician calls ahead while driving, no signal.** Expect: fails at D4 — the number D10 promises isn't on the device.
3. **Office reschedules and re-prices at 14:00; technician signs the job off offline at 14:30.** Expect: D7 resolves it, but exposes that the field list omits cancellation, reassignment, and parts/price-affecting technician edits — so the backend lead's table can't be written from the record as it stands.
4. **One write fails at 16:00 with five other jobs queued behind it.** Expect: all six stall; D12 shows one "blocked" icon; no stated recovery.

I'd also trace the room's own open question (a job leaving the 7-day window holding unsynced changes) and propose the answer rather than leaving it open: never evict a job with unsynced work *or* its photos, and make eviction a function of queue state, not calendar age — because the queue holding the write does not keep the job record and attachments the write refers to, nor keep it visible to the technician.

## Phase 4 — Draft the review

Write `review-offline-mode.md` with: what I think is right and should be kept (D1, D2, D5's ordering intent, D11, D12's restraint); three blocking findings (D4/D9/D10; D6; head-of-line + idempotency); the silences as a checklist; the four traces; my proposed answer to their open question; and the questions section from Phase 5.

## Phase 5 — Questions back to the room, by owner

**Blocking — nothing gets built until these are answered:**
1. *Product + privacy owner:* D4 or D10 — which one is real? (See the stop in Phase 6.)
2. *Product + support lead:* what is the actual longest offline stretch a technician can hit, and what should the app do when the queue reaches its bound rather than pretending the bound isn't reachable?
3. *Backend + mobile lead:* is a D7 conflict a queue "failure"? What clears a blocked queue, and can the technician act, or does it need support?
4. *Backend lead:* what makes replay idempotent, given duplicate closures are half the problem being solved?
5. *Backend + mobile lead:* does a session survive a multi-day offline job, and does sign-out destroy unsynced work?

**Needed before the sync design is final, not before it starts:** refresh-vs-pending ordering; the D11 clock source; the full D7 field enumeration including cancel and reassign; photo storage cap, resolution adequacy for disputes, and mobile-data cost; what "blocked" lets the technician do; jobs assigned while out of coverage; two technicians on one job; signature retention and location.

## Phase 6 — The stop, and how I'd proceed under each ruling

I'd stop and put Q1 to the room rather than resolve it myself, because it is a scope-and-schedule decision with a legal dimension, not a technical one. Branches:

- **D10 wins (personal data on device):** D4 is struck; the data-protection assessment and the six-week security review move onto the critical path and should be started the same week, since they'd then be the long pole. Adds encryption at rest, remote wipe, and a retention rule for the 7-day cache.
- **D4 wins:** D10 and part of D9 are struck; the room owes a replacement mechanism for call-ahead and navigation (masked proxy number cached without the identity, coarse site location, or a session-scoped in-memory copy that never touches disk) and an answer for how the sign-off screen confirms the right customer without naming them.
- **Middle path:** minimum fields, encrypted, scoped assessment — cheapest to accept but still needs the privacy owner's ruling, not the room's.

**My stated default while unanswered:** D9 and D10 are grounded in observed field behaviour and D4 is grounded in an assumption about assessment scope that the signature and photo storage probably already breaks — so I'd draft the rest of the review assuming the device holds personal data and the assessment is required, and say plainly that this is an assumption, which branch it came from, and what changes if the room rules the other way.

## Phase 7 — Delegation and what I'd report

No subagents or workers: this is one document, no code, and reading it costs less than briefing anyone. The routing that matters is to people — Q1 to the product lead plus the privacy/security owner, Q3–Q5 to the backend lead, Q2 and the support-visibility question to the support lead.

Closing report to the room: the review file path; three blocking findings and the four traces that demonstrate them; the proposed answer to their open question; the five blocking questions with owners; and one recommendation on their next steps — that the mobile lead's sync design against D5–D7 and the backend lead's conflict table both wait on Q1, Q3 and Q4, because each would otherwise be written against decisions likely to change under them. I'd state clearly that I did not modify `record.md` and that amendments are the room's call.