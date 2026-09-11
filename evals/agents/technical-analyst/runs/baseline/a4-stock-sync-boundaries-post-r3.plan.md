FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/docs/supplier-systems.md`, `pallet/ops/hosting.md`, `pallet/specs/live-stock/spec.md`

---

# Action plan — quality targets, infrastructure and integration needs for live stock (FEAT-017)

## What ops is actually asking for, and what I will and won't write

Ops wants numbers they can size against. That means: measurable quality targets, the provisioning items those targets imply, and the integration surface with its failure modes — anchored to technical requirements so each number traces to something in the signed-off spec.

Artifacts I would write:

- `pallet/specs/live-stock/technical-requirements.md` — the FR→TR translation, kept tight; it exists because every quality target, integration row and provisioning item needs something to hang off.
- `pallet/specs/live-stock/constraints-and-decisions.md` — constraints (C-), decisions in ADR form (D-), provisioning items (IP-), and the thin integration (INT-) and data-sensitivity (DS-) declarations.
- `pallet/specs/live-stock/sizing-note.md` — the arithmetic ops needs: requests per minute, jobs per minute, rows written, and the monthly cost delta against the £88/month headroom, with the poll-interval trade-off shown as a table rather than a single guess.
- Proposed amendments to the concern rows in `pallet/architecture/spine.md`, where the quality targets are homed (AX-010, AX-011, AX-013, and new rows for stock freshness and connector health). I write the proposed rows; I do not myself flip a `not-now` ruling to `decided` — see Phase 8.

Artifacts I would *not* write on this card, recorded as deliberate absences in the constraints doc: the full data model, the OpenAPI contract, and the integration guide. The card is the pre-sizing analysis pass. One exception: I will classify the per-supplier credentials now, as a thin declaration, because that classification is what forces a provisioning item and ops cannot size around it later.

## Phase 1 — Reconcile what I have read, and name the three contradictions before anything else

No further reading needed: the workspace is seven files and I have read all of them. I would not dispatch a sweep for a repo this size.

The three things I would settle before writing a single target, because each one changes the numbers:

1. **"Stock shown is never stale" (SC-002) cannot be built, and not for want of effort.** Lightspeed is polling-only, so its stock is stale by construction between polls. Shopify events arrive out of order, and on the plan most of those 21 suppliers are on a lost event is gone forever with no history or bulk endpoint to reconcile against. The Sage CSV lands once a night and was absent on 4 of the last 30 nights and truncated twice. There is no reading of those three systems under which stock is never stale. I will not write SC-002 as a quality target. I will replace it with a per-source freshness target plus a visible staleness signal, and take that back to Priya.
2. **"Stock levels are not sensitive, so no special handling is needed" is answering the wrong question.** The stock number may be unremarkable; the things needed to fetch it are not. Lightspeed OAuth access and rotating refresh tokens, Shopify per-shop webhook secrets, and an SFTP password for Northern Provisions are all credentials belonging to another business, and the hosting notes say plainly there is nowhere to put them today. The spine says per-tenant credentials have no ruling and nothing holds any. This is a blocker for FR-002, not a footnote.
3. **The spec's product-matching assumption is already known false for nine suppliers.** The supplier survey says the Sage export uses Northern Provisions' internal codes, they do not match the SKUs those nine suppliers print, and nobody has mapped them. Separately, `supplier_sku` is null on 39% of products. FR-002 is unsatisfiable for the `sftp_csv` cohort as things stand.

## Phase 2 — Translate the five FRs into technical requirements

Write `technical-requirements.md`. Each row gets acceptance criteria and names its source FR. The decomposition I expect, so ops can see the work:

- FR-001 → availability computation from stored stock plus a per-product availability mode (tracked / always-available / untracked); the four-state derivation including the "low" threshold, which the spec never defines and I will flag as needing a number from Priya (default: a per-supplier threshold, default 3 units); basket-add rejection with a specific error type; and the catalogue read path staying a single query per supplier, because the existing catalogue does that today and breaking it trips the caching trigger in AX-010.
- FR-002 → three separate connector requirements, not one: a Lightspeed poller with paginated fetch, 429 back-off honouring `Retry-After`, and safe rotating-refresh-token handling; a Shopify webhook receiver with per-shop HMAC-SHA256 verification, out-of-order suppression by payload `updated_at`, and duplicate tolerance across Shopify's 19 retries over 48 h; an SFTP CSV importer with file-presence and truncation detection. Plus a shared requirement for connection lifecycle (connect, token/credential storage, disconnect, reconnect-after-lost-rotation).
- FR-003 → override write path recording the acting person and time; override-versus-sync precedence; and whether an override expires. Stock is not a financial record, so the existing audit trail (AX-008) does not cover this; the "recorded against the person who made it" wording in FR-003 is an audit requirement in all but name.
- FR-004 → manual stock entry and an always-available mode, which is also the escape hatch for the 72 unconnected suppliers and, on my default ruling, for the nine Sage suppliers.
- FR-005 → a re-check of availability at basket view and at checkout submission, with a distinct response for "went out of stock since added". Since reservation is explicitly out of scope, FR-005 is the only thing standing between a retailer and the very problem the feature exists to fix; its latency target matters more than the catalogue's.

Implicit needs the spec does not mention that I would surface as their own rows: outbound request budgeting per supplier token, per-supplier connector health visible to the supplier (four engineers and "on-call is whoever is awake" cannot babysit 500 connections), stock-history retention and its interaction with the existing nightly purge, and deterministic job ids for every sync job because the governance rule requires them.

## Phase 3 — Constraints, each with a real source

Write the C- rows in `constraints-and-decisions.md`. Every one cites a file and, where the source is an external commitment, its date:

- Infra budget £300/month, £212 spent, so £88/month of headroom (hosting notes, Tom's e-mail 2026-04-14).
- One worker machine, and ops would rather not add a second (hosting notes).
- The worker's egress address `149.248.201.77` is what Northern Provisions' firewall allows, and changing it needs ten working days' notice to their IT (hosting notes; survey, confirmed 2026-08-19). Consequence ops will care about: any connector topology that moves outbound traffic off that worker app breaks the CSV fetch, and it breaks it two weeks before anyone can fix it.
- Only `api` has a public address; the worker has none. Shopify webhooks therefore must land on `api`.
- PostgreSQL is the only datastore; a new one needs platform sign-off recorded in the spine.
- Every background job must carry a deterministic id and check for prior completion.
- Errors are RFC 7807 problem documents built in one place; counterparty data never appears in logs, exports or error bodies.
- Data stays in UK/EU — relevant because Dutch producers onboard from Q4 2026 and the region must remain `lhr`.
- Lightspeed: 60 requests/minute/token, 250-row pages, 24 h access token, refresh token rotates on every use.
- Shopify: no bulk or history endpoint on the relevant plan; out-of-order delivery; 19 retries over 48 h.
- Sage/SFTP: nightly ~02:00 arrival, observed 13% miss rate, two truncations in thirty nights, codes unmapped.
- Scale target: 500 connected suppliers within a year, against 140 suppliers today.
- Coverage must not fall below 75%, and integration tests run against a real PostgreSQL.

I would keep "we shouldn't hammer suppliers' systems" out of the constraint list as phrased and turn it into a numeric outbound budget in Phase 6; and I would classify "one worker is enough today" as a preference with a cost attached, not a hard constraint, so ops can see it is theirs to trade.

## Phase 4 — Integration declarations with failure modes

Three INT- rows, each naming protocol, auth, direction, rate limits, and — the part that is usually missing — what the system does when it fails:

- **INT-001 Lightspeed Retail (X-Series), outbound poll.** Failure modes: 429 (back off, honour `Retry-After`, shed the poll rather than queue-bomb); access-token expiry (refresh); **lost refresh-token rotation, which requires the supplier to reconnect by hand** — this one deserves loud treatment, because a retried job that replays an already-consumed refresh token is how you brick 38 connections at once. The fallback: freeze the last known stock, mark the connection broken, surface it to the supplier, and stop polling rather than retry into a wall.
- **INT-002 Shopify, inbound webhook.** Failure modes: HMAC verification failure (reject, count, never echo the payload); replay/duplicate (idempotent by shop + inventory item + `updated_at`); out-of-order (drop anything not newer than the stored watermark); **permanently lost event with no reconciliation path** — the honest fallback is a staleness clock per product and a visible last-synced time, not silence.
- **INT-003 Northern Provisions SFTP CSV, outbound pull.** Failure modes: file absent by a cut-off time (alert, keep yesterday's values, do not zero anything); truncated file (row-count and checksum sanity check against the previous night, reject the whole file rather than half-apply it — a truncated file silently reads as "everything after row N is out of stock", which would block orders for products that are fine); unmatched product codes (the current state for all nine suppliers).

## Phase 5 — Classify the data, and contradict the spec in writing

Thin DS- declarations. Stock level and availability: internal, not confidential, retention modest — the spec is right about that much. Then the rows the spec missed: Lightspeed access and refresh tokens, Shopify per-shop webhook secrets, and the SFTP password are Restricted credentials belonging to a counterparty, must be encrypted at rest, must never appear in a log line, an export, or an error body, and need a rotation and revocation-on-disconnect path. Supplier stock levels in aggregate are commercial information about a counterparty's trading position, so they inherit the confidentiality expectation for counterparty data even though an individual number is dull.

I would also note that a supplier's override reason ("holding back for a market") is free text a person typed about their own business, so it is not log-safe either.

## Phase 6 — The quality targets

These are the numbers ops came for. Each gets a target, how it is measured, and why that number and not another. They are homed on concern rows in the spine rather than a standalone file.

- **Freshness, stated per source class**, because one number across three architectures would be a lie: Shopify, p95 under 60 s from the event's `updated_at` to the value being visible, measured from stored `synced_at` minus payload `updated_at`; Lightspeed, p95 staleness at or below the poll interval plus 60 s, with the interval itself set in Phase 8's trade-off table; SFTP cohort, staleness up to 26 h on a good night and no guarantee on the 13% of nights the file is missing — which is the argument for not showing these suppliers as live-tracked at all.
- **Sync success rate**, replacing "sync must be reliable": at least 99% of scheduled Lightspeed polls complete per supplier per day; at least 99.9% of verified Shopify webhooks applied or explicitly dead-lettered; CSV applied on at least 85% of nights, which is what their IT's current 4-in-30 miss rate actually supports, with the gap alerted rather than absorbed.
- **Outbound politeness**, replacing "we shouldn't hammer their systems": never exceed 50% of Lightspeed's documented 60 rpm per token averaged over a minute, hard-stop at 80%, and zero sustained 429s in steady state.
- **Read-path latency**: catalogue p95 at or below 400 ms, because that is the exact number the spine already set as the trigger for reconsidering caching; availability re-check at checkout p95 at or below 150 ms, because it sits in the submit path.
- **Connector recovery visibility**: a broken connection is visible to the supplier within 15 minutes and to the team by alert within 15 minutes. Measured from the first failed attempt.
- **Detection of staleness**: no product displays a tracked availability whose backing value is older than the source's stated freshness bound without a visible staleness indication.

What I refuse to write: "near real-time", "reliable", "never stale". Each becomes one of the above or becomes an open question.

## Phase 7 — The decisions, each weighed against real alternatives

ADR-format entries in the same document. The ones that genuinely have to be decided before sizing:

- **D-001 Ingest shape** — one uniform poller for everything, versus per-source connectors, versus webhook-first with a poll fallback. Whether a Shopify poll fallback is even available on these suppliers' plan is an unknown I would flag rather than assume; it decides whether lost events are recoverable.
- **D-002 Where stock lives** — fields on `Product`, versus a separate stock table, versus an event log with a projection. This is the decision with the most consequences: a separate table makes the catalogue read more than one row per product, which by the spine's own wording fires the caching trigger; an event log is the cleanest audit story for FR-003 but the most write volume; fields on `Product` keep the single-query catalogue and collide with `@updatedAt` semantics and with the override/precedence model.
- **D-003 Per-supplier credential storage** — Fly secrets (does not scale per tenant and is not what they are for), versus encrypted columns in PostgreSQL with the wrapping key in a Fly secret, versus a managed secret service (which is a new datastore and needs platform sign-off, and likely eats the budget headroom). My leaning is the middle option; the decision is not mine alone because the spine has no ruling for per-tenant credentials.
- **D-004 Override precedence and expiry** — does the next sync overwrite a manual override? US-003's market example implies the override must win, which implies it must also expire or be cleared, or that supplier's stock is frozen forever and the feature quietly stops working for them.
- **D-005 Ordering and duplicate suppression** — `updated_at` watermark per inventory item versus a per-shop sequence.
- **D-006 Safe refresh of a rotating refresh token** — single-flight refresh serialised on a row lock, with the refresh step marked non-retryable, versus refresh inline in a retryable job. The second is the one that loses connections; it needs to be written down as rejected so nobody rediscovers it.
- **D-007 How staleness reaches the retailer** — a hard block on stale data, versus showing availability with a "as of HH:MM" qualifier, versus treating unverifiable sources as always-available. Product-facing, so Priya rules.
- **D-008 Auditing overrides** — extend the existing `audit_events` table to a non-financial record type, versus a dedicated override table carrying the actor. Touches an existing ruling, so it is flagged for the architect.
- **D-009 The Sage cohort** — defer them, build a code-mapping surface, or get a mapping from Northern Provisions. Default: defer to manual/always-available under FR-004.
- **D-010 Outbound rate control** — per-token limiter in the queue layer versus fixed schedule spacing.

## Phase 8 — Provisioning items and the sizing arithmetic

Write `sizing-note.md` and the IP- rows. The arithmetic I would show, with every input labelled as measured or assumed:

- Requests per minute at the year-one target, as a function of poll interval and of how the 500 suppliers split across the four source types. Today's split (38 / 21 / 9 / 72) scaled forward is an assumption I will mark as such and ask ops to sanity-check, because the answer moves the worker sizing more than anything else. A worked example: if roughly 270 of the 500 end up on Lightspeed and each fits in one 250-row page, a 5-minute interval is about 54 outbound calls per minute plus parsing and writes on a single `shared-cpu-2x`; a 1-minute interval is about 270, which one machine of that size will not do comfortably. The table gives ops freshness, calls/minute, and machine count side by side so they can choose, rather than me choosing for them.
- Write volume: whether the poller upserts every row every cycle or only changed rows. Full upserts at a 5-minute interval across 500 suppliers is a large, pointless write load on an HA pair sized for today's traffic; diff-only writes are a provisioning item, not an optimisation.
- Cost delta against the £88/month headroom: a second worker machine is £14, which fits; a managed secret service probably does not. I would also flag a non-obvious one — per-supplier metric labels at 500 suppliers is high-cardinality on Grafana Cloud and can cost more than the machine, so connector health belongs in a database table with aggregate metrics on top.

IP- rows I expect to write: a public webhook route on `api` with per-shop HMAC verification and a dead-letter for unverifiable payloads; stock queues as repeatable jobs on the existing scheduled-work mechanism with deterministic ids and per-supplier concurrency caps; the worker capacity decision with the trigger for adding the second machine; preservation of the worker's static egress address, with an explicit note that a change costs ten working days of Northern Provisions' notice period; the per-supplier credential store with rotation and revoke-on-disconnect; an SFTP client with host-key pinning plus file-arrival and truncation monitoring; freshness and connector-health telemetry with the specific alerts the current two (5xx rate, queue lag) would miss — neither catches one supplier's token rotation being lost three days ago; Lightspeed sandbox credentials in staging; and the purge job's treatment of stock history.

**Stop I would flag here rather than resolve:** three spine rows change because of this feature. The rate-limiting trigger has plainly fired — Lightspeed's 60 rpm is a documented limit on our first such integration. The caching trigger fires if and only if D-002 goes to a separate table. And per-tenant credentials have no ruling at all. I would write the proposed rows and the evidence that the triggers fired, and route the actual rulings to whoever owns the spine. If the ruling is "cache now", the catalogue read path and its latency target change and I revise them; if it is "still not now", D-002 is constrained toward keeping stock on `Product` and I say so.

## Phase 9 — Stops, and what I do at each one if nobody answers

Each of these is written into an open-questions section with the branch I would take by default, so the sizing is not blocked:

1. **Priya, on SC-002.** Confirm that "never stale" is replaced by the per-source freshness targets plus a visible as-of time. If she wants a hard freshness guarantee, Shopify suppliers cannot be supported on their current plan and that becomes a commercial conversation, not an engineering one. Default while waiting: per-source targets, staleness shown.
2. **Priya, on the "low stock" threshold** the four-state display in FR-001 requires and the spec never gives. Default: per-supplier, defaulting to 3 units.
3. **Priya and ops, on the nine Sage suppliers.** Default: they are not connected sources; they use manual entry or always-available, and FR-002 is scoped to Lightspeed and Shopify for release one. Branch: if someone funds the code mapping, a mapping surface and its own quality targets are added and the Sage freshness numbers come back into scope.
4. **Architect/platform, on the three spine rows** as in Phase 8.
5. **Ops, on the worker machine and the supplier-mix assumption.** Default for the written plan: one machine with a 5-minute Lightspeed interval, and a named trigger for the second.
6. **Anyone proposing Redis as the stock read store.** I would refuse to plan it as a given; it is a second system of record and needs recorded platform sign-off first.
7. **Verification needed, not assumable:** whether the Shopify plan those 21 suppliers are on exposes any per-item read we could reconcile with. I would mark it as an unknown that changes D-001, not fill it in.

## Phase 10 — Checks before handing over

No production code on this card, so the verification is on the artifacts and on the numbers:

- Every FR has at least one technical requirement, and every technical requirement names an FR — checked in both directions, with orphans listed rather than quietly dropped.
- Every quality target has a number, a measurement method, and a source; any row failing that is deleted or converted to an open question.
- Every integration row has at least one failure mode with a stated fallback.
- Every constraint cites a file, and where it is an external commitment, a date.
- Every provisioning item traces to a constraint or a quality target; no orphan infrastructure.
- The cost total is recomputed against £212 and £300 and stated as a single number.

Tests I would specify for the build phase rather than run now: contract-suite assertions that no supplier credential, token, or override free text appears in any error body or log line; a replay test that the same Shopify delivery applied twice and out of order leaves the same stored value; a test that a truncated CSV is rejected whole rather than half-applied; and a staging measurement against the Lightspeed sandbox to replace the assumed per-poll cost in the sizing note with a measured one. I expect that last one to move the numbers, and I would say so in the note instead of presenting the estimate as settled.

## Delegation

The workspace is seven files and I read all of them, so there is nothing worth farming out here — a sweep would cost more than it saves. Against the real repository I would spawn two throwaway cheap-tier readers, one gap each, and keep their output out of my context beyond the facts: one to locate the background-job base class and the queue-id lint rule and quote how the deterministic id is enforced, so the connector jobs match it exactly; one to locate the shared logger's redaction list and the export allowlist and quote the current entries, so I know whether adding credential fields needs a change there. On return I would check each answer cites a file and line and quotes rather than paraphrases; anything vague I would read myself. The interpretive calls — what the freshness targets should be, which decisions need an ADR, whether a missing ruling is a blocker — I keep.

## What I would report

A short note to ops and Priya: the three artifacts and their paths; the headline that this is three integrations with materially different achievable freshness, not one; that SC-002 as signed off is not buildable and what replaces it; that the spec's "stock is not sensitive" conclusion is right about stock and wrong about the credentials, and there is nowhere to store them today; that nine suppliers cannot be connected until somebody maps Northern Provisions' codes; the sizing table with the poll-interval trade-off and the £14/month second-machine decision left with ops; the reminder that the worker's egress address is load-bearing and two weeks expensive to change; the three architecture rows needing a ruling; and the list of open questions with the defaults I wrote under, so anyone can see which numbers would move if a ruling goes the other way.