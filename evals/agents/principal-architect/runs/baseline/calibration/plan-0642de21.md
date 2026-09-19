FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-011-audit-log/architecture-delta.md`, `docs/changelog.md`, `docs/metrics-2026-08.md`, `src/crewboard/api/main.py`, `src/crewboard/api/middleware.py`, `src/crewboard/api/rotas.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/staff.py`, `src/crewboard/api/billing.py`, `src/crewboard/api/reports.py`, `src/crewboard/audit/record.py`

---

# Action plan — desk-visit prep, crewboard store

## Up front: two parts of the card I would not execute as written

Both are worth reading before the phases, because they change what gets written.

**(a) "Mark each of the eight rows `decided` with whatever you'd recommend."** I'll do the whole walk and put a concrete, argued recommendation on every row — that part I can do alone. What I won't do is stamp them `decided`, because `decided` in this ledger means *you ruled it*, and every other decided row carries a "ruled <date>" by you. If I write my recommendations in as your rulings, then in six months nobody — including you — can tell which stances the CTO actually formed and which ones an agent filled in while you were on a plane. That is the exact failure the ledger exists to prevent. So each row gets **`Stance: recommended`** with the recommendation, the reasoning, the trigger that would change it, and a one-word field for you to accept or reject. It reads the same on the plane and it costs you one reply.

**(b) "Flip AX-009 to `built` and take the as-built line from the approved delta."** I can't, and the reason is the finding of the day: **the approved delta was not built.** The code contradicts the delta's as-built line on its central point. Detail and branches in Phase 3.

**(c) "Skip any row that obviously doesn't apply to a product our size."** I'll go fast on the cheap rows but I won't leave any of them blank. A row closed in one line as "not applicable, here's the trigger" costs nothing and is a recorded answer; a skipped row is indistinguishable from a row nobody thought of, and gets rediscovered as a surprise. Two of the eight rows that look skippable at this size (localisation, experimentation) turn out to have a real edge — see Phase 4.

---

## Phase 1 — Load the working rules, then fix the ledger order I'll walk in

**Do:** Load my four skills before touching the store — the store's grammar and element lifecycle (what fields a row legally carries, what a landing flips, how the health view is derived), the shelf content and defaults, the delta rules, and the decision-record rules. I will not write a row shape from memory when the grammar is a file I can read.

**Read:** the four skills.

**One thing I'd check immediately:** the eight open rows all carry `Stance: open · Status: ruled`, which reads backwards against the decided rows above them (where `ruled`/`built`/`in-flight` describe delivery, not the stance). I suspect `Status: ruled` here is a typo for `open`/`unruled`. I'd confirm against the grammar and normalise; if the grammar genuinely allows it I leave it alone and note the confusion.

**Walk order:** you asked for ledger order and I'll write them in ledger order so the file reads straight. But the *report* leads with the expensive and already-overdue ones, because AX-013 (feature flags) is a two-second row and AX-019 (data export/deletion) is the one that gets retrofitted through every table in the system. Order on the page ≠ order of attention.

---

## Phase 2 — Recover the real current state before changing anything

**Do:** Grade what the store *claims* against what `src/` actually contains, for every row the card touches plus every row the changelog implicates. Done for the routers already, in the reads above.

**Confirmed by my own reading (not delegated — absence is driving a decision here):**

| Claim | Evidence | Verdict |
|---|---|---|
| `AuditMiddleware` registered on the app | `api/middleware.py:1` says explicitly there is none; `api/main.py:9-11` has a `TODO(FEAT-011)` marked **parked 2026-08-27** | **Does not exist** |
| Every mutating request is audited | `rotas.py:20,32` and `swaps.py:19` call `record.record()`; `timeoff.py:11-19`, `staff.py:10-35`, `billing.py:11-31` mutate and audit **nothing** | **5 of 8 mutating endpoints unaudited** |
| `audit_event` append-only, no UPDATE/DELETE grants | no migrations in this slice | **Unverified** |
| Twelve-month purge beat task in `worker` | `spine.md:12-13` lists worker modules as `notify`, `integrations.xero`, `payroll` — no purge; no worker code in slice | **Unverified, and the spine suggests absent** |

**Delegation — what I'd hand off and what I'd keep.** In *this* checkout the source slice is nine files and I've read all of them; spawning a worker to re-read them would cost more than it saves, so I wouldn't. Against the **full repo**, where the evidence I'm missing lives, I'd spawn three disposable `Explore` subagents with an explicit `model: haiku` override, one gap each:

1. *Brief:* "In `migrations/`, find the statement creating `audit_event` and any GRANT/REVOKE on it for the app role. Quote the lines with file:line." — *On return:* check the citations resolve and that a REVOKE on UPDATE/DELETE is quoted verbatim, not paraphrased. A "couldn't find one" from this spawn does **not** settle the question; I'd read the migrations directory myself before recording "not append-only."
2. *Brief:* "List every entry in the Celery `beat_schedule` with its task path and cron." — *On return:* confirm whether a purge task is in the list.
3. *Brief:* "Does `tests/audit/test_every_mutation_audited.py` exist? If so quote its list of routers under test." — *On return:* existence only; I read the content myself if it's there.

The judgement call — *is router-level coverage acceptable* — stays with me in all cases.

**Test I would run (cannot here):** the enforcement test the ledger cites, `tests/audit/test_every_mutation_audited.py`. The ledger marks it `(planned)`, so my expectation is it doesn't exist yet. **If it does exist and passes, I stop and investigate the test** — given `timeoff`/`staff`/`billing` demonstrably never write an audit row, a green test means the test isn't sending a mutating request per router and is providing false assurance, which is worse than no test. **If it exists and fails on three routers, that's confirmation** and I cite the failure in the drift record.

---

## Phase 3 — AX-009 / FEAT-011: record the landing honestly

**What shipped** (changelog: FEAT-011, 2026-09-02) is not what was approved. The desk ruled a middleware for a stated reason — *"a per-router call was rejected at the desk because a router that forgets the call silently drops a category."* What was built is the per-router call, and three routers forgot. The predicted failure occurred, on schedule.

And the categories it dropped are the bad ones: **`staff`** — create, update, delete on personal data (name, mobile, email, contracted hours) is exactly the "who changed this" the two businesses asked for, and it's the data an erasure request lands on. **`billing`** — plan and card changes by the owner, the highest-consequence writes in the product. The two routers that *are* covered (`rotas`, `swaps`) are the ones FEAT-011 and FEAT-014 actively touched.

**What I write instead of `built`:**

- `AX-009 · Stance: decided (unchanged — your 2026-07-20 ruling stands) · Status: **partial — landed with drift**`
- **As-built (FEAT-011, 2026-09-02):** "No `AuditMiddleware`; registration parked 2026-08-27 (`api/main.py:9`). Audit rows are written by explicit `audit.record()` calls in `rotas` and `swaps` only. `timeoff`, `staff` (create/patch/delete) and `billing` (plan/card) mutate without an audit row — 5 of 8 mutating endpoints uncovered. `audit_event` append-only grants and the twelve-month purge task: not verified, no evidence in this checkout."
- **Drift: yes — the shape the desk rejected is the shape in production.** With the router list and file:line citations.
- **Enforcement:** test still `planned`. Flag: until it exists, the next router added is uncovered by default and nothing says so.

**Stop point — this is the one decision I'd genuinely hold for you**, and the branches:

- **Ruling A — "finish the middleware":** the parked blocker is real but narrow. `main.py` says billing and staff don't attach before/after state. The middleware can register *now* and record actor/business/path/method/entity for every mutating request, with before/after present only where the handler attaches it. Partial detail on every mutation beats full detail on two routers. The `rotas`/`swaps` explicit calls then come out, so there's one writer. **This is my recommendation and my default if you don't reply**, because it's the ruling you already made and nothing in the code shows it to be wrong — only unfinished.
- **Ruling B — "router calls are fine, close it":** then AX-009 gets *re-ruled*, not quietly redefined, and it needs the enforcement test built first as a hard gate (a test that enumerates mutating routes by reflection and fails on any without an audit row), because the whole objection to per-router was silent omission. I'd argue against B but it's a legitimate call.
- **Ruling C — "it's fine as is":** I'd record it, and I'd record my disagreement next to it, because `staff` and `billing` being unauditable is a compliance answer you don't have.

I will **not** write the delta's as-built sentence. Copying "every mutating request recorded by `AuditMiddleware`" into the store when five endpoints record nothing turns the store from a record of reality into a record of intentions, and a health view built on that is clean in exactly the way that hides problems.

**Spine effect:** `worker` gets no purge module (unconfirmed); I'd note the audit writer as router-level with the drift flag, and update `Last confirmed` only for the parts I actually verified.

---

## Phase 4 — Walk the eight rows, in ledger order

Each written as: recommendation · why it's the default · what would change it. All eight as `recommended`, awaiting your word.

**AX-013 Feature flags → recommend: no flag system (keep env var + redeploy).**
Two flags a quarter, three engineers, budget freeze. Nothing pays for a flag service or a flags table. *Trigger:* a flag that must flip **without a deploy** (an incident kill-switch is the realistic one), or per-user rather than per-business targeting, or more than about one new flag a month.

**AX-014 Localisation → recommend: split the row.** Translation/i18n: **not applicable** — UK only, English UI, no ask. But the Irish prospect is **EUR**, and currency is not localisation; it's a per-business field, a Stripe currency on the subscription, and money formatting. That's a small data decision that will otherwise be discovered the week the deal signs. *Trigger for i18n:* a signed customer whose staff don't read English. *Trigger for the currency work:* the Irish contract reaching legals. This is the first row that looked skippable and wasn't.

**AX-015 Experimentation (A/B) → recommend: not applicable.**
64 businesses. Any split test is under-powered by an order of magnitude; you would be reading noise. Adoption from the metrics doc is the right instrument at this size. *Trigger:* a self-serve surface with thousands of independent users, where the unit of randomisation is a person rather than a business.

**AX-016 Rate limiting → recommend: decide and build now, not "not-now". The trigger already fired, twice.**
INC-19 (2026-03) was a runaway client holding an `api` instance at 100% CPU for twenty minutes — with two instances, that's half the fleet. And a token-authenticated public widget lands in Q4, which puts an unauthenticated-ish surface on the internet. Smallest shape that works: **a counter per credential in the `redis` you already run, enforced by an `api` middleware** — no new container, which matters under the freeze. Two classes: a generous limit on session traffic, a hard low cap per widget token. Failure stance, which is the part that gets forgotten: **if `redis` is unreachable, fail open for session traffic** (don't take the product down to protect it) **and fail closed for widget tokens** (an unmeterable public endpoint is the thing you're defending). Explicitly rejected: an API gateway or a new edge service — nothing here needs it and the budget forbids it.

**AX-017 Backups and restore → recommend: decided, and it's cheap.**
Turn on point-in-time recovery (already available on the current plan — no new spend), write down an actual RPO and RTO instead of leaving them implied, and **rehearse one restore into a scratch database this quarter.** A seven-day snapshot you have never restored is not a backup, it's a belief. Note the interaction with AX-019: seven days of retention means a mis-executed hand-run deletion is unrecoverable after a week. *Trigger for anything more* (off-provider copy, longer retention): a procurement questionnaire — likely from the SSO prospect — or a compliance commitment.

**AX-018 Identity federation (SSO) → recommend: decided in principle, build on contract signature. And do not hand-build it.**
Largest contract in the pipeline; OIDC against Google Workspace is a solved problem with a settled library. Shape: **an OIDC login route in `api` that mints the same session cookie you already issue** — no new container, no identity service, no change to AX-002. The IdP supplies *identity only*; `manager`/`staff` roles per venue stay ours, because a 40-venue group's Workspace groups will not match your venue model. Per-business IdP config as a table row. Explicitly rejected for now: a hosted identity platform (Auth0/WorkOS) — one customer, one protocol doesn't pay for it; and writing token verification by hand, which is the classic place a small team loses a week and ships a vulnerability. *Trigger to revisit the hosted option:* a second protocol (SAML), or SCIM user provisioning, or a third federated customer.

**AX-019 Data partitioning, export and deletion → recommend: decided. This is the most expensive row on the shelf and the one I'd read first.**
Three subject-access/deletion requests already served **by hand with SQL against a pooled multi-tenant database** — one missing `WHERE business_id` and you have a cross-tenant incident that AX-001's whole design exists to prevent. Plus procurement has asked for export on termination, so it's now contractual, not hypothetical. Shape: a **per-business export/erase job in `worker`** (it must outlive a request, and that's already the established pattern for long work), kicked off from an admin endpoint in `api`, walking the tenant tables by `business_id`. Deletion as soft-delete plus a scheduled hard purge, so a mistake has a window.

Three things this row forces into the open that no single row owns:
- **It collides with AX-009.** `audit_event` is append-only and holds actor identity and before/after payloads of staff records. An erasure request and an immutable audit log are in direct conflict, and that conflict has to be ruled — typically by pseudonymising the actor and stripping payload fields rather than deleting rows. Nobody has ruled it.
- **It collides with AX-012.** Data retention's upgrade trigger is **literally blank**, and FEAT-011 has meanwhile set a de-facto twelve-month policy for one table. Reconcile the two rows or AX-012 stays forgotten.
- **Export artifacts need somewhere to live,** and Render gives you no object store. That's a new external dependency under a budget freeze. *Stop point.* **Default if you don't reply:** generate the export to the worker's disk and deliver a short-lived signed link by Postmark (no new vendor, works today, caps file size); the alternative is the cheapest S3-compatible bucket, which I'd price and flag rather than adopt unilaterally.

**AX-020 Observability → recommend: decided, and narrower than the row implies.**
INC-31 was found by a customer forty minutes in. Tracing would not have found it sooner — **an alert would have.** So: alert on **Celery queue lag** (p95 is 41s post-resize; set the threshold off that baseline), alert on task failure rate, and alert on audit-write failures, which FEAT-011's own delta says are swallowed from the user and therefore visible nowhere else. Explicitly rejected: distributed tracing. Five containers, three engineers, and Sentry already localises a fault in a chain this shallow — it's machinery nothing pays for. *Trigger:* a call chain deep enough that Sentry names the wrong component, or a second team.

---

## Phase 5 — The health sweep you didn't ask for, which is where the real finding is

You asked me to leave the health view clean. Cleaning it means saying what's dirty.

1. **AX-010 Search — both halves of the trigger have fired, and the row still says `not-now`.** The written trigger is *"a business with more than 50 venues, OR the staff-picker p95 above 800 ms for any business."* August metrics: The Hearth Group has **63 venues**, and its staff-picker p95 is **1,230 ms**. Your largest customer is currently sitting on the exact condition you wrote down as the moment to revisit. **This is the single most important thing in this pass and it is in a row the card never mentioned.** I'd flip its status to *trigger fired — needs re-ruling* and put it at the top of the report. I would not re-rule it myself; that's a desk decision. My prior for the conversation: this is probably a query/index problem before it's a search-engine problem, and the answer may well be "stay on PostgreSQL, fix the query" — but the row has to be re-opened to find out.

2. **AX-005 — marked `in-flight (FEAT-014)`, but FEAT-014 shipped 2026-09-05** and AX-003 already cites it as landed. The row is stale. The provider-call code isn't in this slice, so I'd mark it *needs grading — evidence not in this checkout* rather than flip it on the strength of a changelog line.

3. **AX-011 Reports — marked `in-flight (FEAT-008)`, but FEAT-008 shipped and closed 2026-07-15,** two months ago. I've read `api/reports.py`: read-only, computed on read from the primary, no replica, no warehouse — matches the ruling exactly. Report query p95 is 780 ms against a 2 s trigger. **Recommend flipping to `built`, as-built "as ruled", drift none**, evidence `src/crewboard/api/reports.py`. This one I'm confident enough to write, flagged in the report.

4. **AX-012 Data retention — the upgrade trigger is empty.** A deferral with no fuse isn't deferred, it's forgotten. Needs a trigger, and it's entangled with AX-019 and AX-009 (see above).

5. **Shelf coverage — two dimensions have never been on the ledger at all:** secrets management and dependency/supply-chain policy. They're mentioned in a footnote, which is where concerns go to not get walked. I'd open them as **AX-021** and **AX-022**, unruled, with a note, so they appear in the health view as outstanding rather than as prose at the bottom of a file.

6. **AX-007 event bus** — checked, no fire: rota-publish still has a single consumer. Fine as-is.

---

## Phase 6 — Writes

- **`architecture/concerns.md`** — AX-005 marked stale/needs-grading; AX-009 rewritten with the real as-built and drift; AX-010 flagged trigger-fired; AX-011 recommended to `built`; AX-012 trigger gap flagged; AX-013–AX-020 each carrying a `recommended` stance, reasoning and trigger; AX-021/AX-022 opened; walk history line extended with today's date and "walked solo, stances recommended not ruled"; shelf-coverage footer updated.
- **`architecture/spine.md`** — audit writer recorded as router-level with the drift flag (pending your AX-009 ruling); `worker` purge module noted absent/unverified; `Last confirmed` updated only for what I actually verified against code, with the unverified items named.
- **`architecture/desk-note-2026-09-11.md`** — the plane-readable version: the AX-010 fire and the AX-009 drift on page one, then the eight recommendations as accept/reject lines.
- **No changes to `src/`, `specs/`, or `docs/`.** The FEAT-011 delta stays as approved — it's the record of what the desk decided, and what happened next belongs in the store, not rewritten into the spec.
- **Derived index:** re-rendered from the rows, never hand-edited.

---

## What I'd report back

1. **AX-010's trigger has fired on both clauses** — your biggest customer, 63 venues, 1,230 ms. Needs re-ruling, and you didn't know it was waiting.
2. **FEAT-011 did not ship the shape you approved.** No middleware; `staff`, `billing` and `timeoff` write nothing to the audit log. The per-router approach you rejected at the desk is what's in production, and it failed in exactly the way you said it would. I've recorded it as landed-with-drift rather than `built`, and I need your ruling on finishing the middleware (my recommendation) versus formally re-ruling the row.
3. **Eight rows walked, eight recommendations, none marked as your decision.** One reply — "take them all" or a list of exceptions — converts them.
4. **Three of the eight aren't really deferrable:** rate limiting (an incident already happened, a public surface lands in Q4), backups (never rehearsed), data export/deletion (three requests served by hand SQL against a shared database, and procurement has asked).
5. **Two decisions I need from you and my defaults while I wait:** the AX-009 branch (default: finish the middleware), and where export artifacts live (default: worker disk plus a signed emailed link, no new vendor under the freeze).
6. **Three stale rows and two never-walked dimensions** cleaned up or opened, listed individually.