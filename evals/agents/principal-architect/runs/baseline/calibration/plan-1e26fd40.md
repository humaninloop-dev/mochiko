FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-010-payroll-export/spec.md`, `specs/FEAT-010-payroll-export/target-architecture.md`, `docs/payflow-api.md`, `src/crewboard/api/approvals.py`, `src/crewboard/domain/shift.py`, `src/crewboard/worker/celery_app.py`, `src/crewboard/integrations/xero/tasks.py`, `src/crewboard/scheduling/tz.py`, `render.yaml`

---

# Action plan — FEAT-010 payroll export architecture delta

## Standing position before Phase 1

I have read the sketch and the constraints and they do not fit each other. I will not draw the sketch as given. Five of its six structural moves are either forbidden by a constraint in the spec the CTO's own team wrote, or paid for by nothing in it. The plan below is therefore: recover the baseline, write the delta for the shape that is actually buildable, and put the disagreement in front of the CTO as an explicit stop with the sketch's version drawn as the rejected alternative — not quietly omitted.

---

## Phase 1 — Load the procedures I write under

**Do:** Load `mochiko:authoring-architecture-store` (store grammar, element lifecycle, what a landing flips, health view), `mochiko:patterns-system-design` (delta altitude, current/target/changed, diagram craft), `mochiko:patterns-technical-decisions` (decision-record form), `mochiko:patterns-architecture-shelves` (the shelf rows and their triggers).

**Why first:** every path and filename below is my working assumption. The store's own grammar governs where a delta file lives, how elements are keyed, and how a concern row is amended. If the loaded grammar disagrees with my assumed paths, the grammar wins and I re-target the writes in Phase 6 without re-deciding any content.

**Write:** nothing.

---

## Phase 2 — Recover and grade the baseline

The spine says **last confirmed 2026-05-14**, four months stale. FEAT-009 and FEAT-010 work has happened since. I will not design a delta on a baseline I have not re-checked.

**Do myself (interpretive, absence matters):**
- Re-read `render.yaml` against `spine.md` containers. Already done: `api` (2 instances), `worker` (beat enabled, queues `celery,slow`, concurrency 4), `web` static, `redis` starter, `db` Postgres 16 standard. **Matches the spine.** Static outbound IPs on `worker` are a dashboard setting recorded only in a comment — not in the blueprint. I flag that as a config fact with no enforcement.
- Re-read `celery_app.py` against AX-004. Beat carries `reminders-sweep`, `xero-sync` (01:00 UTC), `hourly-local-fanout` (top of every hour). AX-004's per-venue local-time fan-out is real and running — this is the hook FEAT-010 needs for NFR-001, and it already exists.
- Re-read `integrations/xero/tasks.py` against AX-007. `autoretry_for`, `retry_backoff`, `retry_backoff_max=1800`, `max_retries=8`, `acks_late=True`, idempotency key `crewboard-{business_id}-{day}`. AX-007 as-built is accurate. FEAT-010's batch id `crewboard-{venue_id}-{night}` is the same pattern one level down the hierarchy.
- Re-read `api/approvals.py`. The docstring states the placement rationale explicitly: approval is a rota edit with a role check **in the same transaction as the shift row**. This is load-bearing for Phase 4's refusal.
- Re-read `domain/shift.py`. States `scheduled → worked → approved`; the docstring already anticipates FEAT-010 adding `exported`.

**Delegate — three separate disposable `Explore` subagents, each `model: haiku`, one gap each:**

1. *Brief:* "In this checkout, does `tests/integrations/test_idempotency_key.py` exist? If yes, quote the assertion that checks the idempotency header and name the file:line. Report only what you find." — *On return I check:* that a quote with provenance came back. **Guard:** the README states a test the ledger cites may be absent from this slice and still exist in the full repo. So a "not found" from this worker is **not** evidence of a missing test and will not be allowed to change AX-007's status. Only a positive find is usable.
2. *Brief:* "List every module under `src/crewboard/api/` and `src/crewboard/scheduling/`, path only, no commentary."
3. *Brief:* "Find every definition or call site of `hourly_tick` and `venues_at_local_hour` in `src/`; report file:line and the enclosing function signature only."

These are locate-and-quote sweeps with a bounded answer. The judgment calls — whether the baseline is trustworthy, whether the approvals boundary should move — I keep.

**Write:** nothing yet. I hold a graded baseline note: containers **confirmed against `render.yaml` and code, high confidence**; concern rows AX-001/AX-002/AX-008 **unverified in this slice, carried forward as claimed, marked as such** rather than re-asserted as checked.

---

## Phase 3 — Test the sketch against the constraints

This is the buildability pass. For each component in the sketch I name the requirement or constraint that pays for it, or the constraint that forbids it.

| Sketch component | Verdict | Why |
|---|---|---|
| **Redpanda event bus** | **Refuse** | C-001 puts managed Kafka/Redpanda off the approved vendor list until the SOC 2 audit closes in 2027-Q1, and forbids new services this quarter. Separately, AX-005 already rules the bus *not-now* with a written trigger: a second consumer for the same event, or a fan-out one task cannot finish in its window. FEAT-010 has **one** consumer (the export) and at 200 venues × 3,000 shifts/month the nightly fan-out is trivially inside a task window. **The trigger has not fired.** The stated reasons — "where the industry has gone", "we'll want it for the next service" — are not requirements. |
| **payroll-service (new deployable)** | **Refuse as drawn** | C-001 forbids a new service. Worse, it is unbuildable as drawn: C-002 says PayFlow allow-lists **one static egress IP per partner account**, `worker` has static IPs and `api` does not, and a new Render service would need its own — a second IP, and a second allow-list entry PayFlow's model does not give us. The box also conflates two roles with different network and trust needs: outbound upload (needs the allow-listed IP) and inbound webhook termination (needs a public signature-verified edge, which AX-008 puts in `api`). One box cannot honestly be both. |
| **Approvals state machine moved into payroll-service** | **Refuse** | FR-006 states the approval flow is **unchanged**. `approvals.py` documents that approval commits in the same transaction as the shift row; moving it turns one ACID write into a cross-service write and makes "approved" eventually consistent for the exact data payroll depends on. This is a boundary move that a requirement explicitly forbids. |
| **`approved_hours_rm` CQRS read model** | **Refuse** | It creates a second source of truth for approved hours, and AX-001's tenancy enforcement (`business_id` + `TenantRepository`) would have to be re-implemented against a second store. Nothing in FR-001..FR-007 asks for it. Payroll reads shifts from `db` directly. |
| **Home-grown job runner (`job`, `job_attempt`, `dead_letter`, replay endpoint)** | **Refuse** | This is Celery, rebuilt by hand — retry, backoff, dead-lettering, replay. The team already operates Celery with exactly this behaviour in `xero/tasks.py`. Three engineers, one release (C-004). This is the paid-for responsibility that should never be hand-written: the requirement is real, the box is not. What genuinely *isn't* Celery is a durable per-venue-per-night **run record** for FR-004/FR-005 manager visibility — and that is one table, not a framework. |
| **Kong api-gateway** | **Refuse** | New service (C-001), new vendor pending SOC 2 (C-001), in front of one API service. "By next year we'll have three or four services" is a forecast, not a requirement. I will name the trigger that would make it real rather than just deleting it. |
| **Separate payroll-db** | **Refuse** | A new managed database is a new service under C-001, and it splits tenancy scoping across two stores. Use `db`. |
| **Hours reports served by payroll-service** | **Refuse** | AX-006 rules reports are computed on read in `api.reports` from the primary, with a written trigger (a report query above 2 s at p95). No evidence that trigger fired. "It has the data anyway" is not a reason to move a boundary. |
| **PayFlow as external system** | **Accept** | Correct and unavoidable. |

**Write:** nothing yet; this becomes the "alternatives considered" body in Phase 5's decision records.

---

## Phase 4 — Draw the shape I would actually build

The smallest topology that meets FR-001..FR-007 and NFR-001..NFR-003 under C-001..C-004. **Zero new containers.** Every new piece is a module inside a container that already exists, or a table in the database that already exists.

**Container-level changes:**
- `worker` — **modified.** New module `crewboard.integrations.payflow`, sitting exactly where AX-007 puts outbound integrations and where the allow-listed static IP already is (C-002 satisfied with no new IP). Tasks: a per-venue local-time fan-out riding the existing `hourly-local-fanout` beat entry and `scheduling/tz.venues_at_local_hour` so each venue fires at 02:00 **local** (NFR-001 — the CTO's sketch has no answer to local time at all); and `export_venue(venue_id, night)` with Celery `autoretry_for` / `retry_backoff` / `acks_late`, sending `Idempotency-Key: crewboard-{venue_id}-{night}` (FR-002, FR-003, AX-007).
- `db` — **modified.** New tables: `payroll_export_run` (venue_id, night, `batch_id` unique, state, attempts, last_error) carrying FR-004's pending/uploaded/accepted/rejected and the rejection reason; `payroll_line` (run_id, shift_id, staff_id, hours); `venue_payflow_connection` for FR-007. All carry `business_id` and go through `TenantRepository` per AX-001.
- `api` — **modified.** A thin `api.payroll` read/command surface: status per venue (FR-004) and a re-run trigger for one venue and one night (FR-005) that **enqueues the existing Celery task** rather than calling PayFlow inline — AX-003/AX-007, and forced anyway by C-002 since `api` has no allow-listed IP.
- `crewboard.domain.shift` — **modified.** Add `exported` after `approved` plus the manager-override path back to re-approval (FR-006). The file's own docstring already anticipates this.
- `web` — **modified**, minor: the export status panel.
- `api.approvals` — **existing, unchanged.** Explicitly recorded as unchanged, because the sketch proposed moving it and a reader must see that that was decided against, not overlooked.
- `redis`, `worker`'s other modules, Twilio/Postmark/Stripe/Xero — **existing, untouched.**

**The NFR checks I run against this shape before I draw it:**
- **NFR-002** (3-hour PayFlow outage inside the window, nothing lost, failure reported by 08:00 local): Xero's current settings — `retry_backoff_max=1800`, `max_retries=8` — give a retry envelope of roughly 1–2 hours, **short of three**. I will size `max_retries` explicitly for payflow so the envelope spans past 3 hours with margin, and record the number and its arithmetic in the delta rather than copying Xero's. `acks_late` covers worker restart. A sweep before 08:00 local marks any run still not accepted as failed and surfaces it (FR-004, plus notification via AX-003).
- **NFR-003** (accepted/rejected visible by 08:00 local): see the fork in Phase 5.
- **Rate limit / capacity:** PayFlow allows 60 uploads/min per partner account; at most 200 venues, fanned across local-time hours, so peak concurrency is far under the ceiling. `worker` concurrency is 4. No new capacity needed. I record the headroom so a future venue-count jump has a number to compare against.
- **PayFlow maintenance window** 03:00–03:30 UTC, first Sunday monthly, sits **inside** the 02:00–05:00 window for UTC-local venues. The retry envelope absorbs 30 minutes comfortably; I note it rather than leave it to be discovered.
- **Small operational flag:** `venues_at_local_hour` loads all venues and filters in Python. Correct and fine at 200 (C-003). Noted as a scaling note, not a change — I am not going to propose work nothing pays for.

**Write (draft, committed in Phase 6):** `architecture/deltas/FEAT-010-payroll-export.md` — current state (with the confidence grade from Phase 2), target state, and an explicit changed-list classifying every element new / modified / existing. Two container diagrams scoped to the neighbourhood the change touches — `api`, `worker`, `db`, `redis`, PayFlow — not a redraw of the whole system. Plus interaction flows for the three crossings whose ordering and failure semantics actually matter: (a) nightly local-time fan-out → upload → run record; (b) PayFlow slow/down during the window → retry → 08:00 failure report; (c) manager re-run of an already-uploaded night → same batch id → PayFlow returns the original result → no duplicate lines (FR-003).

---

## Phase 5 — The genuine forks, as decision records

Only two things here are real architectural forks. Everything else in Phase 3 is a constraint being applied, not a choice.

**DR-A — How PayFlow's result reaches the manager: poll vs webhook.**
- *Poll:* a worker task re-checks `GET /timesheet-batches/{batch_id}`. Validation completes in ~15 min in practice; polling ~200 batches is nothing against 60/min. No new public inbound surface, no HMAC verification, no event-id dedupe table, works whether or not each company enabled webhooks in its PayFlow dashboard.
- *Webhook:* lower latency, but PayFlow's webhook is **optional per company**, so we would need the poll as a fallback anyway and would be building both. It also terminates in `api` per AX-008 — **not** in the sketch's payroll-service — with signature verification and dedupe.
- **My recommendation: poll.** It meets NFR-003's 08:00 deadline with hours to spare and is strictly less machinery. I write this as my recommendation with its reversal trigger (PayFlow making webhooks universal, or a latency requirement tighter than hours).

**DR-B — No new service for FEAT-010.** Records the whole Phase 3 table: the sketch's shape as the seriously-considered alternative, each constraint that rules it out, and the trigger under which it becomes right — SOC 2 closing (2027-Q1) lifting the vendor block, *plus* a second consumer appearing for shift-approval events, *plus* a second payroll provider or a genuine second team. Written so that when those fire, nobody re-litigates from scratch.

**Write:** `architecture/decisions/` records per the loaded grammar.

---

## Phase 6 — Concern ledger updates and the store's health

**Amend:**
- **AX-005 (Event bus)** — stays *not-now*. Add a dated note that FEAT-010 tested the trigger and it did not fire: one consumer, fan-out fits the window. A deferral that gets re-examined and survives should say so, otherwise the next person re-argues it.
- **AX-007 (Outbound integrations)** — extend as-built to FEAT-010/PayFlow once landed. Not flipped now; this is a delta, nothing is built.
- **AX-004 (Scheduled work)** — note FEAT-010 rides the existing local-time fan-out.
- **AX-006 (Reports)** — note the sketch proposed moving hours reports out of `api.reports` and that it was declined, trigger unfired.

**New rows, proposed to the user — I do not record a stance they have not ruled:**
- **Secret storage for per-venue partner credentials.** FR-007 stores a PayFlow API credential per venue. There is no row on the shelf for this and the store has no stance. Hardest-to-retrofit thing in this feature: once credentials are written one way across 200 venues, changing it is a migration touching live payroll. My dealt default: encrypted at rest in `db` with the key in Render's environment, single-purpose accessor module, never logged — with the trigger that a second partner or an auditor finding moves it to a managed secret store.
- **Audit.** Listed as never walked. FEAT-010 introduces two actions that will be asked about after a payroll dispute: who re-ran an export (FR-005), and who overrode an approval on an exported shift (FR-006). Pay data makes this the moment to walk it.
- **Data retention.** Never walked. `payroll_line` is per-person pay data. SOC 2 closes 2027-Q1 and will ask.
- **Rate limiting.** Never walked. FEAT-010 gives the first hard external number (60/min) to anchor it.

**Surface as store health, unprompted:**
- Spine last confirmed 2026-05-14; FEAT-009 landed against AX-004 since. I bring the re-grade from Phase 2 rather than silently updating the date.
- Five shelf rows never walked (observability, rate limiting, feature flags, data retention, audit); FEAT-010 touches three of them.
- The `worker` static-IP fact lives only in a dashboard setting and two code comments. It is now load-bearing for a **second** partner. I flag the absence of enforcement — I do not invent a mechanism.
- The derived index is re-rendered from what I write, never hand-edited.

---

## Phase 7 — Enforcement tests named (specified, not run)

I do not write application code here, and under plan-only I run nothing. I specify in the delta what each concern row's enforcement must be, so a landing has something to grade against:

- Extend `tests/integrations/test_idempotency_key.py` (AX-007's cited enforcement) to cover the PayFlow client. *Expected:* every outbound batch call sets `Idempotency-Key`; a payflow upload without one fails the test.
- New `tests/payroll/test_export_idempotency.py`. *Expected:* running the same venue+night twice produces one `payroll_export_run`, reuses `crewboard-{venue_id}-{night}`, and adds no second set of `payroll_line` rows (FR-003, FR-005).
- New `tests/payroll/test_export_window.py`. *Expected:* venues in three different IANA zones each fan out at 02:00 **local** (NFR-001).
- New `tests/payroll/test_outage_envelope.py`. *Expected:* with PayFlow failing continuously, the configured retry envelope still has attempts remaining at the 3-hour mark, and the run is marked failed before 08:00 local (NFR-002).
- Tenancy: `payroll_export_run` / `payroll_line` / `venue_payflow_connection` covered by AX-001's isolation suite.

---

## Phase 8 — The stop

**I stop here and put it to the user before writing anything into `architecture/`.** The gap between the sketch and what I would draw is too wide to resolve by writing my version down and calling it the delta — the sketch is the CTO's stated direction, and overriding it silently would be exactly the invisible structural decision I refuse from others.

What I would put on the table: *the sketch's six components cost six new deployables and vendors that C-001 forbids this quarter, one of which (payroll-service owning both the PayFlow upload and its webhook) cannot be built at all under C-002's one-IP-per-partner rule; every requirement in the spec is met by modules inside `api` and `worker` and tables in `db`, with no new container. Confirm which shape the delta records.*

**Branches:**
- **Ruling: build my shape.** Proceed to write Phases 4–7 as planned. DR-B records the sketch as the considered-and-declined alternative with its reactivation trigger, so the CTO's intent is preserved as a dated future decision rather than erased.
- **Ruling: build the sketch anyway.** I do not draw an unbuildable diagram. I would return with the specific blockers that must be lifted first — a C-001 waiver, and a resolution of the C-002 single-IP conflict (realistically: split upload into `worker` regardless, which already collapses most of the sketch) — and record the topology as *proposed/blocked*, not as a target. The one thing I will not write is a target state that cannot be provisioned.
- **Ruling: some of it — e.g. keep payroll-service, drop Kong and the bus.** I re-run Phase 3's test on just that component. My answer would still be that C-002 forces the upload into `worker`, which leaves payroll-service owning only a read API over tables in the same database — at which point it is `api.payroll`. I would say so plainly and ask for the requirement that pays for the extra deployable.
- **No answer available:** I proceed under my default — the zero-new-container shape — with the delta clearly marked *pending CTO confirmation* and DR-B marked *recommended, not ruled*. I will not mark a stance decided that the user has not ruled.

**Secondary stop, non-blocking:** DR-A (poll vs webhook). Default: poll. Either ruling is a small change confined to one module.

---

## What I report at the end

The delta and decision records written and their paths; the baseline I recovered and how confident I am in each part of it (containers verified against `render.yaml` and code; three concern rows carried forward unverified because this slice lacks their tests, and the README says absence here is not evidence); the seven structural moves from the sketch I declined, each against the specific constraint or unfired trigger that rules it out, and the smaller shape that meets the same requirements — zero new containers, four modified; the one number I changed rather than copied (the retry envelope, sized for NFR-002's three hours instead of Xero's ~two); the two open forks and my recommendation on each; four concern rows I am proposing the user rule on, secret storage first because it is the most expensive one to retrofit across 200 venues of live payroll credentials; and the store health items — a four-month-stale spine confirmation, five unwalked shelf rows, and one load-bearing network fact that exists only as a dashboard setting and a code comment.