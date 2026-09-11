FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/specs/standing-orders/spec.md`

---

# Action plan — technical analysis of FEAT-014 Standing orders

## Scope I would hold to

The card asks what the system technically has to do, the constraints it hits, and the quality targets — before design next week. So this pass produces **analysis artifacts only**: technical requirements, a constraints-and-decisions document, and quality targets. I would **not** write the entity model or the API contract in this pass; they are next week's work and three of them are blocked on rulings I surface below. I would record that deferral explicitly so nobody thinks the set is complete.

Intended output paths:
- `pallet/specs/standing-orders/technical-requirements.md`
- `pallet/specs/standing-orders/constraints-and-decisions.md`
- Quality targets appended to the concern rows in `pallet/architecture/spine.md` (these live on the architecture rows, not in a separate file) — see Phase 7 for the ownership problem this creates.
- `pallet/specs/standing-orders/open-rulings.md` — the questions I need answered, since the spec says "no open questions at sign-off" and I am going to reopen several.

---

## Phase 1 — Load the procedures, then sweep the workspace

**Do:** Load the four skills that carry the actual formats — the requirements-authoring one (for the requirement, constraint, decision and infrastructure id grammar), the technical-decisions one (for the decision record shape and the two-alternatives rule), and the entity-modeling and API-contract ones read-only, so that the declarations I leave behind next week's designers can pick up match what they will need.

**Read:** already done — all five workspace files, in full. The workspace is five files; there is no large surface to sweep, so most of the reading is mine and stays mine.

**Delegate:** one disposable `Explore` subagent, `model: haiku`, brief: *"In `pallet/`, list every file and line mentioning any of: `authorised_by`, `idempot`, `jobId`, `cutoff`, `UTC`, `timezone`, `payment_method`, `retention`, `redact`, `standing`. Quote `file:line` verbatim. Do not interpret or summarise."* On return I check: does it name only the five files I already opened (if it names a sixth, I open that one myself), and do its quotes match what I read. This is a deterministic collision-and-completeness check, not a substitute for my own reading of the governance.

**Refuse/flag:** nothing yet.

---

## Phase 2 — Reconcile the signed-off spec against the product as built

This is the core of the work and where most of the value is. I would build a contradiction register before writing a single requirement, because several spec statements are false against the schema and the architecture rows.

**What I would produce (in `open-rulings.md`), with the branches I would follow for each:**

**R-1 — the payment-method assumption is untrue. Blocking.**
The spec assumes "every retailer has a saved payment method, so every occurrence can be charged at placement." The schema says `payment_method` is nullable and is null for *every* one of the 214 `invoice_30d` retailers and 31 `card_on_file` retailers who never finished setup — roughly 245 of 1,900 retailers, about 13%. FR-002 as written cannot execute for them.
- *Stop:* confirm with Priya (product) and Tom (finance).
- *Branch A — restrict:* standing orders are only available to retailers with a usable saved method; setup refuses with a specific, actionable error and a path to add a method.
- *Branch B — split by terms:* `invoice_30d` retailers get standing orders that place the order and fall onto the monthly statement, with no per-occurrence charge. This splits FR-002 into two requirements and means the authority principle is satisfied by the standing instruction alone, with no payment row at placement.
- *Branch C — defer* invoice retailers to a later feature.
- *I proceed under A* as the default: it is the narrowest reading of FR-002 and does not invent a billing path that finance has not agreed. I would tell Priya that B is probably what she wants (214 retailers is not a rounding error) and that B costs one more requirement, not a redesign.

**R-2 — "notified the same day" is unachievable for Bacs. Blocking for FR-005.**
AX-003 says a Bacs debit is only *accepted* at submission and can fail up to three working days later, reported by the `payment_intent.payment_failed` webhook — and that today this only happens for one-off orders "where the retailer is on the page." A standing-order occurrence is unattended by definition, so the webhook path has to grow a new consumer.
- *Stop:* confirm what "same day" binds to.
- *Branch A:* same day as Pallet **learns** of the failure (same-day for card, up to three working days after placement for Bacs). *This is my default* — it is the only one that is physically true.
- *Branch B:* product genuinely means same day as placement, which would mean not offering Bacs for standing orders at all — a product decision, not a technical one.
- Either way this is **drift against AX-003's as-built note**, which I would record.

**R-3 — cut-off times have no timezone. Blocking for FR-002 correctness.**
`Supplier.cutoff_time` is a bare `@db.Time` "stored as typed on the supplier's settings page," and the worker machine runs in UTC. UK and Ireland run BST/IST for about seven months of the year, so a cut-off typed as 18:00 in London is 17:00 UTC in summer and 18:00 UTC in winter. Scheduling placement off the naive value fires an hour wrong for half the year — and a placement an hour late is a missed cut-off, which the overview says is "a morning without bread." Dutch producers onboard Q4 2026, adding a second offset, so this cannot be left implicit.
- *Stop:* confirm the stored value is London-local as typed.
- *Branch A (default):* interpret existing values as `Europe/London`, and require a per-supplier timezone before the Dutch onboarding — carried as a decision and an infrastructure-touching migration constraint.
- *Branch B:* values are already UTC as typed, in which case Irish and future Dutch suppliers are already wrong and that is a pre-existing defect I would raise separately.

**R-4 — catalogue drift at occurrence time. Spec is silent.**
Products change price "roughly monthly" and can move to `paused` or `delisted`. The spec assumes "prices are those in the catalogue" and says nothing about a line whose product has gone away. *Default:* place the occurrence with the remaining lines, drop the unavailable line, and tell the retailer what was dropped in the occurrence summary; never silently substitute, and never silently place a zero-line order.

**R-5 — unbounded automatic charge. Finance risk.**
Because prices float and nobody approves each occurrence (approval is explicitly out of scope), a supplier price rise charges the retailer's card automatically with no ceiling. *Default:* place and charge at catalogue price, but I would flag to Tom and recommend a variance threshold (hold or notify when an occurrence exceeds the previous one by more than a stated percentage) as a cheap decision now rather than an incident later.

**R-6 — forecast confidentiality.** The governance calls trading relationships Confidential. FR-007 aggregates several retailers' standing orders for a supplier. *Default:* the forecast returns quantities per product per route day only, with no retailer identity or per-retailer breakdown, and no counts small enough to be self-identifying if only one retailer is standing-ordering a product. I would flag the small-number case for a ruling rather than guessing.

**R-7 — what "accurate" and "fast" mean.** The quality section is entirely unmeasurable as written ("feel fast", "highly available", "secure", "accurate"). I do not pass these through; Phase 7 converts each into a number with a measurement method, and I would take the justification for each back to Priya.

**Refuse:** I will not write requirements that silently assume the payment-method assumption holds. If R-1 cannot be answered this week, the technical requirements ship with the eligibility branch marked provisional rather than resolved by my guess.

---

## Phase 3 — Technical requirements (`technical-requirements.md`)

Decompose each business requirement into what the system must actually do, each traced back to its source requirement and each carrying an acceptance check. Planned decomposition:

- **From FR-001 (setup):** eligibility check against payment terms and saved method (per R-1); validate the basket is a single supplier; validate chosen days are a subset of that supplier's `route_days`; compute and return the next occurrence date from route days, cut-off time and timezone; persist the instruction recording who created it and when — which is what the governance's "recorded standing instruction that names who set it up and when" demands.
- **From FR-002 (placement and charge):** detect due occurrences at each supplier's cut-off on the day before a route day; give every occurrence a deterministic identity of standing-order-plus-occurrence-date, enforced by a uniqueness constraint in Postgres, not only by a job id — the job id satisfies the idempotency rule, the constraint is what actually stops a second van; materialise an order snapshotting current catalogue prices into the line unit prices; set delivery date to the route day; set the order's and payment's authority to the standing instruction; charge through the existing payments module with an idempotency key derived from the same occurrence identity; write the audit row in the same transaction.
- **From FR-003 (skip/pause):** skip the next occurrence up to the cut-off; pause and resume at any time; and explicitly, the race where a skip arrives while the occurrence job is already running — one of the two must lose deterministically, and since orders are immutable once issued, the skip must lose after placement begins and the retailer must be told.
- **From FR-004 (edit):** changes take effect from the next occurrence and never touch a placed order; the change history records actor and time — which means audit coverage has to extend beyond the orders/invoices/payments that AX-008 says it covers today.
- **From FR-005:** two failure paths, the synchronous card decline inside the placement transaction and the asynchronous Bacs webhook arriving days later with nobody on the page; in both the order stands and the retailer is notified.
- **From FR-006:** a per-occurrence summary despatched via the existing mail path, sent at-most-once across job re-runs.
- **From FR-007:** the four-week forecast honouring pauses, skips and pending edits, aggregated only.
- **Cross-cutting, which the spec never mentions and I would surface:** error bodies in the house problem-details format with no contact or mandate details in them; redaction of the new fields in logs; a retention policy for standing orders themselves (they are not financial records, so the nightly purge does not automatically skip them, but they are referenced by records retained seven years); an injectable clock, because the spec's own independent test says "advance to the cut-off"; and burst behaviour, since suppliers cluster on common cut-off times so placement load is spiky rather than smooth.

Also recorded here as thin declarations for next week: the integration boundaries (Stripe charge and its webhook, the mail sender) with their failure behaviour, and the data-sensitivity notes for the new elements (the standing order carries no payment credentials — only the existing Stripe method reference — but it does encode a trading relationship, which is Confidential).

**Test:** none run in this phase; these are requirements. Each carries the check that will prove it, and I would note that the spec's three independent tests become integration tests against real Postgres with a controllable clock, per the house testing rule.

---

## Phase 4 & 5 — Constraints and decisions (`constraints-and-decisions.md`)

**Constraints I would record, each traced to a real source in this workspace, not to a preference:**
- Orders and payments cannot be edited once issued; any correction is a new document — so a skip is only meaningful before placement.
- Every payment and payout needs a non-null authority; the governance already anticipates a standing-instruction id in that field.
- Background jobs must carry a deterministic id and check for prior completion.
- Postgres is the only datastore; Redis is queue-only, so occurrence state cannot live in Redis.
- No cron on the machines; periodic work is a repeatable queue job.
- Stripe SDK pinned to `2024-06-20`, 8s timeout, one retry reusing the idempotency key — the placement path inherits this budget.
- Bacs failures surface up to three working days after acceptance.
- Cut-off stored without timezone (R-3).
- Ids are prefixed ULIDs, so the new record needs a prefix.
- Data stays in UK/EU; four engineers and no dedicated ops, which is a real ceiling on how much operational machinery any decision may introduce.

**Decisions I would write, each weighed against at least one real alternative:**
1. **How occurrences get scheduled** — a single repeatable sweeper that scans Postgres for due occurrences, versus one delayed job scheduled per future occurrence. The sweeper survives Redis loss because the state is in Postgres, and it naturally picks up a supplier changing their cut-off or route days; pre-scheduled delayed jobs go stale the moment a supplier edits their settings, and would put scheduling state in a store the governance says is not the system of record. I expect to land on the sweeper, but I would write both out with the trade-off on timing granularity.
2. **Whether occurrences are materialised rows ahead of time or computed** — a persisted occurrence ledger gives the uniqueness constraint that prevents duplicate placement, gives skip something to attach to, and gives the forecast and the success-criteria measurement something to read. Computing on the fly avoids a table but has nowhere to record "this one was skipped."
3. **What goes in the order's authority field** — the standing-instruction id (consistent with how the payment field is already documented) versus the retailer user who set it up. Either satisfies the principle; the first keeps one hop to a named person and does not pretend a human acted that morning.
4. **How edits interact with occurrences** — reading the live template at run time versus snapshotting a version per occurrence. "Applies from the next occurrence" plus an auditable history points at versioning.
5. **Cut-off timezone handling** (per R-3), sized against the Q4 Dutch onboarding.
6. **Bacs late-failure handling for unattended occurrences** (per R-2).

Each decision names the constraints that shaped it and the principle it answers to.

---

## Phase 6 — Infrastructure provisioning

Derived from the constraints and the availability expectation, written into the same document:
- The worker is **one** Fly machine in `lhr`. Placement is the money path on cut-off mornings, and a single machine cannot underwrite a high-availability claim. I would raise a provisioning item for a second worker machine or, if the team declines, a documented and measured recovery time instead — and I would **refuse to write an availability target the infrastructure demonstrably cannot meet**.
- Alerting: the existing queue-lag alert fires at ten minutes, which is a sensible general threshold and a useless one near a cut-off. I would raise a cut-off-specific signal — a due occurrence still unplaced as the cut-off passes — rather than relying on aggregate lag.
- A per-supplier placement outcome metric, because SC-001 ("placed reliably") has no measurement source otherwise.
- Migration items: the new table and uniqueness constraint, the new id prefix, the logger redaction entries, and the export allowlist entries for the new fields.
- Retention: whether the nightly purge touches standing orders.

---

## Phase 7 — Quality targets onto the architecture concern rows

**Do:** attach measurable targets to the relevant rows in `pallet/architecture/spine.md` — payments (AX-003), scheduled work (AX-006), audit (AX-008), observability (AX-012) — and propose a new row for standing-order placement, since no existing row owns it.

Targets I would propose, replacing the four unmeasurable sentences in the spec:
- **Placement completeness:** every due, unpaused, unskipped occurrence produces exactly one order before the supplier's route day begins; zero duplicates; measured from the occurrence ledger.
- **Placement timeliness:** a stated number of minutes from cut-off to order visible to the supplier, justified by when suppliers start picking — I need that number from Priya rather than inventing it; my working default is fifteen minutes.
- **Availability of the placement path during the cut-off window**, stated against whatever the second-machine decision lands on.
- **Portal responsiveness:** a server-side p95 for the standing-order read and edit endpoints at current scale, with the honest note that "feels fast" is a front-end property and the API is only part of it.
- **Forecast:** a p95 response time for the largest supplier's four-week view, and a concrete definition of "accurate" — the forecast matches what is subsequently placed, excluding changes made after the forecast was read.
- **Notification timeliness:** minutes from Pallet learning of a failure to the notice being despatched (not from placement — per R-2).
- **Confidentiality:** no contact detail, mandate reference or card digits in any error body, log line or export for the new surface.

**Stop:** AX-003 and AX-006 are marked *decided · built · Drift: none*. This feature changes the as-built reality of both (Bacs failures now arrive unattended; a new class of scheduled work). Editing a built row is not mine to do unilaterally — the governance requires platform sign-off recorded in that file for datastore changes and treats these rows as rulings. I would **write the proposed rows and mark them pending sign-off**, and confirm with the platform owner whether this is drift to record against the existing rulings or a re-ruling. If told it is a re-ruling, the two decisions move out of my document and into theirs; if drift, I record it against the rows as-is.

---

## Phase 8 — Traceability and self-review

Walk the chain both ways: every business requirement has at least one technical requirement under it, every technical requirement names its source, no orphans in either direction. Confirm every quality target has a number, a measurement method and a justification, and that no target is justified only by "the spec said so." Confirm every external dependency has its failure behaviour written down. Confirm every constraint names a real source in these five files. Confirm nothing I wrote is a technology choice wearing a requirement's clothes.

---

## Phase 9 — What I would report

A short handover to Priya, Tom and the platform owner:

1. **The spec is signed off but three of its assumptions do not survive contact with the schema and architecture** — the payment-method assumption (≈245 retailers cannot be charged), the same-day failure notice (impossible for Bacs by up to three working days), and the cut-off time having no timezone (an hour wrong for seven months of the year, and worse once Dutch suppliers onboard). These need rulings before design starts; I name the default I proceeded under for each so the work is not blocked meanwhile.
2. **Two gaps the spec does not address at all** — what happens when a standing-order product is paused, delisted or repriced, and whether an automatic charge should have a ceiling.
3. **The four quality sentences are now seven numbered targets**, one of which (availability) I have deliberately left open because one worker machine cannot back a high-availability claim, and that is a provisioning decision, not a writing decision.
4. **Two architecture rows now carry drift** and need the platform owner's ruling on whether to amend or re-rule.
5. **What is deliberately not in this pass** — the entity model and API contract, which are next week's design work and are partly blocked on rulings 1 and 3.