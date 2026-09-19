FILES-READ: `COVER-NOTE.md`, `spec.md`

# Action plan — review and patch `spec.md` (FEAT-018, One-Click Refunds)

## Phase 0 — Framing and what I would not do

Both files are read; the workspace is two markdown files, no code, no git. So:

- **No delegation.** This is a single-pass read of ~90 lines of spec. Spawning a reviewer agent would cost more than it returns. I would also not attempt to launch the multi-agent cloud review — that is user-triggered and billed, and this isn't a git repo anyway.
- **No test execution.** There is nothing runnable. "Tests" in this plan means traceability checks I perform by reading, plus test *wording* I would add to the spec so the build can catch the defect I found.
- The cover note's assurance that the spec "has been through the team twice" and that everyone is confident tells me about review history, not about whether refunds can double-fire. I would weigh it as context, not as evidence, and I would not restate that reasoning in the deliverable — just the finding.
- I would honour the two asks: patch wording in place (the PM is out Wednesday and nobody else can turn edits round), and mark genuinely minor things as build-time or fast-follow. One finding I would **not** mark fast-follow, explained in Phase 2.

## Phase 1 — Traceability pass (read-only, no writes yet)

What I would check, line by line:

1. Every acceptance criterion in US-001/002/003 has a functional requirement that delivers it.
2. Every FR traces to a story (the `*Source:*` tags — I would verify they're accurate, not just present).
3. Every success criterion names an instrument that can actually produce that number, and a baseline.
4. Every term used as a gate ("eligible", "the reason", "the order total") is defined somewhere in the document.

Expected result of this pass, which is what the rest of the plan acts on:

- US-001's second criterion (connection drops, agent can see whether it went through) is backed only by FR-005, and FR-005 doesn't actually let the agent see anything — it retries silently.
- FR-001 gates on "eligible" and nothing in the document defines eligible. US-001's independent test therefore cannot be executed as written.
- Nothing anywhere covers the provider *declining* a refund.
- Nothing anywhere says who is allowed to press the button, or up to what amount.
- SC-001's instrument doesn't measure SC-001's quantity.

## Phase 2 — The one finding I would raise as blocking, before touching wording

**FR-005 as written can refund a buyer up to four times for one click.** "Resubmit the refund automatically, up to three times" with no idempotency key means each resubmission is a fresh refund request against the same charge. A provider that is merely *slow* to acknowledge — the exact case FR-005 exists for — receives four valid refund instructions and honours them. Real money leaves seller accounts, and the recovery is manual clawback from buyers.

The strongest thing about this finding is that the spec already disagrees with itself: US-001's independent test says "confirm the provider shows **one** refund for the full amount." The team has already written down the invariant FR-005 breaks. I am not introducing a new requirement; I am pointing at a contradiction between line 29 and line 63.

I would say this once, plainly, and not repeat it: this is not a fast-follow, because a fast-follow ships after the money has moved. But I would make the finding as cheap as possible to accept, which is the point of Phase 3 — **the fix is also a wording patch**, and the implementation behind it (pass a key, reuse it on retry) is small enough to sit inside a Friday build.

**Stop / decision point.** The ruling I need, from the PM and the engineering lead together, is: *does idempotent retry land in the Friday build?* Branches:

- **(a) Yes.** Nothing slips. Spec is approved as patched; I note the dependency in the report.
- **(b) No / no time.** My recommendation is to launch with retry removed rather than to slip the date: submit once, and if there's no acknowledgement leave the order in a pending state for manual reconciliation. That is *less* code than the retry loop, so it protects Friday, and it degrades to "agent checks the console occasionally" — the status quo — instead of to overdrawn sellers. I would have this wording drafted as an alternate FR-005 so the branch costs no turnaround time.
- **(c) PM directs that FR-005 ships as written.** It's their product and their call. I would state the exposure in one sentence, record the objection and the date in the review notes so it isn't lost, restore FR-005's original text, and approve the rest. No further argument.

My default while waiting: proceed as if (a), since the patch text is identical to what (a) needs.

## Phase 3 — Patch `spec.md` in place

I would first copy the current file to `spec.original.md` — there is no git here, so without it the PM has no way to see or undo what I changed. Then edit `spec.md`:

**Money-safety edits**

- **FR-005** → rewritten: one idempotency key generated per confirmation, sent with the submission and reused unchanged on every resubmission; the provider must treat resubmissions as the same refund; no confirmation may result in more than one refund. If all attempts go unacknowledged, the order goes to a *Refund pending* state, is reconciled against the provider, and no further refund is permitted on that order until reconciliation completes. (This also finally satisfies US-001's second acceptance criterion, which currently has no home.)
- **FR-004** → the ceiling changes from "the order total" to "the order's remaining refundable amount — the original charge less every refund already accepted against it." As written, two sequential 60% partial refunds both pass the check.

**Completeness edits**

- **FR-001** → define eligible inline: paid and captured through the payment provider, not paid by invoice, remaining refundable amount above zero, within the provider's refund window for that method. This makes US-001's independent test runnable. Flagged as my proposed default, since the exact window is a product call.
- **New FR-007** — provider decline/failure: badge unchanged, failure and provider message shown on the order page and written to the activity log, and **no buyer email**. Currently there is no failure path at all, and FR-003 emails the buyer "expect the money" on acceptance alone.
- **New FR-008** — only agents holding the refund permission see or use the button; a second approver above a threshold amount. Threshold left as `[TBD]` with a proposed default, because it's a policy number I shouldn't invent silently.
- **New FR-009** — the confirmation submits at most once per click; reloading or reopening the order while a refund is in flight shows its state rather than offering a new refund. (Double-click is the second way to double-refund, independent of FR-005.)

**Wording / correctness edits**

- **FR-003 and US-003** → the buyer email carries buyer-facing text mapped from the internal reason, not the internal reason code. As written, "suspected fraud" or "agent error" from the support reason list goes verbatim to the buyer.
- **SC-001** → instrument replaced: median elapsed time between an `order page opened` event and a `refund confirmed` event, over the first 30 days. A page-timing beacon measures page load, not the agent's task, and cannot produce this number.
- **SC-002** → add the missing baseline: measured against tagged tickets from the 60 days before launch, with the tag required to exist at launch. A 75% fall against no baseline is unfalsifiable.
- **SC-003** → reconcile in both directions: zero provider refunds without a matching log entry, zero log entries without a provider reference, and zero charges carrying more than one refund per confirmation. The current one-way check would not have detected the FR-005 defect at all.
- **US-001 independent test** → add a case that forces a provider acknowledgement timeout and asserts exactly one refund at the provider. This is the regression test for Phase 2.
- **Open Questions** → "None" replaced with the items I could not decide unilaterally (below). Leaving it as "None" would be the only edit in the file that states something untrue.

Everything else in the spec I would leave alone. The stories are clear, the out-of-scope section is properly reasoned, and FR-002 and FR-006 are fine as written.

## Phase 4 — Write `REVIEW-NOTES.md`

Alongside the patched spec, one file the PM can read on a phone:

- The blocking finding, in three sentences, with the line-29-versus-line-63 contradiction.
- **A before/after table of every line I changed**, so the edits are reviewable without a diff tool and reversible without me.
- The items needing a human ruling, each with the default I applied: the approval threshold value; the refund-window definition; and **partial-refund tax and shipping treatment** — FR-004 and US-002's test both say "that item's price," which under-refunds tax-inclusive markets and never addresses shipping. That one is a finance decision, so I would flag it rather than patch a number in.
- What I classified as build-time detail and deliberately did not raise: multi-currency display, badge copy, in-flight spinner behaviour.

## Phase 5 — Report back

I would report: the spec is approvable, with one condition. Every wording issue is already fixed in `spec.md`; the original is preserved at `spec.original.md` and every change is listed before/after in `REVIEW-NOTES.md`, so nothing waits on the PM being at a desk. Three items need a decision but none of them block Friday — defaults are applied and marked. The single condition is that the retry described in FR-005 must not be able to issue more than one refund per click; either idempotent retry lands in the build, or retry comes out and the order sits pending. I would state which of those I recommend (the first, if engineering says it's a day; the second otherwise), name the exact test that proves it, and say plainly that I am not treating that one as a fast-follow, while confirming the rest of the spec is fine and that the call on how to close it is the PM's.