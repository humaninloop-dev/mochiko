FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-010-payroll-export/spec.md`, `specs/FEAT-010-payroll-export/target-architecture.md`, `docs/payflow-api.md`, `render.yaml`, `src/crewboard/api/approvals.py`, `src/crewboard/domain/shift.py`, `src/crewboard/worker/celery_app.py`, `src/crewboard/integrations/xero/tasks.py`, `src/crewboard/scheduling/tz.py`

---

# Action plan — FEAT-010 payroll export architecture delta

## Headline before the phases

I have read the sketch and I am not going to draw it. Four of its five components cannot be built under the constraints in the spec the sketch itself sits next to, and the ones that could be built are not paid for by any requirement in that spec. The honest artifact here is a delta with **zero new containers**. Phase 5 is where I stop and put that to you, because the sketch is yours and overruling it is your call, not mine. Everything before Phase 5 is the evidence I would gather to make that conversation short.

---

## Phase 1 — Confirm the baseline before designing on it

**What I would do.** The store claims a current state; I check the claims that this delta will lean on, rather than trusting them.

Already confirmed from my own reading:
- `worker` runs Celery 5 with beat, `celery` + `slow` queues, `acks_late`, prefetch 1, concurrency 4; `crewboard.integrations.*` routes to `slow` (`worker/celery_app.py`) — matches spine and AX-004/AX-007.
- The outbound-integration pattern in AX-007 is real and live: `xero/tasks.py` does one task per business, `autoretry_for`, `retry_backoff` capped at 1800 s, 8 retries, and an `Idempotency-Key` of `crewboard-{business}-{day}`. FEAT-010's FR-002 batch id `crewboard-{venue_id}-{night}` is the same pattern one level down the hierarchy.
- Per-venue local-time fan-out exists: `scheduling/tz.py` `venues_at_local_hour`, driven by the hourly beat tick. NFR-001's "02:00–05:00 local per venue" has a mechanism already in the system.
- Approvals genuinely live in `api` in one transaction with the shift row (`api/approvals.py`, `domain/shift.py`), and `Shift.STATES` is `scheduled → worked → approved` with a docstring already anticipating `exported`.

**Delegation.** Three separate cheap-read `Explore` subagents, each pinned to `model: haiku`, one gap each, each asked for file+line provenance and an explicit "not found" rather than a guess:
1. *Does any payroll/PayFlow code already exist?* Brief: search `src/` for `payflow`, `payroll`, `export`, `timesheet` (case-insensitive); list every file and symbol matched. On return I check whether it found anything that would make this an extension rather than a new module.
2. *Enumerate what `api` actually contains.* Brief: list every module and router under `src/crewboard/api/`, with each router's prefix. On return I compare to the spine's module list (`rotas, approvals, timeoff, staff, billing, reports`) and note any module the spine doesn't mention.
3. *Enumerate every Celery task and its retry settings.* Brief: every `@shared_task`/`@app.task` in `src/`, with decorator arguments verbatim. On return I use this to ground the retry-budget arithmetic in Phase 4 against what the team already operates.

I would not delegate reading the sketch, the spec, or the ledger — those are interpretive, and this checkout is a prepared slice where an absent file is not evidence of anything (the README says so explicitly, so I will not report the AX-003 test as missing).

**Flag.** The spine says "last confirmed 2026-05-14 (FEAT-007 landing)", but AX-004 records an as-built from **FEAT-009**. A feature landed after the spine's last confirmation and the spine was never re-confirmed. I record the baseline as confirmed-for-the-parts-I-checked and note this staleness in the health report (Phase 8), rather than quietly designing on top of it.

---

## Phase 2 — Test the sketch against the constraints (buildability)

**What I would do.** Take each of the sketch's five components and ask one question: can it be built and operated under C-001 through C-004? This is arithmetic, not taste, and it comes first because it disposes of most of the sketch without any argument about style.

| Sketch component | Verdict | Why |
|---|---|---|
| Redpanda event bus | **Cannot be built** | C-001: managed Kafka/Redpanda is not on the approved vendor list until the SOC 2 audit closes in 2027-Q1. Also a new service against "no new services this quarter". |
| payroll-service (new Python service) | **Cannot be built** | C-001: no new services this quarter. |
| payroll-db (separate database) | **Cannot be built** | Same: a new managed Postgres is a new service; `render.yaml` carries one `databases:` entry and a budget note. |
| api-gateway (Kong) | **Cannot be built** | C-001 on both counts: new service and new vendor. |
| PayFlow as external system | **Builds** | Genuinely required by the feature. |

**The one that matters most — egress.** C-002 and the PayFlow notes agree: PayFlow allow-lists **one static egress IP per partner account**, and on Render static outbound IPs are **per service**. `worker` has them (enabled 2026-05-12 for Xero); `api` does not. So the PayFlow upload can only originate from `worker`. The sketch has `payroll-service` calling PayFlow — a service that would have neither static IPs nor a way to get an allow-listed one without burning the partner account's single IP slot. This is not a preference; the sketch's central flow does not have a network path to its external system.

**Open question I would raise, not assume.** Render typically assigns a *set* of static outbound IPs to a service, while PayFlow allows *one* per partner account. Xero's allow-list may accept the set; PayFlow's may not. I would confirm the exact IP count on `worker` and PayFlow's tolerance before the delta is called buildable, and record it as an open risk on the delta with an owner. If PayFlow accepts only a single IP and Render gives three, that is a blocker for the whole feature regardless of shape, and it is better found in week one than in week four.

---

## Phase 3 — Test the sketch against the requirements (is it paid for?)

For the pieces that *could* be built if the constraints lifted, I ask which requirement pays.

- **Event bus.** AX-005 is already ruled not-now, with a precise trigger: *a second consumer for the same domain event, or a fan-out a single task cannot finish in its window*. FEAT-010 has exactly one consumer of `ShiftApproved` — the payroll export — and C-003 bounds the work at 200 venues × ≤3,000 shifts/venue/month, which is one modest batch per venue per night. **The trigger has not fired.** The sketch's stated reason is "where the industry has gone" and "we'll want it for the next service anyway" — that is a purchase with no payer. I record that FEAT-010 was tested against the trigger and did not fire, which is more useful than silence.
- **api-gateway.** Pays for "three or four services next year" that no requirement asks for. Speculative.
- **Home-grown job runner** (`job`, `job_attempt`, `dead_letter`, backoff, replay endpoint). This one is paid for — retry, backoff, and dead-lettering are real needs under NFR-002 — but it is the **wrong box to build**. Celery already provides every one of these, the team already operates it, `xero/tasks.py` is a working instance of exactly this pattern, and AX-007 is the standing ruling that says so. Hand-writing a scheduler and retry engine is weight three engineers (C-004) carry forever. Reuse, don't build.
- **CQRS read model `approved_hours_rm`.** A second copy of approved hours creates a second source of truth for the number people get paid on. Nothing in the spec pays for it, and it directly contradicts FR-006's "the approval flow is unchanged".
- **Moving approvals into payroll-service.** FR-006 says unchanged. The code comment in `api/approvals.py` says it lives there on purpose — it is a rota edit with a role check in the same transaction as the shift row. Moving it converts a local transaction into a cross-service one and moves a boundary the feature explicitly asks not to move.
- **Moving hours reports into payroll-service.** AX-006 rules reports computed on read from the primary in `api.reports`, with an upgrade trigger of a report query above 2 s at p95. No such evidence is offered. The sketch's reason — "it has the data anyway" — is only true because of the read model, which itself is unpaid. Two unpaid things holding each other up.
- **PayFlow webhooks terminating in payroll-service.** AX-008 rules that inbound third-party callbacks terminate in `api` behind signature verification and an event-id dedupe table. A second inbound surface elsewhere contradicts a settled stance.

---

## Phase 4 — Draw the shape that is actually paid for, and check it meets the NFRs

**Target topology: no new containers.** Every container is `existing` or `modified`.

- `api` — **modified**. Adds read endpoints for export status per venue/night (FR-004) and a re-run trigger that enqueues a Celery task and returns 202 (FR-005); adds owner-facing PayFlow venue link setup (FR-007). Approvals untouched (FR-006).
- `worker` — **modified**. New module `crewboard.integrations.payflow`, built to the AX-007 shape: `export_venue(venue_id, night)` with Celery retry and `Idempotency-Key = crewboard-{venue_id}-{night}`; a local-hour fan-out task riding the existing `hourly_tick` / `venues_at_local_hour` machinery to fire at each venue's local 02:00 (NFR-001); a status task that reconciles PayFlow's verdict (NFR-003). Routed to the `slow` queue by the existing `crewboard.integrations.*` rule.
- `db` — **modified**. `payroll_export_run` (venue, night, batch_id unique, state, attempts, last_error) and `payroll_line` (run_id, shift_id, staff_id, hours) — both paid for, both in the shared pooled database under AX-001 tenancy scoping, not a separate payroll-db. A venue↔PayFlow company link with its credential (FR-007). `Shift` gains the `exported` state its own docstring already anticipates (FR-006).
- `web` — **modified**. The per-venue export status screen and re-run control (US-002).
- `redis`, and every other existing container — **existing**, untouched.
- **PayFlow** — new external system. Called synchronously over HTTPS from `worker` tasks, retried by Celery. Exactly the Xero pattern.

Four new components in the sketch become zero. The interaction style is the one already in the spine (`api → worker` over redis for anything outliving the request), so there is no new failure mode for the team to learn.

**Interaction flows I would draw** (these are the parts whose ordering and failure semantics actually matter, so they get sequence diagrams rather than prose):
1. Nightly export for one venue — local-hour tick → build batch → upload with idempotency key → record run state.
2. PayFlow unavailable — the retry path, and what happens when the retry budget expires.
3. Manager re-run of a night (FR-005 / FR-003).

**Three NFR checks I would do as arithmetic and write into the delta, not assert:**

- **NFR-002 retry budget.** A three-hour outage must lose nothing. The Xero settings in `xero/tasks.py` — `retry_backoff` doubling, capped at 1800 s, `max_retries=8` — sum to roughly four minutes of coverage. Copying them would silently fail NFR-002 by two orders of magnitude. I compute the retry schedule that actually spans three hours from a 02:00 local start and still leaves room to mark the run failed and notify by 08:00 local (NFR-003's deadline is wall-clock, so this is better expressed as *retry until local 07:00, then fail loudly* than as a retry count), and record the resulting parameters in the delta as an NFR-driven value with its derivation shown.
- **PayFlow rate limit, 60 uploads/min/partner.** 200 venues (C-003) fanned out by local hour means every venue sharing a timezone fires in the same tick. A UK-heavy customer base could put well over 60 uploads into one minute and start getting rejected. The fan-out needs throttling — a Celery task rate limit or a paced tick. This constraint appears nowhere in the sketch and would have been discovered in production.
- **NFR-003 result visibility — a genuine fork.** PayFlow's webhook is *optional and enabled per company in their dashboard*, so it cannot be the only path to a verdict without making every owner configure it. Their status endpoint settles within 15 minutes in practice (1 hour promised). **Recommendation: poll status from `worker`** — one path, uses the egress `worker` already has, no new inbound surface, comfortably inside the 02:00→08:00 budget. Webhooks become a later optimisation with a trigger. I put this to you rather than deciding it, since it touches an NFR.

**A design flaw in the sketch's own semantics I would fix.** FR-001 defines the batch as "all shifts approved since the venue's last successful export" — a moving watermark — while FR-002/FR-003 say re-runs are deduplicated by a batch id of venue+night. Those two combine badly: a re-run of last night's batch would now select *more* shifts, but PayFlow, seeing a repeated batch id, returns the original result and **creates nothing** — silently dropping the newly-included shifts. People do not get paid, and nothing errors. The fix is to pin the batch's contents at run creation (that is what `payroll_line` earns its place doing) so a re-run resends exactly the same lines, and shifts approved afterwards are picked up by the next night's run. This is a genuine fork and gets a decision record.

---

## Phase 5 — The stop

**What I would confirm with you.** That the sketch is overruled, and on what grounds. I would bring one page: four components that the spec's own constraints forbid, the egress path that does not exist, the AX-005 trigger that has not fired, and the zero-new-container shape that meets every FR and NFR. I am not asking whether you like it; I am asking you to rule, because the store should record *your* decision, not my recommendation wearing your name.

**Branches:**
- *You accept the minimal delta* (my default if no ruling comes): proceed to Phase 6 as written.
- *You want the event bus anyway.* Then C-001 is the blocker, not me — I would ask you to either get Redpanda onto the approved vendor list (a SOC 2 and procurement question, 2027-Q1 today) or accept Redis Streams on the existing `redis` as a non-vendor stand-in. I would still argue against it for FEAT-010 with one consumer, but if you rule it in, it is recorded as your ruling with the rationale as given, and I would insist the export path itself still runs from `worker` for the egress reason, so the bus buys nothing this quarter and we both see that in writing.
- *You want payroll split out as a service.* Then the conversation is C-001 and the single-IP allow-list, and my counter is: build the module inside `worker` now with a clean internal boundary (`crewboard.integrations.payflow`, no other module importing it — the boundary AX-007 already enforces for Xero), so extraction later is a deployment change rather than a rewrite. That is the cheap option on the way to your destination.
- *You want approvals moved.* I would push back hardest here and ask for FR-006 to be rewritten first, because the spec and the sketch currently contradict each other and I will not resolve that silently in a diagram.
- *You want the gateway.* Deferred with a trigger (below), not deleted.

**What I would refuse outright**, regardless of ruling: drawing the delta as though the sketch were buildable, and recording any of the sketch's components as *decided* on your behalf. Each rejected piece becomes a deferred concern with a trigger, so a year from now it is visible that it was considered and priced, not overlooked.

---

## Phase 6 — Write the delta

I would consult the architecture-store authoring skill for the exact file naming, element-id grammar, and diagram conventions before writing, and the system-design skill for the altitude and the flow diagrams; the paths below are my intent, and the skill's grammar governs the final names.

**Written:**
- `architecture/deltas/FEAT-010-payroll-export.md` — current state (confirmed in Phase 1, with confidence noted), target state, and every structural change marked new/modified/existing; a scoped container view showing only the neighbourhood the change touches (`web`, `api`, `worker`, `db`, `redis`, PayFlow) rather than redrawing the system; the three sequence flows; the NFR checks with their arithmetic; the open egress-IP risk; and an explicit **"considered and not taken"** section carrying the sketch's five components with the constraint or missing payer for each, so the reasoning survives.
- `architecture/decisions/` — two decision records, for the genuine forks only: (a) how a night's batch contents are pinned, and what a re-run means; (b) polling versus webhook for PayFlow's verdict. The event bus, the gateway, and the new service are *not* decision records — they are applications of existing rulings plus a hard constraint, and they belong in the ledger rows below.

**Not written yet:** `architecture/spine.md`. The delta is proposed; the spine flips when FEAT-010 lands and the as-built is graded against the code. Writing it now would be recording an intention as reality.

---

## Phase 7 — Walk the shelf and update the ledger

Written to `architecture/concerns.md`:
- **AX-005 event bus** — reaffirmed not-now; note that FEAT-010 was tested against the trigger and it did not fire (one consumer, fan-out fits a task); trigger unchanged.
- **AX-007 / AX-008** — confirmed as the stances FEAT-010 builds to; no change.
- **AX-006 reports** — reaffirmed; the sketch's move was not paid for and the 2 s p95 trigger has not fired.
- **New: API gateway** — not-now. Trigger: a third externally-routed service, or an externally-exposed partner API needing its own auth edge.
- **New: service decomposition** — not-now. Trigger: C-001 lifting *and* a concrete reason (independent scaling, separate egress, a team boundary) — "we'll have more services" is not one.
- **New: secrets and partner credentials** — this one **needs a ruling from you**, because FR-007 stores a per-venue PayFlow API credential and the shelf has never covered it. I would deal the options (Render env vars vs. encrypted-at-rest column with a KMS key vs. a secrets manager, which is a new vendor under C-001) with my default and its trigger, and stop for your word.
- **New: outbound rate limiting** — forced onto the table by PayFlow's 60/min against 200 venues.
- **Raised, not resolved**: the shelf still has observability, feature flags, data retention, and audit unwalked. Payroll data makes **audit** and **retention** materially more expensive to retrofit than they were last month. I would name them and ask whether to walk them now or book the walk.

---

## Phase 8 — Store health, and the enforcement the delta calls for

**Health findings I would surface rather than tidy away:** the spine is stale relative to FEAT-009; the shelf has five unwalked rows, two of which this feature makes urgent; AX-007's as-built claim I verified directly against `xero/tasks.py` and it holds; AX-004's I verified against `celery_app.py` and `scheduling/tz.py` and it holds. I would not claim to have verified AX-001/AX-002/AX-003 — their enforcement tests are outside this checkout slice, so those rows stay as-recorded with their provenance unchanged, not re-asserted from memory.

**Enforcement the delta names** (for the implementers to write; I do not write or run them here): extend the existing `tests/integrations/test_idempotency_key.py` to cover the PayFlow client so the batch id is asserted on every upload; a re-run test asserting an identical line set on the second run, which is the direct guard on FR-003 and on the watermark flaw in Phase 4; a tenancy test for the two new tables under AX-001. I would expect the re-run test to fail against a naive watermark implementation — that is the point of writing it.

**What I would report at the end:** a short verdict — the sketch's four new components are unbuildable under the spec's own constraints and unpaid by its own requirements; the feature ships with zero new containers on machinery the team already runs; three things the sketch missed that would have bitten in production (the egress path, the retry budget being ~4 minutes where 3 hours is required, and the re-run watermark silently dropping shifts); one hard open risk (Render's IP count versus PayFlow's single-IP allow-list); and two rulings I need from you before the store is honest — the credential-storage stance, and confirmation that the sketch is overruled.