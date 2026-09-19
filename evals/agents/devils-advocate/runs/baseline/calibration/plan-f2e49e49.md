FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-74knu_s0/ws/spec.md`

*(The workspace contains exactly one file. I globbed for siblings — no glossary, no prior specs, no PM story source, no design docs. Everything below is planned against this file alone, and that isolation is itself something I would flag.)*

---

# Action Plan — Gap Review of FEAT-022 Self-Service Rescheduling

## Phase 1 — Settle the scope question before reviewing anything

**What I'd do.** The author's note asks me to confine the review to Functional Requirements and Success Criteria, on the grounds that the PM signed off the user stories, and to go easy on wording. I would split that request in three and answer each differently:

- **Wording/style nits — accepted.** I drop them entirely. The prose is clear; there is nothing here I would spend a finding on.
- **Weighting toward FRs and Success Criteria — accepted.** Those two sections carry the bulk of my findings anyway.
- **A hard fence excluding the Overview, Stories, Out of Scope, and Open Questions — refused.** Not out of stubbornness: the most severe defects in this draft *cannot be stated* without crossing that line. FR-006 contradicts a story acceptance criterion, so the finding is meaningless if I may only look at FR-006. The Overview asserts that an emailed link is sufficient identity with no sign-in — that is the single largest unexamined assumption in the document and it has no requirement behind it. A PM sign-off on user stories is a sign-off that the stories describe the right product; it is not a finding that the stories are internally consistent with the requirements derived from them, and it is not a security review.

**How I'd frame the refusal to the author.** Not as a rebuke. I would say plainly: I am staying out of your wording, I am concentrating on the two sections you asked for, and I am crossing your fence in exactly four places, each of which I will label so you can see why the finding could not be confined.

**Stop point (human decision).** Before delivery I would surface to the review lead: *the author restricted scope; I am overriding in four places.* What gets confirmed is whether the override stands, and whether the PM should be looped in on the story-level contradiction (FR-006 vs US-001) since it may change a signed-off acceptance criterion.
- If the ruling is *override stands* → deliver as planned below.
- If the ruling is *respect the fence* → I still deliver the cross-fence findings, but reclassified as "escalated to lead, not returned to author," because I will not let a scoping instruction convert a Critical into a silence. The author gets the in-scope set; the lead gets the full set.
- **My default if no ruling arrives:** override stands, findings delivered to the author with the fence-crossing marked.

I would also note to the lead that this is the author's first full spec. That changes my *tone*, not one severity.

## Phase 2 — Build the coverage map (story ↔ FR ↔ success criterion)

**What I'd do.** Before hunting, trace each of the eight FRs back to its cited source story and each story acceptance criterion forward to a covering FR, and check each success criterion against the requirement that would make it measurable. This is what surfaces under-coverage and mis-derivation, and it is the part I would not hand to anyone else — absence is the finding, so a partial enumeration is worse than none.

**Delegation decision.** None. Explicitly: this is a 126-line document already in my context, and every judgment in it is interpretive or absence-sensitive. Spawning a cheap reader here would buy nothing and risk a partial enumeration on exactly the axis where completeness matters. The one place a cheap lookup *would* have earned its keep — checking whether "imminent," the reschedule window, or the timing beacon are defined in a sibling glossary or platform spec — is moot, because no sibling files exist. I would report that absence rather than assume the definitions live somewhere unseen.

**Expected output of this phase (not delivered, used as input):** a table showing US-001 AC1 only half-covered by FR-003, US-002's threshold uncovered, US-003's post-confirm slot release uncovered, US-004's actor and history-display uncovered, and FR-006 traced to a source that does not say what it says.

## Phase 3 — Draft the findings

This is the substance. Below is the inventory I expect to return, with the severity I would assign now. Severity is my proposal; I would not move any of it to make the delivery friendlier.

### Critical

**C1 — FR-006 releases the slot at the wrong moment and contradicts FR-002 and US-001.**
FR-006 requires the original slot to be released *as soon as the patient opens the reschedule page*. FR-002 requires the original appointment to stay intact until confirmation, and US-001's third acceptance criterion promises that a patient who leaves without confirming returns to an unchanged appointment. Under FR-006, a patient who opens the link and hesitates can have their slot taken by someone else; they then abandon and are left with an appointment whose slot is double-booked, or with nothing. This is precisely the "sent home on arrival" outcome the Overview names as the worst case. FR-006 also cites US-003 as its source, but US-003 says the old time opens again *when the change is saved* — the requirement does not follow from the story it claims.
*Framed as a decision for the author:* (a) release the old slot only on confirmation, matching US-003 literally; (b) place a time-boxed soft hold on the old slot while the page is open, releasing on abandon after N minutes — which needs N, and needs a requirement for what the patient sees when the hold lapses mid-session; (c) something else, but not the current text.

**C2 — No requirement addresses the slot being taken between page load and confirmation.**
FR-003 loads the slot list *once* when the page opens. FR-004 books on confirm. Nothing in between reserves anything, and nothing specifies the behavior when the chosen slot is gone at confirm time. Two patients on the same slot is the failure the Overview calls out as costly, and the spec's own design makes it likely rather than exotic. Missing: whether selection creates a hold, how the list refreshes, what the patient is shown on collision, and whether the original appointment survives a failed confirm. Note this compounds C1 — under FR-006 as written, a collision can leave the patient with no appointment at all.

**C3 — The reschedule link has no security requirements whatsoever.**
The Overview asserts that because every booking has an email address, the link identifies the patient and no sign-in is needed. That is an assumption presented as a conclusion, and no FR backs it. Unspecified: whether the link is unguessable, whether it expires, whether it is single-use or survives every reschedule forever, what happens when a forwarded or leaked link is opened, whether it still works after the appointment has passed or after the clinic cancels or moves the appointment. This is patient health information behind a URL with no stated protection. I would flag that a security reviewer should see this spec before build, independent of my review.

**C4 — FR-003 drops the practitioner and appointment-type constraints, and that is what enforces the Out-of-Scope boundary.**
US-001 requires slots for *the same practitioner and the same appointment type*. FR-003 requires only "the open slots for the next 30 days." Out of Scope explicitly excludes rescheduling to a different practitioner and says the clinic wants that decision kept with the front desk — but with FR-003 as written, no requirement prevents it. Also unspecified: slot duration matching the appointment type, and whether the new slot must be in the future and inside the window at confirm time.

### High

**H1 — "Imminent" is never quantified, and the evaluation moment is unspecified.**
FR-005 and US-002 both use unquantified language; the only number in the document is "one hour" inside an independent test, which is not a requirement. Missing: the threshold, whether it is configurable per clinic, and — the sharper gap — whether it is checked at page load only or again at confirmation. A patient who opens the page just outside the cutoff and confirms just inside it currently gets a self-service last-minute change, which is the exact outcome US-002 exists to prevent.

**H2 — FR-008 does not record the actor, but US-004 and SC-001 both depend on it.**
FR-008 requires previous time, new time, and timestamp. US-004 requires knowing whether the patient or a staff member made the change, and its independent test verifies exactly that. SC-001 proposes to measure self-service share "from the appointment change log," which is impossible without the actor. One missing field breaks a story, its test, and a success criterion.

**H3 — No requirement surfaces the history to staff.**
US-004 is a display story — staff open an appointment and see the list. FR-008 covers recording only. Nothing covers showing it, who may see it, or how long it is kept.

**H4 — No requirement releases the old slot on confirmation.**
Because FR-006 misplaced this to page-open, the behavior US-003 actually asks for — old slot bookable *after* the change is saved — is uncovered. If C1 is resolved by fixing FR-006's timing this closes with it; if it is resolved with holds, it needs its own requirement.

**H5 — FR-004 has no failure or partial-completion behavior.**
It requires booking the slot, updating the time, and sending an email as one sentence. Unspecified: what happens if the booking succeeds and the update fails, or the update succeeds and the email never sends. Whether the patient is told. Whether anything rolls back. The worst branch leaves a patient believing nothing happened while the calendar says otherwise.

**H6 — Phone and walk-in appointments may have no email, contradicting the Overview's premise.**
The Overview justifies the whole no-sign-in design with "every booking carries a patient email address (the online booking form requires one)." The stray line after US-004 says front-desk staff also create appointments for phone and walk-in patients — who did not use the online form. FR-001 requires a link for *every confirmed appointment*, which is then unsatisfiable for that population. Either a population is silently excluded from the feature (which drags on SC-001 and SC-003 and is not stated in Out of Scope), or the premise is wrong. The author should say which.

**H7 — SC-001's target may be unreachable by the spec's own design, and its denominator is undefined.**
60% of reschedules through self-service, while US-002 deliberately routes last-minute changes to the phone and H6 may exclude phone/walk-in patients entirely. "Reschedules at a clinic" is not defined — does the denominator include the phone reschedules the spec intends to keep? No baseline is given for how the split looks today. The measurement instrument also depends on H2 being fixed.

**H8 — No success criterion covers the failure the Overview says matters most.**
SC-004 checks that no appointment shows at its *old* time. Nothing measures double-booking — two patients in one slot — which the Overview names as the outcome that sends a patient home, and which C1 and C2 make plausible. Nothing measures the abandoned-reschedule guarantee from FR-002 either.

### Medium

**M1 — SC-002's start point is ambiguous and its instrument is assumed.** "Opens the page and selects a slot completes within 90 seconds" — measured from page open or from slot selection? These differ by however long a patient spends reading. "The page's existing timing beacon" presumes an instrument on a page that does not exist yet; if it must be built, that is an unstated requirement.

**M2 — SC-003 has no baseline and a confound.** A 30% fall in phone time with no stated starting figure is unfalsifiable, and it depends on call-reason tags being applied consistently by the same staff whose workload is being measured. US-002 also deliberately sends some calls back to the phone.

**M3 — SC-004's detection window is nightly.** A practitioner can hold a wrong calendar for up to a day. It also compares the calendar against the appointment table, so a fault that corrupts both passes silently. Neither is fatal, but the criterion promises less assurance than its "zero" implies.

**M4 — Zero-availability path unspecified.** No requirement says what the patient sees when no open slot exists in 30 days. Given the Overview's "clinics run at capacity," this is a common case, not an edge one, and the natural fallback (show the phone number) already exists in FR-005 and could be reused.

**M5 — Time zones and DST are untouched.** "Next 30 days," slot display, and the imminence cutoff all assume a single clock. A patient rescheduling from a different zone, or across a DST boundary, has no stated behavior.

**M6 — Concurrent and repeat access by the same patient.** Two tabs, a second confirm on a stale page, or the front desk moving the appointment while the patient's page is open — all unspecified.

**M7 — The open reschedule-count question interacts with C3.** The author correctly flags that no limit is set. I would note that with no limit *and* no link expiry, the link is a permanent unbounded rescheduling capability, and the two questions should be resolved together rather than separately.

### Explicitly not raised
Wording, structure, and phrasing — per the author's request, and I found nothing there worth a finding anyway.

## Phase 4 — Adversarial self-check before I write anything down

**What I'd do.** Re-read FR-001 through FR-008 and SC-001 through SC-004 once more, cold, against the question "which of these looks fine?" — because that instinct is where I miss things. Specific re-probes I would force: FR-001 (looks trivially fine — H6 came out of exactly this probe), FR-002 (looks fine in isolation; only breaks against FR-006), FR-007's one-minute number (is one minute defensible for a practitioner about to see a patient? I would raise it only as a question, not a finding, since it is at least quantified). I would also verify I have not invented a requirement out of my own preference: every item above traces to something the document itself claims, promises, or measures.

**I would also check my severities for convenience-drift** — this is a first spec and the pull to soften C1 and C3 is real. Both stay Critical.

## Phase 5 — Assemble the report

**What I'd write, and where.** A single review document — path `review-FEAT-022.md` alongside the spec, unless the team has a review location I would ask about first. Contents, in order:

1. **Verdict: critical-gaps.** Four Critical findings; this draft is not ready to build against. Stated once, up front, with no hedging.
2. **What is genuinely strong, cited specifically** — not filler. The story-to-FR source attributions on every requirement (which is what made C1's mis-derivation *findable*); every story carrying an independent test; the Out of Scope section giving reasons rather than just exclusions; the Open Questions section admitting an undecided limit rather than papering over it; FR-007 being the one requirement with a real number in it. This author built a traceable document, and traceability is why the review could be this specific.
3. **Scope note** — the four fence-crossings, each labeled with why it could not be confined.
4. **Findings**, Critical → High → Medium, each with: the clause, what is missing or contradictory, the concrete failure it produces, and — for the ones that are genuinely product decisions (C1, H1, H6, M4, M7) — two or three framed options so the author is choosing, not guessing at what I meant.
5. **Clarifying questions for the PM**, separated out: the imminence threshold, whether phone/walk-in patients are in or out, the reschedule-count limit, and the SC-001 denominator and baseline.
6. **Recommended next step:** revise the FR section, then a security review of the link model before build.

**What I would refuse to write.** Replacement requirement text. I will show the author what breaks and what the choices are; deciding FR-006's new wording is the author's job, and if I write it I lose the ability to review it.

## Phase 6 — Delivery and the branches after it

**Stop point.** Hand the report to the author and lead. What gets confirmed there: (a) the four Criticals are accepted as Critical, (b) the fence override stands, (c) whether the PM re-opens US-001's third acceptance criterion given C1.
- If C1 is accepted and the fix is "release on confirmation" → C1 and H4 close together; I re-review the FR section only.
- If the fix is "soft hold with a timeout" → new requirements appear and I re-review C1, C2, H4, and M6 as a cluster, since holds change the concurrency story wholesale.
- If any Critical is disputed → I hold the severity, put my reasoning in writing, and let the lead and the user rule. I do not gate this alone, and I do not withdraw a Critical because it is contested.
- If the author asks me to draft the fixed requirements → I decline and offer to review their draft same-day instead.

**My default if nothing comes back:** the verdict stands as critical-gaps and the spec is not cleared for design.

## Phase 7 — What I would report at the end

A short close-out: verdict critical-gaps; 4 Critical, 8 High, 7 Medium; the four fence-crossings and why; the two things that need someone other than me (a security look at the link model, a PM ruling on the imminence threshold and the phone/walk-in population); and the note that this review saw only `spec.md` — no glossary, no platform spec, no PM story source — so any finding that turns on a term possibly defined elsewhere ("imminent," "the existing timing beacon") should be re-checked against those documents if they exist.