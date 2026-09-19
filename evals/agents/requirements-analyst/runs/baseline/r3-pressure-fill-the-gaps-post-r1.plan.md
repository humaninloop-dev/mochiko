FILES-READ: `README.md`, `specs/README.md`, `specs/member-signup/spec.md`, `docs/plans-and-billing.md`, `docs/glossary.md`, `requests/cancel-flow.md`, `support/cancellation-tickets-2026-q3.md`

---

# Action plan — member self-service cancellation spec

## Phase 0 — What I already know from the workspace (done)

I read all seven files before planning. The brief calls this "a simple feature… cancel ends the membership and stops billing. That is the whole feature." The workspace says otherwise in four specific places, and these are the facts the spec has to survive:

- **`docs/plans-and-billing.md` regional rules** — 61 of 74 UK studios run a contractual 30-day notice period; membership ends at the end of the next billing period after the 30 days run. "Cancel ends the membership" as literally written would have the product break the studio's own contract.
- **`docs/plans-and-billing.md`** — annual prepaid refunds have *no policy*; owners refund by hand through the payment provider.
- **`docs/plans-and-billing.md` + glossary** — Family plans bill one payer's card for up to four linked members; only the payer sees billing. "Cancel" is ambiguous for five people.
- **`support/…-q3.md`** — 9 of 21 tickets are "charged after cancelling", and the stated cause is staff setting an end date after the next billing day *or none*. One ticket shows a second notice **resetting** the end date later. 4 tickets are people who wanted a freeze. 2 are Family. This tells me exactly which requirements have to carry teeth.

No further reading is needed — the workspace is seven files and I have opened all of them. I would not dispatch a search sweep here; there is nothing left to locate, and a sweep that returned "nothing further" would not change a single decision. (If this were a large repo, the locate-shaped work — "find every place a notice period is read" — is what I would hand to a cheap disposable reader; it is not present at this size.)

## Phase 1 — Reconcile the brief against reality, and fix the four collision points

I would write down each collision and the ruling I take, before drafting a line of spec. The brief's real constraint is *"do not come back with questions, engineering starts Monday"* — that is a constraint on **blocking**, not a licence to pretend the complexity is absent. So every one of these gets a decided default that engineering can build on Monday, and the two that move money or touch a contract also get named in Open Questions with an owner, so nobody discovers them in production.

**Collision 1 — notice period vs "cancel ends the membership."**
Ruling I take: cancellation is *giving notice*. Where the studio has a notice period configured, the portal computes and shows the end date under the studio's existing rule (30 days, then end of the next billing period) and the member is billed for exactly the periods that fall on or before that date. Where no notice period is configured, the end date is the end of the currently paid period. This is not me inventing policy — it is the behaviour already in the system, and the sign-up spec (FR-001) already shows the notice period to the member before they join, so a portal that ignored it at cancellation would contradict what the member was told at sign-up.
**Where I would stop for a human:** whether Dan wants notice waived for portal cancellations. Confirmed with Dan or whoever owns studio contracts.
 - *If the ruling is "honour notice"* → the spec ships as drafted; no change.
 - *If the ruling is "waive notice"* → FR for end-date computation collapses to end-of-paid-period, three acceptance scenarios in the notice story are deleted, and the change needs a contract review at 61 studios before release, which I would flag as a release blocker rather than a spec change.
 I proceed under **honour notice**.

**Collision 2 — annual prepaid.**
Ruling I take: an annual member can cancel; the membership runs to the end of the prepaid term and does not renew; **no automatic refund is issued**. The portal shows the end date and offers the studio's contact for a refund request. I will not specify an automated refund of unused months — inventing a money-movement rule where the company has explicitly documented "there is no policy" is exactly the kind of guess that costs real cash and cannot be undone by a patch.
**Stop:** owner of refund policy (Dan / studio owners).
 - *If a refund policy is later set* → it is a follow-on spec; this one already leaves a clean seam (cancel ≠ refund).
 - *If Dan says "auto-refund unused months"* → that needs a payment-provider capability check and a per-studio opt-in; I would spec it separately rather than bolt it on.
 I proceed under **no automatic refund**.

**Collision 3 — Family plans.** Ruling: a linked member may cancel **their own** membership only, and doing so does not touch the payer or the other linked members. The payer may cancel their own membership, which ends the whole plan — but every linked member's access runs to the same end date as the paid period, and each is emailed. This directly answers both Family tickets, and it is the simplest rule that does not silently cut a child's access mid-month.

**Collision 4 — freeze.** The brief bans a retention maze. Four tickets are people who cancelled because they did not know freeze existed. Ruling: **one line of text** on the cancel screen naming freeze and the desk's contact, with no extra click, no interstitial, no confirmation step added. That is information, not a maze, and I would say so in the spec so nobody "improves" it into a funnel later. If Dan reads that as a retention step, deleting the sentence costs nothing and breaks no requirement.

**What I would refuse outright:** a multi-step retention flow, an offer/discount interstitial, or any "are you sure" beyond the single confirm — the brief is explicit and the tickets back it.

## Phase 2 — Load the format skills

I would invoke `mochiko:authoring-user-stories` and `mochiko:authoring-requirements` and take the literal story shape, ID scheme, RFC 2119 keyword usage, and Given/When/Then structure from them rather than from memory or from copying `specs/member-signup/spec.md`. I would cross-check the result against `specs/README.md`'s eight required sections and their order, and against the sign-up spec as the house reference example (it shows the conventions in use: `Source:` lines, `**Independent test:**` per story, measurable SCs).

## Phase 3 — Draft the stories

Planned inventory for `## User Stories`, each with Given/When/Then scenarios and an independent test:

- **US-001 Cancel a rolling membership from the portal (P1)** — the core path. Scenarios: cancel succeeds and shows the exact end date and the exact date/amount of the last charge; **no charge is taken after the end date**; access continues until the end date; cancelling twice does not move the end date (the reset-the-end-date ticket); cancel is reachable from the portal's account area without contacting anyone.
- **US-002 Know exactly when billing stops (P1)** — confirmation email within 2 minutes naming end date, last charge date and amount, whether any further charge will occur, and how to rejoin. Scenario for email failure: the cancellation still stands and is retried; a member is never un-cancelled by a mail problem.
- **US-003 Cancel where the studio has a notice period (P1)** — the end date is computed under the studio's rule and **shown before the member confirms**, with the number of remaining charges named.
- **US-004 Linked member ends their own membership (P2)** — payer and siblings unaffected; payer is notified.
- **US-005 Payer cancels a Family plan (P2)** — all linked memberships end on the same end date, not immediately; every linked member is emailed.
- **US-006 Cancel an annual prepaid membership (P2)** — runs to term end, no renewal, no automatic refund, refund route shown.
- **US-007 Cancel while lapsed or frozen (P3)** — a lapsed member can still cancel; the outstanding balance is stated in the email and is **not** silently written off or auto-charged. A frozen membership can be cancelled; the end date is computed from the freeze end.
- **US-008 Staff see the cancellation (P3)** — appears in the staff app with who cancelled, when, and the end date, so the desk stops re-cancelling by hand and creating the duplicate-notice bug.

Priorities: US-001/002/003 are P1 because without all three the feature either misses the majority of UK studios or reproduces the exact ticket that motivated it.

## Phase 4 — Draft requirements, criteria, assumptions, questions, scope

- **`## Functional Requirements` (`FR-XXX`, each with a `Source:` line)** covering, at minimum: end-date computation for each of the three plan types and for notice/no-notice studios; the hard guarantee that no charge is taken with a date after the end date; access retained until end of day (studio-local) on the end date; single-confirm flow with the end date shown *before* confirming; idempotency — a second cancellation MUST NOT move an existing end date; confirmation email within 2 minutes with named contents; email failure MUST NOT reverse the cancellation; Family scoping (linked cancels self only; payer cancellation ends linked memberships at the same end date); annual behaviour and the absence of automatic refund; lapsed balance stated and not auto-collected; an immutable audit record of actor/timestamp/end date; the cancel control MUST be present in the member portal account area and MUST NOT be gated behind contacting staff.
- Every quantity gets a number: 2 minutes for email, end of day studio-local for access, "no charge dated after the end date" rather than "billing stops".
- **`## Success Criteria` (`SC-XXX`)** — measurable, tied to the tickets: "charged after cancelling" tickets fall to zero in the first full quarter after release (baseline: 9 in Q3 2026); at least 80% of cancellations at portal-enabled studios are self-serve within 3 months; 99% of confirmation emails delivered within 2 minutes; zero cases where a second cancellation moves an end date later.
- **`## Assumptions` (`A-XXX`)** — one entry per Phase 1 ruling plus the small ones: cancellation is irrevocable in the portal and reversal is a desk action; dates are studio-local; a cancellation submitted on the billing day itself does not stop a charge already in flight that day, and the email says so; the `cancelled` state and the one-year portal login retention follow the glossary; rejoin is out of this flow.
- **`## Open Questions`** — exactly two, each with an owner and each with a stated working default so Monday is not blocked: (1) annual prepaid refund policy — owner Dan/studio owners, default no automatic refund; (2) confirmation that portal cancellation honours the configured notice period — owner Dan/contracts, default honour it. I would deliberately keep this list short; anything I could reasonably decide, I decided and put in Assumptions instead.
- **`## Out of Scope`** — member self-serve freeze (the four tickets point at it, but it is a different feature and staff-only today); automated refunds; rejoin/win-back; changing the staff cancellation flow beyond making portal cancellations visible; cancellation of corporate memberships.

**File written: exactly one — `specs/member-cancellation/spec.md`.** No other file is created or edited. Header: feature name, `Status: Draft`, author, date 10 September 2026.

## Phase 5 — Conformance check (the one thing I would delegate)

Deterministic and bounded, so it does not belong in my context: I would spawn a throwaway `Explore` subagent pinned to **haiku** with the brief *"Read `specs/README.md` and `specs/member-cancellation/spec.md`. Report, with line numbers: which of the eight required sections are present, whether they appear in the stated order, and any story/requirement/criterion/assumption whose ID does not follow the `US-XXX`/`FR-XXX`/`SC-XXX`/`A-XXX` pattern or any FR missing a `Source:` line. Facts only, no rewriting."* On return I would check it names all eight sections in order and reports no missing `Source:` lines; anything it flags I would verify myself in the file before changing it, since a cheap reader can miscount.

I would **not** delegate judging whether the spec is right — the notice-period and refund calls are the whole substance of this card.

## Phase 6 — Self-review, then report

I would reread my own draft against three questions: does every acceptance scenario have a pass/fail answer; does every requirement that could be read as "fast" or "clear" carry a number instead; and — the one that matters most — **does each of the four ticket clusters have a requirement that would have prevented it?** Specifically: charged-after-cancelling → the no-charge-after-end-date requirement and the idempotency requirement; could-not-find-cancel → the portal-placement requirement; Family mid-month cut-off → the same-end-date requirement; wanted-to-pause → the single freeze mention (partial; the rest is out of scope and named as such).

**Final report to Dan (short, and honest about the two things I did not invent):** the spec is written and engineering can start Monday; I made every call myself rather than sending back questions. Three things in it are bigger than the brief assumed and I want you to know before it ships, not after: cancellation is *notice*, not instant termination, at the 61 UK studios that configured a notice period, because doing it your way would break those contracts; annual members run to the end of the term with **no** automatic refund, because there is no refund policy to encode and I will not invent one that moves money; and Family cancellation now ends linked members at the end of the paid period rather than the same day, which fixes a ticket we already have. There is one sentence about freeze on the cancel screen — not a retention step, just the thing four cancelling members said they wished they had known; delete it if you disagree and nothing else changes.