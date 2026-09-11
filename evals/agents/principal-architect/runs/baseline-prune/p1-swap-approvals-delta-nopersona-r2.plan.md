FILES-READ: `README.md`, `render.yaml`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-014-swap-approvals/spec.md`, `specs/FEAT-014-swap-approvals/design-draft.md`, `src/crewboard/domain/swaps.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/rotas.py`, `src/crewboard/notify/tasks.py`, `src/crewboard/notify/twilio_client.py`, `src/crewboard/worker/celery_app.py`, `web/src/swap/eligibility.ts`, `tests/notify/test_no_inline_provider.py`, `docs/incidents/INC-31.md`

---

# Action plan — FEAT-014 architecture change

## Headline finding (drives everything below)

The analyst's draft says "no architecture change", but it proposes a **new Render service** (`notifier`), which C-001 forbids until Q1 2027, and it proposes **`api` calling Twilio inline and blocking on delivery confirmation**, which is the exact INC-31 hotfix that FEAT-014 is on the hook to *revert* (AX-003, and INC-31 follow-up 2). Meanwhile the draft misses the one change that is genuinely needed: after INC-31 the single `celery` queue still serialises an interactive swap notification behind the Monday publish burst (drains in ~6 minutes post-resize), which breaks NFR-001's 60 s p95. So the real architecture change for FEAT-014 is queue isolation inside the existing worker, not a new box. My plan is built around that.

---

## Phase 1 — Establish current state and confirm the draft's premises

**Do:** re-read the pieces the design turns on and pin the facts I will cite in the design doc: the topology and communication rules (`architecture/spine.md`), the eight ledger rows and their enforcement tests (`architecture/concerns.md`), the deployed service set and budget note (`render.yaml`), the good pattern (`src/crewboard/api/rotas.py` → `send_rota_published.delay`), the bad pattern (`src/crewboard/api/swaps.py:38-45`, `src/crewboard/api/timeoff.py:6,18-19`), the task set and queue config (`src/crewboard/notify/tasks.py`, `src/crewboard/worker/celery_app.py`), the eligibility rule and its hand-kept mirror (`src/crewboard/domain/swaps.py`, `web/src/swap/eligibility.ts`), the skipped guard (`tests/notify/test_no_inline_provider.py`), and INC-31 with its two open follow-ups.

**Write:** nothing yet.

**Flag:** `README.md:30` says this checkout is a slice — `tests/tenancy/test_isolation.py` (AX-001's enforcement) and `crewboard.notify.postmark_client` are cited but absent. I will treat them as existing and not "fix" their absence, and I will say so in the design doc so the implementer doesn't recreate them.

---

## Phase 2 — Write the verdict on the analyst's draft

**Do:** produce a point-by-point response so the analyst gets a reasoned answer, not a silent rewrite. Verdicts I would record:

1. **`notifier` service — rejected.** Breaks C-001 (no new services until Q1 2027) and C-003 (three engineers, one release). It also duplicates what `worker` already does under AX-003: Celery *is* the decoupling. A 5 s poll loop is strictly worse than `.delay()` for NFR-001.
2. **`api` → Twilio direct, blocking on delivery confirmation — rejected.** Violates AX-003's "never" rule; contradicts FR-005 (a failed notification must not block the state change), NFR-002 (decision persisted and visible when Twilio is unreachable), and NFR-003 (500 ms p95 against a provider whose p95 is 1.4 s per INC-31). It re-entrenches the hotfix FEAT-014 owns removing.
3. **Eligibility check owned by `notifier`, with copies in `api` — rejected.** FR-002 names one rule: `crewboard.domain.swaps.can_swap`. Three server-side copies is three drift surfaces; the mirror in `web/src/swap/eligibility.ts` has already drifted once (INC-27 follow-up). The reason string is produced once at the state change and carried in the notification payload; nothing recomputes it to word a message.
4. **`notification_outbox` — rejected for this release** (detail in Phase 3.3).
5. **`notification_channel_config` and `channel ∈ {push, whatsapp}` — rejected.** Both are explicitly out of scope; C-002 gives one SMS and one email provider under contract. Speculative per-tenant provider config is a schema and a config surface bought for a 2027 roadmap item.
6. **`swap_request` and `swap_event` — accepted**, with corrections (Phase 3.4).
7. **"No architecture change" — wrong in both directions.** The draft adds a container that isn't allowed; the change that is needed is a queue split plus retiring a live deviation.

**Write:** `specs/FEAT-014-swap-approvals/design-review.md` — the verdict above, addressed to the analyst, each rejection tied to the constraint or ledger row it hits.

---

## Phase 3 — Decide the design

### 3.1 Topology: no new container; split the queue inside `worker`

Add a second Celery queue — `interactive` — for state-change notifications (swap proposed/accepted/approved/declined, time-off decided). Bulk fan-out (`send_rota_published`, `send_shift_reminder`, `sweep_reminders`) stays on `celery`. The **same** worker process consumes both (`-Q celery,interactive`), so no new Render service and no cost change: C-001 is satisfied by the letter and the intent.

Why this is the necessary change and not gold-plating: INC-31's cause line is explicit — "the single `celery` queue serialised every other task behind the burst." The resize cut the burst to ~6 minutes, which is why the incident stopped hurting, but 6 minutes is still ~6× NFR-001's 60 s budget for a swap notification unlucky enough to land at 09:00 Monday. Priority routing, not more concurrency, is what makes NFR-001 hold at p95.

Points to settle in the design doc: dedicated worker `-Q` ordering and whether to reserve concurrency; `task_routes` in `crewboard/worker/celery_app.py` keyed by task name; `render.yaml:14` start command updated. I would specify routing by task name (`notify.tasks.send_sms_interactive` / a `queue=` kwarg on `apply_async`) rather than splitting `send_sms` into two functions, so the provider call stays in one place.

### 3.2 Flow for each state change

`web → api` (HTTPS/JSON, session cookie) → validate role and tenancy → apply the state change and write the audit row **in one transaction** → commit → `apply_async(queue="interactive")` per recipient, after commit. Provider calls only from `worker`. SMS when `recipient.mobile` is set, Postmark otherwise (FR-004's fallback), mirroring the branch already in `send_rota_published` at `src/crewboard/notify/tasks.py:29-31`. NFR-003 becomes trivially achievable once the 1.4 s Twilio call leaves the request.

### 3.3 No outbox table — recorded as a deliberate "not-now"

Enqueue-after-commit has a real dual-write hole: commit succeeds, Redis enqueue fails, notification never sent. A transactional outbox closes it. I am **not** taking it this release: FR-005 already states the rota is the source of truth and the app shows current state, so a lost notification is a degraded notice, not lost data or a wrong rota; and the poll loop the draft wants costs a service we cannot have. I will record it as a ledger row with a stated upgrade trigger rather than leave it as an unexamined omission.

### 3.4 Data model

- `swap_request` — `id`, `business_id`, `venue_id`, `proposer_shift_id`, `colleague_shift_id`, `state`, `created_at`. Add `business_id` scoping via `TenantRepository` per AX-001. States: `proposed → accepted|declined → approved|declined|rejected`; the draft's flat enum is fine but the *legal transitions* need to be written down, including who may drive each (staff colleague for accept/decline, manager for approve/decline) — US-002 and US-003 are different actors and the current `decide()` has no colleague step at all.
- `swap_event` — `id`, `business_id` (added; AX-001 says every tenant table), `swap_request_id`, `actor_id`, `from_state`, `to_state`, `at`. Satisfies FR-006 and US-005. This is append-only; the design doc will say so.
- **Drop** `notification_outbox` and `notification_channel_config`.

### 3.5 FR-003 atomicity and the stale-swap rejection

One transaction: re-check `can_swap`, verify both shifts are unchanged since the proposal, exchange assignees, write the `swap_event`. "Unchanged" needs a concrete mechanism — I would specify capturing each shift's version/`updated_at` on `swap_request` at propose time and comparing under a row lock at approve time (`SELECT … FOR UPDATE` on both shift rows, ordered by id to avoid deadlock between two managers). On mismatch: state `rejected`, reason recorded, 409 to the manager, notifications still fire. This is the INC-27 double-booking guard, so it gets named as such.

### 3.6 The eligibility mirror

Keep `web/src/swap/eligibility.ts` as-is for button state; do **not** add a third copy. Note in the design doc that the server rule is authoritative and the client copy is advisory, and record the drift risk as a known, accepted cost rather than pretending the hand-sync note isn't a hazard. Not expanding scope to unify them.

---

## Phase 4 — Update the concern ledger

**Read:** `architecture/concerns.md`. **Write:** same file.

- **AX-003 Outbound messaging** — currently claims "Drift: none", which is false: `src/crewboard/api/swaps.py:8,42-45` and `src/crewboard/api/timeoff.py:6,18-19` call Twilio from request handlers and `tests/notify/test_no_inline_provider.py:11` skips both. I would set **Drift: yes (INC-31 hotfix, 2026-08-20)**, keep the ruling unchanged, and put the revert under **Work** owned by FEAT-014. The ruling itself needs no change — it was right, and INC-31 was a queueing failure, not a signal that the rule was wrong.
- **AX-005 Provider-call idempotency** — its stated upgrade trigger is "the first incident in which a recipient receives the same message twice." INC-31 records exactly that: two Harbour Kitchen staff got the publish SMS twice. **The trigger has fired**; leaving this at "not-now" would be stale. See the stop in Phase 6.
- **AX-007 Event bus** — check and re-affirm: FEAT-014's fan-out is one task per recipient, one consumer, finishing well inside its window. No trigger. Record "re-checked at FEAT-014, unchanged" so the next walk knows it was considered.
- **New AX-009 Notification latency isolation** — Stance: decided · Status: ruled. Ruling: interactive state-change notifications run on a separate Celery queue consumed by the same worker; bulk fan-out keeps the default queue; no new service (C-001). Rationale: INC-31; NFR-001's 60 s p95 cannot survive being queued behind a 4,100-message burst on one FIFO queue. Enforcement: a routing test (Phase 5).
- **New AX-010 Audit trail** — the shelf lists audit as not-yet-walked and FEAT-014 introduces the first audit table. Ruling: per-domain append-only event tables (`swap_event`), tenant-scoped, no global audit log and no retention policy yet. Status: ruled. This keeps the shelf honest without buying a cross-cutting audit subsystem for one feature.
- **Shelf coverage** — move audit to walked; leave observability, rate limiting, feature flags, data retention as not-yet-walked.

---

## Phase 5 — Name the enforcement work (design phase specifies; implementation writes)

I would not write code in this phase, but each ruling ships with the test that holds it, named and specified:

- `tests/notify/test_no_inline_provider.py` — delete `HOTFIX_SKIP` (lines 10-11, 18-19). **Expected before the revert: two failures** (`crewboard.api.swaps`, `crewboard.api.timeoff`). **After: green.** This un-skip is the acceptance gate for INC-31 follow-up 2.
- `tests/notify/test_queue_routing.py` (new) — asserts swap and time-off state-change sends are enqueued on `interactive` and rota-publish/reminder tasks on `celery`. Expected to fail until `task_routes` lands.
- `tests/swaps/test_approval_concurrency.py` (new) — a shift edited between propose and approve yields state `rejected` with a reason, no assignee exchange, a `swap_event` row written (FR-003, INC-27).
- `tests/swaps/test_notification_failure.py` (new) — Twilio raising does not roll back or alter the persisted decision, and the endpoint still returns the new state (FR-005, NFR-002).
- `tests/tenancy/test_isolation.py` — extend to cover `swap_request` and `swap_event` (AX-001). Not in this checkout; the ticket says "extend", not "create".
- State-machine tests for the transition table, including that a staff member cannot approve and a manager cannot accept on the colleague's behalf.

---

## Phase 6 — Stops for a human ruling

**Stop 1 — AX-005 idempotency (the one I would genuinely hold on).** The ledger's own trigger has fired, so the row must move; *how far* it moves is a product/eng-lead call, not mine alone, and INC-31 follow-up 3 has owner TBD.
- If the ruling is **adopt now**: AX-005 becomes decided/ruled — a dedupe key per (recipient, state-change) persisted with the send and passed to Twilio, with an enforcement test; add it to FEAT-014's scope and flag the ~half-day cost against C-003's one-release constraint.
- If the ruling is **re-defer**: AX-005 stays not-now but the row must be rewritten — the old trigger is spent, so a new one is needed (e.g. a duplicate reaching a *manager decision* notification, or a second duplicate incident), plus a note that INC-31's duplicates were accepted as tolerable.
- **My default while waiting:** design so the dedupe key is cheap to add later — the interactive send tasks take an explicit key argument even if it is only logged this release — and record AX-005 as "trigger fired, ruling pending, owner: eng lead." I would not silently leave the row saying "we have never seen one" when the incident log says otherwise.

**Stop 2 — is reverting `api/timeoff.py` in FEAT-014's scope?** It is a different feature's endpoint, but it carries the same hotfix and INC-31 names FEAT-014 as the owner of the revert.
- If **yes** (my default): both modules revert together and the guard's skip list is deleted outright — a two-line change plus routing the time-off send through `send_sms.apply_async(queue="interactive")`.
- If **no**: revert `swaps.py` only, keep `timeoff.py` in `HOTFIX_SKIP`, and AX-003 stays in drift with a named owner and date. I would push back once, because a half-reverted rule with a permanently skipped guard is how the next INC-31 hides.

**Stop 3 — `render.yaml` start-command change** (`-Q celery,interactive`). Small, but it is a deploy-config edit on a live service; I would flag it for whoever owns the Render blueprint and confirm the rollout order: **routing config and worker must ship before or with the first interactive enqueue**, otherwise messages land on a queue nobody consumes. Default: ship the worker's `-Q` change first, then the routing.

**Nothing here warrants a refusal.** The only thing I decline outright is implementing the draft as written — the new service and the inline blocking Twilio call — and that is a design verdict with reasons, recorded in Phase 2, not a refusal of the task.

---

## Phase 7 — Deliverables

**Files I would write:**
- `specs/FEAT-014-swap-approvals/design.md` — the design of record: rejected options with reasons; target topology and the corrected component diagram (no `notifier` box; `worker` shown consuming two queues); the state machine with actors; `swap_request` / `swap_event` schemas with `business_id`; the per-transition notification matrix for FR-004; the atomicity and stale-shift mechanism for FR-003; how each of NFR-001/002/003 is met and where it is measured; the enforcement tests from Phase 5; open questions from Phase 6.
- `specs/FEAT-014-swap-approvals/design-review.md` — the reply to the analyst.
- `architecture/spine.md` — add the `interactive` queue to the `worker` entry and the communication-styles list; extend the `api → worker` line beyond "nothing else today" to include swap and time-off state changes; update "Last confirmed".
- `architecture/concerns.md` — AX-003 drift + work, AX-005 trigger-fired note, AX-007 re-check, new AX-009 and AX-010, shelf coverage.
- `docs/incidents/INC-31.md` — tick follow-up 2 only when the revert actually lands (implementation phase, not now); update follow-up 3's owner per Stop 1.

**Delegation:** none. This is a single-context reading-and-judgement task across ~16 small files; farming out the ledger or the design doc to another worker would cost more in briefing and review than doing it, and the judgement calls here (which draft proposals to reject, whether AX-005's trigger has fired) are exactly what shouldn't be delegated.

**Final report to the user:** the draft's central proposal is not buildable under C-001 and its inline-Twilio step contradicts AX-003 and three of the spec's own requirements; FEAT-014's actual architecture change is a second Celery queue in the existing worker plus retiring the INC-31 hotfix; five files written or amended, two ledger rows added and two revised; one decision (AX-005 idempotency) is waiting on an eng-lead ruling with a stated default; the design phase can start on the state machine and schema immediately since neither depends on that ruling.