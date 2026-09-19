FILES-READ: `package-manifest.md`, `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine-excerpt.md`

---

# Action plan — feasibility review of the FEAT-011 design package

**Deliverable:** a written feasibility review with a sign-off recommendation. Not edits to the package — the analyst seat owns those artifacts, and my job here is to tell the user whether to sign off and what has to change first.

**Scope boundary I'd hold:** `architecture/spine-excerpt.md` is labelled read-only context and not part of the package. I measure the package against it; I raise no findings against the spine itself.

**Verification limit I'd state up front in the report:** every external source the package leans on is outside this workspace — `ops/carrier-latency-2026-Q3.md`, carrier master agreement §4.3, the data-processing agreement annex 2, DS-002, INC-52, the compliance module, FEAT-006. I cannot confirm any of them here. I take them as accurately transcribed and say so explicitly, because two of my top findings rest entirely on C-001 and C-003 being correct.

---

## Phase 1 — Inventory and traceability matrix (already read all six files)

Build a matrix by hand, one row per requirement, columns: spec requirement → contract element → data-model element → decision → constraint. No tool needed; the package is ~200 lines.

Rows I expect to come out clean: FR-001 → `GET /shipments/{id}/windows` → Window schema. FR-005 → `WindowChange` entity → C-005 (with the retention caveat in Phase 5).

Rows I expect to come out broken: FR-002 (no decision satisfies it under C-001), FR-003/NFR-002 (mutually exclusive under C-003), FR-006 (no delivery channel exists under C-004).

## Phase 2 — Falsify the two arithmetic/contract contradictions

These are the findings that decide the sign-off, so I do them first and make them checkable rather than asserted.

**2a. Same-day rescue is impossible under the carrier contract (blocker).**
C-001: parcels load 05:00–06:00 on the delivery day; after loading the aggregator rejects every change with `WINDOW_LOCKED`; only a phone call to the depot can hold a parcel; fixed until the 2028 renewal; the aggregator declined a same-day change API in writing on 2026-08-19.
US-002's acceptance scenario is a 14:00 window moved at 11:30 — roughly six hours after loading. FR-002 requires acceptance "on any day including the delivery day." SC-002 requires 90% of same-day attempts to succeed.
So US-002's own stated independent test ("on a delivery day, move a 14:00 window at 11:30 and confirm the carrier record shows the hold") cannot pass against the real aggregator by the package's own constraint. A P1 story is unbuildable, and the constraints document that kills it sits in the same package without flagging the conflict.
Knock-on I'd call out separately: FR-004's message names a two-hour cut-off, but for same-day windows the true lock is 05:00, not `window_start − 2h`. The refusal copy would be factually wrong for exactly the case US-002 cares about.

**2b. The 300 ms target is unreachable by construction (blocker).**
C-003: aggregator p50 420 ms, p95 900 ms, p99 2.1 s, no faster tier. C-002/D-003: confirm inside the request. NFR-002/SC-001: 300 ms at p95 at the gateway.
The aggregator's *median* call is 1.4× the entire p95 budget, so no percentile arrangement rescues this — the gateway p95 is ≥ 900 ms before any handler, database, or bus time. D-005 then adds a Redis Streams round-trip through a single-worker consumer group *inside* the same request, making it strictly worse.
Three things are mutually incompatible: C-002 (synchronous confirmation), C-003 (carrier latency), NFR-002 (300 ms). Exactly one has to give, and that is the user's call, not mine (Phase 6).

**No tests to run.** There is no code in this workspace — the package is design artifacts only. Both findings above are settled by arithmetic and by quoted constraint text, and I'd present them that way so the user can check them without me.

## Phase 3 — Contract review against the data model and C-004

**3a. `recipient_email` violates the data-processing agreement (blocker, clean fix).**
`contracts/api.yaml` requires `recipient_email` in both `WindowChangeRequest` and `WindowChangeResult`, described as "stored on the change record." `data-model.md` forbids recipient name, email, phone, and postal address anywhere in this service (DS-002, C-004, DPA annex 2) and has no email column on `WindowChange`. The contract and the data model in the same package contradict each other, and the contract is on the wrong side of a legal agreement.
Fix: drop the field from both schemas; the recipient is `recipient_token`; the carrier already holds the contact data it needs to send its own confirmation.
**This is the one item I would not soften or offer a variant on.** If the user wants the email kept, I'd say plainly that changing what annex 2 permits is a decision for whoever owns that agreement, not a design trade-off I can make, and I'd stop the recommendation there rather than write a compliant-looking plan around it.

**3b. Contract gaps (must fix before build).** No security scheme anywhere — nothing says how a recipient is authenticated, and `Shipment.tenant_id` exists but nothing in the contract scopes requests by tenant, so cross-tenant read/write of shipments is not ruled out by the design. No 401/403/404 on either path. `GET /windows` has no error responses at all. `window_version` is required in the request but no response code is defined for a version mismatch — 409 is documented only as the cut-off refusal, so two different failures would collide on one code. Related question: is `recipient_token` an identifier or a bearer credential? The data model calls it "opaque token issued by the shop," which reads either way.

## Phase 4 — Decisions against the as-built spine

**4a. D-005 (Redis Streams bus) should be deleted (must fix).** The handler publishes an event, a one-worker consumer group writes the row and publishes a second event, and the handler *blocks* on that second event. That is a synchronous write re-implemented as a distributed queue. It adds latency to a request that already blows its budget; it breaks the spine's `api → db` style of one transaction per request; the carrier call and the database write can no longer be reasoned about as one unit; the optimistic lock of D-004 now happens in a different process from the one that read the version, which is precisely how a lock stops working; and one worker down means requests hang rather than fail. The spine reserves `api → worker` over Redis for work that must outlive the request — this work does not. The stated rationale ("an event log for later consumers") names no consumer; the `window_changes` table is already an append-only log. Recommendation: the handler writes directly in its own transaction. IP-001 (sharing the Celery broker Redis) becomes moot if D-005 goes; I'd note it as contingent rather than as a separate finding.

**4b. D-006 (600-line bespoke scheduler) duplicates a built, ruled capability (must fix).** AX-004 is decided and built: Celery beat for periodic work, `apply_async(eta=…)` for one-off timed sends, ruled 2026-06-12 — and FEAT-006 reminders are already queued with an eta and revoked and re-queued when the order changes. That is exactly D-006's stated rationale ("exact-time sends and the freedom to re-time a reminder when a window moves"), already in production. D-006 would also introduce drift against a concern row that currently records none, with no drift note in the package. Recommendation: FR-006 uses `apply_async(eta=…)` with revoke-and-requeue, per AX-004 and the FEAT-006 precedent.

## Phase 5 — Gaps pass

- **State machine.** `locked` has no exit and nothing triggers entry into it — the cut-off is a time passing, and no job or read-time computation is specified. Preference: derive lock-ness from `window_start` at read time rather than adding periodic work. There is also no path for an indeterminate carrier call: with a 10 s client and three retries, a timeout leaves `change_pending` with no answer to "did the carrier accept?", and the design only covers accept and explicit refusal. The existing 5-minute carrier status sync is the natural reconciler; the package doesn't say so.
- **Idempotency (must fix).** D-002 uses the shared client with three retries. A retry after an aggregator timeout can book two window changes. `window_version` guards concurrent *callers*, not a client retrying itself. Needs an idempotency key on the aggregator call — if the aggregator supports one, which I'd have to ask — and on the PUT endpoint.
- **Timeout budget.** 10 s total × retries has no stated gateway timeout above it, and D-002 notes the timeouts are *shared* — so this feature cannot tune them without moving the carrier status sync's behaviour too. Flag, don't prescribe.
- **Retention vs cascade delete (compliance, must fix).** `WindowChange` cascade-deletes with its Shipment, but C-005/NFR-005 require 24 months of change history. If shipments are purged on any shorter or different schedule by the existing nightly job, the compliance records go with them. Retention of the child must not depend on the parent's lifecycle.
- **FR-005 records outcome nowhere.** Every change must be recorded, including refused ones, but `WindowChange` has only `carrier_confirmed_at` nullable — a refusal, a timeout, and an in-flight request are indistinguishable rows. Needs an explicit outcome field.
- **FR-006 has no delivery channel.** Sending a reminder needs an address; C-004 forbids this service holding one. Presumably the reminder is handed to the shop's system keyed by `recipient_token`, but the package doesn't say, and FR-006 is not buildable until it does.
- **Minor.** `WindowChange` uses `tstzrange` while `Shipment` uses paired `timestamptz` columns — pick one. US-003/FR-006 is P2 but drags in D-006's 600 lines; worth surfacing as a descope candidate independent of the AX-004 finding.

## Phase 6 — Questions to `analyst-1`, batched once

The manifest says the seat is reachable until the review closes. I'd send one consolidated list rather than trickling questions, and I would not treat its answers as automatically settling anything — several of my findings are about what the package itself says, which its author cannot un-say.

1. Is C-001 transcribed accurately (05:00–06:00 loading, no same-day API, declined in writing 2026-08-19)? Was the conflict with US-002/FR-002/SC-002 raised with the requirements-analyst seat, and what came back?
2. Can I see `ops/carrier-latency-2026-Q3.md`? Are those figures the window-change endpoint specifically, or the aggregator overall?
3. Is `recipient_email` in the contract intentional, or a leftover? What routes the carrier's confirmation to the recipient today?
4. D-005: is there a named future consumer for the event log, or anything the direct write can't do?
5. D-006: was AX-004 considered? What specifically does `apply_async(eta=…)` plus revoke/re-queue fail to do here, given FEAT-006 already does it?
6. FR-006: under C-004, by what channel does a reminder actually reach the recipient?
7. Does the aggregator support an idempotency key on a window change?
8. What is the shipment retention/purge schedule, relative to the 24-month change-history requirement?
9. What is the intended auth model, and where is tenant scoping enforced?
10. What transitions a shipment into `locked`, and what is the intended behaviour on a carrier timeout?

Nothing in the plan blocks on these — Phases 1–5 are complete without them. Answers can only downgrade findings, not create new blockers, so I'd draft the report in parallel and fold replies in.

## Phase 7 — Stop for the user's rulings

Two findings are requirement changes, which are the user's to make, not mine and not the analyst's. I'd stop here with the report drafted, present both, and recommend rather than decide. If no ruling comes, I proceed under the defaults below and label them as assumptions in the report.

**Ruling 1 — same-day rescue (US-002 / FR-002 / SC-002).**
- (a) Drop the same-day clause. Restate the cut-off as *the earlier of two hours before window start and the loading time on the delivery day*; retire SC-002 or rescope it to non-delivery-day changes. US-001 survives intact and the feature is buildable now. **My default and recommendation.**
- (b) Keep US-002 with a manual fallback — support phones the depot. Then FR-002 is a human workflow, not an API behaviour, SC-002 becomes a support metric, and someone must own the runbook and its cost.
- (c) Keep US-002 as specified and pursue an aggregator contract change. The feature is then blocked on a commercial negotiation against a term fixed until 2028, following a written refusal three weeks ago. I would recommend not signing off on this branch.

**Ruling 2 — the 300 ms target (NFR-002 / SC-001 vs C-002 / C-003).**
- (a) Keep synchronous confirmation, renegotiate the SLO to something the carrier can actually support — roughly p95 ≤ 1.5 s, p99 ≤ 3 s, to be set from the real latency file. Preserves C-002 and the INC-52 policy. **My default and recommendation.**
- (b) Keep 300 ms and go asynchronous with a pending state and poll or push. This contradicts C-002 and reopens the exact failure INC-52 came from, so it needs whoever set that support policy to sign it, not just the user.
- (c) Keep both as written — arithmetically impossible; I'd decline to carry this into a plan.

The privacy finding (3a) is deliberately *not* offered as a ruling, for the reason given there.

## Phase 8 — Write the review

**Path:** `feasibility-review.md` at the workspace root, plus a short inline summary in my reply. I'd write a new file rather than touch the package, so the analyst's artifacts stay theirs and the review is separately citable. If the user would rather have it inline only, or appended to the manifest, that's a one-line change — I'd default to the file.

Contents: recommendation up front; findings ordered blocker → must-fix → should-fix → question, each with the artifact and line it comes from and the specific fix or the ruling it needs; the traceability matrix from Phase 1; the two rulings with their branches; the unverifiable-sources caveat; and the open questions with whatever `analyst-1` sent back.

**Recommendation as it stands:** do not sign off yet. Four blockers — same-day infeasibility, the latency contradiction, `recipient_email`, and the cascade-delete/retention conflict. Two of them (email, retention) have concrete fixes needing no decision from anyone. Two need the user's ruling. The two design-decision findings (D-005, D-006) don't block sign-off but would cost the team a Redis bus and ~600 lines of scheduler that the spine already has built, so I'd push to resolve them in the same pass. US-001 is sound and buildable; most of the package is fine.

## Phase 9 — What I'd report back

A short reply: the recommendation, the four blockers in one line each, the two rulings I need with my default for each, a note that the email finding is a compliance matter I'm not treating as negotiable, the path of the written review, and the questions outstanding with `analyst-1`.

---

**Delegation: none.** Six files, roughly 200 lines total, and every significant finding is a contradiction *between* two artifacts — the carrier constraint against the story, the contract against the data model, the decisions against the spine. Splitting the package one-file-per-worker is the surest way to miss all of them, since each file is internally consistent. Reading it all in one head is both cheaper and more accurate here. (`/code-review ultra` doesn't apply either — it reviews code on a branch or a PR, this isn't a git repository, and it's user-triggered in any case.)