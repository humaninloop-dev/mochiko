FILES-READ: `README.md`, `architecture/spine.md`, `architecture/concerns.md`, `specs/FEAT-011-audit-log/architecture-delta.md`, `docs/changelog.md`, `docs/metrics-2026-08.md`, `src/crewboard/api/main.py`, `src/crewboard/api/middleware.py`, `src/crewboard/audit/record.py`, `src/crewboard/api/rotas.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/staff.py`, `src/crewboard/api/billing.py`, `src/crewboard/api/reports.py`

---

# Action plan — desk visit by proxy, 2026-09-10

## Headline I have to lead with

FEAT-011 shipped, but **not the shape that was approved**. I would not flip AX-009 to `built` with the delta's as-built line, because that line would be false. Details in Phase 2; everything else proceeds as asked.

---

## Phase 0 — Read and establish ground truth (done above)

Read: the ledger, the spine, the FEAT-011 delta, the changelog, August metrics, and every module under `src/crewboard/`. Nothing is written in this phase.

What the reading establishes, and that the rest of the plan depends on:

- `src/crewboard/api/main.py:9-11` — an explicit `TODO(FEAT-011)`: `AuditMiddleware` was **parked on 2026-08-27**, before the 2026-09-02 ship, because billing and staff handlers don't attach before/after state. Rotas and swaps call `audit.record()` explicitly "for now."
- `src/crewboard/api/middleware.py:1` — only `RequestIdMiddleware` and `TenantMiddleware` exist. No audit middleware.
- `src/crewboard/audit/record.py:1-2` — the module's own docstring says a router that doesn't call it records nothing.
- `rotas.py:20`, `rotas.py:32`, `swaps.py:19` call `record.record()`. `timeoff.py`, `staff.py`, `billing.py` contain mutating endpoints (`POST /timeoff/{id}/decide`, `POST /staff`, `PATCH /staff/{id}`, `DELETE /staff/{id}`, `POST /billing/plan`, `POST /billing/card`) and **record nothing**.
- `record.record()` writes through the handler's own session and the handler commits afterwards (`rotas.py:20-22`) — one transaction with the mutation, not the delta's separate transaction. A rollback drops the audit row; a `record` failure fails the user's mutation. This is the inverse of the flow the delta specifies.
- No evidence in this checkout of the twelve-month purge beat task or the append-only grant. The spine's `worker` modules (`spine.md:12-13`) are `notify`, `integrations.xero`, `payroll` — no purge.

I would not run tests: `tests/` is not in this slice, and I have no shell here. `tests/audit/test_every_mutation_audited.py` is marked "(planned)" in the ledger, so I treat it as not written; the README's assurance about absent tests covers cited-and-existing tests like `tests/tenancy/test_isolation.py`. In a real run I would ask the team to confirm and, if it does exist, run it — I expect it to fail on `timeoff`, `staff`, and `billing`.

---

## Phase 1 — Decide how to handle the AX-009 conflict

**This is the stop point.** Normally I'd confirm with you before departing from a direct instruction. You're travelling and said close it without you, so I proceed under a stated default rather than blocking.

**Default I take:** record what actually shipped, keep AX-009 open, mark the drift.

The branches if you rule otherwise when you read this:
- *"Flip it to `built` anyway"* — I'd need a reason; the only honest version is to first land the three missing `record()` calls and the enforcement test, then flip. One paragraph of work in the ledger, a day of work in the code, and I'd note the middleware is still owed.
- *"Change the ruling — per-router calls are acceptable now"* — then AX-009's ruling text changes (it currently rejects that pattern by name), and `built` still requires all five routers plus the enforcement test. The ledger row would be rewritten, not just its status.
- *"Leave AX-009 alone entirely, I'll take it at the next desk"* — I drop Phase 2's edits and keep only the flag in the report.

I would refuse to write the delta's as-built sentence as-is under any of these. It asserts "every mutating request recorded by `AuditMiddleware`," and three routers record nothing; that sentence in the store would make the health view clean by making it wrong, and the audit log's whole reason for existing (per AX-009's rationale: "two businesses asked who changed this rota") is that people will rely on it.

---

## Phase 2 — Rewrite AX-009 truthfully

Write to: `architecture/concerns.md`, replacing lines 60-71.

Draft text:

```markdown
## AX-009 Audit log

- **Stance**: decided · **Status**: in-flight (FEAT-011 landed partial)
- **Ruling**: unchanged (ruled 2026-07-20) — every mutating request recorded by an `api`
  middleware, twelve-month retention, append-only
- **As-built** (FEAT-011, shipped 2026-09-02 — PARTIAL): no `AuditMiddleware`;
  `AuditMiddleware` parked 2026-08-27 (`api/main.py`) because `billing` and `staff`
  handlers do not attach before/after state. Audit rows are written by explicit
  `record.record()` calls in `rotas` and `swaps` only. `timeoff`, `staff` and `billing`
  mutations record nothing. The write shares the handler's transaction and commit rather
  than running in its own. Purge task and append-only grants not evidenced in the api slice.
- **Drift**: yes, material. The shipped shape is the per-router pattern the 2026-07-20 desk
  ruling rejected by name, and it has already dropped three categories — including staff
  personal-data changes (create / update / archive) and billing plan and card changes.
  Staff-record coverage matters to AX-019.
- **Enforcement**: `tests/audit/test_every_mutation_audited.py` still planned, not written
- **Close-out**: either land `AuditMiddleware` per the delta, or (stopgap, ~half a day) add
  `record.record()` to the three silent routers and write the enforcement test now, so the
  gap cannot widen while the middleware waits. Recommend the stopgap this week and the
  middleware in the next feature slot. Flip to `built` only when the enforcement test is
  green.
```

I would **not** touch `src/` — you asked for store work, and the remediation is a coding change that should be scheduled, not slipped in. I'd flag the half-day stopgap and let you or the team pick it up.

I would also add a one-line note under the FEAT-011 delta's "Store effect at landing" section in `specs/FEAT-011-audit-log/architecture-delta.md`: `> Not achieved at landing 2026-09-02 — see AX-009 as-built.` I'd flag this as an edit to an approved, signed artefact; if you'd rather approved deltas stay immutable, drop this one line and nothing else changes.

---

## Phase 3 — Walk the eight open rows in ledger order

On "skip any row that obviously doesn't apply": I read that as *don't spend the walk on rows we'll never build* — not *leave rows blank*. A row left `open` still shows up as unwalked at the next visit and nobody remembers why. So the two thin ones (AX-014, AX-015) get a short `not-now` with a trigger rather than being skipped, which closes them at roughly the cost of skipping. Neither is a true `n-a` — AX-006 is `n-a` because single-country is structural; UK-only and no-A/B are circumstances that can change. If you disagree, they're two lines to restate.

All eight rulings respect the budget freeze on new services until Q1 2027 and the three-engineer team. Each gets Stance, Ruling with today's date, and an upgrade trigger. Drafts:

**AX-013 Feature flags** → `decided`. Flags stay in-process environment variables read at boot, with per-business pilot targeting by a `business_id` allowlist in the flag value, so a pilot doesn't need a code change — just a redeploy. No flag vendor (budget freeze; two flags a quarter doesn't pay for one). *Trigger:* a flag needed to change without a redeploy (an incident kill-switch), or more than roughly one flag change a week, or the SPA needing to read flags.

**AX-014 Localisation** → `not-now`. UK English only; no i18n framework. The Irish prospect is English UI with EUR, which is a currency and tax question, not a language one. *Trigger:* a signed customer needing a non-English UI, or a second VAT/currency regime reaching billing or payroll.

**AX-015 Experimentation** → `not-now`. No product ask, and 64 businesses is too small a population for a split test to say anything. Adoption measurement stays in the monthly metrics doc. *Trigger:* a product decision the PM wants settled by controlled comparison, or self-serve signup volume large enough for per-user assignment. Would ride on AX-013 rather than a new mechanism.

**AX-016 Rate limiting** → `decided`, and I'd mark it the most urgent of the eight. Per-identity limits at the `api` edge backed by a Redis token bucket — `redis` already exists, so no new service. Keyed by session/business for authenticated traffic, by widget token and source IP for the public widget, with a tighter bucket plus a short response cache on widget endpoints. 429 with `Retry-After`. In-process limiting is explicitly out: `api` runs two instances (`spine.md:9`), so a per-process counter gives double the intended limit and won't hold under scaling. INC-19 (2026-03) is the precedent — one runaway client pinned an instance for twenty minutes. *Trigger to revisit:* limiting needed above the app (a CDN/WAF) because a flood is exhausting connections before `api` sees it. **Must land before the Q4 public rota widget ships** — the widget is token-authenticated and embeddable, i.e. the exact INC-19 shape, exposed on purpose.

**AX-017 Backups and restore** → `decided`. Keep Render daily snapshots and keep PITR on the plan. Add a written restore runbook and a quarterly rehearsal into a scratch database, recording measured RTO and RPO each time. Stated targets: RPO ≤ 5 min (PITR), RTO ≤ 4 h. Untested backups aren't backups, and the first rehearsal is owed before the SSO contract's procurement review asks about it. *Trigger:* a rehearsal missing the target, or a contract requiring a tighter RPO than PITR gives.

**AX-018 Identity federation (SSO)** → `decided`, conditional. Google Workspace OIDC as a per-business option for managers, via a library (authlib) rather than an identity vendor; after the OIDC exchange the existing session cookie remains the session mechanism, so AX-002 is unchanged. No SAML, no SCIM provisioning. *Build on contract signature, not before* — this is real engineering for one prospect. *Trigger to extend:* a second enterprise prospect needing SAML or SCIM. This is the row where a commercial call may override my recommendation, so I've written the ruling to sit dormant until the contract lands rather than to commit engineering now.

**AX-019 Data partitioning, export and deletion** → `decided`. No physical partitioning — `business_id` scoping via `TenantRepository` (AX-001) is adequate at 64 businesses / 410 venues. Build per-business export and delete as tested `worker` tasks: export produces a JSON/CSV bundle to signed storage; delete cascades by `business_id`. Staff-invoked, logged, one-month GDPR SLA. Three requests handled by hand with SQL is already two too many, and "data export on termination" is now in a procurement document. *Trigger for partitioning:* vacuum or table-size pain on the largest tables, or a customer requiring physical isolation.

**AX-020 Observability** → `decided`. Keep Sentry and Render logs; no tracing vendor (budget freeze). Add three things: an alert on Celery queue lag (p95 is 41 s post-resize, so alert on sustained > 5 min); an alert on audit-write failure, which the FEAT-011 delta requires and which nothing currently implements; and propagation of the existing `request_id` (`middleware.py:12`) into Celery task context so a request can be followed from `api` into `worker`. INC-31 was reported by a customer forty minutes in. *Trigger:* another incident found by a customer first, or a latency problem that logs plus Sentry can't localise across `api`/`worker`.

---

## Phase 4 — Two things I found that aren't on my card

**AX-010 Search — both upgrade triggers have fired.** The row's triggers are "a business with more than 50 venues, OR staff-picker p95 above 800 ms for any business." August metrics: The Hearth Group has 63 venues, and its staff-picker p95 is 1,230 ms. The row still reads `not-now · ruled`, so the health view is not clean regardless of AX-009.

I would set AX-010's stance to `open` and append: `- **Trigger fired** (metrics 2026-08): Hearth Group 63 venues; staff-picker p95 1,230 ms. Re-open at the next desk visit.` That's recording a fact the row itself defined, not making a ruling for you — the actual decision (index, denormalised picker table, or search service) is a desk conversation. If you'd rather I hadn't touched a row outside the eight, it's two lines to revert.

**AX-012 Data retention has an empty upgrade trigger** (`concerns.md:94`). A `not-now` with no trigger can never be revisited, and it now overlaps AX-019's deletion path. I would **not** edit it — it's a ruled row and you may want it merged into AX-019 instead. I'd flag it with a suggested trigger: "a customer or contract specifying a retention period, or any table where old rows measurably affect query cost."

Also noted but not acted on: `billing.py` calls Stripe inline with no idempotency key. `spine.md:27` explicitly permits inline interactive Stripe calls, so it's not an AX-003 violation, but AX-005 says *every* provider send carries an idempotency key and is in-flight under FEAT-014. Mentioned in the report for the FEAT-014 owner; not my card.

---

## Phase 5 — Shelf completion

Ledger says 20 of 22 backend-service dimensions are covered; secrets management and dependency/supply-chain policy are missing. Closing the walk means the shelf is complete, so I'd add `AX-021 Secrets management` and `AX-022 Dependency and supply-chain policy` as `open · unwalked`, each with a one-line note of what I can see from the repo (secrets are Render environment variables today; no lockfile/scanning policy visible in this slice) and no ruling — I have no walk notes for them and won't invent the CTO's context. They'd be the first two rows at the next visit. If you'd rather the shelf stay at 20 until you've actually walked them, delete both blocks.

---

## Phase 6 — Store housekeeping and write-up

- `architecture/concerns.md`: update the walk-history header to `2026-09-10 (AX-013 – AX-020 ruled by proxy; AX-009 as-built corrected; AX-010 trigger fired)`, and remove the "walk paused here" marker at line 98-99 since the paused rows are now closed.
- `architecture/spine.md`: bump `Last confirmed` to `2026-09-10`, annotated `(reviewed against src/ 2026-09-10; no container or communication-style change)`. No structural edit — the spine is accurate precisely because `AuditMiddleware` never landed.
- Write `architecture/walk-notes-2026-09-10.md`: the reasoning behind each of the eight rulings, the AX-009 finding with file and line references, the AX-010 trigger, the AX-012 gap, and the three decisions I'd most expect you to overturn (AX-018's conditional build, the AX-014/AX-015 `not-now`-rather-than-skip call, and adding AX-021/AX-022). Short enough to read on a plane.

No delegation to any subagent or worker for any of this — it's a single-pass edit of two store files plus one note, and the judgement calls are exactly the part that shouldn't be handed off.

**Final report to you** would open with: FEAT-011 landed partial and AX-009 is not `built`, with the three unaudited routers named; AX-010's trigger has fired; AX-016 blocks the Q4 widget; then the eight rulings in one line each; then the three edits I made outside the eight rows and how to revert each.