FILES-READ: pallet/specs/live-stock/spec.md, pallet/docs/supplier-systems.md, pallet/ops/hosting.md, pallet/architecture/spine.md, pallet/CLAUDE.md, pallet/README.md, pallet/prisma/schema.prisma

# Action plan — FEAT-017 live stock: quality targets, infrastructure, integration needs

## Scope I would hold to

Ops asked for three things before sizing: quality targets, infrastructure needs, integration needs. So this pass produces the **analysis layer only**:

- `pallet/specs/live-stock/constraints-and-decisions.md` — the constraints, the decisions this feature forces, the infrastructure provisioning items, the three integration declarations, and thin data-sensitivity declarations.
- Proposed quality-target rows appended to the concern rows in `pallet/architecture/spine.md` (that file is where this project homes them — I would not create a parallel `nfrs.md`).

I would **not** write the data model, the API contract, or the integration guide in this pass, and would say so explicitly in the handoff: sizing does not need them, and writing the stock entity now would pre-empt two rulings (caching, audit) that aren't mine to make. The credential-storage shape is the one exception — it drives an infrastructure line item, so it gets a decision here.

## Phase 1 — Build the integration fact table (reads only, already done)

I have read all seven files in the workspace; a glob of `**/*` confirms there is nothing else — no service code is mounted, only these notes and a schema excerpt.

I would first reduce the supplier survey to a comparison table I can reason about per source: transport, auth, freshness ceiling achievable, loss behaviour, ordering behaviour, rate limit, recovery path, and what it needs from our side (inbound public URL / outbound static IP / stored credential). This is the spine of everything downstream.

Delegation: none. The whole corpus is seven short files that I must read interpretively — absence of a detail here (for example, that nothing holds per-supplier credentials today) is itself load-bearing, so I read it myself. If the actual NestJS repo were mounted I would spawn one throwaway `Explore` on `model: haiku` to enumerate any existing HMAC-verification, outbound-retry, or column-encryption helper and quote the file/line, then check its returns for real paths rather than plausible ones. It is not mounted, so I would skip it rather than pretend.

## Phase 2 — Contradiction pass, and the stops

Before writing anything I would list the places where the spec, the ops notes, and the governance disagree. Each is a stop; for each I state what I would ask, who rules, and what I do under each ruling. I proceed on the stated default and mark the artifact provisional rather than waiting.

1. **"Stock shown is never stale" (SC-002) is not achievable and not measurable.** Shopify events can be lost with no replay; the Sage file was absent on 4 of the last 30 nights; Lightspeed is poll-only. I would refuse to carry this through as a quality target. *Ask Priya:* accept per-source freshness ceilings instead. *Default:* write the ceilings, mark SC-002 as needing a spec amendment. *If she insists on "never stale":* the only honest branch is that FR-002 is undeliverable for Shopify and SFTP suppliers, and I record that instead.
2. **What the catalogue shows when sync is broken.** The spec never says. Blocking the basket on a stale feed turns a supplier's failed 02:00 file into lost orders all day. *Ask Priya:* fail open (last known value, then "always available" past the staleness ceiling, with a banner and an ops alert) or fail closed. *Default:* fail open, because the platform is fail-open today and a truncated CSV must never silently delist a supplier's catalogue. *If fail closed:* the freshness targets tighten into availability targets and the sizing goes up — I'd flag that to ops as a cost branch.
3. **The Northern Provisions codes are not mapped to anything, and nobody owns mapping them.** This contradicts the spec's assumption that Pallet ids can be matched to supplier codes. FR-002 is not deliverable for those nine suppliers. *Ask Priya/ops:* who owns the mapping, or do those nine ship on the manual path (FR-004) first. *Default:* treat SFTP as a later phase, keep the constraint and the egress-IP dependency recorded anyway so ops does not lose the 10-working-day lead time. Related, smaller: `supplier_sku` is populated on only 61% of products, so even Lightspeed/Shopify matching has a coverage gap worth a measurable target.
4. **"Stock levels are not sensitive, so no special handling."** Partly true for the numbers, false for what the feature drags in: per-supplier OAuth tokens and an SFTP password are the most sensitive credentials on the platform, and per-supplier stock is commercial counterparty data, which the operating manual already classifies as confidential and bars from logs and error bodies. I would not transcribe that sentence. *Default:* classify explicitly and flag the spec line.
5. **Four architecture concern rows move.** Lightspeed's documented 60-requests-per-minute limit fires the rate-limiting trigger verbatim. Putting stock on the catalogue very likely fires the caching trigger (a second row per product per request). Per-supplier credentials are the open note on the secrets row. Attributing an override to a person needs an audit mechanism that today only covers financial records. *These rulings belong to the architecture owner, not me.* *Default:* I write proposed rulings marked as proposals and do not change any row's status to decided.
6. **Sizing input: 500 connected suppliers within a year** against 140 suppliers today, of which 68 have a connectable system. *Ask ops/Priya* whether 500 is the target to size against. *Default:* size for 500 and show the per-100-supplier increment so the number is re-usable.

## Phase 3 — Constraints (C-XXX)

Written into `constraints-and-decisions.md`, each traced to the note that evidences it, not to an assumption. The ones I already know I have: the worker's single static egress address is the only thing Northern Provisions allowlists and changing it costs 10 working days; only the `api` app has public ingress, so Shopify webhooks must land there while all outbound sync belongs on the worker; £300/month infra ceiling with £212 spent; one worker machine and ops' preference not to add a second; no store exists for per-supplier credentials; PostgreSQL is the only datastore without a platform sign-off; scheduled work is BullMQ repeatable jobs, never machine cron; every background job carries a deterministic id and is re-runnable; errors are Problem Details; data stays in UK/EU; Lightspeed's rate limit and rotating refresh token; Shopify's out-of-order, lossy, 19-retry delivery; the nightly file's 13% absence rate and history of truncation.

## Phase 4 — Integration declarations (INT-XXX)

One per source, each carrying the failure modes and the fallback, because that is what ops is actually sizing:

- **Lightspeed** — poll; failure modes: 429 with `Retry-After`, expired access token, and the sharp one, a lost refresh-token rotation which costs a manual supplier reconnect. That forces serialising sync per supplier so two overlapping jobs can never both spend the same refresh token, plus an atomic token write. Fallback: back off, keep last known value, alert after N consecutive failures.
- **Shopify** — inbound webhook on `api`; verify HMAC before doing anything else, acknowledge fast, enqueue with a deterministic id built from shop + item + `updated_at` (satisfies the idempotency rule and de-duplicates the 19 retries), and resolve out-of-order arrivals by discarding any event whose `updated_at` is older than what we hold. The honest gap: a missed event is unrecoverable on these suppliers' plans. I would **not** invent a reconciliation endpoint; I would record the gap, propose a periodic re-read as the fallback, and flag "confirm whether any per-item read is available on that plan" as an open question owned by whoever runs the supplier survey.
- **SFTP/CSV** — nightly fetch from the worker's pinned egress IP. Failure modes: file absent (expect it ~13% of nights), truncated, stale. Mandatory guard: absence of a product row never means zero stock, and a file whose row count falls materially below the previous good file is rejected rather than applied. Fallback: hold last known, alert, escalate after two consecutive misses.

## Phase 5 — Quality targets (proposed rows on the spine concern rows)

Every one gets a number, how it is measured, and why that number. Planned set: per-source freshness ceilings (tightest for Shopify webhooks, an interval-based ceiling for Lightspeed, next-morning for the nightly file) and the staleness ceiling past which the catalogue degrades; catalogue read latency with stock included, pinned below the 400 ms figure that already sits in the caching trigger; sync success rate per source per day, measured off job outcomes; webhook acknowledgement latency and duplicate-suppression correctness; SKU match coverage; steady-state database write rate, with the explicit requirement that a sync writes only changed rows — re-upserting every product every cycle at 500 suppliers is millions of writes a day against an HA pair for no information gain; and alerting thresholds that reuse the existing 5xx and queue-lag alerts rather than inventing a new alerting story.

I would derive the poll-volume and worker-load arithmetic here and show the working, including the per-supplier product count I had to assume and where that assumption bites. Missing input I would flag rather than guess: the actual distribution of products per supplier.

## Phase 6 — Decisions (D-XXX, ADR form, real alternatives)

Only the decisions this feature genuinely forces, each weighed against at least one serious alternative and referencing the constraints above:

1. **Where per-supplier credentials live** — encrypted column in Postgres with the key in Fly secrets, versus Fly secrets directly (unworkable: per-supplier, dynamic, hundreds of them, each change a deploy), versus an external secret manager (cost, and it needs the platform sign-off the one-datastore rule requires). Expected recommendation: the Postgres option, which keeps the one-datastore rule intact; the sign-off question is why this is a decision and not an assumption.
2. **Where sync work runs** — a dedicated queue on the existing worker versus a second worker machine versus running polls on `api`. The last is ruled out by the egress-IP constraint. The choice between the first two is the sizing question ops is asking, so it carries an explicit trip-point.
3. **How an override is attributed** — extend the audit table beyond financial records versus attribution columns on the stock record. Proposed, deferred to the architecture owner.
4. **Rate-limit handling** — per-supplier token buckets with `Retry-After` honoured, versus global throttling. Proposed against the now-fired rate-limiting trigger.

## Phase 7 — Infrastructure provisioning (IP-XXX) and the sizing numbers

This is the section ops reads. Each item traced to the constraint or target that demands it: a public webhook route with its inbound protection; a stock sync queue with its schedule and concurrency ceiling; whatever compute the arithmetic actually justifies, stated as "one machine holds until X, and here is the measurement that says when to add the £14/month second one"; credential storage and an encryption key in Fly secrets plus a rotation story; per-source sync dashboards and alerts hung off the existing Grafana Cloud setup; a Lightspeed sandbox credential for CI; and an egress-IP preservation item.

The single highest-value question I would put to ops here, because getting it wrong costs ten working days: **does a second worker machine share the app's dedicated egress address, or would scaling out break the Northern Provisions allowlist?** Default until answered: plan as if it must be verified before any worker scale-out, and keep SFTP work pinned to a known-IP process.

I would also include the cost line against the £88/month headroom so finance sees it in one place.

## Phase 8 — Data sensitivity declarations (DS-XXX)

Thin declarations only — supplier credentials as the most restricted class on the platform with a no-logging rule; per-supplier stock as confidential counterparty data; override actor identity as personal data. Note that both API vendors sit outside the UK/EU while the platform's data-residency rule says data stays in region, and record what actually crosses the boundary (our credentials outbound, stock inbound) rather than hand-waving it. Full per-attribute classification waits for the data model.

## Phase 9 — Self-review and report

I would check every target traces back to a functional requirement or an ops note; every integration has a named failure mode and fallback; every constraint cites the document that evidences it; no target uses a word like "reliable" or "near real-time"; nothing in the spine is marked decided by me. No tests to run — there is no code in this workspace and these are documents; the closest thing to a check is the traceability pass.

**The handoff to ops would say:** here are the freshness and reliability targets, replacing "never stale", which cannot be built; here is the poll volume and write volume at 68 and at 500 connected suppliers with the arithmetic shown; here is the compute recommendation with the trip-point for the second machine and its cost against your headroom; here is the credential store you do not have today and the decision that needs platform sign-off; here are three integrations with their failure modes, including that roughly one night in eight there is no file and that a missed Shopify event is gone for good. **Blocking before this can be sized honestly:** the egress-IP behaviour on scale-out, the unowned Northern Provisions code mapping, the fail-open/fail-closed ruling, four architecture rows awaiting rulings, and the per-supplier product-count figure I had to assume.