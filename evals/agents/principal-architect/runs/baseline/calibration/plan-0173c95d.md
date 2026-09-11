FILES-READ: `specs/FEAT-014-swap-approvals/spec.md`, `specs/FEAT-014-swap-approvals/design-draft.md`, `architecture/spine.md`, `architecture/concerns.md`, `README.md`, `render.yaml`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/rotas.py`, `src/crewboard/domain/swaps.py`, `src/crewboard/notify/tasks.py`, `src/crewboard/notify/twilio_client.py`, `src/crewboard/worker/celery_app.py`, `web/src/swap/eligibility.ts`, `tests/notify/test_no_inline_provider.py`, `docs/incidents/INC-31.md`

---

# Action plan — FEAT-014 architecture delta

## Phase 1 — Recover and confirm the baseline before drawing anything on it

The analyst says the current-state picture is taken from `architecture/spine.md`. I've read both and they disagree with each other and with the code, so the first job is to fix the ground I'd be designing on.

**What I'd do:**

1. **Grade the store's claims against the code, not against my memory of the design.** Three claims fail:
   - `concerns.md` AX-003 says outbound messaging is worker-only and **"Drift: none."** That is false today. `api/swaps.py` line 8 and `api/timeoff.py` line 6 both import `crewboard.notify.twilio_client` and send SMS inline from the request handler; `tests/notify/test_no_inline_provider.py` skips exactly those two modules via `HOTFIX_SKIP`. The ledger is recording an intention as a fact. This must be corrected before the delta is drawn, because FEAT-014 is the named owner of the revert in `docs/incidents/INC-31.md`.
   - `spine.md` is stamped "Last confirmed 2026-07-02" but already carries a 2026-08-28 edit (worker concurrency). Its confirmation date is not trustworthy; I'd re-confirm it as part of this work and re-stamp it.
   - `concerns.md` AX-005 (provider-call idempotency, "not-now") states the upgrade trigger as *"the first incident in which a recipient receives the same message twice."* INC-31 records exactly that — two staff at Harbour Kitchen received the rota-publish SMS twice. **The trigger has already fired and nobody re-ruled the row.** Surfacing this is my job, not the user's.

2. **Correct the draft's picture of what exists.** The draft's target diagram shows `billing`, `payroll-export`, `xero-sync`, and `reports` as boxes alongside `api` and `worker`. They are not containers. Per `spine.md` and `celery_app.py`, `billing` and `reports` are modules *inside* `api`; Xero sync and PayFlow payroll are modules *inside* `worker` on the Celery beat schedule. The draft has invented five containers that do not exist, which means its "the existing boxes stay where they are" claim is drawn against a baseline that was never confirmed. `render.yaml` is the authority: four services plus one managed database — `api`, `worker`, `web` (static), `redis`, `db`.

3. **Size the AX-003 revert honestly.** I need to know whether the inline-provider drift is confined to those two modules.

   **Delegation:** one disposable `Explore` subagent, `model: haiku`. Brief: *"List every file under `src/` and `web/` that references `twilio_client`, `postmark_client`, `twilio.messages`, or `postmark.send`. Return file path and line number only. Do not summarise or interpret."* On return I check the list against my own reading of `api/swaps.py`, `api/timeoff.py`, `notify/tasks.py`, `notify/twilio_client.py` — if it returns only those four, it agrees with me; if it returns more, I read the extras myself.

   **What I will not conclude from it:** the README says this checkout is a slice and that tests cited by the ledger exist in the full repo. So *absence* in this checkout is not evidence of absence in the repo. I'd mark the recovered baseline **"reconstructed from a partial checkout — high confidence on the four container topology (`render.yaml` is complete), medium confidence on the full extent of the AX-003 drift"** and say so in the artifact rather than pretending to certainty.

**Written in this phase:** nothing yet. This phase produces the corrected baseline I carry into Phase 2.

---

## Phase 2 — Weigh the draft and write down what I would cut

The draft is not a small adjustment away from being right; four of its structural choices are things I would refuse. I'd write the critique as a section of the delta doc (Phase 4), not as a separate memo, so the reasoning travels with the design. Itemised:

| # | Draft proposal | My ruling | Why |
|---|---|---|---|
| 1 | New **`notifier`** service | **Refuse** | It is a fifth container. `render.yaml` line 2 and spec C-001 forbid new services until Q1 2027 (finance, 2026-07-01). A shape that cannot be provisioned under the stated constraint is not a design. It also moves outbound messaging out of `worker`, reversing AX-003, without saying it is doing so. |
| 2 | `notification_outbox` with `attempts`, `next_attempt_at`, exponential backoff to five attempts, polled every 5 s | **Refuse as drawn** | This hand-builds machinery the product already owns and pays for. `notify/tasks.py` `send_sms` already has `autoretry_for`, `retry_backoff`, `retry_backoff_max=600`, `retry_jitter`, `max_retries=5`, `acks_late`. Redis+Celery *is* the queue. Building a polling loop over a Postgres table to redo it is a box the team maintains forever for no requirement. Also: a 5-second poll cycle is a poor fit for NFR-001's 60 s budget when `.delay()` is immediate. |
| 3 | `notifier` owns the eligibility check (FR-002), with `api` keeping "its own copy" and `web` keeping the client mirror | **Refuse** | FR-002 names the single existing home: `crewboard.domain.swaps.can_swap`. The draft's own justification is wrong — `can_swap` already returns `(ok, reason)` as a tuple, so the reason for wording the message is available as **data to pass along**, not a rule to recompute. Three copies of one rule is a defect I catch now, not at runtime; the mirror in `web/src/swap/eligibility.ts` already drifted once (its own comment cites the INC-27 follow-up). |
| 4 | `api` approve endpoint calls Twilio directly and returns 200 only after Twilio confirms delivery | **Refuse, hardest of the four** | It re-entrenches the exact hotfix FEAT-014 is chartered to revert (INC-31 follow-up, explicitly "owner: FEAT-014"). It contradicts NFR-003 arithmetically: Twilio p95 is 1.4 s per INC-31, the endpoint budget is 500 ms p95. It contradicts NFR-002 (decision must persist and be visible when the provider is unreachable) and FR-005 (a failed notification must not block the state change) — blocking the 200 on delivery does both of the things FR-005 forbids. |
| 5 | `channel` enum including `push`/`whatsapp`; `notification_channel_config` table for the 2027 roadmap | **Cut** | The spec's out-of-scope section names push and WhatsApp explicitly. Structure built for a future no requirement asks for. |
| 6 | `swap_event` has no `business_id` | **Flag as a defect** | AX-001 rules `business_id` on *every* tenant table, enforced by `tests/tenancy/test_isolation.py`. `swap_request` has it; `swap_event` does not. It would fail the isolation test. |
| 7 | "No architecture change — this just adds tables and a helper service" | **Reject the framing** | Adding a container, relocating a responsibility across a boundary, duplicating a domain rule, and reversing a hotfix are four structural changes. A structural change the reader cannot see is a structural change nobody chose. This is a real delta and I'd label it as one. |

**One thing the draft is right about and I keep:** decoupling the send from the request. It just already exists — it's called Celery, and it's ruled in AX-003.

**One thing the draft never asked, which is the genuine architectural question here:** INC-31's root cause was not worker size alone — it was that *"the single `celery` queue serialised every other task behind the burst."* The resize fixed the symptom; the single queue is still there (`celery_app.py`: `task_default_queue="celery"`, one queue). NFR-001 gives a swap notification 60 s at p95, and a Monday publish burst still takes ~6 minutes to drain. An interactive swap notification queued behind that burst can miss NFR-001. **That** is the piece a requirement actually pays for, and it costs no new service.

---

## Phase 3 — The desk visit: shelf walk and the rulings I need

This is where I stop. I'd bring the Phase 1 findings and the Phase 2 rulings, deal each open concern with my recommendation and the trigger that would change it, and let the user rule. Hardest-to-retrofit first. For each, what I'd confirm and where each branch goes:

**A. AX-003 — revert the inline sends? (highest, unblocks everything)**
- *My recommendation:* yes. Remove the Twilio import and inline sends from `api/swaps.py` and `api/timeoff.py`, delete `HOTFIX_SKIP` from the guard test, and route all four FEAT-014 notification points through `worker` tasks. FEAT-014 already owns this follow-up.
- *If ruled "revert":* the delta includes `api/timeoff.py` even though time-off is not FEAT-014's feature — small, and leaving half the hotfix in place means the guard test stays half-disabled and the drift returns.
- *If ruled "defer the timeoff half":* I scope the revert to `swaps.py`, keep `HOTFIX_SKIP = {"crewboard.api.timeoff"}`, and record the residue in AX-003 as named live drift with a new owner — not as "none."
- *If ruled "keep inline sends":* I refuse to write NFR-003 as met and say so in the delta; AX-003 would need to be re-ruled from "decided" to something else at the user's word, and I would not quietly leave it reading "decided/built/no drift."
- **Default if no ruling reaches me:** full revert.

**B. AX-005 — provider-call idempotency, trigger already fired**
- *My recommendation:* re-rule to decided-and-scoped-small — pass a Twilio idempotency key on `send_sms`. Note the important distinction the draft misses: the duplicates in INC-31 happened because *Twilio had already accepted the message* and the task timed out and Celery retried. A `dedupe_key` on a database outbox row does not prevent that; only a key the provider itself honours does. The draft's dedupe column would give false comfort.
- *If ruled "do it now":* it lands in `notify/tasks.py` alongside FEAT-014 (small, one call site) and AX-005 moves to decided.
- *If ruled "still not now":* I'd insist the row be re-written with a *new* trigger, because the current one has fired and a deferral without a live trigger is a forgotten decision, not a deferred one.
- **Default:** raise it, recommend now, proceed assuming not-now with a rewritten trigger so FEAT-014 isn't blocked on it.

**C. Queue separation for interactive notifications (new row)**
- *My recommendation:* a second Celery queue, `interactive`, for user-decision notifications (swap, time-off), leaving bulk fan-out (rota publish, reminders) on `celery`. The same single `worker` service consumes both — `startCommand` gains `-Q celery,interactive`. **No new service, so C-001 holds.** With `worker_prefetch_multiplier=1` and concurrency 8, the worker interleaves across queues, so an interactive task waits roughly one in-flight send (~1.4 s) for a free slot rather than the ~6-minute burst drain. That is what makes NFR-001 credible instead of hopeful.
- *If ruled "not needed":* I record NFR-001 as at-risk during Monday publish bursts, with INC-31 as the evidence, rather than claiming it is met.
- **Default:** propose the second queue.

**D. Observability (never walked; NFR-001 says "measured in the worker")**
- The spec asks for a p95 measured in the worker and the store has no observability stance at all. You cannot claim NFR-001 without a measurement point. *Recommendation:* minimum viable — emit change-time-to-provider-handoff from the notification task, one row/metric per send. Open a concern row either way.
- **Default:** open the row, recommend the minimum, proceed.

**E. Concurrency control on shift assignment (never walked; FR-003)**
- FR-003 requires the exchange to be atomic and a swap whose shifts changed since proposal to be *rejected with a reason*. Today `decide()` re-runs `can_swap` but has no version guard on the shift rows — which is the shape of the INC-27 double-booking this feature exists to stop. *Recommendation:* optimistic concurrency — capture shift version/`updated_at` at proposal, guard the update inside the single transaction. Open a row so the stance applies beyond swaps.
- **Default:** open the row, recommend optimistic locking.

**F. Audit and data retention (never walked; FR-006)**
- `swap_event` is an audit trail by another name. *Recommendation:* keep it feature-local for now — one consumer, no general audit requirement yet — but open the row with the trigger "a second domain object needs the same history view," and a retention row with the trigger "first data-deletion or subject-access request." These are cheap to open and expensive to retrofit.

**G. Eligibility mirror in `web` (scope question, not a shelf row)**
- *Recommendation:* delete `web/src/swap/eligibility.ts` and have `api` return a computed `eligible`/`reason` on the shift list, leaving `can_swap` the single home. That removes a duplicate that has already drifted once. It is a small addition to FEAT-014's scope.
- *If ruled out of scope:* the mirror stays, and I record it in the delta as a known duplicated responsibility with FR-002's single-home rule noted as violated by an existing artifact, not by this change.
- **Default:** propose the deletion, proceed with the mirror retained if I get no ruling, since it is not required by any FR.

**H. Rate limiting (never walked)** — one line: could a staff member spam a colleague with proposals and thus SMS? *Recommendation:* not-now, trigger "first abuse report or a venue exceeding N proposals/day." I'd raise it rather than let it pass silently just because the answer looks obvious.

Also confirmed at this stop: **does FEAT-014 own reverting `api/timeoff.py`** (item A), and **does the `interactive` queue count as an infrastructure change needing finance sign-off?** My reading is no — it's a flag on an existing service's start command, not a new billable service — but I'd say so out loud rather than assume.

---

## Phase 4 — Write the delta

**File I'd write:** `specs/FEAT-014-swap-approvals/architecture.md`

Contents:

1. **Baseline (current state, scoped to the neighbourhood this change touches)** — `web`, `api` (`swaps`, `timeoff`, `rotas`), `db`, `worker` (`notify`), `redis`, Twilio, Postmark. Marked *reconstructed 2026-09-11 from code + `render.yaml`*, with the confidence note from Phase 1 and an explicit correction of the draft's five phantom containers. Billing/Stripe, Xero, PayFlow, reports are named as out-of-neighbourhood and linked to `spine.md` rather than redrawn — I'm not putting a hundred boxes in front of the reader.
   The baseline shows the **live drift honestly**: `api.swaps` and `api.timeoff` currently hold a direct Twilio edge, drawn as a marked violation of AX-003, not airbrushed out.

2. **Target** — **zero new containers.** Every element classified:
   - `web` — *modified*: propose / accept-decline / approve screens, swap history view (US-005).
   - `api.swaps` — *modified*: new colleague accept/decline endpoint (US-002); `decide()` gains the manager-role check it has, plus the stale-shift guard (FR-003) and a reason on rejection; **inline Twilio edge removed**; enqueues a notification task after `db.commit()`.
   - `api.timeoff` — *modified, revert only*: inline Twilio edge removed, enqueues the existing task instead.
   - `domain.swaps` — *modified*: `can_swap` unchanged and confirmed as the single home for FR-002; state-transition helpers added.
   - `db` — *modified*: `swap_request`, `swap_event` (**both with `business_id`**, AX-001). No outbox table, no channel-config table.
   - `notify.tasks` — *modified*: one new task taking a `swap_event_id`, re-reading through the tenant-scoped session and choosing SMS-or-email exactly as `send_rota_published` already does (mobile on file → SMS, else email — FR-004's fallback rule already exists in code, I'm reusing it, not reinventing it). Routed to `interactive`.
   - `worker` — *modified*: consumes `celery,interactive`.
   - `redis` — *existing, unchanged*.
   - Twilio / Postmark — *existing, unchanged*; still reached only from `worker`.
   - **`notifier` — does not exist.** Named explicitly as considered and rejected, so nobody re-proposes it in three months.

3. **Interaction flows** — four, drawn only where ordering or failure semantics actually matter:
   - *Propose → colleague notified* (US-001, FR-001/2): sync `web→api`, `can_swap` sync in-transaction, commit, then async enqueue.
   - *Colleague accepts → proposer and manager notified* (US-002, FR-004).
   - *Manager approves → assignees exchanged atomically → both parties notified* (US-003, FR-003): the transaction boundary drawn explicitly, with the commit strictly **before** the enqueue.
   - *Failure flow*: Twilio down. State already committed; task retries with existing backoff; after 5 attempts it gives up; **state never reverses; app shows current state** (FR-005, NFR-002). This is the flow the draft's blocking-200 design gets wrong, so it gets drawn.

4. **Why each piece is paid for** — one line per added element naming the FR/NFR/constraint that forces it. Anything I can't name a payer for doesn't go in.

5. **The critique table from Phase 2**, so the reasoning for each refusal is recorded where the analyst and the design phase will read it.

6. **Open questions carried forward**, with my default recorded for each unruled item from Phase 3.

---

## Phase 5 — Decision records

Two genuine forks turn the shape; each gets its own record under `architecture/decisions/` (I'd confirm the existing numbering and filename convention first — no `decisions/` directory is present in this slice, and per the README absence here isn't proof of absence in the repo, so I'd check before inventing a scheme):

- **"Transport for swap-state notifications"** — options weighed: (a) Celery task on Redis, reusing AX-003; (b) database outbox + polling `notifier` service, per the draft; (c) inline provider call from the request, per the draft's approve path. Decision: (a). Consequences recorded, including the one real thing (b) buys that (a) doesn't: if Redis is unreachable at enqueue time the notification is lost with no durable record. I'd state plainly that this is the accepted trade — Redis unavailable means Celery is dead product-wide, and no FR or NFR pays for surviving that case — and give the trigger that would reopen it: *a Redis outage that loses user-visible notifications, or a durability requirement on notification delivery.*
- **"Queue separation for interactive notifications"** — options: single queue as today; second queue on the same worker; separate worker service (rejected, C-001). Decision: second queue. Evidence: INC-31's serialisation cause and NFR-001's 60 s budget.

---

## Phase 6 — Store updates

**`architecture/concerns.md`:**
- **AX-003** — replace the false "Drift: none." Record the INC-31 hotfix as live drift with dates and the two affected modules, and record FEAT-014 as the owner of the revert with the guard test as the enforcement. On landing this flips back to no-drift; not before.
- **AX-005** — mark the upgrade trigger as **fired** (INC-31, 2026-08-20), record whatever the user rules, and note that a database dedupe key would not have prevented the observed duplicates.
- **AX-007 (event bus, not-now)** — check its trigger too while I'm here: "a second consumer for the same domain event." FEAT-014's swap-approved event has one consumer (the notification task). Trigger has **not** fired; I'd record that it was checked and stayed not-now, rather than leave it silently unexamined. This also kills any argument that FEAT-014 justifies an event bus.
- **New rows** from the shelf walk: observability, concurrency control, audit, data retention, rate limiting, queue routing — each with the stance the *user* ruled, not my recommendation dressed up as their decision. Anything they didn't rule stays open, not "decided."
- Update the shelf-coverage line to reflect what was actually walked on 2026-09-11.

**`architecture/spine.md`:** at design time I'd add the FEAT-014 delta as in-flight and re-stamp the confirmation date with a note that the 2026-07-02 stamp was stale. The container list, the `worker` module list (`notify` gains swap notifications), the queue line ("one queue `celery`" → two queues on one worker), and the communication-styles line ("`api → worker`: … nothing else today") all change **on landing**, not now.

**`render.yaml`:** the `worker` `startCommand` gains `-Q celery,interactive`. Named in the delta as the one infrastructure edit; no new service block. I'd flag it for the implementer rather than edit it during the design phase.

**Derived index / health view:** re-rendered from what I write, never hand-maintained.

**I would not edit `docs/incidents/INC-31.md`** — incident history is a record, not a working document. Its unchecked follow-up already names FEAT-014 as owner; I confirm that's in the delta's scope and leave the file alone.

---

## Phase 7 — Buildability and NFR check before I hand off

Test the shape against the constraints and targets, and write the result into the delta:

- **C-001 (no new services):** ✅ four services in, four services out. `-Q` is a flag, not a service.
- **C-002 (Twilio/Postmark only):** ✅ — and the `push`/`whatsapp` channel enum is gone, so the schema no longer implies otherwise.
- **C-003 (team of three, one release):** ✅ — modified modules and two tables. The draft's version needed a new service, a polling loop, a retry engine, and a duplicated rule; that is not a one-release change for three people.
- **NFR-001 (60 s p95 to provider, measured in the worker):** met *given* queue separation; the measurement point is the observability row from Phase 3D. If that row is refused, I record NFR-001 as unverifiable rather than met.
- **NFR-002 (decision visible when SMS provider unreachable):** met by commit-then-enqueue; the failure flow in Phase 4 is the proof.
- **NFR-003 (approve p95 < 500 ms):** met once the inline Twilio call is gone. With it, it cannot be met — 1.4 s p95 per INC-31. Stated as a hard blocker on the revert.
- **AX-001:** `business_id` on both new tables, and I'd name `tests/tenancy/test_isolation.py` as the enforcement that must cover them.

**Tests I'd name as the enforcement of this delta** (specified here, written in implementation):
- `tests/notify/test_no_inline_provider.py` — delete `HOTFIX_SKIP` entirely. Expected: passes across all `crewboard.api` modules. Before the revert it should **fail** on `swaps` and `timeoff` — I'd want that failure seen first, so the guard is proven to bite rather than assumed to.
- Tenancy isolation extended to `swap_request` and `swap_event`. Expected: zero rows read cross-business.
- A concurrency test for FR-003: shift reassigned between proposal and approval → approval rejected with a reason, assignees unchanged. This is the INC-27 regression test.
- A test that a provider failure leaves swap state committed and unchanged (FR-005 / NFR-002).

*(No test runs in this phase — I'm specifying them for the design and implementation phases.)*

---

## Phase 8 — Report

To the user, at the end:

1. **The store was wrong in three places and I fixed the record**: AX-003 claimed no drift while two API modules send SMS inline; AX-005's upgrade trigger fired in INC-31 and nobody re-ruled it; the spine's confirmation date was stale.
2. **The draft's core proposal is not buildable**: `notifier` is a fifth service under an explicit budget freeze, and its approve-blocks-on-Twilio path contradicts NFR-003, NFR-002 and FR-005 at once.
3. **The draft was carrying weight nothing pays for**: an outbox, a poll loop, and a retry engine that duplicate Celery; a third copy of an eligibility rule the spec names a single home for; channel config for two channels the spec puts out of scope.
4. **The delta needs zero new containers** — modified modules, two tables, one queue flag.
5. **The one architectural question the draft never asked** — INC-31's single-queue serialisation versus NFR-001 — and the second queue that answers it for the price of a command-line flag.
6. **Six shelf rows opened** and what each was ruled, plus which remain open with my default noted.
7. **The design phase can start** on `specs/FEAT-014-swap-approvals/architecture.md`, with the AX-003 revert as the first thing to land because NFR-003 depends on it.