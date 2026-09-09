FILES-READ: `package-manifest.md`, `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine-excerpt.md`

---

# Action plan — feasibility review of the FEAT-011 design package

## Framing (decided before starting)

- **Under review:** `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md` (the `analyst-1` package). `spec.md` is an input from the requirements seat; `architecture/spine-excerpt.md` is read-only context. I review the design *against* those two, but findings that land **on** the spec still get reported — a design can be infeasible because the requirement it implements is impossible.
- **Deliverable:** one written review with itemized, severity-ranked findings and a recommendation. The sign-off is the user's; I do not sign off and do not rewrite the package.
- **Severities:** Blocker (cannot be built as written) / Major (buildable but will fail a stated criterion or the architecture ruling) / Minor / Question.

## Phase 1 — Traceability matrix

Read: all six files (done). Build a table mapping FR-001…FR-006 and SC-001/SC-002 to the artifact element that implements each, and each design decision D-001…D-006 back to a requirement or constraint. Expected gaps I will confirm and record:

- **FR-006** has a decision (D-006) but **no entity** — D-006 names "a persistent schedule table" and a "retry ledger"; `data-model.md` defines only `Shipment` and `WindowChange`. Coverage gap in the data model.
- **FR-001** says the system must "accept one of" the offered windows; nothing in the contract or decisions states server-side validation that the submitted window is one the carrier offered, and no 422 response exists for it.
- **SC-002** ("90% of same-day rescue attempts before the cut-off succeed, measured from the change log") has no denominator: `WindowChange` rows record changes, and FR-005 records *changes*, not refused or failed attempts. The criterion is unmeasurable from the designed schema. Major.

## Phase 2 — Constraint-versus-design consistency (the core pass)

For each hard constraint, check the design satisfies it. Expected findings, each of which I would state with the arithmetic or the quoted conflict, not as an opinion:

**F1 — Same-day rescue is not deliverable. Blocker.** C-001: parcels load 05:00–06:00 on the delivery day and after loading the aggregator rejects every change with `WINDOW_LOCKED`; only a phone call holds a parcel; the aggregator declined a same-day change API in writing on 2026-08-19, and the master agreement is fixed until 2028. US-002/FR-002 require accepting a change at 11:30 for a 14:00 window *that same day*, and C-002 forbids showing an unconfirmed window. These cannot all hold. The design package never acknowledges the collision — `constraints-and-decisions.md` states C-001 and FR-002 on the same page without reconciling them. Consequence: SC-002 is unachievable (the true success rate for same-day attempts after ~06:00 is 0%), and US-002's own independent test would fail. This is the headline finding and it is a **requirements-level** decision, not something `analyst-1` can design around.

**F2 — The 300 ms target is arithmetically impossible with in-request confirmation. Blocker.** NFR-002/SC-001 set 300 ms at p95 at the gateway. C-003 gives the aggregator alone p95 900 ms (p99 2.1 s) with no faster tier, and D-003/C-002 put that call inside the request. The gateway p95 cannot be below a component that is on every request's critical path at 900 ms. D-005 then adds two Redis Streams hops and a single-worker consumer round trip *before* the handler responds, and the spine's carrier client is 10 s total with three retries, so the tail is far worse. Either NFR-002 changes or C-002 changes — and C-002 is policy after INC-52, so it is the target that has to move. I will state the two branches and recommend re-baselining NFR-002 rather than weakening C-002.

**F3 — The contract stores and returns recipient email, which C-004 forbids. Blocker.** `contracts/api.yaml` makes `recipient_email` **required** in both `WindowChangeRequest` and `WindowChangeResult`, and its description says it is "stored on the change record." C-004 and DS-002 say no recipient contact data is stored, logged, or transmitted by notify-svc; `data-model.md`'s own forbidden-attributes note repeats this and `WindowChange` has no email column. So the contract contradicts C-004, the data model, and the data-processing agreement, and it is also internally inconsistent with its sibling artifact from the same seat. Note the secondary effect: an email in a required request body will land in gateway access logs and error payloads. Recommended direction (not a rewrite): carry `carrier_ref`/`recipient_token` and let the carrier notify from its own record.

**F4 — Retention conflicts with cascade delete. Major.** `WindowChange.shipment_id` is `FK → Shipment; cascade delete`, while C-005/NFR-005 require 24 months of change history and the *existing nightly retention purge* deletes rows. If a shipment is purged or deleted inside 24 months, its compliance records go with it. Needs either no cascade, a soft delete, or an explicit statement that shipments are never deleted within 24 months.

## Phase 3 — Internal correctness of the decision set

**F5 — D-004's optimistic lock cannot do its job under D-005. Blocker.** D-005 says the handler is kept "free of writes": it publishes `WindowChangeRequested`, and the `window-writer` worker writes `window_changes` and updates `shipments`. But D-003 has the handler call the carrier and wait for confirmation *before* that, and the state machine has `confirmed → change_pending` — which is a write. So either the handler writes (contradicting D-005) or the version check happens only in the worker, *after* the carrier has already confirmed. In the latter case two concurrent changes both confirm at the carrier and only one persists, leaving our record and the carrier's record diverged — the exact failure D-004 exists to prevent. I will lay this out as a concrete interleaving of two requests.

**F6 — D-005 adds a new bus, a single point of failure, and a new failure mode for no stated benefit. Major.** One worker in the consumer group means no failover; the handler blocking on `WindowChangePersisted` means a worker stall becomes a request timeout; Redis Streams is at-least-once, so the writer needs idempotency that is not designed. The spine's stated style is `api → db` synchronous SQL, one transaction per request. The stated rationale ("keeps the handler free of writes and gives us an event log") is speculative — writing in the request transaction and publishing after commit gets the log without the round trip. Recommend removing the blocking round trip.

**F7 — IP-001's "nothing new" is not accurate. Major.** Reusing the Celery broker Redis for a Streams bus is new operational surface on a shared, capacity-sensitive instance: no stream trimming/`MAXLEN` policy, no memory budget, and no statement of the eviction policy. If that Redis is not configured to reject writes rather than evict, stream entries — i.e. pending window changes — can be dropped silently. Also, contention with the Celery broker affects the FEAT-006 reminders already running there.

**F8 — D-006 is drift from a decided, built architecture ruling. Major.** Concern row AX-004 is *decided and built*: Celery beat for periodic work, `apply_async(eta=…)` for one-off timed sends, ruled 2026-06-12, and the as-built note says FEAT-006 reminders are already queued with an eta and **revoked and re-queued when the order changes** — which is precisely FR-006's "re-time it when the window moves." D-006 proposes ~600 new lines (schedule table, 30 s poll, advisory-lock leader election, retry ledger) whose rationale is a capability the existing pattern demonstrably already has, and the spine records drift as "none," so no drift entry was filed. Recommend reusing AX-004 or filing an explicit drift/change request against the spine before sign-off; either way, the current package understates its architectural cost.

**F9 — State machine gaps. Major.** (a) `confirmed → locked` fires at the two-hour cut-off, but C-001's real lock is vehicle loading at 05:00–06:00 — two different lock times, and for a next-day window the two-hour cut-off is *later* than the loading lock, so `locked` will be wrong in both directions. (b) No exit from `locked` (delivery, cancellation, carrier-side reschedule). (c) No timeout/compensation path for `change_pending` when the carrier confirms but the response is lost — the recipient sees a 502 while the carrier has moved the window, and there is no idempotency key in the contract to make a retry safe. (d) No initial state and no transition producing the first `confirmed`.

## Phase 4 — Contract mechanics pass

Read `contracts/api.yaml` line by line against the OpenAPI 3.1 structure. Findings I expect to record (all Minor unless noted):

- `{id}` is never declared as a path parameter on either path.
- `409` and `502` say "Problem Details" but define no `content`/schema; there is no Problem Details component.
- No security scheme at all, and no tenant scoping despite `Shipment.tenant_id`; nothing establishes how `WindowChange.requested_by` (recipient/shop/support) is authenticated — Major, because that enum is a compliance field per FR-005.
- Missing 404 (unknown shipment) on both paths; missing 422 (window not among those offered); no 412/428 for idempotency.
- **409 is overloaded** — it is the cut-off refusal (FR-004) *and* the natural code for a `window_version` conflict (D-004), with no distinguishing error code, so a client cannot tell "too late" from "try again." Major.
- `Window` has no constraint that `end > start`; no timezone guidance despite same-day cut-off logic.
- GET response has no pagination, caching, or freshness statement even though offered windows are volatile and the aggregator is slow (C-003).

If a shell were available I would run an OpenAPI linter over the file and expect it to report the undeclared path parameter and the empty response contents; since I cannot run anything here, this is a manual read and I will say so in the report rather than implying tooling confirmed it. There is no code in the workspace, so there are no tests to write or run — the artifacts are documents and the review is a document review.

## Phase 5 — Questions to `analyst-1`

I would send **one batched message**, not a stream, and only for things the documents genuinely cannot settle. I would *not* ask the seat to adjudicate F1, F2, F3, or F5 — those are decidable from the package itself, and asking would just launder my finding through the author.

Questions:
1. `recipient_email` in the contract — is this an actual aggregator requirement discovered during design (in which case there is a real problem needing a design answer), or an oversight against C-004?
2. C-001 — does `WINDOW_LOCKED` after loading apply to a change whose *target* is a later day, or only to the current day's slot? My reading is that the parcel is locked regardless of target; confirming this decides whether F1 is total or partial.
3. Was AX-004 considered for FR-006, and is a spine drift entry filed for D-006?
4. Does anything hard-delete a `Shipment` inside 24 months (F4)?
5. Is there a memory/trim/eviction plan for the Streams bus on the shared Redis (F7)?

On return I would check each answer against the artifacts rather than accepting it: for (2), whether it is sourced to the master agreement or the 2026-08-19 refusal letter or is a recollection; for (3), whether a drift entry actually exists or is merely intended. If an answer contradicts a document, the document wins and I record the discrepancy. If the seat is unreachable or slow, I proceed and mark those five items "unanswered — assumption stated," since none of them changes the Blocker set.

## Phase 6 — Stop point: the recommendation

This is where a human ruling is needed. I would present the verdict and the branches rather than deciding scope myself:

- **My default recommendation: do not sign off yet.** F1 and F3 cannot be resolved inside the design — F1 needs a requirements decision, F3 needs a contract change plus confirmation that the carrier can notify from its own data. F2 needs a re-baselined number that only the user owns.
- **Branch A — user rules "descope same-day rescue":** US-002, FR-002's same-day clause, and SC-002 are removed or re-scoped to the phone-call path; the design becomes feasible for next-day changes, and F2/F5/F6/F8 remain as required rework before build.
- **Branch B — user rules "same-day must ship":** the only route within C-001 is the depot phone call, which is a process/support feature, not this API; the package needs a new design and C-002 needs an explicit exception. I would say that plainly and not attempt to design it in the review.
- **Branch C — user rules "sign off with conditions anyway":** I record the conditions as named, owner-assigned follow-ups (contract fix for F3 before any client integrates; NFR-002 re-baselined before SC-001 is used as a gate) and note in writing that F1 makes US-002 undeliverable regardless.

I would not narrow the review to the easy contract nits to reach a "pass," and I would not edit `analyst-1`'s files to fix anything — the package is theirs, and fixing it in place would hide the findings the user is signing off against.

## Phase 7 — Write the report

Write one new file, `reviews/FEAT-011-feasibility-review.md` (new path; nothing existing is overwritten, and I would not touch the six package files). Contents:

1. Verdict line and scope statement (what was reviewed, what was context).
2. Findings table: ID, severity, artifact and line, the conflict stated with its evidence, and the direction of fix — Blockers F1, F2, F3, F5 first.
3. Traceability matrix from Phase 1 with the FR-006 and SC-002 gaps marked.
4. Contract defect list from Phase 4.
5. Open questions to `analyst-1` with answers received or marked unanswered.
6. The three branches from Phase 6, with my recommendation.

**What I would report back to the user at the end:** that the package is internally consistent in style but not feasible as written; four blockers, of which the two that matter most are that C-001 makes the same-day rescue story undeliverable through this API and that the contract requires an email field the data-processing agreement forbids; that the 300 ms target and the synchronous carrier call are mutually exclusive by the aggregator's own measured latency; that D-006 diverges from a built architecture ruling that already does what it needs; and that the decision on US-002 is theirs, with the three branches and my default of holding sign-off.