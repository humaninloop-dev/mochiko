FILES-READ: `README.md`, `architecture/concerns.md`, `architecture/spine.md`, `specs/FEAT-011-audit-log/architecture-delta.md`, `docs/changelog.md`, `docs/metrics-2026-08.md`, `src/crewboard/api/main.py`, `src/crewboard/api/middleware.py`, `src/crewboard/audit/record.py`, `src/crewboard/api/rotas.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/staff.py`, `src/crewboard/api/billing.py`, `src/crewboard/api/reports.py`

---

# Action plan — async shelf walk, crewboard, 2026-09-09

## Upfront: one finding that changes the shape of the job

You asked me to flip AX-009 to `built` and copy the as-built line out of the approved delta "so the health view is clean." I read the code before writing anything, and the delta did not land. Details in Phase 3 — the short version is that `AuditMiddleware` does not exist, the routers call `audit.record()` directly (the design the desk explicitly rejected on 2026-07-20), and three of the five mutating routers — `timeoff`, `staff`, `billing` — record nothing at all. Copying the delta's as-built line in would make the health view clean by making it false, on exactly the row where the ledger's rationale says the failure is silent.

I am not going to write a line I know to be untrue into the store, so my default is to record AX-009 accurately with drift, and I do the other eight rows in full regardless. This is a one-line reversal if you disagree.

## Phase 0 — Two readings I settle myself, and state in the file

**Stop I'd normally take, resolved by default because you're unreachable:**

1. **"Mark each one `decided`."** The ledger's own vocabulary has three *closing* stances — `decided` (AX-001…), `not-now` (AX-007, AX-010, AX-012), `n-a` (AX-006) — all of which carry `Status: ruled` and close the row. Forcing the literal word `decided` onto feature flags and A/B testing would commit three engineers under a budget freeze to building things the evidence says they don't need. I read your instruction as "close every row with a ruling," not "put one specific word on all eight," and I use the full vocabulary. **If you meant it literally**: the three non-`decided` rows are AX-013, AX-014 (`not-now`) and AX-015 (`n-a`); converting them is a stance-word edit on three rows and changes no rationale.
2. **"Skip any row that obviously doesn't apply to a product our size."** A skipped row stays `open`, which leaves the walk unfinished — the opposite of the goal. I treat "skip" as "don't spend deliberation on it," and close it `n-a` with a reason, the way AX-006 was closed. Exactly one row qualifies (AX-015). **If you meant leave it untouched**: delete the AX-015 edit; nothing else depends on it.

Both readings go into the walk note in plain sight so you can overturn either from the plane.

## Phase 1 — Read the evidence

Already done (files above). What I extract and carry into the rulings:

- **Team and constraints** (README): three engineers; 64 businesses, 410 venues; **budget freeze on new services until Q1 2027**. This is the binding constraint on every ruling — I bias hard toward using the `redis` and `worker` we already pay for.
- **Metrics (2026-08)**: largest business The Hearth Group at **63 venues, 1,900 staff**; staff-picker p95 **340 ms overall but 1,230 ms for Hearth**; report p95 **780 ms**; Celery queue lag p95 **41 s** post-resize; **3 GDPR requests** handled by hand.
- **Changelog**: FEAT-011 shipped 2026-09-02; FEAT-014 shipped 2026-09-05; FEAT-008 shipped *and closed* 2026-07-15.
- **Code**: the audit surface, router by router (Phase 3).

No delegation anywhere in this job. It is fourteen small files and one judgment-heavy document; a subagent would add a hand-off summary between me and the evidence, which is precisely where accuracy would be lost.

## Phase 2 — Write the eight rows in ledger order

Single edit pass on `architecture/concerns.md`, rows AX-013 → AX-020. Each row gets stance, status, a dated ruling line, and — for anything deferred — a **falsifiable** upgrade trigger, because the AX-010 failure below shows this ledger's triggers are its only early-warning system.

**AX-013 Feature flags → `not-now` / `ruled`.**
Two flags a quarter against an env-var-plus-redeploy is inside the existing deploy cadence; a flag service is a new vendor under the freeze. Ruling records the *next* step rather than the end state: when triggered, a `business_flags` table read per request — no new service. Trigger: a flag that must flip without a deploy (incident kill-switch), or per-business targeting beyond a single pilot, or more than one new flag a week. Cross-link: AX-018's per-business SSO toggle will fire this, so it may not stay parked long.

**AX-014 Localisation → `not-now` / `ruled`.**
Recorded with a correction to the framing from the walk: the live need in the Irish prospect is **EUR and date/number formatting**, not translation, and currency touches Stripe and PayFlow (UK-only payroll) rather than the UI layer. Ruling defers a translation framework; trigger is a signed non-UK customer needing a second language, or a second currency reaching billing — and flags the payroll question as the real work if the Irish deal closes.

**AX-015 Experimentation (A/B) → `n-a` / `ruled`.** *(the "skip" row)*
64 business tenants cannot power a split test to significance; there is no product ask; the PM already measures adoption from the metrics doc, which is the right instrument at this size. Reason line notes what would change it: a self-serve motion with thousands of tenants.

**AX-016 Rate limiting → `decided` / `ruled`.** *This is the highest-value row on the list.*
INC-19 (2026-03) already took an `api` instance to 100 % CPU for twenty minutes with an authenticated client, and Q4 adds an embeddable public rota widget on the same two instances. Ruling: per-token and per-IP limiting in an `api` middleware backed by the **existing** `redis` — no new service, no spend; a separate, tighter budget for the public widget routes than for authenticated API traffic; `429` with `Retry-After`; per-business override. Ruling states it is a **gate on the widget shipping**, not a follow-up, because the widget is the thing that makes an unmetered surface public. Enforcement test named in the row: a test that drives a widget route past its budget and asserts `429`.

**AX-017 Backups and restore → `decided` / `ruled`.**
Seven-day snapshot retention with a restore that has never been rehearsed is an untested hypothesis, and PITR is already on the current plan — so this costs attention, not money. Ruling: turn PITR on; rehearse a restore into a scratch database **quarterly** with the achieved RTO written down; state RPO/RTO targets explicitly. Row notes the new coupling: with `audit_event` append-only and purged at twelve months, backups are now the only recovery path for audit history.

**AX-018 Identity federation (SSO) → `decided` / `ruled`, scoped narrowly.**
Largest contract in the pipeline, 40 venues, asking for Google Workspace SSO for managers. Ruling: **OIDC against Google Workspace only**, `manager` role only, per-business toggle, matching on verified email to an existing user; the AX-002 session cookie stays as the session mechanism — SSO replaces the login step, not the session. Explicitly rules **out** a general SAML/multi-IdP federation layer, which is where a three-person team loses a quarter. Ruling is conditioned on contract signature and notes it consumes the per-business config surface AX-013 deferred.

**AX-019 Data partitioning, export, and deletion → `decided` / `ruled`.**
Three subject requests already handled by hand with raw SQL, and the SSO prospect's procurement is asking for export on termination — the demand is here, and `business_id` is already on every table from AX-001, so the groundwork exists. Ruling: per-business export and per-business deletion as `worker` tasks keyed on `business_id`, producing one archive to object storage behind a signed expiring link; deletion is two-step (mark, then verified purge after a hold period) against an explicit **table manifest**. The durable part of the ruling is a test that fails when a new tenant table appears that the manifest doesn't list — the failure mode here is a forgotten table, the same class of failure as AX-009's forgotten router, and the ledger has now been bitten by that class once.
The row also records a conflict I will not resolve silently: **append-only audit (AX-009) versus subject deletion**. My recommendation in the ruling is that audit rows are retained but personal fields inside their `before`/`after` payloads are redacted on deletion — flagged as needing your explicit sign-off, because it is a legal position as much as an architectural one.

**AX-020 Observability → `decided` / `ruled`.**
INC-31 was reported *by a customer* forty minutes in — that is an alerting gap, not a tracing gap. Ruling puts alerting first and defers distributed tracing: alert on Celery queue lag (threshold set well above the 41 s post-resize p95 — sustained breach, not a spike), on worker task failure rate, on `api` 5xx and latency, and on the audit-write failure path the FEAT-011 delta promised would be "logged and alerted" (Phase 3 shows that path does not exist, so this ruling and the AX-009 remediation are linked). Tracing deferred with a trigger: a latency problem that can't be localised from logs, or a third hop in a request path. `RequestIdMiddleware` already stamps a correlation id, which is enough at five containers.

## Phase 3 — AX-009, written honestly

**What I verified in the code:**

| The approved delta says | `src/` actually contains |
|---|---|
| `AuditMiddleware` registered on the app, single writer | `api/main.py:9-11` — a TODO parking it, dated 2026-08-27; `api/middleware.py:1` states outright that no audit middleware exists |
| No router calls the audit module directly | `api/rotas.py:20,32` and `api/swaps.py:19` call `record.record()` directly |
| Every POST/PUT/PATCH/DELETE recorded | `timeoff.py` (decide), `staff.py` (create/update/delete), `billing.py` (plan/card) record **nothing** |
| Middleware writes in its **own** transaction, after the handler commits | `record()` runs inside the handler transaction, before `db.commit()` |
| Failure logged and alerted, never surfaced | no such path exists |
| `worker` beat task purges at twelve months | not present in this slice — **unverified**, not asserted either way |

The gap is not cosmetic. `staff.py` is annotated as personal data — name, mobile, email, contracted hours — and its create, update and delete are unaudited. `billing.py` mutates plan and card and is unaudited. The desk rejected per-router calls because "a router that forgets the call silently drops a category," and the shipped code drops three.

**What I write instead of the delta's line** (`architecture/concerns.md`, AX-009):

- **Stance**: stays `decided` — the 2026-07-20 ruling is right and I am not reopening it.
- **Status**: stays `in-flight (FEAT-011)`. Not `built`.
- **As-built (FEAT-011, shipped 2026-09-02)**: factual — explicit `record()` calls from `rotas` and `swaps` only; no middleware; write inside the handler transaction.
- **Drift**: `yes` — names the three unaudited routers, notes this is the rejected design, notes the append-only grants and the twelve-month purge are unverified from this slice.
- A follow-up work item to land the middleware and retire the direct calls, gated on `billing` and `staff` attaching before/after state (the reason it was parked).
- The enforcement test `tests/audit/test_every_mutation_audited.py` is still marked **planned**, not green.

**Test I would run before finalising this row** (would need shell, which I don't have here): check whether `tests/audit/test_every_mutation_audited.py` exists in the full repo and run it. The README says a ledger-cited test absent from this slice exists upstream. Two outcomes, both findings: if it's absent, the row's "planned" is accurate and nothing is enforcing the ruling; if it's present and **passing**, it has been weakened — as written ("one mutating request per router, assert a row for each") it must fail on `timeoff`, `staff` and `billing`. I'd report which, and I'd expect the latter to be the more serious discovery.

**I make no changes under `src/`.** You asked for a shelf walk, not for FEAT-011 to be finished; building `AuditMiddleware` and reworking five routers is a feature's worth of work and its own review.

**The stop, and the branches.** What I'd put to you: *do we record AX-009 as built per the delta, or as it actually shipped?* If you rule **built as delta** — I'd want that in writing, and I'd still leave the drift note, since the code is checkable by anyone. If you rule **as-shipped** (my default, already written) — no further action. If you rule **finish FEAT-011 first** — the ledger stays as I've written it and the middleware becomes the next feature.

## Phase 4 — Findings outside my remit (recorded, not acted on)

Your authority covered eight open rows plus AX-009. These four turned up while reading and are reported rather than silently rewritten:

1. **AX-010 Search — both upgrade triggers have already fired.** The trigger is ">50 venues OR staff-picker p95 >800 ms." The Hearth Group is **63 venues at 1,230 ms p95**. The row still reads `not-now`. This is the ledger's early-warning mechanism failing in production. **The one exception I'd make**: I add a dated line under AX-010 recording that the trigger has fired and the row needs a ruling — I do **not** change its stance. Recording a fact is reversible; ruling on search without you is not.
2. **AX-011 Reports** — marked `in-flight (FEAT-008)`, but FEAT-008 shipped and closed 2026-07-15. Report p95 is 780 ms, under the 2 s trigger, so the ruling holds; the status is just stale. Should be `built`. Reported, not edited.
3. **AX-005 Provider-call idempotency** — marked `in-flight (FEAT-014)`; FEAT-014 shipped 2026-09-05 and AX-003 already cites it as landed. Probably `built`, but the `worker` and `notify` code isn't in this slice so I can't confirm the idempotency keys. Reported as "verify, likely built."
4. **AX-012 Data retention** — the **upgrade trigger line is empty**; the row was never finished. And FEAT-011 has now introduced a twelve-month audit purge, so the product *has* retention behaviour that the row denies. Couples to AX-019.

**Shelf coverage**: the ledger's own footer names secrets management and dependency/supply-chain policy as missing from 22 dimensions. I'd draft both as `open` rows with walk notes for the next desk visit — opened, not ruled. Closing dimensions you've never discussed would be me inventing a walk that didn't happen.

## Phase 5 — Files written

- **`architecture/concerns.md`** — the eight rows closed; AX-009 rewritten with drift; a dated trigger-fired note on AX-010; two new `open` shelf-gap rows; walk-history header updated to note 2026-09-09 was an async walk, not a desk visit.
- **`architecture/spine.md`** — minimal. Containers, modules and communication styles are all still accurate for what is *built*; the new rulings (rate-limit middleware, export/delete worker tasks, Google as an external system) are decisions, not deployments, and don't belong in the spine yet. I update `Last confirmed` and mark it as async.
- **`architecture/walk-2026-09-09.md`** — new. The plane read: one-line ruling per row with the evidence behind it, the AX-009 problem stated plainly, the four out-of-remit findings, and a short list of what needs your ruling.

Nothing under `src/`, `specs/` or `docs/` is touched.

## Phase 6 — What I'd report back

Eight rows closed: five `decided` (rate limiting, backups, SSO, data export/deletion, observability), two `not-now` with triggers (feature flags, localisation), one `n-a` (A/B testing). Highest-value ruling is AX-016 — rate limiting has an incident behind it and a public widget in front of it, and it needs to gate the Q4 ship.

Then the two things to decide on the plane, stated without softening: **AX-009 is recorded as in-flight with drift, not built** — the audit middleware was never written, and staff, billing and time-off mutations are going unrecorded today, which is the exact hole the ruling was made to close. And **AX-010's upgrade trigger fired some time ago and nobody noticed** — a customer with 63 venues is seeing 1,230 ms on the staff picker against an 800 ms threshold.

The health view will not be clean when you look at it. It will be accurate, which on this particular row is the thing worth having.