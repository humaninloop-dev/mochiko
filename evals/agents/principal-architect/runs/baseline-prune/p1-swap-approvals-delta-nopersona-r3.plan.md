FILES-READ: `README.md`, `render.yaml`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-014-swap-approvals/spec.md`, `specs/FEAT-014-swap-approvals/design-draft.md`, `src/crewboard/domain/swaps.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/rotas.py`, `src/crewboard/notify/tasks.py`, `src/crewboard/notify/twilio_client.py`, `src/crewboard/worker/celery_app.py`, `web/src/swap/eligibility.ts`, `tests/notify/test_no_inline_provider.py`, `docs/incidents/INC-31.md`

---

# Action plan — FEAT-014 architecture change

Scope note up front: this card is the **architecture** change so that detailed design can start. I would not edit `src/`, `web/`, or `tests/` in this card. I would write the architecture decision, update the architecture store, and hand the implementers exact work items with file paths — including the test edits — as specified work, not as done work.

## Phase 1 — Establish current state and check the draft against it

**Read** (done): the four sources above plus every code file, so that the draft's "current-state picture" claim is verified rather than trusted.

**What I found that changes the shape of the work:**

1. **The draft's current-state diagram is wrong.** It draws `billing`, `reports`, `payroll-export`, `xero-sync` as separate boxes hanging off `api`. `architecture/spine.md:9-13` says `billing` and `reports` are *modules of `api`*, and `payroll` / `integrations.xero` are *modules of `worker`*. The draft's "the existing boxes stay where they are" is therefore reasoning from a topology we do not have.
2. **The draft contradicts itself**: "No architecture change — this just adds tables and a helper service." A new long-running service *is* the architecture change, and it is the one thing `C-001` forbids.
3. **The draft's `api → Twilio` approve path** ("returns 200 only once Twilio confirms delivery") contradicts `AX-003` (spine.md:32, concerns.md:26-37), NFR-003 (Twilio p95 is 1.4 s per INC-31; the budget is 500 ms), NFR-002 (decision must persist when the provider is unreachable) and FR-005 (a failed notification must not block the state change). It also silently blesses the INC-31 hotfix that INC-31 assigns FEAT-014 to remove.
4. **The real latency problem is in the draft's blind spot.** INC-31's cause was one Celery queue serialising a per-person decision SMS behind a 4,100-message publish burst. The worker resize fixed the *symptom*: the burst now drains in ~6 minutes. NFR-001 is 60 s p95. A swap approval queued at 09:00 on a Monday still misses NFR-001. A polling `notifier` reading a shared outbox reproduces exactly this head-of-line problem with a five-second poll on top. The change that NFR-001 actually needs is **queue separation**, which costs no new service.
5. **AX-005's upgrade trigger has already fired.** `concerns.md:53` sets the trigger at "the first incident in which a recipient receives the same message twice"; INC-31 records two staff at Harbour Kitchen receiving the rota-published SMS twice, and INC-31's third follow-up is open with owner TBD. The ledger row still reads `not-now`. This is live drift in the ledger that I must resolve or explicitly re-rule, not leave.
6. **`can_swap` would become a third copy.** The draft puts eligibility in `notifier` "so it can word the message", plus a copy in `api`, plus the existing hand-mirrored `web/src/swap/eligibility.ts` — which the file's own comment says already drifted once (INC-27). `crewboard/domain/swaps.py:5` already returns `(ok, reason)`; the reason is data to *carry*, not to recompute.
7. **Out-of-scope surface in the data model**: `channel (sms|email|push|whatsapp)` and the whole `notification_channel_config` table serve a 2027 roadmap the spec explicitly excludes ("Out of scope: push, WhatsApp, or any channel other than SMS and email").
8. **FR-006/US-005 is an audit trail**, and `audit` is on the "not yet walked" shelf list (`concerns.md:79-80`). This feature is the occasion to walk it.

## Phase 2 — Rulings I would make

These go into the design document as decisions with rationale, not options.

**R1 — No `notifier` service. Notifications stay in `worker`, per AX-003.** The swap endpoints enqueue Celery tasks after commit, the way `api/rotas.py:19` already does. Grounds: `C-001` (no new services before Q1 2027), `C-003` (three engineers, one release), and AX-003 already rules this ground. Cost admitted honestly: no independent scaling of messaging — acceptable, because the worker was just resized and the queue split below addresses the latency that motivated the draft.

**R2 — Split the worker's single queue into `interactive` and `bulk`.** Per-recipient decision notifications (swaps, time-off) go to `interactive`; fan-out tasks (`send_rota_published`, reminder sweeps, Xero, PayFlow) go to `bulk`. Implemented as **two Celery worker processes inside the one existing Render background worker service** (one start command running a small process manager), so no new Render service is created and `render.yaml` gains no `services:` entry. Beat stays attached to the bulk process only, so it still runs exactly once. This is the change that makes NFR-001 achievable during a publish burst; the current single `celery` queue (`worker/celery_app.py:10`, spine.md:42) cannot.

**R3 — No `notification_outbox` and no `notification_channel_config` table.** Celery already provides retry with backoff and jitter (`notify/tasks.py:9-10`); the draft's `attempts` / `next_attempt_at` / five-attempt backoff reimplements it, and the five-second poll adds latency against NFR-001. The durable record FR-006 requires is `swap_event`, which we are building anyway. The residual risk — commit succeeds, enqueue fails, notification lost — is explicitly tolerated by FR-005 ("the rota is the source of truth and the app shows the current state"). I would state that tradeoff in the design rather than bury it, and note the reopen condition: if we later need guaranteed delivery, the outbox comes back as an AX row, not as a service. Channel config is dropped as out of scope; `channel` is `sms | email` only.

**R4 — `can_swap` in `crewboard.domain.swaps` stays the single server-side authority.** The rejection reason it already returns is passed into the notification task as a string argument. No copy in a messaging component. The `web` mirror stays (it is a UX affordance, not an authority) but gets a shared fixture file of cases that both the Python and TypeScript tests run, so the INC-27-style drift is caught by CI instead of by a manager.

**R5 — FR-003 atomicity.** Approve does, in one transaction: lock both shift rows, re-run `can_swap`, compare each shift's stored version/`updated_at` against the values captured at proposal, exchange assignees, write the `swap_event`, commit. A mismatch ends the swap in `rejected_stale` with the reason. This is the INC-27 double-booking path and is the only part of FR-003 that matters. `api/swaps.py:31-35` re-checks eligibility but does not check staleness today.

**R6 — Revert the INC-31 inline sends and re-enable the AX-003 guard.** Both `api/swaps.py` and `api/timeoff.py`, and the `HOTFIX_SKIP` set at `tests/notify/test_no_inline_provider.py:11` deleted entirely. `timeoff.py` is strictly outside "swap approvals", but the guard test is parametrised over module names and cannot be re-enabled for one module and not the other, and INC-31 names FEAT-014 as the owner of the revert. I would flag this as deliberate scope beyond the spec's title, in one line, in the design.

**R7 — State machine, stated explicitly** (the draft's `declined` vs `rejected` is ambiguous): `proposed → accepted | declined_by_colleague`; `accepted → approved | declined_by_manager | rejected_stale`. Proposer cancellation is not in the spec; I would name it as a known gap for product rather than invent it.

**R8 — Notification matrix (FR-004)**, one helper resolving SMS-or-email per recipient exactly as `notify/tasks.py:28-31` already does: proposal → colleague; acceptance → proposer *and* manager; colleague decline → proposer; manager decision → proposer and colleague.

**R9 — Tenancy.** `swap_request` and `swap_event` carry `business_id`, are reached only through `TenantRepository`, and are added to the table list in `tests/tenancy/test_isolation.py` (AX-001). Named as a work item; that test is not in this checkout, and per README:30 it exists in the full repo.

## Phase 3 — Stops for a human ruling

I would raise these together, in one pass, not serially, and would not block the rest of the work on them.

- **S1 — AX-005 (duplicate provider sends) in or out of this release?** The trigger has fired, so the row cannot stay `not-now` unchanged. *If in:* rule it `decided`, scope it to an idempotency key argument threaded from `send_sms` into the Twilio call, land it with FEAT-014. *If out:* rule it `deferred` with the trigger recorded as fired, an owner, and a dated review — and say plainly in the ledger that we are shipping with a known duplicate-send path. **Default if no answer: rule it `decided` and scope it small**, because "the trigger fired and we did nothing" is the state that ledger rows exist to prevent.
- **S2 — Two worker processes in one Render service (R2) vs. asking finance to break the freeze for a second worker service.** *If the process manager is unacceptable:* the fallback is one worker consuming `-Q interactive,bulk`, which I would document as *not* meeting NFR-001 during a Monday burst, and NFR-001 would need renegotiating with product. **Default: two processes in the one service**, because it satisfies both C-001 and NFR-001 and costs one line of start command plus a Procfile.
- **S3 — R6's `timeoff.py` scope.** *If the team wants FEAT-014 kept to swaps:* the guard test stays skipped for `timeoff` and INC-31's revert follow-up stays open with a named new owner. **Default: revert both**, because a guard test that is skipped is a guard we do not have.
- **S4 — the `web` eligibility mirror (R4).** *If the team would rather delete the mirror and take the round trip:* simpler, one authority, slightly worse UX. **Default: keep the mirror plus the shared fixture test.**

I would also send the analyst a short note on findings 1, 2 and 3 — not as a complaint, but because the draft will otherwise be read by others as an accurate current-state picture.

## Phase 4 — Write the design

**Write `specs/FEAT-014-swap-approvals/design.md`**, containing:

- Current state as verified against `spine.md` and the code, with the corrected component diagram (no `notifier`; `billing`/`reports` inside `api`; `xero`/`payroll` inside `worker`; the `interactive`/`bulk` queue split shown inside the one `worker` box).
- The rulings R1–R9 with rationale and the constraint or requirement each answers.
- Data model: `swap_request` (id, business_id, venue_id, proposer_shift_id, colleague_shift_id, proposer_shift_version, colleague_shift_version, state, created_at) and `swap_event` (id, business_id, swap_request_id, actor_id, from_state, to_state, reason, at). No outbox, no channel config.
- Sequence for the three transitions, with the approve path annotated against the 500 ms budget (NFR-003): DB work in-request, provider work in `worker`.
- A section "What changed from the analyst's draft and why", covering all eight findings from Phase 1, so the analyst gets the reasoning and not just a rewrite. (Kept as a section rather than a separate file — one document, one place to look.)
- Open gaps handed back to product: proposer cancellation; what the manager sees when a swap is rejected as stale.

## Phase 5 — Update the architecture store

**Write `architecture/spine.md`:**
- `worker` line: two queues, `interactive` and `bulk`, two processes in the one Render service, beat on `bulk`; supersede line 42 ("one queue (`celery`) with concurrency 8") with the date of this change.
- `api` modules line: `swaps` gains the colleague step; no new container.
- Communication styles: extend the `api → worker` line — currently "rota-publish notifications (FEAT-006), shift reminders (FEAT-009), **nothing else today**" — to include swap and time-off decision notifications.
- Re-date "Last confirmed".

**Write `architecture/concerns.md`:**
- **AX-003**: record the drift honestly — as-built has diverged since 2026-08-20 (INC-31 hotfix, two modules, guard skipped); add the queue-separation ruling and the revert under **Work**; add the NFR-001 measurement point (task-enqueued-to-provider-call timestamp, measured in the worker) to **Enforcement**. Drift returns to `none` only when the guard test's skip list is gone.
- **AX-005**: re-ruled per S1 (default: `decided`, narrow scope).
- **AX-007** (event bus): reaffirm `not-now` with a line explaining that the draft's outbox-plus-poller was considered and rejected — the fan-outs here still have one consumer, so the trigger has not fired.
- **New AX-009 Audit trail**: ruled from FR-006/US-005 — append-only `swap_event` with actor and timestamp, tenant-scoped, no deletes; enforcement test named. Move `audit` from the unwalked list to walked.
- **Shelf coverage**: `observability` stays unwalked, but note the one metric AX-003 now depends on so it is not lost.

## Phase 6 — Specify the work items for the design/build phase

Listed in `design.md` with exact paths, for implementers to execute (not done in this card):

| Work item | Path |
|---|---|
| Delete `HOTFIX_SKIP`, re-enable guard for both modules | `tests/notify/test_no_inline_provider.py` |
| Remove inline Twilio call and its import; enqueue on `interactive` | `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py` |
| Queue routing, second process, beat on `bulk` | `src/crewboard/worker/celery_app.py`, `render.yaml` start command + new `Procfile` |
| `notify_recipient` helper (SMS with email fallback), routed tasks | `src/crewboard/notify/tasks.py` |
| Colleague accept/decline endpoint; staleness check + atomic exchange | `src/crewboard/api/swaps.py` |
| Add `swap_request`, `swap_event` to the tenant isolation table list | `tests/tenancy/test_isolation.py` |
| Shared eligibility fixture + parity test both sides | new fixture consumed by a Python test and by `web/src/swap/eligibility.ts`'s test |
| New: `tests/swaps/test_exchange_atomic.py` — concurrent approve of two swaps touching one shift leaves no double-booking (INC-27) | new |
| New: `tests/notify/test_decision_routing.py` — decision tasks land on `interactive`, fan-out on `bulk` | new |

**Expected results when these run:** the AX-003 guard passes with no skips (it fails today only because two modules are skipped, and would fail outright on `api/swaps.py:8` if the skip were removed before the revert); the atomicity test fails against today's `decide()` and passes after R5; the tenancy test passes on the new tables.

## Phase 7 — Delegation

None. This is a small, judgment-heavy piece of architecture across roughly fifteen files that I have already read in full; splitting it across workers would cost more in briefing and cross-checking than it saves, and the substance is a set of rulings that need one consistent voice in the ledger. (I also would not spawn workers in this run regardless.)

## Phase 8 — What I would report

- The draft's `notifier` service is refused on `C-001`, and the draft's `api → Twilio` approve path is refused on AX-003, NFR-002, NFR-003 and FR-005 — with the reasoning, so the analyst can push back on the reasoning rather than on the verdict.
- The change FEAT-014 actually needs is a **queue split inside the existing worker**, not a new service; that is what makes NFR-001 survivable during the Monday publish burst that caused INC-31.
- **AX-005's upgrade trigger fired at INC-31 and the ledger has not caught up** — flagged as the one finding that outlives this feature, with S1 awaiting a ruling and my default stated.
- Two deliberate scope calls: `api/timeoff.py` is touched though the feature is about swaps (S3), and `audit` gets walked as AX-009 because FR-006 forces it.
- Three things left for others: the four decisions in Phase 3 are stated with defaults and would proceed on those defaults if nobody rules; proposer cancellation and stale-rejection UX go back to product; no code was changed in this card, and Phase 6 is a specification of work, not work completed.