FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-014-swap-approvals/spec.md`, `specs/FEAT-014-swap-approvals/design-draft.md`, `docs/incidents/INC-31.md`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/rotas.py`, `src/crewboard/domain/swaps.py`, `src/crewboard/notify/tasks.py`, `src/crewboard/notify/twilio_client.py`, `src/crewboard/worker/celery_app.py`, `tests/notify/test_no_inline_provider.py`, `web/src/swap/eligibility.ts`

---

# Action plan — FEAT-014 shift-swap approvals, architecture change

## Headline before the phases

The reading already turned up two things that change the job. The analyst's draft says "no architecture change." It is, in fact, almost entirely architecture change — a new deployable service, a second home for an existing business rule, and an inline provider call that reverses a standing ruling — and separately, **the store's picture of the current system is wrong in a way that this feature owns the fix for**. So this is not "draw the target and go." It is: repair the baseline, then draw a much smaller delta than the one I was handed.

---

## Phase 1 — Load the working procedures

**Do:** Invoke, in this order, `mochiko:patterns-architecture-shelves` (I have five unwalked concern rows and at least two this feature forces), `mochiko:authoring-architecture-store` (I am going to write to a store whose grammar, element lifecycle and file layout I must not improvise), `mochiko:patterns-system-design` (the delta's altitude and diagram conventions), and `mochiko:patterns-technical-decisions` (there is at least one genuine fork here that needs a decision record).

**Why this is first:** every path I name below is provisional until the store-authoring skill tells me the real convention. I will not invent a filename for a delta artifact or a decision record and then write to it. Where I give a path in this plan I mark it *provisional*.

**Refuse/flag:** if the store skill's layout disagrees with what I've assumed, the skill wins and I re-path everything before writing a byte.

---

## Phase 2 — Recover and grade the baseline against the code

The spine's "last confirmed" is 2026-07-02. Two things landed after that date (the INC-31 hotfix on 2026-08-20, the worker resize on 2026-08-28). I will not design a delta on a baseline that stale.

**Read (done in this pass):** the seven `src/` modules, the one test, the one `web/` file, plus `architecture/spine.md`, `architecture/concerns.md`, `docs/incidents/INC-31.md`.

**Findings I already hold, to be written down as graded claims:**

| Store claim | Code says | Grade |
|---|---|---|
| Spine: "`api → Twilio / Postmark`: **never** … (AX-003)" | `api/swaps.py:8` and `api/timeoff.py:6` both import `crewboard.notify.twilio_client`; `swaps.py:42-45` and `timeoff.py:18-19` call `twilio.messages.create` inline after commit | **False since 2026-08-20** |
| AX-003 "**As-built**: as ruled · **Drift**: none" | Two api modules bypass it; the guard test skips them by name (`test_no_inline_provider.py:11`) | **Drift, unrecorded** |
| AX-005 rationale: "a duplicate SMS … we have never seen one" | INC-31: "Two staff at Harbour Kitchen received the 'rota published' SMS twice" | **Rationale falsified 2026-08-20** |
| Spine boundary: "worker runs one queue (`celery`) with concurrency 8 since 2026-08-28" | `celery_app.py:10,13` — `task_default_queue="celery"`, `worker_concurrency=8` | **True** |
| AX-004 as-built | `celery_app.py` beat schedule + `send_shift_reminder`/`sweep_reminders` stubs present | **Consistent; stub bodies elided in this slice** |

**Delegation — one disposable `Explore` subagent, `model: haiku`, one gap:**
> Brief: "In this repo, list every file that contains the string `render.yaml`, and if a file named `render.yaml` exists anywhere, print it verbatim. Report file paths and line numbers only; do not interpret." 
> **On return I check:** whether the service list in it matches the spine's five containers, and whether the budget-freeze constraint C-001 is visible as a real deployment fact rather than only prose in the README. If the file is absent from this slice I record C-001 as taken on the README's and the spec's word, not verified.

**Second `Explore` spawn, `model: haiku`, separate gap:**
> Brief: "List every call site of the symbol `can_swap` across `src/`, `web/` and `tests/`, with file and line. Exact matches only." 
> **On return I check:** that the only Python callers are `api/swaps.py:18` and `api/swaps.py:31`, confirming the rule has exactly one implementation home today plus the acknowledged hand-synced TypeScript mirror. If a third copy exists, the smearing problem is already worse than the draft proposes and that changes my Phase 5 recommendation.

**I do not delegate:** the reading of the ledger, the incident, or the draft. Those are interpretive and absence of a claim in them drives decisions.

**Explicit non-finding — a trap I will not fall into:** the ledger's AX-001 enforcement cites `tests/tenancy/test_isolation.py`, which is not in this checkout. The README states plainly that a test the ledger cites but the slice omits exists in the full repo and was not removed. I record AX-001 as **unverified in this slice**, not as drift, and I do not raise it as a finding.

**Confidence statement I will carry into the delta:** baseline recovered from code at **high** confidence for the notification path, the swap endpoints, the eligibility rule and the Celery topology; **medium** for anything named but not present in the slice (`api/deps.py`, `settings.py`, `notify/postmark_client.py`, `integrations.xero`, `payroll`, the `billing`/`reports` api modules) — for those I carry the spine's word and mark it as such.

---

## Phase 3 — Ledger health sweep

Beyond the two findings above, I walk every row for a fired trigger or a stale status.

- **AX-005 — the fuse has burned.** Its upgrade trigger is *"the first incident in which a recipient receives the same message twice."* That happened on 2026-08-20. The row still reads `not-now` on a rationale the incident falsified. Saying this out loud is my job, not the user's. INC-31's third follow-up ("decide whether provider sends need an idempotency key, owner: **TBD**") is exactly this decision, ownerless for three weeks. I bring it to the desk in Phase 7.
- **AX-007 event bus — trigger has *not* fired.** Its trigger is a second consumer for the same domain event or a fan-out too big for one task. FEAT-014's notifications are single-consumer fan-outs finishing inside a task. This row stays `not-now` untouched — and this is precisely the row the draft's outbox-plus-poller design would quietly overturn without anyone ruling it.
- **AX-003 — needs a drift record and a repair owner.** INC-31's second follow-up already assigns the revert to FEAT-014.
- **Shelf coverage** — five rows never walked: observability, rate limiting, feature flags, data retention, audit. This feature forces **audit** (FR-006, US-005 are literally an audit trail) and **observability** (NFR-001 says "measured in the worker" — nothing in the store says how anything is measured, so the NFR is currently unverifiable by construction). Those two get walked at this desk. The other three get offered and will likely close in seconds or defer with a trigger — but they get offered, not skipped because they look boring.

---

## Phase 4 — Assess the analyst's draft, and say plainly what to cut

I write this assessment as a section of the delta artifact, not as a private opinion. Seven objections, each with the smaller shape:

1. **`notifier` as a new service is barred by C-001.** The budget freeze on new services runs to Q1 2027 (finance, 2026-07-01) and the spec restates it. The draft's own closing line — *"No architecture change — this just adds tables and a helper service"* — is the exact failure mode of topology smuggled in as a footnote. A new deployable box *is* the topology. **Cut it.**
2. **The outbox-and-poller is a hand-built Celery.** `redis` + Celery already provide the durable queue, the backoff, the retry ceiling and the late-ack (`tasks.py:9-10`: `autoretry_for`, `retry_backoff`, `retry_backoff_max=600`, `max_retries=5`, `acks_late=True`). The draft proposes re-implementing `attempts`, `next_attempt_at` and exponential-backoff-to-five in application code, plus a five-second poll loop, on top of infrastructure already paid for and already ruled (AX-003, AX-004). This is the paid-for box that should never be hand-written. **Cut it; the existing `worker` is the notifier.**
3. **`notifier` owning FR-002 eligibility is responsibility smearing, and the draft admits it.** It proposes the rule live in `notifier` *and* "`api` keeps its own copy" *and* `web` keeps its mirror — three implementations of one rule. The rule has one home today, `crewboard.domain.swaps.can_swap`, and the web mirror is already documented as having drifted once (INC-27 follow-up, per `eligibility.ts:2`). Rejection reasons are already returned by `can_swap` as strings; the notification carries the reason it is handed, it does not recompute it. **One home stays one home.**
4. **`api` calling Twilio and blocking on delivery confirmation must be rejected on four separate grounds.** It reverses AX-003. It cannot meet **NFR-003** (500 ms p95) when Twilio's p95 per send is 1.4 s by the incident's own measurement. It contradicts **NFR-002** and **FR-005**, which require the decision to persist and display when the provider is unreachable — blocking the 200 on Twilio does the opposite. And it is *the very hotfix this feature is chartered to revert*. **Cut it.**
5. **`notification_channel_config`, and the `push`/`whatsapp` channel values, are speculative.** The spec's out-of-scope section names push and WhatsApp explicitly. A per-business channel-config table for a 2027 roadmap item is structure no requirement pays for, and it is the kind the team maintains forever. **Cut the table; the channel enum carries `sms` and `email` only.**
6. **The draft's target diagram misstates the baseline.** It draws `billing`, `payroll-export`, `xero-sync` and `reports` as containers. Per the spine they are *modules*: `billing` and `reports` inside `api`, `integrations.xero` and `payroll` inside `worker`. It also omits `api → redis` (the enqueue path that AX-003 depends on) and draws `worker ◀── redis` in a way that reads as redis initiating. A delta drawn on a mis-drawn current state is not a delta. **Redraw the current state first.**
7. **What I keep from the draft:** `swap_request` and `swap_event` are right in substance. FR-006's history is a table and a read endpoint, not a component. I keep them, drop `notification_outbox` and `notification_channel_config`, and hand the field-level shape down to the design phase rather than settling columns myself — that is below the altitude of this artifact.

---

## Phase 5 — Shape the delta

**Target topology: unchanged. Zero new containers.** Every component is `existing` or `modified`; none are `new`. That is the honest headline.

| Element | Class | Change |
|---|---|---|
| `web` | modified | colleague accept/decline screen (US-002), manager decision screen (US-003), swap history view (US-005). `eligibility.ts` mirror **left alone** — not extended, its hand-sync cost re-flagged. |
| `api` / `swaps` module | modified | new colleague accept/decline step; decision endpoint reverted to enqueue; history read endpoint. **Provider import removed.** |
| `api` / `timeoff` module | modified | provider import removed — scope question, Phase 7. |
| `domain.swaps` | modified | gains the proposal state machine (proposed → accepted → approved/declined/rejected) alongside `can_swap`. One home for both rules. |
| `notify.tasks` | modified | one new task for swap-state notifications, SMS with email fallback (FR-004, US-004), using the existing `send_sms`/`send_email` primitives. |
| `db` | modified | `swap_request`, `swap_event`. |
| `worker`, `redis`, external systems | existing | untouched. |

**Interaction style, decided rather than defaulted.** Every crossing this feature adds:
- `web → api`: synchronous. The user is waiting for the answer; NFR-003 is a 500 ms budget on a database write plus an enqueue, which is comfortable.
- `api → db`: synchronous, one transaction. **FR-003 requires the assignee exchange to be atomic**, and it requires re-checking that the shifts have not changed since the proposal — a stale-swap rejection. That is a same-transaction concern with a concurrency guard; I flag it to the design phase as needing a specific locking or version-check answer, and I name it in the flow rather than leaving it to be discovered.
- `api → worker` via `redis`: asynchronous, and this is the *load-bearing* choice. FR-005 states a failed notification must neither block nor reverse the state change, and NFR-002 requires the decision to survive an unreachable provider. Async is not stylistic here; the requirements name the failure semantics and only async satisfies them.
- `worker → Twilio/Postmark`: synchronous from the task with Celery backoff, per AX-008 and existing practice. Unchanged.

**Interaction flows I would draw** (only where ordering or failure semantics actually matter): (a) manager approves — commit, then enqueue, then respond, with the failure branch where the enqueue succeeds and the send later exhausts retries; (b) the stale-swap rejection path of FR-003; (c) a swap decision arriving during a Monday publish burst, which is the flow that Phase 6 turns on.

---

## Phase 6 — The one real fork: can a plain AX-003 revert meet NFR-001?

This is the question the draft never asked, and it is the only place I think new structure might genuinely be paid for.

**The arithmetic.** NFR-001: a notification reaches the provider within **60 s** of the state change at p95, measured in the worker. INC-31's root cause was not only worker size — it was that *"the single `celery` queue serialised every other task behind the burst."* The resize fixed throughput; the burst now drains in **under six minutes**. Six minutes is six times the NFR-001 budget. `celery_app.py:10` confirms one default queue is still the live configuration, and `tasks.py:46` states every task runs on it. So: reverting the hotfix as-is puts swap decisions back behind a weekly burst that blows the stated NFR. The revert is necessary and **not sufficient**.

**Alternatives I would write into a decision record** (path per the decisions skill; provisionally `specs/FEAT-014-swap-approvals/decisions/` or the store's central decisions location):

- **A. Revert only, accept the burst window.** Cheapest, zero new structure. Fails NFR-001 for roughly one hour a week unless product relaxes the NFR to exclude the publish window.
- **B. Celery task priorities within the single queue.** No new structure, but priority on a Redis broker is implemented via key ordering and interacts badly with `prefetch_multiplier` and late-ack; behaviour under a 4,100-task burst is hard to predict and harder to test. Cheap to configure, expensive to trust.
- **C. A second named queue (e.g. `interactive`) consumed by the same worker process** with dedicated concurrency — transactional, human-waiting sends (swap and time-off decisions) routed to it; bulk fan-out (rota publish, reminders) stays on `celery`. **No new service, so C-001 is respected.** It is one config change in `celery_app.py`, one routing decision per task, one line in the Render start command. It attacks INC-31's actual root cause rather than its symptom.
- **D. A second worker service.** Cleanest isolation, **barred by C-001**. Recorded and rejected, so nobody re-proposes it in six months.

**My recommendation: C**, with the reasoning that it is the smallest shape that meets a stated NFR, and with the trigger that would change my mind stated plainly — if product rules that NFR-001 need not hold during the publish window, C is unpaid-for structure and A is correct. **This is the user's ruling, not mine.** I will not write it into the ledger as settled on my own say-so.

---

## Phase 7 — The desk session: what I stop for, and the branch on each ruling

I would not write to the store before this conversation. Five things go on the table, hardest-to-retrofit first.

**Stop 1 — AX-005 idempotency. The trigger has fired.** 
*I present:* INC-31 produced the exact duplicate the trigger names; the row's rationale is now factually wrong; INC-31's own follow-up is ownerless; and FEAT-014 increases transactional send volume. 
*Branches:* **(a) Rule it in now** → AX-005 becomes `decided`, a dedupe key is scoped into this release, and I add it to the delta as a `notify.tasks` modification (the natural key is the swap-event id, which `swap_event` gives us for free — this is cheap *because* FR-006 already pays for the event table). **(b) Keep deferred with a corrected rationale and a sharper trigger** → I rewrite the rationale to acknowledge the incident and get a new trigger the user names; I record it as knowingly deferred, not forgotten. **(c) Split** — key the swap sends only, leave bulk publish alone. 
*My default if no answer comes:* **(a)**, scoped to swap notifications only. The trigger fired; leaving a falsified rationale in the ledger is the thing I most want to avoid.

**Stop 2 — the notification path under burst (Phase 6, options A–D).** 
*Branches:* **C** → I add a queue-isolation row to the ledger, amend the spine's boundary line about the single queue, and the delta carries the routing change. **A** → I record NFR-001 as relaxed during the publish window, on product's word, with who ruled it and when; no ledger row for queues. **B** → decision record notes the testability risk and I require a load-shaped test before the release ships. **D** → refused while C-001 stands; I'd need finance to lift the freeze in writing before I'd draw it. 
*My default:* **C**.

**Stop 3 — does the `api/timeoff.py` revert belong to FEAT-014?** INC-31 assigns the revert to FEAT-014 "since it touches the same endpoint," but that reasoning covers `swaps.py`; `timeoff.py` is a second module the feature does not otherwise touch. 
*Branches:* **In scope** → both `HOTFIX_SKIP` entries go and the guard test runs clean across all of `crewboard.api`. **Out of scope** → I refuse to leave it silent: the skip list keeps one entry, AX-003 stays in drift with a named owner and a date, and it goes to the backlog explicitly. 
*My default:* **in scope.** It is a four-line change, it is the same import of the same client for the same reason, and a guard test with a permanent exception list is a guard that has stopped guarding.

**Stop 4 — audit (unwalked, forced by FR-006/US-005).** Who may read swap history — managers of the venue only, or owners across venues? Is `swap_event` append-only? Is this the seed of a general audit facility or a feature-local table? 
*My default:* feature-local, append-only, visible to venue managers per AX-002's role model; recorded as a deliberately narrow stance with the trigger "a second feature needs the same trail" for generalising.

**Stop 5 — observability (unwalked, forced by NFR-001).** NFR-001 says "measured in the worker" and the store records no measurement capability anywhere. As written the NFR cannot be verified, which makes it decoration. 
*My default:* a minimal stance — record enqueue time on the task payload and emit the delta at send — with the wider observability row deferred and given a trigger.

**Also offered, expected to close fast:** rate limiting, feature flags, data retention. Each gets a sentence and a default; none passes silently. Data retention interacts with `swap_event` growth and may not close as fast as it looks.

---

## Phase 8 — Write the store and the delta

Paths *provisional* pending Phase 1's grammar:

- **`architecture/spine.md`** — bump "Last confirmed" to the session date. Correct the communication-styles line so it states the AX-003 rule *and* the live exception until the revert lands (an aspiration written as fact is what got us here). Add `swaps` state handling to the `api` module list and the swap-notification task to `worker`. If Stop 2 rules C, amend the boundary line about the single queue.
- **`architecture/concerns.md`** — AX-003 status moved to reflect drift, with the drift described, its cause (INC-31), its owner (FEAT-014) and its clearing condition (guard test un-skipped and green). AX-005 updated per Stop 1. AX-007 explicitly re-affirmed as `not-now` with a note that FEAT-014 considered and rejected an outbox/poller, so the next reader knows it was weighed. New rows from Stops 2, 4, 5 and the three quick offers. Shelf-coverage list updated to move audit and observability out of "not yet walked."
- **`specs/FEAT-014-swap-approvals/architecture.md`** *(provisional)* — the delta: reconstructed current state with the confidence statement, corrected current-state diagram, target diagram, the change table with every element classed new/modified/existing, the three interaction flows, the buildability check against C-001/C-002/C-003, the NFR check against NFR-001/002/003, and the draft assessment from Phase 4 written as an explicit record of what was cut and why.
- **A decision record for the notification path under burst** (Phase 6, options A–D). Possibly a second one recording the rejection of the `notifier` service, if the store skill's convention is that a rejected structural proposal of that size earns its own record rather than a section.
- **The derived index** — rendered by the tooling from what I write, never hand-edited. There is no index file in `architecture/` in this slice; I would confirm from the store skill whether one is expected and have it regenerated rather than typing one.

**I would not write** to `specs/FEAT-014-swap-approvals/design-draft.md`. It is the analyst's artifact and a record of what was proposed; my assessment lives in mine and cites theirs.

---

## Phase 9 — Enforcement, and what the tests should show

Architecture that nothing checks is architecture that drifts. Three checks, specified here and written in the design/build phase:

1. **Un-skip `tests/notify/test_no_inline_provider.py`** — delete `HOTFIX_SKIP` (lines 10-11) and its guard (lines 18-19). 
 *Expected today:* **FAIL** on `crewboard.api.swaps` and `crewboard.api.timeoff` — that failure *is* the drift, made visible instead of skipped past. 
 *Expected after the revert:* green across every module under `crewboard.api`. I would want it red first, deliberately, so we can see the guard actually bites.
2. **A routing assertion**, if Stop 2 rules C: swap and time-off notification tasks resolve to the `interactive` queue, bulk publish and reminders to `celery`. Expected to fail before the config change, pass after.
3. **A single-home assertion for eligibility and the state machine**: no module outside `crewboard.domain.swaps` defines a swap transition table or re-implements `can_swap`. Expected green at the start and to *stay* green — its job is to fail the day someone re-introduces the draft's second copy. The `web/` mirror is a known, accepted, documented exception and stays out of the assertion's reach.

I would also flag to the design phase — without designing it myself, as it is below this artifact's altitude — that FR-003's "shifts changed since the proposal" check needs a concrete concurrency answer and a test that two simultaneous approvals cannot both exchange, since a double-booking is the original sin (INC-27) this whole feature exists to end.

---

## Phase 10 — What I report at the end

1. **The delta is far smaller than the draft.** No new containers. Six modified elements. C-001 is respected because nothing new is deployed.
2. **Seven things cut from the draft**, each with the requirement that failed to pay for it — and specifically: the `notifier` service (barred by C-001), the outbox/poller (re-implements Celery), the inline-Twilio approve (breaks AX-003, NFR-002, NFR-003, FR-005 simultaneously), and the push/WhatsApp channel config (explicitly out of scope).
3. **The store was wrong, and now it isn't.** AX-003 claimed zero drift while two api modules had been calling Twilio inline for three weeks behind a skipped test. That is now recorded, owned and given a clearing condition.
4. **AX-005's fuse burned on 2026-08-20 and nobody noticed.** Raised, with INC-31's ownerless follow-up attached to it.
5. **NFR-001 does not survive a plain revert.** The single shared queue — not worker size — was INC-31's root cause and is still live in `celery_app.py`. Recommendation C, the user's ruling to make.
6. **Open items carried out of the session**: whichever stops the user deferred, each with its trigger, plus the three quick shelf rows and the FR-003 concurrency question handed to design.
7. **Confidence:** high on the notification path, the swap endpoints and the Celery topology, all read from code. Medium on the modules named in the spine but absent from this slice, carried on the store's word and marked as such — and `tests/tenancy/test_isolation.py`'s absence noted as a slice artifact, not drift.