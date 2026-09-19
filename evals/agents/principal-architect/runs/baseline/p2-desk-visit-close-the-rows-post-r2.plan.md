FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-011-audit-log/architecture-delta.md`, `docs/changelog.md`, `docs/metrics-2026-08.md`, `src/crewboard/api/main.py`, `src/crewboard/api/middleware.py`, `src/crewboard/api/rotas.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/staff.py`, `src/crewboard/api/billing.py`, `src/crewboard/api/reports.py`, `src/crewboard/audit/record.py`

---

# Action plan — desk-visit prep, `architecture/` store

## Headline before the phases

Two parts of the card I will not carry out as written, and I'd rather you hear why on the plane than discover it later:

- **AX-009 cannot be flipped to `built` with the delta's as-built line.** The line in the approved delta describes a system that was not built. FEAT-011 shipped the per-router shape that was *rejected* at the 2026-07-20 desk — and three of the six mutating routers record nothing at all. Copying that line in would put a false claim into the store.
- **I will not mark eight rows `decided`.** `decided` means *you* ruled. I'll put a full, specific recommendation with a reason and a trigger on every one of the eight, signed as mine and dated, with the stance left `open` — so the plane read is a yes/no per row and the walk closes in ten minutes when you land. And I won't skip any row as "obviously doesn't apply"; the two that look inapplicable get a written n-a recommendation *with* a trigger, which costs seconds and is the whole value of a shelf walk.

---

## Phase 1 — Grade the FEAT-011 claim against the code

**Do:** check the approved delta line by line against `src/`, because the store's as-built line is worth exactly what it was checked against.

**Read (already done):** the delta; `api/main.py`, `api/middleware.py`, `audit/record.py`, and all six routers.

**What the code says:**

| Delta promised | Reality in `src/` |
|---|---|
| `AuditMiddleware` registered on the app, single writer | Does not exist. `main.py` registers only `RequestIdMiddleware` and `TenantMiddleware`, with a `TODO(FEAT-011)` parked 2026-08-27 |
| No router calls the audit module directly | `rotas` and `swaps` call `record.record()` inline — the rejected shape |
| Every POST/PUT/PATCH/DELETE recorded | 3 of 9 mutating routes recorded. Unaudited: `timeoff.decide`, `staff.create`, `staff.update`, `staff.remove`, `billing.change_plan`, `billing.update_card` |
| before/after diff on every mutation | `rotas.publish` records no before/after; `staff` and `billing` record nothing |
| `audit_event` append-only, no UPDATE/DELETE grant | **Unverified** — no migration or grant file in this checkout |
| `worker` beat task purges at twelve months | **Unverified** — worker source is not in this slice |

I will report the last two as *unverified*, not as *missing*. The README says this checkout is a slice; claiming absence from a slice is exactly the kind of guess I refuse to write down.

**The structural point, not just the tally:** the reason the desk rejected per-router calls was that a router which forgets the call silently drops a category. That has already happened — `staff` holds personal data (name, mobile, email, contracted hours) and has no audit trail at all, which is precisely the category a subject-access request asks for. The responsibility is smeared across six routers with no single owner.

**Note on why it drifted:** the TODO says the middleware was parked because before/after diffs need handlers to attach state, which `billing` and `staff` don't do. That's a real obstacle and worth recording — but it argues for a middleware that records actor/business/path/method with a *degraded* diff for handlers that don't attach one, not for abandoning the single writer. I'd put that as the recommended remediation.

**Delegation:** none here. Every file in the checkout is read — all fifteen — so there is no sweep left to farm out. If this were the full repo I'd send one throwaway `Explore` at `model: haiku` with the brief "return file:line for every occurrence of `audit_event`, `AuditMiddleware`, and any Celery beat schedule entry, plus any migration creating `audit_event`; facts only, no interpretation," and on return I'd re-open each cited line myself before writing anything into the store, since a wrong absence here would corrupt the as-built line.

---

## Phase 2 — The stop on AX-009, and both onward branches

**This is where I'd stop for you.** What I'd confirm: *FEAT-011 landed the rejected shape. Do we fix the code to the ruling, or re-rule the ledger to match the code?*

- **If you rule "fix the code":** AX-009 stays `in-flight`, now against a remediation item (call it FEAT-011a), with the as-built line describing the partial state and the gap named. I'd sketch the smallest honest delta: register `AuditMiddleware` as the single writer, have it record actor/business/path/method/entity unconditionally, let handlers optionally attach a before/after diff to request state, and delete the two inline `record.record()` calls so there is one writer again. No new container, no new dependency — it's `api`-internal.
- **If you rule "re-rule to per-router":** AX-009 becomes `decided` on a *new* ruling with your date, `built`, and the enforcement test becomes load-bearing rather than planned — because per-router only works if something fails the build when a router forgets. I'd say plainly that I don't recommend this: the enforcement test has to enumerate routes anyway, and if you're enumerating routes you may as well intercept them.

**My default while you're away (what I'd actually write):** the truthful middle. Status `built` (the *feature* shipped — pretending otherwise breaks the changelog), with an as-built line that says what is there, `Drift: yes` naming the six unaudited routes and the missing middleware, and the remediation recommended but not ruled. That way the health view is *accurate*, which is what "clean" should mean. I'd rather hand you a clean-and-true store than a clean-looking one.

**Test I would specify (not write — this is plan-only):** `tests/audit/test_every_mutation_audited.py`, the one the ledger already cites as planned. It should discover every route on the app with method POST/PUT/PATCH/DELETE, fire one request at each, and assert an `audit_event` row. **Expected result today: fail, 6 of 9** — `timeoff.decide`, three `staff` routes, two `billing` routes. I'd flag that the ledger currently lists this test under **Enforcement** as though it guards something; it doesn't exist yet, and an enforcement claim that isn't running is worse than none.

---

## Phase 3 — Sweep the rest of the store while I'm in there

You asked for a clean health view; these are what the sweep turns up, and three of them are more urgent than any of the eight open rows.

1. **AX-010 Search — trigger has fired, twice.** Its upgrade trigger reads: more than 50 venues, *or* staff-picker p95 above 800 ms for any business. August metrics: The Hearth Group is at 63 venues and 1,230 ms. A deferral whose fuse has burned down is not deferred. I'd move it from `not-now` to `open` with a note that the trigger fired, and recommend the cheap first step — index/query work on the existing PostgreSQL before anyone says the word "Elasticsearch," which the budget freeze rules out anyway.
2. **AX-011 Reports — stale in-flight.** Marked in-flight against FEAT-008, which the changelog shows shipped and closed on 2026-07-15. Needs an as-built line. Its own trigger (2 s p95) has *not* fired — report p95 is 780 ms — so the stance itself still holds.
3. **AX-005 Provider-call idempotency — stale in-flight.** Marked in-flight against FEAT-014, which shipped 2026-09-05 (AX-003 already cites that landing). I'd mark it needing an as-built line but leave the line blank and flagged, since worker source isn't in this slice and I won't write a claim I couldn't check.
4. **AX-012 Data retention — a deferral with no fuse.** The upgrade-trigger field is literally empty. Also it now contradicts reality: FEAT-011 introduced a twelve-month retention rule for `audit_event`. I'd propose a trigger and cross-link it to AX-019.
5. **Shelf gaps.** The coverage note says 20 of 22 dimensions are on the ledger. I'd open `AX-021 Secrets management` and `AX-022 Dependency and supply-chain policy` as `open` rows with walk notes. Opening a row commits nothing; leaving a dimension unopened is how it gets retrofitted a year later.
6. **Spine.** `audit` is already listed as an `api` module, so FEAT-011 needs no box change. I would **not** bump `Last confirmed` to today — I can only confirm the `api` slice, not `worker` or `db`. I'd add one dated line saying the api-side audit shape was checked on 2026-09-11 against a partial checkout.

---

## Phase 4 — The shelf walk: eight recommendations, in ledger order as you asked

Ledger order for the write-up, since that's how you'll read it. But flagged at the top of the file: **AX-019 and AX-016 should not wait for the next visit** — one has a trigger that already fired three times, the other has an incident behind it and a Q4 deadline in front of it.

Each row gets three lines in the file: `Recommendation (mochiko, 2026-09-11 — NOT RULED)`, `Because`, `Would change my mind`.

- **AX-013 Feature flags** → recommend **not-now**. Two flags a quarter and env-var-plus-redeploy is genuinely adequate; a flag service is a box nothing pays for. *Changes my mind:* a flag that must differ per business without a redeploy, more than ~6 live at once, or an incident where the rollback needed a deploy. The step then is a small `business_feature` table in the existing `db` — not a vendor.
- **AX-014 Localisation** → recommend **n-a for language, decided for money**. Not a skip, because the Irish prospect isn't a language problem, it's a currency one. Store an explicit currency code on monetary values now rather than assuming GBP; that's a column, and retrofitting it after the first EUR customer is a migration across live billing data. *Changes my mind on language:* any customer needing a non-English UI.
- **AX-015 Experimentation** → recommend **n-a**. The one row that truly doesn't apply — and it still gets written down with a fuse. *Changes my mind:* a product question that needs a concurrent comparison rather than before/after adoption from the metrics doc.
- **AX-016 Rate limiting** → recommend **decided, and build it before the widget**. INC-19 already cost twenty minutes of an `api` instance at 100% CPU, and a token-authenticated public rota widget in Q4 is a near-anonymous surface arriving on a date. Shape: per-token and per-session limiting in the existing `api` middleware stack, counters in the `redis` you already run. No new container — the budget freeze holds. And this is a box that shouldn't be hand-written: use a maintained limiter library rather than rolling a token bucket. *Changes my mind:* the widget slipping out of Q4 (delays it, doesn't cancel it — INC-19 stands on its own).
- **AX-017 Backups and restore** → recommend **decided**. Turn on point-in-time recovery, which the current plan already includes, and rehearse one restore into a scratch database this quarter with the measured time written into the ledger. A backup nobody has restored is a hypothesis. Near-zero cost, no new service. *Changes my mind:* nothing — I'd argue this one hard.
- **AX-018 Identity federation** → recommend **not-now, with the shape pre-agreed**. This is a commercial call, not a technical one, so it's a genuine stop for you. If the 40-venue contract signs, add Google Workspace OIDC *only*, as a second login path onto the existing session cookie from AX-002, via a library — not a generic SAML/OIDC federation layer, which is a structure one prospect does not pay for. *Changes my mind:* contract signed, or a second prospect asking.
- **AX-019 Data partitioning, export, deletion** → recommend **decided; the trigger has already fired three times**. Three subject-access/deletion requests handled by hand with SQL, plus procurement asking for export-on-termination. Hand-run SQL against production for a deletion is an unaudited, unrepeatable operation on personal data. Shape: one owner — a `worker` task that walks tables by `business_id` and produces an export, and a tested, documented deletion cascade. Two things I'd raise rather than invent: there is **no object store in the spine** for an export artifact to live in, and the budget freeze is in the way — so I'd name that gap and let you pick, rather than quietly drawing a bucket into the diagram. Second point: `staff` — the table holding the personal data — currently has **no audit trail**, which directly weakens the answer to a subject-access request. AX-019 and AX-009's drift are the same wound.
- **AX-020 Observability** → recommend **split it**. Decided on the cheap half now: an alert on Celery queue lag (p95 is 41 s *after* the resize) and an alert on audit-write failure — the FEAT-011 delta promises failures are "logged and alerted" and nothing alerts today. INC-31 was reported by a customer forty minutes in; that's what pays for it. Tracing: **not-now**, *changes my mind:* a latency problem spanning `api` → `worker` → provider that Sentry and logs can't resolve inside an hour.

---

## Phase 5 — Writes

Only two files. Everything stays in the store; I won't spawn a side document that becomes a second source of truth, and I won't hand-edit any derived index.

**`architecture/concerns.md`**
- New dated prep block at the top: walk-history entry for 2026-09-11 marked *prep, not a walk*, the two rows that shouldn't wait, and a one-line explanation that eight recommendations are recorded unruled.
- **AX-005, AX-011**: stale in-flight flagged; AX-011 gets its as-built; AX-005's left blank and marked unverifiable from this checkout.
- **AX-009**: status `built`, as-built rewritten to the truth (middleware never landed; `rotas` and `swaps` call the audit module inline; `timeoff`, `staff`, `billing` record nothing), `Drift: yes` with the six routes named, the enforcement test marked *not yet written*, the parking reason from `main.py` preserved, and the remediation recommended-not-ruled.
- **AX-010**: `not-now` → `open`, trigger-fired note with the August numbers.
- **AX-012**: proposed trigger, cross-linked to AX-019.
- **AX-013–AX-020**: stance stays `open`; each gains recommendation / because / would-change-my-mind.
- **AX-021, AX-022**: opened.
- Shelf-coverage line updated to 22 of 22.

**`architecture/spine.md`** — one dated confirmation line about the api-side audit shape; `Last confirmed` **not** advanced.

**Refused writes:** stance `decided` on any of the eight; the delta's as-built sentence for AX-009; any claim about the `audit_event` grants or the purge task.

**No tests written or run** — plan-only, and no shell here regardless. The one test I'd specify is in Phase 2, with its expected failure.

---

## Phase 6 — What I'd report

A short plane-read at the top, then the detail:

1. **FEAT-011 did not land as approved.** The rejected shape shipped; six of nine mutating routes are unaudited, including all of `staff`, which holds the personal data. AX-009 is recorded `built` with drift, not clean. One question for you: fix the code, or re-rule the ledger.
2. **Eight recommendations are waiting, none ruled.** Your yes/no on each closes the walk. AX-016 and AX-019 are the two I'd act on before the next visit; AX-017 is nearly free and I'd take it today.
3. **Three stale rows and a fired trigger** found in the sweep — AX-010's search deferral has expired on both conditions, AX-005 and AX-011 are marked in-flight for work that shipped, AX-012 has a deferral with no fuse.
4. **Two blind spots** I could not check from this slice (the `audit_event` grants, the twelve-month purge) and two shelf dimensions now opened.
5. **One thing I need from you rather than from the code:** whether the budget freeze can bend for somewhere to put a per-business data export, because AX-019 has no honest answer without it.