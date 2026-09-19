FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/docs/supplier-systems.md`, `pallet/ops/hosting.md`, `pallet/specs/live-stock/spec.md`

---

# Action plan — write down quality targets + infrastructure and integration needs for FEAT-017 (live stock)

The deliverable is a document Ops can size from, not code. One new file:

**`pallet/specs/live-stock/quality-and-infrastructure.md`** — a companion to the signed-off spec, with numbered, measurable requirements (`NFR-nnn`, `INF-nnn`, `INT-nnn`) each traced back to an FR/US or to a line in the ops notes.

I would **not** edit `spec.md` (signed off 2026-09-07, owner Priya) and **not** edit `architecture/spine.md` (its rulings need the team's sign-off). Proposed changes to both go inside the new doc as drafts for someone else to accept.

No subagents or workers for any phase. This is a single reconciliation of five short documents; splitting it across workers would lose exactly the cross-document contradictions that are the point of the exercise (spec says "not sensitive" ↔ hosting says there is nowhere to put OAuth tokens; spec assumes product codes match ↔ supplier survey says nine suppliers' codes are unmapped).

---

## Phase 1 — Establish the baseline and list what is genuinely unknown

**Read (done):** the five files above. Nothing else in the workspace exists (glob returns only those seven paths, of which README/CLAUDE cover the stack and governance).

**What I take as fixed facts, with their source:** four engineers and no dedicated ops (README); £300/month infra cap with £212 spent, so £88 headroom (hosting.md); `api` is the only public ingress (hosting.md, spine.md); the worker's single egress IP `149.248.201.77` is what Northern Provisions allowlists and changes need 10 working days (hosting.md, supplier-systems.md); there is no store today for per-supplier credentials (hosting.md, AX-013 note); `Product` has no stock fields and `supplier_sku` is populated for 61% of products (schema.prisma).

**What I would mark "to measure" rather than invent.** I have no shell and no database here, and guessing these would poison the sizing. The doc lists each with the exact query or check for whoever runs it:
- product count per supplier for the 38 Lightspeed and 21 Shopify suppliers (`select supplier_id, count(*) from products group by 1` filtered by `inventory_source`) — drives requests-per-poll via the 250-row page size;
- observed Shopify `inventory_levels/update` event rate per shop — drives inbound webhook sizing;
- current catalogue p95 latency — the AX-010 caching trigger is stated as 400 ms and I need the starting point;
- current Upstash command volume and plan terms — polling adds job traffic against a per-command price;
- whether these Shopify suppliers' plan exposes any REST inventory read (the survey says no history or bulk endpoint; a plain read endpoint would change the recovery story completely).

Everywhere a number depends on these, the doc gives the formula plus a worked example at a stated assumption, so the figure updates once measured instead of being re-derived.

## Phase 2 — Turn the four vague sentences in "Quality expectations" into targets

The spec's quality section is four sentences and one of them is wrong. I would convert each and say so plainly.

| Spec says | Becomes |
|---|---|
| "Stock should update in near real-time" | Per-source staleness budgets, because the sources differ by four orders of magnitude |
| "Sync must be reliable" | Per-source success rates, failure detection thresholds, and what the retailer sees on failure |
| "We shouldn't hammer suppliers' systems" | A hard cap as a fraction of Lightspeed's documented 60 req/min-per-token limit |
| "Stock levels are not sensitive, so no special handling is needed" | Correct for stock values; **wrong for the credentials that fetch them**. See Phase 4 |
| "500 connected suppliers within a year" | A capacity model — and a flag, since there are 140 suppliers in total today |

**Staleness (NFR-001…004)**, measured as time from the change in the supplier's system to it being visible on the catalogue, using each source's own timestamp (`updated_at` for Shopify, the poll's read time for Lightspeed, the file's drop time for SFTP):
- Shopify: p95 ≤ 60 s, p99 ≤ 5 min.
- Lightspeed: p95 ≤ poll interval + 60 s; at a 5-minute cadence, p95 ≤ 6 min.
- SFTP CSV: ≤ 26 h, and the catalogue must show an "as of" date rather than implying live data.
- Manual/always-available: no target; show when it was last set.

**SC-002 "Stock shown is never stale" is not achievable and I would not write a target that pretends otherwise.** A nightly CSV is up to 26 hours old by construction, and a missed Shopify webhook is unrecoverable on those suppliers' plan. The doc proposes replacing SC-002 with "stock is within its source's published staleness budget 99% of the time, and the catalogue shows how fresh it is."

**Reliability (NFR-005…008):**
- Lightspeed poll success ≥ 99.5% per supplier per day; a connection goes *degraded* after 3 consecutive failures and *reconnect-needed* on any auth failure, with the supplier e-mailed.
- Shopify: ≥ 99% of received webhooks verified and applied within the staleness budget; signature failures alert rather than silently drop.
- SFTP: file present on ≥ 90% of nights — I set this from the observed 26 of 30, not from a wish — plus a truncation guard: reject a file whose row count is below 80% of the trailing 7-run median, alert, and keep the previous data rather than zeroing stock. Two truncated files in 30 nights, applied naively, means products showing out of stock and unsellable.
- Ordering: a stock write is applied only if its source timestamp is newer than the stored one. Shopify explicitly delivers out of order, so arrival order must never win.
- Idempotency per the operating manual: every job carries a deterministic id (the `PLT002` lint rule already blocks otherwise); re-running a poll or replaying a webbook changes nothing.

**Read path (NFR-009…010):** catalogue p95 stays under 400 ms — the AX-010 trigger — and stock must not add a per-product query. Basket and checkout re-check stock (FR-005) with their own budget. I would state the schema consequence: stock lives in one row per product joined in the existing single catalogue query, so AX-010 stays "not-now"; if the design instead reads per product, the caching trigger fires and that is extra work Ops must size.

**Two behaviour gaps I would raise, not silently resolve:**
- FR-001 lists four availability states with no *unknown/stale* state. With these sources there will be broken feeds. Default proposal in the doc: show last-known stock with a staleness note and keep the product orderable, rather than blocking sales because a supplier's till broke. **Stop point:** Priya rules. If she wants blocking instead, the doc's alerting requirements tighten (a broken feed becomes revenue-affecting and needs a same-day response, which conflicts with "on-call is whoever is awake") and I would add that as a named risk.
- "Low" stock has no threshold. Default: a per-supplier default with a per-product override, supplier-set. If Priya wants a platform-wide constant, one line changes.

## Phase 3 — Capacity model

Written as formulas with a worked example, in the doc:

- Lightspeed requests per poll per supplier = `ceil(products / 250)`. At an assumed ~300 products that is 2. 38 suppliers at a 5-minute cadence ≈ 76 requests per 5 min, ~22k/day. Per supplier that is 0.4 req/min against a 60 req/min per-token limit — under 1%. The rate limit is per token, so suppliers do not contend with each other; **the binding constraint is our worker, not their API.** I would set the cap at 25% of the per-token limit as the never-exceed line and note there is room to poll every 2 minutes if Priya wants tighter freshness.
- At the 500-connected target, holding today's source mix (38 of 68 connectable are Lightspeed, ~56%), that is ~280 pollers → ~160k requests/day, ~110 jobs/min. At 1–2 s per job that needs a concurrency of roughly 4 on a dedicated queue, alongside the existing invoice/e-mail/PDF/export/purge work.
- Redis: ~10 commands per job → ~220k commands/day today, ~1.6M/day at target. Flagged as a cost line to check against the Upstash plan before it surprises anyone.
- Postgres: writes only on actual change (compare-and-set), so write volume tracks stock churn, not poll frequency.

**Conclusion I would state:** one worker machine is enough for today's 68 connectable suppliers, and Ops' preference to avoid a second machine holds. The revisit trigger is the existing queue-lag alert (10 min) or sustained worker CPU above 70%, whichever comes first. The £14/month second machine fits the £88 headroom — but see the next phase, because adding one is not purely a money question.

## Phase 4 — Infrastructure needs

**INF-001 — Inbound webhook ingress.** Shopify pushes to a URL we register per shop, and only `api` has a public address. So webhook receipt is an `api` endpoint that verifies HMAC-SHA256 with the shop's secret in constant time, returns 200 fast, and enqueues to BullMQ; all real work happens on the worker. This is the first unauthenticated public endpoint on `api` and its error responses must stay within the RFC 7807 shape without leaking whether a secret matched.

**INF-002 — The egress IP is a hard constraint, and it is the sharpest one in this card.** Northern Provisions' firewall allowlists `149.248.201.77`, the worker's dedicated IPv4, and changing that allowlist takes 10 working days' notice to their IT. A second worker machine gets a different egress address, so the SFTP job would start failing from a machine the firewall does not know. The doc states: if a second worker is ever added, the SFTP job must be pinned to the machine holding the allowlisted address, or a second address must be allowlisted with 10 working days' lead time — booked *before* the machine is provisioned, not after. The cost of an additional dedicated IPv4 is not in hosting.md; flagged for Ops to price. The lead time matters more than the cost.

**INF-003 — Per-supplier credentials have nowhere to live.** This is the item the spec's "no special handling is needed" hides. We would hold, per supplier: Lightspeed OAuth access and refresh tokens, Shopify per-shop webhook secrets, and an SFTP password. AX-013 covers platform credentials in Fly secrets and explicitly notes per-tenant credentials have no ruling and nothing holds any today. Fly secrets are per app and do not scale to hundreds of tenants with rotating values. **Recommendation: envelope encryption in PostgreSQL** — ciphertext in a table, the data key in a Fly secret — because it keeps the "one datastore" principle intact and needs no platform sign-off for a new datastore, unlike a managed secrets service which adds cost, a vendor, and operational surface a team with no dedicated ops has to carry. **Stop point:** this needs a recorded platform ruling. If the team prefers a secrets manager, the sign-off must be recorded in `architecture/spine.md` per the one-datastore principle, and the monthly cost comes out of the £88 headroom. I would draft both spine rows and apply neither.

**INF-004 — Rate limiting.** AX-011 is "not-now" with the trigger "the first integration, inbound or outbound, with a documented rate limit." Lightspeed's 60 req/min per token is exactly that, so **this feature fires the trigger.** The doc says AX-011 now needs a ruling and proposes its content: outbound token-bucket per supplier token, honour `Retry-After` on 429 with backoff, plus a cap on the inbound webhook endpoint. Ops should size the ruling as work, not assume it is free.

**INF-005 — Secrets and logging hygiene.** Extend the shared logger's redaction list to tokens, shop secrets, and the SFTP password; keep them out of exports and out of every error body — the contract suite already asserts no e-mail or account number in error bodies and gains cases for these.

**INF-006 — Observability.** New per-source metrics (staleness p95/p99, poll success rate, 429 rate, webhook verification failures, nightly file present/absent and row count) and new alerts: a source stale beyond its budget, a connection in reconnect-needed, and the CSV missing or short. Existing 5xx and queue-lag alerts carry the rest. Named honestly: on-call is "whoever is awake," so these alerts must be actionable in the morning, not paging at 02:00 for a file a supplier's IT controls.

**INF-007 — Scheduling** uses BullMQ repeatable jobs per AX-006; no cron on the machines.

**INF-008 — Cost summary.** Today's design adds no machine: £212 unchanged, plus Redis command growth to verify. The 500-supplier scenario adds one worker at ~£14/month plus the egress consideration in INF-002, landing near £226 against the £300 cap.

## Phase 5 — Integration needs, per source

**INT-001 Lightspeed (38 suppliers).** OAuth 2 per supplier; the access token lives 24 h and **the refresh token rotates on every use**, so a lost rotation forces the supplier to reconnect by hand. That demands single-flight refresh (one refresh in progress per supplier, others wait) and persisting the new refresh token before the request that uses it — otherwise a crash mid-refresh costs a manual reconnect. Plus: pagination at 250, honour `Retry-After`, a `reconnect-needed` state with supplier notification, and use of their sandbox for the integration tests (which, per the operating manual, run against real PostgreSQL).

**INT-002 Shopify (21 suppliers).** Per-shop webhook registration and de-registration; HMAC verification; tolerate 19 retries over 48 h (idempotent by shop + inventory item + `updated_at`); tolerate out-of-order delivery. **A missed event is unrecoverable** on these suppliers' plan — no history, no bulk endpoint — so I would write that as an accepted, bounded risk rather than promise a reconciliation we cannot build, and add the Phase 1 check on whether a plain REST inventory read exists, since that would let us add a low-frequency reconciliation sweep and change the risk entirely.

**INT-003 SFTP CSV (9 suppliers, via Northern Provisions) — I would recommend this is not in the first release.** The survey is explicit: the Sage export uses Northern Provisions' internal codes, they do not match the SKUs those nine suppliers print on their price lists, and nobody has mapped them. That directly contradicts the spec's assumption that Pallet product ids can be matched to supplier codes. Without the mapping there is nothing to join on, so the integration cannot produce correct stock at all — and a wrong mapping shows the wrong product out of stock, which is worse than showing nothing. **Stop point for Priya and Ops:** either (a) fund the mapping as a separate piece of work with a named owner at Northern Provisions and treat SFTP as a later phase — my default, and what I would write the doc around — or (b) keep it in the first release, in which case the mapping exercise, its ownership, and the 10-working-day firewall lead time all belong in the estimate, and I would size it as such. Either way the connection details (nightly ~02:00, missing 4 of 30 nights, truncated twice, allowlisted egress) are written down so nothing is re-discovered later.

**INT-004 Product matching generally.** `supplier_sku` is set on 61% of products, so roughly two in five products have nothing to match on even for Lightspeed and Shopify. The feature needs an unmatched state and a supplier-facing way to map codes. This is integration work Ops must size and it is not mentioned anywhere in the spec.

**INT-005 Unconnected suppliers (72).** Manual entry and always-available (FR-004) need no infrastructure — worth stating, because it is the majority of suppliers today and the cheapest path to SC-001.

**INT-006 Override provenance.** FR-003 requires an override recorded against the person who made it. AX-008 audits financial records only and says stock is not audited today, so this is a new record with an actor, not something the existing audit trail gives us — a small but real schema and API addition.

## Phase 6 — Decisions, blockers, and the spec changes I would ask for

A closing section listing, each with a recommendation so the team is ruling on a proposal rather than an open question:

1. SC-002 rewritten to a bounded staleness target (Priya).
2. A fifth availability state for stale/unknown, and whether it blocks ordering (Priya) — default: does not block.
3. The "low" threshold (Priya).
4. Where per-supplier credentials live, needing a platform ruling and a new AX-013 line (team).
5. AX-011 rate limiting moves from not-now to decided (team).
6. SFTP phase-two vs. fund the code mapping now (Ops + Priya) — default: phase two.
7. The 500-connected-suppliers figure: there are 140 suppliers in total today, of which 68 have a connectable system. 500 connected within a year implies supplier growth well beyond anything in the README, and it is the number driving the capacity model. **Stop point:** if it means "500 suppliers on the platform, of which perhaps 250 connected," the infrastructure conclusion is unchanged and one worker still suffices; if it genuinely means 500 *connected*, the Phase 3 numbers hold as written. I would model the higher figure by default — sizing against the larger number is the safe direction — and say plainly which reading I assumed.

## Phase 7 — Checks before handing over

No code, so no test suite to run. The checks are on the document:
- every FR-001…005 and every user story is covered by at least one numbered requirement, and every requirement traces to an FR or to a line in the ops notes;
- every quality target has a threshold, a measurement source, and a stated time window — no target of the form "should be fast";
- every number is either derived from a file I read or marked "to measure" with the query to run; no invented product counts, event rates, or Fly prices;
- nothing in the doc contradicts the operating manual (one datastore, idempotent jobs, RFC 7807 errors, 75% coverage, real-PostgreSQL integration tests) — and where the feature forces a change to an architecture row, it is drafted for sign-off rather than applied;
- `spec.md` and `spine.md` are untouched.

**Final report to Ops:** one new file at `pallet/specs/live-stock/quality-and-infrastructure.md`; the headline being that the feature is affordable on today's infrastructure but has three items that are not free — nowhere to store per-supplier credentials, a rate-limiting ruling this feature triggers, and an egress-IP constraint with a 10-working-day lead time on any second worker machine; that SC-002 as written cannot be met and needs replacing; and that two things block part of the scope — the unmapped Northern Provisions product codes for nine suppliers, and the 39% of products with no `supplier_sku` to match on. Plus the list of seven decisions with my recommendation on each, and the short list of measurements to take before the estimate is finalised.