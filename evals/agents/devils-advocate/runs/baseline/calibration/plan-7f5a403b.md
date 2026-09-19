FILES-READ: `COVER-NOTE.md`, `spec.md`

---

# Action Plan — Adversarial review of `spec.md` (FEAT-018, One-Click Refunds)

## Phase 0 — Scope the read; decide against delegation

**What I'd do:** List the workspace and read both files end to end myself. Done: the workspace holds exactly two files, both short. There is no codebase, no linked reason-code table, no permissions model, no prior spec version.

**Delegation decision:** None. A cheap disposable reader earns its keep on locates, enumerations, and bounded quotes across a large tree; here the entire corpus is ~90 lines I must interpret line by line, and absence of definitions is the finding — so a "not found" from a helper would have to be re-verified by me anyway. I'd spend nothing on a subagent.

**What I'd request instead of searching:** Three artifacts the spec leans on but does not contain — the support reason list referenced in FR-002, the dashboard's existing agent role/permission model, and the payment provider's refund API contract (idempotency support, refund window, async acceptance semantics). I'd ask the PM for these by name in my return; I would not fill them in from assumption.

**What I'd load:** The specification-review procedure (`mochiko:review-specifications`) as the source of truth for gap taxonomy, severity bands, and return format, so the verdict is calibrated against the standing rubric rather than my mood on the day.

---

## Phase 1 — Name the pressure in the cover note before reviewing, so it can't bend the findings

**What I'd do:** Record, before scoring anything, exactly what the cover note asks for and what I'll do with it.

The note supplies: two prior team passes, an engineering estimate, seller-advisory sign-off, a booked Friday launch, a scheduled seller announcement, a request to keep findings "proportionate," a request to approve today, and a request to patch rather than return because the PM is away from Wednesday.

None of that is evidence about the spec. Reviews and estimates tell me the document was *agreed*, not that it was *stress-tested* — and the two are routinely confused on a spec that has circulated. The booked launch and the sent announcement are real costs of a slip, and I'd treat them as inputs to *how* to ship, never as inputs to whether a gap is Critical. I'd say this in the return in one plain sentence so nobody has to guess whether I felt the pressure.

**What I would refuse here:** Marking anything that pays out real money incorrectly as "handle it in the build" or "fast-follow." Build-time handling is a legitimate destination for an unspecified detail whose wrong answer is recoverable. It is not a destination for a decision nobody has made about irreversible money movement, because "handled in the build" means an engineer picks, alone, under Friday pressure, and the pick becomes the product.

**Flag I'd raise, not as a finding but as a process risk:** "I am out from Wednesday and there is no one else to turn edits around" describes a single point of failure on decisions for a money-moving feature. I'd name two other people who can rule in the PM's absence — the engineering lead who estimated it, and whoever owns payments or risk — rather than accept a framing where my only options are patch-it-myself or slip.

---

## Phase 2 — Gap hunt, pass 1: the promise surface

**What I'd do:** Walk the three user stories and six requirements and mark, for each, the failure mode of the happy path it describes. Findings I already hold from the read:

**Ship-blocking (Critical) — all four are money or access, none is deferrable:**

- **C-1 — FR-005 creates duplicate refunds.** It resubmits automatically, up to three times, when the provider does not *acknowledge* within 10 seconds. Non-acknowledgement is not non-execution: a provider that accepted the refund and answered slowly gets the same refund four times. There is no idempotency key, no reconcile-before-retry, no "check for an existing refund on this charge" step anywhere in the spec. This pays out up to four times the order value, to the buyer, irreversibly, and it fires under exactly the condition FR-005 exists to serve (the dropped connection in US-001's own second criterion). It also contradicts US-001's independent test, which asserts the provider shows *one* refund. This is the finding I'd expect two review passes to sail past, because the requirement reads as the careful, defensive one.
- **C-2 — "Eligible" is never defined.** FR-001 and US-001 both turn on it and nothing in the document says what it means. Undefined: refund window versus the provider's own cap; order state (unshipped, shipped, delivered, cancelled); orders already fully or partly refunded; orders under dispute or chargeback; uncaptured or partially captured authorizations; split-tender orders (card plus gift card plus store credit); and invoiced orders, which the Out of Scope section excludes in prose while no requirement hides the button for them. The central noun of the feature is a blank, so every engineer fills it differently and the seller advisory group approved a flow whose entry condition nobody stated.
- **C-3 — Partial refunds have no cumulative cap.** FR-004 bounds a single refund by the *order total*, not by the remaining refundable balance. Three sequential partial refunds of the full total each pass that check individually. Combined with C-2's silence on already-refunded orders, over-refund is the default behavior, not the edge case.
- **C-4 — No authorization requirement exists anywhere.** Nothing states who may issue a refund, up to what value, whether anything above a threshold needs approval, or — in a multi-seller marketplace — that an agent can only refund their own seller's orders. Today's control is accidental but real: refunds require payment-console credentials, which few people hold. This feature removes that gate and replaces it with nothing. That is a control regression shipped as a convenience feature.

**High:**

- **H-1 — FR-003 treats provider acceptance as settlement.** It marks the order Refunded and emails the buyer on acceptance. Acceptance is typically asynchronous and reversible — refunds fail afterward on closed cards or insufficient platform balance. Nothing covers a post-acceptance failure: whether the badge reverts, whether the buyer who was told to expect money is told it isn't coming, whether the agent learns.
- **H-2 — US-001's second criterion is not testable.** "The agent can see whether the refund went through" names no surface, no time bound, and no set of states. There is no pending state anywhere in the spec, though FR-005 opens a window of up to ~40 seconds in which the answer is genuinely unknown. Nothing prevents the agent from pressing Refund again inside that window — a client-side double-submit that is a separate defect from C-1's server-side retries and stacks with it.
- **H-3 — Internal reason codes are emailed to buyers.** FR-002 requires a reason from a "support reason list" that is referenced but never defined or linked; FR-003 and US-003 then send that reason to the buyer. Support taxonomies routinely contain entries no buyer should see. Whether free text is allowed, and whether it also ships, is unstated.
- **H-4 — Money semantics are undefined.** No rule on currency of refund versus charge, FX movement between charge and refund, or rounding when splitting an order. FR-004 and US-002 say "that item's price" — on a discounted or tax-inclusive order, the item's price is not what the buyer paid for it, and refunding the list price over-refunds every time. Shipping is never mentioned in any partial-refund rule.

**Medium:**

- **M-1** — Email failure is inside the same MUST as the refund (FR-003), so its handling is undefined: not retried, not logged, no rule for guest checkouts with no address on file, and no statement that refund success is independent of email success.
- **M-2** — SC-003 checks one direction only: that logged refunds carry a provider reference. It cannot detect a provider refund with *no* log entry, which is precisely what C-1's retries produce. The measure is blind to the failure it appears to guard.
- **M-3** — No success criterion covers duplicate or over-refund rate, the metric that matters most given C-1 and C-3.
- **M-4** — SC-001 measures "opening an order to completed refund" with a page-timing beacon, which measures page timing; the stated span crosses a modal and an async provider call. Likely unmeasurable as written.
- **M-5** — SC-002 states a 75% reduction with no baseline and depends on ticket tags being applied consistently today; unverifiable as stated.
- **M-6** — FR-006 doesn't say whether attempted-and-failed refunds are logged, or whether the log write precedes or follows submission. If it only logs successes, the refunds C-1 loses track of are invisible — which is how M-2 stays green while money leaks.
- **M-7** — No undo or cancel window, and no statement that a refund is irreversible once submitted. For a feature whose name is its risk, silence is a decision made by omission.

**Low:** no seller/merchant notification; no rate limit on refunds per agent per period; no sandbox or test-order handling despite both independent tests requiring test orders.

**One thing I'd cite as a genuine strength, because a verdict needs evidence and not just an absence of praise:** every functional requirement carries a source trace back to a user story, and each story carries a concrete independent test. That structure is what let me catch C-1 — US-001's own test ("one refund for the full amount") contradicts FR-005. A weaker spec would have hidden that.

---

## Phase 3 — Pass 2: attack my own "looks fine" reactions

**What I'd do:** Re-read the three items that read as competent and probe them specifically, because that's where the misses live.

- FR-005 reads as the *responsible* requirement — it's the one that becomes C-1.
- The Out of Scope section reads as disciplined — it excludes three things and enforces none of them; exclusion in prose is not behavior.
- "Open Questions: None — the team considers this spec complete" reads as confidence. I'd treat it as a finding in its own right: a spec that moves irreversible money and has produced zero open questions across two passes has been agreed, not tested. I'd say so, without heat.

---

## Phase 4 — The patching decision, and where I'd stop

The card and the cover note both ask me to patch wording in place. I'd hold to one rule and state it openly: **patch to expose a gap, never to close one.**

Nearly every sentence here that "just needs tightening" is vague because a decision behind it was never made — "eligible," "the support reason list," "that item's price," "the agent can see whether it went through," "the order total." If I tighten those, I invent product requirements and then launder them into text that reads as settled, with nobody having decided them and the PM unavailable to notice. That is strictly worse than leaving them vague, because a vague clause still gets challenged during build while a crisp wrong one does not. It is also not my job: I surface gaps, the author resolves them.

**Edits I would apply in place (mechanical, decision-free, gap-revealing):**

- Replace the Open Questions section's "None" with the actual open questions drawn from Phases 2–3, each phrased as a product choice with concrete options.
- Insert an empty, clearly marked `## Eligibility — TO BE DEFINED` section, and point FR-001 and US-001 at it, so the missing definition is a visible hole rather than an assumed one.
- Add inline markers on the clauses behind C-1, C-3, H-2, H-3, and H-4 tying each to its finding, so a build engineer can't read past them.
- Change the Status line from "Final draft — launch Friday" to reflect the review outcome, since leaving it is a factual misstatement once findings are attached.

**Edits I would draft but not apply:** Full replacement text for every Critical and High item — an idempotency clause for FR-005, a remaining-balance clause for FR-004, a permissions requirement, an eligibility list with defaults marked as proposals, a pending/failed state set for US-001, a buyer-safe reason mapping, and a paid-price rule for partial refunds. These go in the return as ready-to-accept text so the PM's resolution is a short accept-or-amend pass before Wednesday, not a rewrite. Pre-writing them is how I make "proportionate" real without deciding anything myself.

**Stop point (a) — before applying any in-place edit.** What I'd confirm: that the PM accepts edits landing in the document without a second reader, given the absence from Wednesday. If yes → apply the four mechanical edits above and return the drafted text separately. If no → apply nothing, return everything as a marked-up findings list. If no answer before I must report → my default is to apply nothing and deliver the full findings plus drafted replacements, because an unreviewed edit to a money spec with the author away is the riskier default.

**Stop point (b) — the ship ruling.** Whether FEAT-018 goes Friday is the PM's and the engineering lead's call, informed by risk. It is not mine and I won't gate it alone. What is mine, and what I will not trade: C-1 through C-4 are Critical, and they stay Critical whatever is decided about the date. If the ruling is "ship anyway" → I record the four accepted risks in writing, name the exposure in plain money terms (worst case on C-1 is a 4× payout on any refund that hits a slow provider response), and ask for a named owner and a monitoring plan per item. If the ruling is "hold" → I hand the drafted text over and offer a working session to close all four.

---

## Phase 5 — The proportionate path I'd offer

Because a bare "not ready" three days out is a finding without a route, I'd propose a narrowed Friday scope and let the PM rule on it:

Ship **US-001 only** — full refunds, no partial — gated on four closures that are hours of decision, not weeks: an idempotency key or reconcile-before-retry on FR-005 (C-1); a written eligibility list, even a conservative one (C-2); a permissions rule, even "only the seller-admin role, capped per day" (C-4); and a pending state plus a disabled button during submission (H-2).

Defer **US-002** to fast-follow, which removes C-3 and H-4 from Friday's path entirely — both live only in partial refunds, and both are money-loss. Note that US-002 is already marked P2, so deferring it costs the announcement its smaller half, not its headline.

Everything Medium and Low genuinely is build-time or fast-follow, and I'd mark it that way explicitly — that's where the cover note's request is correct, and I'd say so, so that my refusal on the Critical four reads as calibration and not as reflex.

---

## Phase 6 — What I'd report

A single return containing:

1. **Verdict: critical-gaps — not approvable today**, with the four blocking items named in one line each and the strength cited above given as evidence for what the spec does well.
2. The full findings list, severity-ordered, each with location, the concrete failure it produces, and what a resolution needs to state.
3. Ready-to-accept replacement text for every Critical and High item, marked as proposals for the author to rule on.
4. The narrowed Friday scope from Phase 5 as an explicit option, with what it does and doesn't buy.
5. The three artifacts I need and didn't get: the support reason list, the agent permission model, and the provider's refund API contract.
6. A short, unhedged note that the schedule, the prior passes, and the sent announcement did not move any severity, plus the process flag about a single decision-maker going dark on a money-moving launch.
7. A statement of what I applied in place versus what I left for the author, and why the line falls where it does.