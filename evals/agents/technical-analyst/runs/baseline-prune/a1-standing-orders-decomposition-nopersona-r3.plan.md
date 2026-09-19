FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/specs/standing-orders/spec.md`

# Action plan — technical requirements analysis for FEAT-014 (standing orders)

**What this card is.** The product spec is signed off and says what retailers and suppliers get. Design happens next week. My job is the layer in between: what the system must technically do, what it will run into when it tries, and what numbers "good" means. The deliverable is a document, not code. The workspace holds no source — only the manual, one architecture excerpt, and a schema excerpt — so there is nothing to build or test here, and I would write no code and run no test suite. I would say that plainly in the report rather than inventing verification.

**Scope boundaries I would hold to.** I would not edit `specs/standing-orders/spec.md` — it is signed off and dated, and quietly amending a signed artifact hides the fact that its assumptions did not survive contact with the schema. I would not write rows into `architecture/spine.md` either: those are rulings, and AX-003/AX-006/AX-008 all carry a decision date and an author. What I would do instead is draft *candidate* spine rows inside my own document, marked as proposals for design week to ratify or reject.

---

## Phase 1 — Establish the baseline (reading, already done)

Read all four product files plus the spec. From them I fix the facts the analysis rests on:

- Placement and charging today are synchronous, inside a request, with a human on the page (`api → Stripe`, spine).
- The worker exists but runs on **one** Fly machine, and its current jobs (invoice generation, e-mail, PDF, accounting export, nightly purge) are all things nobody is waiting on. Standing orders would be the first job where lateness costs a delivery.
- Postgres is the only system of record; Redis is "BullMQ queues only".
- Four engineers, no dedicated ops, 140 suppliers, 1,900 retailers, UK + Ireland, Dutch producers from Q4 2026.

No further reading is needed — this workspace is complete for the task.

## Phase 2 — Trace every requirement to what exists, and name the gap

For each of FR-001…FR-007, write one line: what the current system already does, and what is missing. This produces the raw gap list. Concretely, the gaps I already see:

- **No data for the instruction itself.** Nothing in the schema can hold "this basket, this supplier, these route days". Needed: the standing order, its lines, and — importantly — its *occurrences* as durable rows, not just queue entries (see Phase 4, constraint on Redis).
- **`Order.authorised_by` is documented as "the user who placed it".** For an occurrence, no user acts. `Payment.authorised_by` already anticipates this ("user id or standing-instruction id"); `Order.authorised_by` does not. The standing-order id must become a legitimate authoriser on the order too, or the money-authority principle is satisfied on the payment and silently broken on the order.
- **No notification path driven by anything other than a request.** FR-005 and FR-006 both need worker→Postmark mail triggered by a background outcome.
- **No forecast read model.** FR-007 needs projected demand four weeks out, which means projecting occurrences forward through skips, pauses, and route-day changes.
- **No schedule machinery.** AX-006 gives repeatable and delayed BullMQ jobs; nothing today computes "the cut-off before Tuesday".

## Phase 3 — Cross-check the spec's assumptions against the schema (the highest-value pass)

This is where the signed-off spec meets the database. Four contradictions I would document with the exact evidence, because each one changes what gets built:

1. **"Every retailer has a saved payment method" is false.** `Retailer.payment_method` is nullable, and the schema comment says it is null for **every one of the 214 `invoice_30d` retailers** and for **31 `card_on_file` retailers who never finished setup**. FR-002 charges at placement. So either standing orders are gated on having a method (excluding ~245 retailers, 13% of the base, including the entire invoice-30d segment), or occurrences for invoice-30d retailers are placed and settled on the monthly statement like their one-off orders. This is a product call, not mine.
2. **Bacs debits cannot fail the same day.** AX-003 as-built: a Bacs debit is *accepted* at submission and can fail **up to three working days later**, reported by the `payment_intent.payment_failed` webhook — and the row explicitly notes this is tolerable today only because one-off retailers are on the page. FR-005's "told the same day" therefore has to mean same day *as the failure becoming known*, which can be three days after the delivery already happened. Card charges do fail synchronously; the two payment kinds need different failure requirements.
3. **The cut-off has no timezone.** `Supplier.cutoff_time` is `@db.Time`, "stored as typed on the supplier's settings page; the worker machine runs in UTC". Today the cut-off is advisory. Once it triggers automatic placement, a UK supplier who typed 17:00 gets placement at 18:00 local through British Summer Time, and Irish and (from Q4 2026) Dutch suppliers compound it. An hour late on a cut-off morning is a missed delivery — exactly the failure the feature exists to prevent.
4. **Prices and product availability move underneath the instruction.** `Product.price_minor` is the *current* price and suppliers change prices "roughly monthly"; `OrderLine.unit_price_minor` freezes the price at placement. The spec's assumption "prices are those in the catalogue" means an occurrence is charged at whatever the price is that morning, with no human present. And `ProductStatus` has `paused` and `delisted` — the spec says nothing about an occurrence whose basket contains a delisted product.

I would also note two smaller ones: a supplier can drop a route day the standing order depends on, and "the day before each route day" is ill-defined for a Monday route day (Sunday cut-off) — and for a standing order created *after* the coming Tuesday's cut-off has passed, where the honest next occurrence is the following Tuesday, not the one US-001's independent test names.

## Phase 4 — Derive the technical requirements

Write numbered technical requirements, each traced back to an FR or to a governance principle. The substantive ones:

**Placement and correctness**
- Each occurrence is identified by (standing order, occurrence date) and placed **at most once**, enforced in Postgres by a uniqueness constraint, not by queue behaviour. The manual's own rationale applies literally: "a duplicated order is a real van at a real café."
- The BullMQ job id is derived deterministically from that same pair, satisfying the mandatory-`jobId` rule (`PLT002`) and making a worker restart mid-batch a no-op.
- The Stripe idempotency key is likewise derived from the occurrence, so the existing "one retry on a network error reusing the same idempotency key" behaviour cannot double-charge.
- **Occurrences must be materialised in Postgres, not held only as delayed Redis jobs.** Redis is declared queues-only and Postgres is the system of record; if the Upstash instance is lost or flushed, pending placements must be recoverable. This means a reconciliation sweep that finds due-and-unplaced occurrences, which also becomes the safety net for a worker outage.
- The order is placed even when the charge fails (US-002, FR-005) — so placement and charging must be separable, and the audit trail must record the payment attempt regardless of outcome.

**Authority, audit, retention**
- Every order and payment created by an occurrence carries the standing-order id in `authorised_by`, and the standing order records who created it and when.
- Basket changes (FR-004) are append-only history — who, what, when — and must not mutate any already-placed occurrence. Existing orders are immutable financial records; a change applies from the next occurrence only.
- The standing order is referenced by payments retained seven years, so it cannot be hard-deleted and the nightly purge must skip it. This extends AX-008's audit coverage (currently orders, invoices, payments) to the standing instruction itself.

**Scheduling**
- The system must compute, for each standing order and each chosen route day, the cut-off instant **in the supplier's local time**, correct across DST transitions. This requires a supplier timezone that does not exist today — a schema addition, and a backfill for 140 existing suppliers.
- Skip and pause must be race-safe against the placement job: a conditional state transition, so that a skip arriving as the job starts either wins cleanly or is rejected with a clear message. "Up to the cut-off" needs a defined tiebreak; I would specify that the occurrence's terminal state is decided by a single conditional update and the loser is told which way it went.

**Forecast**
- Four-week projection per product per route day, computed from active standing orders with pauses and skips applied, and excluding retailers ineligible under whatever Phase 3 decision #1 lands on.
- Under the confidentiality principle, the default is that a supplier sees **aggregate quantities only**, not a per-retailer breakdown — trading relationships are Confidential. Flagged for a ruling, since the supplier does see individual retailers on placed orders.

**Notifications**
- Occurrence summary (FR-006) and charge-failure notice (FR-005) via worker→Postmark, with counterparty details kept out of logs and out of any error body per the redaction list and the export allowlist.
- Errors on the standing-order endpoints are RFC 7807 Problem Details built by the shared exception filter, with `correlation_id`.

## Phase 5 — Constraints

A separate, explicit list, because these bind design week's hands:

- **Single worker machine.** Placement is the first latency-critical background work in the product; one machine in `lhr` is a single point of failure on exactly the mornings the spec calls "when the money is". Either the worker becomes multi-machine (which the idempotency requirements above already make safe) or the availability target must be honestly written down as lower than the spec's aspiration.
- **Stripe**: pinned API version `2024-06-20`, 8 s timeout, one network retry. Bacs asynchronicity per AX-003.
- **No new datastore** without a recorded platform sign-off; a read model for the forecast should be Postgres.
- **No cron** — repeatable and delayed BullMQ jobs only (AX-006).
- **Coverage floor 75%, blocking**, and integration tests against real Postgres.
- **Team**: four engineers, no dedicated ops. Anything that needs someone awake at 05:00 is not a viable design.
- **Data residency**: UK/EU, Fly `lhr`, Stripe EU entity.
- **Currency**: GBP and EUR both live (Ireland now, Netherlands Q4 2026); a standing order is single-supplier and therefore single-currency.

## Phase 6 — Turn the quality expectations into numbers

The spec's quality section is four sentences with no measurements, and SC-001 ("placed reliably") is not measurable as written. I would convert each into a scenario with a trigger, a measure, and a target, marking every number as **proposed** and needing Priya's and the team's agreement:

- **Placement timeliness** — proposed: 99.9% of due occurrences result in a placed order within 5 minutes of the supplier's cut-off; 100% within 60 minutes; zero placed after the route day's delivery window. This is what SC-001 should have said.
- **No duplicates** — zero duplicate orders per occurrence, asserted by constraint, not by target.
- **Placement availability** — proposed 99.9% over the cut-off window, with the explicit note that today's single worker machine does not support that claim.
- **Portal responsiveness** — proposed p95 under 300 ms and p99 under 1 s for standing-order read and edit endpoints, since "feels fast" is otherwise untestable.
- **Forecast accuracy and freshness** — proposed: reflects any change within 5 minutes; projected quantities match what is actually placed for unskipped, unpaused occurrences exactly (any mismatch is a bug, not a tolerance).
- **Notification timeliness** — occurrence summary within 15 minutes of placement; card-charge failure notice within 15 minutes; Bacs failure notice within 15 minutes of the webhook, which may be up to three working days after placement.
- **Security** — unchanged posture: Stripe PaymentMethod ids only, no card or bank numbers, no counterparty contact details in logs, exports, or error bodies.
- **Measurability of the success criteria** — SC-002 is measured "from the standing-orders table", so that table must record creation time and enough to identify the retailer's prior repeat-order behaviour; SC-003 needs the helpdesk tag to exist and an occurrence count to divide by. Both are requirements on what we record, not just aspirations.

**Capacity sizing** I would do here rather than hand-wave: if SC-002 lands, roughly 500–600 standing orders averaging two route days gives on the order of 1,000–1,200 occurrences a week, but they bunch at cut-off times shared across suppliers. A worst case of ~500 occurrences inside a 15-minute window against an 8 s Stripe timeout needs a charge concurrency in the low tens to clear in time — comfortably inside Stripe's rate limits, but well above anything the single worker does today. This number is what makes the worker-machine constraint concrete instead of rhetorical.

## Phase 7 — Testability and observability requirements

- The spec's own independent tests say "advance to the cut-off", which is only possible with an injectable clock. I would record a hard requirement that scheduling logic take time as a dependency, and that integration tests drive it against real Postgres per the testing principle. Without this, US-002 and US-003 are untestable as written.
- Test cases that must exist: DST boundary crossing; skip racing the placement job; charge failure still placing the order; Bacs late failure via webhook; delisted product in the basket; price changed between setup and occurrence; duplicate job delivery after worker restart; standing order for a retailer with no payment method.
- Observability: the existing 5xx and 10-minute queue-lag alerts (AX-012) do not detect "the cut-off passed and nothing was placed" — a silent failure with no 5xx and possibly no queue backlog. A new alert on due-but-unplaced occurrences is required. I would raise this as a proposed amendment to AX-012 for design week.

## Phase 8 — Decisions I would stop on, and what I would do meanwhile

I would not block on any of these — I would write the document in full under stated defaults, with each decision listed by name, owner, and the branch each way. Sign-off happened yesterday and the spec says "no open questions outstanding", so surfacing these is the main service this analysis provides.

| Decision | Owner | Default I would write under | Other branch |
|---|---|---|---|
| Retailers with no saved payment method (245, incl. all 214 invoice-30d) | Priya, with Tom | Eligibility gate: standing orders only for retailers with a saved method; invoice-30d out of scope for launch, and SC-002's denominator adjusted to eligible retailers | Place-without-charge for invoice-30d, settled on the monthly statement — adds an unpaid-occurrence path and changes FR-002 |
| "Same day" for Bacs failures | Priya | Requirement split: card failures notified within 15 min of placement; Bacs failures within 15 min of the webhook, which may be days later | Restrict standing orders to card-on-file retailers at launch |
| Supplier timezone | Team, at design | Add an IANA timezone per supplier, default `Europe/London`, backfill 140 suppliers, confirm with Irish suppliers before launch | Treat all cut-offs as UTC and accept an hour of drift half the year — I would argue against this |
| Price change before an occurrence | Priya, with Tom | Charge the catalogue price and show it on the summary, per the spec's assumption, plus notify the retailer when a line's price moved more than a set threshold since the last occurrence | Cap or hold the price, which contradicts the signed assumption |
| Delisted or paused product in a basket | Priya | Place the occurrence with the remaining lines and tell the retailer what was dropped; if every line is unavailable, place nothing and tell them | Skip the whole occurrence on any unavailable line — safer, worse for the café |
| Supplier drops a route day the order uses | Priya | Suspend that day's occurrences and notify both sides | Auto-move to the nearest remaining route day — I would not do this silently |
| Forecast granularity | Priya, against the confidentiality principle | Aggregate quantities per product per route day, no retailer identities | Per-retailer breakdown, which needs an explicit confidentiality ruling recorded |

If a ruling later contradicts my default, the affected requirements are individually numbered so the change is a small edit, not a rewrite.

## Phase 9 — Write the document

One file: **`pallet/specs/standing-orders/technical-requirements.md`**, sections in this order — scope and status; derived technical requirements with traces to FR/US; contradictions found against the schema and architecture (Phase 3, with file and line evidence); constraints; quality scenarios with proposed numbers; testability and observability; capacity sizing; open decisions with defaults and owners; proposed spine amendments (a new row for standing-order scheduling, a note against AX-003 on unattended Bacs, an extension of AX-008 to the standing instruction, and an alert addition to AX-012) marked clearly as *proposed, not ruled*.

No other file is created or modified. If the team would rather have the open decisions as a standalone one-pager for Priya, that is a trivial split I would offer rather than assume.

Then a self-review pass: re-open `schema.prisma` and `spine.md` and check every factual claim in the document against them, particularly the nullable payment method, the Bacs timing, the `@db.Time` cut-off, the single worker machine, and the `authorised_by` wording — a requirements document that misquotes the schema is worse than none.

## Phase 10 — Delegation, and what I would report

**Delegation: none.** Five files, all read, and the work is judgment about how a signed spec collides with an existing schema — splitting that across workers would cost more in briefing and cross-checking than it saves, and the contradictions in Phase 3 only surface by holding the spec and the schema in mind together.

**Report to the user:**
- The document is written, at that path; nothing else was touched, and the signed spec was deliberately left unamended.
- The four contradictions between the signed-off spec and the shipped system, stated up front, because they are the finding: the payment-method assumption is false for 245 retailers, Bacs cannot fail same-day, the cut-off has no timezone and DST will make placement an hour late for half the year, and prices and product availability move underneath an unattended basket.
- The two structural constraints design week has to answer: one worker machine now carries delivery-critical timing, and occurrences must live in Postgres because Redis is not the system of record.
- Every quality number in the document is a proposal, not a measurement — there is no running system here to measure and I did not run anything.
- The seven decisions needing Priya's or Tom's ruling, with the defaults I wrote under, and a note that the spec currently records no open questions.

Finally, I would record one short project note in my memory directory — that FEAT-014's technical requirements were drafted on 2026-09-09 ahead of design week, with the payment-method eligibility question and the Bacs same-day question outstanding with Priya — since that is context the repository itself would not carry. In this plan-only run I cannot write it, so I am naming it instead.