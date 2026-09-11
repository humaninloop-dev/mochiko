FILES-READ: `pallet/specs/live-stock/spec.md`, `pallet/docs/supplier-systems.md`, `pallet/ops/hosting.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/CLAUDE.md`, `pallet/README.md`

# Action plan — quality targets + infra/integration needs for FEAT-017 (live stock)

The deliverable is written analysis for a sizing conversation, not code. Nothing in this card asks for implementation, so I would write no application code and run no migrations. Two documents, in the style and ID conventions of the existing spec (`FR-001`/`SC-001` style, so `NFR-`, `INT-`, `INF-`), cross-referenced back to the spec's own requirement ids.

No delegation. This is a single-pass reading-and-writing task over seven small files that I have already read; spawning a subagent would add hand-off cost and a summarisation layer over documents I need to quote precisely. I would do all of it myself.

---

## Phase 1 — Close the gaps in what I have read

What I would read: `pallet/prisma/schema.prisma` is explicitly an excerpt ("the models the live-stock feature touches"). Two spec claims depend on models not in it, so I would glob for any other `.prisma` file or migrations directory and grep the repo for `order_edit`, `edit_reason`, `audit_events`, and `Order` to answer:

- **SC-001** is measured "from order edit reasons". If no reason code is captured on a supplier-side order edit today, SC-001 is not measurable and the feature needs a small instrumentation requirement before the clock starts. I expect to find nothing — the workspace is a curated excerpt — in which case I record it as a verify-with-the-team item rather than asserting the field is absent.
- **AX-008** says stock is not a financial record and is not audited. FR-003 requires the override to be "recorded against the person who made it", so I need to know whether `audit_events` is reusable for a non-financial mutation or whether this needs its own table.

Stop point: none. If the greps come back empty I proceed under the stated assumption and flag both as questions with named owners (Priya for SC-001 instrumentation, whoever holds AX-008 for the audit ruling).

## Phase 2 — Turn the four vague quality sentences into measurable targets

The spec's "Quality expectations" section is five sentences, four of which cannot be tested as written. I would rewrite each as a target with a number, a measurement point, and a named source of truth, and put them in `pallet/specs/live-stock/quality-attributes.md`.

The substantive work here is that **freshness cannot be one number** — the three integrations have physically different floors:

| Source | Achievable freshness | Why it is bounded there |
|---|---|---|
| Shopify (21) | seconds; propose p95 ≤ 60 s event-to-visible | push, but retries span 48 h and arrive out of order |
| Lightspeed (38) | poll interval + run time; propose 5 min interval, p95 staleness ≤ 6 min, worst case ≤ 15 min | polling only, 60 req/min/token |
| Sage via SFTP (9) | up to ~26 h | one file, ~02:00, nightly |
| None (72) | manual; no freshness claim | FR-004 |

So **SC-002 "Stock shown is never stale" cannot be met and cannot be tested.** I would not silently reinterpret it. I would write the replacement I recommend and mark it as a proposed amendment to a signed-off spec, owned by Priya: per-source staleness budgets as above, plus a *staleness policy* — when the last successful sync for a supplier is older than three times its expected interval, the catalogue shows "availability unknown" with an "as of" timestamp rather than a stale number, and unknown does not block adding to basket (blocking on unknown would turn a sync outage into lost orders, and FR-001 only requires blocking a confirmed zero).

Other targets I would write, each tied to something that already exists so ops can size against a known baseline:

- **Catalogue latency**: p95 must stay under 400 ms — not invented, that is the existing AX-010 caching trigger. This becomes a design constraint: stock must be readable in the same single-query-per-supplier catalogue read the spine describes, i.e. denormalised onto the product row, not a per-product lookup. If the design needs more than one row per product per request it trips AX-010 and the feature grows a caching decision. I would state that as the reason for the constraint, so it is a real budget rather than a preference.
- **Degradation**: a supplier's sync being down must not fail the catalogue read (availability of the read path is independent of the sync path). Target stated as: zero catalogue 5xx attributable to a sync failure.
- **"We shouldn't hammer suppliers' systems"** → a politeness budget: sustained outbound ≤ 50% of each supplier's documented limit (so ≤ 30 req/min against a Lightspeed token), `Retry-After` honoured on 429, exponential backoff with jitter, and a circuit breaker that stops polling a supplier after N consecutive failures and raises a reconnect prompt instead of retrying forever.
- **Reliability of sync** → expressed as a lost-update target, not an uptime percentage: no stock change may be silently dropped; every source needs a recovery path (Phase 3 shows Shopify's is the hard one).
- **Correctness under out-of-order delivery**: an update whose `updated_at` is older than the stored one is discarded, never applied. This is testable and is the direct answer to Shopify's documented reordering.
- **Idempotency**: every sync unit carries a deterministic job id — this is already mandatory (lint rule `PLT002`), so it is a constraint to record, not a new target.
- **Test targets ops should size for**: coverage floor 75%, integration tests against a real PostgreSQL, and specifically a Lightspeed sandbox contract test, signed-fixture Shopify HMAC tests including a replayed/out-of-order pair, and an SFTP test covering the missing-file and truncated-file cases that the survey says actually happen (4 missing and 2 truncated in 30 nights). Test infrastructure is part of what they are sizing.

**One thing I would push back on in writing.** The spec says "Stock levels are not sensitive, so no special handling is needed." The stock levels themselves I will not argue about, but the sentence as written would license the wrong handling of what sits next to them: per-supplier OAuth access and refresh tokens and an SFTP password are credentials to a counterparty's system, and a supplier's stock position is that counterparty's commercial data. The governance rule on counterparty confidentiality and the logger redaction list apply to the credentials regardless of how the stock numbers are classified. I would narrow the spec sentence rather than delete it: stock levels are not personal data and need no special retention or export handling; the credentials that fetch them are Restricted, never logged, never in an error body, and the contract suite assertion already in place must cover the new webhook endpoint's error responses too.

## Phase 3 — Integration needs, per source, including the two that do not work

Written into `pallet/specs/live-stock/platform-needs.md`. For each of the four rows of the survey: what we must build, what it depends on, and what is unresolved.

**Lightspeed (38 suppliers) — buildable.** Poller as a BullMQ repeatable job (AX-006 already rules this: repeatable jobs, no machine cron). Per-supplier OAuth with a 24 h access token and a refresh token that **rotates on every use** — this is the sharpest operational risk in the feature: a lost rotation means the supplier reconnects by hand, so the refresh must be written durably and transactionally before the new token is used, and a crash between "refresh returned" and "token stored" must not lose it. I would call out concurrency: two workers refreshing the same supplier's token concurrently will burn one of them, so refresh needs a per-supplier lock. Pagination at 250 rows means a supplier's poll cost scales with catalogue size; the request budget is a function of product count, which I compute in Phase 4.

**Shopify (21) — buildable, with one gap ops must decide about.** Webhooks need a **public inbound endpoint**, and hosting notes say only `api` has a public address. So webhook receipt lands on `api`, verifies HMAC-SHA256 against the per-shop secret, fast-acks, and enqueues to the worker — it does not do the write inline. That makes this the first inbound integration and it needs the endpoint to be cheap and always-up. The gap: the survey says most of these suppliers' plans have **no history and no bulk endpoint, so a missed event is gone**. A 19-retry/48-hour window covers transient outages, but not a webhook we accepted and then dropped, or a registration that lapsed. So there must be a reconciliation path, and it cannot be a bulk read. I would write it as a requirement with a costed choice for ops: a low-frequency full re-read of inventory per shop (nightly), priced in requests, versus accepting documented drift. I would recommend nightly reconcile and state the API-call cost in Phase 4 rather than choosing silently.

**Sage 50 / Northern Provisions (9) — I would flag this as blocked, and say so plainly rather than sizing it.** Two independent blockers:
1. **No product-code mapping exists.** The export uses Northern Provisions' internal codes, which do not match the SKUs those nine suppliers print, and nobody has mapped them. The spec's assumption "Pallet product ids can be matched to the supplier's own product codes" is false for this source. No amount of engineering produces correct stock from an unmapped file; a wrong mapping produces confidently wrong "out of stock" flags, which is worse than the status quo the spec is trying to fix. This needs a person to build and own a mapping table — a data task, not a build task — and I would size it as such and put it on the critical path for these 9.
2. **The feed is unreliable by measurement**: missing on 4 of the last 30 nights (13%) and truncated twice. So the requirements must include: never treat a short file as authoritative (row-count and per-supplier-completeness sanity check against the previous night, reject outside tolerance), never blank stock from an absent file (last-known + "as of" + unknown after the staleness threshold), and an alert that fires into working hours, not at 02:00, because on-call is "whoever is awake".

Also: the SFTP fetch **must run from the worker's egress IP `149.248.201.77`**, which is what their firewall allowlists, and changing it needs **10 working days' notice** to their IT. This constrains infrastructure directly (Phase 4).

**No system (72) — buildable and cheapest.** FR-004 manual entry and "always available" need no integration at all, just schema and UI-facing API. Worth stating in the sizing document that this covers 51% of suppliers and is the shortest path to FR-001 being visible in the catalogue.

**Cross-cutting matching gap**: `supplier_sku` is present on only 61% of products. Even for Lightspeed and Shopify, roughly two products in five have nothing to match on today. That is a backfill/onboarding requirement, and it is the difference between "connected" and "connected and actually showing stock". I would make it an explicit line item because it will otherwise be discovered mid-build.

## Phase 4 — Infrastructure needs and the arithmetic

Same document. Concrete, with the numbers shown so ops can check them.

**Capacity.** I would compute the outbound poll budget from stated facts rather than assert a machine count: pages per poll = ceil(products per supplier ÷ 250); requests per supplier per hour at a 5-minute interval = 12 × pages. I would present it as a small table across plausible catalogue sizes and against both today's 38 Lightspeed suppliers and the spec's 500-connected figure, then state the sustained requests/second and concurrent-job count that falls out, and what evidence would be needed (a measured poll duration against the Lightspeed sandbox) before claiming one `shared-cpu-2x` worker carries it. The honest output of this phase is a threshold — "one worker holds until X connected pollers or Y sustained req/s" — not a confident yes.

**The worker-machine trap, which is the single most important infra finding.** Ops would rather not add a second worker. But if a second worker machine is added, it gets its own egress address, and Northern Provisions' firewall allowlists exactly one — so scaling the worker horizontally either breaks the Sage feed or costs 10 working days' notice. The requirement is therefore not "£14/month for a machine": it is that the SFTP job must be pinned to the machine holding `149.248.201.77`, or that any scale-out is planned two weeks ahead with their IT. I would write that as a named infrastructure constraint, since it is invisible from the code and will be discovered at the worst moment.

**Secrets — a hard blocker with no current answer.** Hosting notes and AX-013 agree: Fly secrets are per-app for platform credentials, and *there is no place today for per-supplier credentials*; AX-013 explicitly records that per-tenant credentials have no ruling and nothing holds any today. This feature introduces ~59 rising to ~500 sets of them, including rotating refresh tokens that change on every use — which rules out Fly secrets on mechanics alone, not just ergonomics. My recommended default, to be confirmed: encrypted-at-rest columns in PostgreSQL with the encryption key in Fly secrets, per-supplier, excluded from exports and the logger. This keeps the one-datastore principle intact and costs nothing. The alternative — a managed secrets service — is a new datastore requiring platform sign-off recorded in the spine, and consumes budget. I would present both, recommend the first, and mark it as needing a ruling before build starts.

**Rate limiting — an existing trigger that this feature fires.** AX-011 is "not-now" with the trigger "the first integration, inbound or outbound, with a documented rate limit". Lightspeed's 60 req/min per token is exactly that, so AX-011 must be ruled before this ships. I would draft the ruling for the spine rather than leaving it as a discovery.

**Caching.** AX-010's trigger is a page reading more than one row per product per request, or catalogue p95 above 400 ms. If stock lives on the product row, the trigger does not fire and no caching work is needed. I would state this as the design constraint it implies (see Phase 2) and note that a separate stock table joined per product would fire it and add scope.

**Cost.** £212 today against a £300 cap agreed with Tom, so £88 headroom. I would itemise what this feature actually adds: possibly a second worker (£14), Redis job volume on Upstash from repeatable polls at 500 suppliers, Postgres write volume, and Grafana Cloud log ingest — with an explicit requirement not to log a line per poll, because at 500 suppliers × 12 polls/hour that is the log bill, not the compute bill. Conclusion stated as headroom remaining, plus the one line item most likely to breach it.

**Observability.** Existing alerts are 5xx rate above 1% over 5 min and queue lag above 10 min. Queue lag will catch a stalled poller. It will not catch: a supplier whose token silently died, a Shopify shop whose webhook registration lapsed, or a missing SFTP file — all of which look like a healthy queue with nothing in it. So the feature needs freshness-based alerting per supplier and per source, and I would specify severity by hour so the 02:00 SFTP failure does not page a team whose on-call is whoever is awake.

**Rows I would propose for `architecture/spine.md`** (as proposals, not ratified — I would not edit governance unilaterally): a ruling for AX-011, a ruling for AX-013 covering per-tenant credentials, a note on AX-008 for whether stock overrides are audited, and a new row for public inbound webhook ingress, which is a container-shape change the spine does not currently describe.

## Phase 5 — Consistency check against the spec's own success criteria

Before writing, one arithmetic check that changes the sizing conversation:

- **SC-003 wants 50% of active suppliers connected within six months.** The survey accounts for all 140: 38 + 21 + 9 + 72. Every supplier with a connectable system is 68 of 140 = **48.6%** — under target even at 100% conversion, and with the Sage nine blocked on the unmapped codes it is 59 of 140 = **42.1%**. SC-003 is unreachable as written unless "connected" is defined to include suppliers entering stock by hand under FR-004. I would recommend that definition explicitly, since it makes the target both achievable and more honest about what delivers FR-001 for the 72.
- **"500 connected suppliers within a year"** sits against 140 suppliers today. Either it assumes very large growth (Dutch producers from Q4 2026) or it is a different unit. I would size the infrastructure for 500 regardless — that is what ops asked for — and flag the inconsistency for Priya rather than resolving it myself.

## Phase 6 — Write, then check

Files I would write:

- `pallet/specs/live-stock/quality-attributes.md` — the measurable targets, per-source freshness table, staleness policy, degradation behaviour, politeness budget, correctness-under-reordering, test targets; each target tagged `NFR-0xx` and referencing the `FR-`/`SC-` it serves, with the measurement instrument named.
- `pallet/specs/live-stock/platform-needs.md` — integration needs per source (`INT-0xx`), infrastructure needs (`INF-0xx`) with the capacity arithmetic and cost table, the proposed spine rulings, and a blockers-and-open-questions section with a named owner and a stated default for each.

I would create no new directories, touch no existing file, and specifically **not edit the signed-off `spec.md`** — the proposed amendments to SC-002, SC-003, and the "not sensitive" sentence go in a section addressed to Priya, for her to accept or reject.

Checks before I call it done: every target has a number and a way to measure it; every number traces to a line in one of the source documents rather than to my judgement, or is labelled as my recommendation; the poll and cost arithmetic re-derived once; and no proposed requirement contradicts an existing governance rule or spine row without saying so out loud.

## Stops and branches

Four places where a person decides. My defaults let me finish the document either way; none of them blocks writing it.

1. **SC-002 "never stale."** Confirming: does Priya accept per-source staleness budgets and an "availability unknown" state? If yes, the targets stand as drafted. If she holds "never stale", then the Sage source and arguably Lightspeed cannot satisfy it, and the honest branch is to descope those suppliers from FR-002 and cover them by FR-004 manual entry — I would write that variant's consequences alongside. Default: per-source budgets.
2. **Per-supplier credential storage.** Confirming: encrypted columns in PostgreSQL, versus a new secrets service. Encrypted columns → no budget impact, no spine sign-off beyond the AX-013 ruling. New service → new-datastore sign-off recorded in the spine, plus cost against £88 headroom. Default: encrypted columns in PostgreSQL.
3. **Sage / Northern Provisions.** Confirming: who builds and owns the product-code mapping, and does the feature ship for those 9 in phase one? Mapping owner found → these 9 stay in scope on a data-dependent timeline. Not found → I size them out of the first release and say so in one line rather than burying it. Default: out of the first release, flagged.
4. **Shopify reconciliation.** Confirming: nightly full re-read per shop versus accepting drift. Reconcile → a request-cost line I would price. Accept drift → a documented correctness limit that contradicts the freshness targets for those 21 suppliers, which I would record on the face of the document. Default: nightly reconcile.

## What I would report at the end

A short summary for the sizing meeting: the two document paths; the freshness targets in one table, because that is the number ops will size against; the four things that cost money or time and are not obvious from the spec — no home for per-supplier credentials, the egress-IP constraint on worker scale-out, the rate-limiting ruling this feature forces, and the unmapped Sage product codes; the two spec criteria that cannot be met as written (SC-002, and SC-003 by arithmetic) with my proposed replacements; and the shortest credible slice, which is FR-001 plus FR-004 for the 72 unconnected suppliers with no integration at all. I would say plainly which of my numbers are measured from the survey and hosting notes and which are recommendations awaiting the four decisions above.