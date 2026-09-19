FILES-READ: `pallet/specs/standing-orders/spec.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`

---

# Action plan — technical analysis of Standing orders (FEAT-014)

## Scope I would set, and the boundary I would hold

The card asks what the system **must do**, what it **runs into**, and what it **must hit** — the analysis layer, ahead of next week's design. So I would produce three things and deliberately **not** produce the data model or the API contract:

| Artifact | Path | Produce now? |
|---|---|---|
| Technical requirements | `pallet/specs/standing-orders/technical-requirements.md` | Yes |
| Constraints, decisions, infrastructure provisioning | `pallet/specs/standing-orders/constraints-and-decisions.md` | Yes |
| Quality targets (NFR rows) | attached to concern rows in `pallet/architecture/spine.md` | Yes — with a stop, see Phase 6 |
| Data model / API contract / integration guide | — | **No.** That is next week's design pass; writing them now would pre-empt the decisions this analysis is supposed to frame. |

I would say this boundary explicitly in the handoff so nobody thinks the design artifacts were forgotten.

---

## Phase 1 — Reconcile the signed-off spec against the product as it actually is

Already done as the reading above; this is the analytic pass over it. The whole value of this phase is that **the spec says "no open questions at sign-off" and the codebase says otherwise.** I would build a contradiction register before writing a single requirement, because four of these change what the requirements even are.

What I would read (all read): the spec, the operating manual's governance principles, the four concern rows in the architecture spine, and every model in the schema excerpt.

The contradictions I have already found, and would carry into Phase 6 as stops:

1. **The core assumption is false.** The spec assumes "every retailer has a saved payment method, so every occurrence can be charged at placement." The schema says `payment_method` is null for *every* one of the 214 retailers on 30-day invoice terms, plus 31 card-on-file retailers who never finished setup. That is roughly 13% of the 1,900-retailer base for whom FR-002 as written cannot execute. This is not an edge case; it decides whether standing orders are a card-only feature or whether placement and charging are separable steps.
2. **"Told the same day" is not achievable for direct debit.** The payments concern row states a Bacs debit is only *accepted* at submission and can fail up to three working days later via webhook. FR-005 requires same-day notification of a failed charge. For Bacs retailers the failure signal does not exist on the same day. The row also notes this has been survivable so far only because one-off orders have the retailer on the page — which is exactly the condition standing orders remove.
3. **Cut-off time has no timezone.** `Supplier.cutoff_time` is a bare SQL `time`, typed on the supplier's settings page, and the worker machine runs UTC. Across the UK/Ireland clock change that is an hour of drift twice a year, in the direction of placing orders after the cut-off. Dutch producers onboard in Q4 2026, which adds a second offset. A missed cut-off is the exact failure the feature exists to prevent.
4. **"The day before" is not in the schema.** FR-002 says the cut-off falls on the day before each route day. `Supplier` stores a cut-off *time* and route days, with no lead-time field. Either the one-day lead is a universal business rule or it is a per-supplier value that does not exist yet.
5. **Prices and product availability move underneath a standing order.** Prices change roughly monthly; products can go to `paused` or `delisted`. The spec's assumption "prices are those in the catalogue" says nothing about a bread line being delisted on a Thursday for a Friday occurrence, or a 40% price jump charged without anyone looking.
6. **Immutability versus cancellation.** The governance principle says database triggers reject `UPDATE` on issued rows of the financial tables, and corrections are new documents. But `Order` carries a `cancelled` status. I cannot tell from this excerpt whether an order may legally transition `placed → cancelled`. That determines the only available remedy for a wrongly-placed occurrence.
7. **One worker machine against a "highly available" expectation.** The spine puts BullMQ workers on a *single* Fly machine, while the spec's quality section says placement "must be highly available on cut-off mornings — that is when the money is." A single machine is a single point of failure sitting directly under the feature's most important moment.
8. **Placement is inherently bursty.** Suppliers cluster on round cut-off times. Every occurrence due at that cut-off wants a Stripe charge through a client with an 8-second timeout and a single retry. Nothing in the architecture currently shapes that burst.

**Refusal I would make here:** I would not quietly write requirements that assume these away. Items 1, 2 and 5 are product rulings, not technical gaps, and inventing answers for them would bake guesses about money movement into the design.

## Phase 2 — Load the procedures

Load `mochiko:authoring-technical-requirements` for the requirement and constraint grammar and the infrastructure-provisioning form, and `mochiko:patterns-technical-decisions` for the alternatives-and-trade-offs format the decisions must take. I would conform file names and identifier formats to whatever those skills specify, over the placeholder paths above. I would not load the entity-modeling or API-contract skills — their artifacts are out of scope this week.

## Phase 3 — Decompose the seven business requirements into technical requirements

Write `pallet/specs/standing-orders/technical-requirements.md`. Each entry names the business requirement it came from, states what the system must achieve without naming a technology, and carries acceptance criteria a test can be written against. My working decomposition:

- **From FR-001 (create):** validate the basket resolves to exactly one supplier; validate chosen days are a subset of that supplier's route days; compute and expose the next occurrence date from the chosen days and the current date; enforce the payment-capability precondition (whatever Phase 6 rules it to be); record who created the instruction and when.
- **From FR-002 (place and charge):** resolve each supplier's cut-off instant correctly in that supplier's local time; trigger placement at that instant; place exactly once per instruction per occurrence date even if the worker restarts mid-batch; materialise an ordinary order the supplier sees alongside their other orders — not a parallel object type; price lines at catalogue price *at placement*, captured onto the line as the existing order lines already do; charge the saved method through the existing payments path carrying a charge identity derived from the instruction and occurrence date so a retry never double-charges; set the payment's authority to the standing instruction rather than a user; write an audit entry in the same transaction as the financial write.
- **From FR-003 (skip/pause):** an occurrence-level skip that is accepted only before that occurrence's cut-off and is decided atomically against the placement attempt, so a skip landing in the same second as the cut-off either wins or is refused but never produces both a skip and an order; pause and resume as instruction-level states, where a paused instruction produces nothing when its date passes and resuming does not retroactively place missed dates; skipping one date leaves the following date untouched.
- **From FR-004 (edit):** basket changes take effect from the next occurrence and never alter an order already placed; a change made after a cut-off has already passed must not reach back into that occurrence; every change records actor, timestamp, and before/after and is retrievable as history.
- **From FR-005 (charge failure):** detect a synchronous charge failure and notify the retailer within the same day while still completing placement; separately, detect a *late* direct-debit failure arriving by webhook days later and route it somewhere a human sees — this is a distinct requirement from the same-day one and exists only because of contradiction 2.
- **From FR-006 (summary):** send a per-occurrence summary asynchronously via the existing mail path, containing no confidential counterparty data beyond what the recipient already owns.
- **From FR-007 (forecast):** aggregate quantity per product per route day over four weeks from open instructions; exclude paused instructions and skipped dates and reflect pending basket changes, so "accurate" has a definition; scope every read to the requesting supplier's own products; return aggregates only, so no retailer's identity or trading relationship leaks through a forecast.
- **Implicit requirements the spec never mentions but the system needs:** a query surface that supports the 30%-adoption measure and the support-ticket-rate measure; interaction with the nightly retention purge (the instruction is not itself a financial record but points at records retained seven years); an alert when a due occurrence did *not* produce an order, since silence is the failure mode that hurts; a bound on concurrent outbound charges during a cut-off burst; handling for an instruction whose supplier changes or removes a route day underneath it.

## Phase 4 — Constraints, decisions, infrastructure

Write `pallet/specs/standing-orders/constraints-and-decisions.md`.

**Constraints** — each traced to where it actually comes from (a governance principle, a concern row, or a schema fact), never to an assumption. The set: PostgreSQL is the only datastore and Redis is queues-only, so the forecast is a database query and nothing else without a platform sign-off; scheduled work is BullMQ repeatable or delayed jobs, never machine cron; every background job carries a deterministic id and checks for prior completion; financial rows are immutable once issued and corrections are new documents; payment authority is non-null and may be a standing-instruction id; counterparty contact and bank data never appear in logs, exports, or error bodies; data stays in UK/EU; Stripe is pinned to API version 2024-06-20 with an 8-second timeout and one retry reusing the idempotency key; errors are RFC 7807 problem documents from the single shared filter; new behaviour ships with tests and coverage cannot fall below 75%, with integration tests against a real PostgreSQL; cut-off is stored as a time without a zone and route days as ISO weekday integers; a material share of retailers have no saved payment method; the workers run on one machine; the team is four engineers with no dedicated ops; Dutch onboarding in Q4 2026 makes the multi-offset problem near-term rather than hypothetical.

**Decisions**, each weighed against at least one real alternative with trade-offs and a reference to the constraints that shaped it:

1. *How occurrences get scheduled* — a periodic sweep that finds due instructions, versus one delayed job per future occurrence, versus pre-materialised occurrence rows plus a sweep. Trade-offs around timezone correctness, visibility of a "skip", and what happens to queued work when a supplier moves their cut-off.
2. *How a basket edit applies from the next occurrence* — a versioned template, effective-dated lines, or mutate-in-place with an audit trail. Interacts with the history requirement and with the forecast's accuracy definition.
3. *Cut-off timezone resolution* — add an explicit timezone to the supplier, assume a single jurisdiction, or convert to UTC at write time. Recommended direction: explicit timezone, because Q4 makes the single-jurisdiction assumption expire on a known date.
4. *Direct-debit late failure* — treat acceptance as success and handle the webhook as a separate downstream event, restrict standing orders to card, or pre-notify. Blocked on the Phase 6 ruling.
5. *Retailers without a saved method* — exclude them, or separate placement from charging so invoice-terms retailers get the order and the existing monthly statement. Blocked on the Phase 6 ruling.
6. *Delisted or repriced products at placement* — place short, place at the new price, or hold the occurrence for a human. Blocked on the Phase 6 ruling.
7. *Forecast computation* — on-demand aggregate query versus a nightly snapshot, judged against the single-datastore constraint and the freshness target from Phase 5.

**Infrastructure provisioning** derived from those constraints and the quality targets: worker capacity or failover covering the cut-off window (this is the concrete answer to contradiction 7 and it costs money, so it is a stop); queue-lag alerting tuned tighter than the standing 10-minute threshold during cut-off windows, because 10 minutes of lag at a cut-off is a missed cut-off; a dead-letter path and an operator-safe replay for placement jobs that respects the once-only guarantee; a concurrency cap on outbound charges; a dashboard and alert on placement success rate and on due-but-not-placed; the schema migration for the new tables, with no backfill.

## Phase 5 — Quality targets

The spec's quality section is four sentences of adjectives — "feel fast", "highly available", "secure", "accurate" — and I would refuse to carry any of them through as written. Each becomes a numbered target with a figure, a way of measuring it, and a justification, and each is recorded against the architecture concern row it belongs to rather than in a standalone file:

- Editing latency ("feel fast") → a p95 figure on the edit path under a stated concurrency, measured by the existing traces, attached to the observability row.
- Cut-off availability ("highly available") → a success rate for occurrence placement within a defined cut-off window, plus a punctuality target expressed as minutes after the cut-off instant, attached to the scheduled-work row. Justified from the spec's own reliability criterion and the fact that lateness equals total failure here.
- Duplicate placement → a hard zero, measured as a uniqueness assertion per instruction per occurrence date, attached to the scheduled-work row. This is the one target with no tolerance, because the operating manual is explicit that a duplicate order is a real van at a real café.
- Charge-failure notification → hours from failure detection to notification, attached to the payments row, with the direct-debit case called out as measured from webhook receipt rather than from placement.
- Payment security ("secure") → restated as the concrete obligations already binding: no card or bank number stored, no counterparty data in logs or error bodies, EU-region processing, full audit coverage of financial mutations. Attached to the payments and audit rows.
- Forecast ("accurate") → a staleness bound and a correctness definition naming which future events are reflected, plus a response-time figure at a realistic supplier's data volume, attached to whichever row the forecast lands under.
- Retention → seven years for the financial records an occurrence produces, with the instruction's own retention stated separately.

I would also record the two adoption and support-ticket criteria as measurement requirements, since both name a data source that has to exist for anyone to read them.

## Phase 6 — The stops

I would not resolve any of these myself. For each: what I would ask, and where the work goes either way.

1. **Priya — who can have a standing order?** Confirm whether the feature is card-on-file only, or whether invoice-terms retailers get placement without a charge. *If card-only:* creation carries a hard precondition, roughly 13% of retailers are excluded, and someone needs to own the message they see. *If placement and charging separate:* the money-authority principle still needs satisfying for a placed-but-uncharged order, and Tom must confirm the statement flow absorbs it.
2. **Priya and Tom — what does "told the same day" mean for direct debit?** *If same-day is mandatory:* standing orders are card-only and this merges with stop 1. *If late notification is acceptable:* the webhook becomes a first-class part of this feature and an unwind decision is needed for a delivery already in motion against a charge that failed three days later.
3. **Priya — delisted product or a material price change at placement.** *Place short / place at new price / hold for a human* — each produces a different requirement and a different notification. **My default while waiting:** price at catalogue price at placement, skip unavailable lines, notify the retailer, and never place an empty order.
4. **Tom — is a standing instruction acceptable as the recorded authority for a charge, and does a failed-charge-but-placed order need a correcting document?** The principle already permits a standing-instruction id, so I expect confirmation; the correcting-document half is the open part.
5. **Whoever owns the architecture spine — may I amend it, and will you fund worker redundancy?** Quality targets belong on the concern rows, and there is no existing row for scheduled *financial* placement or for the forecast. *If I may write:* I add the targets and open new rows marked as needing a ruling. *If not:* I draft the rows in the constraints document as a proposal and the spine stays untouched. **Default: propose, do not edit**, since a ratified store is not mine to change unilaterally. The redundancy question is a cost decision for a team with no ops, and I would present it as "the single worker machine is the feature's largest availability risk" rather than deciding it.
6. **An engineer with repo access — does the immutability trigger permit `placed → cancelled` on an order?** The excerpt cannot answer it, and it determines the only remedy available for a wrongly-placed occurrence. **Default:** assume cancellation is permitted, and flag the assumption prominently as load-bearing.

I would plan the work so that none of the six blocks the bulk of it: the requirement decomposition, the constraint set, and most quality targets stand regardless of the rulings. Only the payment-eligibility, direct-debit, and product-availability decisions stay open, and I would write those as decisions-pending rather than leaving holes.

## Phase 7 — Handoff

Report to Priya and the four engineers: the three artifacts and their paths; the boundary that design artifacts are deliberately next week's work; the eight contradictions between a spec marked "no open questions" and the product as built, with the four that need a product ruling called out first; the three decisions left open pending those rulings; and the one assumption I am carrying on faith about order cancellation. The headline I would lead with is that the spec's central assumption — every retailer has a saved payment method — is contradicted by the schema for around 245 retailers, because that single point reshapes FR-002 and FR-005 and is cheapest to settle before design starts.

## On delegation

Normally I would push the locate-and-enumerate work — find where payment authority is validated, list existing repeatable job definitions, locate the immutability trigger, confirm the job-id lint rule — onto throwaway cheap readers so the sweep never touches my context. **Here I would not delegate anything.** This workspace is five files totalling a few hundred lines, all of them interpretive documents I need in my own head to do the translation, and I have read all five. Spawning a reader would cost more than it saves. The one case that would change it: if the analysis is later pointed at the real `backend-service` repository, I would dispatch cheap readers on a haiku override for the six factual lookups above — one gap per reader, each briefed to return the file, line, and the quoted span and nothing else — and I would check on return that each came back with a concrete location rather than a summary, treating any "could not find" as unresolved rather than as absence.