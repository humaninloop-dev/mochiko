FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-nqfyssio/ws/spec.md`

(That is the only file in the workspace — I globbed `**/*` and it returned one entry.)

---

# Action Plan — Review of FEAT-022 Self-Service Rescheduling

## Scope decision up front

The author asked me to restrict the review to **Functional Requirements** and **Success Criteria**, and to skip wording. I would honor both:

- **Drop entirely:** style, phrasing, heading structure, MUST/SHOULD hygiene, doc formatting. No nits.
- **Keep in full:** every substantive defect in FR-001…FR-008 and SC-001…SC-004, at full severity. "Go easy on the wording" is a request about polish, not about how hard I press on a contradiction that would ship a double-booking.
- **Two items sit outside the requested scope but are load-bearing for the FRs.** I would raise them, clearly labeled as out-of-scope-but-blocking, rather than silently withholding them — see Phase 5.

## Phase 1 — Build a traceability matrix (analysis, no files written)

Read: `spec.md` lines 26–125 (already done).

Construct three tables in my working notes:

1. **US → FR coverage.** Every acceptance criterion in US-001…US-004 mapped to the FR(s) that would implement it. Flag criteria with no FR.
2. **FR → US backing.** Every FR mapped to its claimed `*Source:*`. Flag any FR whose stated source does not actually say what the FR says.
3. **SC → data source.** For each success criterion, name the FR (or existing system) that produces the measurement data. Flag any SC whose data source is not produced by anything in this spec.

This matrix is the verification method for the whole review — it is how I would catch gaps rather than asserting them.

**Expected result of the matrix, based on my read:**
- US-001's third criterion (abandon leaves appointment unchanged) is contradicted, not implemented.
- US-003's "old time is open again" has no FR at the correct moment.
- US-004's "history lists each change… and whether the patient or a staff member made it" has no display FR, and the actor field is missing from FR-008.
- SC-001's data source (the change log) does not record the field the metric needs.

## Phase 2 — Walk each Independent Test through the FRs

For each of the four `**Independent test:**` blocks, trace the steps against FR-001…FR-008 and record pass / fail / undefined.

- **US-001 test** (book, open link, move two days later, confirm): passes the happy path, but the third acceptance criterion adjacent to it fails under FR-006.
- **US-002 test** (appointment starting in one hour): **undefined** — FR-005 says "imminent" and never defines it. The test asserts a one-hour case that no requirement decides.
- **US-003 test** (old slot bookable, new slot taken): passes for the wrong reason — FR-006 released the old slot much earlier than confirmation.
- **US-004 test** (once from patient link, once from front desk, "with the correct actor"): **fails** — FR-008 does not record the actor.

Two of four independent tests cannot be executed against the requirements as written. That is the headline structural result and I would report it that way.

## Phase 3 — Draft the Functional Requirements findings

Ordered by severity, each with the concrete text problem, the consequence in the terms the spec itself uses, and a proposed fix.

**F1 — Blocker. FR-006 contradicts FR-002 and US-001, and its cited source does not support it.**
FR-006 releases the original slot "as soon as the patient opens the reschedule page." FR-002 requires the original appointment stay intact until confirmation; US-001's third criterion requires an abandoned reschedule to change nothing; US-003 (FR-006's cited source) releases the old time only "when the change is saved." Under FR-006, a patient who opens the link and closes the tab has silently lost their appointment slot to whoever books it next — and the Overview names exactly this outcome: "a slot booked twice means one patient is sent home on arrival." Proposed fix: delete FR-006; add an FR that releases the original slot as part of the confirmation transaction, alongside FR-004. If the intent behind FR-006 was to stop the patient's own old slot from looking unavailable to themselves, that is a display concern and should say so.

**F2 — Blocker. Nothing specifies what happens when the chosen slot is gone.**
FR-003 loads slots "once when the page opens"; FR-004 says the system MUST book the selected slot, with no failure branch. The Overview states clinics run at capacity, so two patients reaching for the same slot is the expected case, not the edge case. Needs: either a short hold on the selected slot, or a defined conflict outcome (refresh the list, tell the patient, keep the original appointment intact), plus a statement that the original appointment survives a failed confirm.

**F3 — Blocker. "Imminent" in FR-005 is undefined and is not listed as an open question.**
US-002 says "close to its start time," the independent test assumes one hour, FR-005 says "imminent." No cutoff exists anywhere. This is untestable and it also silently gates SC-001 and SC-003 (see F9). Needs a number, or an explicit "PM to set; this draft assumes N hours" so the assumption is visible.

**F4 — Major. FR-008 drops the actor, which US-004 explicitly requires and SC-001 depends on.**
FR-008 records previous time, new time, and change time. US-004 requires "whether the patient or a staff member made it," and SC-001 says it is measured from this same log. Add the actor (and actor type) to FR-008. Also state that staff-initiated reschedules write to the same log — otherwise SC-001 has no denominator.

**F5 — Major. FR-003 loses the same-practitioner / same-appointment-type filter.**
US-001 specifies slots "for the same practitioner and the same appointment type"; Out of Scope confirms a different practitioner is deliberately excluded. FR-003 just says "the open slots." As written, the page could offer slots that the feature is not allowed to book.

**F6 — Major. No requirement covers the reschedule link's lifetime or binding.**
FR-001 generates a link and emails it; nothing says the link is bound to one appointment, stops working once the appointment has passed or been cancelled, or is unusable after the reschedule it authorized. The spec's own premise (line 23–24) is that the link alone identifies the patient with no sign-in, which means a forwarded or leaked email is full authority over that appointment. At minimum: bind the link to a single appointment, expire it at the appointment's start time, invalidate it after use. Whether more is needed is a decision for the author and PM, but the current draft has no requirement at all here.

**F7 — Minor. US-004's history display has no FR.**
FR-008 records the history; nothing requires staff to be able to see it. The story and its independent test are both about staff reading it.

**F8 — Minor, but worth a decision. The open question about reschedule limits has a cost the draft does not name.**
"This draft sets no limit," combined with FR-001 issuing a fresh link on every new confirmation, means unlimited self-service churn against slots the clinic considers scarce, and it inflates SC-001 (one patient rescheduling five times counts as five self-service reschedules). I would recommend the draft state a provisional limit rather than leaving it open, and note the SC-001 interaction.

## Phase 4 — Draft the Success Criteria findings

**F9 — SC-001 cannot be computed from the source it names.** It is measured "from the appointment change log," but FR-008's log has no actor field (F4). Also: patients refused by FR-005 are pushed to the phone and land in the denominator, so the 60% target moves with the undefined "imminent" threshold (F3). Fix F4, then restate the target as a share of *eligible* reschedules, or accept and state that the refused population counts against it.

**F10 — SC-002 cites a measurement instrument that does not exist.** "The page's existing timing beacon" — the reschedule page is the thing this spec is building; it has no existing anything. Either the beacon is on a different page (say which), or instrumentation needs to be a requirement. Separately, the criterion conditions on patients who *select a slot*, which excludes everyone who opened the page and gave up — the failure mode most worth catching. And a 90-second median is a design target, not an outcome measure; I would ask what decision gets made if it comes in at 100 seconds.

**F11 — SC-003 has no baseline and a soft instrument.** A 30% fall in front-desk phone time depends on call-reason tags applied by hand by the staff whose workload the feature is changing; tagging discipline will drift in the same direction as the metric. Needs a defined pre-launch baseline window and a stated assumption about tagging coverage. Also note that FR-005 deliberately keeps last-minute reschedules on the phone, so this target is partly self-limiting.

**F12 — SC-004 is measured too late to matter and is the wrong kind of criterion.** A nightly comparison cannot prevent a practitioner preparing for a patient who moved that morning — the harm lands inside the detection window. It is also a correctness invariant rather than a success measure; invariants belong in the FRs (paired with FR-007's one-minute target), with the nightly job as a backstop audit.

**F13 — No criterion measures the harm the Overview leads with.** The Overview justifies the feature on unfilled slots (lost revenue), no-shows from patients who could not get through, and double-booked slots (a patient sent home). None of SC-001…SC-004 measures any of these. SC-004 covers only stale calendar entries, not two patients in one slot — which is exactly the failure FR-006 introduces. I would propose adding a double-booking criterion and a no-show or unfilled-slot criterion so the spec measures the outcome it was sold on.

**F14 — Measurement unit is inconsistent.** SC-001 and SC-003 are per-clinic and time-boxed from rollout; SC-002 and SC-004 state no population or window. Make all four per-clinic with an explicit window, or say which are global.

## Phase 5 — The two out-of-scope items, and the stop

The author scoped me to FRs and SCs. These two sit above that line, and I would raise them rather than let an FR ship broken. I would state plainly that they are outside what was asked and why I am raising them anyway.

**F15 — The spec contradicts itself about patient email, and FR-001 depends on the contradiction.**
Line 23 asserts "every booking carries a patient email address (the online booking form requires one), so the emailed link is sufficient… and no sign-in is needed." Line 79 says "front-desk staff also create appointments for patients who book by phone or walk in." Those bookings do not go through the online booking form. So FR-001's "every confirmed appointment" is unsatisfiable for phone and walk-in bookings — plausibly a large share at clinics whose front desks currently spend a third of every morning on the phone. This also weakens the no-sign-in premise, since the population without email is exactly the population the feature is meant to relieve pressure from.

**F16 — US-004 needs an FR for the staff-facing history view** (same as F7; noting it here because the fix lands in a story the PM signed off).

**Stop / decision point.** These findings touch the PM-signed-off Overview and stories. I would surface the question and not act unilaterally: *does the author want to reopen the stories with the PM, or contain the fix at the FR layer?*

- **If the author reopens with the PM:** I would propose specific edits — narrow line 23 to online bookings, and either add an eligibility rule (self-service available only where a patient email exists) or a story for capturing email at phone/walk-in booking.
- **If the author keeps the stories frozen:** I would propose an FR-layer containment — an explicit precondition FR stating self-service reschedule applies only to appointments with a patient email on file, plus a documented assumption and a risk entry noting FR-001's "every" is aspirational until email capture is universal.
- **Default I would proceed under, absent an answer:** report both findings, recommend the FR-layer containment as the smaller change, and leave the story text alone.

**Second stop — what I touch on disk.** I would not edit `spec.md`. It is the author's draft under review, and rewriting someone's spec in place is not what a review request asks for. I would write the review as a separate file, `review-FEAT-022.md`, in this workspace, and summarize in chat. If the author would rather have inline comments or a suggested-edit diff against `spec.md`, I would do that on request; absent a request, separate file is the default.

## Phase 6 — Assemble and deliver

Write `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-nqfyssio/ws/review-FEAT-022.md` containing:

1. One-paragraph summary: the spec is well-motivated and the stories are clear, but three blockers make it not-ready-to-build, and two of the four independent tests cannot be run against the requirements as written.
2. The traceability matrix from Phase 1 (US ↔ FR ↔ SC), since it is the evidence for most findings.
3. Findings F1–F16, each as: severity / what the text says / consequence / proposed fix, grouped Blockers → Major → Minor, FRs then SCs, with the out-of-scope pair last and labeled.
4. A short "what I did not review" note: wording, story structure, and the Overview, per the author's request — except where noted in F15.

No tests to run and no code in this workspace, so verification is the matrix and the independent-test walkthrough, not execution.

**No delegation.** This is a single document review well inside what I can do directly; spinning up a worker would add coordination cost and a second opinion I would have to re-verify line by line anyway.

**What I would report at the end:** three blockers (FR-006's contradiction, undefined slot-contention behavior, undefined "imminent"), the SC-001↔FR-008 dependency break that makes the headline metric uncomputable, the email premise contradiction with my recommended containment, and the open decision I am holding for the author — reopen the stories with the PM, or contain at the FR layer. I would say plainly that the draft is not ready to build yet and that the structural problems are exactly the kind a first spec produces — the stories and motivation are the hard part and those are solid.