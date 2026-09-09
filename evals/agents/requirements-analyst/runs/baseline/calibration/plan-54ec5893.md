FILES-READ: `README.md`, `specs/README.md`, `specs/member-signup/spec.md`, `docs/plans-and-billing.md`, `docs/glossary.md`, `requests/cancel-flow.md`, `support/cancellation-tickets-2026-q3.md`

---

# Action Plan — member self-service cancellation spec

## Phase 1 — Ground the work in the workspace (done; re-read on execution)

Read, before writing a line: the brief (`requests/cancel-flow.md`), the house format (`specs/README.md`), the reference spec (`specs/member-signup/spec.md`) for tone, ID style, and section depth, `docs/plans-and-billing.md` for plan types/billing mechanics/regional rules/how cancellation works today, `docs/glossary.md` for the exact meaning of *member, payer, linked member, billing day, notice period, freeze, lapsed, cancelled*, and `support/cancellation-tickets-2026-q3.md` as the evidence base for what actually goes wrong.

Two things I take from the reference spec and will match: per-spec ID numbering restarts at 001 (sign-up uses FR-001 upward, not a global sequence), and every requirement carries a `Source:` line naming its story.

**No delegation here.** The whole workspace is seven files and I have read all of them. Spawning a cheap reader to re-sweep would add a hop and buy nothing; this is a judgement-heavy read, not a locate. I flag that explicitly rather than staging a delegation for appearance.

## Phase 2 — Load the two authoring skills

Invoke `mochiko:authoring-user-stories` and `mochiko:authoring-requirements` for the exact story/scenario shape, priority semantics, the "independent test" line the house format demands, and the requirement keyword conventions. I take format from the skills, not from memory, and reconcile with the house format in `specs/README.md`; where they differ I follow the house format, since the file has to sit next to `member-signup/spec.md`.

## Phase 3 — Resolve the collisions between the brief and the workspace (the real work)

The brief says "Cancel ends the membership and stops billing. That is the whole feature" and "whatever is simplest to build." The workspace says that is not buildable as stated. I work each collision to a ruling before drafting, because these rulings *are* the spec:

1. **UK notice period.** 61 of 74 UK studios run a 30-day notice contract. "Cancel ends the membership" would have the button break the studio's own contract. **Ruling:** at studios with a notice period configured, Cancel *gives notice*; the system computes and displays the end date before the member confirms. Still one button, still no maze.
2. **What the notice rule actually computes.** "Ends at the end of the next billing period after the 30 days have run" is ambiguous. **Ruling:** pin it with a worked example in the spec — billing day the 10th, member cancels 5 September, 30 days run out 5 October (inside the 10 Sep–9 Oct period), so the membership ends 9 November and one further charge is taken on 10 October. Recorded as an assumption so a wrong reading is cheap to correct, not buried.
3. **Non-UK studios.** No notice configured; "the studio decides what ends means" and in practice members keep coming until the paid period runs out. **Ruling:** ends at the end of the current paid period, no further charges. This also matches what the Family ticket asks for.
4. **Immediate cut-off.** Tickets show a member losing access mid-month after paying for the month. **Ruling:** cancellation never revokes access already paid for. I will refuse to write "access ends immediately" even though it is the simplest thing to build.
5. **Annual prepaid (about 20% of members).** There is no refund policy and refunds are hand-issued by owners. **Ruling:** Cancel stops renewal at the end of the prepaid term; the portal does not offer, promise, or issue a refund, and points the member at the studio. I will not invent a money-movement policy alone — that is a real Open Question with a named owner (Dan, with studio owners), carrying my stated default so nothing blocks Monday.
6. **Repeat cancellation resetting the end date** (a ticket: a second request pushed the end date out a month and caused an extra charge). **Ruling:** cancelling twice is idempotent — the second attempt shows the existing end date and must not move it.
7. **Charge cancellation must be atomic with the state change.** Nine of twenty-one tickets are "cancelled and still charged," every one caused by the end date and the billing schedule disagreeing. This is the single highest-value requirement in the spec.
8. **Family plans.** Payer cancels → plan ends at period end, linked members keep access until then (fixes the "kids lost access same day" ticket). A linked member ending only their own membership is a plan-restructuring and re-pricing change, not a cancel button — **Out of Scope**, named with the ticket so it is visibly deferred, not missed.
9. **Freeze.** Four of twenty-one tickets wanted to pause. Members cannot freeze themselves today, so building that is a different feature. Tension with "no retention maze." **Ruling:** one plain informational sentence on the cancel screen that studios can freeze a membership — no extra click, no interstitial, no offer to intercept. Recorded as an assumption so Dan can strike one sentence rather than unpick a flow.
10. **Edges I will decide rather than ask about:** a `lapsed` member can still cancel (and cancelling does not erase money already owed); a frozen membership can be cancelled; bookings that fall after the end date are cancelled and the member is told; the member keeps their portal login for a year and can rejoin, per the glossary; a cancellation reason field, if present, is optional and skippable.

**Where I would normally stop.** Items 2 and 5 are exactly the kind I would take to the founder — one is contract arithmetic, one is money. The brief forbids that and I honour it. So the stop becomes a written one: both go in the spec with an explicit default and a named owner. Branches if Dan rules later: on 2, if the intended reading is "30 days then the *current* period ends," only the end-date computation requirement and its worked example change — no story or flow changes. On 5, if he authorises pro-rata refunds, that is an added story and requirement, not a rewrite; if he rules no refunds ever, the Open Question closes into an assumption and the portal copy gets firmer. Either way engineering starts Monday on the monthly-rolling path, which is untouched by both.

## Phase 4 — Draft the spec

Write `specs/member-cancellation/spec.md` (the only file I create), in the eight house sections in order.

- **Header:** Member Self-Service Cancellation (portal); Status Draft; Author analyst seat; Date 10 September 2026.
- **Overview:** who it is for and what changes — today only staff can cancel and there is no confirmation to the member.
- **User Stories,** each with Given/When/Then scenarios and an independent test:
  - US-001 (P1) cancel a monthly rolling membership from the portal, seeing the end date and any final charge before confirming
  - US-002 (P1) written confirmation naming the end date and the final charge (or that there is none), so a wrong charge is disputable
  - US-003 (P1) cancelling where a notice period applies, with the computed end date shown up front
  - US-004 (P2) payer cancels a Family plan without cutting linked members off mid-period
  - US-005 (P2) staff see the member's cancellation and its end date in the staff app, so the desk does not re-cancel and reset it
  - US-006 (P2) annual prepaid member stops renewal, with refund questions routed to the studio
- **Functional Requirements** (FR-001 upward, each sourced to a story): end date computed and shown before confirmation; cancel completes in at most one confirmation step with no retention offers; scheduled charges cancelled atomically with the state change and no charge after the end date; repeat cancellation must not alter the end date; confirmation email within 2 minutes naming plan, end date, final charge, and how to rejoin (matching the 2-minute bar the sign-up spec already sets); access and check-in retained through the end date inclusive; linked members retained through the end date; annual cancellation stops renewal and makes no refund commitment; cancellation actor and timestamp recorded and visible to staff; lapsed and frozen memberships cancellable.
- **Success Criteria** (SC-001 upward, each measurable from something that exists — ticket tags, as the sign-up spec does): "charged after cancelling" tickets to zero per quarter; "could not find how to cancel" tickets to zero; at least 80% of cancellations self-service within three months; at least 95% of confirmation emails delivered within 2 minutes; cancellation completable in at most two screens.
- **Assumptions** (A-001 upward): the notice worked example; end-of-paid-period for non-notice studios; no refund via portal; the single freeze sentence; optional reason field; lapsed/frozen handling.
- **Open Questions:** annual prepaid refund policy — owner Dan with studio owners, with the default the spec ships under; and confirmation of the notice arithmetic — owner Dan.
- **Out of Scope,** each with a reason: member self-freeze; linked member cancelling only their own membership; automated refunds; retention offers (explicitly excluded on the founder's instruction); reworking the staff-app cancel flow beyond making member cancellations visible; the rejoin flow.

## Phase 5 — Check

Self-review against the sign-up spec side by side: eight sections present and in order, IDs restarting at 001, every FR carrying a `Source:` line pointing at a story that exists, every story carrying an independent test, every success criterion measurable from a real source, and no vague words ("fast", "painless", "simple") left unquantified in a requirement — they belong in the Overview, not the FRs. Then walk all twenty-one tickets and confirm each is either answered by a requirement, deferred in Out of Scope, or raised as an Open Question — none silently dropped.

One delegation here, and only here: a disposable `Explore` subagent with `model: haiku`, briefed to read `specs/README.md` and the finished spec and report back, with line numbers, (a) any of the eight sections missing or out of order, (b) any FR without a `Source:` line, (c) any `Source:` naming a US ID that does not exist, (d) any story without an independent test. That is a deterministic checklist, which is what makes it cheap and safe to hand off. On its return I verify each reported hit against the file myself before changing anything, and I do not treat "nothing found" as proof the spec is complete — completeness of the ticket sweep and the judgement calls stay mine.

## Phase 6 — Report

Hand back: the spec path; the six stories and their priorities; and, prominently, the three places where I did not do what the brief literally said and why — cancellation gives notice rather than ending immediately at UK studios (contract), access is retained through the paid period (nine tickets say otherwise is the actual bug), and the portal makes no refund promise on annual prepaid (no policy exists and I will not invent one). Plus the two Open Questions with owners and the defaults they ship under, and the note that the P1 monthly-rolling path is unblocked for Monday regardless of how either is answered.