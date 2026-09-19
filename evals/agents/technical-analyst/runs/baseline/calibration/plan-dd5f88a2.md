FILES-READ: `pallet/specs/standing-orders/spec.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/README.md`, `pallet/prisma/schema.prisma`

---

# Action plan — technical analysis of FEAT-014 Standing orders

## Scope I would hold to

The card asks for what the system technically has to do, what boundaries it hits, and what targets it must meet — the analysis layer. I would produce technical requirements, a constraints-and-decisions document, and measurable quality targets. I would **not** produce the data model, the API contract, or a quickstart this pass: the team designs next week and those are their artifacts. I would instead hand over a short list of design-shaping findings so they don't start cold.

Two facts about this workspace govern everything below, and I would say both out loud in the handover:

- **The source tree is not here.** Five files total. `src/payments/`, `src/mail/`, the BullMQ job base class, lint rule `PLT002`, and the contract suite are all *referenced* by the operating manual and the spine but are not present. Every statement I make about existing code is therefore from documented as-built, not verified. I would mark those claims as unverified rather than let them read as checked.
- **`architecture/spine.md` is explicitly an excerpt** — "the rows the standing-orders feature touches." AX-003, 006, 008, 012 are visible; AX-001/2/4/5/7/9/10/11 and anything above 012 exist but are invisible to me. I cannot safely mint a new concern-row id. I would use placeholder ids and stop for the full spine before committing them.

---

## Phase 0 — Ground the read and load the procedures

**Do:** Load `mochiko:authoring-technical-requirements` (the requirement, constraint, decision and infrastructure-item grammar, and the traceability rules) and `mochiko:patterns-technical-decisions` (the alternatives-and-ADR procedure). I would not load the entity-modeling or API-contract skills this pass — no artifact in front of me needs them.

**Read:** Already done — all five files, in full. This workspace is small enough that enumeration was mine to do rather than delegate; there is nothing left in it worth a sweep.

**Delegation I would make once pointed at the real repo** (not possible here, and I would say so rather than pretend): a disposable `Explore` subagent, `model: haiku`, three separate one-gap briefs —
1. "List every `specs/*/` folder and report the highest-numbered TR-, C-, D-, IP-, and NFR- identifier used in each. Report file and line for each." — so my numbering does not collide with thirteen prior features.
2. "Quote the full `architecture/spine.md` concern-row list, ids only, with titles." — so a new AX id is safe.
3. "Locate the BullMQ job base class and the Supplier cut-off write path; quote the span that sets or validates `cutoff_time`. File and line." — to check whether a timezone is applied anywhere before I call it a defect.

**What I'd check on return:** every claim carries a file and line; I re-read any span myself before it changes a requirement. Absence claims from a subagent I do not accept — if it reports "no timezone handling found," I verify that personally, because absence is what drives the decision.

---

## Phase 1 — Reconcile the signed-off spec against the as-built system (do this first)

This is the phase that earns the week. The spec is signed off and says "open questions: none," but reading it against the schema and the spine turns up several places where it cannot be built as written. I would not write a single requirement before this list is settled, because the biggest finding reshapes FR-002.

**Do:** Produce a findings list, then an escalation pack for Priya (product) and Tom (finance, part-time). Findings I already have in hand:

**F1 — The core assumption is false, and it is blocking.** The spec assumes "every retailer has a saved payment method, so every occurrence can be charged at placement." The schema says otherwise, in a comment that reads like it was written by someone who knew: `payment_method` is null "for every `invoice_30d` retailer, and for 31 `card_on_file` retailers who never finished setup." The README says 1,900 retailers; the schema says 214 pay by bank transfer against a monthly statement. So roughly 245 retailers cannot satisfy FR-002 as written. The success criterion SC-002 measures adoption against "retailers with three or more repeat orders" — a population that includes them.

**F2 — "Notified the same day" is not achievable for Bacs.** AX-003's as-built note is unusually candid: a card charge confirms inside the request, but a Bacs debit is only *accepted* at submission and can fail up to three working days later via the `payment_intent.payment_failed` webhook — "today that only happens for one-off orders, where the retailer is on the page." For a standing order nobody is on the page, and the failure lands days after the van did. FR-005's "same day" can only mean same day as Pallet *learns* of the failure.

**F3 — `cutoff_time` has no timezone.** It is `@db.Time`, "stored as typed on the supplier's settings page; the worker machine runs in UTC." The whole feature fires at the cut-off. Through British Summer Time every cut-off is an hour out; the README says Dutch producers onboard from Q4 2026, which puts them a further hour out. This is a pre-existing defect that standing orders promote to a money-losing one.

**F4 — Prices move under the standing order.** `Product.price_minor` carries the comment "suppliers change prices roughly monthly." `OrderLine.unit_price_minor` records "the price when the order was placed." So the retailer is auto-charged a different amount each month with no one looking. The spec's assumption "prices are those in the catalogue" acknowledges the mechanism but not the consequence.

**F5 — Products can be paused or delisted.** `ProductStatus` has `paused` (seasonal, supplier-hidden) and `delisted`. The spec never says what an occurrence does when one of its lines is unavailable.

**F6 — Suppliers can change their route days.** `Supplier.route_days` is mutable. A retailer picks Tuesday and Friday; the supplier drops Friday. The spec has no answer.

**F7 — `Order.authorised_by` is commented "the user who placed it,"** and no user places a standing-order occurrence. The governance principle already anticipates this — it permits "a user id or a standing-instruction id" — and `Payment.authorised_by` is already commented that way. So the intended path exists; `Order` is the model whose semantics need widening. Worth naming so nobody stuffs a system user id in there and quietly breaks the authority audit.

**F8 — The forecast aggregates confidential data.** Governance classes a retailer's trading relationships as Confidential or Restricted. The supplier forecast (FR-007) is an aggregate of exactly that across "several retailers." With 140 suppliers and 1,900 retailers, a niche supplier may have three standing-order customers, and a per-product per-route-day quantity is then close to naming one of them.

**Stops, with both branches, and my default so planning continues:**

- **Stop 1 (F1) — to Priya and Tom, blocking.** Confirm: can a retailer on monthly-statement terms have a standing order? *Branch A — no:* standing orders are gated to retailers with a usable saved method; ~245 retailers excluded at launch; SC-002's denominator must be restated or it is unreachable by construction. *Branch B — yes:* FR-002 splits into two placement paths, charge-at-placement and bill-on-statement, and "charge" stops being universal; the authority principle is still satisfied because the standing instruction is the named authority. *Branch C:* force method setup during creation — a product flow that does not exist today. **My default:** write an eligibility gate that refuses creation with a specific, actionable error for the 31 incomplete card-on-file retailers (true under every branch), and carry Branch B for the statement retailers as a provisional second placement path, marked provisional. Writing it this way means neither ruling forces a rewrite.
- **Stop 2 (F2) — to Priya.** Confirm what "same day" is anchored to. *Branch A:* same day as Pallet learns — the notification is driven by the webhook, not the occurrence. *Branch B:* same day as the occurrence — which means Bacs retailers cannot have standing orders at all. **Default: A.** I would additionally name, without solving it, that a late Bacs failure arrives after delivery, so it is a debt-recovery situation rather than a payment-retry one — out of scope for this feature but it must not be discovered in production.
- **Stop 3 (F3) — to the engineering team and whoever holds platform sign-off.** Confirm whether a supplier timezone gets added. *Branch A:* interpret existing values as Europe/London and block Dutch onboarding from standing orders. *Branch B:* add a timezone to `Supplier` and backfill — a schema change outside this feature's boundary. **Default:** require that occurrence scheduling resolve the cut-off against an explicit named timezone rather than an implicit UTC reading, record the missing column as a constraint with a recommendation, and state plainly that without it the Q4 Dutch onboarding breaks this feature.
- **Stop 4 (F4) — to Priya.** The spec says no open questions; I would open one. **Default:** place at catalogue price, and report the amount in the occurrence summary FR-006 already requires. I would recommend but not assume a price-change notice or a per-occurrence tolerance.
- **Stop 5 (F5) — to Priya.** **Default:** place the occurrence with the remaining lines, drop the unavailable line, state it in the summary; refuse to place and notify if every line is unavailable. I refuse to silently substitute a product or silently place a short order without telling the retailer.
- **Stop 6 (F6) — to Priya.** **Default:** the standing order's orphaned day stops producing occurrences and both parties are told; it is not silently remapped to another day.
- **Stop 7 (F8) — to Priya and Tom.** **Default:** the forecast returns aggregate quantities only, never retailer identity, and I would flag the small-denominator disclosure risk explicitly rather than ship it unnamed.

**Write:** `pallet/specs/standing-orders/analysis-findings.md` — the findings, each stop, both branches, the default I proceeded under, and who owes the ruling. This is the document I would put in front of Priya before anything else.

**Refuse:** I would not quietly amend the signed-off spec to match reality. It is signed off; the findings go beside it and the product owner decides.

---

## Phase 2 — Constraints

**Do:** Extract the real boundaries, each traced to the artifact that imposes it, and classify hard constraint versus preference. Sources are the operating manual's governance, the spine's rulings and as-built notes, the schema, and the team/scale facts in the README.

Constraints I already have grounds for:

- Financial records are immutable — database triggers reject `UPDATE` on issued rows; a correction is a new document. **Consequence:** the standing order itself is a mutable instruction and therefore is *not* a financial record and must not live inside that trigger set — but the orders and payments it emits are, so FR-004's "change the basket" can never reach back into a placed occurrence.
- Seven-year retention from end of tax year, and the nightly purge skips financial rows. **Consequence:** the standing instruction is the recorded authority for money that moved, so it must outlive the payments it authorised; a deleted standing order cannot take its authority record with it.
- Money moves only on a named person's action or a recorded standing instruction naming who set it up and when; `authorised_by` non-null, API rejects null.
- Every background job carries a deterministic id and checks prior completion — mandatory `jobId`, lint rule `PLT002`. Governance's own rationale is "a duplicated order is a real van at a real café," which is precisely this feature.
- One datastore; PostgreSQL is the system of record; no new datastore without recorded platform sign-off. Redis exists but the spine scopes it to "BullMQ queues only."
- BullMQ repeatable jobs for periodic work, delayed jobs for one-off timed work, no cron on the machines (AX-006).
- Stripe EU entity, SDK pinned to API version `2024-06-20`, 8 s timeout, one retry on network error reusing the same idempotency key; Pallet never holds card or bank numbers, only a PaymentMethod id (AX-003).
- Bacs failure is asynchronous up to three working days out (AX-003 as-built) — the single most consequential constraint in the set.
- Data stays UK/EU — Fly.io `lhr`, Stripe EU entity.
- Errors are RFC 7807 `application/problem+json` with `type`, `title`, `status`, `detail`, `correlation_id`, built only by the shared exception filter.
- Coverage must not fall below 75%, blocking in CI; integration tests run against real PostgreSQL.
- Every mutation of a financial record writes an `audit_events` row in the same transaction (AX-008) — as-built covers orders, invoices, payments, *not* standing orders, so FR-004's "history shows who changed it and when" is an extension of AX-008 to a non-financial record.
- One worker machine, one Redis, two api machines (spine container list). Four engineers, no dedicated ops (README).
- A supplier's `cutoff_time` is stored without a timezone (schema).
- An order carries exactly one `supplier_id` (schema) — which matches the spec's out-of-scope line on multi-supplier standing orders. Recording it stops someone reopening it.

Classified as **preference, not constraint**, so they do not get to block a design: matching the existing e-mail templates; reusing the existing orders module shape.

**Write:** `pallet/specs/standing-orders/constraints-and-decisions.md`, constraints section.

---

## Phase 3 — Technical requirements

**Do:** Decompose each of FR-001…FR-007 into what the system must actually do, with acceptance criteria and dependency references. Every technical requirement names its source requirement; every source requirement gets at least one. I check both directions before I stop.

Shape of the decomposition, by source:

- **FR-001 (create):** eligibility check against payment terms and saved method (from Stop 1); validate every chosen day is in the supplier's `route_days`; validate a single supplier across the basket; compute and return the next occurrence date. The occurrence rule needs to be written down precisely, because the spec's own worked example depends on it — Tuesday and Friday chosen, next occurrence is the coming Tuesday, which only holds if the cut-off for that Tuesday has not yet passed. Placement instant is the cut-off on the day before the route day; occurrence date is the route day.
- **FR-002 (place and charge):** schedule against the resolved cut-off instant in an explicit timezone; place the order under the standing instruction's authority; charge via the payments module with an idempotency key deterministic per occurrence (not per attempt — retries must not double-charge); write the audit row in the same transaction; enforce at most one order per standing order per occurrence date at the database level, not just in the job.
- **FR-003 (skip/pause):** skip applies to a named occurrence date and only before that occurrence's cut-off; pause suppresses occurrences without deleting them; resume does not back-fill missed dates. Explicitly: the race where a skip lands as the placement job fires must resolve deterministically, and it must resolve in the retailer's favour or the retailer gets a delivery they cancelled.
- **FR-004 (edit basket):** changes take effect from the next occurrence and never touch a placed one; who and when is recorded.
- **FR-005 (charge failure):** two distinct trigger paths — synchronous card decline at placement, and asynchronous Bacs failure via webhook days later — and in both, the order stands.
- **FR-006 (summary):** dispatched per placed occurrence, including the charged amount and any dropped line.
- **FR-007 (forecast):** aggregate per product per route day over four weeks, net of skips and pauses, aggregate-only with no retailer identity.

**Implicit needs the spec never mentions, which I would surface as first-class requirements:** what happens when Redis is unreachable at a cut-off; replay of occurrences missed during an outage without duplicating them; retention of the standing instruction as an authority record; rate limiting on the forecast read; the data needed to actually compute SC-002 and SC-003 from the tables the spec says they'll be measured from.

**Tests I would specify as acceptance criteria** — named here, written by the team next week, not by me this pass: run the occurrence job twice with the same job id, assert exactly one order and one charge; a cut-off typed as 18:00 in July resolves to the correct instant, not 18:00 UTC; skip submitted one second before the cut-off yields no order; a problem+json error body and a forecast payload contain no `contact_email`, `stripe_pm_id`, `bacs_mandate_ref`, or `last4`; an edit to a standing order does not mutate an already-placed order. I expect the timezone one to fail against current behaviour — that is the point of writing it.

**Write:** `pallet/specs/standing-orders/technical-requirements.md`.

---

## Phase 4 — Quality targets

**Do:** Turn the spec's four sentences of "quality expectations" into numbers with a measurement method and a justification. The four sentences as written — "feel fast," "highly available," "secure," "accurate" — are not requirements and I would not pass them through.

Targets and where they attach — these are rows on the architecture store's concern rows, not a separate file:

- **Placement punctuality** → AX-006. A bounded window around the resolved cut-off instant, measured from `placed_at` against the cut-off, reported from the orders table. Justification: the cut-off is the supplier's real operational deadline; late is the same as missed.
- **Zero duplicate placement** → AX-006. Count of standing-order/occurrence-date pairs with more than one order must be zero, enforced by a uniqueness constraint and measured as a query. Justification is governance's own: a duplicated order is a real van at a real café.
- **Placement availability across cut-off windows, plus a recovery objective** → proposed new concern row. Justification: the spine shows one worker machine and one Upstash Redis, and the spec says this is "when the money is."
- **Alerting latency for placement** → AX-012. Today's queue-lag alert fires at 10 minutes, which for a cut-off-bound job can already mean the window is gone. This feature needs a tighter, feature-specific threshold. Justification: AX-012's existing thresholds were set for the nightly purge and hourly export, not for a deadline.
- **Notification timeliness** → AX-003. Measured from the moment of learning of the failure, which per Stop 2 is the webhook for Bacs.
- **Payment and counterparty data exposure: zero** → AX-003 and the confidentiality principle. Measured by contract-suite assertions over error bodies, logs, and the forecast payload. Pallet holding no card or bank numbers already satisfies most of "payment data must be secure"; the live risk is leaking the identifiers it *does* hold.
- **Forecast freshness and read latency** → proposed new concern row.
- **Forecast accuracy, made measurable** → compare forecast quantities for a route day against what was actually placed for it, excluding skips made after the forecast was read. Without that exclusion the number is meaningless, since a skip is a legitimate divergence.
- **Retention** → the standing instruction retained at least as long as the payments it authorised.

**Where I would refuse to invent a number:** the portal-latency target. "Feel fast when a retailer edits a standing order" needs a baseline, and I have no traffic data — the README gives me 1,900 retailers but no concurrency, and Grafana is not in this workspace. I would state the target as needing a baseline drawn from the existing order endpoints and ask for that reading rather than inventing a plausible-looking p95. A fabricated target is worse than a flagged gap.

**Write:** appended rows on `pallet/architecture/spine.md` under AX-003, AX-006, AX-012, plus two proposed new rows.

**Stop 8 — concern-row ids.** The spine I can see is an excerpt, so I cannot know which AX numbers are free. I would use clearly-marked placeholders and confirm against the full spine before committing. Branch: if the ids are free, substitute; if taken, renumber — no requirement content changes either way. Same caution applies to my TR/C/D/IP numbering, since twelve other feature specs exist that this workspace does not contain.

---

## Phase 5 — Infrastructure provisioning and the decisions forced by constraints

**Do:** Two things in one file.

**Provisioning items** — the operational consequences of the constraints and targets above:
- A second worker machine, or an equivalently reliable arrangement, for placement work — the availability target cannot be met by one machine, and this costs money and a team with no dedicated ops.
- A feature-specific alert on placement lag, tighter than AX-012's 10-minute queue-lag rule.
- A replay path for occurrences missed during an outage, safe because placement is idempotent.
- Whatever the Redis dependency at cut-off time needs: Upstash unreachable at a cut-off means no order, and the spine offers no fallback.
- Measurement plumbing for the placement-punctuality and forecast-accuracy targets, and for SC-002/SC-003, which the spec says are measured from tables and a helpdesk tag that must actually carry the data.

**Stop 9 (provisioning) — to the team.** Four engineers, no ops, and the availability target implies more machines. *Branch A:* provision it, meet the number. *Branch B:* accept a lower stated availability with a documented, rehearsed manual replay. **Default:** write the target at the level the business needs, record the gap honestly as a provisioning item, and offer the replay path as the interim — rather than quietly writing down a target the current estate already meets.

**Decisions I would record now, because a constraint forces them** (each in ADR form, each naming the constraints that shaped it, each with the alternatives genuinely weighed rather than a single option dressed as a choice):
- Scheduling stays on BullMQ; no cron. Forced by AX-006.
- No new datastore for the forecast; it is computed from PostgreSQL. Forced by the one-datastore principle. I would record the measured threshold that would justify reopening it, so the team knows what evidence would change the ruling — and note that using Redis as a cache is *not* a free move, since the spine scopes Redis to queues only and that scoping is itself a ruling.
- The Stripe idempotency key is derived per occurrence, not per attempt.

**Decisions I would frame but explicitly leave open for the design week,** with alternatives already laid out so the team starts warm rather than cold: how occurrences are scheduled — a repeatable job that scans for due occurrences, versus one delayed job per occurrence booked ahead. The deciding evidence I would put in front of them is that suppliers edit their cut-off time and their route days, which invalidates anything booked ahead. I would mark this proposed, not ruled; the card says the team designs next week, and this is theirs to rule on.

**Write:** `pallet/specs/standing-orders/constraints-and-decisions.md`, decisions and provisioning sections.

---

## Phase 6 — Integration boundaries and data sensitivity

**Do:** Brief declarations, each with its failure mode, in the same constraints document.

- **Stripe** — charge at placement, and the late-failure webhook. Failure modes: 8 s timeout, network error with one retry on the same idempotency key, card decline, and the Bacs acceptance-then-failure gap. The last one is the boundary the spec does not know exists.
- **Postmark via the worker** — occurrence summaries and failure notices. Failure mode: Postmark unavailable means the retailer is not told, and FR-005's same-day promise silently breaks. Needs a stated fallback.
- **Redis / BullMQ** — internal, but a genuine dependency whose failure at a cut-off means no order.
- **PostgreSQL** — the HA pair; failover behaviour during a cut-off window.

**Data sensitivity:** standing-order lines reveal a retailer's supplier relationship and volumes — Confidential under the governance principle, and never in logs, exports, or error bodies. The forecast is a derived aggregate of exactly that, and carries the small-denominator disclosure risk from F8. Payment method references (`stripe_pm_id`, `bacs_mandate_ref`, `last4`) are Restricted. Pallet holds no card or bank numbers, which is worth stating so nobody re-derives it.

---

## Phase 7 — Self-check and handover

**Do:** Walk the traceability in both directions and fix the gaps rather than reporting them as complete. Every FR-001…FR-007 has at least one technical requirement; every technical requirement names a source; every quality target has a number, a way to measure it, and a reason for that number; every constraint names the file it came from — governance, spine, or schema — and none rests on an assumption I made; every external dependency has a stated failure mode. Then a short design-week handover naming the findings that will shape their artifacts: the standing order is not a financial record but emits them; `Order.authorised_by` must widen to accept a standing-instruction id; occurrences need an identity so that placement can be made exactly-once; the forecast is aggregate-only.

**Write:** `pallet/specs/standing-orders/handover.md`.

---

## What I would report at the end

1. **One blocking item:** the spec's founding assumption is contradicted by the schema — around 245 retailers have no saved payment method, 214 of them by design. FR-002 cannot be built as written, and this needs Priya and Tom before design week starts.
2. **Six further rulings owed,** each with my stated default so the team is not blocked waiting: the meaning of "same day" for Bacs failures, the missing supplier timezone, price drift between occurrences, unavailable products in an occurrence, suppliers dropping a route day, and forecast disclosure with few retailers.
3. **One quality target I declined to invent:** portal edit latency, pending a baseline from the existing order endpoints.
4. **One cost decision:** the availability target this feature needs exceeds what one worker machine delivers, for a team with no dedicated ops.
5. **The artifacts:** findings, technical requirements, constraints-and-decisions with provisioning items and integration boundaries, quality-target rows appended to the architecture spine under placeholder ids pending the full file, and a handover note. No data model, no API contract, no quickstart — those belong to design week, and the quickstart in particular I would only write if the integration surface turns out to warrant it.
6. **The standing caveat:** the backend source is not in this workspace, so every claim about existing code comes from the operating manual and the spine rather than from reading the code, and I would mark it as such rather than let it pass for verified.