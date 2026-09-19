FILES-READ: `README.md`, `specs/README.md`, `specs/member-signup/spec.md`, `docs/plans-and-billing.md`, `docs/glossary.md`, `requests/cancel-flow.md`, `support/cancellation-tickets-2026-q3.md`

---

# Action Plan — member self-service cancellation spec

## Phase 0 — Orientation (done as part of planning)

Read all seven workspace files. The corpus is small and every file bears directly on the spec, so I read them myself rather than farming them out; there is no bulk sweep here worth a disposable reader's round trip.

What I now hold as fact:

- The house format is fixed and ordered: Header, Overview, User Stories (`US-XXX`, P1/P2/P3, Given/When/Then, plus an "Independent test" line), Functional Requirements (`FR-XXX`, each with a `Source:` line), Success Criteria (`SC-XXX`), Assumptions (`A-XXX`), Open Questions with an owner, Out of Scope with reasons. `specs/member-signup/spec.md` is the shape to match, down to the em-dash header block and bolded Given/When/Then.
- Three plan types (monthly rolling, annual prepaid, Family with up to four linked members), a billing-day rule, a three-strikes `lapsed` rule, and a staff-only freeze of one to three months.
- 61 of 74 UK studios run a 30-day notice period where the membership ends at the end of the next billing period after the notice runs. Non-UK studios have no notice period and "the studio decides what ends means".
- Annual prepaid refunds have no policy at all — owner decides by hand.
- Today only staff can cancel, they type a free-text end date with no default, and the member gets nothing in writing.
- 21 support tickets: 9 "charged after cancelling" (all traceable to a staff-chosen end date after the next billing day, one to a second request resetting the date later), 5 "couldn't find how to cancel", 4 wanted a freeze, 2 Family problems (payer's cancel killed the kids' access same day mid-month; a linked member couldn't stop only her own), 1 annual refund taking three weeks.

## Phase 1 — Reconcile the brief against what the workspace says is true

Before writing a line, I resolve where Dan's "cancel ends the membership and stops billing, that is the whole feature" collides with the built system. I do this as a written decision list I carry into the draft, not as a question list back to Dan — he ruled that out and I honour it.

The four collisions and how I decide each:

1. **"Ends the membership" cannot mean "ends today" in the UK.** 61 UK studios have a contractual 30-day notice. A button that ends the membership on the spot either breaks the contract or silently ignores the configured field. Decision: the member's action is *requesting cancellation*; the system computes the end date from the studio's configuration and the plan, and shows it before the member confirms. One screen, one confirm — still fast, still painless.
2. **"Stops billing" needs a definition of the last charge.** The 9 worst tickets exist because nobody ever told a member the true end date and last charge. Decision: the confirm screen and the email both state the end date, the date and amount of any remaining charge, and the date access ends. This is disclosure, not a retention maze — I will say so explicitly in the spec so nobody reads it as a violation of Dan's rule 2.
3. **Annual prepaid has no refund policy.** I will not invent one; inventing a money-back rule is exactly the kind of guess that hurts. Decision: cancelling an annual plan stops the renewal and ends the membership at the end of the paid term, with no automatic refund, and the confirm screen offers a "request a refund" link that raises a ticket to the studio owner. Recorded as an assumption *and* raised as an Open Question with an owner, because it is a money and contract call.
4. **Four members wanted a freeze, not a cancel.** Dan forbids a retention maze. Decision: exactly one line of informational text on the confirm screen ("Going away for a while? Your studio can pause your membership for up to three months — ask the desk."), with no extra step, no interstitial, no click required to proceed. I flag in the spec that this is the deliberate limit of retention behaviour so nobody expands it into a funnel later.

**Stop I would otherwise take, and its branches.** With a human available I would confirm one thing with Dan: that member-initiated cancellation must honour a studio's configured 30-day notice rather than end immediately. If he ruled *honour it* — my default — the spec stands as planned. If he ruled *end immediately everywhere*, then FR-004/FR-005 below invert, the last-charge disclosure becomes a refund-of-part-month question, and I would refuse to write it without a legal read on the UK contract, saying so in the spec instead of quietly shipping a contract breach. He is unavailable per the brief, so I proceed under the default and log it as an Open Question owned by Dan with the branch spelled out.

## Phase 2 — Draft the stories

I would consult **`mochiko:authoring-user-stories`** here for the story and scenario format before writing (not loadable in this run; in the real run it is the source of truth for the templates, and I follow the sibling spec's rendering of them).

Planned stories, each with the role/capability/benefit sentence, Given/When/Then scenarios, and an independent test:

- **US-001 — Cancel my membership from the portal (P1).** The core. Scenarios: rolling plan at a studio with no notice period → ends at the end of the paid period, no further charge, access to that date; rolling plan at a UK studio with 30-day notice → end date computed as the end of the next billing period after the 30 days, remaining charge named before confirm; confirmation email inside 2 minutes naming end date, last charge, and access end; the Cancel control is reachable within two taps of portal home (kills the 5 "couldn't find it" tickets).
- **US-002 — Not be charged after I cancel (P1).** Scenarios: no charge is attempted after the end date; a second cancellation request while one is already scheduled changes nothing and shows the existing end date (this is the ticket where a belt-and-braces second request pushed the date out a month); staff app shows the scheduled cancellation and its end date so the desk cannot contradict it.
- **US-003 — Cancel a Family plan without cutting my family off mid-month (P1).** Scenarios: payer cancels → whole plan ends at the period end, all linked members keep access until then; a linked member cancels → only their own linked membership ends, the payer's plan and the other links are untouched; a linked member cannot cancel the payer's plan.
- **US-004 — Cancel an annual prepaid plan (P2).** Scenarios: cancel stops the renewal, membership runs to the end of the paid term, no automatic refund, the screen and email say so plainly and offer the refund-request link.
- **US-005 — Change my mind before the end date (P3).** Scenarios: while a cancellation is scheduled, the member can stop it and the membership continues on its original billing day with no re-pricing. Cheap because the state already exists; it also protects members from the panic re-cancel that caused a ticket.

Edge scenarios I place inside the stories rather than leaving implicit: cancelling while `lapsed`, and cancelling while frozen.

## Phase 3 — Requirements and success criteria

I would consult **`mochiko:authoring-requirements`** for the requirement and success-criteria format before writing. Every FR gets a `Source:` line naming its story, matching the sibling spec.

The requirement set I intend, in plain terms:

- Cancel control present in the portal for any active, lapsed, or frozen membership, reachable in at most two taps from portal home. *US-001*
- The system, not the member and not staff, computes the end date; the member never types one. *US-001*
- With no notice period configured: end date is the last day of the current paid period. *US-001*
- With a notice period configured: end date is the end of the next billing period after the notice days elapse, matching the studio's existing contract wording. *US-001*
- Before the member confirms, the screen must show the end date, the date access ends, and the date and amount of every remaining charge — or state plainly that there are none. *US-001, US-002*
- At most one confirmation step; no retention offer, no survey, no second "are you sure". A single informational line about staff-applied freeze is permitted and must not be a step. *US-001*
- No charge may be attempted after the end date. *US-002*
- Cancellation is idempotent: a repeat request never moves the end date in either direction. *US-002*
- Confirmation email within 2 minutes, naming plan, end date, access end, and last charge. *US-001* (mirrors the sign-up spec's 2-minute welcome-email bar, so we are consistent)
- Payer cancellation ends the Family plan at the period end and keeps all linked members' access until then. *US-003*
- A linked member may end only their own linked membership and must not be able to end the payer's plan. *US-003*
- Annual prepaid cancellation stops renewal, keeps access to the end of the paid term, and issues no automatic refund; the refund request routes to the studio owner. *US-004*
- Cancelling a lapsed membership ends it at the request date; any outstanding balance is neither collected nor written off by this feature. *US-002*
- Cancelling a frozen membership ends it at the end of the freeze and billing never resumes. *US-001*
- A scheduled cancellation is visible in the staff app with its end date and who initiated it. *US-002*
- A member may stop a scheduled cancellation up to the end date; price and billing day are unchanged. *US-005*

Success criteria, all measurable and tied to the evidence I have:

- Zero tickets tagged "charged after cancelling" attributable to a portal cancellation, measured per quarter (baseline: 9 in Q3).
- At least 80% of cancellations at portal-enabled studios initiated by the member rather than staff, measured monthly.
- Confirmation email delivered within 2 minutes for at least 99% of cancellations.
- Under 2% of portal cancellations raise a support ticket within 14 days, matching the sign-up spec's bar.
- Median time from opening the portal to a completed cancellation under 60 seconds.

## Phase 4 — Assumptions, Open Questions, Out of Scope

Assumptions I will write down rather than bury (each as `A-XXX`): access continues to the end date rather than ending on request; non-notice studios keep the member to the end of the paid period because that is what "most let" them do today; Family pricing is flat so a departing linked member triggers no proration; a cancelled member keeps the portal login for a year per the glossary and can rejoin at the then-current rate; the notice period is read from existing per-studio configuration and this feature adds no new configuration.

Open Questions, each with a named owner, because these are money or contract calls I should not settle alone: the annual prepaid refund policy (Dan, with studio owners); whether non-notice studios may configure "ends immediately" instead of period-end (Dan); the outstanding-balance treatment when a lapsed member cancels (Dan/finance); and the confirm-immediately-in-the-UK branch from Phase 1 if Dan overrules my default.

Out of Scope with reasons: member self-service freeze (real demand — 4 of 21 tickets — but it is a separate feature with billing-suspension mechanics, and folding it in would turn Cancel into the maze Dan forbade); automated refunds of any kind (no policy exists); staff-side cancellation changes beyond displaying the scheduled end date; win-back and retention offers.

## Phase 5 — Write the file

Write **one** file: `specs/member-cancellation/spec.md`. Header block: feature "Member Self-Service Cancellation (portal)", status Draft, author "analyst seat" to match the house convention, date 10 September 2026. No other file is created or edited; I do not touch the brief, the docs, or the support summary.

## Phase 6 — Self-check before reporting

A verification pass over my own draft: the eight sections present in the house order; IDs dense and gapless within the new spec (IDs are per-spec — the sign-up spec's FR-001 is not a collision, and I will confirm that reading of the format rather than renumbering); every FR carrying a `Source:` naming a story that exists; every story carrying a priority, at least two scenarios, and an independent test; no unquantified words like "fast", "quick", or "clear" left standing; every one of the 21 tickets' five clusters either answered by a requirement or explicitly parked in Out of Scope with a reason.

This is where I would use a disposable reader: after the file exists, one cheap `Explore` subagent pinned to **haiku**, briefed as "in `specs/member-cancellation/spec.md`, list every heading in document order, every `US-`/`FR-`/`SC-`/`A-` identifier in order of appearance, and every line under Functional Requirements that lacks the text `Source:` — facts only, quote line numbers." On return I check the headings match the eight house sections in order, the identifiers are sequential with no duplicates or gaps, and the no-`Source:` list is empty; anything else I fix myself. I would not delegate the judgement pass — whether a requirement is actually testable is mine.

No test suite exists in this workspace and nothing here is executable, so there is no test to run; the "Independent test" line on each story is the manual check engineering runs, and I write those to be performable on a test studio (e.g. for US-001: cancel a monthly membership at a UK test studio with the 30-day notice on, confirm the screen and the email name the same end date and the one remaining charge, then confirm no charge lands after that date).

## Phase 7 — What I report back

A short note stating: the spec is at `specs/member-cancellation/spec.md`, engineering can start Monday; the four places where "simplest to build" would have broken something real (UK notice period, annual refunds, Family mid-month cut-off, the second-request date reset) and what I decided in each; the single informational freeze line as the deliberate and only retention behaviour; and the four Open Questions with owners — noting that only the annual refund policy could block delivery of US-004, and that US-001 through US-003, which cover 16 of the 21 tickets, are unblocked and buildable as written.