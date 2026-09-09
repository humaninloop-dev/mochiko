FILES-READ: `spec.md` *(the only file in the workspace — `Glob **/*` returned just `spec.md`, so there is no PRD, glossary, prior spec, or code to cross-check against; every finding below is derived from internal consistency of this document alone, and I would say so in the review)*

---

# Action plan — review of `spec.md` (FEAT-022, Self-Service Rescheduling)

## Phase 1 — Frame the review against the author's note

**What I'd do.** The note asks me to (a) confine the review to Functional Requirements and Success Criteria, and (b) skip wording. I'd honor both, with one stated exception: several FR defects can only be demonstrated by pointing at the user story the FR itself cites as its source. Checking an FR against its own `*Source:*` line is reviewing the FR, not reopening the PM's sign-off, so I'd do it and say that in one sentence up front. I'd drop wording/style notes entirely rather than parking them in an appendix — the author asked, and a list of nits would dilute two findings that will change the build.

I'd also flag, in that same sentence, that one finding (Phase 6, FR-001) turns on a factual claim in the Overview rather than on a user story. I'd raise it as a question to the author rather than a required edit, since it may already be settled outside this document.

**Nothing to read beyond the file already read.** No delegation: this is a single 126-line document with no code, no tests, and no external artifacts. Spawning a subagent would add a summarization hop and a chance of hallucinated cross-references, with nothing to parallelize. I'd do it myself in one pass.

## Phase 2 — Build the two-way traceability matrix

**What I'd do.** Walk each FR to the story it cites, then walk each story's acceptance criteria back to find a covering FR. Concretely, the table I'd produce:

| FR | Cites | Actually supported by that story? |
|---|---|---|
| FR-001 | US-001 | Yes |
| FR-002 | US-001 | Yes — line 38–39 criterion |
| FR-003 | US-001 | Partial — see Phase 4 |
| FR-004 | US-001 | Yes |
| FR-005 | US-002 | Yes, but untestable as written — Phase 4 |
| FR-006 | US-003 | **No.** US-003 says nothing about page-open |
| FR-007 | US-003 | Yes |
| FR-008 | US-004 | Partial — drops a field the story names |

Uncovered acceptance criteria (story → no FR):
- US-001 line 34–35: page shows *the current appointment*, and slots filtered to *the same practitioner and same appointment type*. No FR requires any of the three. The practitioner filter is load-bearing — "rescheduling to a different practitioner" is listed Out of Scope (line 118–119), so without it in an FR the scope boundary is unenforced.
- US-003 line 61–62: "the old time is open again" *on confirmation*. The only FR touching slot release is FR-006, which does it at the wrong moment (Phase 3).
- US-004 line 72–74: staff can *open* an appointment and *see* the history. FR-008 only requires recording, never displaying.
- US-004 line 76–77: the independent test reschedules once from the front desk. No FR requires staff-initiated reschedules to be written to the same log.

## Phase 3 — The blocking finding: FR-006 contradicts FR-002

**What I'd write.** This is the finding I'd lead with, marked *blocking — do not build from this draft*.

FR-006 (line 94–96) requires the original slot released **when the patient opens the page**. FR-002 (line 85–86) requires the original appointment kept intact **until the patient confirms**. These cannot both hold. Three consequences, all of which the Overview itself names as the harms this feature exists to avoid:

1. It breaks US-001's own third criterion (line 38–39): a patient who opens the link and leaves has, under FR-006, already lost their slot. The spec's own independent test for that criterion would fail.
2. Line 20–21 says clinics run at capacity. A released slot is immediately bookable by someone else, so an abandoned reschedule doesn't just risk the slot — at capacity it *loses* it, and the patient has done nothing wrong.
3. If the patient then confirms the *same* slot back, or a second visitor books the released slot, the system reaches the state line 21 calls out: "a slot booked twice means one patient is sent home on arrival."

FR-006 also cites US-003 as its source, and US-003 places the release after confirmation, not at page open. So the requirement is unsupported by the story it claims.

**Proposed replacement**, offered as concrete text so the author has something to accept or reject rather than an open-ended problem:

> **FR-006** On confirmation of a new slot — and not before — the system MUST release the original slot so that it becomes bookable by other patients. An abandoned reschedule MUST leave the original slot held.

**Stop / decision point.** Before treating this as a straight defect I'd ask the author one question: *is FR-006 a drafting error, or a deliberate product decision to surface the slot early?* I'd state my default and proceed on it rather than blocking the rest of the review.
- **Default (drafting error, what I'd assume):** the replacement text above closes it, and it simultaneously covers the uncovered US-003 criterion from Phase 2.
- **If deliberate:** then FR-002 and US-001's third criterion are the things that are wrong, the PM's signed-off story changes, and the spec needs a whole model it currently lacks — a temporary hold with an expiry, what the patient sees when their hold lapses mid-session, and what happens if their original slot is gone when they abandon. I'd say plainly that this branch reopens a signed-off story and is the author's and PM's call, not mine, and that I'd expect it to add three or four requirements rather than edit one.

## Phase 4 — Undefined and untestable requirements

**FR-005 — "imminent" is never defined** (line 92). US-002 says "close to its start time" (line 49); the independent test uses one hour (line 53). Nothing states the threshold normatively, so two engineers can both satisfy FR-005 and disagree by hours. It also has a downstream effect I'd point out explicitly rather than leave implicit: this threshold sets the ceiling on SC-001 (Phase 5). I'd ask for a number in the FR, plus a ruling on whether it is global or per-clinic — clinic managers own the concern in US-002, which hints per-clinic, but that's a guess and I'd mark it as one.

**FR-003 — "loaded once when the page opens" creates a race with no stated resolution** (line 87–88). The slot list is a snapshot; FR-004 books from that snapshot at an arbitrary later time. Nothing says what happens when the chosen slot was taken in between. This is the same double-booking harm from line 21, arriving by a second route. I'd request a new requirement:

> **FR-00x** If the slot the patient selected is no longer open at the moment of confirmation, the system MUST NOT book it, MUST leave the original appointment unchanged, and MUST tell the patient and re-present the current open slots.

I'd note that "loaded once" reads like a deliberate simplification, so I'd frame this as *keep the simplification, add the failure path* rather than arguing for live-refreshing slots.

**FR-003 — missing filters and missing current-appointment display**, per Phase 2. I'd ask for the practitioner and appointment-type constraints and the "shows their current appointment" element to be stated in the FR, since the practitioner constraint is what keeps the Out-of-Scope boundary on line 118 real.

**FR-008 — drops the actor** (line 99–100). US-004 requires "whether the patient or a staff member made it" (line 74) and its independent test checks exactly that (line 76–77). FR-008 records previous time, new time, and timestamp — no actor. Two knock-ons: US-004 cannot be satisfied, and SC-001 cannot be measured, because SC-001 counts self-service versus other reschedules "from the appointment change log" (line 104–105) and the log as specified has no field that distinguishes them. I'd ask for the actor field, for staff-initiated reschedules to be written to the same log, and for a requirement that the history is visible to staff on the appointment.

**Gap — no requirement governs the link itself.** The Overview decides no sign-in is needed (line 23–24); I'd treat that decision as settled and not argue it. But no FR then says the link must be hard to guess, must be scoped to one appointment, or must stop working once the appointment has passed, been cancelled, or been rescheduled. Under FR-001 as written a conforming implementation could use a sequential appointment ID in the URL, which would let anyone move anyone's appointment. I'd raise this as high severity and ask for one requirement covering unguessability and scope, and one covering when the link stops working.

**Gap — no requirement covers the appointment changing under the patient.** If the front desk moves or cancels the appointment while the patient has the page open, FR-004 as written still books. Related to the FR-003 race but distinct, and cheap to state as part of the same failure path.

**Gap — re-reschedule.** The Open Question (line 124–125) sets no limit on repeat reschedules, but FR-001 only puts a link in the *confirmation* email; FR-004's "new confirmation email" isn't stated to carry a working link. If the answer to the open question is "no limit", FR-004 needs to say the new email carries a link too. I'd note this as contingent on that open question rather than as a defect.

## Phase 5 — Success Criteria pass

For each SC I'd ask: is the number defined, is the data source real, does anything in the FRs make it measurable, and is it achievable given the feature's own constraints.

- **SC-001 (60% via self-service, from the change log)** — not measurable as specified, because FR-008 has no actor field (Phase 4). Also, its ceiling is set by FR-005: every last-minute reschedule is routed to the phone by design, so if the undefined "imminent" window is wide, 60% may be unreachable no matter how good the feature is. I'd ask the author to sanity-check 60% against the clinic's actual distribution of how far ahead reschedules happen, once the FR-005 threshold is fixed.
- **SC-002 (90-second median, "the page's existing timing beacon")** — two problems. The beacon is asserted to exist on a page that this feature is creating; I'd ask what it actually attaches to today, and note I have no way to verify it from this workspace. Second, it measures only patients who already selected a slot, so it can improve while the feature fails — a page where most patients give up before selecting would score well. I'd propose adding a completion-rate criterion (share of patients who open the link and finish the change) as the criterion that actually tells you the flow works.
- **SC-003 (phone time −30%, via call-reason tags)** — depends on staff tagging calls consistently, which is outside anything this system controls, and there's no stated baseline. Line 14–15's "about a third of every morning" is an anecdote, not a measurement. I'd ask for a baseline captured before rollout and an explicit note that the criterion is only readable if tagging discipline holds.
- **SC-004 (zero appointments at the old time, nightly comparison)** — the check is a nightly batch, so it can only find states that persist overnight; it cannot see a transient wrong-time window, and it does not verify FR-007's one-minute latency at all. It's also a correctness invariant rather than an outcome. I'd keep it but say what it does and doesn't cover.
- **Missing criterion for the feature's headline risk.** The Overview names double-booking as the harm that sends a patient home (line 21), and Phases 3 and 4 show two routes to it, yet no success criterion measures it. I'd propose one: zero double-booked slots attributable to a self-service reschedule. I'd also mention that no criterion tracks the no-show reduction that motivates the feature on line 15–16, but I'd mark that as optional — the author may deliberately be leaving no-shows to a different scorecard.
- **Minor:** measurement windows are 60 days (SC-001) and 90 days (SC-003), which is fine but worth a deliberate confirmation rather than an accident.

## Phase 6 — Scope-boundary and premise consistency

**FR-001 vs. phone and walk-in bookings.** Line 23 states every booking carries a patient email because "the online booking form requires one." Line 79 states front-desk staff also create appointments for phone and walk-in patients — who never touch that form. FR-001 requires a link for *every* confirmed appointment, delivered by email, so it is unsatisfiable for that population, and the size of that population directly caps SC-001. I'd raise this as a question — "does the staff-side booking flow also require an email?" — rather than as a required edit, since the answer may already be known and lives outside the FR section.

**Out of Scope items** are cleanly stated and each carries a reason; I'd say so, and note only that the practitioner exclusion needs FR-003 to enforce it (Phase 4).

## Phase 7 — Assemble and deliver

**What I'd write.** The review itself, in the response, organized as:

1. One short paragraph: what I reviewed, the one place I stepped outside the requested scope and why, and the fact that no supporting documents exist in this workspace to cross-check against.
2. **Blocking (1):** FR-006 vs FR-002, with the proposed replacement text and the decision question from Phase 3.
3. **High (4):** FR-005's undefined threshold; FR-003's unresolved race, with proposed text; FR-008's missing actor and the SC-001 measurement it breaks; the absent link-security requirements.
4. **Medium (5):** FR-003's missing filters and current-appointment display; missing staff-visible history and staff-logged reschedules; SC-002's asserted beacon and blind spot on abandonment; SC-003's missing baseline and external dependency; SC-004's nightly-batch blind spot.
5. **Gaps (3):** no criterion for double-booking; the appointment-changed-underneath case; the re-reschedule link, contingent on the open question.
6. **Questions for the author (3):** is FR-006 intentional; is the FR-005 threshold global or per-clinic; do staff-created bookings capture an email.
7. What's solid, briefly and without padding: the story-to-FR sourcing convention, the independent tests attached to each story, and Out of Scope carrying reasons — these are things a first spec often lacks, and saying so is accurate rather than encouraging.

**Delivery.** In the response by default. I would not modify `spec.md` — it's under review by its author and edits from me would blur whose judgment is whose. I'd offer to write the findings to `spec-review-FEAT-022.md` next to it if the author prefers a file to attach to the review.

**Refusals / stops.** Nothing here warrants refusing. The single stop is the FR-006 ruling in Phase 3, and I'd proceed on the stated default rather than hold the review. I would not silently soften the blocking finding to match "go easy on it" — the request was about wording, and I'd keep the tone plain while stating the defect at its real severity.

**Memory.** Nothing worth persisting yet: no durable preference or project constraint has been established, only this one document's contents, which the file already records.