FILES-READ: `README.md`, `render.yaml`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-010-payroll-export/spec.md`, `specs/FEAT-010-payroll-export/target-architecture.md`, `docs/payflow-api.md`, `src/crewboard/api/approvals.py`, `src/crewboard/domain/shift.py`, `src/crewboard/worker/celery_app.py`, `src/crewboard/integrations/xero/tasks.py`, `src/crewboard/scheduling/tz.py`

---

# Action plan — FEAT-010 payroll export architecture delta

## Headline before the phases

I would not turn the sketch into the delta as drawn. Three of its five components are forbidden by the spec's own constraints, and a fourth is hand-building something the system already runs. I would bring the user a delta that meets every FR and NFR with **zero new containers**, alongside a written account of what I cut and why, and then stop for their ruling — because several of these are the CTO's calls to reverse, not mine.

---

## Phase 1 — Recover and grade the baseline before drawing any delta

**What I do.** The store has a spine last confirmed 2026-05-14 at the FEAT-007 landing. FEAT-009 has landed since (the ledger cites it under AX-004) without a spine reconfirmation, so I treat the spine as *stale-but-close* rather than confirmed, and I check it against the code before I design on it.

**What I read (already done, plus targeted confirmations).** `spine.md` and `concerns.md` against `render.yaml`, `worker/celery_app.py`, `integrations/xero/tasks.py`, `api/approvals.py`, `domain/shift.py`, `scheduling/tz.py`.

**What I find and would record as baseline confidence:**

- Containers `web`/`api`/`worker`/`db`/`redis` — **confirmed high** against `render.yaml`.
- `worker` static outbound IPs — **confirmed** by the `render.yaml` comment and the spine note; matches C-002.
- AX-007 outbound-integration pattern — **confirmed high**: `xero/tasks.py` is exactly the ruled shape (one task per business, `autoretry_for`, `retry_backoff`, idempotency key per batch, `acks_late`).
- AX-004 local-time fan-out — **confirmed**: beat `hourly-local-fanout` → `scheduling.tasks.hourly_tick`, with `tz.venues_at_local_hour` as the selector.
- Approvals boundary — **confirmed**: `api/approvals.py` carries an explicit note that the state machine runs in the same transaction as the shift row, and `domain/shift.py` already anticipates FEAT-010's `exported` state.
- **Gap I'd mark, not guess**: `src/crewboard/scheduling/tasks.py` and the `tests/` tree the ledger cites are not in this checkout (README says the checkout is a slice). I mark AX-001/AX-003/AX-007 enforcement tests as *cited, not verified here* rather than claiming I checked them.

**Delegation.** One disposable `Explore` subagent, `model: haiku`, brief: *"List every file under `src/` that references `payflow`, `gateway`, `kafka`, `redpanda`, or `kong`, and every file defining a Celery `@shared_task`; return paths and one-line quotes, nothing else."* On return I check it names no pre-existing payroll module and no gateway config — but I do **not** rest the extend-vs-new decision on its silence; I already globbed the tree myself, because absence is what drives that decision.

**What I write in this phase.** Nothing yet.

---

## Phase 2 — Audit the sketch against the constraints and the ledger

I test each of the sketch's five components for whether it can be built and whether a requirement pays for it. This is the analysis that becomes the "considered and rejected" section of the delta.

| Sketch component | Verdict | Why |
|---|---|---|
| **Redpanda event bus** | **Refuse — unbuildable** | C-001 puts managed Kafka/Redpanda off the approved vendor list until the SOC 2 audit closes in 2027-Q1. Separately, AX-005 stands at *not-now* with a stated trigger — a second consumer for one domain event, or a fan-out too big for one task. FEAT-010 has exactly one consumer (the nightly export) and a 3,000-shift monthly ceiling per venue (C-003). The trigger has not fired. Self-hosting a broker is a new service, also barred by C-001. |
| **payroll-service** | **Refuse — unbuildable** | C-001: no new services this quarter. Every responsibility it claims already has a home: export → `worker` (AX-007), status surface → `api`, storage → `db`. |
| **Moving approvals into it** | **Refuse — contradicts the spec** | FR-006 says the approval flow is unchanged. `api/approvals.py` runs the state machine in the same transaction as the shift row; splitting it across a service boundary turns one transaction into a distributed one and buys nothing FEAT-010 asks for. |
| **Moving hours reports into it** | **Refuse — unpaid boundary move** | AX-006 rules reports are computed on read in `api.reports`, with an upgrade trigger of a report over 2 s at p95. No evidence that fired. No FEAT-010 requirement asks for a reports change. |
| **`approved_hours_rm` CQRS read model** | **Refuse — smeared responsibility** | `db` already holds approved hours as the single truth. A second copy fed by events creates a lag window inside the pay-period boundary that FR-001 depends on ("approved since the last successful export"), and two places that can disagree about what someone worked. |
| **Home-grown job runner** (`job`, `job_attempt`, `dead_letter`, replay endpoint) | **Refuse — a paid-for box that shouldn't be hand-written** | Retry-with-backoff, dead-lettering, and replay are what `worker` already does under AX-007, and the team operates it today. Hand-building a second scheduler is a component three engineers (C-004) maintain forever. Worse, FR-005's re-run and the export's own retry state are the same row — `payroll_export_run` *is* the job record. |
| **Kong api-gateway** | **Refuse — speculative and unbuildable** | A new service (C-001), justified only by "by next year we'll have three or four services." No FR or NFR pays for it. Routing and auth today are one service and a session cookie. |
| **PayFlow as an external system** | **Accept** | Genuinely new. Belongs in the spine's external list. |
| **PayFlow webhooks** | **Accept the need, contest the placement and necessity** — see Phase 4 fork B. | |

**What I refuse to draw at all:** the sketch's `CREATE TABLE` block and its endpoint list. Table columns and request bodies are detailed design drawn to fit the shape; putting them in the architecture delta is the wrong altitude and would freeze decisions the implementing engineer should make. I carry forward only what is structural: *which container owns the export-run record*, not its columns.

---

## Phase 3 — Test the smaller shape against the NFRs before proposing it

The shape I intend to propose is: **`api` modified, `worker` modified, `db` modified, PayFlow added external, zero new containers.** A shape is only a design if it clears the numbers, so I run the arithmetic first. These are the checks whose results go into the delta as its buildability evidence.

1. **NFR-001 (upload 02:00–05:00 local).** Existing `hourly-local-fanout` beat + `venues_at_local_hour` already delivers per-venue local-hour dispatch. Check passes on existing machinery — no new scheduler.
2. **NFR-002 (survive a 3-hour PayFlow outage).** This is the one that bites. Xero's settings — `retry_backoff_max=1800`, `max_retries=8` — give a cumulative retry span of roughly four minutes, nowhere near three hours. Copying the Xero task verbatim would silently fail this NFR. Two ways out: raise `max_retries` to ~16–20 against the 1800 s ceiling, or — my recommendation — let the **hourly local sweep re-pick any export run still unfinished** until a 07:00-local cutoff, then mark it failed and notify. The second reuses AX-004 machinery, keeps the retry state in a durable row rather than in a Celery retry chain that a worker restart loses, and makes FR-005's manual re-run the same code path. I would state in the delta that NFR-002 is what pays for the export-run row existing at all.
3. **NFR-003 (result visible by 08:00 local).** PayFlow's webhook is *optional and enabled per company in their dashboard* (their docs), so it cannot be the guaranteed path for 200 venues. Their status endpoint resolves within 15 minutes in practice, one hour promised. A status poll on the hourly sweep clears 08:00 local with hours to spare.
4. **Rate limit (60 uploads/min per partner account).** 180 venues today, up to 200 (C-003), most plausibly in one timezone — a 02:00-local fan-out is a single burst of ~180 uploads against a 60/min ceiling. This forces a throttle: a per-task rate limit on the payflow task and routing to the existing `slow` queue. I name this explicitly; it is small, but it is a real constraint the sketch's diagram hid.
5. **C-002 (one static egress IP).** Uploads must leave from `worker`, which has static IPs; `api` does not. This alone rules out the sketch's payroll-service owning the upload, since a new Render service would need its own IP and PayFlow allow-lists one per partner account.
6. **Volume.** 3,000 shifts/venue/month → roughly 100–200 lines in a nightly batch. One HTTP call, one task. Nothing here needs a bus.

---

## Phase 4 — Stop for the user's rulings

These are structural forks, and several reverse the CTO's stated intent. I put them on the table with my recommendation and the trigger that would change it; the ruling is the user's and I do not write a stance into the store that they did not make.

**Fork A — the sketch's four new components (bus, service, gateway, job runner).**
*What I'd confirm:* that the delta lands on the zero-new-container shape, and the sketch's topology is recorded as considered-and-rejected with the constraints that rejected it.
- *If they accept (my default, and what I plan under):* proceed to Phase 6.
- *If they want the bus or the gateway anyway:* I do not draw it as buildable this quarter. I record it as a **deferred target shape with a dated trigger** — SOC 2 close in 2027-Q1 plus the AX-005 second-consumer condition — and note in the delta that FEAT-010's design is deliberately migration-tolerant: the export reads approved hours through one module boundary, so a future consumer swaps its source without touching the export logic.
- *If C-001 has changed since the spec was written* (budget freed, vendor list updated): that is new information, not a design argument, and I'd re-run Phase 3 against a bus topology before conceding it.

**Fork B — PayFlow result: poll, webhook, or both.**
*What I'd confirm:* whether we accept polling only.
- *Poll only (my recommendation and default):* one mechanism, works for every company regardless of their dashboard setting, no inbound endpoint, no signature verification, no event-id dedupe table. Clears NFR-003 with margin.
- *Webhook only:* I'd push back — it is optional per company, so coverage is not guaranteed, and it would leave NFR-003 unmet for any venue that hasn't enabled it.
- *Both:* buildable and consistent with AX-008 (terminate in `api`, verify HMAC, dedupe by event id), but it is a second path to the same state for a latency gain we don't need. If ruled, the webhook endpoint lives in `api` — never in a new service — and I record it as a marked addition with AX-008 as the paying rule.
This fork is genuinely two-sided and turns on structure, so **I would write it up as a decision record** rather than bury it in the delta prose.

**Fork C — AX-005's row.** The trigger has not fired, so my default is to re-affirm *not-now* with the date and this feature named as evidence. But the CTO's sketch is effectively an argument to change it, and a ledger row moves on the user's word, not mine. If they rule *decided → adopt*, C-001 still blocks it this quarter and the row becomes *decided, deferred to 2027-Q1* with the audit as the fuse.

**Fork D — the `exported` shift state.** FR-006 adds `exported` after `approved` plus a manager override, which `domain/shift.py` already anticipates. This modifies the FEAT-004 approvals boundary in place. My default: keep it in `api`/`domain` where it lives, mark it a modified component, and flag that the override path needs an audit trail (Phase 5).

---

## Phase 5 — Walk the shelf rows this feature makes expensive

I would not write the delta without putting these in front of the user, hardest-to-retrofit first. Four are unwalked rows from the ledger's own "not yet walked" list, and this feature is what makes three of them expensive.

1. **Secrets / per-venue credentials (new row).** FR-007 stores a PayFlow company id *and an API credential* per venue. Where does that credential live, how is it encrypted, who can read it, and does it appear in logs or error rows? The ledger has no secrets row at all. This is the single most expensive thing to retrofit here, because it is money-adjacent and it lands in `db` on day one. I'd bring a default (application-level encryption of the credential column, key from Render env, never returned by any read endpoint, redacted in `last_error`) with the reason and the trigger that would move it (a second provider, or a compliance requirement from the SOC 2 work).
2. **Audit (unwalked).** Who re-ran an export (FR-005), who overrode a re-approval on an exported shift (FR-006). Payroll disputes are exactly where this gets asked for after the fact.
3. **Data retention (unwalked).** How long export runs, batch payloads, and rejection reasons are kept. Payroll line data has a retention profile the rest of crewboard doesn't.
4. **Observability (unwalked).** NFR-002 and NFR-003 are promises about a nightly job nobody watches. A silently failing nightly export is the classic failure mode; the manager-facing status screen (FR-004) covers the venue's view but not ours.
5. **Rate limiting (unwalked).** Partly forced already by the 60/min PayFlow ceiling — I'd note the outbound side is now decided by this feature and the inbound side remains open.

I present each as a stance to rule, with my default and its trigger, and record deferrals only with a fuse attached. **Health note I'd raise unprompted:** the spine's "last confirmed" date predates FEAT-009's landing, so the store is carrying an unconfirmed spine. I'd fix the date as part of this landing rather than leave it.

---

## Phase 6 — Write the delta

**Skills I'd load first** (not loaded here — plan-only): the system-design pattern skill for the delta's altitude and diagram conventions, the store-authoring skill for the store's grammar and element lifecycle, the decisions skill for the record format, and the shelf skill for Phase 5's dimension list and defaults. **Where the skill's file convention differs from my default path below, the skill wins** and I'd say so in the report.

**Path (default, matching the store's existing flat style):** `architecture/deltas/FEAT-010-payroll-export.md`

**Contents:**

- **Baseline it's drawn on**, with the confidence grading from Phase 1 and the explicit note that `scheduling/tasks.py` and `tests/` were not in this checkout.
- **Component delta table**, every element classified:
  - `api` — **modified**: export status read surface and re-run trigger (FR-004, FR-005); venue↔PayFlow connection storage (FR-007); `exported` state and override in the approvals path (FR-006). *No* provider SDK import — AX-007's boundary holds.
  - `worker` — **modified**: new `crewboard.integrations.payflow` module, mirroring the Xero task shape; nightly per-venue upload task; hourly status-poll/re-attempt sweep. Routed to the `slow` queue with a rate cap.
  - `db` — **modified**: export-run and export-line records, and the venue↔PayFlow connection. Named as responsibilities, **not** as DDL.
  - `PayFlow` — **new external system**.
  - `web`, `redis` — **existing, unchanged**.
  - Explicitly: **no new container.**
- **Two container-level diagrams**, scoped to the neighborhood this touches (`web`, `api`, `worker`, `db`, `redis`, PayFlow) rather than redrawing crewboard: current state, and target with every change marked as new/modified.
- **Interaction flows** for the two crossings whose ordering and failure semantics matter:
  1. *Nightly export* — beat hourly tick → venues at 02:00 local → per-venue task → build batch from shifts approved since last success → upload with `Idempotency-Key: crewboard-{venue_id}-{night}` → record result. Failure legs: PayFlow down (run stays open, re-attempted on the hourly sweep, failed and reported at the 07:00-local cutoff — NFR-002); PayFlow times out after accepting (the next attempt replays the same batch id and PayFlow returns the original result, which is what makes FR-003 hold); rate-limit rejection (backs off within the window).
  2. *Result resolution* — hourly status poll → accepted/rejected + reason → manager-visible state (NFR-003, FR-004). Plus the manual re-run path (FR-005) showing that it reuses the same task and batch id, so a re-run cannot duplicate lines.
- **Buildability and NFR evidence** — the Phase 3 arithmetic written out, including the retry-window finding and the rate-limit burst finding, each tied to the requirement that pays for it.
- **Considered and rejected** — the Phase 2 table, with the constraint or ledger rule that rejected each, and for the bus and gateway the trigger that would reopen them. This section is the honest answer to the CTO's sketch and I would not soften it.
- **Enforcement hooks handed to the implementing team** (I specify, I don't write the code): extend the existing `tests/integrations/test_idempotency_key.py` assertion to cover the PayFlow batch call — expected to fail until the task sets the key, then pass; an import-boundary test that nothing outside `crewboard.integrations.payflow` imports the PayFlow client — expected to pass from day one and to fail the moment `api` reaches for it; a test that the configured retry-and-sweep window covers three hours — expected to fail against Xero's copied settings, which is precisely the point of writing it.

---

## Phase 7 — Land it in the store

- **`architecture/concerns.md`** — new rows for the concerns ruled in Phase 5 (secrets/credentials at minimum), each with the user's ruling, date, rationale, and trigger; AX-005 updated per Fork C; AX-007 and AX-008 gain FEAT-010 as-built notes **only after the code lands**, not now — at kick-off they stay as-ruled with FEAT-010 marked in-flight. Shelf-coverage line updated to reflect what was walked and what remains.
- **`architecture/spine.md`** — PayFlow added to external systems; `worker` module list gains `integrations.payflow`; `api` module list gains the payroll surface; communication-styles and boundaries lines extended (PayFlow called from `worker` for the static-IP reason, C-002). "Last confirmed" bumped, noting FEAT-009 was folded in.
- **Decision record** for Fork B (and Fork C if the user moves it), default path `architecture/decisions/` pending the skill's convention.
- **Derived index**, if the store has one, is rendered rather than hand-edited.
- **A note appended to the CTO sketch is *not* something I'd do** — I don't edit someone else's sketch; the delta's rejection section is the reply.

---

## Phase 8 — Self-check, then report

**Check before I hand it over:** every FR and NFR maps to something in the delta; every component in the delta maps back to a requirement or constraint that pays for it (if one doesn't, it comes out); every ledger row I touched carries a ruling the user actually made; no DDL or endpoint signatures crept in.

**What I'd report:**
1. The delta is **zero new containers** — `api`, `worker`, and `db` modified, PayFlow added external — and it meets all seven FRs and all three NFRs under C-001 through C-004.
2. **Four of the sketch's five components I refused**, with the constraint or ledger rule behind each: the bus and the gateway and payroll-service are barred outright by C-001 and the vendor list; the job runner duplicates machinery `worker` already runs and that three engineers would maintain forever.
3. **Two findings the sketch's shape concealed**, both structural: copying Xero's retry settings would quietly miss NFR-002's three-hour outage promise, and a 200-venue same-timezone fan-out would breach PayFlow's 60-uploads-per-minute ceiling.
4. **One real fork recorded as a decision** — poll versus webhook for the PayFlow result — with polling recommended because their webhook is opt-in per company and so can't carry NFR-003 alone.
5. **Store health:** the spine was carrying an unconfirmed baseline since FEAT-009; FEAT-010 opens a credentials/secrets concern the ledger has never had a row for; and four shelf rows remain unwalked with three of them now more expensive than they were last month.
6. **What I could not verify here:** the enforcement tests the ledger cites and `scheduling/tasks.py` are outside this checkout, so their status in the delta is marked as cited rather than checked.