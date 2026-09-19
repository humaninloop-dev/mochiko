FILES-READ: `pallet/specs/standing-orders/spec.md`, `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`

---

# Action plan — technical requirements analysis for FEAT-014 (Standing orders)

## Scope and stance

The ask is analysis, not design: what the system must do, what stands in its way, and what "good" measures out at. The team designs next week, so I produce a requirements artefact they design *against* — I do not pick the scheduler shape, the table layout, or the queue topology for them beyond what the existing rules force.

Two framing facts from the workspace that shape everything below:

- **There is no code here.** The workspace is five files: a readme, the operating manual, one architecture excerpt, one schema excerpt, one spec. `src/payments/`, the lint rule `PLT002`, the contract suite, and the job base class are all referred to but absent. So every statement I make about as-built behaviour is *asserted by the docs, unverified in this workspace*, and I would label it that way in the deliverable rather than presenting it as checked.
- **The spec's "Open questions: None outstanding" is wrong**, and provably so from the schema. That is the single most important output of this work. I would not treat sign-off as closing the questions; I would take it as meaning the *product* intent is settled and the technical consequences were never worked.

I would not edit `spec.md`. It is Priya's, it is signed off and dated, and quietly amending a signed artefact hides the fact that its assumptions failed. Proposed amendments go in my document as proposals.

---

## Phase 1 — Build the ground-truth inventory

**Do:** Re-read the four product files with one question each, and write down, per file, the hard statements the feature must live inside. No inference yet — just the register.

**Read:** `pallet/CLAUDE.md` (the seven governance principles and their stated enforcement points), `pallet/architecture/spine.md` (AX-003, AX-006, AX-008, AX-012 and the four communication styles), `pallet/prisma/schema.prisma` (all nine models/enums, especially the inline comments, which carry more load than the types), `pallet/README.md` (scale, team, geography).

**Produce (working notes, folded into the Phase 7 document):**
- Governance rules that bind: immutability + 7-year retention; `authorised_by` non-null on payments/payouts; confidentiality of counterparty data and UK/EU residency; mandatory idempotent jobs with deterministic ids; Postgres-only; RFC 7807 errors; ≥75% coverage with real-Postgres integration tests; pino + correlation id + OTel + `/health` + 5xx and queue-lag alerts.
- Architecture facts that bind: one worker machine; Stripe EU, SDK pinned `2024-06-20`, 8 s timeout, one retry on the same idempotency key; Bacs accepted-then-fails-later via `payment_intent.payment_failed`; BullMQ repeatable/delayed jobs, no cron; audit row in the same transaction as the mutation.
- Schema facts that bind: `Supplier.cutoff_time` is a bare `@db.Time` with no zone and the worker runs UTC; `route_days` is an int array; `Product.price_minor` changes roughly monthly and `status` can go `paused`/`delisted`; `Order.authorised_by` is documented as "the user who placed it"; `Payment.authorised_by` already admits a standing-instruction id; `Payment.status` is a bare string; money is integer minor units and currency is per-product and per-order.
- Scale facts: 140 suppliers, 1,900 retailers, UK + Ireland today, Dutch producers from Q4 2026, four engineers, no dedicated ops, part-time finance lead.

**Flag at this phase:** `Payment.authorised_by` already says "user id or standing-instruction id" while no standing-instruction table exists. Someone anticipated this feature in the money model. That is the seam the feature is meant to fit, and I would treat it as a design intent to honour rather than a coincidence.

---

## Phase 2 — Conflict analysis: spec assumptions against product reality

This is the phase that carries the value. For each of the spec's three assumptions and seven functional requirements, I check it against the schema and spine and record whether it holds.

**Do:** Work each of the following to a conclusion, with the evidence line cited.

1. **"Every retailer has a saved payment method" — false.** `schema.prisma:16` says `payment_method` is null for *every* `invoice_30d` retailer and for 31 `card_on_file` retailers who never finished setup. The enum at `schema.prisma:7` says 214 retailers are `invoice_30d`. So at least 245 of 1,900 retailers — roughly 13% — cannot satisfy FR-002 as written. FR-002 makes charging inseparable from placing; for these retailers, either no standing order exists or an order is placed with no charge. This is a product ruling, not an engineering one.

2. **Bacs failure is not same-day — FR-005 is unsatisfiable as literally written.** AX-003 states plainly that a Bacs debit is *accepted* at submission and can fail up to three working days later by webhook, and that this "today only happens for one-off orders, where the retailer is on the page." Standing orders are the first unattended flow. FR-005 ("notified the same day when an occurrence's charge fails") can only mean *the day Pallet learns of the failure*. This is genuine drift against AX-003, which currently reads "Drift: none."

3. **Cut-off has no timezone, and the feature is a wall-clock scheduler.** `cutoff_time` is `@db.Time`, "stored as typed on the supplier's settings page; the worker machine runs in UTC." Today a cut-off is only ever compared against a request happening now. FR-002 requires firing *at* that time, on a specific date, repeatedly, across BST/GMT transitions — and from Q4 2026 across Europe/Amsterdam too, which is a different offset from Europe/London on every day of the year. Without a zone the schedule is wrong for half the year and wrong permanently for Dutch suppliers.

4. **Placement is on one worker machine.** The spine lists `worker` as a single Fly machine already carrying invoice generation, e-mail, PDF rendering, accounting export and the nightly purge. The spec says placement "must be highly available on cut-off mornings — that is when the money is." One machine is the availability ceiling, and placement now competes with PDF rendering for it.

5. **Cut-offs cluster; I would size the burst.** Suppliers pick round times. If adoption reaches the SC-002 target and a quarter of retailers run ~2 route days, that is on the order of 900–1,000 standing orders and ~1,900 occurrences a week, arriving in a handful of spikes rather than evenly. A spike of a few hundred occurrences, each holding a synchronous Stripe call with an 8 s timeout and a retry, sets a concrete throughput requirement and a Stripe rate-limit question. I would put the arithmetic in the document rather than the adjective "spiky".

6. **Catalogue drift between setup and occurrence.** Prices change monthly; products can be `paused` (seasonal) or `delisted`. The spec's "Prices are those in the catalogue" says nothing about a product that is no longer purchasable, or a price that has doubled, at the moment an unattended charge fires. The schema already snapshots `OrderLine.unit_price_minor`, so the recording is solved; the *policy* is not.

7. **Skip races placement.** FR-003 allows skipping "up to the cut-off" — precisely the instant placement fires. Without an atomic state transition this produces either a delivery the café closed for, or a duplicate.

8. **Idempotency has a sharper edge here than elsewhere.** The manual's rationale is literally "a duplicated order is a real van at a real café." A deterministic BullMQ `jobId` is necessary but not sufficient: it protects against re-running a job, not against two different jobs for the same occurrence, nor against a crash between the Stripe charge and the Postgres commit. The Stripe idempotency key must be derived from the occurrence, not minted per attempt.

9. **`Order.authorised_by` is typed for humans.** `schema.prisma:74` comments it as "the user who placed it," while `Payment.authorised_by` already admits a standing-instruction id. Governance requires *both* to be attributable. The order side needs widening, and the ULID prefix convention at `schema.prisma:2` needs a new prefix for the standing instruction.

10. **"From a basket" — there is no basket.** No `Basket` model appears in the excerpt. Either baskets are client-side state in the two front-end repos, or the excerpt omits the model. I cannot tell from here, and it changes what FR-001's endpoint receives. I would ask rather than assume, and default to "the API receives an explicit line list, not a basket reference."

11. **Retention collides with deletion.** The spec offers pause but never cancel/delete. Once a standing order id appears in `Payment.authorised_by`, it is the authority record for a payment and falls under the seven-year rule and the update-rejecting triggers. FR-004 lets retailers edit it. So the instruction must be versioned — edits create a new version, the payment points at the version in force — or the authority trail breaks. This is the least obvious requirement in the whole feature and I would give it its own section.

12. **Forecast versus confidentiality.** Trading relationships are Restricted. FR-007's forecast spans several retailers. A supplier legitimately sees their own orders, but a forward projection broken down per retailer is a different disclosure, and skipped/paused occurrences leak a retailer's closure plans. Default: aggregate per product per route day per currency only.

13. **Existing alert thresholds are too loose for this.** AX-012 alerts on queue lag above 10 minutes. Ten minutes of lag on a cut-off is a missed cut-off, which is the exact failure the feature exists to prevent.

14. **Multi-currency.** `Product.currency` defaults GBP; Ireland trades EUR; the Netherlands arrives in Q4 2026. A standing order must be single-currency and the forecast must group by currency, or it silently sums pounds into euros.

15. **Measurability of the success criteria.** SC-002 is "measured from the standing-orders table" and needs a creation timestamp, creator, and a way to identify retailers with 3+ prior repeat orders to the same supplier — a query over `Order` that must be defined now, not after launch. SC-003 is per-occurrence and needs a support-quotable occurrence reference tied to the correlation id. SC-001 ("placed reliably") is not measurable at all as written and needs a number.

**Refuse/flag:** Nothing here is a refusal. Item 2 and item 1 are the two I would escalate before the design session rather than after it, because both change what gets designed.

---

## Phase 3 — Derive the technical requirements

**Do:** Write derived requirements as `TR-###`, each traced to its source FR/US and to the governing rule it satisfies. Grouped:

- **Instruction model & authority** — a standing instruction record naming creator and creation time; versioning on edit so that an issued payment's authority is immutable; a new id prefix; `Order.authorised_by` widened to accept it; the instruction is not hard-deletable while referenced.
- **Scheduling & occurrences** — occurrences materialised in Postgres (system of record; Redis carries only the job, so a Redis loss loses nothing); a deterministic occurrence identity per (instruction, date); an explicit state machine covering scheduled → skipped / placing / placed / charge-failed / missed; a zone-aware cut-off resolution rule; behaviour when a supplier changes `cutoff_time` or removes a `route_day` the instruction depends on.
- **Placement & payment** — exactly-once order creation enforced in the database, not only by job id; a deterministic Stripe idempotency key; order placed even when the charge fails (FR-005) with the payment recorded as failed; the `payment_intent.payment_failed` webhook path extended to unattended occurrences; eligibility gate on payment method per the Phase 6 ruling.
- **Catalogue handling at occurrence time** — price snapshot into the line; rules for `paused`/`delisted` products and for a wholly-unavailable basket; whether a price rise needs disclosure.
- **Notifications** — per-occurrence summary and same-day-as-known failure notice, both idempotent under job retry, both to `contact_email` (the only address the schema holds — there is no per-user notification preference, which I would flag), none containing anything the redaction rules forbid.
- **Forecast** — aggregation per product per route day per currency for four weeks, honouring pause and skip, excluding retailer identity.
- **Audit & observability** — instruction mutations audited in the same transaction (an extension of AX-008, which today covers only orders/invoices/payments); correlation id threaded from the scheduled job through Stripe and the e-mail; a support-quotable occurrence reference.
- **Measurement** — the fields and queries SC-002 and SC-003 require, specified now.

**Write:** this is a section of the Phase 7 document; no separate file.

---

## Phase 4 — Constraints register

**Do:** Record constraints as `CN-###` separately from requirements, because they are not negotiable by this feature and the design session needs them visible: single worker machine; Postgres-only; no cron; Stripe pinned version, 8 s timeout, single retry, EU entity, Pallet holds no card/bank numbers; UK/EU data residency; immutability triggers and 7-year retention; mandatory job ids; RFC 7807 error bodies; 75% coverage floor with real Postgres; four engineers and no dedicated ops (which is itself a constraint on how much operational surface this feature may add); Q4 2026 Dutch onboarding as a dated forcing function on the timezone work.

---

## Phase 5 — Quality targets

**Do:** Convert the spec's four sentences of "Quality expectations" — fast, highly available, secure, accurate — into numbers with a measurement method and a verification route, as `QT-###`. Concretely I would propose, for the team to accept or move:

- **Timeliness:** an occurrence is placed within 5 minutes of its cut-off at p99, 15 minutes worst case; anything past the ruled catch-up window is not placed and is alerted. Measured from occurrence cut-off timestamp to order `placed_at`.
- **Correctness:** zero duplicate orders per occurrence — a hard target, database-enforced, with a reconciliation job asserting that every due occurrence ended in exactly one terminal state daily.
- **Availability:** the placement path survives a Redis outage without losing occurrences and a worker restart without duplicating them; state the single-worker-machine risk explicitly as an accepted or unaccepted exposure rather than burying it.
- **Latency:** standing-order read/edit endpoints p95 under 300 ms, p99 under 800 ms; forecast p95 under 1.5 s over four weeks at full adoption.
- **Security/privacy:** no card or bank number stored; no counterparty contact, bank, or trading-relationship data in logs, exports, or error bodies — verified by the contract suite that already exists for this purpose.
- **Accuracy:** define forecast accuracy honestly as "reflects currently-scheduled occurrences, including pauses and skips, within 60 seconds" — it cannot predict future edits, and promising more than that is how SC-003 tickets get created.
- **Alerting:** a placement-specific lag threshold well under AX-012's global 10 minutes, plus an alert on any occurrence passing its deadline unplaced.
- **Testing:** the coverage floor applies; integration tests against real Postgres; the specific scenarios the spec's own "Independent test" lines demand, plus the ones it omits — skip-versus-placement race, worker restart mid-batch, Bacs late failure, delisted product at occurrence time, DST boundary.

**Test I would run:** none. There is no code and no test harness in this workspace, so I would not claim any verification. What I write instead is the acceptance-test outline the team implements, marked as specification rather than as passing tests.

---

## Phase 6 — Decisions I would stop for

I would not resolve these myself; each changes the product. I would send them as a short list to Priya (and Tom for the money ones) before the design session, state my default, and keep writing under that default so no work is blocked waiting.

1. **Retailers with no saved payment method (Priya + Tom).** Confirm: do the 214 `invoice_30d` and 31 incomplete-setup retailers get standing orders?
 - *Ruled out of scope* → eligibility gate; FR-002 holds; document the excluded segment and its effect on the SC-002 denominator.
 - *Ruled in* → FR-002 splits: place-and-charge for saved-method retailers, place-and-bill for `invoice_30d`; the authority model still works because the instruction id is the authority; SC-002's measurement changes.
 - **My default: gate to retailers with a usable saved method**, because it is the only reading FR-002 supports as signed off.
2. **What "same day" means for Bacs (Priya + Tom).** Default: same day as Pallet learns of the failure, via webhook; record the AX-003 drift.
3. **Missed cut-off behaviour (Priya).** Default: past the catch-up window, do not place — an order after cut-off is a delivery that will not arrive and a charge that will need reversing; notify the retailer, roll to the next occurrence, alert.
4. **Unavailable product or moved price at occurrence time (Priya).** Default: place with the current catalogue price, drop `paused`/`delisted` lines and say so in the summary, place nothing if every line is unavailable. No price-change ceiling — flagged as a candidate follow-up.
5. **Forecast granularity (Priya).** Default: aggregate only, no retailer identity.
6. **Does a basket exist server-side (engineering)?** Default: the API takes an explicit line list.

If any ruling goes against my default, the affected `TR-`/`QT-` entries change but the structure holds; I would revise the document rather than restart.

---

## Phase 7 — Write the deliverables

**Write:**
- `pallet/specs/standing-orders/technical-requirements.md` — the main artefact: derived requirements, constraints register, quality targets, a traceability matrix from each US/FR to its `TR-`s and to the acceptance tests, and an appendix drafting the *proposed* architecture rows for the design session to rule on (AX-003 drift for unattended Bacs; an AX-006 extension for wall-clock recurring work; an AX-008 extension covering standing instructions; a new row for the standing-instruction authority model). Marked proposed, not decided — the manual requires a recorded platform sign-off, which is next week's meeting, not mine.
- `pallet/specs/standing-orders/open-questions.md` — the six rulings above, each with evidence, my default, and the consequence of each branch. This exists as a separate file so it can be sent to Priya on its own and so it visibly contradicts the spec's "None outstanding" line.

**Not written:** `spec.md` (not mine to amend), `architecture/spine.md` (rows need a ruling), `schema.prisma` (that is next week's design).

---

## Phase 8 — Self-check before reporting

Re-read both documents against the spec once more and confirm: every US and FR traces to at least one derived requirement; every quality target has a number and a measurement method; every conflict names the file and line that evidences it; nothing asserts a verification I did not perform; no requirement smuggles in a design decision the team should make.

---

## Delegation

None. This is a single analytical document over five files that I have already read in full; splitting it across workers would cost more in reconciling their reads than it saves, and the value is in holding all the contradictions in one head at once. No subagents, no parallel workers.

## What I would report

The two documents and their paths; the headline finding that the spec's assumptions do not survive contact with the schema — specifically that ~245 retailers cannot be charged as FR-002 requires, that FR-005's "same day" is unachievable for Bacs given AX-003's own description, and that the cut-off time carries no timezone in a feature that is fundamentally a wall-clock scheduler; the six decisions waiting on Priya and Tom with my defaults; the three architecture rows that need a ruling next week; and a plain statement that nothing was executed or verified here because the workspace contains no code — every claim about as-built behaviour rests on the documentation.

If Priya rules on the payment-method question, that ruling is worth keeping as a durable project note, since it is a product decision that will not be recoverable from the code later.