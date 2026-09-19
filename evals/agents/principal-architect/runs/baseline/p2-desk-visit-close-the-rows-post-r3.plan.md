FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-011-audit-log/architecture-delta.md`, `docs/changelog.md`, `docs/metrics-2026-08.md`, `src/crewboard/api/main.py`, `src/crewboard/api/middleware.py`, `src/crewboard/audit/record.py`, `src/crewboard/api/rotas.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/staff.py`, `src/crewboard/api/billing.py`, `src/crewboard/api/reports.py`

---

# Action plan — desk-visit prep, crewboard architecture store

## Headline before the plan

Two parts of the card I would not carry out as asked, and I want them at the top rather than buried in a phase:

1. **AX-009 does not flip to `built`, and the as-built line from the approved delta would be false.** The code says FEAT-011 shipped something materially different from what was approved at the desk on 2026-07-20 — in fact it shipped the exact shape the desk rejected. Details in Phase 2.
2. **I will not mark eight rows `decided`.** `decided` means the CTO ruled. Writing my recommendation into that field manufactures a ruling nobody made, and in six months nobody will be able to tell the difference — which is precisely how a stance gets "discovered" rather than chosen. I will do all the work of the walk and stage it so ratification is a two-minute read on the plane. Stop point and branches in Phase 4.

Everything else on the card I do in full, plus a health sweep that turned up four problems the card did not mention.

---

## Phase 1 — Ground the baseline (mostly done in prep)

**What I do:** I have already read the whole checkout — it is nine source files plus the store, changelog and metrics. I re-confirm the spine's container list against `README.md` and the code layout before touching anything, because every delta and every stance I write rests on it.

**Read:** the fifteen files listed above (complete).

**Skills I would bind here:** the architecture-store authoring skill for the ledger grammar and what a landing is allowed to flip; the shelves skill for the dimension list, defaults and triggers used in Phase 4; the system-design skill if Phase 2's branch forces me to draw a corrected delta; the decisions skill only if AX-018 gets ruled and needs a decision record written.

**Delegation:** **none, deliberately.** My standing habit is to push locates and enumerations to a cheap `Explore` subagent on haiku, but this checkout is nine files totalling a few hundred lines and the enumeration that matters — *which routers write to the database without an audit call* — is completeness-sensitive and drives the Phase 2 refusal. A miss there produces a wrong verdict on the one thing the card asks me to certify. I read it myself. Spawning here would cost more in brief-writing than the read.

Against the **full** repo (this checkout is a slice) I would spawn exactly four `Explore` subagents, `model: haiku`, one gap each, terse facts with file:line provenance:
- *"Does a migration create table `audit_event`, and does any migration or grant script revoke UPDATE/DELETE on it from the application role? Quote the lines."*
- *"Is there a Celery beat schedule entry that purges `audit_event`? Name the file and the schedule."*
- *"Does `tests/audit/test_every_mutation_audited.py` exist, and which routers does it exercise? List them."*
- *"In the provider-send path, is an idempotency key passed to Twilio/Postmark/Xero/PayFlow calls? Quote each call site."* (for AX-005)

On return I would check each answer carries a real path and line, and that the third one's router list is *enumerated*, not summarised — if it comes back "all routers" without naming them I re-ask, because that is the claim the whole AX-009 verdict turns on. In this slice all four are unanswerable, so they become named unverified items in the report, **not** assumptions. I specifically will not write "the enforcement test is missing" — the README says a cited test absent from the slice exists upstream.

---

## Phase 2 — Grade AX-009 against the code, and refuse the flip

**What I found (evidence, not recollection):**

| Approved delta says | Code says |
|---|---|
| `AuditMiddleware` registered on the app; the middleware is the single writer | `api/main.py:9-11` — a TODO: *"AuditMiddleware — parked 2026-08-27 … Rotas and swaps call `audit.record()` explicitly for now."* `api/middleware.py:1` — *"No audit middleware exists."* |
| No router calls the audit module directly | `rotas.py:20,32` and `swaps.py:19` call `record.record()` directly |
| Every POST/PUT/PATCH/DELETE recorded | `timeoff.py` decide, `staff.py` create/update/delete, `billing.py` plan/card — **six mutating endpoints, zero audit writes** |
| `worker` purges rows older than twelve months | no purge module; spine lists `worker` modules as `notify`, `integrations.xero`, `payroll` only |
| Audit write in its own transaction, after commit | `record()` writes on the same session before `db.commit()` (`rotas.py:20-22`) — a rolled-back mutation loses its audit row *and* an audit failure now rolls back the user's mutation |

The desk's stated reason for rejecting per-router calls was *"a router that forgets the call silently drops a category."* That is not a hypothetical any more — it happened on the first pass, and the category it dropped is `staff`: name, mobile, email, contracted hours. Personal data, unaudited, in a product that has already handled three GDPR requests by hand. Billing plan changes are unaudited too.

**What I write:** AX-009 goes to `Stance: decided · Status: drift`, ruling unchanged, with:
- **As-built (FEAT-011, shipped 2026-09-02, graded against `src/` 2026-09-11):** audit writes are made by explicit `record.record()` calls in `rotas` and `swaps` only; no `AuditMiddleware`; `timeoff`, `staff`, `billing` mutations record nothing; writes share the handler's transaction; no purge task found in this checkout.
- **Drift:** severe — the built shape is the per-router pattern rejected at the desk on 2026-07-20, and it has already lost the personal-data category. Coverage 3 of 9 mutating endpoints.
- **Enforcement:** the guard test the row cites would fail as described if it does what the row says; existence and scope unverified in this slice → flagged, not asserted.
- A dated grading note so the next reader knows what the claim was checked against.

**Refusal:** I do not write the delta's "Store effect at landing" line. It describes a system that does not exist. A store that records intentions as reality is worse than no store, and this one would have told the next GDPR request that staff edits were covered.

**Stop — what I would confirm with you:** *AX-009 status and what FEAT-011 owes.* Branches:
- **(a) You accept `drift` (my default, and what I proceed under):** the row stays open as drift, and I write a short remediation note naming the smallest closing move — finish the parked middleware and delete the two router call sites, rather than adding calls to the other four routers. The parking reason was that `billing` and `staff` don't attach before/after state; that is a handler-level fix, not a reason to keep the rejected topology.
- **(b) You rule the per-router pattern is now the accepted shape:** then the 2026-07-20 ruling is *superseded*, not satisfied — I rewrite the ruling, record why the original objection is being accepted as a cost, and the row still cannot go `built` until the four silent routers record, because the ruling's own words are "every mutating request".
- **(c) You tell me the full repo has the middleware and this slice is stale:** I re-grade against the full repo before writing anything. Cheap to check, and I would rather check than argue.

---

## Phase 3 — Health sweep (not on the card; found while in there)

The card asks for a clean health view. Four things stop it being clean, none of which the card mentions:

1. **AX-010 Search — trigger fired, on both limbs.** The trigger is ">50 venues OR staff-picker p95 above 800 ms for any business". August metrics: The Hearth Group, 63 venues, staff-picker p95 **1,230 ms**. The row still reads `not-now`. I flip it to `Status: trigger-fired (2026-08 metrics)` with the evidence inline and a note that this is now a live row for the next walk — I do not invent the replacement ruling, because "index the picker" vs "paginate/scope the picker to a venue" vs "reach for a search service" is a real fork and one of them is much cheaper than the others. My recommendation attached, unruled: fix it inside `db` first (trigram index + venue-scoped query); a search service is a new container and there is a budget freeze on new services until Q1 2027.
2. **AX-011 Reports — stale in-flight.** Marked `in-flight (FEAT-008)`; the changelog says FEAT-008 shipped and the feature closed 2026-07-15. I grade it: `reports.py` is read-only, computed on read from the primary session, no replica, no warehouse — matching the ruling. Report p95 780 ms, under the 2 s trigger. Flip to `built`, as-built "as ruled", drift none, graded 2026-09-11 against `api/reports.py`.
3. **AX-005 — stale in-flight.** Marked `in-flight (FEAT-014)`; FEAT-014 shipped 2026-09-05 and AX-003's own as-built line already cites that landing. But the provider-send code is outside this slice, so I *cannot* certify idempotency keys. I mark it `in-flight · landing unverified — FEAT-014 shipped 2026-09-05, idempotency-key evidence not in this checkout` and put it top of the report as the one row that needs a five-minute check upstream. I will not flip it on the strength of a changelog line; that is memory, not evidence.
4. **AX-012 Data retention — a deferral with no fuse.** The `Upgrade trigger` field is literally empty. That is not deferred, it is forgotten. I fill it with a proposed trigger (first contractual retention term, or the first regulator/customer request for a defined retention period, or `audit_event` passing a size threshold) and mark the trigger as proposed-not-ruled. I also note the row is already partly overtaken: AX-009 rules a twelve-month audit retention, so retention is decided for one table and undecided everywhere else.

Also recorded: **shelf coverage gaps.** The coverage note names secrets management and dependency/supply-chain policy as not on the ledger. I open them as **AX-021** and **AX-022** in `open` state with walk notes, so the shelf is complete on paper even though they weren't walked. An unopened row is invisible; an open row with no notes is at least visible.

---

## Phase 4 — The eight rows (AX-013 … AX-020)

**Stop — what I would confirm with you:** *may my recommendations be written into the `Stance` field as `decided`?* My default, and what I proceed under: **no.** Branches:
- **(a) You confirm on return that each recommendation is right:** I flip that row to `decided`, dated with your ruling date, in a single pass. Two minutes of work.
- **(b) You want them decided in advance regardless:** I would still write them as `recommended` and attach a line saying you pre-authorised ratification — the ledger records *who ruled and when*, and a batch of eight rulings made by an architect in an empty room is exactly the kind of thing that gets found later and disbelieved. If you insist after reading that, I do it — but you'd be asking me to make the store lie about its own provenance, and that is the store's only real asset.
- **(c) You ratify some and want to talk about others:** the likely outcome; AX-018 and AX-016 are the two I expect you to want to talk about.

**What I write into each row:** `Stance: recommended (awaiting ruling) · Status: open`, plus `Recommendation`, `Why this is the default`, `What would change my mind`, and — where it matters — `Alternatives weighed and why not`. Dated 2026-09-11, attributed to me, not to you.

**Refusal on "skip any row that obviously doesn't apply to our size."** I walk all eight. A row that takes ten seconds costs nothing; the row nobody raised is the one retrofitted through the whole system a year later. Two of these eight look skippable on size and are the *most* expensive to retrofit (AX-019, AX-017). Where a row genuinely doesn't apply I record it `n-a` **with a trigger** — that is a walked row, not a skipped one.

Taken in ledger order:

**AX-013 Feature flags.** Recommend: a `feature_flag` table in `db` keyed `(flag, business_id)`, read through one helper in `api`; drop the env-var-plus-redeploy pattern. Why: they already roll to one pilot business, which an env var cannot express without a redeploy per business, and a redeploy to turn a flag *off* is an incident-response liability. No new container, no vendor — the budget freeze holds. Changes my mind: percentage rollouts, or flags needed inside `worker` at task frequency (then it needs caching in `redis`), or flag count past ~15.

**AX-014 Localisation.** Recommend: `n-a` for multi-language; **decided** for money and time — currency stored per business (the Irish prospect is EUR), all timestamps stored UTC and rendered in the venue's timezone. Why: the second language is a real project nobody has asked for; currency is a column, and finding out it isn't one *after* signing the Irish prospect is a data migration. Changes my mind: a non-English market, or a second timezone in one business.

**AX-015 Experimentation (A/B).** Recommend: `n-a`. No product ask, 64 businesses is too small a population for most splits to say anything, and it would ride on AX-013 anyway. Trigger: a product decision where someone actually wants a control group — at which point build it on the flag table, not a vendor.

**AX-016 Rate limiting.** Recommend: **decide now, build before the Q4 widget.** A token-bucket limiter in `api` backed by the existing `redis`, three tiers — per-session, per-integration-token, per-IP for the unauthenticated widget path — returning 429 with `Retry-After`. Why now: INC-19 already spent the money (one runaway client, one `api` instance at 100% CPU for twenty minutes, and there are only two instances — that is half the fleet), and the Q4 widget puts a token-authenticated endpoint on the public internet. This is the row I would least accept being deferred. Changes my mind: nothing about size; if anything, two instances makes it *more* urgent. Alternative weighed: limiting at Render's edge — rejected, it can't see the integration token or the tenant, so it can't do the thing INC-19 needed.

**AX-017 Backups and restore.** Recommend: turn on point-in-time recovery (already on the plan — it costs a checkbox), and rehearse one restore into a scratch database this quarter, recording measured RPO and RTO in the row. Why: a backup that has never been restored is an untested claim, and seven-day snapshot retention is shorter than the time it takes to notice quiet data corruption. Three engineers can do this in an afternoon. Changes my mind: nothing — this is the cheapest row on the shelf and the one whose absence ends companies.

**AX-018 Identity federation (SSO).** This is a genuine fork and genuinely yours — it turns on a commercial judgement I don't hold. I write the fork, not the ruling. Options: (i) Google Workspace OIDC bolted onto the existing session-cookie auth, manager logins only, staff unchanged — smallest shape, one provider, no new container; (ii) a general OIDC/SAML layer for any IdP — much more machinery, pays only if enterprise becomes a segment; (iii) an auth vendor (Auth0/WorkOS) — fastest to SSO, but it moves AX-002 wholesale and adds a hard runtime dependency on the login path, plus spend during a freeze. My recommendation: **(i)**, and only once the contract is signed or credibly close. Changes my mind: a second enterprise prospect on a different IdP flips it to (ii); procurement demanding SCIM user provisioning changes the shape again and should be priced separately. **Stop:** if you want SSO committed to on the plane, tell me which of the three and whether the contract is real — I won't guess at a decision whose input is a deal I can't see.

**AX-019 Data partitioning, export, and deletion.** Recommend: **decide now** — for my money the most important row of the eight. Three GDPR requests already served by hand with ad-hoc SQL (each one an unlogged, unreviewed write against production), and the SSO prospect's procurement has asked for export on termination, so it is now a deal term. Shape: a `tenant_data` manifest listing every table carrying `business_id` and its treatment (export / delete / anonymise / retain-for-legal, and `audit_event` is retain — deleting the audit trail on request is its own problem); two `worker` tasks, export-business and delete-business, driven by that manifest; a test that walks the schema and fails if any table with a `business_id` column is missing from the manifest. No new container, no new service. Why now rather than at the fourth request: `business_id` is already on every table, which makes this cheap *today* and steadily more expensive with every table added. Changes my mind: nothing I can see. Note the coupling — this row is the one that makes AX-012's empty trigger matter.

**AX-020 Observability.** Recommend: two alerts now, tracing not-now. Alerts: Celery queue lag over a threshold (p95 is 41 s *after* the resize, and INC-31 was reported by a customer forty minutes in — the queue was the signal and nobody was watching it), and a failure-rate alert on audit writes, which the FEAT-011 delta explicitly promised and which nothing currently backs. Tracing: `not-now`, with the trigger — a latency problem that spans `api` → `redis` → `worker` and can't be pinned from logs, or a third runtime container. Why: three engineers cannot carry a tracing stack, and Sentry plus two alerts closes the actual incident pattern they've had. Changes my mind: the trigger above, or a customer-facing latency SLA.

---

## Phase 5 — Spine

**File:** `architecture/spine.md`

Edits, all small:
- `api` modules: annotate `audit` as *present but router-invoked, not middleware — see AX-009 drift*, so the spine stops implying the approved shape.
- Communication styles: add the audit write as an in-transaction `api → db` write, marked as diverging from the approved delta. The delta drew it as a separate transaction after commit; that difference decides whether a failed mutation leaves a phantom audit row or a failed audit kills a user's save. It is currently the latter. Invisible in the spine today.
- `worker` modules: note that the twelve-month `audit_event` purge named in the approved delta is not present in this checkout — unverified upstream, flagged not asserted.
- Update **Last confirmed** to 2026-09-11 and say what it was confirmed *against* (this code slice), because "last confirmed" without a source is the same class of unbacked claim I'm objecting to in AX-009.

I do **not** redraw the topology. Nothing in the eight rows adds a container as recommended — that is intentional and worth saying out loud: every recommendation above lands inside `api`, `db`, `worker` or `redis`. The budget freeze on new services until Q1 2027 is not a constraint I'm straining against; it is a constraint that happens to agree with the right answers here.

---

## Phase 6 — Tests

I write no test code in this pass; the card is store work. Two I would specify for whoever picks up the remediation, because both are the kind of guard that turns a stance into something that can't quietly stop being true:

1. **Audit coverage guard** (the one AX-009 already names): enumerate every route on the app with method POST/PUT/PATCH/DELETE, exercise one of each, assert an `audit_event` row appears. Expected **today: fail**, on six endpoints — `timeoff.decide`, `staff.create`, `staff.update`, `staff.remove`, `billing.change_plan`, `billing.update_card`. It should enumerate routes from the app object, not from a hand-written list, or it will drift the same way the audit calls did.
2. **Tenant-table manifest guard** (AX-019, if ruled): read the schema, fail if any table with a `business_id` column is absent from the export/delete manifest. Expected on first run: fail, listing the tables — which doubles as the inventory nobody has written down.

I would run neither here (no execution in this pass), and I say plainly that the AX-009 expectation above is derived from reading the six handlers, not from a run.

---

## Phase 7 — What I report back

A short desk note in my reply — not a new file; the store is `spine.md` plus `concerns.md` and a third document is structure nobody asked for. Contents, in the order I'd want you to read them on the plane:

1. **AX-009 is not built and I did not flip it.** FEAT-011 shipped the per-router pattern the desk rejected, with `staff`, `timeoff` and `billing` recording nothing — personal data and payment changes are unaudited since 2026-09-02. One decision needed: finish the middleware, or supersede the ruling.
2. **AX-010's trigger fired on both limbs** in the August metrics — 63 venues and 1,230 ms, against a 50-venue / 800 ms fuse. Live row, not a deferral any more.
3. **Three rows that were telling you the wrong thing:** AX-011 in-flight for a feature that closed in July (now graded `built`), AX-005 in-flight for a feature that shipped 2026-09-05 (landing unverified — needs one check upstream), AX-012 deferred with an empty trigger (proposed one written in).
4. **Eight rows walked, recommendations written, none marked `decided`** — with the one-line reason, and a note that ratifying them is a two-minute pass on return. Of the eight: AX-016 and AX-019 I'd argue shouldn't wait for the next walk; AX-018 is yours alone and I've written the fork rather than a ruling; AX-015 is the only clean `n-a` and it still got a trigger.
5. **Two new rows opened** (secrets management, dependency/supply-chain) so the shelf is complete at 22 of 22 on paper.
6. **Four items I could not verify from this checkout** and did not guess at: the `audit_event` migration and its append-only grants, the twelve-month purge task, the audit enforcement test, and FEAT-014's idempotency keys. Named as unverified, with the exact four questions I'd ask of the full repo.