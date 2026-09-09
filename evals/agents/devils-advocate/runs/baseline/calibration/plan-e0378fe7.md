FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-lv2tm2qf/ws/spec.md` (the only file in the workspace; I globbed `**/*` to confirm there is no glossary, PM story doc, or sibling spec that might define the undefined terms below)

---

# Action Plan — Adversarial review of `spec.md` (FEAT-022, Self-Service Rescheduling)

## Phase 1 — Frame the review and settle the scope question the author raised

**What I'd do.** Invoke `mochiko:review-specifications` to carry the pass and shape the report, and read the spec end to end twice: once for what it says, once for what it assumes.

Then I'd deal with the author's note before anything else, because it constrains the review and I would decline the constraint. The note asks me to stay inside Functional Requirements and Success Criteria and to skip wording. I'd hold the line on three specific grounds and say so in the report's opening, plainly and without lecturing a first-time author:

- The single worst defect in this document (FR-006) is only visible by reading it against US-001's third acceptance criterion. A review fenced to the FR list cannot see it.
- The load-bearing security decision in the whole feature — "the emailed link is sufficient to identify the patient and no sign-in is needed" — lives in the Overview, not the FR list. PM sign-off on user stories does not sign off on an authentication model.
- The Overview's claim that every booking carries an email is contradicted by a sentence inside the User Stories section (line 79, walk-ins and phone bookings). One of the two is wrong and it changes FR-001.

On wording I'd agree in part and refuse in part: I will not raise prose style, but "imminent" (FR-005) and "close to its start time" (US-002) are not tidying items — an unquantified threshold is a requirement defect, and I'd say that explicitly so the distinction lands rather than reading as me ignoring the request.

**Stop / decision point.** This is where a human ruling matters, so I'd name it rather than assume: *does the author (or the PM) accept a review that reaches outside FR and SC?* Branches: if **yes**, I deliver the full report below. If **no**, I deliver the FR/SC-scoped findings and attach the out-of-scope items as a separate, explicitly-labelled escalation to the PM and the spec lead, because I will not sit on a Critical finding to honor a scope preference. Either way the findings ship. **Default I proceed under:** deliver the full report, with the out-of-scope items marked as such so the author can see I noticed the boundary and crossed it deliberately.

**Delegation.** None, and deliberately so. This is one 126-line document and every judgment in it is interpretive — contradiction-hunting and completeness are exactly the work I cannot hand off without losing the finding. The one mechanical question (is there another workspace file defining the imminence threshold or link policy?) I answered myself with a single glob; had this been a repo with a glossary and sibling specs, I'd have spawned one throwaway `Explore` reader on Haiku with a brief of "list any file defining 'imminent', 'reschedule window', or link-expiry policy; return paths and quoted lines only," and on return I'd check that it quoted real lines with paths rather than summarizing, and I'd treat a "nothing found" as unproven absence and confirm it myself, since absence is what my finding would rest on.

## Phase 2 — Contradiction sweep (FR against FR, FR against stories, prose against prose)

**What I'd do.** Build the requirement-to-story trace by hand and read each pair for conflict. Expected products of this phase — these are the findings I'd write:

**C1 — Critical. FR-006 directly contradicts FR-002 and US-001, and cites the wrong source.**
FR-006 releases the patient's original slot the moment they *open* the page. FR-002 says the original appointment stays intact until confirmation. US-001's third criterion promises that a patient who leaves without confirming returns to an unchanged appointment. These cannot all hold. Concretely: a patient opens the link, their slot is released, another patient books it, the first patient closes the tab. Now either the first patient has an appointment in a slot someone else owns — the "slot booked twice, one patient sent home on arrival" harm the Overview names as the thing to avoid — or they have silently lost their appointment by clicking a link. FR-006 also cites US-003 as its source, but US-003 only says the old time reopens *after* the patient confirms. The requirement does not follow from its stated story.
*Stop point:* I'd ask the author whether FR-006 was meant to describe a temporary hold rather than a release. **Branch A** (it was a hold): the requirement still needs rewriting from scratch — hold duration, expiry, and what the other patient sees are all undefined, and it stays Critical as an ambiguity. **Branch B** (it's an error): delete FR-006 and add a confirm-time release requirement. **Default:** report it as a contradiction that is Critical under either reading, since the document currently states both.

**C4 — Critical. "Imminent" is never quantified, and the boundary has no evaluation point.**
FR-005 says "imminent," US-002 says "close to its start time," and only the independent test hints at one hour — a test is not a requirement. Three questions are unanswered: what is the threshold; whose clock and timezone decides; and is it checked at page load or at confirm? A patient who loads the page at two hours out and confirms at fifty-nine minutes out currently falls through the exact gap US-002 exists to close. Every implementer will pick a different answer.

**H2 — High. The "every booking has an email" assumption is contradicted inside this document.**
The Overview justifies having no sign-in on the grounds that the online booking form requires an email. Line 79 states front-desk staff also create appointments for phone and walk-in patients. Those bookings plausibly have no email, so FR-001's "every confirmed appointment" is either unsatisfiable or silently degrades for the patients most likely to need the front desk. Needs a stated decision: capture email at the desk, no link for those bookings, or something else.

## Phase 3 — Failure-path and edge-case probe (what happens when it doesn't go well)

**What I'd do.** Walk each FR asking what breaks it. The spec describes only the happy path; these are the paths it never describes:

**C2 — Critical. Nothing handles two patients wanting the same slot.**
FR-003 loads the slot list once at page open. FR-004 says on confirm the system MUST book that slot — with no branch for the slot no longer being available. Two patients holding the same stale list both confirm the same slot and the spec instructs the system to book it twice. This is the second of the two harms the Overview calls out, and no requirement guards against it. Needed: revalidation at confirm, a defined error state for the patient, and a rule for who wins.

**C3 — Critical. The link is the sole credential and has no stated properties.**
No requirement covers link entropy or guessability, expiry, single-use, revocation after the appointment passes or after staff cancel it, or what happens when the confirmation email is forwarded or sits in a shared family inbox. Anyone holding the URL can see a named patient's practitioner, appointment type, and schedule, and can move the appointment. For clinic appointment data this is a disclosure and integrity decision that must be made explicitly, not inherited by default. This is the finding that most needs to escape the requested review fence.

**H3 — High. No defined behavior when there are zero open slots in the next 30 days.**
The Overview says clinics run at capacity. An empty slot list is not an edge case here, it is a routine Tuesday. The spec says nothing about what that page shows.

**H4 — High. Rescheduling *into* the imminent window is unrestricted.**
US-002 protects the practitioner from a gap that cannot be filled. Nothing stops a patient moving their appointment *to* a slot twenty minutes from now, which creates the same unfillable churn from the other direction.

**H5 — High. FR-004 bundles three actions with no atomicity or failure semantics.**
Book the slot, update the appointment, send the email. If the email fails, is the move applied? If the slot books but the appointment update fails, what state is the patient in? The requirement is silent, and the patient's only evidence of success is that email.

**H6 — High. If FR-006 is corrected, no requirement releases the old slot at all.**
US-003 promises the old time is open again after confirmation. FR-006 is the only requirement that releases a slot, and it does so at the wrong moment. Fixing C1 by deleting it leaves US-003 uncovered — I'd flag this now so the fix doesn't open a new hole.

**H7 — High. Concurrent and stale states are undefined.** Staff cancel or move the appointment while the patient has the page open; the appointment start time passes while the page is open; the patient uses the link from an older confirmation email that names a time the appointment no longer has.

**M-series (Medium).** FR-003 never restricts the list to the same practitioner and appointment type — that constraint appears only in US-001 prose, while "different practitioner" is explicitly out of scope, so the boundary is unenforced by any requirement. Timezone ownership is unstated for display, for the 30-day window, and for imminence. The old confirmation email and its link are never invalidated, so a patient can hold two live links showing two different times. Nothing notifies a practitioner whose day just changed; only calendar state is required. The 30-day window is asserted with no relation to the clinic's own booking horizon. And the acknowledged open question about a reschedule limit interacts badly with the absent link expiry: unlimited reschedules through a never-expiring link is a churn path, so the open question is doing more work than the author thinks.

## Phase 4 — Success Criteria audit (is each one actually measurable as written?)

**What I'd do.** Test every criterion against its own stated measurement source and against the FRs that would have to produce the data.

**H1 — High. FR-008 drops the actor, which breaks US-004 *and* SC-001.**
US-004 requires the history to show whether a patient or a staff member made the change, and its independent test checks exactly that. FR-008 records previous time, new time, and timestamp — no actor. So US-004 is not covered by any requirement. Worse, SC-001 measures the share of reschedules made through the self-service link "from the appointment change log" — the log as specified cannot distinguish the two, so the primary success metric is uncomputable from the primary requirement. This cross-cut is the finding I'd lead the SC section with. Related and unaddressed: whether staff-side reschedules are in scope at all (FR-008 says "every reschedule" but nothing in this feature creates the staff path), and whether history is immutable.

**H8 — High. SC-002 measures with a beacon that does not exist.**
It cites "the page's existing timing beacon" on a page this feature is introducing. Also, SC-002 only counts patients who select a slot, so abandonment is invisible, and nothing at all measures the rate of "the slot you picked is gone" errors — the failure C2 predicts is unmeasured by design.

**M — Medium.** SC-004 is a nightly comparison, which is misaligned with FR-007's one-minute promise and lets a bad state stand for up to a day; it also checks only old-time ghosts, never double-bookings, so the harm the Overview ranks highest has no success criterion. SC-001 and SC-003 have no baseline and no stated denominator ("reschedules at a clinic" — including phone ones?). SC-003 relies on call-reason tags applied by the same front-desk staff whose workload is being measured, with no requirement that tagging coverage is sufficient for the number to mean anything.

## Phase 5 — Assemble and hand back

**What I'd write.** A single structured report to the author, in the shape `mochiko:review-specifications` defines: verdict, findings ordered by severity, each with the exact location, why it bites, and what decision would close it. Nothing else in this workspace changes — I do not edit `spec.md`. Fixing these is the author's job, and me rewriting FR-006 would rob them of the review.

**Verdict: critical-gaps.** Four Critical findings; a spec with a self-contradicting requirement, no double-booking control, an unspecified access model, and an unquantified threshold cannot go to design.

**Strengths I'd cite by name**, because a "needs work" verdict without evidence of what's right is not a useful review and this author is new: every FR carries an explicit source story, so the trace gaps I found were findable at all; every user story ships an independent test that is genuinely executable; the out-of-scope list gives a *reason* per exclusion rather than just a list; and the open question about reschedule limits was surfaced honestly instead of buried. The document's structure is what let this review be precise.

**What I'd ask the author for**, framed as product decisions with options rather than as defects: (1) hold-the-slot or keep-the-slot while the patient chooses, and for how long; (2) the imminence threshold in hours, and whether it is checked at open or at confirm; (3) what a leaked or forwarded link should be able to do, and when a link should stop working; (4) what the patient sees when their chosen slot was taken a second earlier; (5) whether walk-in patients get a link.

**Escalation I'd attach.** The link-access model (C3) and the walk-in email contradiction (H2) go to the PM as well as the author, since both touch decisions the PM already believes are settled.