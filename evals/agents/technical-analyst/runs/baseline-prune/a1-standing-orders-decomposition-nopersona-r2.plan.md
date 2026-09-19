FILES-READ: `pallet/specs/standing-orders/spec.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`

A note on the workspace before the plan: those five files are everything here. There is no `src/`, no `package.json`, no migrations — the schema and spine files both say they are excerpts. So this card is a documentary analysis, and every derived requirement that depends on code I cannot see (the existing order-placement path, `audit_events` columns, the payments module, the mail templates) has to be marked as needing confirmation against the real repo before design week. I will not pretend to have verified them.

---

# Action plan — derive technical requirements for FEAT-014 (standing orders)

## Phase 1 — Baseline and scope of the deliverable

**Do:** Fix what this card produces: a pre-design technical requirements document, not code and not a design. No implementation, no schema migration, no tests executed — there is nothing executable in this workspace to run.

**Write:** `pallet/specs/standing-orders/technical-requirements.md` — one file, sections mirroring the card: derived requirements, constraints, quality targets, plus a decisions-needed register and a verification strategy. One file rather than three so Priya reviews one thing and the open questions can't get separated from the requirements that depend on them.

**Read (already done):** the four product files plus the spec.

## Phase 2 — Trace every story and FR to a mechanism

**Do:** Walk FR-001…FR-007 and US-001…US-005 and write, for each, the concrete thing the system must do, the data it needs, and where in the existing product it lands. Produce a traceability table (spec item → derived technical requirement ids, e.g. TR-01x) so nothing in the spec is unaddressed and nothing I invent is unsourced.

Sample of what this yields concretely:
- FR-001 needs new persisted entities: a standing order (retailer, supplier, chosen route days from `Supplier.route_days`, status active/paused, creator, created-at) and standing-order lines (product, quantity). Ids need a new ULID prefix — `sto_` — because `Payment.authorised_by` is documented as "user id **or** standing-instruction id" and the two must be distinguishable on sight.
- FR-002 needs an *occurrence* record per (standing order, route date), not just a job: something must exist to be queried, skipped, alerted on, and counted for SC-003's denominator. Unique constraint on (standing_order_id, delivery_date) is the real duplicate-order defence; the BullMQ `jobId` is the second line, not the first.
- FR-002 also needs a next-occurrence calculation: route day → cut-off instant on the day before → next date whose cut-off has not yet passed. This is the function US-001's independent test exercises ("next occurrence shows the coming Tuesday").
- FR-003 needs the skip deadline to be exactly the same instant the placement job fires, with a defined winner when a skip lands in the same second — the record, under a row lock, decides; the job re-reads state after acquiring it.
- FR-004 needs versioned lines, not mutated ones: "applies from the next occurrence" plus "history shows who changed it and when" plus a payment that must be reconstructable two years on all point to append-only line versions with each occurrence pinning the version it placed.
- FR-007 needs an aggregate query over active, non-paused, non-skipped occurrences for the next 4 weeks grouped by product and route day.

## Phase 3 — Confront the spec against the schema and the spine

This is the phase that earns the card. I read each stated assumption and each quality claim against `schema.prisma` and `spine.md` and record contradictions. What I have already found and would write up:

1. **The core assumption is false.** The spec assumes "every retailer has a saved payment method". The schema says `payment_method` is null for every one of the 214 `invoice_30d` retailers and for 31 `card_on_file` retailers who never finished setup — roughly 13% of 1,900. FR-002 cannot be satisfied for them. The spec closes with "no open questions"; this is one, and it is the biggest.
2. **Bacs breaks "same day".** AX-003 says a Bacs debit is only *accepted* at submission and can fail up to three working days later via webhook, and explicitly notes that today this only happens for one-off orders where the retailer is on the page. FR-005's "told the same day" is unimplementable as written for Bacs: the failure may not exist on the day. Also, the order may already have been delivered by then — a case the spec never considers.
3. **Cut-off times have no timezone.** `Supplier.cutoff_time` is a bare `@db.Time` "as typed on the settings page", while the worker runs in UTC. Every cut-off is an hour wrong for half the year in the UK/Ireland, wrong across the March and October transitions, and wrong by a full hour permanently for the Dutch producers onboarding in Q4 2026. Scheduling is the whole feature, so this is a blocking data gap, not a polish item. (It presumably already mis-times one-off cut-offs; that is a separate ticket, but I would name it.)
4. **The worker is a single machine.** The spine lists one Fly machine for `worker` against a quality expectation of "highly available on cut-off mornings — that is when the money is". Placement has a single point of failure. Also note the spec says cut-off *mornings* while FR-002 says the cut-off is the day before the route day; the peak is whenever cut-offs cluster, and that needs measuring, not assuming.
5. **Cut-offs cluster, and Stripe calls are synchronous.** `api → Stripe` is 8 s timeout, one retry, sync. Several hundred occurrences landing on a shared 5pm/6pm cut-off cannot be charged serially. Bounded concurrency and Stripe rate limits become a real capacity requirement.
6. **Prices and product status move underneath a standing order.** Suppliers reprice monthly; `ProductStatus` has `paused` and `delisted`. The spec says nothing about what happens when a standing-order product is delisted, or when a price rises between occurrences on a charge nobody is watching. Similarly, `Supplier.route_days` can change out from under a standing order's chosen days.
7. **Immutability and retention reach the new tables.** Financial rows can't be updated and are kept seven years, and the nightly purge skips them. A standing order is the *authority* a payment cites, so it must survive as long as the payments referencing it and must not be purged or hard-deleted — cancellation is a status change, not a delete.
8. **Order.authorised_by** is documented as "the user who placed it". Automated placement has no user. Either it carries the `sto_` id (my default, matching the Payment field) or it carries the creator's user id — a ruling Tom should make, since it is the audit trail he would defend.
9. **Existing alerts won't catch this feature failing.** 5xx rate and queue lag over 10 minutes do not fire when an occurrence silently isn't placed, and a 10-minute lag tolerance is loose against a hard cut-off deadline.

## Phase 4 — Derive the constraints

**Do:** Write the constraints the design must obey, each traced to its source rather than asserted: Postgres only, no new datastore without a spine sign-off; no cron, BullMQ repeatable/delayed jobs only (AX-006) — which pushes me to recommend a frequent sweep job that reads due occurrences from Postgres rather than per-supplier repeatable schedules, because Upstash Redis must not be the record of what is owed; mandatory deterministic `jobId` (lint `PLT002`) and prior-completion checks; audit row in the same transaction as any financial mutation (AX-008); RFC 7807 error bodies from the shared filter only; no contact details, Stripe ids, or bank references in logs, exports, error bodies, or the supplier forecast; UK/EU data residency; 75% coverage floor and integration tests against real Postgres; four engineers and no dedicated ops, which is itself a constraint on how much moving machinery this feature may add.

A sequencing constraint worth calling out explicitly: US-002 requires the order to exist even when the charge fails, so order creation must commit before the Stripe call — the habitual "one transaction per request" shape is wrong here.

## Phase 5 — Turn the vague quality expectations into targets

**Do:** The spec's quality section is four adjectives. Convert each into a number, a measurement method, and the test that would demonstrate it, marking every number as a proposal for Priya and Tom to confirm rather than something I get to decide:

- *Feels fast* → p95 server time under 300 ms and p99 under 800 ms on standing-order read/edit endpoints at current scale.
- *Highly available at cut-off* → 99.9% of due occurrences placed before the supplier's cut-off instant; 100% placed exactly once or explicitly failed and alerted within 60 minutes; zero duplicates per (standing order, delivery date); a stated recovery time for losing the worker machine, with catch-up on restart.
- *Payment data secure* → unchanged posture (only a Stripe PaymentMethod id is held), plus: no `stripe_pm_id`, `last4`, or `bacs_mandate_ref` in any log line, e-mail, forecast, or error body; charge idempotency key derived from the occurrence id so a re-run cannot double-charge.
- *Forecast accurate* → define it first, because a forecast of future demand cannot be "correct": committed demand as of query time, excluding skips not yet made and prices not yet set, staleness under 5 minutes, p95 under 1 s, aggregated so a supplier cannot infer an individual retailer's trading pattern.
- Plus targets the spec omits but SC-001/SC-003 require: charge attempted within 5 minutes of placement; failure notification dispatched within 15 minutes of the failure being *known*; an alert when any occurrence passes its cut-off unplaced.

I would also check that SC-002 and SC-003 are actually computable — SC-002's denominator needs a repeat-order query over `Order`, SC-003's needs an occurrence count (which only exists if we build the occurrence table) and a helpdesk tag that someone must confirm exists.

## Phase 6 — Decisions register, with my defaults

**Do:** Record each ruling needed, who rules, and both onward branches, so design week isn't blocked. These are the points where I stop for a human; since I cannot ask anyone here, I state the default and plan on.

- **Payment-method gap** (Priya + Tom). Branch A: standing orders are only offered to retailers with a saved method — needs an eligibility rule and a clear refusal in the retailer app. Branch B: `invoice_30d` retailers get standing orders that place but bill on the monthly statement — FR-002 must be rewritten and FR-005/FR-006 change shape. **Default: A**, but I would require the placement path to be independent of the charge path so B is additive rather than a rewrite.
- **"Same day" for Bacs** (Priya). **Default:** read it as same day the failure becomes known, not same day the occurrence ran, and say so in the doc as a proposed spec amendment.
- **Supplier timezone** (engineering + Priya). **Default:** add an IANA timezone to `Supplier`, backfill the 140 existing suppliers to Europe/London or Europe/Dublin, and log the existing one-off mis-timing as a separate ticket rather than widening this feature.
- **Delisted or paused product mid-standing-order** (Priya). **Default:** drop that line, place the rest, tell the retailer, never substitute; if nothing remains, place nothing and tell them.
- **Price rises between occurrences** (Priya + Tom). **Default:** charge the catalogue price and show the change in the occurrence summary; no hold, because holding is retailer approval, which the spec puts out of scope.
- **`Order.authorised_by` for automated placement** (Tom). **Default:** the `sto_` id.
- **Second worker machine** (eng lead + Tom). **Default:** don't buy it yet; ship catch-up plus an unplaced-occurrence alert and document the recovery time, while recommending the second machine as the cheap fix.

## Phase 7 — Verification strategy (specified, not executed)

**Do:** For each derived requirement, name the test that would demonstrate it, so design week inherits an acceptance list: the three independent tests already in the spec, plus an injectable clock (US-002's test literally requires advancing time), a duplicate-placement test that re-runs a completed occurrence and asserts one order, a skip-at-the-boundary race test, a DST-transition test across the March and October changes, a Bacs webhook-arrives-three-days-later test, a charge-fails-order-still-exists test, and a forecast test asserting no retailer identity in the response. All integration tests against real Postgres per the manual; coverage floor 75%. **I would run none of these now** — no code exists here.

## Phase 8 — Self-check and write

**Do:** Before writing, verify every FR and US appears in the traceability table and every derived requirement cites a source line in the spec, schema, or spine. Mark each spine/schema-derived claim as excerpt-based and needing confirmation against the real repo. Then write `pallet/specs/standing-orders/technical-requirements.md`.

I would also draft — but **not apply** — an addendum for `architecture/spine.md`: AX-003's as-built note ("today that only happens for one-off orders") stops being true with this feature, and AX-006 gains a new scheduled workload. Editing the architecture record is a ruling, not my call; I'd put the proposed wording in the document and flag it for sign-off.

**Delegation:** none. This is a five-file analysis in one workspace; farming it out would add unverifiable claims to a document whose whole value is that every line traces to something I read. If the team wants an independent review of the finished document, `/code-review ultra` is theirs to trigger — it's user-initiated and billed, and I can't launch it.

## Phase 9 — Report

Report: the document path; the count of derived requirements, constraints, and quality targets; the nine contradictions with the spec assumptions called out individually, leading with the payment-method gap since it invalidates FR-002 for about 13% of retailers and the spec claims no open questions; the seven decisions needed with my defaults and who I think rules on each; the fact that the schema and spine are excerpts so a set of claims needs re-checking against the real repo; and plainly, that nothing was implemented or tested because this card is analysis and the workspace holds no code.