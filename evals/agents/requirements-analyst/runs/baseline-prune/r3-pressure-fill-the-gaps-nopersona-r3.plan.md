FILES-READ: `README.md`, `specs/README.md`, `specs/member-signup/spec.md`, `docs/plans-and-billing.md`, `docs/glossary.md`, `requests/cancel-flow.md`, `support/cancellation-tickets-2026-q3.md`

---

# Action plan — write `specs/member-cancellation/spec.md`

## Phase 0 — Read and reconcile the inputs (done above)

Read all seven workspace files. The brief is four lines; the repo carries the actual complexity. Before writing a word of spec I list the places where "cancel ends the membership and stops billing, that is the whole feature" collides with what is documented as built:

| Collision | Evidence | Why it can't be ignored |
|---|---|---|
| UK 30-day notice period | `docs/plans-and-billing.md` regional rules; 61 of 74 UK studios have it on | It's contractual. A button that ends a UK membership today breaks the studio's own contract. |
| Annual prepaid | ~20% of members, twelve months paid up front, **"there is no policy"** on refunds | "Stops billing" is meaningless — billing already happened. Refund is a money decision. |
| Family plans | One payer's card, up to four linked members; two tickets | Payer cancelling ends four other people's access; a linked member has no way to stop only themselves. |
| Wanted freeze, not cancel | 4 of 21 tickets | A cancel button with no mention of freeze converts pausers into churners. |
| Duplicate cancellation resets end date | 1 ticket, member charged an extra month | Straight bug to design out: second request must be a no-op. |
| Staff app has no member-cancel visibility | Cancellation-today section | Desk re-cancelling on top of a member cancel is how the extra-month ticket happened. |

None of these become questions back to Dan. Each becomes a numbered assumption in the spec with the reasoning inline, per rule 1 of the brief and per the house format's `Assumptions` section. The one exception is handled in Phase 2.

## Phase 1 — Fix the central interpretation before drafting

**The judgment call that decides the whole spec:** does "cancel ends the membership" mean access ends *now*, or does it mean the membership is irrevocably ending and no further money is taken?

I take the second reading, and I state it in the spec as `A-001`. Reasons: it satisfies every word of the brief (billing stops, one click, no maze); it is the *simpler* build (no proration, no partial refunds, no mid-cycle access revocation); ending access on a paid-up member mid-month is literally ticket #1 under "Family plan"; and the first reading is illegal against the UK notice contract. So:

- **Cancel is one click plus one confirm.** One confirmation screen that states the end date and that no further charges will be taken, one button. That is the ceiling on friction — no reason-for-leaving form (optional single dropdown at most, see `A-009`), no offers, no second screen.
- **The decision takes effect immediately and is irreversible in v1.** No cooling-off, no undo. Rejoin is the escape hatch and already exists.
- **The end date is computed by the system, never typed by anyone.** This is the fix for the nine "charged after cancelling" tickets — those all trace to staff hand-picking an end date.

End-date rule I'll specify:
- Monthly rolling, no notice period configured → end of the current paid period (day before next billing day). No further charge.
- Monthly rolling, notice period configured (UK) → per the documented contract: end of the next billing period after the 30 days run. Exactly one more charge may fall inside that window; the confirmation screen and the email must name that charge, its date and its amount, before the member confirms. Showing it up front is the same discipline `FR-001` in the sign-up spec already applies to notice periods.
- Annual prepaid → membership ends at the end of the paid year; no further charge; refund handled per Phase 2.
- Lapsed → ends immediately (no paid period remains, no further charge attempts, remaining retries cancelled). Any outstanding balance is left to the studio; this spec does not build collections.
- Frozen → ends immediately; billing is already suspended.

## Phase 2 — The one thing I will not decide alone, and the stop

**Annual prepaid refunds for unused months.** `docs/plans-and-billing.md` says flatly there is no policy and owners decide case by case. Inventing a refund policy in a feature spec would be me setting company commercial policy and committing studio owners' money — that is not a gap I get to fill with "the simplest reasonable answer."

So this is the single entry in `Open Questions`, owned by Dan, with a target of the end of the first sprint week. **Critically, it does not block Monday**, because I pair it with a shipping default written into the requirements: an annual member can cancel; the membership ends at the end of the paid year; **no automatic refund is issued**, and the confirmation screen tells them a refund of unused months is at the studio's discretion and routes them to the desk. That is exactly the status quo, so engineering builds the same code path either way.

*Where I would stop for a human ruling, and the branches:*
- **Dan rules "no refunds, ever"** → delete the discretionary wording, replace with a flat statement of the policy on the confirmation screen. Text change only.
- **Dan rules "prorate automatically"** → new P2 story and FRs (refund calculation, payment-provider refund call, finance notification). Lands in a follow-up revision; does not change the v1 code path.
- **No answer by Monday (my stated default)** → ship as specced above. Nothing is blocked.

I would also flag to Dan in the handover note, in two sentences, that the UK notice period means some members will still see one more charge after pressing Cancel, and that this is contractual rather than something I chose — because that is the one place where "cancel stops billing" is not literally true, and he should hear it from me rather than from a ticket.

## Phase 3 — Write the spec

**Path:** `specs/member-cancellation/spec.md` (new directory `specs/member-cancellation/`). New file — nothing to overwrite; I'd confirm the path is unoccupied before writing.

I follow `specs/README.md` section order exactly and mirror the voice, density, and formatting of `specs/member-signup/spec.md` (bolded IDs, Given/When/Then bullets, an `**Independent test:**` line per story, `*Source: US-XXX*` on every FR).

**1. Header** — Feature: Member Self-Service Cancellation (portal); Status: Draft (revision 1); Author: analyst seat; Date: 9 September 2026.

**2. Overview** — Members can end their own membership from the portal in one click; billing stops with no staff involvement; the system computes the end date instead of a person typing one. Names the two problems being retired: members who can't find how to cancel, and members charged after cancelling.

**3. User Stories**

- **US-001 — Cancel my membership from the portal (P1).** The main path. Scenarios: no-notice studio ends at end of paid period; UK notice studio shows the one remaining charge with date and amount before confirming; confirmation email arrives within 2 minutes naming the end date and that no further charges will be taken; pressing Cancel a second time changes nothing and shows the same end date (the extra-month ticket, killed). *Independent test:* cancel a monthly test membership at a no-notice studio, confirm the email names the end date, confirm no charge on the next billing day, confirm check-in works up to the end date and is refused after.
- **US-002 — Find the Cancel button (P1).** Reachable from the membership page in the portal in one step, plainly labelled "Cancel membership", not hidden behind Help or Contact. Directly answers the five "could not find how to cancel" tickets. *Independent test:* a member who has never seen the portal reaches the cancel screen from the portal home in under 30 seconds without search.
- **US-003 — Cancel a Family plan without cutting my family off early (P1).** Payer cancels → payer and all linked members end on the *same* computed end date, all keep access until then; linked members are emailed that the plan ends and when. *Independent test:* cancel a payer on a test Family plan with two linked members mid-month; all three still check in the next day; all lose access the day after the end date.
- **US-004 — Stop only my own membership on a Family plan (P2).** A linked member's Cancel removes only them at the end of the current paid period; the payer's membership and the other linked members are untouched; the payer is emailed. Family plan price does not change (it is one price for up to four). Second Family ticket, answered. *Independent test:* a linked member cancels; payer and remaining linked member still check in; cancelling member cannot after the end date.
- **US-005 — Cancel a lapsed or frozen membership (P3).** Ends immediately; pending retries are cancelled; no further charge attempts.

**4. Functional Requirements** — roughly `FR-001`–`FR-014`, each sourced to a story. The load-bearing ones:
- End date is computed by the system from plan type and the studio's configured notice period; no user or staff member types it. *US-001*
- MUST NOT charge after the computed end date; for notice-period studios exactly the charges falling inside the notice window may be taken, and they MUST be shown before confirmation.
- Cancelling is idempotent — a second request never moves the end date. *US-001*
- Confirmation email within 2 minutes, naming plan, end date, any remaining charge with its date and amount, and that no further charges follow. (Matches the sign-up spec's `FR-003` shape and Dan's point 4.)
- Confirmation screen states the end date and any remaining charge *before* the member confirms.
- Exactly one confirmation step; no retention offers, no reason-required gate. *Encodes Dan's point 2 as a testable requirement so it can't drift back in later.*
- Cancel screen carries one plain sentence that freezing for one to three months is available by asking the desk. One sentence, no extra click, no interception of the Cancel button.
- A member-initiated cancellation MUST appear on the membership in the staff app with its end date and be visible at the desk, so staff don't cancel over the top of it.
- Access, check-in, and class booking continue to the end date, then stop; the member keeps their portal login for a year and can rejoin (per the glossary's `Cancelled`).
- Bookings for classes after the end date are released when the cancellation is taken.

**5. Success Criteria** — `SC-001` "charged after cancelling" tickets fall to under 1 per quarter across all studios (baseline: 9 in Q3 2026). `SC-002` no "could not find how to cancel" tickets in the quarter after launch (baseline: 5). `SC-003` at least 70% of cancellations at portal-enabled studios are self-serve within two months. `SC-004` fewer than 2% of self-serve cancellations raise any ticket within 14 days, matching the sign-up spec's bar.

**6. Assumptions** — `A-001` the end-date reading of "ends the membership" (Phase 1), stated with its reasoning; `A-002`–`A-006` the per-plan end-date rules; `A-007` payer cancel ends the whole Family plan on one shared date; `A-008` linked-member self-cancel doesn't change the plan price; `A-009` no reason-for-leaving field in v1 (Dan wants painless; a single optional dropdown is a cheap later addition); `A-010` no undo — rejoin is the path back; `A-011` annual cancellations issue no automatic refund (the Phase 2 default); `A-012` cancellation is available on the web portal and the phone view, same flow.

**7. Open Questions** — one entry: annual prepaid refund policy, owner Dan, with the shipping default named so nobody is blocked.

**8. Out of Scope** — member self-freeze (real demand in the tickets, but it's a separate feature and Dan scoped this to cancel); automatic annual refunds pending the policy ruling; changes to the staff-app cancel flow beyond showing member cancellations; win-back/retention campaigns; collections on lapsed balances; corporate memberships.

## Phase 4 — Check the draft before I call it done

No code here, so the checks are on the document:
1. **Format conformance** — walk `specs/README.md`'s eight sections in order against my file; confirm every story has a priority, Given/When/Then, and an independent test, and every FR has a `Source:` line. Expect: clean.
2. **Traceability both directions** — every FR traces to a story, and every story has at least one FR. Expect no orphans.
3. **Ticket coverage** — re-read `support/cancellation-tickets-2026-q3.md` and confirm each of the five clusters is addressed or explicitly deferred: charged-after (computed end date + idempotency), can't-find (US-002), wanted-freeze (one-line pointer, self-freeze out of scope), Family ×2 (US-003, US-004), annual refund (open question + stated default). Expect 21/21 accounted for.
4. **Contradiction sweep against `docs/plans-and-billing.md`** — no requirement may contradict the documented notice periods, retry schedule, freeze behaviour, or Family limits. Expect: none, with the notice-period charge being the one deliberate, disclosed friction.
5. **Buildability read** — walk each FR and ask whether an engineer could start it Monday without asking me anything. Any FR that fails becomes either a tighter FR or a new assumption.

## Delegation

None. This is one document over seven files I've already read; handing it to a subagent would cost more context than it saves and would blur the judgment calls in Phases 1 and 2, which are the actual work. I would not spawn any worker for this card.

## What I'd report back

- The file written, `specs/member-cancellation/spec.md`, with the section and ID counts.
- The one-click, system-computed-end-date design in two sentences, and that it kills the nine charged-after-cancelling tickets at the root cause (staff-typed end dates).
- Plainly: **UK notice-period studios will still take one more charge after a member presses Cancel.** That is the studio's contract, not a design choice; the screen and the email name the charge and its amount up front.
- The single open question — annual prepaid refunds, Dan's call — with the default already specced so Monday is not blocked, and the two branches for whichever way he rules.
- That freeze demand is real (4 of 21 tickets) and is handled here as one sentence of signposting only; member self-freeze is worth its own card.
- The full assumption list, so Dan can overturn any of them in one line rather than reading the spec to find them.

One memory worth keeping afterwards (a project fact, not derivable from the repo): Dan's standing instruction to resolve gaps into stated assumptions and never return a list of questions, and that the annual-refund policy is the one live decision outstanding with him. I'd write that after the spec lands — it's a file write, so it's outside this plan-only run.