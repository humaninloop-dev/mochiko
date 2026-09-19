FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/docs/supplier-systems.md`, `pallet/ops/hosting.md`, `pallet/specs/live-stock/spec.md`

---

# Action plan — quality targets, infrastructure and integration needs for FEAT-017 (live stock)

## Scope I would hold to

Ops wants three things before sizing: measurable quality targets, what has to be provisioned, and what the supplier integrations demand. That maps to three artifacts and no more:

- `pallet/specs/live-stock/technical-requirements.md` — TR rows traced to FR-001..FR-005, plus the integration-boundary declarations for Lightspeed, Shopify and the Northern Provisions SFTP feed, plus data-sensitivity declarations for the credentials the feature introduces.
- `pallet/specs/live-stock/constraints-and-decisions.md` — the hard boundaries taken from the hosting notes and the supplier survey, the decisions those boundaries force, and the provisioning items ops actually needs to price.
- Proposed quality-target rows against the concern rows in `pallet/architecture/spine.md` — the targets live on the concern rows, not in a separate file.

I would **not** write the data model or the API contract on this card. The stock entity, the override attribution table and the catalogue/basket endpoints are real work, but ops is sizing infrastructure, and a half-considered entity model written under a sizing deadline is worse than none. I would say so in my hand-back rather than silently omitting them. I would also **not** write a quickstart/integration guide: the external surface here is inbound (three supplier systems pushing or exposing stock to us), not a surface a client integrates against. That changes once the supplier-app OAuth connect flow gets an API contract; I'd record the absence and the trigger.

## Phase 0 — Workspace sweep (done)

Already complete: seven files, all read above. The whole workspace is in context, so there is no locate or enumeration worth delegating. The two bounded checks I would otherwise farm out to a throwaway `Explore` subagent on haiku — "is there any existing stock/inventory code, migration or spec artifact anywhere under `pallet/`?" and "does anything besides the SFTP work reference the worker egress IP?" — are already answered by the file listing and by the hosting note's own sentence that nothing else depends on that address. Spawning for them would cost more than it returns. If the real repo turned out to be larger than this excerpt, the first spawn would be: *haiku, one gap — list every file mentioning `stock`, `inventory`, `supplier_sku` or `InventorySource`, path and line only*; on return I'd check it names the schema and spec files I already have, and treat any third file as something I must read myself.

## Phase 1 — Load the authoring procedures

Load `mochiko:authoring-technical-requirements` (TR/constraint/decision/infrastructure identifier grammar, and where quality-target rows are homed), and `mochiko:patterns-technical-decisions` (the alternatives-and-criteria format) before writing anything. I would not load the entity-modeling or API-contract skills on this card — those artifacts are out of scope here.

## Phase 2 — Translate the five FRs into technical requirements

Read again, closely: the spec's FR block and quality paragraph, and the schema excerpt. Decompose rather than transcribe. Expected shape:

- FR-001 (show availability, block out-of-stock in basket) becomes separate requirements for the availability computation, the four-state derivation from a raw count plus source, the add-to-basket guard, and the catalogue read path. **Gap I would flag, not guess:** "low" has no threshold and no owner. Who sets it — per product, per supplier, or a platform default? The spec never says. Default I'd proceed under while asking: a per-supplier integer threshold with a platform default, because a supplier who holds 12 cases of one line and 400 of another cannot use one global number.
- FR-002 (sync from connected systems) becomes three distinct requirement sets, because the three sources share almost nothing: a polled pull, a signed inbound push, and a nightly file fetch. Treating them as one "sync" requirement is the mistake that would make the sizing wrong.
- FR-003 (override, recorded against who made it) becomes an attribution requirement plus a precedence requirement — an override must survive the next sync, or it is not an override. The spec does not say whether an override expires. Default: it persists until cleared, and the next sync does not overwrite it; flagged for Priya.
- FR-004 (manual counts / always-available) becomes the no-source path, and the interaction with FR-001's blocking rule: "always available" never blocks a basket.
- FR-005 (told in basket and at checkout) becomes a read-time re-check requirement with its own latency target.

Implicit needs the spec never mentions, which I would surface as requirements in their own right: job idempotency for every sync path (the operating manual makes deterministic job ids mandatory and the base class refuses a job without one); out-of-order write resolution; outbound request throttling; per-supplier sync health signals; and retention for whatever stock history we keep, since a nightly retention purge already exists and would otherwise have no rule for these rows.

## Phase 3 — Integration boundaries, each with its failure modes

One boundary declaration per system, written from the survey, each stating auth, transport, volume, rate limit, ordering guarantees, failure modes and the fallback. The substance I already have:

- **Lightspeed (38 suppliers, polling).** 24-hour access token; refresh token **rotates on every use**, and a lost rotation means the supplier reconnects by hand. That is the sharpest failure mode in this feature: a crash between "used the refresh token" and "persisted the new one" permanently breaks that supplier's connection. It forces a requirement that token persistence be transactional and durable before the new token is used for anything else, and it pushes hard on the credential-store decision in Phase 5. Also: 250-row pages, 60 requests/minute per token, `429` with `Retry-After`, sandbox available for the contract tests.
- **Shopify (21 suppliers, webhooks).** HMAC-SHA256 per shop secret; 19 retries over 48 hours; **out-of-order delivery** with `updated_at` in the payload; and on these suppliers' plans **no history and no bulk endpoint — a missed event is gone forever**. There is no reconciliation path available. I would write that plainly as a boundary with no fallback, rather than inventing a nightly full-resync that the plan does not support.
- **Northern Provisions SFTP (9 suppliers).** Nightly CSV around 02:00; **missing 4 of the last 30 nights, truncated twice** — that is roughly 13% no-file and 7% bad-file, and I'd carry those observed rates into the targets rather than assuming the feed is reliable. Egress-IP allowlist with a **10-working-day** change lead time, confirmed in writing 2026-08-19. That lead time is a scheduling constraint on any infrastructure change that moves the fetch off the current worker.
- **No system (72 suppliers).** Not an integration; the manual/always-available path.

**Stop S2 — blocking, for Priya and ops.** The spec assumes "Pallet product ids can be matched to the supplier's own product codes." The survey says that is false for the nine Sage suppliers: the export uses Northern Provisions' internal codes, which do not match the SKUs those suppliers print, and **nobody has mapped them**. Separately, `supplier_sku` is populated on only 61% of products, so matching is partial even for Lightspeed and Shopify. FR-002 cannot be satisfied for the SFTP cohort as things stand. Branches: (a) drop `sftp_csv` from the first release — then I write the CSV boundary as deferred and remove its provisioning from the sizing, and the egress-IP constraint stops being load-bearing; (b) the wholesaler's IT supplies a code map — then I add a requirement for ingesting and maintaining that map, and the sizing gains a mapping store; (c) Pallet builds a supplier-facing code-matching screen — that is new scope the spec does not contain and I would send it back to Priya rather than absorb it. **Default while waiting: (a).** I'd note that `InventorySource.sftp_csv` already exists in the schema, so somebody has assumed this cohort is in — that assumption needs testing, not inheriting. For the 39% of products with no `supplier_sku`, the default is that they stay on the manual/always-available path and are reported as unmatched.

## Phase 4 — Quality targets

This is the part ops actually asked for, and it is where the spec is weakest. The quality paragraph says "near real-time", "reliable", "shouldn't hammer suppliers' systems", "not sensitive", "500 connected suppliers within a year". Four of those five are unmeasurable as written, and the fifth is the only number in the document.

**Stop S1 — blocking, for Priya.** SC-002 says "Stock shown is never stale." That is not achievable and not measurable. A nightly CSV is by construction up to ~26 hours stale, and is absent about one night in seven. A single freshness target across three sources is the wrong shape. I would refuse to write SC-002 through as a target and instead propose **per-source freshness targets**, each with its own measurement:

- Shopify: 95th percentile from supplier-side change to catalogue visibility under ~30 s, measured from the payload's `updated_at` to our row's write time — the only source where "near real-time" is honest.
- Lightspeed: bounded by the poll interval; target expressed as interval plus processing, measured the same way. The interval is chosen in Phase 5 against the rate limit.
- SFTP: freshness measured in hours since the last successful import, with an explicit target that the previous night's file is applied before the trading day starts, plus a staleness ceiling after which the data is no longer shown as authoritative.

Branch on Priya's ruling: if she insists on one platform-wide freshness figure, the only consistent outcome is that SFTP suppliers cannot be shown as live-stock suppliers at all, which collapses into Stop S2 branch (a).

**Stop S3 — for Priya.** What does the catalogue show when a feed is stale or absent — the last known figure, "always available", or a visible "stock unknown"? This is a user-facing correctness decision, and with a 13% missed-night rate it will fire roughly weekly. I will not guess it. Default while waiting: hold the last known figure up to the staleness ceiling, then fall back to unknown-but-orderable, since blocking orders on our own feed failure is worse than the problem the feature exists to solve.

Other targets I would write, each with a number, a measurement method and a stated source:

- **Catalogue read latency** — a 95th-percentile page target deliberately set under the 400 ms figure that the spine's caching row names as its trigger, so that the target is what keeps the trigger from firing rather than something discovered later. Measured through the existing OpenTelemetry traces.
- **Basket/checkout re-check latency** for FR-005 — the check sits in the critical path of placing an order.
- **Outbound request ceiling per supplier token** — a sustained rate set materially below Lightspeed's 60/minute, with `Retry-After` honoured and a `429` rate target near zero. This is the measurable form of "shouldn't hammer suppliers' systems".
- **Sync success rate** per source, with the SFTP target acknowledging the observed 13%/7% failure rates as the baseline being improved on, not assumed away.
- **Token-refresh durability** — a target of zero refresh-rotation losses, because each one costs a supplier a manual reconnection.
- **Capacity at 500 connected suppliers**, sized explicitly.

**Open figure I would have to ask for, and the assumption I'd size under:** nothing in the workspace gives the average number of products per supplier, and the whole polling and write-volume calculation turns on it. I'd ask ops for the real distribution from the `products` table. Meanwhile I'd size the arithmetic openly so the number can be swapped: at roughly 2,000 products per supplier, a full Lightspeed inventory read is ~8 pages; 500 suppliers polled every 5 minutes is ~4,000 outbound requests per 5 minutes, which is a sustained low-teens requests per second from the worker — comfortable for I/O-bound work on one machine. The pressure is not the HTTP; it is the database, where a naive implementation compares and writes on the order of a million product rows every 5 minutes. So the sizing conclusion is a **requirement that syncs write only changed rows**, and the target is expressed as sustained stock-row writes per second, not requests per second. I would present this arithmetic in the artifact, because it is the thing that tells ops whether the answer is "one worker" or "two".

**Stop S4 — I would refuse the spec's sensitivity claim.** The spec says "Stock levels are not sensitive, so no special handling is needed." The stock numbers themselves, taken alone, I'd accept as low sensitivity. But this feature introduces per-supplier **OAuth access and refresh tokens, Shopify per-shop HMAC secrets, and an SFTP password** — those are Restricted, they are exactly the counterparty credentials the operating manual's confidentiality principle covers, and they must never reach logs, exports or error bodies. I would also flag that one supplier's stock must not be readable by another supplier, since a stock level is a commercial signal about a trading relationship. I'd write the classification correctly and note in the artifact that it contradicts the signed-off spec sentence, with the reason.

## Phase 5 — Constraints, the decisions they force, and provisioning

Constraints taken only from real sources, each cited: the £300/month budget agreed 2026-04-14 with £212 spent (so ~£88/month headroom, and a second worker machine is ~£14/month); only `api` has public ingress; the worker holds the allowlisted egress address `149.248.201.77` and nothing else depends on it; per-supplier credentials have nowhere to live today; PostgreSQL is the sole datastore and a new one needs recorded platform sign-off; no cron on machines — periodic work is BullMQ repeatable jobs; background jobs must be idempotent with deterministic ids; errors are RFC 7807; coverage floor 75% with integration tests against real PostgreSQL; data stays UK/EU.

Decisions I would work up properly, each against at least two real alternatives with the trade-offs written out:

1. **Where per-supplier credentials live.** Fly secrets per app (the current pattern) does not extend to 500 suppliers' rotating tokens and cannot be written transactionally, which the Lightspeed rotation failure mode demands. Candidates: encrypted column in PostgreSQL with the key-encryption key in Fly secrets, versus an external secrets manager. The one-datastore principle and the transactional-rotation requirement both point at PostgreSQL; an external manager would need the recorded platform sign-off the manual requires. This decision also fills the acknowledged hole in the spine's secrets row.
2. **Where the Shopify webhook endpoint lives.** It must be publicly reachable, and only `api` is — so the receiver sits in `api`, verifies the HMAC, and enqueues. Alternative (exposing the worker) contradicts the ingress constraint and would put a second public surface on the app holding the allowlisted egress address.
3. **Where the SFTP fetch lives.** It must stay on the worker, because the worker's egress address is the one allowlisted, and moving it costs a 10-working-day notice to the wholesaler's IT. This is a constraint that pins a placement, and I'd say so plainly.
4. **Poll scheduling and throttling for Lightspeed** — repeatable jobs per supplier versus a single sweeping job, with the per-token rate ceiling and backoff behaviour.
5. **Out-of-order resolution** — last-write-wins on the source's own `updated_at` rather than our receive time, since Shopify explicitly delivers out of order and carries that field.

Provisioning items for ops to price, each traced to the constraint or target that demands it: worker capacity (with the arithmetic above and the explicit question of whether a second machine at ~£14/month is needed — well inside headroom, but ops has said they'd rather not); the credential store and its key management; the public webhook route and its abuse protection; Redis queue capacity for the new job classes; and monitoring — which today only alerts on 5xx rate and queue lag over 10 minutes. New alerts this feature needs: per-supplier sync staleness breach, CSV absent by a morning deadline, HMAC verification failures, token-refresh failures, and outbound `429` rate.

## Phase 6 — The architecture rows this feature disturbs

Three concern rows in `pallet/architecture/spine.md` are affected, and one of them is triggered outright:

- **Rate limiting** is marked "not-now" with the trigger "the first integration, inbound or outbound, with a documented rate limit." Lightspeed's 60 requests per minute per token **is that trigger**. The row has to move to decided. Both directions matter: outbound throttling to suppliers, and inbound protection on the new public webhook route.
- **Caching** is "not-now" with the trigger "a page that reads more than one row per product per request, or catalogue p95 above 400 ms." Whether this fires depends on how stock is stored — which is the data-model work I am deliberately not doing on this card. I would state the dependency and set the latency target under the 400 ms threshold, rather than pre-empting the modelling decision.
- **Secrets** records that per-tenant credentials have no ruling and nothing holds any today. This feature makes that hole load-bearing.
- **Audit trail** says stock is not a financial record and is not audited. FR-003 requires overrides be attributed to a person. Either the audit mechanism extends to stock overrides or attribution is carried on the override itself — an explicit choice, not a silent one.

**Stop S5 — for whoever rules on the spine.** These are ratified architecture rulings with dates; I would draft the proposed rows and their wording and bring them for a ruling rather than editing rulings unilaterally. If the ruling comes back different from my draft, the dependent items in the constraints-and-decisions artifact change with it and I would rewrite them before hand-back.

**Stop S6 — for ops.** The worker sizing. Branch: if the per-supplier product counts come back well above my assumed figure, a second worker machine goes into the plan at ~£14/month against ~£88/month headroom; if they come back lower, one machine holds and I say so with the numbers behind it.

**Stop S7 — for Priya and ops jointly.** Shopify missed events are unrecoverable on these suppliers' plans. Either we accept permanent drift and add a detection signal, or those suppliers need a plan upgrade, which is a commercial conversation and not mine. Default: accept and detect, with the drift risk written into the boundary.

## Phase 7 — Consistency pass before hand-back

Check every technical requirement traces to one of FR-001..FR-005 and that no FR is left without one; check every quality target has a number, a measurement method and a named source; check every one of the three boundaries states what happens when it fails; check every provisioning item traces back to a constraint or a target; check the credential classification is consistent across the artifacts; and check that nothing I wrote contradicts a ratified principle without saying so out loud.

## Tests

No code is written on this card, so there is nothing to run. What I would write down for the build that follows: contract tests against the Lightspeed sandbox the survey confirms exists (including a forced `429` to prove `Retry-After` is honoured, and a simulated crash between refresh-token use and persistence to prove the rotation is not lost); HMAC verification tests including a bad signature and a replayed, out-of-order delivery that must not regress a newer value; and CSV import tests covering the absent file and the truncated file, since both have actually happened. Integration tests against real PostgreSQL, per the manual; the 75% coverage floor is blocking in CI.

## What I would report at the end

The three artifacts and their paths; the proposed spine row changes awaiting a ruling; the sizing arithmetic with the product-count figure I had to assume and the request to replace it; the seven stops above with my stated defaults and which ones block sizing (S2 and S6 do; S1, S3, S4, S5, S7 do not block sizing but block the build); the two places where I contradicted the signed-off spec and why — "never stale" is unachievable, and "not sensitive, no special handling" is wrong about the credentials this feature introduces; the undefined "low stock" threshold; and the two artifacts I deliberately did not write, the data model and the API contract, with a note that the caching question cannot be settled until the first of those exists.