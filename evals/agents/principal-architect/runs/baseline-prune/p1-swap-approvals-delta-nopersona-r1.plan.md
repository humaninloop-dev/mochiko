FILES-READ: `README.md`, `specs/FEAT-014-swap-approvals/spec.md`, `specs/FEAT-014-swap-approvals/design-draft.md`, `architecture/spine.md`, `architecture/concerns.md`, `src/crewboard/domain/swaps.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/rotas.py`, `src/crewboard/notify/tasks.py`, `src/crewboard/notify/twilio_client.py`, `src/crewboard/worker/celery_app.py`, `web/src/swap/eligibility.ts`, `tests/notify/test_no_inline_provider.py`, `docs/incidents/INC-31.md`, `render.yaml`, `architecture/spine.md`

---

# Action plan — FEAT-014 architecture change

## Framing

The card is architecture work, not build work: the output is a design decision record plus an updated architecture store, precise enough that three engineers can start the design/build phase from it. I would not write feature code or migrations in this pass; I would name the files that must change and the tests that must hold, and leave the writing to the build phase.

One thing to say up front to whoever handed me this: the draft's closing claim, "No architecture change — this just adds tables and a helper service," is wrong on its own terms. It adds a container, a new communication style (a service polling the database), and it reverses a standing ruling that the API never calls a provider inline. Meanwhile the change FEAT-014 actually needs — undoing the INC-31 hotfix and fixing the queue contention that caused it — the draft does not mention. I proceed with the full task; this is a correction to the draft, not a reason to stop.

---

## Phase 1 — Reconcile the store against the code, and record the drift

**Do:** compare what `architecture/spine.md` and `architecture/concerns.md` claim against what `src/` actually does, before designing anything on top of a picture that is out of date.

**Already read; findings I would carry forward:**

- `concerns.md` AX-003 says "**Drift**: none" and "**Work**: —". That is false as of 2026-08-20. `src/crewboard/api/swaps.py:8` and `src/crewboard/api/timeoff.py:6` both import `crewboard.notify.twilio_client`, and `api/swaps.py:42-45` calls Twilio inside the request handler. `tests/notify/test_no_inline_provider.py:11` skips both modules. The ledger's own enforcement test has been disarmed and the ledger does not say so.
- `concerns.md` AX-005 (provider-call idempotency, stance *not-now*) states its upgrade trigger as "the first incident in which a recipient receives the same message twice." `docs/incidents/INC-31.md:5` records exactly that — two staff at Harbour Kitchen got the rota-published SMS twice. The trigger has fired and the row was never revisited.
- `INC-31.md:22` assigns the revert of the inline sends to FEAT-014 by name. This card owns that work; it is not optional scope.
- `spine.md:42` ("one queue, concurrency 8") is current, but the single-queue design is the direct cause of INC-31: a 4,100-message publish burst serialised every interactive notification behind it. FEAT-014's NFR-001 (60 s to provider at p95) lands squarely on that.
- `spine.md` "Last confirmed 2026-07-02" predates both the hotfix and the worker resize, so the whole file is stale by two known events.

**Write:** nothing yet. These become ledger edits in Phase 4.

**Flag:** the store said "no drift" while a ruling was actively bypassed in production code. I would call that out in the final report as a process point — a hotfix that suspends a ruling should land a drift row the same day, not wait for the next feature to notice.

---

## Phase 2 — Rule on the analyst's draft, element by element

**Do:** go through `design-draft.md` and accept, amend, or reject each proposal against the spec's constraints. I would write these rulings into the design doc so the analyst can see the reasoning, not just the verdict.

| Draft proposal | Ruling | Reason |
|---|---|---|
| New `notifier` container | **Reject** | C-001 and `render.yaml:2` freeze new services until Q1 2027. Also a third deployable for a three-person team (C-003) with a feature that must ship in one release. The work belongs in the existing `worker`. |
| Poll `notification_outbox` every 5 s | **Reject as designed** | Polling exists to replace a queue; we already have a queue (`redis`, Celery). Adding DB polling next to Celery is two transports for one job. Also a 5 s poll plus five-attempt backoff is a slower path to the provider than `delay()`, against NFR-001. |
| `push` / `whatsapp` channels, `notification_channel_config` | **Reject** | The spec's out-of-scope section names push and WhatsApp explicitly, and C-002 makes Twilio and Postmark the only contracted providers. Per-business provider settings for channels we cannot send on is schema we would migrate and maintain for a 2027 maybe. |
| `notifier` owns the eligibility check (FR-002) | **Reject** | This would make a *third* copy of `can_swap`. There are already two (`src/crewboard/domain/swaps.py`, `web/src/swap/eligibility.ts`) and the mirror has drifted once already — `eligibility.ts:1-2` says so in a comment, and the client copy already differs from the server: it checks only the current user's role and hours, never the colleague's. The rejection *reason* is a string the domain function already returns; the notifier should be handed that string, not recompute it. |
| `api` calls Twilio inline on approve and returns 200 only after delivery confirmation | **Reject, firmly** | Three separate failures. It re-enshrines the INC-31 hotfix as permanent design against AX-003. It cannot meet NFR-003 (500 ms p95) when Twilio's p95 per send is 1.4 s (`INC-31.md:9`). And it directly contradicts FR-005 and NFR-002, which say an undeliverable notification must not block the state change — blocking the 200 on delivery does exactly that. |
| `swap_request`, `swap_event` tables | **Accept, amended** | Good shape. Amendments in Phase 3: `business_id` on `swap_event` too (AX-001), shift version columns for FR-003, a `reason` column for FR-003/FR-005 visibility. |
| `dedupe_key` for send idempotency | **Accept, relocated** | Right idea, wrong home. It belongs to AX-005, whose trigger INC-31 already fired, and it applies to *all* sends (the duplicates were rota-publish, not swaps) — not to an outbox table that isn't being built. |
| "No architecture change" | **Reject** | There is one, described in Phase 5. |

**Stop / confirm:** I would put the `notifier` rejection to whoever can speak for the budget line before the design doc is final — the confirmation being "C-001 still holds; no third Render service before Q1 2027." **If confirmed (my default):** proceed as below. **If the freeze were lifted:** I would *still* not build `notifier` for this feature — a separate service for a fan-out of at most three SMS per state change is not earned — but I would reopen AX-007 (event bus) as a genuine option and note it as a future path rather than a rejection.

---

## Phase 3 — Design the swap domain

**Read again while doing this:** `src/crewboard/domain/swaps.py`, `src/crewboard/api/swaps.py`, `web/src/swap/eligibility.ts`.

**Decide and document:**

1. **State machine.** Keep the spec's five states. `proposed → accepted | declined`, `accepted → approved | declined | rejected`. Who declined is recoverable from `swap_event.actor_id` and `from_state`, so no extra states are needed. `rejected` is reserved for FR-003 (shifts moved under the proposal), which is a system rejection, not a person's decision — I would say that explicitly, because the draft's state list leaves it ambiguous.

2. **Data model (revised from the draft):**
   - `swap_request` — id, `business_id`, `venue_id`, `rota_week`, proposer_id, proposer_shift_id, colleague_id, colleague_shift_id, state, `proposer_shift_version`, `colleague_shift_version`, created_at, updated_at.
   - `swap_event` — id, `business_id`, swap_request_id, actor_id, from_state, to_state, `reason`, at. Append-only; this is FR-006 and US-005, and it is also the record the notification sweep reads.
   - `notification_send` — id, `business_id`, `dedupe_key` (unique), channel, recipient_id, provider_message_id, sent_at. This is AX-005's mechanism. It is **not** the draft's outbox: nothing polls it, Celery remains the transport, it has no channel config, and it holds no payload.
   - Dropped: `notification_outbox`, `notification_channel_config`.

3. **FR-003 atomicity.** Approve runs in one transaction: lock both shift rows, compare their versions against the ones stored on the proposal, re-run `can_swap`, exchange assignees, write the `swap_event`, commit. A version mismatch commits `state = rejected` with the reason instead. This is what stops the INC-27 double-booking from coming back through a stale proposal.

4. **FR-002, one source of truth.** `crewboard.domain.swaps.can_swap` stays the only server-side implementation; the API calls it at propose, at accept, and again at approve. The `web` mirror stays (it is a UX affordance, and rewriting the SPA's early button state is out of scope here), but I would stop calling it "kept in sync by hand" and pin it: a shared fixture file of eligibility cases exercised by both the Python and the TypeScript test. I would also note in the design doc that the client copy is currently *not* a mirror — it omits the colleague-side role and hours checks — so the build phase either aligns it or documents it as deliberately one-sided.

**Write (Phase 6):** all of this into the design doc. No schema or code files written in this pass.

---

## Phase 4 — Update the concern ledger

**Write to `architecture/concerns.md`:**

- **AX-003 (outbound messaging)** — change `**Drift**` from "none" to a recorded drift: since 2026-08-20 `api/swaps.py` and `api/timeoff.py` call Twilio inline, guard test skipped, per INC-31. Change `**Work**` from "—" to: revert both inline sends, delete `HOTFIX_SKIP` from the guard test, owned by FEAT-014. The ruling itself is unchanged and stands — I am not reopening it, and I would say so plainly, because the draft proposed to break it a second time.
- **AX-005 (provider-call idempotency)** — flip stance from *not-now* to **decided**, status *ruled* (not yet built). The trigger fired at INC-31. Ruling: every provider send carries a caller-supplied dedupe key recorded in `notification_send`; a send whose key is already present is a no-op. Rationale cites the Harbour Kitchen duplicates and the `acks_late` + timeout-then-retry path in `notify/tasks.py:9-12` that produced them. Enforcement: `tests/notify/test_send_idempotency.py`.
- **New row, AX-009 Queue isolation** — stance *decided*, status *ruled*. Ruling: interactive notifications (swap and time-off state changes) run on a queue separate from bulk fan-out (rota publish, reminder sweep, Xero, PayFlow), served by separate worker pools inside the one existing Render worker service. Rationale: INC-31's root cause was head-of-line blocking on the single `celery` queue; FEAT-014's NFR-001 puts a 60 s p95 on precisely the traffic that got stuck behind the burst. Enforcement: `tests/notify/test_queue_routing.py`.
- **AX-007 (event bus)** — leave at *not-now*, and record *why* FEAT-014 does not trip it: each state change fans out to at most three recipients, has one consumer, and completes inside a single task. This is worth writing down so the next person does not re-litigate it.
- **Shelf coverage** — add a line that observability is still unwalked and that NFR-001 needs a worker-side p95 measurement that does not exist today (see Phase 7 open item).

**Stop / confirm:** AX-009's shape is the one genuine engineering judgement call I would put to the team. Confirming: *two Celery pools in one Render service*, e.g. an interactive pool at concurrency 4 and a bulk pool at concurrency 8 with beat, both started from the one `startCommand` in `render.yaml:14`. **Alternative branch A** — a single pool consuming `-Q interactive,bulk`: cheaper, but a 4,100-task burst can still occupy all eight slots, so it does not actually solve INC-31; I would only take it if memory on the standard plan cannot hold two pools. **Alternative branch B** — Celery task priorities on the one queue: works on Redis only approximately, and depends on prefetch behaviour that is easy to get subtly wrong. **My default is two pools**, with the memory headroom checked against the standard plan during the build phase, and branch A as the documented fallback if it does not fit.

---

## Phase 5 — Update the spine

**Write to `architecture/spine.md`:**

- Containers: no new ones. `api` modules unchanged in count; note `swaps` gains the accept step. `worker` gains `notify.swaps`.
- Communication styles: amend the `api → worker` line — it currently says "rota-publish notifications (FEAT-006), shift reminders (FEAT-009), **nothing else today**." Add swap and time-off state-change notifications (FEAT-014). This sentence is the load-bearing edit; it is what makes the revert of the hotfix a stated architectural fact rather than a cleanup task someone might drop again.
- The `api → Twilio / Postmark: **never**` line stays exactly as it is.
- Boundaries: replace "one queue (`celery`) with concurrency 8" with the two-queue arrangement from AX-009, dated.
- Update "Last confirmed" to today, 2026-09-09, and note the two events that had gone unrecorded (2026-08-20 hotfix, 2026-08-28 resize).
- Reproduce a corrected component diagram: no `notifier` box, no push/WhatsApp arrows, notifications flowing `api → redis → worker → Twilio/Postmark`. The draft's diagram (`design-draft.md:40-50`) also shows `billing`, `payroll-export`, `xero-sync` and `reports` as separate boxes; per `spine.md:8-13` those are modules inside `api` and `worker`, not containers. I would fix that rather than let a wrong topology propagate — and mention it to the analyst, since it is probably where the instinct to add a fourth service came from.

---

## Phase 6 — Write the design document

**Write:** `specs/FEAT-014-swap-approvals/design.md`, structured as:

1. What is being built, in two paragraphs, traced to the user stories.
2. The rulings table from Phase 2, with reasons — addressed to the analyst.
3. Target design: state machine, data model, the approve transaction (FR-003), the notification path.
4. Requirement trace: every FR and NFR to the mechanism that satisfies it. Specifically — FR-004 to the per-state-change task and its recipient matrix; FR-005/NFR-002 to "commit first, enqueue after"; NFR-001 to the interactive queue; NFR-003 to an approve handler that does database work only.
5. The INC-31 revert, called out as in-scope deliverable with both files named.
6. Concern-ledger deltas (AX-003 drift, AX-005 flip, AX-009 new) and the spine deltas.
7. Enforcement tests the build phase must land (Phase 7).
8. Open items and the decision points from Phases 2 and 4 with their defaults.

**Also write:** a short "architect's response" section at the top of the design doc rather than editing `design-draft.md` — the draft is the analyst's artifact and dated; I would leave it intact as the record of what was proposed.

---

## Phase 7 — Specify the enforcement tests

These are specified now and written in the build phase. I would not run anything in this pass — and I would note that this checkout is a slice (`README.md:30`), so `tests/tenancy/test_isolation.py`, cited by AX-001, is not present here and exists in the full repo.

| Test | What it asserts | Expected before / after the build phase |
|---|---|---|
| `tests/notify/test_no_inline_provider.py` (edit: delete `HOTFIX_SKIP` and its two entries, lines 10-11 and 18-19) | No module under `crewboard.api` imports a provider client | **Fails** the moment the skip is removed, until `api/swaps.py` and `api/timeoff.py` are reverted. Then passes. This failure is the point — it is the ledger's guard coming back online. |
| `tests/notify/test_queue_routing.py` (new) | Every discovered task has an explicit queue route; swap/time-off state-change tasks route to `interactive`; publish fan-out, reminder sweep, Xero, PayFlow route to `bulk` | Fails until routing is configured in `worker/celery_app.py` |
| `tests/notify/test_send_idempotency.py` (new) | Two sends with the same dedupe key produce one provider call; a Celery retry after a provider timeout does not re-send | Fails until `notification_send` and the key check exist. This is the test AX-005 has been missing since INC-31. |
| `tests/swaps/test_approval_atomicity.py` (new) | Approving a proposal whose shift versions moved commits `rejected` with a reason and leaves both shifts untouched; a successful approve exchanges both assignees or neither | FR-003; this is the INC-27 regression guard |
| `tests/swaps/test_state_machine.py` (new) | Only the legal transitions are accepted; every transition writes a `swap_event` with actor and timestamp | FR-006, US-005 |
| `tests/swaps/test_eligibility_cases.py` + `web/src/swap/eligibility.test.ts` (new), over a shared `tests/swaps/eligibility_cases.json` | Both implementations agree on the shared cases, or the divergence is asserted explicitly | Replaces "keep in sync by hand"; I expect the first run to surface the colleague-side gap in `eligibility.ts` |
| `tests/tenancy/test_isolation.py` (extend, full repo) | The three new tables are scoped by `business_id` | AX-001 |

**Open item I would flag rather than solve here:** NFR-001 says p95 within 60 s "measured in the worker," and there is no such measurement today — observability is on the unwalked list in `concerns.md:79-80`. I would record this as a gap with a named owner rather than quietly design a metrics stack into a feature card. Without it the NFR is unverifiable, which is worth the product owner knowing before the release rather than after.

**Second open item:** INC-31's third follow-up ("duplicate sends: idempotency key, owner TBD") is answered by AX-005 in this plan; I would say so in the report so that line can be closed rather than left orphaned.

---

## Phase 8 — Scope check before finishing

The time-off revert (`src/crewboard/api/timeoff.py`) is not a shift-swap change, so strictly it sits outside FEAT-014's spec. I include it because `INC-31.md:22` names FEAT-014 as its owner and because leaving one module skipped in the guard test means AX-003 stays half-disarmed and the ledger stays untrue. **If someone rules it out of scope:** I would still remove `crewboard.api.swaps` from the skip list, keep `timeoff` in it, and convert the remaining skip into a dated drift row in AX-003 with a named owner — an honest, visible exception rather than a silent one. **Default: both go.**

---

## Delegation

None. This is a single-thread reading-and-judgement task over about a dozen small files that I have already read in full; splitting it across workers would cost more in briefing and cross-checking than it saves, and the rulings need to be consistent with each other. I would do it myself.

---

## What I would report at the end

1. **The draft's central proposal is rejected**, with the reasons: a new `notifier` service breaks the budget freeze; inline Twilio calls from the approve endpoint break the standing no-provider-in-the-request rule and cannot meet the 500 ms target against a 1.4 s provider; push/WhatsApp schema is for channels the spec puts out of scope; and putting the eligibility rule in the notifier would make a third copy of a rule that has already drifted once.
2. **What is kept from the draft**: the two swap tables, amended, and the dedupe idea, relocated to where it belongs.
3. **The real architecture change**: no new containers; swap notifications go `api → redis → worker → provider`; the worker gains an interactive queue separate from bulk fan-out; the INC-31 inline sends are reverted and the guard test rearmed.
4. **Store updated**: AX-003 drift and remedial work recorded, AX-005 flipped to decided because its trigger fired three weeks ago and nobody noticed, AX-009 added for queue isolation, AX-007 explicitly held at not-now with the reason, spine re-confirmed and its stale "nothing else today" line corrected.
5. **Two things the design phase inherits as open**: no worker-side p95 measurement exists for NFR-001, and the web eligibility mirror is not actually a mirror.
6. **The decision points and my defaults**: two Celery pools in one service (fallback: one pool, two queues); time-off revert included in this card.
7. **The process point**: the ledger claimed no drift for three weeks while a ruling was bypassed in shipped code. A hotfix that suspends a ruling should record it the same day.