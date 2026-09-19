FILES-READ: `README.md`, `specs/FEAT-014-swap-approvals/spec.md`, `specs/FEAT-014-swap-approvals/design-draft.md`, `architecture/spine.md`, `architecture/concerns.md`, `docs/incidents/INC-31.md`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/rotas.py`, `src/crewboard/domain/swaps.py`, `src/crewboard/notify/tasks.py`, `src/crewboard/notify/twilio_client.py`, `src/crewboard/worker/celery_app.py`, `tests/notify/test_no_inline_provider.py`, `web/src/swap/eligibility.ts`

---

# Action plan — FEAT-014 shift-swap approvals, architecture delta

## Headline finding before any phase

The reads already show three things that change the shape of this job:

- **The store's baseline is wrong about the live system.** `spine.md` states `api → Twilio / Postmark: **never**`, and the ledger records AX-003 as built with *drift: none*. But `api/swaps.py:8,42-45` and `api/timeoff.py:6,19` import and call the Twilio client inline, and the guard test skips both modules by name. AX-003 has been in drift since 2026-08-20. I cannot draw a delta on the baseline as written.
- **A deferred concern's trigger has already fired.** AX-005 defers provider-call idempotency until "the first incident in which a recipient receives the same message twice." INC-31 records exactly that — two staff at Harbour Kitchen got the rota-publish SMS twice. The row still says not-now.
- **The analyst's draft says "no architecture change" while proposing a new deployable service, a hand-built queue, a reversal of AX-003, and the eligibility rule living in three components.** It also draws `billing`, `reports`, `payroll-export` and `xero-sync` as services when the spine has them as modules inside `api` and `worker`. Its current-state picture is wrong, so its target is drawn on sand.

The plan below is sequenced around fixing the baseline first, then contesting the feature against it.

---

## Phase 1 — Load the working grammar, confirm what the checkout can and cannot tell me

**Do:** Load the four skills I work from — the system-design pattern (altitude, diagram conventions, what a container-level delta must contain), the technical-decisions pattern (when a fork earns a decision record and the record's shape), the architecture-store authoring skill (file grammar, element lifecycle, what a landing flips, how the health view is derived), and the shelf skill (which concerns to walk and the defaults to deal). Exact output paths and filenames in Phases 5–8 are provisional until this confirms the store's grammar — in particular whether decision records live as their own files under `architecture/` or as rows in the ledger. This checkout only shows `spine.md` and `concerns.md`, so I will not invent a third file layout without checking.

**Read:** the four skills.

**Guard I set here:** `README.md:30` says this is a *slice* — a test the ledger cites but that is absent (e.g. `tests/tenancy/test_isolation.py` for AX-001) exists in the full repo. So absence of a file in this checkout is never evidence of drift. I will grade drift only on positive evidence I can point at with a file and line. I keep that judgement myself and do not delegate it, because a missing-file answer would otherwise be read as a finding.

**Refuse:** any conclusion about AX-001, AX-004, AX-007 or AX-008 as-built claims from this checkout. I'll say in the report that they were out of reach here, not that they're clean.

---

## Phase 2 — Recover and correct the current state

**Do:** Write down the confirmed baseline as it actually is, marking each claim's evidence.

Confirmed from code:
- Containers are exactly `web`, `api`, `db`, `worker`, `redis`. `billing`, `reports` are `api` modules; `integrations.xero`, `payroll`, `notify` are `worker` modules (`worker/celery_app.py:16-22`). The draft's four extra boxes do not exist.
- One Celery queue, `celery`, default for everything, concurrency 8 (`celery_app.py:10-13`). Beat runs the reminder sweep, Xero sync and payroll upload on the *same* queue as interactive sends.
- `api/rotas.py:19` follows AX-003 (enqueues `send_rota_published`). `api/swaps.py` and `api/timeoff.py` do not.
- Swaps today have **no colleague step**: `propose()` creates the swap, `decide()` is manager-only (`api/swaps.py:1-2,25-27`). FEAT-014's US-002 is a genuinely new state in the machine, not a tweak.
- `decide()` re-runs `can_swap` then calls `db.exchange_assignees` with no guard that the shifts are unchanged since the proposal — FR-003's "rejected if the shifts changed" has no mechanism today.
- `can_swap` lives once in `domain/swaps.py`, with a hand-maintained mirror in `web/src/swap/eligibility.ts` that the file's own comment says drifted once already (INC-27 follow-up).

**Delegate (cheap, `Explore`, model haiku, one gap per spawn):**
1. *"List every file under `src/` and `web/` that names `twilio_client`, `postmark_client`, `twilio` or `postmark`. Return path plus line, no commentary."* — on return I check it against the four I already found; anything extra means the inline-send blast radius is wider than INC-31 documented and Phase 6's revert grows.
2. *"List every file under `src/crewboard/` that references `can_swap` or `eligibility`. Path and line only."* — on return I check whether a fourth copy of the rule already exists that neither the draft nor the spine mentions.

Both are bounded enumerations over a small tree; the interpretive call on what the hits mean stays with me.

**Write:** nothing yet — this becomes the "current state" section of Phase 5's delta document and the correction in Phase 7.

---

## Phase 3 — Architectural judgement on the draft (the substance of the job)

**Do:** Work through the draft claim by claim and settle what I will carry forward, what I cut, and what I hand to the user as a fork. My positions going in:

**3a. Cut the `notifier` service.** It is a new deployable container, which C-001 forbids outright until Q1 2027. Beyond the constraint, it isn't paid for: `worker` + Celery + Redis *is already* the outbound-messaging component, with backoff, retry caps and `acks_late` configured (`notify/tasks.py:9-10`). The draft would stand up a second, hand-written copy of that machinery — polling loop, `attempts`, `next_attempt_at`, exponential backoff, `dedupe_key` — beside a broker that does all four. Even if a box here were paid for, this is the box you should never hand-write. **Target container set for FEAT-014: unchanged. Zero new containers.**

**3b. Cut the inline Twilio call on approve.** The draft has `api` call Twilio and return 200 only on delivery confirmation. That breaks NFR-003 on arithmetic alone — Twilio's p95 is 1.4 s per INC-31, against a 500 ms p95 budget, before the two-recipient loop. It also breaks NFR-002 (decision must be visible when the provider is unreachable) and re-commits the exact AX-003 violation that FEAT-014 is on the hook to revert. The approve path is: persist state + audit event in one transaction, commit, then enqueue.

**3c. Cut `notification_outbox` and `notification_channel_config`.** The channel enum carries `push` and `whatsapp` and the config table exists for a 2027 roadmap; the spec puts both explicitly out of scope. Nothing in FEAT-014 pays for either. On the outbox itself there is a real question, and I'll put it to the user rather than just deleting it (stop S2 below): commit-then-`.delay()` can lose the enqueue if the process dies in the gap. My read is that FR-005 already says a notification failure must not block or reverse the state change and the rota is source of truth — so a lost notification is a tolerated outcome, and a transactional outbox is not paid for. Default: cut.

**3d. Reject the eligibility rule living in three places.** The draft puts `can_swap` in `notifier`, keeps a copy in `api`, and keeps the `web` mirror. That is one responsibility with three homes, in a codebase where this exact rule has already drifted once. The rule stays in `domain/swaps.py`, single home. The rejection *reason* the notifier supposedly needed to recompute is produced once at decision time and travels in the task payload — recomputing it in a second component to word an SMS is not a reason to duplicate a business rule. I will flag the pre-existing `web` mirror as a standing hazard in the report and recommend the button take its reason from the server response, but I will not smuggle that into this delta as a requirement; what I *will* hold is that FEAT-014 adds no fourth copy.

**3e. Raise what the draft was actually reaching for: queue contention.** INC-31's root cause was not worker size alone — it was that one queue serialises a manager's decision SMS behind a 4,100-message publish burst. The resize helped; the topology didn't change. NFR-001 (60 s p95 to hand off, measured in the worker) is exposed to the next Monday burst. The honest, cheap answer is a second Celery queue for interactive sends with its own routing, on the *existing* worker — a config change, no new container, satisfies C-001 and C-003. This is the alternative I'll set against the draft's `notifier` in a decision record, because it addresses the real problem with far less structure. It is a genuine fork, so the user rules it (stop S3).

**3f. Name the transaction boundary FR-003 needs.** The exchange and the staleness check must sit in one transaction with the shifts locked or version-compared; today's code has neither. I'll state it at container level (one transaction in `api → db`, no cross-component step inside it) and hand the mechanism to detailed design.

**Write:** working notes only; this feeds Phases 5 and 6.

---

## Phase 4 — Shelf walk, including the rows nobody has walked

**Do:** Walk the shelf for this desk visit, hardest-to-retrofit first, dealing a default and a change-trigger for each and marking every one as *my recommendation pending the user's ruling*. Rows in play:

- **AX-005 idempotency — trigger fired.** I bring INC-31's duplicate sends as the evidence and recommend ruling it now: an idempotency key per logical send, checked at the provider boundary in `notify/tasks.py`. Cheap here, expensive to retrofit across every send site later. (Stop S1.)
- **AX-003 — restate, don't re-rule.** The ruling stands; what changes is the as-built record and the drift status.
- **Audit (never walked).** FR-006 and US-005 demand actor+timestamp per state change, manager-visible. The stance question is whether that's a feature-local `swap_event` table or a general audit facility. Default I'd deal: feature-local for now, since swaps are the only thing asking; trigger = the second feature needing the same trail (time-off decisions are the obvious next). (Stop S5.)
- **Data retention (never walked).** How long swap history lives. Default: no policy yet, follows the rota data's lifetime; trigger = first tenant deletion request or the first table crossing a size where it matters. (Stop S6.)
- **Observability (never walked).** NFR-001 says "measured in the worker" — that measurement has to exist or the NFR is unverifiable. Default: emit a hand-off latency metric from the notify task and a queue-depth metric; this is small and FEAT-014 pays for it directly.
- **Rate limiting, feature flags (never walked).** Raise both, expect both to defer; I will still put them on the table with triggers rather than skip them because the answer looks obvious.

**Write:** proposed rows drafted, not committed, until Phase 7.

---

## Phase 5 — Write the FEAT-014 architecture delta

**Write:** `specs/FEAT-014-swap-approvals/architecture.md` containing:

1. **Current state (confirmed 2026-09-11, from code)** — the five containers, the single queue, the AX-003 drift, the missing colleague step, with file/line evidence and my confidence per claim. Explicitly noting the draft's four phantom services do not exist.
2. **Target state** — same five containers. Component classification: `api` **modified** (new colleague accept/decline endpoint, decision endpoint reworked, inline Twilio removed); `worker` **modified** (new swap-notification task, optional new queue); `db` **modified** (swap tables); `web` **modified**; `redis`, Twilio, Postmark **existing, unchanged**. **No new components.**
3. **Component diagram** — current and target, drawn at container level, with every change marked as new/modified/existing so a reader can see the delta rather than diff two pictures.
4. **Interaction flows** for the two that carry ordering and failure semantics:
   - *Approve*: manager → `api` → single transaction (staleness re-check, `can_swap`, exchange assignees, write audit event) → commit → enqueue notify task → 200. Then `worker` → Twilio, Postmark on no-mobile fallback, Celery retry on provider error. Annotated with what happens when Twilio is slow, down, or accepts-then-times-out (the INC-31 duplicate case), and where each of NFR-001/002/003 is met.
   - *Propose → accept/decline*: the new colleague state, who is notified at each transition per FR-004.
5. **Buildability and NFR check** — against C-001 (no new services: satisfied), C-002 (Twilio/Postmark only: satisfied), C-003 (three engineers, one release: the cut scope is what makes this true), and each NFR traced to the mechanism that meets it.
6. **What I cut from the analyst draft and why** — the `notifier` service, the inline confirm-then-200, the outbox, the push/WhatsApp channel machinery, the duplicated eligibility rule — each with the constraint or the absent requirement that justifies the cut, and the smaller shape named in its place. Written as reconciliation for the analyst, not as a rebuke.
7. **Open forks** — the stops below, each with the branch consequences, so design cannot start on a fork I answered myself.

---

## Phase 6 — Decision records

**Write** (paths confirmed against the store grammar in Phase 1; provisionally `architecture/decisions/`):

- **The swap-notification path.** Options: (a) enqueue on the existing single Celery queue; (b) enqueue on a new dedicated interactive queue on the same worker; (c) the draft's `notifier` service with a polling outbox; (d) inline provider call. Verdict: (d) rejected on NFR-003, NFR-002 and AX-003; (c) rejected on C-001 and as hand-built machinery Celery already provides; recommend (b) over (a) on INC-31's serialisation evidence and NFR-001. Records the fork honestly and marks (b) as recommended-pending-ruling.
- **Provider-send idempotency**, only if the user rules it now at stop S1; otherwise the ledger row is re-dated with the trigger noted as fired and consciously carried, which is a different and legitimate answer but must be recorded as *chosen*, not as still-unfired.

---

## Phase 7 — Update the store

**Write** `architecture/spine.md`:
- Correct the container list narrative where the draft would have misled (no change needed — the spine is right and the draft was wrong; I note this in the report).
- AX-003 line changes from a flat "never" to the ruling plus a named live exception with its remediation owner, until Phase 8's revert lands.
- Add the swap-approval flow to the `api → worker` communication line, which currently reads "rota-publish notifications, shift reminders, **nothing else today**."
- Queue line updated if S3 rules for the second queue.
- Re-date "last confirmed."

**Write** `architecture/concerns.md`:
- **AX-003**: as-built corrected to record the INC-31 hotfix; **drift: yes**, with evidence (`api/swaps.py:42-45`, `api/timeoff.py:19`, guard test skip list at `tests/notify/test_no_inline_provider.py:11`); work item pointing at FEAT-014.
- **AX-005**: trigger marked fired with the INC-31 citation and the date, then whatever the user rules at S1.
- **AX-007 event bus**: check its trigger explicitly ("a second consumer for the same domain event"). FEAT-014's fan-out is one task per state change with one consumer, so the trigger has *not* fired — I record that I checked rather than leaving it silent.
- **New rows**: audit, data retention, observability, rate limiting, feature flags — each with stance, rationale, and either a ruling or a deferral *with a trigger*. None written as decided unless the user ruled it.
- **Shelf coverage** line updated to move the newly-walked rows across.
- Re-render the derived health view from these rows rather than hand-editing it.

**Refuse:** writing any stance as "decided" that came out of my mouth and not the user's. Unruled rows go in as recommended-pending with the question attached.

---

## Phase 8 — The INC-31 revert, scoped as an architecture obligation

**Do:** Specify (not implement — this is design phase) the revert that INC-31 assigns to FEAT-014: remove the `twilio` import and inline loop from `api/swaps.py`, replace with the notify task enqueue, and remove the module from `HOTFIX_SKIP` in the guard test.

**Test I would name as the landing gate:** `tests/notify/test_no_inline_provider.py` with an empty `HOTFIX_SKIP`. Expected before the revert: it skips two modules and passes vacuously for them. Expected after: it runs across every `crewboard.api` module and passes, which is the only mechanical proof AX-003 is back to as-ruled. I would additionally specify a test that the approve endpoint enqueues rather than sends, and a test that a Twilio failure leaves the swap approved and visible (NFR-002, FR-005).

**Stop S4:** `api/timeoff.py` carries the same hotfix but is not FEAT-014's feature surface. If FEAT-014 reverts only swaps, AX-003 stays in drift and the guard stays partly skipped after this feature lands. My default: revert both — it is a handful of lines, `send_sms` already exists, and it is the only way the guard goes fully green. Branch if the user says swaps-only: AX-003 remains recorded as drifted with `timeoff` named, and I open a carried work item with an owner rather than letting it fall off.

---

## Stops — where I would put the work down and ask

Each is described with the branch, and I proceed under the stated default if no ruling comes.

| # | Question | Default I proceed on | Other branch |
|---|---|---|---|
| S1 | AX-005: rule idempotency now, given INC-31 fired the trigger? | Rule it now; key per logical send at the provider boundary | Carry consciously — ledger records trigger fired *and* deliberately deferred, with a new trigger |
| S2 | Transactional outbox for the enqueue gap? | No — FR-005 tolerates a lost notification; cut the table | Yes — outbox becomes a delta element, but still drained by Celery, never by a polling service |
| S3 | Second Celery queue for interactive sends? | Yes — INC-31's serialisation is unaddressed and NFR-001 is exposed | No — single queue, NFR-001 accepted as at-risk on burst, recorded as such rather than assumed safe |
| S4 | Revert `timeoff` inline send too? | Yes — otherwise AX-003 stays drifted post-landing | No — drift stays recorded, work item gets a named owner |
| S5 | Audit trail: feature-local or general facility? | Feature-local `swap_event`; trigger = second feature needing it | General facility — larger delta, needs its own decision record |
| S6 | Swap history retention? | No policy yet; follows rota data | A policy now — becomes a ruled row with enforcement |
| S7 | Analyst reconciliation: I am cutting `notifier`, the outbox, the channel config, and two of the three eligibility copies | Cut as argued | If the user wants any of them kept, I need the requirement or constraint that pays for it — and for `notifier` specifically, C-001 has to move first |

---

## What I would report at the end

- **The store was wrong about the running system.** AX-003 has been in drift for three weeks and the ledger said "drift: none"; the spine's "never calls a provider from a request handler" is contradicted by two live modules. Corrected, with evidence.
- **A deferred concern's trigger fired on 2026-08-20 and nobody moved the row.** AX-005, evidenced by INC-31's duplicate sends. Surfaced for ruling.
- **The draft's "no architecture change" was the opposite of true** — it added a container, a hand-built queue, a second copy of a business rule, and a reversal of a standing ruling. And its baseline diagram invented four services that are modules.
- **FEAT-014 needs no new containers.** The delta is `api` and `worker` modified, plus tables. The thing the draft was reaching for is real — interactive notifications queue behind bulk bursts — and it is answered by a routing config on the worker we already run, not a new service the budget freeze forbids.
- **Five shelf rows that had never been walked are now on the table**, with defaults and triggers, awaiting the user's rulings.
- **Design can start on** the delta document plus the decision record, once S1–S7 are ruled; the items still open, and what each one blocks.
- **Out of reach in this checkout:** as-built claims for AX-001, AX-004, AX-007, AX-008 — the slice doesn't carry their code or tests, and I will not grade them from memory.