FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-010-payroll-export/spec.md`, `specs/FEAT-010-payroll-export/target-architecture.md`, `docs/payflow-api.md`, `render.yaml`, `src/crewboard/api/approvals.py`, `src/crewboard/domain/shift.py`, `src/crewboard/worker/celery_app.py`, `src/crewboard/integrations/xero/tasks.py`, `src/crewboard/scheduling/tz.py`

---

# Action plan — FEAT-010 architecture delta

## Headline finding I'd surface before writing anything

The sketch and the spec's own constraints are in direct conflict on four of the five components. I'd still produce a complete delta — but the delta that lands in `architecture/` is the constraint-respecting one, with the sketch's divergences recorded explicitly rather than silently dropped.

| Sketch item | Conflict |
|---|---|
| Redpanda event bus | C-001 (Redpanda not on approved vendor list until SOC 2 closes, 2027-Q1) and AX-005, whose upgrade trigger is "a second consumer for the same domain event, or a fan-out one task can't finish" — FEAT-010 adds exactly one consumer that finishes in a task |
| payroll-service (new Render service) | C-001 "no new services this quarter"; C-004 three engineers, one release; and hard-blocked by C-002 — PayFlow allow-lists **one** static egress IP per partner account, and that IP is `worker`'s |
| api-gateway (Kong) | C-001 again; no requirement in the spec asks for it |
| Home-grown job/job_attempt/dead_letter runner | Re-implements what Celery already does under AX-004 and AX-007, which have been in production since April |
| Approvals moved out of `api` | FR-006 says the approval flow is unchanged; `src/crewboard/api/approvals.py:1-3` documents it as a rota edit in the same transaction as the shift row. Moving it splits that transaction across a service boundary for no requirement |
| Hours reports in payroll-service | AX-006 rules reports computed on read in `api.reports`; its trigger (a report over 2s p95) isn't claimed |
| Webhooks landing in payroll-service | AX-008 rules inbound callbacks terminate in `api` behind signature verification + event-id dedupe |

C-002 is the one that isn't a matter of taste: a new service can't make the PayFlow upload at all without PayFlow changing the partner account's single allow-listed IP. The upload has to run from `worker`.

---

## Phase 1 — Confirm the divergence with the CTO (the stop)

**What I'd confirm:** that the delta should be written against the spec's constraints (C-001–C-004) rather than the sketch, i.e. no Redpanda, no new Render service, no Kong, no home-grown job runner, approvals staying in `api`.

**What I'd put in front of them:** the table above, plus the specific point that PayFlow allows one egress IP per partner account and `worker` holds it (`render.yaml:15`, `docs/payflow-api.md:11`).

**Branches:**
- *CTO agrees* → Phases 2–6 as written.
- *CTO says a constraint is being lifted* (vendor approval or budget landed early) → I'd ask for the date it lifts, then still ship FEAT-010 on `worker` for this release and record AX-005 as `not-now` with a revised review date, since C-004 (one release, three engineers) doesn't change. The bus becomes a separate future delta with its own trigger.
- *CTO overrides and wants the sketch as drawn* → I'd write it up as ruled-with-known-conflict: each concern gets the sketch's ruling, an explicit "conflicts with C-00x" line, and the PayFlow IP problem flagged as a prerequisite that must be resolved with PayFlow before any code ships. I'd still not silently mark it as-built.

**Default if no answer:** proceed with the constraint-respecting delta and hand back the divergence note.

**No delegation.** This is a small, tightly-coupled read-and-write task over ~11 files; splitting it across workers would cost more in briefing than it saves, and the judgement calls are the whole job.

---

## Phase 2 — Settle the target shape (no files written)

Decide the design the delta describes, driven by each requirement:

- **Upload path** — new module `crewboard.integrations.payflow` in `worker`, mirroring `integrations/xero/tasks.py`: one Celery task per venue per night, `autoretry_for` on PayFlow/transport errors, `Idempotency-Key: crewboard-{venue_id}-{night}` (FR-002, FR-003, AX-007, and already covered by `tests/integrations/test_idempotency_key.py`). Task routed to the `slow` queue by the existing `crewboard.integrations.*` rule (`celery_app.py:13`).
- **Scheduling (NFR-001)** — reuse `crewboard.scheduling.tz.venues_at_local_hour` via the existing `hourly-local-fanout` beat entry; fan out at local 02:00. No new scheduler (AX-004).
- **Retry budget (NFR-002)** — this needs sizing, not hand-waving. Xero's settings (`max_retries=8`, `retry_backoff_max=1800`) cover well under three hours. For PayFlow I'd specify `retry_backoff_max=1800` with `max_retries` around 15, which spans ~3.5h from a 02:00 start, plus a local-07:00 sweep that marks anything not accepted as failed and notifies the manager (AX-003, from `worker`) so the 08:00 deadline is met either way.
- **Result path (NFR-003, FR-004)** — PayFlow's webhook is *optional and per-company* (`docs/payflow-api.md:9`), so correctness can't depend on it. Primary: a status poll task ~20 minutes after upload (validation completes in ~15 min) plus the 07:00 sweep — both from `worker`, since the status GET is IP-allow-listed too. Webhook, where enabled, is an accelerator only, and terminates in `api` at `POST /webhooks/payflow` with HMAC-SHA256 verification and an event-id dedupe table, per AX-008 and the Stripe precedent.
- **Manager-facing endpoints** — in `api`: a status read for the manager's venues and a re-run trigger (FR-005) that enqueues the same idempotent task. Hours reports stay in `api.reports` (AX-006).
- **Data** — in the existing `db`, all carrying `business_id` for AX-001: `payroll_export_run` (venue, night, batch_id unique, state, attempts, last_error), `payroll_line` (run_id, shift_id, staff_id, hours), `venue_payflow_link` (company id + credential reference, FR-007), `payflow_webhook_event` (dedupe). No `approved_hours_rm` — there's no second database to keep a read model for.
- **Domain** — `Shift.STATES` gains `exported` after `approved`, with re-approval blocked absent a manager override (FR-006, and `src/crewboard/domain/shift.py:1-2` already anticipates this).
- **Rate limit** — 60 uploads/min per partner account; with 200 venues fanned out by local hour and `worker_concurrency=4` there's headroom, but I'd note a task-level rate limit as the guard.

---

## Phase 3 — Write the spine delta

**File:** `architecture/spine.md` (edited in place; the store is canonical, so the delta lands *in* it).

- Bump **Last confirmed** to 2026-09-10 (FEAT-010 landing).
- **Containers**: unchanged — explicitly state that FEAT-010 adds no container, and add `integrations.payflow` to `worker`'s module list; add the payroll-export read/re-run endpoints and the PayFlow webhook to `api`'s.
- **External systems**: add PayFlow — nightly payroll export (FEAT-010), called from `worker`.
- **Communication styles**: `worker → PayFlow` synchronous HTTPS, one task per venue per night, Celery backoff, idempotency key per batch; `PayFlow → api` signed webhooks (optional per company, accelerator only); status polling from `worker`.
- **Boundaries**: extend the static-outbound-IP line to note PayFlow allow-lists the same `worker` IP and permits only one per partner account — which is why outbound payroll traffic cannot originate from `api` or any new service.

## Phase 4 — Write the concern-ledger delta

**File:** `architecture/concerns.md` (edited in place).

- **AX-005 Event bus** — keep `not-now`, add a 2026-09-10 re-walk: FEAT-010 was assessed against the trigger and does not meet it (one consumer; the fan-out is 200 per-venue tasks that finish inside their window). Record C-001's vendor block and the 2027-Q1 review date. This is where the sketch's bus is answered on the record rather than ignored.
- **AX-006 Reports** — reaffirmed; hours reports stay in `api.reports`; note the sketch proposed moving them and the 2s p95 trigger is unmet.
- **AX-007 Outbound integrations** — add **As-built (FEAT-010, PayFlow): as ruled · Drift: none**, with the batch-id-as-idempotency-key detail and the sized retry budget for NFR-002. Note the existing enforcement test covers the new call.
- **AX-008 Inbound webhooks** — add **As-built (FEAT-010, PayFlow)**: terminates in `api`, HMAC-SHA256 verified, deduped by event id; note the webhook is optional per company so the status poll is the guarantee behind NFR-003.
- **AX-004 Scheduled work** — add FEAT-010 as-built: per-venue local-time fan-out at 02:00 through the existing hourly tick.
- **New AX-009 Partner credentials at rest** — FR-007 introduces a per-venue PayFlow credential, which nothing in the ledger currently covers. Stance `decided`, ruling: stored as a secret reference, never in application tables in plaintext, one per venue under AX-001 scoping. If I can't establish the existing secret-handling practice from this checkout, I'd mark it `open` with a named question rather than invent a ruling.
- **New AX-010 Export/approval lifecycle** — the `exported` shift state and the manager-override rule (FR-006), so the state machine change is on the record.
- **Shelf coverage** — add credentials/secrets to the walked list; leave observability, rate limiting, feature flags, retention, audit as not-yet-walked.

## Phase 5 — Record the sketch reconciliation

**File:** `specs/FEAT-010-payroll-export/architecture-delta-notes.md` (new).

A short document for the CTO: each of the five sketched components, the ruling taken, and the reason — constraint reference or unmet trigger, not opinion. It also carries the two things worth keeping from the sketch as future triggers: when a second consumer of `ShiftApproved` appears, AX-005 reopens; when a third backend service is genuinely funded, the gateway question reopens. I'd keep this in the spec folder rather than `architecture/`, so the store stays free of per-feature argumentation.

I would **not** edit `src/`, `render.yaml`, or the spec in this task — the card asks for the architecture delta, and the code changes are the implementation release that follows.

## Phase 6 — Self-check before handing back

No test suite to run here (this is a documentation delta, and the checkout omits the tests the ledger cites). Instead I'd verify by re-reading:

- every FR and NFR in the spec maps to something in the delta — I'd expect FR-001…FR-007 and NFR-001…NFR-003 all to land, with NFR-002 the one to scrutinise because it depends on the retry budget I sized;
- every constraint C-001…C-004 is satisfied by the delta as written;
- every sketch component has an explicit disposition, none silently dropped;
- `spine.md` and `concerns.md` don't contradict each other on where the webhook terminates or where reports are computed.

**What I'd report:** the delta is written to `architecture/spine.md` and `architecture/concerns.md` with a reconciliation note in the spec folder; FEAT-010 lands with no new containers and no new vendors; four of the sketch's five components were ruled against on stated constraints, with AX-005's re-walk and the PayFlow single-IP finding called out as the two the CTO most needs to see; AX-009 (credential storage) flagged as either newly ruled or left open pending an answer I couldn't get from this checkout; and the implementation work — `crewboard.integrations.payflow`, the `exported` shift state, the migrations, the `api` endpoints — named as the next release, not done here.