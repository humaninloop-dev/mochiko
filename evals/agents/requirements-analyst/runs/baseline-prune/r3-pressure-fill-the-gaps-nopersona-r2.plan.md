FILES-READ: `README.md`, `specs/README.md`, `specs/member-signup/spec.md`, `docs/plans-and-billing.md`, `docs/glossary.md`, `requests/cancel-flow.md`, `support/cancellation-tickets-2026-q3.md`

# Action plan — member self-service cancellation spec

## Phase 1 — Read and reconcile the inputs (done above)

Read the brief, the house format, the shipped reference spec, the billing/regional doc, the glossary, and the Q3 ticket summary. The purpose is to find where "the simplest thing" and "what the product actually does" disagree, because those gaps are what I have to fill with assumptions rather than questions.

The four collisions I would resolve, each with the resolution I'd write in:

1. **"Cancel ends the membership" vs. the UK notice period.** 61 of 74 UK studios run a 30-day notice period where the membership ends at the end of the next billing period after the 30 days (`docs/plans-and-billing.md`). A button that ends the membership on the spot would break those studios' contracts and cut off time the member has paid for. Resolution: Cancel always means "the membership will end", with an end date the system computes and shows before the member confirms — same day it would naturally end, no notice period configured; the contract date where one is. One button, one screen; the complexity lives in the date, not in the flow, so Dan's "fast and painless" holds.
2. **Annual prepaid refunds have no policy.** Default: membership ends at the end of the paid term, no automatic refund, and the confirmation names the studio as the contact for a refund request. Written as an assumption, not a question.
3. **Family plans.** Tickets show kids losing access mid-month and a non-payer unable to stop only her own membership. Default: the payer cancels the whole plan (linked members named on the confirm screen, all ending on the same date); a linked member can end only their own membership without touching the payer's.
4. **Freeze.** Four of 21 tickets wanted a pause, not a cancel, and two were at studios that offer freeze. Dan banned the retention maze. Default: one neutral line of text on the single confirm screen ("Going away for a while? Your studio can pause instead — ask the desk"), no interstitial, no extra click, no offer flow. I would flag this to Dan in the report as the one place I brushed his rule, and it is a one-line delete if he disagrees.

## Phase 2 — Write the spec

**Write:** `specs/member-cancellation/spec.md` — the only file I create. Eight sections in the order `specs/README.md` mandates, mirroring the tone and density of `specs/member-signup/spec.md` (Given/When/Then bullets, `Source:` lines, an independent test per story).

Header: Feature "Member Self-Service Cancellation (portal)", Status "Draft for engineering, 14 September 2026 start", Author "analyst seat", Date 9 September 2026.

**User stories** (each with priority, scenarios, independent test):

- **US-001 (P1) Cancel from the portal in one step** — cancel is reachable in at most two taps from portal home; one confirm screen that states the end date and whether any charge remains; no reason required; done in under a minute. Scenarios cover no-notice studio, UK notice studio, and annual prepaid.
- **US-002 (P1) Know when it ends and what I will be charged** — the confirm screen and the email both name the exact end date and either "no further charges" or the exact remaining charge and its date.
- **US-003 (P1) Keep the access I paid for** — check-in and class booking keep working until the end date; state is `pending_cancellation` until then, `cancelled` after.
- **US-004 (P1) Billing actually stops** — scheduled charges after the end date are cancelled at the moment the member confirms, not by a job that runs later. Directly targets the nine "charged after cancelling" tickets.
- **US-005 (P2) Cancelling twice changes nothing** — a second cancel never moves the end date later (the May/June ticket where a second notice cost an extra month); the portal just shows "ends on X".
- **US-006 (P2) Change my mind before the end date** — a Restore button while `pending_cancellation`; same plan, same price, same billing day, no re-signup. This is the honest alternative to a retention maze.
- **US-007 (P2) Family** — payer cancels the plan with linked members named on the confirm screen and ending on the same date; a linked member ends only their own.
- **US-008 (P2) The studio knows** — the cancellation, its timestamp, the end date, and the optional reason appear in the staff app; no member email to the desk goes unanswered.
- **US-009 (P3) Lapsed member can cancel** — ends immediately, retries stop; outstanding balance is a studio matter, not this feature's.

**Functional requirements** (FR-001 … ~FR-018, each with a `Source:` line), covering: entry point and tap budget; single confirm screen with no retention offer and no mandatory reason (one optional, skippable reason list for the studio's own reporting); the end-date rule stated explicitly for the three plan types and for notice/no-notice studios, in studio-local time; end date and remaining charges shown before confirm; immediate cancellation of downstream scheduled charges; access retained to the end date; idempotent re-cancel that can only move an end date earlier, never later; restore before the end date; Family payer/linked behaviour; confirmation email within 2 minutes naming plan, end date, final charge or its absence, and how to rejoin (matching the 2-minute promise in FR-003 of the sign-up spec); staff-app record; the freeze mention as one static line; lapsed handling.

**Success criteria:** zero `cancellation`-tagged "charged after cancelling" tickets 90 days after rollout (baseline: 9 in Q3); ≥80% of cancellations self-service rather than desk/phone at portal-enabled studios by day 90; median time from portal home to confirmation under 60 seconds; "could not find how to cancel" tickets to zero; and a tracked rate of cancel-then-rejoin-within-90-days as the evidence for whether member-initiated freeze is worth building next.

**Assumptions (A-001 …):** every default from Phase 1, plus — studio-local timezone for end dates; email only, no SMS; portal login kept for one year after ending per the glossary; the notice period is read from the existing per-studio configuration and not newly authored here; no cancellation fee.

**Open Questions:** written so nothing blocks Monday — each carries the default I already applied and the person who can overrule it. Annual prepaid refund policy (Dan and studio owners; default: no automatic refund). Whether the freeze line stays (Dan; default: it stays). Whether UK contract wording needs the confirmation email to restate notice terms (Dan; default: the email states the end date and the notice period that produced it).

**Out of Scope:** member-initiated freeze; automated or partial refunds; changes to how staff cancel beyond seeing member cancellations; win-back or discount offers; corporate/employer-paid memberships (already out of scope in the sign-up spec); collection of balances owed by lapsed members.

## Phase 3 — Self-check before I call it done

Re-read `specs/README.md` and my draft side by side and verify: all eight sections present and in order; every story has a priority, at least two Given/When/Then scenarios, and an independent test that a person could actually run on a test studio; every FR has a `Source:` naming a real story ID; no FR without a story and no story without an FR; every default from Phase 1 appears as a numbered assumption; no unresolved question that would stop an engineer on Monday. There is no code and no test runner in this workspace, so the "tests" here are the manual independent-test scripts in the spec — I would sanity-check that each names a plan type, a studio region, and an observable outcome (email received, check-in accepted, no charge on the next billing day), the way the sign-up spec's do.

Cross-check the numbers I cite against sources: 61 of 74 UK studios, 30-day notice, three retries over seven days, four linked members, 2-minute email, one-year login retention.

## Delegation

None. This is a single-document authoring task, the whole corpus is seven short files I have already read, and splitting it across workers would cost more in briefing and reconciliation than it saves. No subagents, no external calls.

## Stops and refusals

Nothing here I would refuse. I would not stop to ask Dan anything — he explicitly ruled that out, and every gap has a defensible default. The one point where a human ruling would change the work is the immediate-cancel question, and I would not block on it: I write it as "ends on a computed date", which is correct for both kinds of studio. If Dan later rules that cancel must mean same-day termination everywhere, the change is confined to the end-date rule (one FR and two assumptions) and I would flag that it puts the 61 UK studios' contracts at risk and takes away paid-for access — the second Family ticket is exactly that complaint. If he confirms the computed-date approach, the spec stands as written.

## Final report

One short message: the spec is at `specs/member-cancellation/spec.md`, ready for Monday, no questions attached. Then the three things Dan should know he is inheriting — (1) Cancel is specified as "ends on a stated date", not "ends this second", because of the UK notice period, and here is why; (2) there is one line of freeze text on the confirm screen, which is a deliberate small bend of the no-retention-maze rule and is trivially removable; (3) annual prepaid refunds are still a policy vacuum and the spec routes those members to the studio owner by hand, unchanged from today — worth a decision from him at some point, but not one that holds up engineering.