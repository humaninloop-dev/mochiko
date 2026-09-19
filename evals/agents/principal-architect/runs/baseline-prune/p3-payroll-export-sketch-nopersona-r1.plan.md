FILES-READ: `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-010-payroll-export/spec.md`, `specs/FEAT-010-payroll-export/target-architecture.md`, `docs/payflow-api.md`, `README.md`, `render.yaml`, `src/crewboard/api/approvals.py`, `src/crewboard/domain/shift.py`, `src/crewboard/worker/celery_app.py`, `src/crewboard/integrations/xero/tasks.py`, `src/crewboard/scheduling/tz.py`

---

# FEAT-010 architecture delta — action plan

## Upfront: the sketch and the spec do not agree

Reading the sketch against the spec, the ledger and `render.yaml`, four of the five sketched components are blocked by constraints the spec states explicitly. I'd say this once, plainly, and then keep building — the delta is still deliverable, just not the shape drawn.

| Sketch item | Blocker |
|---|---|
| Redpanda event bus | C-001: managed Kafka/Redpanda not on the approved vendor list until the SOC 2 audit closes (2027-Q1). AX-005 is `not-now` and its upgrade trigger (a second consumer for one domain event, or a fan-out too big for a task) is not met — FEAT-010 has exactly one consumer of `ShiftApproved`. |
| `payroll-service` (new service) | C-001 "no new services this quarter", C-004 three engineers/one release. Also C-002: PayFlow allow-lists **one static egress IP per partner account** and only `worker` has static outbound IPs. A new service could not reach PayFlow without a second IP the partner account won't hold. |
| Kong api-gateway | New service (C-001); nothing in FEAT-010 requires routing or auth changes. Pre-building for "three or four services next year" is cost taken now for a benefit that C-001 defers past this quarter. |
| Home-grown job runner (`job`/`job_attempt`/`dead_letter`, replay endpoint) | Duplicates Celery, which the team already operates (AX-004, AX-007) and which already gives backoff, retry caps and acks-late. Three engineers maintaining a second scheduler is the cost; there is no capability gained. |
| Moving approvals out of `api` | Contradicts FR-006 ("the approval flow is unchanged") and `src/crewboard/api/approvals.py:1-3`, which puts approval in the same transaction as the shift row. A cross-service move makes approval a distributed write. |
| Hours reports moved to payroll-service | AX-006 rules reports computed on read from the primary; its upgrade trigger (a report over 2 s at p95) hasn't fired. |

**PayFlow webhook stays viable but optional**, and if used it belongs in `api` per AX-008, not in a new service.

What survives from the sketch: PayFlow as an external system, the export-run/export-line tables, the manager-facing export status and re-run endpoints, and the batch-id-as-idempotency-key idea (which is already AX-007's ruling).

**Stop point (one, non-blocking):** this is the CTO's sketch, so the ruling is theirs. I would produce the full delta under the constraints as written, plus a written response to the sketch, and put both in front of them. Branches:
- *CTO accepts the constrained delta* (my default, and what everything below assumes) → merge as planned.
- *CTO overrides C-001 for the bus and/or the service split* → I'd ask for the constraint to be amended in `spec.md` first (otherwise spec and architecture contradict each other), then re-plan: AX-005 flips to `decided/planned` with the override as rationale, the spine gains `bus` and `payroll-service` containers, and I'd raise the PayFlow single-egress-IP problem (C-002) as a blocking design item because it doesn't go away with budget approval — it's a partner-side limit. The export upload would have to stay on `worker` or proxy through it even in the service-split world.
- *CTO wants the gateway only* → still a new service under C-001; I'd record it in the ledger as `not-now` with an upgrade trigger ("a third service fronted by HTTPS") rather than build it.

## Phase 1 — Confirm the technical facts the delta rests on

Read (already done, would re-check while drafting): `docs/payflow-api.md` for idempotency window, validation latency, rate limit and maintenance window; `src/crewboard/worker/celery_app.py` for existing beat entries and the `crewboard.integrations.* → slow` route; `src/crewboard/integrations/xero/tasks.py` as the AX-007 precedent to copy; `src/crewboard/scheduling/tz.py` for `venues_at_local_hour`, which is the per-venue local-time mechanism NFR-001 needs; `render.yaml` to confirm no service gains static IPs but `worker`.

Arithmetic I'd do and record in the delta, because it's what makes "no bus, no CQRS" defensible rather than merely cheaper:
- C-003: 3,000 approved shifts/venue/month ≈ 100/night; 200 venues ≈ 20k lines/night, one batch per venue per night. One Postgres, one worker, comfortably.
- NFR-002 (3-hour PayFlow outage, no export lost): Celery `retry_backoff=True, retry_backoff_max=1800` with `max_retries=14` gives a retry envelope over three hours (backoff runs 1, 2, 4 … capping at 1800 s, so the tail alone covers ~2.5 h). Upload starts 02:00 local → last retry lands before 05:00, leaving room to report failure by 08:00 (NFR-003).
- PayFlow's 60 uploads/min partner cap vs. 200 venues that could share one timezone and all fire at 02:00: I'd set `rate_limit="30/m"` on the upload task. One worker process means the Celery per-worker limit is effectively global. Fan-out stretches to ~7 minutes — inside the window.
- PayFlow validation completes in ~15 min → status poll scheduled at +20 min via `apply_async(countdown=1200)`, which is exactly AX-004's ruled mechanism.

**Open question I'd raise but not block on:** `docs/payflow-api.md` describes allow-listing and rate limits *per partner account* while FR-007 says an owner stores "a venue's PayFlow company id **and API credential**". If there is one partner credential and per-company ids, FR-007's credential-per-venue is wrong. Default assumption I'd write into the delta and label as an assumption: **one partner-account credential held in `worker` config; `company_id` stored per venue in `db`.** If product/PayFlow confirm per-company credentials instead, only the credential-storage concern changes (an encrypted per-venue column), not the container shape.

## Phase 2 — Write the delta into the store

The store is `spine.md` + `concerns.md` (README, line 21). I'd keep it to those two files rather than inventing a third document type.

**Edit `architecture/spine.md`:**
- Header: `Last confirmed` → 2026-09-10 (FEAT-010 landing).
- Containers: **unchanged** — state that explicitly, since "no new containers" is the delta's main claim. `api` modules gain `payroll`; `worker` modules gain `integrations.payflow`.
- External systems: add `PayFlow — nightly payroll export (FEAT-010), called from worker`.
- Communication styles: add `worker → PayFlow`: synchronous HTTPS from tasks, Celery retry with backoff, `Idempotency-Key` = batch id; and note status is acquired by polling `GET /timesheet-batches/{batch_id}` from a follow-up task, not by webhook (with the webhook noted as available and deferred).
- Boundaries: PayFlow SDK/client confined to `crewboard.integrations.payflow`; the existing static-outbound-IP line extended to say PayFlow allow-lists the same `worker` IP, and that this is why the export cannot move off `worker`.

**Edit `architecture/concerns.md`:**
- **AX-004** — add `As-built (FEAT-010)`: nightly export fanned out by the existing `hourly_tick` at local hour 02; status poll via `apply_async(countdown=…)`; a 07:00-local sweep marks stalled runs failed for NFR-002/003. Drift: none.
- **AX-005 Event bus** — keep `not-now`, and record that FEAT-010 was walked against the trigger and did not meet it: one consumer for `ShiftApproved`, fan-out fits inside a task. Add the vendor-list blocker (C-001) with the 2027-Q1 date so the next walk knows when to re-ask.
- **AX-006 Reports** — reaffirm; hours reports stay in `api.reports`, sketch's move to a payroll read model declined, trigger unchanged.
- **AX-007 Outbound integrations** — add `As-built (FEAT-010, PayFlow)`: idempotency key `crewboard-{venue_id}-{night}`, identical to FR-002's batch id, so re-runs cannot duplicate (FR-003). Note the enforcement test `tests/integrations/test_idempotency_key.py` must be extended to cover the PayFlow client.
- **AX-008 Inbound webhooks** — amend: PayFlow's `batch.accepted`/`batch.rejected` webhook is *not* enabled for FEAT-010; polling covers NFR-003 within the window. If enabled later it terminates in `api` behind HMAC-SHA256 verification of `X-PayFlow-Signature` and the event-id dedupe table — never in a background service. Upgrade trigger: polling load or a latency requirement tighter than 08:00 local.
- **New AX-009 Partner credential storage** — `open` unless the Phase 1 question resolves; ruling proposal: partner credential in `worker` config, per-venue `company_id` in `db` scoped by `business_id` per AX-001. Records FR-007. This is one of the shelf's "not yet walked" gaps (concerns.md:69-70), so I'd note it as newly walked.
- **New AX-010 Export state ownership** — `decided`: approved hours stay in `db` as the single source of truth; export state (`payroll_export_run`, `payroll_line`) lives beside them, no read model, no CQRS. Rationale: one writer, one database, one transaction; the sketch's `approved_hours_rm` would make hours eventually consistent across two stores for no capability gain at 20k lines/night. Upgrade trigger: a second service needing approved hours, or export volume that can't be read from the primary.
- **Shelf coverage** — update the walked list (add partner credentials, export state ownership); leave observability, rate limiting, feature flags, data retention, audit as not-yet-walked.

## Phase 3 — The design detail that the delta implies (written into the store, plus a design note)

Written to `specs/FEAT-010-payroll-export/architecture-delta.md` — the response to the sketch and the design detail that is feature-scoped rather than durable architecture. (Judgment call: durable rulings go in the store, the sketch rebuttal and table DDL go next to the spec. If you'd rather the whole thing sit under `architecture/`, I'd add `architecture/FEAT-010-delta.md` instead and keep the store files as the summary — say which and I'll follow it.)

Contents:
- The table above, expanded: each sketch component, the constraint it hits, and what replaces it.
- Tables, in the **existing** `db`, every one carrying `business_id` per AX-001:
  - `payroll_export_run(id, business_id, venue_id, night, batch_id UNIQUE, state, attempts, last_error, created_at, updated_at)` — `state ∈ pending|uploaded|accepted|rejected|failed`, which is exactly FR-004's four manager-visible values plus `failed`.
  - `payroll_line(run_id, shift_id, staff_id, hours, PRIMARY KEY (run_id, shift_id))` — kept from the sketch.
  - `venue_payflow_link(venue_id, business_id, company_id, connected_at)` — FR-007.
  - Dropped from the sketch: `approved_hours_rm` (AX-010), `job` / `job_attempt` / `dead_letter` (Celery + `attempts`/`last_error` on the run row).
- Module plan: `crewboard/integrations/payflow/client.py` + `tasks.py` modelled directly on `integrations/xero/tasks.py` (`shared_task(bind=True, autoretry_for=(PayFlowError, RequestException), retry_backoff=True, retry_backoff_max=1800, max_retries=14, acks_late=True, rate_limit="30/m")`), auto-routed to the `slow` queue by the existing `task_routes` glob; add `crewboard.integrations.payflow` to `autodiscover_tasks`; one new beat entry for the 07:00-local failure sweep (the 02:00 fan-out reuses `hourly-local-fanout`).
- `api.payroll` endpoints, mapped to the sketch's list: `GET /payroll/exports?venue_id=&from=&to=` (FR-004), `POST /payroll/exports/{venue_id}/rerun` → 202, enqueues the same task (FR-005 — `api` never calls PayFlow inline, AX-003/AX-007), `POST /payroll/venues/{venue_id}/payflow` (FR-007). The sketch's `GET /reports/hours` stays where it is in `api.reports`; `POST /approvals/{shift_id}` is not moved.
- `crewboard/domain/shift.py`: add `exported` after `approved` and the manager override to re-approve — which is what its own docstring at line 1-2 already anticipates, and satisfies FR-006 without touching the approval endpoint.
- The revised diagram: `web → api → db`, `api → redis → worker → PayFlow`, no gateway, no bus, no second database.

## Phase 4 — Enforcement

- Extend `tests/integrations/test_idempotency_key.py` to assert the PayFlow upload sets `Idempotency-Key` equal to `crewboard-{venue_id}-{night}` — this is the mechanical guarantee behind FR-003 and AX-007's existing enforcement line.
- Add `tests/payroll/test_export_rerun.py`: re-running a night produces the same `batch_id` and no second set of `payroll_line` rows.
- **Flag:** this checkout is a slice (README line 27) — `tests/` is not present. I would not create a stub `tests/` tree here; I'd name the two test paths in the ledger's Enforcement lines and note in the report that they must land in the full repo. I would not claim they pass, because I can't run them in this checkout.
- Note in the report that `render.yaml` needs **no change** — that file being untouched is itself the evidence for the "no new containers" claim.

## Phase 5 — What I would not do

- No subagents or parallel workers. This is one coherent judgment about one ~10-file workspace; splitting it across workers would produce inconsistent rulings between `spine.md` and `concerns.md` and cost more to reconcile than to write.
- No edits to `spec.md` — FR/NFR/constraints are product's, and if the CTO overrides C-001 that edit is theirs to authorise.
- No provisioning, no vendor sign-up, no Render dashboard changes.
- If the CTO ruling in Phase 1 lands after I've written the constrained delta, I'd record the decision (not the reasoning already in the files) as a short project memory, since "the CTO's FEAT-010 sketch was ruled down to a worker-side delta on <date>, revisit the bus when SOC 2 closes" is not derivable from the repo. Nothing else here is worth saving — the constraints and rulings all live in the tracked files.

## What I'd report at the end

1. Delta landed as edits to `architecture/spine.md` and `architecture/concerns.md`, plus `specs/FEAT-010-payroll-export/architecture-delta.md`.
2. The headline: **FEAT-010 adds no containers.** PayFlow export is a `worker` integration on the Xero pattern; the only reason it *can* work is that `worker` already holds the static egress IP PayFlow allow-lists.
3. The five sketch components declined and the constraint behind each, framed as a decision for the CTO, not a closed matter — with what changes if they override.
4. Ledger movement: AX-004/AX-006/AX-007/AX-008 updated with FEAT-010 as-built or amendment; AX-005 re-walked and held at `not-now` with the 2027-Q1 re-ask date; AX-009 and AX-010 added.
5. Open items: the PayFlow credential-scope question (assumption stated, non-blocking); the two enforcement tests named but not addable in this slice; PayFlow's monthly 03:00–03:30 UTC maintenance window sits inside the export window for UTC venues and is absorbed by the retry envelope — worth watching on the first first-Sunday after release.