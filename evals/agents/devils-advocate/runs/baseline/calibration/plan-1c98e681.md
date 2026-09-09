FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-0l_4home/ws/record.md`

---

# Action plan — cold review of the concluded `offline-mode` design session

## Phase 0 — Establish the review surface, and refuse to widen it

**What I would do.** Confirm what exists in this workspace before judging the record. I already ran the enumeration myself rather than handing it to a cheap reader: the result here is *absence*, and absence drives the central judgement of this review (whether the record stands alone or leans on documents I am supposed to have). A worker returning "no other files" is a fact I would then have to re-verify anyway, so delegating it saves nothing and risks a false clean. Result: `record.md` is the only file.

**Consequence I would carry into every later phase.** The record cites five external things that are *not* in the workspace: the Fieldline product itself, the support ticket log ("about 40 lost or duplicated job closures a week"), the "regional contracts" with multi-day no-coverage sites, the data-protection assessment process, and the six-week security review queue. I would not fetch, infer, or reconstruct any of them. Everything asserted on their authority — most importantly D4's claim that this design "lets us keep the device out of the data-protection assessment" — gets reviewed as an *unverified premise the room is relying on*, not as established fact. That is a finding in its own right, not a limitation I quietly absorb.

**What I would refuse.** I would not treat "the mobile lead believes the queue guarantees this" (the open question under D3) as a resolution. A belief recorded as unwalked-through is an open decision, and I would say so.

**Delegation.** None in this phase or any other. The whole artifact is 123 lines and every part of it is interpretive — the findings live in the *interactions between* decisions, not in any span a locate-and-quote worker could return. There is no bounded enumeration, no deterministic check, and no bulk read to keep out of my context. Dispatching a Haiku reader here would cost a round trip and return me a file I have already read. I would note this explicitly in my report so the non-delegation reads as a decision, not an oversight.

## Phase 1 — Build the decision-interaction matrix before forming any opinion

**What I would do.** Take D1–D12 and cross every decision against every other, plus against the Context section, looking for pairs that cannot both be true, or that are silent about what happens where they meet. I do this mechanically and on paper before I form a verdict, because the reading that feels fine on first pass is exactly where the expensive contradictions hide — each decision here is individually reasonable, and that is the warning sign.

**Specific pairs I would force through, and what I already expect to find:**

- **D4 × D10, and D4 × D9.** D4: no customer personal data on the device; name, phone and address are "fetched live and never persisted." D10: the *offline* job card shows name, phone and site address, explicitly for use "while driving between jobs, often without signal." D9: the sign-off screen shows the customer's name and site address, on a screen D1 requires to work with no connectivity. These cannot all hold. A live fetch is unavailable in precisely the conditions D10 and D9 are specified for.
- **D4's compliance claim standing on that contradiction.** D4's rejected alternative (a) was declined partly to avoid a six-week security queue. The saving is claimed on a data-minimisation property that D9 and D10 dissolve. I would trace whether anything downstream in the record depends on the assessment being avoided — Next steps do not mention it, which means the schedule assumption is load-bearing and unowned.
- **D4 × D2 × D8 × D9, on a second axis.** Even granting the ID-only rule, the device holds site photos, free-text technician notes, and a captured customer signature. I would ask whether the room considered that a signature and a technician's notes are themselves customer-identifying, and whether D2's SQLite store is encrypted at rest at all — D4 rejected "encrypt at rest *and store everything*," which decides the storage scope, not the encryption.
- **D6 × Context × D3.** D6 asserts technicians are back in coverage by end of shift because vans return to the depot, and concludes the queue needs no compaction or overflow handling. The Context section states some regional contracts run multi-day jobs at sites with no coverage at all, and D3 rejected a 1-day window for exactly that reason. D6's premise is contradicted by the same document, two decisions earlier.
- **D6 × D8.** Reduced-resolution photos accumulating across a multi-day no-coverage job against a queue explicitly designed with no overflow handling, on a store with no stated size bound. I would look for any statement of what happens when the device fills. There is none.
- **D5 × D8.** Whether photos ride the same strictly-ordered queue as job-state writes is never stated. If they do, one failed large upload on a flaky link blocks every subsequent job closure; if they do not, D5's ordering guarantee does not cover photos and a photo can land against a job state that has moved.
- **D5 × D7.** The queue replays oldest-first; the server applies field-level rules. Whether "the office won this field" counts as a *failed* replay (stopping the queue per D5) or a silent success is undecided, and the two readings produce opposite field behaviour.
- **D7 × D12 × technician awareness.** D7 forbids a merge screen, which is a sound call. But it never decides whether the technician is *told* their offline schedule or price edit was overwritten. "No merge screen" and "no notification" are different decisions and the record only makes one of them.
- **D11 × D9.** A job stays editable for 24 hours after closure — after the customer has signed a summary. The signed artifact and the stored artifact can diverge with no record of which content was attested.
- **D11 × D5 × device clock.** What starts the 24 hours, and on whose clock — a device clock that is offline, drifting, and user-settable. Queue ordering (D5) rests on the same clock.
- **D3 × the record's own open question.** The one thing the room already knows is unresolved is not assigned to anyone in Next steps.
- **Purpose × everything.** The purpose cites 40 lost or duplicated closures a week. No decision states how the room would know whether that number moved.

**What I would write.** Nothing to disk in this phase; the matrix is working material. No test exists to run here — this is a document review, there is no built system, and I would refuse to pretend otherwise.

## Phase 2 — Hunt the decisions that *aren't* in the record

**What I would do.** Walk a technician's full offline day against the twelve decisions and mark every point where the app must do something no decision covers. Missing decisions are the failure mode of a "concluded" record, because the room stops looking once the list feels complete.

**The traversal, concretely.** Van leaves depot → sign-in (D3 pulls jobs *on sign-in* — what happens when the app is cold-started or the session expires with no signal? If sign-in requires connectivity, offline mode fails exactly when it is needed, and nothing in D1–D12 decides this) → drive to site, call ahead (D10, which needs data D4 forbids) → work the job → job is reassigned or cancelled by the office while the technician is offline and still working it (undecided) → sign-off (D9) → drive on, job list refreshes → does a refresh evict a job holding unsynced work (the open question) → end of shift, technician signs out or hands the device to the night shift (does sign-out wipe the store, and with unsynced work in it?) → technician logs into a *different* device tomorrow while yesterday's queue is still unreplayed (duplicate closures — the exact failure the project exists to eliminate) → a write fails on replay and the queue blocks (D5/D12): who unblocks it, with what tool, on what timescale, and can the technician keep working meanwhile, or is the whole shift's output frozen behind one bad record?

I would also probe the abuse and boundary edges the room did not raise: a lost device holding an unreplayed shift; a technician who deletes and reinstalls the app; a customer who refuses to sign; a job closed offline whose customer record was deleted server-side before replay; two technicians on the same job.

## Phase 3 — Classify, and hold the severities

**What I would do.** Assign each finding a severity on impact if built as written, not on how awkward it is to raise. My working expectation before final calibration:

- **Critical:** the D4 ↔ D9/D10 contradiction, and the compliance claim resting on it. The D6 one-shift premise contradicted by the record's own multi-day no-coverage context, together with the absent storage/overflow bound. The blocked-queue dead end in D5/D12 with no owner or recovery path.
- **High:** identity/sign-in offline being entirely undecided; the D5×D8 single-or-separate-queue ambiguity; the D5×D7 "is an office win a failure?" ambiguity; the D11 post-signature editability question; eviction and multi-device re-login duplicate risk.
- **Medium:** clock trust for ordering and for the 24-hour window; whether the technician is notified of overwritten fields; D12's scope ambiguity (one icon for the list, or one per job row); encryption-at-rest of the SQLite store; the absence of any way to tell whether the 40-a-week number moved.

**What I would refuse.** To soften the D4 finding because the fix costs the room a six-week security queue slot. The cost of the fix is the room's problem to weigh, not my reason to downgrade. And to declare the record clean — a review of a twelve-decision record that surfaces nothing means I read it as its authors intended rather than as an adversary.

**What I would not do.** Write the corrected decisions. I would not draft a replacement D4, propose the storage cap number, or specify the unblock flow. Every finding goes back with options and a question, and the room decides. Authoring the fix is the room's job; if I write it, nobody reviews it.

## Phase 4 — Frame the questions for the room

**What I would do.** Convert each Critical and High finding into a question the product lead can actually rule on — user-impact framing, concrete options, consequence attached to each. These are the questions I would put back before the mobile lead or backend lead starts the Next-steps work:

1. **Which is true — D4 or D10?** Options: (a) the offline card shows name/phone/address, the device is in scope for the data-protection assessment, and the six-week queue is on the critical path; (b) the offline card shows the site address only and technicians cannot call ahead without signal, contradicting the "most-requested use" that justified D10; (c) something narrower — e.g. a first name and a masked number. There is no option where D4 and D10 both survive unchanged. And whichever wins, **does D9's sign-off summary still show the customer's name and address to the person signing?** — that is a separate ruling with a legal flavour, not a consequence of the first.
2. **Are the signature, the site photos, and free-text notes personal data on this device, and who has actually said so?** D4's saving is claimed against a process nobody in the room owns.
3. **Does the queue survive a multi-day no-coverage job?** D6 says one shift; the Context section and D3 say multi-day sites exist. Which population is D6 designed for, and what does the app do when the storage or queue bound is reached mid-job — refuse new work, drop photos, or degrade some other way?
4. **When the queue blocks, who unblocks it and how long does the technician wait?** Can they keep closing other jobs while one write is stuck, or is the shift frozen behind the head of the queue? Is there a back-office tool, and is it in scope?
5. **Can a technician start a shift, or restart the app, with no signal?** If not, D1's "offline is the default working mode" is not true at the boundary that matters most.
6. **Is a server-side field loss under D7 a failed replay, and does the technician find out?** Two sub-rulings: does it stop the queue, and does it produce any visible signal.
7. **Do photos ride the same ordered queue as job state?** If yes, accept that a stuck upload blocks closures; if no, accept that photos can arrive against a moved job state.
8. **What was signed?** If D11 lets notes and photos change for 24 hours after the customer signed the summary, does the signature bind the content at signing time, and does anything record that content?
9. **What happens to unsynced work on sign-out, on job eviction from the 7-day window, and on login from a second device?** The third case reproduces the duplicate closures this project exists to remove.
10. **How will the room know this worked?** The purpose names 40 incidents a week; nothing measures them afterwards.

**The stop point.** Question 1's ruling is not mine — it trades a product capability against a schedule and a compliance exposure, and it belongs to the product lead with the user in the loop. **If the ruling is (a)**, the assessment enters the critical path, D4 is rewritten by the room, and the Next-steps sync design can proceed since D5–D7 are unaffected. **If (b) or (c)**, D10 and D9 are rewritten, support's call-ahead expectation needs re-setting, and I would want the support lead back in the room before that is settled. **Until it is ruled either way**, I would not have anyone build against D4/D9/D10. I would say plainly that I do not gate this alone: I hand up the finding and the recommendation, and the lead decides whether the Next-steps work starts.

## Phase 5 — Verdict, evidence, and hand-back

**What I would produce.** A single review returned to the room, in this shape:

- **Verdict: critical-gaps** — not needs-revision. Proposed, not imposed; the lead rules. My default while awaiting that ruling is that the two Next-steps items are held for the D5-adjacent parts and blocked for anything touching D4/D9/D10.
- **The findings**, ordered by severity, each citing the decision numbers it spans, stating what breaks in the field if built as written, and carrying its question from Phase 4. Contradictions cite both sides verbatim so the room cannot read past them.
- **Evidence for what is genuinely strong**, because a verdict with no cited strengths is as lazy as one with no findings: every decision records its rejected alternatives *with reasons*, which is what made this cross-check possible at all; D7's field-level split is a real design call with a named owner (the support lead's veto) rather than a deferral; D11 is grounded in observed support tickets; D1 traces cleanly to the stated context. The record's weakness is not carelessness — it is that each decision was sound in isolation and the room never crossed them against each other.
- **An explicit note that the record's own open question under D3 remains open and unassigned**, and that "the mobile lead believes the queue guarantees this" is not an answer.
- **A disclosure of my own limits:** I reviewed one file; the support log, the contract terms, the assessment criteria and the product itself were not available to me, and every finding that depends on them is flagged as resting on the room's unverified premise rather than on anything I checked.

**What I would not report.** A count of findings as a proxy for rigour, and any suggestion that the room's decisions were careless. The point of the hand-back is that they can act on it tomorrow, not that they feel reviewed.