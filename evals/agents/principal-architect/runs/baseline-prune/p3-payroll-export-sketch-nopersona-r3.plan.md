FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-010-payroll-export/spec.md`, `specs/FEAT-010-payroll-export/target-architecture.md`, `docs/payflow-api.md`, `src/crewboard/api/approvals.py`, `src/crewboard/domain/shift.py`, `src/crewboard/worker/celery_app.py`, `src/crewboard/integrations/xero/tasks.py`, `src/crewboard/scheduling/tz.py`, `render.yaml`

---

# Action plan — FEAT-010 architecture delta

## Framing

The deliverable is architecture-store content, not FEAT-010 code. I would not touch `src/`, `render.yaml`, or write implementation tests in this piece of work; the ledger entries name the enforcement tests that implementation must add later.

The reading above already surfaces the central tension: the sketch proposes five things, and four of them collide with constraints written into the same spec the sketch sits next to. I would not quietly shrink the sketch, and I would not write up a design I think is wrong. The plan below produces the delta the constraints support, plus an explicit sketch-vs-delta reconciliation so the CTO can overrule me on the record.

---

## Phase 1 — Test each sketch component against the constraints

No files written. Reasoning I would complete before drafting, with the evidence I'd cite in the delta:

1. **Redpanda event bus.** C-001 forbids new services this quarter and puts managed Kafka/Redpanda off the vendor list until the SOC 2 audit closes (2027-Q1). Independently, AX-005 is already a ruled *not-now* with a stated upgrade trigger: a second consumer for the same domain event, or a fan-out too big for one task. FEAT-010 adds exactly one consumer of `ShiftApproved`. The trigger is not met and the vendor is not available. → **Not in this delta.** AX-005 stays not-now, restated with FEAT-010 as evidence.
2. **payroll-service (new Python service, own database).** C-001 (no new services), C-004 (three engineers, one release). C-002 is the harder one: PayFlow allow-lists one static egress IP per partner account, Render static IPs are per service, and `worker` is the only service that has them. A new service cannot reach PayFlow without a second allow-listed IP the partner account does not offer. → **Not in this delta.** Export runs from `worker`.
3. **Moving approvals out of `api`.** FR-006 says the approval flow is unchanged. `src/crewboard/api/approvals.py:1-3` records that it lives in `api` deliberately — it is a rota edit with a role check in the same transaction as the shift row, and `Shift.approve` (`src/crewboard/domain/shift.py:10`) is the state machine. Moving it across a service boundary makes approval a distributed write and breaks the AX-001 tenant-session scoping. → **Not in this delta.**
4. **Hours reports moved to payroll-service.** AX-006 rules reports computed on read in `api.reports`, upgrade trigger "a report query above 2 s at p95". No evidence that trigger is met; C-003 caps volume at 3,000 shifts per venue per month. → **Not in this delta.**
5. **Home-grown job runner (`job`, `job_attempt`, `dead_letter`).** Duplicates Celery, which AX-004 and AX-007 already rule for exactly this shape of work, and which the team already operates. → **Not in this delta.** But the sketch is half-right: `payroll_export_run` is genuinely needed, because FR-004/FR-005 require durable per-venue state (pending/uploaded/accepted/rejected, rejection reason, re-run) and Celery's result backend is not a durable record. I keep the run/line tables and drop the generic job machinery.
6. **Kong api-gateway.** C-001; and with one backend service there is no routing or auth to centralise. → **Not in this delta**, recorded as a new ledger concern with an upgrade trigger so it is picked up rather than forgotten.
7. **PayFlow webhooks on payroll-service `/webhooks/payflow`.** AX-008 rules inbound third-party callbacks terminate in `api` behind signature verification and an event-id dedupe table. PayFlow's webhook is HMAC-SHA256 in `X-PayFlow-Signature`, retried 24 h — fits AX-008 unchanged. → **Endpoint moves to `api`**, if we take webhooks at all (Phase 3, decision B).

**What survives from the sketch:** the export-run/line state tables, the PayFlow batch-id idempotency scheme (FR-002, already the AX-007 pattern), the manager-facing endpoints, and the strategic intent. I would keep the intent alive by scoping the new code as its own package (`crewboard.payroll` for state and endpoints, `crewboard.integrations.payflow` for the client and tasks) with no cross-imports from `rotas`/`approvals` beyond the domain event, so extraction into a service in 2027 is a packaging change rather than a rewrite. That is the honest version of "ready for the next few services" under this quarter's constraints.

## Phase 2 — Findings the sketch does not cover, and that I would resolve in the delta

These come out of the code and the PayFlow notes; each becomes a stated design point with a number in it.

- **Retry budget does not currently reach NFR-002.** The Xero task (`src/crewboard/integrations/xero/tasks.py:11-12`) uses `retry_backoff=True, retry_backoff_max=1800, max_retries=8` — delays 1,2,4…128 s, roughly four minutes total. NFR-002 requires surviving a three-hour PayFlow outage. With the same backoff shape, covering 10,800 s needs about 16 retries (1…1024 sums to ~2,047 s, then five more at the 1,800 s cap). I would specify `max_retries=18` **and** a hard wall-clock deadline in the task: past 07:00 venue-local, stop retrying and write `failed` with the last error, so NFR-002's "reported failed by 08:00 local" and NFR-003 hold. The deadline is the actual guarantee — Celery's default `retry_jitter` randomises delays downward, so a retry count alone does not bound elapsed time.
- **Rate limit versus fan-out.** PayFlow allows 60 uploads/min per partner account. Up to 200 venues (C-003), and hospitality venues in one group largely share a timezone, so local 02:00 is a single burst. I would specify a Celery `rate_limit` on the upload task (30/m leaves headroom and drains 200 venues in ~7 minutes, far inside the 02:00–05:00 window) and route it to the existing `slow` queue alongside Xero.
- **DST breaks exact-hour selection.** `venues_at_local_hour` (`src/crewboard/scheduling/tz.py:12-14`) matches `local_now(v).hour == hour`. On spring-forward the local clock goes 01:59 → 03:00, so a venue selected at local 02:00 is never selected and that night's export silently does not happen — a direct NFR-001 failure. Ruling: select on state, not on the exact hour — each hourly tick between local 02:00 and 04:00 picks venues with no `payroll_export_run` row for tonight and creates one, with the run row's unique `batch_id` making a double tick a no-op.
- **Tenancy.** Every table in the sketch carries `venue_id` but no `business_id`. AX-001 requires `business_id` on every tenant table, scoped by `TenantRepository`, enforced by `tests/tenancy/test_isolation.py` (cited by the ledger; the README says it exists in the full repo and is simply not in this slice — I would not treat its absence as removal). The delta specifies `business_id` on `payroll_export_run`, `payroll_line`, and the venue↔PayFlow connection table.
- **Partner credentials (FR-007).** A per-venue PayFlow company id and API credential is new: crewboard has not previously stored per-tenant third-party secrets. The ledger's "not yet walked" list does not include secrets at all. I would open a new concern rather than let it land undecided, ruling encryption at rest with the key from Render env config, credentials readable only by `worker`, never returned by any `api` endpoint.
- **Egress IP count.** Render issues a *set* of static outbound IPs per service; C-002 and the PayFlow notes both say one IP per partner account. This needs confirming with PayFlow before implementation — see Phase 3, decision C.
- **No new read model, no new database.** At ~100 approved shifts per venue per night (C-003), the export reads `shift` rows directly through the tenant session; `approved_hours_rm` and `payroll-db` are unnecessary and would create a consistency problem the current design does not have.

## Phase 3 — Decision points I would stop on

I would draft the whole delta first so the stop is "review this document", not "wait before I start".

**A. Overruling four of the five sketch components.** This is the CTO's own sketch and the disagreement is substantive, so I would not land it silently. I would present the delta with a sketch-versus-delta table, one line of evidence per row. *Branches:* if the ruling is "constraints win", the delta lands as drafted. If it is "we want the bus and the service anyway", then C-001/C-002 must move first — the delta becomes a staged plan whose first gate is the SOC 2 audit and a second PayFlow-allow-listed egress path, FEAT-010 still ships from `worker` in the meantime, and I add a written migration path rather than blocking the feature. If it is "Kong only", I would say the routing problem does not exist yet with one service and ask for the concrete driver; if there is one I have not seen, it becomes its own ledger entry. **Default while unanswered: the constrained delta.**

**B. Webhook or polling for accepted/rejected.** PayFlow's webhook is optional and enabled per company in *their* dashboard, so an owner-driven step we do not control sits between us and NFR-003. Polling `GET /timesheet-batches/{batch_id}` from a `worker` task (validation completes within 15 minutes in practice, one hour promised) meets the 08:00-local deadline with no new inbound surface and no per-company setup. **Default: polling as the mechanism of record; webhook not in this release.** *Branch:* if we want the webhook too, it terminates in `api` under AX-008 (signature verification plus event-id dedupe) as an accelerator only, with polling still the guarantee — one extra ledger line, no change to the rest of the delta.

**C. PayFlow egress IP.** Someone must confirm with PayFlow whether they will allow-list the full set of Render static IPs on `worker`, or only one. *Branch — set accepted:* nothing changes. *Branch — one only:* FEAT-010 has an unresolved network dependency and I would flag it as a release blocker, with the options (a fixed proxy hop, or a PayFlow-side exception) recorded in the delta as open rather than papered over. I cannot resolve this from the workspace; it goes in the delta as an open item with an owner.

## Phase 4 — Write the delta document

**Write:** `architecture/deltas/FEAT-010-payroll-export.md` (new; the store currently has no deltas directory, so I am establishing the pattern — if the team would rather the delta live as a flat `architecture/FEAT-010-delta.md`, that is a rename and nothing else).

Contents:
- Header: feature, date 2026-09-10, source sketch, status *proposed*.
- Container and module delta: no new containers; `api` gains a `payroll` module (manager status view FR-004, re-run trigger FR-005, venue↔PayFlow connection FR-007); `worker` gains `integrations.payflow` (nightly upload, status poll); `db` gains `payroll_export_run`, `payroll_line`, `venue_payflow_connection`, all with `business_id`; PayFlow added as an external system.
- Communication styles added: `worker → PayFlow` synchronous HTTPS from Celery tasks with backoff and `Idempotency-Key` = `crewboard-{venue_id}-{night}` (FR-002/FR-003); local-time fan-out via the existing hourly tick.
- The numbered design points from Phase 2 (retry budget and deadline, rate limit, DST-safe selection, tenancy columns, credential handling, no separate read model).
- Sketch-versus-delta reconciliation table: each of the five components, kept/deferred, and the specific constraint or ruling that decides it.
- Preserved-intent section: package boundaries that make later extraction cheap, plus the upgrade triggers that would justify the bus and the gateway.
- Open items: the PayFlow egress-IP question (C), the webhook choice (B) if not yet ruled.
- Updated diagram, honest to the delta: `web → api → db`, `api → redis → worker → PayFlow`, `worker` polling PayFlow for status.

## Phase 5 — Fold the rulings into the store

**Write:** `architecture/concerns.md` — additions and amendments, in the existing entry format (stance, status, ruling with date, rationale, enforcement, as-built, drift):
- **AX-005 Event bus** — stays *not-now*; add that FEAT-010 was assessed against the trigger and adds one consumer only, so the trigger is not met; note the vendor gate (2027-Q1).
- **AX-007 Outbound integrations** — extend to name PayFlow as the second integration on the pattern; add the retry-budget and deadline ruling and the rate-limit ruling, since the Xero settings are not sufficient here; enforcement continues to include `tests/integrations/test_idempotency_key.py`, plus a new `tests/payroll/test_export_deadline.py` that implementation must add (asserting a run past the local 07:00 deadline lands in `failed` with the last error rather than retrying).
- **AX-004 Scheduled work** — amend with the DST-safe, state-based venue selection for nightly exports; enforcement `tests/payroll/test_nightly_selection.py` (implementation to add), expected to show that a spring-forward night still produces exactly one run per venue.
- **AX-006 Reports** — unchanged, with a line recording that FEAT-010 proposed moving hours reports out and the trigger was not met.
- **AX-008 Inbound webhooks** — a line recording the FEAT-010 ruling per decision B; if webhooks are taken, the endpoint is in `api`, not elsewhere.
- **AX-009 API gateway** (new) — stance *not-now*, status *ruled*, rationale one backend service, upgrade trigger: a second externally routed service, or auth/rate-limiting needed in more than one place.
- **AX-010 Partner credential storage** (new) — stance *decided*, status *planned*, ruling as in Phase 2; also remove secrets from the implicit gap by naming it in the shelf-coverage line.
- **Shelf coverage** — add the walk date for this delta and move credential storage out of the unwalked set; leave observability, rate limiting, feature flags, data retention, and audit still listed as not walked.

**Write:** `architecture/spine.md` — add the `api` and `worker` modules above, PayFlow to external systems, the `worker → PayFlow` communication style, and a boundary line stating that PayFlow is reachable only from `worker` because that is the service holding allow-listed static IPs (C-002). Mark the FEAT-010 items *planned*, not *built* — the code does not exist yet — and update "Last confirmed" only when FEAT-010 lands, leaving the current 2026-05-14 line intact.

I would keep every FEAT-010 entry visibly *proposed/planned* until implementation, so the store does not claim as-built state for unwritten code.

## Phase 6 — Consistency pass

Re-read the two store files as edited plus the spec, and check: every FR and NFR maps to something in the delta (FR-001…FR-007, NFR-001…NFR-003); no delta element violates C-001…C-004; no ledger entry is left contradicting another; the diagram matches the prose; each new table carries `business_id`. Fix what does not line up.

## Delegation

None. This is a small workspace, the whole of it fits in context, and the work is judgment about constraints rather than parallelisable volume — handing it out would cost more in briefing and checking than doing it. If the CTO takes branch A-2 (bus and service anyway), the staged migration plan is still one document and still not worth splitting.

## What I would report at the end

- The delta is written and the store is updated, with FEAT-010 marked planned.
- Plainly: I kept one of the sketch's five components as proposed and reshaped or deferred the other four, each against a specific constraint in the CTO's own spec or an existing ruling — with the table to check my reasoning against.
- The three findings that are bugs-in-waiting rather than opinions: the retry budget is roughly four minutes against a three-hour requirement, the local-hour venue selection drops a night at DST, and 200 venues in one timezone burst past PayFlow's 60/min limit.
- The two open items: PayFlow's egress-IP answer (a release blocker if they allow only one IP and Render gives several), and the webhook-versus-polling ruling, on which I have defaulted to polling.
- That no code, tests, or `render.yaml` changes were made, and which enforcement tests implementation now owes the ledger.