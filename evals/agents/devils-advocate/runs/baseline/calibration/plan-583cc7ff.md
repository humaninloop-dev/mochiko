FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-mrsc74vy/ws/spec.md`

---

# Action Plan — Gap Review of FEAT-022 Self-Service Rescheduling

## Phase 0 — Load the review procedure before forming any verdict

**Do:** Invoke the specification-review skill and work from it, not from memory. It owns the gap taxonomy, the severity rubric, and the required output shape; every severity I name below is provisional until I've conformed it to that rubric. If the skill's rubric disagrees with my draft severity, the rubric wins.

**Read:** the skill's criteria and output format.

**Write:** nothing yet.

**Refuse/flag:** I would not skip straight to writing findings from my own head, and I would not shape severity to the rubric's *lenient* end because this is the author's first spec.

## Phase 1 — Settle the author's scope fence before reviewing, and say so out loud

**Do:** The note at the top asks me to review only Functional Requirements and Success Criteria, to leave the user stories alone because the PM signed off, and to go easy on wording. I split that request into three parts and answer each explicitly in the report's opening:

- **Wording — accepted.** I will not file prose, tone, or phrasing findings. That costs the review nothing.
- **FR/SC focus — accepted as the *place findings land*, rejected as a *place findings may come from*.** Several of the most severe defects in FR and SC are only visible by reading them against the Overview and the stories. I will trace them, but file them as FR/SC findings with the upstream text quoted as evidence.
- **"PM signed off on the stories" — rejected as a suppression reason.** Sign-off means the stories are agreed; it does not make a requirement that *contradicts its own cited story* correct. Where an FR and its source story disagree, I report the disagreement and let the author and PM decide which side moves. That is a routing decision, not mine to make — but hiding it is not an option.

**Stop point:** If the author or lead insists the fence excludes cross-section findings, I stop and confirm. Branch A (fence relaxed — my default and my recommendation): file everything, as planned below. Branch B (fence held): I still file every finding, but split the report into "in-fence" and "held at your instruction, unreviewed by request" sections, and I attach my verdict *only* to the in-fence part while stating plainly that the verdict is not trustworthy under that restriction — because at least two Critical items originate outside it. I would not issue a clean-looking verdict over a fence I know hides a Critical. **Default I proceed under: Branch A.**

## Phase 2 — Build the traceability grid (mechanical pass)

**Do:** By hand, build a two-way map: every acceptance criterion in US-001..US-004 → the FR that implements it; every FR → the story it cites as its source. Then check each FR's cited source actually says what the FR says. Also map every FR and every named harm in the Overview → a success criterion that would detect its failure.

**Read:** spec.md only — lines 26–126.

**Expected to show (and this is what I already see on the first pass):**
- US-004 requires the *actor* of each change; FR-008 records previous time, new time, and timestamp only — no actor. Its own independent test checks the actor.
- FR-006 cites US-003, but US-003's criterion releases the old time "when the change is saved," not when the page opens.
- No FR covers a front-desk-initiated reschedule, yet US-004's independent test requires one.
- The Overview names double-booking as the headline harm; no FR prevents it and no SC measures it.

**Delegation:** none. This workspace holds one 126-line file which I have already read in full, and this pass is completeness-sensitive — an enumeration where a missed row *is* the finding. I do it myself. The one case where I would spawn a cheap throwaway reader on the small model: if the workspace contained sibling specs, a glossary, or a prior appointment-domain spec, I'd send one to fetch any existing definition of an "imminent appointment" cutoff or an established reschedule-link policy, brief it to quote the defining lines with file and line numbers and to report explicitly if it finds none, and on return I'd re-open the cited lines myself before relying on an absence. No such files exist here, so no spawn.

## Phase 3 — Adversarial pass on the Functional Requirements

**Do:** Walk FR-001 through FR-008 asking, for each: what happens when it fails, when two users hit it at once, when its input is absent, and when time passes between its steps. Draft findings. Current draft (severities provisional pending Phase 0):

**Critical**

1. **FR-002 and FR-006 directly contradict each other, and FR-006 loses the patient their appointment.** FR-002 keeps the original appointment intact until confirmation; FR-006 releases the original *slot* the moment the page opens. A patient who opens the link out of curiosity and closes the tab can have their slot taken by someone else, leaving them holding an appointment whose slot is double-booked or gone. This also contradicts US-001's third criterion ("the original appointment is unchanged") and its cited source US-003. Framed for product: *should merely looking cost a patient their booking?*
2. **No requirement covers the slot being taken between page load and confirmation.** FR-003 loads the list once, at page open; FR-004 says the system MUST book the selected slot, with no branch for "that slot is no longer open." Nothing holds the slot, nothing re-validates it, nothing tells the patient. This is exactly the harm the Overview names — "a slot booked twice means one patient is sent home on arrival."
3. **The reschedule link has no stated security properties, and it carries patient health information.** FR-001 requires generating and emailing a link; nothing states that it is unguessable, single-use, expiring, scoped to one appointment, revoked after the appointment passes or is cancelled, or rate-limited. The page exposes the patient's name, practitioner, appointment type, and clinic. The Overview asserts no sign-in is needed; that is an *assumption presented as a settled fact*, and it is the assumption that decides how much a leaked or forwarded confirmation email costs.

**High**

4. **The Overview's premise that every booking carries a patient email is contradicted inside this spec.** Line 23 grounds it in "the online booking form requires one"; line 79 says front-desk staff also create appointments for phone and walk-in patients, who never touch that form. FR-001 nonetheless requires a link for *every* confirmed appointment. Undefined: what FR-001 does when there is no email address.
5. **FR-008 omits the actor** — traceability break against US-004 and its own test (Phase 2).
6. **"Imminent" is never quantified, and its anchor is never named.** US-002 says "close to its start time," FR-005 says "imminent," the independent test uses one hour. Three phrasings, no threshold. Separately: does the cutoff apply to the current appointment's start, the target slot's start, or both? As written a patient could move a next-week appointment *into* a slot twenty minutes from now — which recreates precisely the unfillable-gap harm US-002 exists to prevent.
7. **No requirement defines a staff-side reschedule**, though US-004's test depends on one and FR-008 must distinguish it.
8. **Nothing defines what happens when the link is opened against an appointment that has already passed, been cancelled, or been moved by the front desk**, nor what happens when the front desk edits the appointment while the patient has the page open.

**Medium**

9. **FR-004 bundles booking, updating, and emailing with no failure semantics.** If the confirmation email fails to send, is the reschedule complete? The patient's only record of the new time is that email.
10. **Empty-slot state undefined.** FR-003 lists open slots over 30 days; nothing says what the page shows when there are none, which for a clinic that "runs at capacity" is a common case, not an edge case.
11. **No timezone or daylight-saving rule** for how slots are displayed or interpreted.
12. **FR-003 covers the slot list only**; US-001 also requires the page to show the current appointment.
13. **The open question about a reschedule limit is resolved by default to "unbounded"** without the consequence being stated — with FR-006 as written, repeated open-and-abandon cycles churn the clinic's availability.

## Phase 4 — Adversarial pass on the Success Criteria

**Do:** For each SC ask: is the measurement instrument real and stated, is there a baseline, could the number be hit by a broken feature, could a working feature miss it, and does any SC detect the Overview's named harms?

Draft findings:

- **Critical — nothing measures double-booking.** SC-004 checks only that appointments don't *appear at the old time*, comparing calendar entries against the appointment table nightly. Two patients booked into the same new slot are consistent in both places and pass this check. The headline harm is unmeasured.
- **High — SC-004 cannot detect what FR-007 permits.** FR-007 allows up to a minute of calendar staleness; a nightly comparison cannot see a one-minute window, so SC-004's "zero" is compatible with the requirement being violated on every reschedule. The measurement and the requirement don't meet.
- **High — SC-001's denominator is contaminated by reschedules FR-005 forbids.** Last-minute reschedules must go by phone; they land in the same change log. If most reschedules are last-minute, 60% is unreachable no matter how good the feature is — and no baseline for the current mix is given.
- **Medium — SC-002 measures only patients who reach the selection step**, so a slot picker so confusing that most patients abandon it scores perfectly. It also asserts "the page's existing timing beacon" exists; that is an unverified dependency.
- **Medium — SC-003 relies on call-reason tags applied by the same front-desk staff whose time it measures**, with no stated baseline and no stated tagging discipline.
- **Medium — no criterion covers the patient-side failure the FR gaps create:** patients who arrive at the page and cannot complete, or who lose their original slot.

## Phase 5 — Self-challenge before filing

**Do:** Re-read the sections that felt fine on first pass — the Out of Scope list and the Open Questions — specifically looking for what their *exclusions* silently assume. Check whether "no cancellation from the link" plus "unlimited reschedules" gives patients a de-facto cancellation path by moving an appointment repeatedly to the far edge of the 30-day window, and whether the group-appointment exclusion is enforced anywhere in the FRs or merely declared in prose. Add findings if so; note the check as performed if not. I would not file this review with only the items that were obvious on first read.

## Phase 6 — Write and hand back

**Write:** the review as my response to the author (no file writes are available in this run; if they were, I'd put it alongside the spec as a separate review document and would still not touch `spec.md`). Contents, in the skill's required shape:

- The scope-fence answer from Phase 1, stated first and plainly.
- Findings grouped by severity, each with: the exact requirement or criterion ID, the quoted line, the concrete failure it produces, and the user-visible consequence.
- Clarifying questions with options for the items that are product rulings, not defects I may settle — at minimum: (a) does opening the link release the slot, hold it, or change nothing until confirm; (b) what is the imminent cutoff in hours, and does it apply to the old time, the new time, or both; (c) what does a leaked or forwarded confirmation email get access to, and for how long.
- **Verdict: critical-gaps** (provisional on Phase 0's rubric). Justification names the specific blockers — FR-002/FR-006 contradiction, unhandled slot contention, unspecified link security — rather than resting on issue count.
- Genuine strengths cited with evidence, because the verdict is only credible if it discriminates: every FR carries a source-story back-reference; every story carries an executable independent test; Out of Scope gives a *reason* per exclusion rather than a bare list; SC-001, -003 and -004 each name their measurement instrument. Those are habits many third specs lack.

**Refuse:** I will not rewrite any requirement, supply the missing threshold value, or draft the replacement FR-006. I surface the gap and the options; the author writes the spec.

**Stop point:** After delivery, if the author disputes a severity, I re-examine the evidence but do not lower a Critical to keep the peace; if they show me a fact that changes the failure mode, the severity changes with the fact and I say which fact moved it.