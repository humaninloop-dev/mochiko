FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-1zwv2g21/ws/record.md` (the only file in the workspace)

---

# Plan — review of the concluded offline-mode design session

The deliverable is a written review returned to the room plus a prioritized question list, not code. There is no application source in this workspace, so nothing can be verified against an implementation; the review is an internal-consistency and completeness check of the record against its own stated context.

## Phase 1 — Build the decision inventory

Re-read `record.md` end to end and extract, for each of D1–D12: what it asserts, what it assumes to be true about the world, what data it implies lives on the device, and which other decisions it touches. Also extract the four factual claims in **Context** (no signal minutes-to-hours; multi-day jobs at zero-coverage sites; ~40 lost/duplicated closures a week; job card + parts list + sign-off currently need a connection) because several decisions are justified by them and at least one contradicts them.

Nothing written yet. Output is a working table I carry into Phase 2.

## Phase 2 — Contradiction pass

Check every decision against every other decision and against the Context section. Findings I already have from the read, each of which I would state in the review with the exact decision numbers:

1. **D4 vs D10, and D4 vs D9 — the central contradiction.** D4 says no customer personal data on the device and that this is what keeps the device out of the data-protection assessment. D10 puts name, phone, and site address on the offline job card *specifically for use without signal*. D9 shows name and site address on the offline sign-off screen and stores a captured signature locally until sync. D4 cannot be true while D9 and D10 are true. D4's stated payoff — avoiding the assessment and the six-week security review queue — is therefore not available, and the room recorded it as a settled benefit.
2. **D6 vs Context and vs D3.** D6 asserts technicians are back in coverage by the end of every shift, so the queue holds at most one shift. The Context says some regional contracts run multi-day jobs at sites with no coverage at all, and D3's 7-day window was sized explicitly so multi-day jobs do not fall off mid-job. D6's premise is contradicted by the record it sits in, and D6 used that premise to delete overflow handling and compaction.
3. **D5 vs D12 vs D6.** D5 stops the whole queue on a failed replay. Nothing in the record says how a stopped queue is cleared, whether the technician can keep working into it, or whether one bad write on job 3 strands jobs 4–9. D12 gives a single "blocked" state and a list of pending items, which tells the technician something is wrong but not what to do. With D6 having removed overflow handling, a queue that is both blocked and growing has no defined behavior.
4. **D11 vs D6 and vs D7.** A job closed at 18:00 and edited at 21:00 puts writes into the queue after the shift, and edits arriving after a closure has already synced are server-side edits to a closed job — D7's field rules do not say whether post-closure technician edits still beat office edits.
5. **D7 gaps.** The field split (office: schedule, price; technician: status, notes, photos, sign-off) has no ruling for cancellation (is cancelling a schedule change or a status change?) and no ruling for parts consumed, which the Context names as one of the three things that need connectivity today. It also does not say whether resolution applies per queued write or per final job state, which interacts directly with D5's ordered replay.
6. **D3 vs the room's own open question.** The open question — job with unsynced changes ageing out of the 7-day window — is left resting on the mobile lead's belief that "the queue guarantees this." The queue holds writes; D3 governs which *job records* are on the device. If a refresh evicts the job row, queued writes reference a job the device no longer holds, and D11's 24-hour edit window has nothing to edit.

## Phase 3 — Scenario traces (the verification instrument in place of tests)

No code exists, so instead of running tests I would write a small set of end-to-end traces into the review and walk each one against D1–D12. These are what turn the Phase 2 findings from assertions into things the room can check. The traces and what I expect each to show:

- **T1 — Two-day rural job.** Sign in at depot Monday, no signal Monday 09:00 to Wednesday 11:00. Expect: D3 holds the job fine; D6 fails outright (two shifts of queue); offline session/token lifetime is undefined anywhere in the record.
- **T2 — Call ahead while driving, no signal.** Expect: the phone number has no defined source. Proves D4/D10 is a real contradiction, not a wording problem.
- **T3 — Offline sign-off.** Expect: customer name, address, and a captured signature sit on the device until sync — precisely the exposure D4 claims does not exist.
- **T4 — Poison write.** Third write of nine is rejected by the server. Expect: no defined recovery, unclear blast radius across the other jobs, and D12 surfaces a state with no action attached.
- **T5 — Office reschedules a job the technician closed offline.** Expect under D7: closed *and* rescheduled for next week simultaneously; no ruling on what the office sees.
- **T6 — Office cancels a job the technician signed off offline.** Expect: D7 has no answer.
- **T7 — Day-7 boundary with unsynced changes.** Expect: the open question resolves to "no, nothing guarantees this" unless eviction is explicitly gated on an empty queue for that job.
- **T8 — Device lost with a day of unsynced work.** Expect: unrecoverable, which reproduces the ~40 lost closures a week the project exists to fix, in a new form.
- **T9 — Parts used offline.** Expect: falls outside D7's field list and outside every decision.

## Phase 4 — Gap pass

List decisions the room needed and did not make: offline authentication and session lifetime (nothing in the record; D3 only says jobs pull on sign-in); device clock trust, given that offline timestamps drive queue order, the D11 24-hour window, and sign-off time; encryption at rest for whatever does live locally, which D4 rejected as part of a bundle rather than on its own merits; how dispatch changes reach a device that is offline for two days when D7 says office wins on schedule; photo resolution defined concretely, plus whether a reduced-resolution photo is still acceptable as contractual evidence, plus cellular data cost of D8's "any connection"; whether D2's "the platform's standard binding" means one platform or two, and who owns schema migration; and how success gets measured after ship against the 40/week baseline.

## Phase 5 — Write the review

Write `review-offline-mode.md` in the workspace root, alongside `record.md`. Structure: a short "what's strong" opening (the record captures rejected alternatives with reasons, which is what makes this review possible at all), then findings ordered blocking → significant → minor, each naming the decisions involved and the trace that exposes it, then the Phase 3 traces in full, then the question list from Phase 6.

I would **not** edit `record.md`. It is a concluded record of what the room decided on 2026-09-08; corrections to it belong in the review and in a follow-up session, not in a silent rewrite of the minutes. If the room would rather have findings appended to the record, that is a one-line change to this plan and I would ask before doing it.

## Phase 6 — The questions back to the room, with defaults

Four blocking, in this order. Blocking means I would recommend the first next step (mobile lead's sync design against D5–D7) not start until they are answered, because each one changes the local schema or the queue design.

**Q1 — D4 or D10/D9: which one is actually the decision?** This is a privacy-posture and schedule call that belongs to product plus whoever owns the data-protection assessment; I would not settle it in a review. Branches:
- *D4 stands* → D10 is void, D9's summary drops to non-identifying fields, the call-ahead use case (the record's most-requested use) dies, and the signature needs a home that is not the device. Back to product and support.
- *D10 and D9 stand* → D4 is void. The device holds identifying data, the assessment is required, and the six-week security review queue lands on the critical path now rather than at hardening. Encryption at rest returns as a live decision, since D4 rejected it only as part of a bundle whose payoff has evaporated.
- *Middle path* → identity fields cached with a defined lifetime and scope (e.g. only for jobs in today's route, wiped on close). Still needs privacy sign-off; still not free.

My stated default while this is unanswered: the second branch. The D9/D10 requirements are grounded in observed field behaviour, while D4's rationale is avoidance of a process step, and a process step cannot be avoided by a design that doesn't hold. I would plan and cost the rest of the review on that basis and flag the schedule hit explicitly.

**Q2 — Does the queue have to survive more than one shift?** The Context says yes. If the room confirms, D6 is reopened and both the sizing and the deleted overflow/compaction behavior come back. Default: yes, size to the longest offline stretch the regional contracts permit, plus a defined behavior on exhaustion.

**Q3 — How does a blocked queue get cleared, and what is the blast radius?** Can the technician keep working while blocked; does one bad write strand the rest of the day; is there an office-side unblock; what does the technician actually do when D12 shows "blocked"? Default: this must be answered inside the mobile lead's sync design before it is considered complete.

**Q4 — What is the offline session lifetime?** Not addressed anywhere. Default: the session must outlast the longest planned offline stretch with margin, and expiry-while-offline must not lock a technician out of a job in progress.

Then the non-blocking questions from Phase 4, which the backend lead's conflict table and the sync design can absorb: cancellation and parts in D7; per-write vs per-state resolution; device-clock trust; concrete photo resolution and evidentiary standing; data cost of D8; the D3 eviction rule for jobs with pending writes (this converts the room's existing open question into a decision rather than a belief); device-loss recovery; and D2's platform scope.

## Phase 7 — Delegation

None. This is a 123-line document reviewed in a single pass; splitting it across workers would cost more in reconciliation than it saves, and the findings depend on holding all twelve decisions against each other at once. I also cannot start the multi-agent cloud review from here — `/code-review ultra` is user-triggered and billed, so if the room wants a second independent pass over the eventual sync design, that is theirs to launch.

## Phase 8 — What I would report

- The record is well-formed and the alternatives-with-reasons format is worth keeping; that is not padding, it is why the contradictions were findable.
- One contradiction invalidates a decision the room believed it had banked: D4's promise of staying out of the data-protection assessment does not survive D9 and D10. This is the headline, and it moves a six-week queue onto the critical path unless product rules the other way.
- One decision (D6) is justified by a premise the record's own Context contradicts, and that premise was used to delete work (overflow, compaction) rather than to defer it.
- Two failure paths are undefined rather than decided: blocked-queue recovery, and offline session expiry.
- The room's single open question is real and understates itself — it is not only about the queue, it is about whether the job record survives eviction.
- Recommendation on the two agreed next steps: the backend lead's D7 conflict table can start immediately, provided it adds cancellation and parts. The mobile lead's sync design should wait on Q1–Q3, since Q1 determines what is in the local schema and Q2/Q3 determine the queue's shape.
- Explicitly stated: I did not resolve Q1, and the rest of the review is written assuming the device does hold customer identity data.