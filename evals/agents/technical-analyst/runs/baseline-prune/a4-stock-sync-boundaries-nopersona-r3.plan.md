FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/docs/supplier-systems.md`, `pallet/ops/hosting.md`, `pallet/specs/live-stock/spec.md`

---

# Action plan — FEAT-017 live stock: quality targets + infrastructure & integration needs

**Deliverable framing.** Ops wants numbers they can size against, not an implementation. So this card produces two documents and no code, no schema migration, no test run. I'd say that plainly in the final report so nobody expects a branch.

**Delegation:** none. Seven files, all read; the work is analysis and drafting against material already in context. Spawning a worker would add handoff cost and a second chance to get the arithmetic wrong. I'd do it directly.

---

## Phase 1 — Establish the load model (the one input I don't have)

Everything downstream — poll rate, worker concurrency, Redis command volume, DB write rate — keys off **products per supplier**, which appears in no file I read. `Product` exists in the schema but there are no counts anywhere.

What I'd do:
- Ask for a single number: active products per supplier (median and 90th percentile), from a `SELECT count(*) FROM products GROUP BY supplier_id WHERE status='active'`. I can't run it here.
- I would **not** block on it. I'd carry it as a named assumption — **200 active products per supplier, p90 600** — put it in a clearly marked "Inputs" box at the top of both documents, and state every derived figure as a formula plus the value at that assumption, so ops can re-run the arithmetic when the real number lands.
- Second unknown, same treatment: **Shopify webhook event rate per shop per day**. Nothing in `supplier-systems.md` bounds it. Assume 300 events/shop/day, flag it as the least reliable input, and note it only moves Redis cost and the ingress rate limit, not the architecture.

Branch if the real numbers come back far higher (say 2,000 products/supplier): the Lightspeed poll goes from 1 page to 8 pages per supplier per cycle, and the "one worker is enough" conclusion in Phase 4 needs re-testing. I'd write the doc so that recalculation is a one-line change, not a rewrite.

---

## Phase 2 — Challenge the spec's quality section before writing targets

The spec's "Quality expectations" (spec.md:61-65) is four sentences, three of which can't be sized. I'd resolve these first because the NFR document is mostly the resolution.

**2a. SC-002 "Stock shown is never stale" is not achievable and not testable.** The suppliers' systems make it false on their own terms:
- SFTP/Sage: one file a night at ~02:00. Stock is up to **26 hours** old by construction, and on the observed record (missing 4 of 30 nights, truncated twice) is over 48 hours old roughly **20% of nights**.
- Shopify: a missed webhook is unrecoverable on those plans — no history, no bulk endpoint. Staleness after a miss is **unbounded**.
- Manual entry (72 of 140 suppliers, the majority): as fresh as the supplier's typing.

So a single global freshness target is meaningless. The NFR document replaces SC-002 with per-source targets (Phase 3). I'd write this as a proposed amendment to a signed-off spec, not an edit — see Phase 6.

**2b. "Stock levels are not sensitive, so no special handling is needed" is half right and I'd push back on it.** The stock *values* are low-sensitivity, agreed. But this feature is the first thing on the platform to hold per-supplier OAuth access and refresh tokens, an SFTP password, and Shopify HMAC shared secrets. Those are as sensitive as the Stripe key. The governance line about counterparty data being confidential and the logger redaction list both apply here, and the redaction list does not cover these tokens today because nothing holds any (`spine.md` AX-013 note). I'd correct this sentence rather than inherit it.

**2c. The spec's Assumption 2 — "Pallet product ids can be matched to the supplier's own product codes" — is false, and it's the single biggest sizing risk.** Two separate failures:
- `Product.supplier_sku` is populated for **61% of products** (schema.prisma:30). For the other 39% there is nothing to match on, for any source.
- The Sage/Northern Provisions export uses the wholesaler's internal codes, which do not match the SKUs those nine suppliers print, and **nobody has mapped them** (`supplier-systems.md:11-12`).

FR-002 cannot be delivered for those nine suppliers, or for any product missing a SKU, without work that is not in the spec. **This is a stop-and-confirm point** (Phase 5).

**2d. The spec never says what the catalogue shows when sync is broken.** FR-001 defines four states (in stock / low / out / always available); none of them means "we don't know." This is a product decision, not mine (Phase 5).

---

## Phase 3 — Write the quality targets

**Path:** `pallet/specs/live-stock/nfr.md`

Structure: one row per target, each with an ID (`NFR-0xx`), the number, the source it applies to, how it's measured with the instrumentation that already exists (pino + OTel to Grafana Cloud, AX-012), and what would make it fail. No target goes in that I can't name a measurement for.

Content I'd write:

**Freshness — replaces SC-002, split by source:**
| Source | Suppliers | Target (p95 staleness) | Hard ceiling |
|---|---|---|---|
| Shopify webhook | 21 | 60 s from `updated_at` to catalogue | unbounded after a missed event — see NFR on staleness cap |
| Lightspeed poll | 38 | 5 min | 15 min (three failed cycles) |
| SFTP CSV | 9 | 26 h | 50 h (one missed night) |
| Manual / always-available | 72 | n/a — no freshness claim | — |

Measured as a `stock_age_seconds` gauge per supplier derived from a `stock_confirmed_at` timestamp, exported to Grafana.

**Staleness cap and degradation.** Every displayed availability carries an age. Past the source's hard ceiling the product moves to a degraded state (Phase 5 decides which). Target: fewer than 2% of catalogue product-views served past the hard ceiling, measured monthly.

**Correctness — the target that actually maps to SC-001.** SC-001 measures supplier-side order edits, which is a lagging business metric. I'd add a direct one: **false-in-stock rate** — the share of order lines shown "in stock" at add-to-basket that the supplier later edits or cancels for stock reasons. Target under 2% by month three. This is the number that tells us whether the feature works; SC-001 is what it should cause.

**Politeness to suppliers' systems — quantifying "shouldn't hammer":**
- Never exceed **50% of the documented Lightspeed limit** — i.e. 30 requests/min against a 60/min-per-token ceiling, so a retry storm still can't trip it.
- Always honour `Retry-After` on a 429; exponential backoff with jitter, capped at the source's hard ceiling so backoff can't silently break the freshness target.
- Per-supplier circuit breaker: after N consecutive failures, stop polling that supplier and raise a "needs reconnect" state rather than retrying forever.
- Target: **zero 429s per week in steady state**; any 429 is a bug, not a normal condition.

**Catalogue read performance.** p95 under 400 ms — deliberately set at the AX-010 caching trigger. Note in the doc: reading stock as a joined second row per product also trips the AX-010 trigger ("more than one row per product per request") on its own wording, regardless of latency. I'd flag to the platform reviewer that keeping availability denormalised on the `products` row keeps the catalogue a single query and avoids opening the caching decision as part of this feature. That is a design steer for sizing, not a decision I'd make alone.

**Sync reliability.**
- Poll-cycle success rate ≥ 99% weekly per supplier.
- Webhook processing: every accepted delivery either applied or dead-lettered, none silently dropped; HMAC verification failures alert (a real failure means a rotated secret or a forged request).
- SFTP: given the observed 20% bad-night rate, the target is **not** "the file arrives" — that's outside our control and an alert on it fires six mornings a month. Target is instead: a missing or truncated file is detected before 08:00, the previous good data is retained rather than zeroed, and those nine suppliers' products are flagged stale. Truncation detection needs a rule — I'd propose rejecting a file with fewer than 80% of the prior night's row count.

**Idempotency and ordering** (these are governance obligations, restated as testable targets because they change the sizing):
- Every job carries a deterministic `jobId` (mandatory, lint `PLT002`). For webhooks: shop id + inventory item id + `updated_at`, which makes Shopify's 19 retries over 48 h collapse to one application for free.
- Out-of-order webhook deliveries are explicitly expected. A write with an `updated_at` older than the stored `stock_confirmed_at` is discarded. Test: replay two events out of order, assert the newer value survives.

**Availability, security, observability, testing** — each as a short measurable row: sync subsystem failure must not fail the catalogue read; credentials never in logs, exports, or error bodies (extending the existing redaction list); webhook endpoint errors go through the shared RFC 7807 filter and must not disclose whether a shop id exists; coverage stays ≥ 75%, integration tests against real PostgreSQL.

**Tests these targets imply** (listed so ops can size the test work; I'd write none of them on this card):
- Lightspeed against the **sandbox** the survey confirms exists — pagination, 429 + `Retry-After` handling, token refresh, refresh-token rotation.
- Shopify against recorded fixtures — HMAC verify pass/fail, duplicate delivery, out-of-order delivery.
- SFTP against a local SFTP container in CI — missing file, truncated file, unparseable row.
- One integration test per freshness target asserting the staleness gauge crosses the ceiling when the source is stubbed dead.

**One trap I'd call out explicitly in the doc:** the Lightspeed refresh token rotates on every use, and a lost rotation means the supplier reconnects by hand. Combined with the mandatory-idempotent-job rule, a job that crashes after Lightspeed rotated the token but before we committed the new one leaves a dead credential, and the deterministic `jobId` retry will re-run with the stale token and fail permanently. The token write has to commit before the token is used for anything else, and the job must be safe to re-enter with either token. This costs real engineering time and ops should see it before they size.

---

## Phase 4 — Write the infrastructure and integration needs

**Path:** `pallet/specs/live-stock/infrastructure.md`

**Three integrations, three shapes** — a table per source covering protocol, auth, credential lifetime, failure modes, rate limits, what breaks it, and what a supplier has to do to recover. Drawn straight from `supplier-systems.md` with the operational consequences added.

**Public ingress — new, and it's a change.** Shopify webhooks need a public URL. Only `api` has a public address; the worker has none (`hosting.md:13`, `spine.md:7`). So webhook receipt lands on `api`: verify HMAC, enqueue to BullMQ, return 200 fast. This makes `api` the first app with an unauthenticated public endpoint that isn't a retailer/supplier session. Consequences to size: an inbound rate limit on that route, and a per-shop secret available to `api` (not just the worker).

**Egress IP — a hard constraint ops needs to see spelled out.** Northern Provisions' firewall allowlists `149.248.201.77`, the worker's dedicated IPv4, and nothing else depends on it (`hosting.md:6`). Therefore:
- The SFTP fetch must run from the worker, not `api`.
- Adding a second worker machine risks a second egress address. If the SFTP job lands on it, the fetch fails silently against their firewall, and fixing the allowlist takes **10 working days' notice to their IT** (confirmed 2026-08-19). Either the SFTP job is pinned to the machine holding the allowlisted IP, or we give both machines the same egress, or we accept a two-week lead time.
- This lines up with ops's stated preference not to add a second worker. I'd note the trigger condition that would force one, rather than asserting one is or isn't needed.

**Worker capacity — the arithmetic, shown.**
- Today, Lightspeed: 38 suppliers × ⌈200/250⌉ = 1 page each per 5-minute cycle = 38 requests / 5 min ≈ **0.13 req/s** aggregate; 12 requests per hour per token against a 60-per-minute ceiling. Not close to the limit.
- At the spec's 500 connected suppliers, holding today's mix (56% Lightspeed): ~280 pollers × 1–3 pages / 5 min ≈ **1–3 req/s** outbound. Still far under the per-token limits; the constraint is worker concurrency and outbound socket count, not Lightspeed.
- Conclusion for sizing: **one worker machine plausibly holds this**, with a bounded concurrency (~10 in-flight polls) so the poller can't starve the existing nightly invoice/PDF/export jobs. I'd write the trigger that changes the answer: sustained queue lag above the existing 10-minute alert threshold, or products-per-supplier materially above the assumed 200.

**Storage and DB write load.**
- Worst case if every product changed every cycle: 500 × 200 / 5 min = 20,000 upserts / 5 min ≈ 67 writes/s. Realistically under 1% of products change per cycle, so under 1 write/s — but the *comparison* still reads 20k rows per cycle. Requirement: write only on change, and do not touch `Product.updated_at` on a no-change poll (it's `@updatedAt`, so a blind upsert churns the row and any downstream that watches it).
- Retention: stock is **not** a financial record (AX-008 says so explicitly), so it's outside the seven-year rule and the nightly purge can trim it. Proposed: sync history 90 days, manual override records kept for the life of the product (FR-003 requires "who last changed it"). Needs a ruling — small, but it belongs in the doc.
- Note that FR-003's "who changed it" is not covered by `audit_events`, since that's financial-records-only. It needs its own record. Sizing item.

**Redis / BullMQ.** One job per poll cycle per supplier plus one per webhook. At the assumed 300 events/shop/day × 21 shops ≈ 6,300 webhook jobs/day today; the poll adds ~11,000 jobs/day at 500 suppliers. Upstash bills per command and BullMQ is command-chatty. Requirement: estimate commands/month against the current Upstash tier before committing, because this is the line item most likely to move cost unexpectedly.

**Cost.** Budget £300/month, currently at £212, so **£88/month headroom** (`hosting.md:7-8`). A second `shared-cpu-2x` worker is ~£14/month — affordable if needed. The genuine unknowns are Upstash command volume and Grafana Cloud log/trace volume from a chatty integration (three sources × hundreds of suppliers × structured logs with correlation ids). I'd write these as "estimate before commit," not as a number I invented.

**Secrets — this feature forces a decision that doesn't exist yet.** Fly secrets are per-app; there is no home for per-supplier credentials today, and `spine.md` AX-013 says per-tenant credentials have no ruling and nothing holds any. This feature needs to hold, per supplier: a Lightspeed access token (24 h) and a rotating refresh token, a Shopify shared secret, and an SFTP password. My recommendation, with reasoning: **encrypted columns in PostgreSQL, envelope key in Fly secrets, rotated quarterly like the platform keys.** It adds no datastore, so it stays inside the "one datastore" principle and needs no platform sign-off, and four engineers don't take on a secret manager. The alternative — a dedicated secret store — is a new datastore and requires platform sign-off recorded in `spine.md`, plus cost against the £88.

**Architecture rows this feature moves** — drafted as *proposed* text, not applied:
- **AX-011 rate limiting** is currently "not-now," triggered by "the first integration, inbound or outbound, with a documented rate limit." Lightspeed's 60/min trips it, and the public webhook endpoint trips it inbound too. This feature promotes AX-011 and needs a ruling.
- **AX-010 caching** is triggered by a catalogue reading more than one row per product. Whether this feature trips it depends on the denormalise-vs-join choice above.
- **AX-013 secrets** needs its per-tenant ruling written.
- **AX-006 scheduled work** already covers the pollers: BullMQ repeatable jobs, no machine cron. No change needed — good, it means the nightly SFTP fetch and the 5-minute poll both fit the existing pattern.

**Observability additions.** The existing alerts (5xx > 1% over 5 min, queue lag > 10 min) don't cover any live-stock failure. New signals: per-supplier stock age, poll failure rate, 429 count, HMAC verification failures, token-refresh failures, SFTP file missing/short. Alert design must respect that **on-call is "whoever is awake"** (`hosting.md:14`): a 02:00 SFTP failure has no responder, so it must not page overnight. It should surface as a morning-actionable "these nine suppliers are stale" item, with the product degrading safely in the meantime. A token-refresh failure is different in kind — it needs a human to ask the supplier to reconnect, so it belongs in a supplier-facing state, not only a dashboard.

---

## Phase 5 — Decisions I would stop and confirm

I'd write all four into a "Decisions needed" section with the branches, proceed on the stated default so the documents are complete, and mark every default clearly as mine rather than agreed.

**D1 — The nine Sage suppliers, product-code mapping. Ask Priya and ops.**
- *(a) Defer them to manual entry* (the US-004 path) and drop them from FR-002's scope for v1. **My default.** They're 9 of 140 suppliers on the worst data (a fifth of nights bad, 26-hour freshness at best) behind an unmapped code set. Building the SFTP path first, for the least fresh data, is the wrong order.
- *(b) Fund the mapping.* Then the doc needs a line item: someone maps the wholesaler's internal codes to those nine suppliers' SKUs, by hand, plus somewhere to store the mapping, plus a process for new products. Sizeable and ongoing.
- *(c) Ask Northern Provisions' IT to add the suppliers' SKUs to the export.* Cheapest if they'll do it, but it's their IT, the firewall change alone runs 10 working days, and the export is something "their IT runs" — we have no leverage.
- If ops overrules toward (b) or (c), the SFTP integration stays in v1 and the infrastructure doc's egress-IP constraint becomes load-bearing rather than a note.

**D2 — The 39% of products with no `supplier_sku`. Same conversation.** Unmatched products can't sync at all, on any source. Default: they stay on today's behaviour (status-based, effectively "always available") and the supplier app shows the supplier which of their products aren't matched, so they can fix it themselves. This is what makes SC-003's "50% of active suppliers connected" reachable — worth saying, because a supplier who connects and sees a third of their catalogue unmatched will read that as broken.

**D3 — What the catalogue shows when we don't know. Priya's call, since it adds a state FR-001 doesn't have.**
- Default: show last-known value with its age up to the hard ceiling; past that, "availability unknown — check with supplier," and *allow* adding to the basket with a warning. Rationale: failing closed turns our sync outage into blocked orders for a supplier who has the stock, which is a worse failure than the one we're fixing.
- If Priya rules fail-closed (unknown ⇒ out of stock, cannot add): our sync uptime becomes revenue-critical, several NFR targets tighten a full nine, and a second worker plus the egress-IP resolution in Phase 4 moves from "maybe" to "yes." I'd note that this ruling is the largest single swing in the sizing.

**D4 — Credential storage.** Default: encrypted Postgres columns, no new datastore, no sign-off needed. If the platform reviewer wants a dedicated secret store, that's a new datastore, which needs sign-off recorded in `spine.md` and a cost check against the £88 headroom — and I'd want that recorded before anyone sizes.

---

## Phase 6 — Proposed spec amendments (proposed, not applied)

`spec.md` is signed off (2026-09-07, Priya). I would **not** edit it. Instead, a "Proposed amendments to FEAT-017" section at the end of `nfr.md`, each with the current text, the proposed text, and one line of why:
- **SC-002** — replace "Stock shown is never stale" with the per-source freshness table, since as written it's unachievable on every one of the three sources and can't be tested.
- **Quality expectations** — replace "no special handling is needed" with the credential-handling requirement.
- **Assumption 2** — mark false as stated; qualify to the 61% of products with a SKU, excluding the Sage path.
- **FR-001** — add the fifth availability state, if D3 lands on my default.
- **FR-005** — note that with reservation out of scope, "went out of stock since it was added" requires a re-read at basket render *and* at checkout submit; that's a read-path cost the catalogue NFR has to cover.

I'd hand these to Priya rather than assume them.

---

## Phase 7 — Self-check before reporting

- Re-read both documents against `CLAUDE.md` and `spine.md` and confirm nothing I proposed contradicts a ratified principle — particularly one datastore, mandatory `jobId`, redaction, RFC 7807, 75% coverage, real-Postgres integration tests.
- Verify every number is traceable to a file and line or to a stated assumption. No unsourced figures.
- Confirm each quality target names its measurement and that the measurement uses instrumentation that exists (pino, OTel, Grafana) or is listed as new work.
- Confirm I have not written code, a migration, or edited the signed-off spec.
- No tests run — nothing executable changed. I'd say so rather than imply a green suite.

---

## What I'd report

Two documents written — `pallet/specs/live-stock/nfr.md` and `pallet/specs/live-stock/infrastructure.md` — with:

1. **Four things ops must decide before sizing is meaningful**, D1–D4, with my defaults marked as mine. D3 (fail-open vs fail-closed) is the biggest swing.
2. **One finding that changes the shape of the feature**: FR-002 cannot be delivered for the nine Sage suppliers as specified, because the wholesaler's product codes have never been mapped to those suppliers' SKUs — and separately, 39% of all products carry no SKU to match on at all. The spec assumes otherwise.
3. **SC-002 as written is unachievable**, replaced by per-source freshness targets ranging from 60 seconds (Shopify) to 26 hours (nightly SFTP), with the honest note that a missed Shopify webhook is unrecoverable on those suppliers' plans.
4. **Two architecture rows this feature promotes** — rate limiting (AX-011, tripped by Lightspeed's documented 60/min and by the new public webhook endpoint) and per-tenant secrets (AX-013, which has no ruling and nothing to store credentials in today).
5. **Infrastructure verdict**: one worker machine probably holds it — ~0.13 req/s today, ~1–3 req/s at 500 suppliers — with the caveat that the SFTP fetch is pinned to the allowlisted egress IP `149.248.201.77`, and any change there costs 10 working days with Northern Provisions' IT. £88/month budget headroom; Upstash command volume and Grafana log volume are the two costs to estimate before committing.
6. **Two open inputs** I assumed rather than blocked on: products per supplier (assumed 200) and Shopify events per shop per day (assumed 300). Both are one-line recalculations in the documents once the real figures land.