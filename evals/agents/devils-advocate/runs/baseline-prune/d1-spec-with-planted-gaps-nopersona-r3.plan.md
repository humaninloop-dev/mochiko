FILES-READ: `spec.md` (the only file in the workspace; `**/*` returned nothing else — no code, no related specs, no repo)

---

# Action plan — review of `spec.md` (FEAT-022, Self-Service Rescheduling)

## Phase 0 — Scope decision, made up front

The author asks me to (a) stay inside Functional Requirements and Success Criteria and (b) go easy on wording.

- **(b) I'd honor fully.** No prose, tone, or formatting nits. Not one. The spec reads well.
- **(a) I'd honor as the center of gravity, with two named exceptions.** Three defects in the FR/SC sections have their root cause in the Overview or User Stories (the "every booking has an email" assumption vs. front-desk bookings; the missing threshold for "imminent"; the SC-002 "existing timing beacon"). I can't report the FR defect honestly without naming its cause. I'd put those in a clearly-marked short appendix — "root causes sitting outside the fence you drew" — and not touch anything else upstream. I would **not** re-open the PM-signed-off user stories as stories.

**Stop point (non-blocking):** I'd note in the delivery that the appendix crosses the requested boundary and ask whether the author wants it carried to the PM or dropped. Branch: if they say drop it → I still leave the FR-level finding in place, because the FR is defective on its own terms; only the upstream framing goes. If they say carry it → I'd offer to draft the two-sentence note to the PM. **Default while unanswered: include the appendix, clearly fenced.**

I would **not** soften or omit findings on the grounds that this is the author's first spec. Severity labels stay accurate; the delivery is constructive but the substance is unabridged.

## Phase 1 — Build a traceability matrix (read: `spec.md` lines 26–112)

Construct, on scratch, a two-way table: every FR → its cited source story and acceptance criterion; every acceptance criterion → the FR(s) that would make it pass. Then run each story's stated "Independent test" against the FRs alone and ask: could a team implement only these eight FRs and pass that test? This is the mechanism that surfaces most of the findings below.

## Phase 2 — Contradiction pass on the FRs

Check FRs pairwise for conflict and against the Overview's two stated failure modes (unfilled slot = lost revenue; double-booked slot = patient sent home).

## Phase 3 — Testability pass on the FRs

Flag every FR containing a term an engineer or QA cannot resolve without asking someone: "imminent," "open slots," "confirmed appointment," "the slot list."

## Phase 4 — Success Criteria pass

For each SC ask four questions: is it an outcome or a restatement of the mechanism; does the measuring instrument it names actually exist today; is there a baseline to measure against; and does the set as a whole cover the harms the Overview names.

## Phase 5 — Assemble the findings register

This is the deliverable. Below is the register as I'd expect it to come out, ordered by severity.

### Blockers

**F1 — FR-006 contradicts FR-002 and US-001, and is traced to a story that doesn't say it.** FR-006 releases the original slot when the page *opens*. FR-002 exists to keep the appointment intact until the patient *confirms*, and US-001's third criterion promises an abandoned reschedule changes nothing. FR-006 cites US-003, but US-003 says the old time reopens "when the change is saved" — not on page open. As written, a patient who opens the link and closes the tab can have their own slot booked out from under them, producing exactly the "sent home on arrival" outcome the Overview names as the thing to avoid. Recommendation: delete FR-006 and fold the release into FR-004 (on confirmation: book new slot, update appointment, release old slot, send email). If someone genuinely wants the old slot visible during selection, that's a *hold* mechanism and needs its own requirement plus an expiry — but I'd argue against it for a v1.

**F2 — Nothing handles the slot being taken between page load and confirm.** FR-003 loads the slot list once, on open. FR-004 then books "that slot" with no requirement that the booking is atomic or that a now-taken slot is rejected. Two patients on two stale lists both confirm the same 10:40; the Overview says one of them is sent home. Needs an FR: confirmation must fail cleanly if the slot is no longer open, tell the patient, and re-present a current list. Currently this failure has no requirement, no acceptance criterion, and no success criterion — see F8.

**F3 — FR-005 has no threshold, so it cannot be built or tested.** "Imminent" is undefined; US-002 says "close to its start time"; the independent test happens to use one hour. Three different readings. Needs a stated number, and a decision on whether it's global or per-clinic (clinics with different fill dynamics will want different values — I'd recommend per-clinic with a default). Root cause is upstream in US-002, but the FR is the thing that's unbuildable.

**F4 — FR-008 drops the actor, and nothing makes the history visible.** US-004's whole point is answering a patient who says "I never moved it," and its independent test checks that patient-made and staff-made entries show the right actor. FR-008 records previous time, new time, and timestamp — no actor. Separately, no FR requires the history to be *shown* to front-desk staff; FR-008 only requires recording. US-004 cannot pass on the FRs as written. This also breaks SC-001, which proposes measuring self-service share "from the appointment change log" — a log with no actor field can't answer that question.

### Major

**F5 — FR-003 drops the same-practitioner and same-appointment-type filters.** US-001 requires slots "for the same practitioner and the same appointment type"; the Out of Scope section explicitly reserves practitioner changes for the front desk. FR-003 says only "the open slots for the next 30 days." Implemented literally, the page violates a documented out-of-scope boundary. Add both filters to FR-003.

**F6 — No FR governs the link as an access control boundary.** The spec's identification model is "the emailed link is sufficient, no sign-in." FR-001 requires generating a link and says nothing about it being unguessable, scoped to one appointment, expiring, or being invalidated after the appointment passes. A forwarded confirmation email, or a sequential ID, exposes a named patient's practitioner and appointment time — health information — and lets a third party move the booking. I'd flag this as the highest-consequence gap after F1 even though it reads as quiet. I would *not* argue the no-sign-in decision itself (PM-signed-off); I'd argue that the decision creates obligations the FRs don't yet carry. Needs at least: unguessable token, bound to one appointment, invalid once the appointment is past or cancelled.

**F7 — No FR covers the state of the appointment when the link is opened.** Already cancelled, already past, already rescheduled, group appointment (out of scope but still emailable), or an appointment created before the feature shipped. FR-001 says "every confirmed appointment" gets a link; nothing says what the page does when the appointment is no longer in a reschedulable state.

**F8 — SC-002 names an instrument that does not exist.** "The page's existing timing beacon" — the reschedule page is new in this feature, so there is no existing beacon on it. Either the beacon exists on some other page and needs porting (a requirement), or the criterion is unmeasurable on day one.

**F9 — SC-002 measures the wrong population.** It times only patients who already selected a slot, so it's blind to the failure that matters: patients who open the link and give up. A page could score a perfect 90-second median while 80% of patients bounce to the phone. Recommendation: replace or supplement with completion rate — of patients who open the link and are offered slots, what share confirm a new time — and keep the timing as a secondary diagnostic.

**F10 — The Success Criteria don't cover either harm the Overview names.** The Overview justifies the feature on two failure modes and one motivation: unfilled slots, double-booked slots, and no-shows from patients who couldn't get through by phone. No SC measures double-booking (F2's failure), and none measures no-show rate. SC-004 covers a narrower calendar-consistency invariant only. I'd propose adding a no-show criterion (this is arguably the headline outcome the feature is for) and a double-booking criterion at zero.

### Moderate

**F11 — SC-004's detector can't see the failure it's aimed at.** A nightly comparison of calendar entries against the appointment table misses any wrong state that resolves within the day, and FR-007 explicitly allows up to a minute of lag; more importantly, if the calendar and the appointment table are updated wrongly *together*, the comparison finds nothing — it only catches divergence, not incorrectness. Also, "zero" with a nightly detector is a correctness invariant, not a success metric. Recommendation: keep it as an alerting invariant, and let the practitioner-facing outcome be measured by something patient-visible.

**F12 — SC-001 and SC-003 have no baseline and inconsistent windows.** SC-001 (60% within 60 days of a clinic going live) needs a per-clinic go-live date recorded, and SC-003 (30% reduction in 90 days) needs pre-rollout phone time captured *before* the first clinic launches — if that measurement isn't taken up front it can't be reconstructed later. The 60/90-day windows differ with no stated reason; I'd align them or say why.

**F13 — SC-003 depends on call-reason tagging that the spec never establishes.** The criterion assumes clinics tag calls with a reason and do so consistently enough for a 30% movement to be legible. That's an unverified dependency on staff behavior at the measurement layer. Worth confirming the tags exist and are used before committing to the number.

**F14 — FR-004's new confirmation email: does it carry another reschedule link?** Unspecified, and it interacts with the Open Question about capping reschedules. If yes, the Open Question becomes load-bearing; if no, the second reschedule silently requires a phone call, which no story describes.

### Appendix (outside the requested fence, flagged as such)

**A1 — The "every booking has an email" premise doesn't hold for the bookings line 79 describes.** The Overview grounds the whole no-sign-in model on the online booking form requiring an email. Line 79 then notes front-desk staff also create appointments for phone and walk-in patients — who never touch that form. FR-001 ("every confirmed appointment") is unsatisfiable for those, and the sentence at line 79 is orphaned: no story or FR consumes it. Either those bookings capture an email, or the spec needs to say self-service is unavailable for them.

**A2 —** the FR-005 threshold (F3) and **A3 —** the SC-002 beacon (F8) originate upstream, per the phase-0 note.

## Phase 6 — What I'd say about the spec's strengths

Not padding — the author is new and needs to know which instincts to keep. Concretely: every FR carries a *Source* trace (that's what let me find F1's mis-sourcing at all); the Out of Scope section gives reasons rather than bare exclusions; the Open Question is stated with its current default; SC-001, SC-003 and the acceptance criteria are outcome-shaped rather than describing the implementation. That structure is what made this spec reviewable.

## Phase 7 — Delivery

- **Default: return the findings in the conversation**, ordered as above, each as *defect → why it matters → concrete suggested replacement text*. No file written unasked.
- **Offer:** I'd write `spec-review.md` in this workspace, or supply drafted replacement text for FR-003/004/005/006/008 and SC-001/002/004 as a patch to `spec.md`, if the author wants either. I'd wait to be asked before touching `spec.md` — it's the author's document.
- **No delegation.** One 126-line document, single reviewer; spawning workers here would add coordination cost and a summarization layer between me and the text for no gain.
- **Closing report:** 14 findings — 4 blockers, 6 major, 4 moderate — plus a 3-item appendix that crosses the requested scope, with the reason it crosses. I'd state plainly that F1 and F2 would each cause a patient to be turned away at the door if built as specified, that US-004 cannot pass its own independent test on the current FRs, and that I skipped wording entirely as asked.