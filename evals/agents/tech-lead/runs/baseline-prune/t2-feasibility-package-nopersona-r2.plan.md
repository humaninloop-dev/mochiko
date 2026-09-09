FILES-READ: `package-manifest.md`, `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine-excerpt.md`

---

# Plan — feasibility review of the FEAT-011 design package

## Framing

Six files, ~200 lines total. This is a desk review of a design package before a human sign-off gate: there is no code, no test suite, and no repository in this workspace, so "verification" means cross-document tracing and arithmetic, not test runs. The deliverable is a review verdict with findings, not a rewritten package. I would do the whole review myself in this session (details in Phase 8) — the package is small enough that splitting it would cost more in coordination than it saves.

The read above is the complete read; nothing further is needed before analysis.

---

## Phase 1 — Build the trace matrix

**Do:** Construct one table linking every requirement to the artifacts that are supposed to realise it, so gaps show up as empty cells rather than as impressions.

Rows: US-001, US-002, US-003, FR-001…FR-006, SC-001, SC-002, NFR-002, NFR-005, C-001…C-005.
Columns: data-model support · API contract support · decision (D-00x) covering it · state-machine transition · conflict.

**Read:** all four package files plus `architecture/spine-excerpt.md` as read-only context (the manifest is explicit that the spine is not under review — I would not raise findings against the spine itself, only against the package's conformance to it).

**Expect to show:** FR-006 has no data-model support at all (no reminder-schedule field on `Shipment` or `WindowChange`); FR-005's audit row is never created for refused attempts; SC-002 has no measurable source.

**Write:** nothing yet; matrix held in working notes and folded into the Phase 9 report.

---

## Phase 2 — Feasibility check 1: same-day rescue against the carrier constraint

**Do:** Compare US-002 / FR-002 against C-001 directly.

- FR-002 requires accepting a change "up to two hours before the start of the current window, on any day including the delivery day."
- C-001 states parcels are loaded 05:00–06:00 on the delivery day and after loading the aggregator rejects every change with `WINDOW_LOCKED`; only a phone call to the depot can hold a parcel; the agreement is fixed until 2028 and the aggregator refused a same-day change API in writing on 2026-08-19.
- US-002's own acceptance scenario is a 14:00 window moved at 11:30 — roughly six hours after loading.

**Conclusion I expect to record:** the P1 story the feature is partly justified by cannot be built as written. This is the top blocker. SC-002 ("90% of same-day rescue attempts made before the cut-off succeed") follows it down: measured against the real carrier behaviour, the achievable rate is approximately zero, not 90%.

Note the cross-seat dimension: FR-002/US-002/SC-002 come from the requirements-analyst seat's `spec.md`; C-001 comes from `analyst-1`. Nothing in the package acknowledges the collision — the state machine and `api.yaml` model only a two-hour cut-off, with no depot-loading lock anywhere. So this is not merely an analyst-1 defect to hand back; it needs a scope ruling from the user (Phase 9).

**I would flag, not resolve:** the resolution options are (a) drop same-day rescue from scope, (b) implement it as a phone-call-to-depot workflow outside the API with a much weaker success criterion, (c) reopen the aggregator negotiation, which C-001 says is closed until 2028. Choosing among these is the user's call.

---

## Phase 3 — Feasibility check 2: the latency budget arithmetic

**Do:** Add up the stated numbers on the request path defined by D-003 + D-005.

- Target: NFR-002 / SC-001 = 300 ms at p95, measured at the API gateway.
- C-002 + D-003: the carrier confirmation happens inside the request; the recipient never sees an unconfirmed window.
- C-003: aggregator p95 = 900 ms, p99 = 2.1 s, no faster tier offered.
- D-005 adds, inside the same request: publish to Redis Streams → single-worker consumer group picks it up → DB write → publish second event → handler blocks until it observes that event.

**Expect to show:** the carrier call alone puts p95 at ≥ 900 ms — 3× the 300 ms budget — before any local work, and D-005's handshake is pure addition on top. D-002's client (10 s total, three retries) means the tail is bounded at ~10 s, far outside anything a gateway budget tolerates. The 300 ms target and the confirm-inside-the-request rule are mutually exclusive with the measured carrier latency; one of NFR-002 / C-002 must change. I would note that C-002 has a strong justification behind it (INC-52: a window shown, never confirmed, parcel returned), so my recommendation is to relax NFR-002/SC-001 to a number derived from C-003, not to relax C-002.

**Test:** arithmetic only; I would show the addition explicitly in the report rather than asserting "too slow."

---

## Phase 4 — Compliance check: contact data and retention

**Do:** Two checks against the stated compliance constraints.

1. `contracts/api.yaml` requires `recipient_email` in `WindowChangeRequest` (line 46, with the description "stored on the change record") *and* in `WindowChangeResult` (line 56). C-004 and the data model's forbidden-attributes block prohibit storing, logging, or transmitting any recipient contact data, citing the ShopLoop data-processing agreement annex 2 and DS-002. The `WindowChange` entity has no such column either. This is a self-contradiction inside one seat's own package and, on the stated source, a breach of the processing agreement — second blocker. Remedy is straightforward: drop both fields and let the carrier resolve contact details from `carrier_ref`/its own record, which is where C-004 says contact data lives.
2. `WindowChange.shipment_id` is declared `FK → Shipment; cascade delete`, while C-005/NFR-005 require every change to be retained 24 months for consumer-protection record-keeping. Deleting a shipment silently destroys compliance records ahead of retention. Flag as a design defect with a concrete fix (restrict/nullify the FK, or copy the audit rows out of the cascade path).

**Refusal note:** I would not soften or "interpret around" the contact-data finding to keep the contract intact; it is stated as a legal constraint with a named source.

---

## Phase 5 — Architecture conformance against the spine

**Do:** Check each decision against the spine excerpt, which the manifest supplies precisely as the conformance baseline.

- **D-005 (new Redis Streams bus + `window-writer` consumer, handler blocks on the round trip).** The spine says `api → db` is synchronous SQL, one transaction per request. D-005 replaces an ordinary in-transaction write with a distributed handshake that the handler then waits on anyway — so it buys no asynchrony, adds two hops of latency, introduces a single-worker single point of failure, and separates the write from the request transaction, which undermines D-004's optimistic-locking claim (the `window_version` check now lands in the worker, not in the transaction that validated it). The stated rationale is "an event log for later consumers," but the append-only `window_changes` table already is that log. Recommendation: write in the request transaction; drop the new bus. This is a design finding, not a blocker on its own — but it compounds Phase 3.
- **D-006 (new ~600-line `window_scheduler`: schedule table, 30 s poll, advisory-lock leader election, retry ledger).** AX-004 in the spine is a decided, built ruling: Celery beat for periodic work, `apply_async(eta=…)` for one-off timed sends, with FEAT-006 reminders already revoked and re-queued when the order changes. That is exactly the re-timing capability D-006 claims it needs to build from scratch. Drift is currently recorded as "none." The package neither follows the ruling nor files a drift/change request. Finding: either use the built mechanism or raise an explicit exception with justification.
- **IP-001 ("nothing new").** Contradicted by D-005, which introduces a new stream, a new consumer group, and a new worker process onto the Redis instance that currently serves as Celery broker and result backend — with no stream trimming/`maxlen`, no dead-letter path, and no note on memory or persistence semantics for a broker Redis. Finding: the provisioning claim is inaccurate as written.

---

## Phase 6 — Contract and state-machine robustness

**Do:** Read `api.yaml` as a spec-validity and completeness check, and walk the state machine for dead ends.

Contract items I expect to raise:
- `/shipments/{id}/windows` and `/shipments/{id}/window` use a path template `{id}` but declare no `parameters` entry for it — invalid under OpenAPI 3.1, independent of any design question.
- No `security` scheme anywhere, though `recipient_token` is described as the sole recipient identifier; and no tenant scoping in the paths despite `tenant_id` on `Shipment`.
- `PUT` carries no idempotency key while D-002 retries three times against the carrier — a retry after a carrier-side success can double-book. Given C-002 this matters more than usual.
- `409` is spent on the cut-off (FR-004), leaving D-004's optimistic-lock conflict without a distinct status (`412` is the natural home). No `401/403/404/422`; the `409`/`502` responses reference Problem Details but declare no media type or schema. `GET` has no error responses at all.
- `Window` doesn't require `end > start` or state timezone handling; `Shipment` stores two timestamp columns while `WindowChange` uses `tstzrange` — inconsistent representations of the same concept.
- Nothing re-validates an offered window between the `GET` (FR-001) and the `PUT`; offers can go stale.

State machine items:
- No exit from `locked` — no delivered/returned terminal state.
- No transition for a carrier call that times out or errors: `change_pending` has no failure edge, though `502` exists in the contract.
- No transition for a late carrier confirmation arriving after the handler gave up.
- No transition representing the depot-loading lock from C-001 (Phase 2).
- FR-006 re-timing has no state or field to hang off, and no rule for shipments whose window is less than 24 hours away, where a 24-hour reminder is unreachable.

Measurability item: FR-004 refusals never produce a `WindowChange` row, so SC-002's denominator ("attempts made before the cut-off") is not derivable from the change log it says it will be measured from.

---

## Phase 7 — Questions to the `analyst-1` seat

The manifest says that seat is reachable until the review closes. I would send one consolidated question list rather than a drip of messages, and I would send it *after* Phases 2–6 so the list is complete. This is a message to a reachable seat, not work handed to a worker — I would not spawn anything.

Questions, each with what I would accept as a resolving answer:
1. How is US-002/FR-002 meant to work given C-001's 05:00–06:00 loading lock? Accepting answer: a named mechanism in the package. If the answer is "it isn't," the blocker stands and goes to the user.
2. Is `recipient_email` in `api.yaml` an oversight or a deliberate exception to C-004? Accepting answer: written confirmation it is removed, or an annex-2 exception reference. I would treat "the carrier needs it" as insufficient — C-004 covers transmission too.
3. What p95 does D-003 + D-005 actually target, given C-003? Accepting answer: a revised NFR-002 figure with arithmetic.
4. What does D-005 provide that an in-transaction write does not, and how does D-004's lock hold across the handoff?
5. Why does D-006 rebuild what AX-004 rules is already built, and has a drift entry been raised?
6. Is the cascade delete on `WindowChange` intended, given the 24-month retention rule?

I would check any answers against the cited sources — but note plainly in the report that `carrier master agreement §4.3`, the 2026-08-19 refusal letter, `ops/carrier-latency-2026-Q3.md`, and the DPA annex 2 are **not in this workspace**. My feasibility verdict on Phases 2–4 rests on those citations being accurate; I would mark them "asserted by the package, not independently verified here" rather than imply I confirmed them.

---

## Phase 8 — Delegation

None. The package is six short files and the analysis is cross-referential — handing slices to separate workers would lose exactly the cross-file contradictions that constitute most of the findings (the `recipient_email` breach is only visible by reading the contract against the data model and constraints). The single outbound communication is the Phase 7 question list to `analyst-1`. If the user later wants an independent second pass, the right route is their own multi-agent review command — that is user-triggered and billed, so I would suggest it, not launch it.

---

## Phase 9 — Stop for the user's ruling

This is the point where I stop and do not proceed alone.

**What I would put in front of the user:** a recommendation of **do not sign off as-is**, with three blockers (same-day rescue infeasible under C-001; contact-data breach in the contract; latency target unachievable with confirm-in-request), three design findings (D-005, D-006/AX-004 drift, IP-001 inaccuracy), and a list of contract/state-machine gaps.

**Branches:**
- *User agrees the package goes back:* I write the findings report and the question list, and stop there — the fixes belong to the producing seats.
- *User rules that same-day rescue is dropped from scope:* the blocker becomes a spec change; I would then note that US-002, FR-002's "including the delivery day" clause, and SC-002 all need editing together, and that the cut-off in FR-004 may need to become "before depot loading," not two hours.
- *User rules the package is signed off anyway (e.g. schedule pressure):* that is their decision and I would proceed, but I would ask that the three blockers be recorded as accepted risks with owners, since two of them (contact data, unachievable success criterion) have compliance and measurement consequences that outlive the sign-off.
- *User is unavailable:* my default is to hold at "not signed off" and deliver the report — I would not sign off on their behalf, and I would not edit the package files to fix the defects, since this was commissioned as a review.

---

## Phase 10 — What I would write and report

**Write (only after the Phase 9 ruling, or immediately if the user just wants the report):**
- `review/FEAT-011-feasibility-review.md` — verdict, blockers with cited line references, design findings, contract/state-machine gaps, trace matrix, and the explicit note about unverifiable external sources.
- `review/FEAT-011-questions-for-analyst-1.md` — the Phase 7 list.

No changes to `spec.md`, `data-model.md`, `contracts/api.yaml`, or `constraints-and-decisions.md`; no changes to the spine excerpt, which the manifest places outside the review.

**Report to the user:** the recommendation not to sign off, the three blockers in one line each with the file and line that shows them, the fact that the same-day-rescue conflict spans two seats and needs a scope decision rather than an analyst fix, the two architecture findings against the built spine, and a clear statement that the carrier, latency, and data-protection source documents were taken at their stated value because they are not present in this workspace.