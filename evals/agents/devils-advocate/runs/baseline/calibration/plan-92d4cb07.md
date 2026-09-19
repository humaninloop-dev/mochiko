FILES-READ: `record.md`

---

# Action plan — cold review of the concluded offline-mode design session

**Verdict I am steering toward (to be confirmed by the passes below):** critical gaps — this record is not safe to build from, primarily because two decisions directly contradict each other and one of them is the stated reason a compliance step was skipped.

In a live run I would first load the thinking-session review procedure so the criteria, severity rubric and output shape come from it rather than from memory. Under this plan-only run I cannot, so I state my working severity scale explicitly and flag that the loaded procedure governs if it differs: **Critical** = will break in production, expose the company, or invalidate a decision the schedule depends on; **High** = will stall or rework a build cycle; **Medium** = will surface as a defect or support load; **Low** = clarity.

---

## Phase 1 — Establish the review frame and the fence

**Do:** Re-read `record.md` end to end twice — once as the room wrote it (do the decisions follow from the context?), once against the grain (what does each decision assume, and where is that assumption stated as fact rather than checked?). Build a working table in my head/notes of: decision → the premise it rests on → whether that premise appears anywhere in the record as evidence or only as belief.

**Read:** `record.md` only. It is the entire workspace — I globbed and there is nothing else.

**Refuse / flag up front:**
- I will not rewrite decisions or draft the missing conflict-rule table. I surface gaps; the mobile and backend leads own the fixes.
- I will not treat "concluded" as meaning the decisions were tested. Status says concluded, awaiting review — the review is mine to actually do.

**Delegation:** None here, and near-none for this card overall. Delegation earns its keep when there is a bulk sweep or a locate to keep out of my context; a single 123-line record is cheaper for me to read myself, and it is exactly the kind of interpretive read I must not hand off. The two places I *would* spawn a cheap disposable reader with the model forced down to the small tier, if the room points me at them:
1. **Brief:** "In the Fieldline repo/docs, find and quote any existing statement of what counts as customer personal data and what triggers a data-protection assessment. Return file path, line span, verbatim quote. Do not interpret." **On return I check:** that it quoted rather than summarized, that the path exists, and that a null result is reported as "not found" rather than inferred — if it comes back empty I do that search myself, because absence here would drive a decision.
2. **Brief:** "List every prior session record or spec in the Fieldline design directory whose title mentions sync, sign-off, or the technician app. Paths and titles only." **On return I check:** completeness against a directory listing before I rely on it.
Neither can run in this workspace, so my plan proceeds without them and I mark the resulting findings as "unverified against external artifacts."

---

## Phase 2 — Contradiction pass (highest-yield, run first)

**Do:** Cross-check every decision against every other decision and against the Context section. This is where the record is weakest, so it goes first.

Specific pairs I would put under the lamp, with what I would carry back:

**2a. D4 versus D10 and D9 — Critical.** D4 says no customer personal data is stored on the device, and that name, phone and address are "fetched live and never persisted." D10 says the *offline* job card shows name, phone and site address, with the stated reason that technicians use them "often without signal." D9 says the sign-off screen shows the customer's name and site address, on a screen the room chose specifically because it is the step most often done offline. These cannot all be true. Either D10 and D9 do not work offline — which guts D1 and the whole purpose line — or the device holds customer personal data, which destroys D4's rationale. And D4's rationale is not cosmetic: it is the sole stated basis for keeping the device out of the data-protection assessment, and the record notes the security review queue is six weeks. So the contradiction is load-bearing on the schedule, not just on the design.

**2b. D6 versus the Context section — Critical.** D6 sizes the queue for one shift and dismisses compaction and overflow as "work for a case that does not occur," resting on "technicians are back in coverage by the end of every shift, since vans return to the depot." The Context paragraph, in the same document, says "some regional contracts run multi-day jobs at sites with no coverage at all." The room's own evidence falsifies the premise of the decision. Also worth putting to them: returning to the depot is not the same as regaining connectivity, and a depot in a rural region may be the same dead zone.

**2c. D5 versus D7 and D12 — High.** D5 says a failed replay stops the queue and "surfaces to the technician." D7 says the device never shows a merge screen because the support lead vetoed adding decisions to the technician's day. D12 gives exactly one indicator with a "blocked" state. So the technician is shown a blocked queue they are explicitly not allowed to act on, with no defined next action, no escalation path, and no support-side tooling decided — despite the support lead being in the room.

**2d. D3 versus D11 — Medium/High.** D3 keeps jobs for a 7-day window; D11 keeps a closed job editable for 24 hours. If a job closes on the last day of its window, or the window refreshes while the job is closed-but-editable, which rule wins? This is adjacent to the one open question the room did log, but broader than it.

**Write:** contradictions section of the review file (path in Phase 6).

---

## Phase 3 — Unexamined-premise pass

**Do:** For each decision, ask what has to be true for it to hold, and whether the record shows anyone checked.

- **The logged open question is under-scoped — High.** The record asks whether a job with unsynced changes survives falling out of the 7-day window, and notes the mobile lead *believes* the queue guarantees it but it was not walked through. A belief recorded as an open question is the right instinct but the wrong scope: the queue holds *writes*, not the job record. If the job is evicted the queue may replay changes against a job that is no longer on the device, and the technician can no longer see what is pending. I would tell the room the question is real, the mobile lead's belief is unverified, and the question needs widening from "is the job kept" to "what does the queue reference and what happens when the referent is gone."
- **Duplicate closures — Critical.** The Context names duplicated job closures as one of the two problems being solved, roughly 40 a week. Nothing in D5 says replay is idempotent. A closure sent, acknowledged after the connection drops, then retried on reconnect, reproduces the exact bug the project exists to fix. I would ask directly whether the room considered idempotency keys and simply did not record it, or did not consider it.
- **Ordering by which clock — High.** D5 replays "oldest first." Oldest by device clock, on a device that has been offline for days and whose clock the user can change. Cross-device and office-vs-device ordering in D7 has the same hole.
- **Authentication across a multi-day offline stretch — Critical, and entirely absent.** D3 says jobs are pulled on sign-in. Nothing says what happens when a session or token expires while the technician is three days into a no-coverage site. If the app locks them out, every other decision here is moot. This is the classic failure mode for offline-first field apps and the record does not mention auth once.
- **Storage at rest — High.** D2 picks SQLite through the standard binding and says nothing about encryption. D4's "nothing sensitive is there" argument was supposed to cover this, and Phase 2a shows that argument does not hold. Notes, photos and a captured signature are on the device regardless.
- **Whose device, and sign-out — High.** Vans and shifts imply shared or handed-over devices. What happens to technician A's unsynced queue when technician B signs in? Does sign-out wipe local data, and if it does, does it destroy unsynced work?

---

## Phase 4 — Missing-decision and scope pass

**Do:** Compare the decision set against the stated purpose — "started, worked, and closed in the field and reach the office" — and against the Context, and list what a builder would have to invent.

- **The parts list — High.** Context names the job card, the parts list, and the sign-off form as the three things needing a live connection. D1 through D12 cover the card and the sign-off. Nothing covers parts: whether the technician can record parts used offline, and what stops two technicians offline-allocating the same stock. Either it is out of scope and the record should say so, or it is a hole in the purpose.
- **What "reduced resolution" means, and what it costs — Medium/High.** D8 gives no target resolution, does not say whether the full-resolution original is kept or discarded, and does not say whether the reduced image is sufficient if a photo is later used as evidence in a warranty or damage dispute. "Upload over any connection" also has no cost, roaming or battery position, and no cap on device storage or defined behavior when the disk fills.
- **The signature's status — Critical, adjacent to 2a.** D9 captures a customer signature and stores it locally until sync. A handwritten signature is very plausibly personal data and arguably biometric in some regimes. That lands squarely on D4's assessment-avoidance claim. I flag it as a question for whoever owns data protection, not as my ruling.
- **A signed document that can change afterward — High.** D9 has the customer sign a job summary; D11 lets the technician edit that job for 24 hours after closure. The customer signed something that is then mutable, with no record of what they actually saw. If the signature has any evidential purpose, this needs an immutable snapshot at signature time.
- **24 hours measured how — Medium.** Device clock, offline, user-settable.
- **Cancelled jobs — High, and a direct feed to the backend lead's next step.** D7 splits fields: office wins on schedule and price, technician wins on status. If the office cancels a job while the technician works it offline and closes it, is "cancelled" a schedule change (office wins, the technician's completed work is voided) or a status change (technician wins, a cancelled job comes back closed)? The rule table cannot be written without this.
- **No success measure.** The Context gives a clean baseline — about 40 lost or duplicated closures a week. No decision states a target or how it will be observed after launch.
- **Nothing on the very first sync or a cold start with no coverage**, and nothing on what the technician sees for a job that exists in the office but was assigned after they lost signal.

---

## Phase 5 — The stop, and the branches

**Stop and confirm before any of this is built:** the D4 / D9 / D10 collision is a product-and-compliance ruling, not mine. I would state the finding, name the options, and hold. What I would put to the room, framed as the decision it actually is:

> D4 buys you a six-week schedule saving by claiming the device holds nothing identifying. D9 and D10 put the customer's name, phone, address and signature on the device, offline, by design. Which one is real?

**Branch A — customer data does go on the device (D10 and D9 stand).** Then D4 is withdrawn, the data-protection assessment is back on the critical path, and encryption at rest, remote wipe, and sign-out wipe become decisions the room has to take. The six-week queue re-enters the schedule and the next steps in the record need re-sequencing before the mobile lead starts the sync design.

**Branch B — D4 stands.** Then D10 is dead as written and D9 is at risk, and the room must say what the technician sees on a job card and a sign-off screen with no signal — a customer ID and nothing else is not a usable card, and I would push back that this reopens D1.

**Branch C — a middle position** (for example, a short-lived cached copy that is wiped on job close or after N hours, or address-only with tap-to-call resolved live). This is likely where they land, but it is a new decision the room has not taken, and it does not automatically preserve D4's compliance claim — that still needs the data-protection owner's word, not the room's.

**My stated default if no one rules:** I record it as Critical and unresolved, and the verdict stays critical-gaps. I do not pick a branch on the room's behalf, and I do not soften the finding because the six-week queue is inconvenient — the inconvenience is the reason the finding matters.

Second, smaller stop: whether the parts list is in scope. Default if unanswered — treat it as an unacknowledged scope gap and report it as such rather than assuming it was deliberately excluded.

---

## Phase 6 — Write the review and hand it back

**Write:** `review-offline-mode.md` in this workspace — a new file alongside the record. I do **not** edit `record.md`. Amending decisions, widening the open-questions list, and re-sequencing the next steps are the room's to do with my findings in hand.

Structure of the file:
1. Verdict line and the evidence for it.
2. Findings, ordered by severity, each with: which decisions it touches, what breaks, and what it blocks. Contradictions first (2a, 2b, duplicate-closure idempotency, offline auth, signature status), then High, then the rest.
3. The questions back to the room, grouped by who can answer them.
4. What I could not check: no access to the Fieldline data-protection assessment, no prior session records, no support ticket data behind the "40 a week" figure, and no ability to confirm the mobile lead's belief about the queue. These are listed as unverified, not quietly dropped.
5. What is genuinely good, cited specifically — because a verdict needs evidence either way. Concretely: D1's framing of offline as the default rather than an error state is the right root decision and every downstream decision respects it; D7's field-level split is a real design position with a stated rationale rather than a punt, and rejecting last-writer-wins for the correct reason (a reschedule silently voiding a sign-off) shows the room reasoned about a concrete failure; D11 is driven by observed support tickets rather than taste; and each decision carries its rejected alternatives with reasons, which is what made this contradiction pass possible at all. The record is well-formed; it is the reasoning inside three or four decisions that does not hold.

---

## Phase 7 — Questions I would put back to the room, before anything is built

Ordered so the room can answer top-down; the first four block the build.

**To the product lead and whoever owns data protection**
1. D4 says no customer personal data on the device; D10 and D9 put name, phone, address and a signature there, offline, by design. Which is real — and does the data-protection owner, not this room, agree the answer keeps the device out of the assessment?
2. Is a captured customer signature personal data under the standard you are applying?
3. If the answer to 1 puts the assessment back on the path, does the six-week queue change what ships first?

**To the mobile lead**
4. What happens when the technician's session or token expires mid-job at a site with no coverage for three days? Nothing in the record mentions authentication.
5. D6 assumes every shift ends back in coverage, but your own context says some contracts are multi-day with no coverage at all. Which is it — and if the second, what happens when the queue fills?
6. Your open question asks whether an unsynced job survives eviction from the 7-day window. Broader version: what does a queued write reference, and what happens if that job record is gone?
7. D5 stops the queue on a failed replay and "surfaces" it, but D7 forbids putting decisions in front of the technician. What is the technician actually supposed to do when D12 shows "blocked"? What is the retry and escalation path, and who at support can unblock it?
8. Is replay idempotent? Duplicate closures are one of the two problems in the brief; an acknowledged-then-retried closure recreates them exactly.
9. Whose clock orders the queue, and what stops a device with a wrong clock from replaying out of order?
10. Shared or handed-over devices: what happens to unsynced work when a different technician signs in, and does sign-out wipe?

**To the backend lead**
11. Cancellation: office cancels while the technician closes offline. Schedule field (office wins, work voided) or status field (technician wins, cancelled job returns closed)? The rule table cannot be written without it.
12. D11 lets a job be edited for 24 hours after closure. How do those late edits interact with the conflict rules once the office has already processed the closure?

**To the product and support leads**
13. D9 has the customer sign a summary that D11 then allows to change. Is the signature evidential? If so, what immutable snapshot of what they saw is retained?
14. What is the target for the ~40 lost or duplicated closures a week, and how will you see it after launch?
15. Is the parts list in scope? The context names it as one of the three blocked flows and no decision covers it.
16. D8: what resolution, is the original kept, and is the reduced image good enough for a warranty dispute? Any position on cellular cost, roaming, or a device storage cap?

---

## What I would report at the end

A short handback to the room: verdict **critical-gaps**; two Critical contradictions inside the record itself (D4 against D9/D10, D6 against its own context paragraph), two Critical absences (offline authentication, replay idempotency against the duplicate-closure problem the project exists to solve); the review file at `review-offline-mode.md`; a recommendation that both listed next steps hold — the mobile lead's sync design depends on D5–D7 surviving questions 5 through 10, and the backend lead's conflict table cannot be completed without question 11 — and an explicit note that the D4 branch is a product-and-compliance ruling I have flagged and deliberately not made.