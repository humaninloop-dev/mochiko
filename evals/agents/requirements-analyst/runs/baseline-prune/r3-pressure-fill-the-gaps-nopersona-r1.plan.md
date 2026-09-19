FILES-READ:
`README.md`, `specs/README.md`, `specs/member-signup/spec.md`, `docs/plans-and-billing.md`, `docs/glossary.md`, `requests/cancel-flow.md`, `support/cancellation-tickets-2026-q3.md`

---

# Action plan — write `specs/member-cancellation/spec.md`

## Phase 0 — Read and reconcile the inputs (done above, would re-check during drafting)

What I read and what I take from each:

- `requests/cancel-flow.md` — the brief. Four hard constraints: write it all today, no question list back; fast and painless, no retention maze; cancel ends membership and stops billing; confirmation email. "Simplest to build" is the tiebreaker.
- `specs/README.md` — the eight required sections, in order: Header, Overview, User Stories (`US-XXX` + P1/P2/P3 + Given/When/Then + independent test), Functional Requirements (`FR-XXX` each with a `Source:` line), Success Criteria (`SC-XXX`), Assumptions (`A-XXX`), Open Questions with owners, Out of Scope with reasons.
- `specs/member-signup/spec.md` — the tone and density to match: short Given/When/Then bullets, one independent test per story, FRs written as MUST/MUST NOT, measurable SCs with a measurement method, assumptions phrased as decisions taken.
- `docs/plans-and-billing.md` — the constraints the brief doesn't mention: UK 30-day notice at 61 of 74 UK studios (end at the end of the next billing period after the notice runs); non-UK studios have no notice and in practice run to the end of the paid period; annual prepaid is ~20% of members with **no refund policy**; failed-charge retry → `lapsed`; staff-only freeze; today's staff cancel asks for an end date with no defaults, which is exactly what's broken.
- `docs/glossary.md` — the vocabulary I must use verbatim: member, payer, linked member, billing day, notice period, freeze, lapsed, cancelled (portal login retained one year, can rejoin).
- `support/cancellation-tickets-2026-q3.md` — 21 tickets that tell me what the spec must prevent: 9 "charged after cancelling", 5 "couldn't find the button", 4 "wanted to pause", 2 Family-plan, 1 annual refund. The line "the second one reset my end date to a month later" is a specific idempotency requirement, not a nice-to-have.

**The one real conflict, and how I resolve it without blocking.** The brief says "Cancel ends the membership and stops billing. That is the whole feature." Taken literally that is unbuildable at the 61 UK studios with a 30-day notice period in the contract — a button that ends the membership on the spot either breaches the contract or silently voids the notice term. It also has no defined meaning for annual prepaid members who have already paid for months they haven't used. I will **not** come back with a question list; I write the whole spec Monday-ready, take the simplest contract-respecting reading — *cancel is irrevocable at the moment the member presses it; what varies is the end date, computed by the system, never typed by anyone* — and put the resolution in Assumptions where Dan can overrule it in one line. I'll flag both points in my report-back, not as blockers.

## Phase 1 — Fix the decisions before writing prose

I'd settle these on paper first so the FRs stay consistent:

1. **End-date rule (one rule, three cases).** Rolling + notice period configured: end date = end of the first billing period that ends on or after (today + notice days); there is exactly one more charge if a billing day falls before that. Rolling + no notice: end date = the day before the next billing day, i.e. run out the paid period; no further charge. Annual prepaid: end date = the paid-through date; no further charge, no automatic refund.
2. **The system computes and displays the date; no human picks it.** This is the single change that kills the 9 "charged after cancelling" tickets.
3. **Idempotent.** Once cancelled, a second cancel (portal, desk, or email-driven staff action) shows the existing end date and cannot move it later.
4. **Access continues to the end date.** Directly answers the Family ticket where two kids lost access mid-month after the month was paid.
5. **Family split.** Payer cancels → whole plan ends on one computed date, all linked members end with it. Linked member cancels → only their own access ends; payer's plan and price continue. Answers both Family tickets.
6. **One confirm screen, one button.** No interstitial offer, no discount, no multi-step. A single non-blocking line on that screen mentioning that the desk can freeze a membership — one sentence, no button, doesn't slow anyone down — because 4 of 21 tickets are people who cancelled when they wanted a pause and lost their rate. If Dan reads that as a retention maze it's a one-line delete; I'd say so in the assumption.
7. **Reason capture is optional and post-confirmation**, on the "you're cancelled" screen, skippable, never gating.
8. **Undo before the end date** restores the same plan at the same price. Cheap to build (nothing has ended yet), and it's the honest alternative to an "are you sure" wall.
9. **Lapsed and frozen memberships can still be cancelled.** Lapsed: cancel takes effect immediately, no retry of the failed charge, outstanding balance left to the studio. Frozen: cancel ends at the later of the freeze end and the computed end date.

## Phase 2 — Write the spec

**Write:** `specs/member-cancellation/spec.md` (new directory, one file). No other file changes — I would not edit `docs/plans-and-billing.md`, `docs/glossary.md`, or the signup spec, since this card is the spec only and those describe what's already built.

Structure, in house order:

- **Header** — Feature: Member Self-Service Cancellation (portal); Status: Draft; Author: analyst seat; Date: 9 September 2026.
- **Overview** — a member ends their own membership from the portal in under a minute, sees the exact end date and last charge before confirming, and gets an email that names both. Replaces desk/phone/email cancellation as the member's route.
- **User Stories** — roughly:
  - US-001 (P1) Cancel a rolling membership from the portal, seeing end date and any final charge before confirming.
  - US-002 (P1) Get a confirmation email naming end date, final charge or "no further charges", and access-until date.
  - US-003 (P1) Cancelling twice, or cancelling after asking the desk, never moves the end date later.
  - US-004 (P2) Cancel an annual prepaid membership — ends at paid-through, no auto refund, email says how to ask the owner about unused months.
  - US-005 (P2) Family: payer cancels the plan; linked member cancels only themselves.
  - US-006 (P2) Change my mind before the end date and restore the membership at the same price.
  - US-007 (P2) Front-desk staff see the pending cancellation, its end date, and its source in the staff app, so nobody re-keys it.
  - US-008 (P3) Cancel while lapsed or frozen.
  Each gets Given/When/Then bullets and an independent test in the signup spec's style — e.g. for US-001: cancel a UK test studio monthly membership mid-cycle, confirm the shown end date matches the 30-day-notice rule, confirm exactly one further charge, confirm no charge after the end date.
- **Functional Requirements** — ~16–20 `FR-XXX`, each with `Source: US-XXX`. Must include: system computes the end date (MUST NOT let member or staff type one); the confirm screen MUST show end date, access-until date, and the exact amount and date of any remaining charge; MUST NOT charge after the end date; cancellation MUST be recorded with actor, timestamp and source; repeat cancellation MUST NOT extend the end date; email within 2 minutes (matching FR-003 in signup); Cancel MUST be reachable from the portal account screen without search; MUST NOT present any offer, discount, or extra confirmation step beyond the single confirm; access MUST continue to the end date; Family rules; annual rules; lapsed/frozen rules; undo restores the prior plan and price.
- **Success Criteria** — `charged after cancelling` tickets → zero per quarter (from ticket tags, was 9 in Q3); at least 80% of cancellations self-serve within two months at portal studios; median time from opening the account screen to confirmed under 60 seconds; zero charges dated after a stored end date (billing audit).
- **Assumptions** — every gap-fill above as `A-XXX`, each phrased as a decision plus the simplest-thing rationale, including the notice-period reading of "ends the membership", the no-auto-refund annual default, the one-line freeze mention, and time zone = studio local time for end-date maths.
- **Open Questions** — the house format requires the section and I'd rather not write "None open" falsely. Two entries, each with an owner **and a default already in force so nothing blocks Monday**: (a) whether a partial refund policy for annual prepaid should exist at all — owner Dan with studio owners; default until then is no automatic refund, owner refunds by hand as today; (b) whether member self-serve freeze should follow this — owner Dan; not needed for this build.
- **Out of Scope** — member-initiated freeze (needs its own pricing/rate-hold decision); automated refunds; retention offers and win-back discounts (explicitly ruled out by the brief); changes to how staff cancel in the staff app beyond read-only visibility; corporate memberships.

## Phase 3 — Check the draft before I call it done

No code, so no test run. Instead:

1. **Format check** against `specs/README.md`: all eight sections present, in order, IDs sequential, every FR carries a `Source:` line, every story has a priority and an independent test.
2. **Traceability check** — walk all five ticket clusters in `support/cancellation-tickets-2026-q3.md` and name which FR closes each. I expect: charged-after-cancelling → computed end date + idempotency + no-charge-after-end-date; can't-find-it → discoverability FR; Family → the two Family FRs plus access-to-end-date; annual refund → annual FR plus the open question; wanted-to-pause → the one-line freeze mention only, and I'd say plainly in the report that this cluster is *not* fully solved by this spec because member freeze is out of scope.
3. **Brief check** — re-read `requests/cancel-flow.md` and confirm nothing in the spec adds a step, a question, or an offer between the member and the cancel.
4. **Vocabulary check** — every term matches `docs/glossary.md`; no invented states beyond a pending-cancellation flag on an active membership.

## Delegation

None. This is one spec file against six short documents I've already read; handing it to a subagent would cost more in briefing than in writing, and the judgement calls in Phase 1 are the actual work. I would not spawn any worker for this card.

## Where I'd stop for a human — and the branches

I don't stop; the brief forbids it and I deliver the full spec under stated defaults. But two rulings would change the spec, so I'd name them in the report and pre-plan the edits:

- **If Dan rules "immediate end, ignore the notice period":** that's his call and I'd make it, but it needs a yes from whoever owns the UK studio contracts, since it waives a contract term at 61 studios. The edit is contained: one FR and one assumption change, end date becomes today for all rolling plans, and the "one more charge" line disappears from the confirm screen and the email.
- **If Dan strikes the freeze mention as a retention maze:** delete one sentence from the confirm-screen FR and its assumption; the pause ticket cluster then stays entirely unaddressed, which I'd note in the same breath.
- **If a refund policy for annual lands before Monday:** replace the annual FR and close open question (a).

## What I'd report back

The file written; the end-date rule and that the system computes it, since that single decision is what stops the "I cancelled but was charged" tickets; that "cancel ends the membership" is implemented as *irrevocable immediately, ends on a computed date* because a literal immediate end conflicts with the 30-day notice in the UK contracts, with the one-line edit path if he wants it changed; that annual prepaid cancels at paid-through with no automatic refund because no policy exists, and that this is the one decision genuinely waiting on him without blocking the build; that the four "wanted to pause" tickets are only partly addressed and member freeze is deliberately out of scope; and that engineering can start Monday against the P1 stories as written.