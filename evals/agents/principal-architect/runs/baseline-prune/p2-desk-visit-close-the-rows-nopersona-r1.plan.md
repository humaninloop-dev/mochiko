## FILES-READ:

`README.md`, `architecture/concerns.md`, `architecture/spine.md`, `specs/FEAT-011-audit-log/architecture-delta.md`, `src/crewboard/api/main.py`, `src/crewboard/api/middleware.py`, `src/crewboard/audit/record.py`, `src/crewboard/api/rotas.py`, `src/crewboard/api/swaps.py`, `src/crewboard/api/timeoff.py`, `src/crewboard/api/staff.py`, `src/crewboard/api/billing.py`, `src/crewboard/api/reports.py`, `docs/changelog.md`, `docs/metrics-2026-08.md`

---

# Action plan — asynchronous shelf walk, crewboard, 2026-09-09

## Up front: one thing I will not do as asked

You asked me to flip AX-009 to `built` and copy the as-built line out of the FEAT-011 delta. **I would not write that line, because it isn't true of the code.** The delta's target was a single `AuditMiddleware` as the only writer. What is in `src/` is the per-router `audit.record()` call that the desk explicitly rejected on 2026-07-20 — and only two of the five mutating routers make it:

- `src/crewboard/api/main.py:9-11` — a TODO parking `AuditMiddleware`, dated 2026-08-27, five days before the changelog says FEAT-011 shipped.
- `src/crewboard/api/middleware.py:1` — docstring states no audit middleware exists.
- `src/crewboard/api/rotas.py:20,32` and `src/crewboard/api/swaps.py:19` call `record.record()`.
- `src/crewboard/api/timeoff.py`, `staff.py`, `billing.py` — no audit call at all. Staff records are the personal-data table (`staff.py:1`); its create/patch/delete write nothing to `audit_event`.

Marking that `built` would put a false line in the exact view you're using to judge health, and it would silently reverse a ruling you made at the desk. My default is to record what is actually there and flag it. Everything else in the card I would do in full. The stop and the branches are in Phase 5.

---

## Phase 1 — Read and reconcile (no writes)

Read, in this order: `README.md`, `architecture/concerns.md`, `architecture/spine.md`, the FEAT-011 delta, then every file under `src/crewboard/api/` and `src/crewboard/audit/record.py`, then `docs/changelog.md` and `docs/metrics-2026-08.md`. (Done above.)

Then do three reconciliation passes before touching anything:

1. **Ledger vs code** for every row claiming `built` or `in-flight`.
2. **Ledger vs metrics** for every row carrying an upgrade trigger — check each trigger against the August numbers.
3. **Ledger vs changelog** for status staleness.

I would attempt to locate and run `tests/audit/test_every_mutation_audited.py` (cited at `concerns.md:68`). The ledger marks it `(planned)` and this checkout has no `tests/` tree, so I expect it not to exist. If it does exist in the full repo and I can run it, I expect it to **fail on `timeoff`, `staff` and `billing`** — one missing `audit_event` row per router. I would not report a test result I did not actually get; if I can't run it, the code reading above is the evidence and I'd say so in those terms.

**Findings this phase produces** (all carried into Phase 6):

- **AX-010 Search: its upgrade trigger has already fired, on both limbs.** The trigger is ">50 venues for one business, OR staff-picker p95 above 800 ms for any business". August shows The Hearth Group at **63 venues** and its staff-picker p95 at **1,230 ms**. The row still reads `not-now`. Nobody caught this at the 2026-08-30 walk.
- **AX-005 looks stale and may be in drift.** It reads `in-flight (FEAT-014)`, but the changelog has FEAT-014 shipped 2026-09-05 and AX-003 already cites it as-built. Separately, the ruling says *every* provider send carries an idempotency key; `src/crewboard/api/billing.py:16,27` calls Stripe inline with none. The spine permits those inline Stripe calls (`spine.md:27`), so this is not an AX-003 violation — it's an AX-005 gap.
- **AX-011 trigger has not fired**: report p95 780 ms against a 2 s threshold.
- **AX-012 has an empty upgrade trigger** (`concerns.md:94`) — the field is blank.

## Phase 2 — Decide how to close the eight rows

The eight open rows are AX-013 through AX-020, in ledger order.

You said to mark each one `decided` and to skip any that obviously don't fit a product our size. Those two instructions pull apart, so here is the call I'd make and why: **I would close all eight, but using the ledger's own stance vocabulary** — `decided`, `not-now`, `n-a` — rather than stamping `decided` on rows where the honest ruling is "not yet". AX-006, AX-007, AX-010 and AX-012 already set that precedent, and `not-now` is what carries an upgrade trigger, which is the part that makes the row useful six months from now. I'd also **not literally skip** the doesn't-apply rows: I'd rule them `n-a` with a reason, so nothing is left open and there's no ambiguity on the plane about whether I ran out of time. If you'd rather every row said `decided`, that's a one-word change per row and I'd make it on your say-so.

## Phase 3 — Write the eight rulings

**File written: `architecture/concerns.md`** (in place, ledger order preserved).

Each row gets: stance, status, ruling with today's date, a one-line rationale tied to the walk notes or the metrics, an upgrade trigger where the stance is `not-now`, and — because these were not ruled at the desk — a provenance marker.

**AX-013 Feature flags → `not-now`.** Two flags a quarter does not pay for a flag system, and the budget freeze rules out a vendor anyway. Ruling: a flag stays an environment variable holding the pilot `business_id` list, read at request time; removing the flag is part of closing the feature that introduced it. Trigger: needing to flip a flag *without* a deploy (an incident kill-switch), or more than five live flags at once, or more than one flag change a week. I'd note that the Q4 widget (AX-016) is the first thing likely to want a no-deploy kill-switch.

**AX-014 Localisation → `not-now`.** UK-only, one Irish prospect on an English UI. No i18n framework. Trigger: a signed customer needing a non-English UI. Separate note, not a build: the Irish prospect brings **EUR**, which lands on Stripe and PayFlow, not on the UI — worth a line so it isn't mistaken for a localisation problem when it turns up as a billing one.

**AX-015 Experimentation → `n-a`.** No product ask; the PM reads adoption off the monthly metrics doc. Sixty-four businesses cannot power a split test. Trigger to reopen: a product decision that genuinely needs a split population.

**AX-016 Rate limiting → `decided`, and it's the one I'd build first.** This is the only open row with an incident behind it: INC-19 took an `api` instance to 100 % CPU for twenty minutes, and there are only two instances. The Q4 public rota widget is token-authenticated and embeddable, i.e. the first properly hostile surface. Ruling: per-identity limiting at the `api` edge using the **existing Redis** (no new service — the freeze holds): a budget per session, a budget per business across all its sessions, and a tighter separate budget per widget token; over-limit returns 429 with `Retry-After`; limits are config, not code. Must land before the widget ships. Enforcement: a test that a single identity exceeding its budget gets 429 while a second identity in the same business is unaffected.

**AX-017 Backups and restore → `decided`.** A seven-day snapshot with a restore that has never been rehearsed is not a backup, it's a hope. Ruling: enable point-in-time recovery (already on the plan — no new spend), and rehearse a restore into a scratch database once a quarter, with a written RPO and RTO produced from the first rehearsal rather than assumed. The rehearsal is the deliverable; no new infrastructure. Cross-reference: AX-009 asks for twelve months of audit retention, which seven-day snapshots do not underwrite — the audit table's durability rides on PITR plus the purge job, not on snapshots.

**AX-018 Identity federation → `decided`, conditional on the contract.** Ruling: OIDC login for Google Workspace only, enabled per business (business configures issuer + email domain; managers in that domain sign in through it). The session cookie and the `manager`/`staff` per-venue roles are unchanged downstream, so AX-002 stands. Explicitly **not** ruled: a general SAML or multi-IdP identity layer, or SSO for staff. **This is my second stop point** — it's a build justified by one unsigned prospect, and it's the largest single piece of work in the eight. Default I'd write: `decided`, marked *conditional on the 40-venue group signing*. If it signs, work starts as specced. If it doesn't, the row reverts to `not-now` with the trigger "any signed customer requiring SSO" — I'd write that fallback into the row now so no one has to re-walk it.

**AX-019 Data partitioning, export and deletion → `decided`.** Three subject requests done by hand with raw SQL is a per-request risk of touching the wrong tenant, and the SSO prospect's procurement has asked for export on termination — so this rides with AX-018. Ruling: per-business export and delete as `worker` jobs that walk the `business_id` foreign-key graph from one manifest, export to object storage behind a signed expiring link, and delete in two steps (mark, then purge after a hold window) with a row count reported per table. **Conflict I'd surface rather than resolve alone:** erasure meets AX-009's append-only audit log, which by the delta has *no UPDATE or DELETE grant for the app role* and stores request bodies — meaning staff names, mobiles and emails end up inside `audit_event.before/after`. My recommendation in the row: keep personal fields out of the audit payload at write time (store entity ids, not bodies), and give the purge job a distinct database role for redaction of anything historical. Flagged for you because it changes what AX-009 stores.

**AX-020 Observability → `decided`, cheap version.** INC-31 was reported by a customer forty minutes in; queue lag p95 is 41 s post-resize and nothing alerts on it. Ruling: stay on Render logs + Sentry (freeze), and add three alerts — sustained Celery queue lag over a threshold set from the 41 s baseline, provider-send failure rate, and audit-write failures (the FEAT-011 delta says those are "logged and alerted", which today means logged only). Distributed tracing stays `not-now`; trigger: an incident that logs and Sentry cannot localise, or a fourth service in the request path.

## Phase 4 — Ledger housekeeping (same file)

- Append to the walk-history line at `concerns.md:3-5`: `2026-09-09 (AX-013 – AX-020 ruled asynchronously at the CTO's request; not a desk visit)`. These rulings need to be distinguishable from ones made at the desk.
- Update the shelf-coverage line: 22 of 22 rows would be on the ledger only if secrets management and dependency/supply-chain policy get rows. I'd **open** those as AX-021 and AX-022 with stance `open` and no ruling — adding rows is safe, ruling on them uninvited isn't, and they weren't part of your eight.
- **AX-012's blank upgrade trigger**: I'd propose text (`the first customer contractual retention requirement, or any table exceeding a size where per-business deletion cannot finish in one job window`) in the flag list rather than editing a row you've already ruled.

## Phase 5 — AX-009 (the stop)

**What I would write instead of `built`:**

```
## AX-009 Audit log
- **Stance**: decided · **Status**: in-flight (FEAT-011 partially landed)
- **Ruling**: unchanged (ruled 2026-07-20)
- **As-built** (FEAT-011, shipped 2026-09-02): NOT as ruled. No AuditMiddleware exists
  (api/main.py TODO, parked 2026-08-27). rotas and swaps call audit.record() directly;
  timeoff, staff and billing record nothing.
- **Drift**: material. Three of five mutating routers are unaudited, including staff
  (personal data). The shipped shape is the per-router pattern rejected at the desk on
  2026-07-20, for the reason it is now failing: a router that forgets the call silently
  drops a category. audit_event append-only grants and the twelve-month worker purge
  are unverified in this checkout.
- **Enforcement**: tests/audit/test_every_mutation_audited.py still planned; it would
  fail today on timeoff, staff, billing.
```

**What I would confirm with you, and the branches:**

- *Ruling A — the middleware is still the ruling.* AX-009 stays `in-flight`, a FEAT-011b lands `AuditMiddleware` plus the before/after state the parking note says `billing` and `staff` don't yet attach, and the enforcement test is written first so it fails and then passes. This is my default and what I'd assume if I hear nothing.
- *Ruling B — accept per-router audit as the new ruling.* Then the 2026-07-20 ruling is amended on the record, not quietly, and the enforcement test still has to exist, because per-router is exactly the pattern that needs a test to hold it up. Three routers still need the call added.
- *Ruling C — you tell me to write `built` and the delta's line anyway.* I'd say once that it makes the health view wrong, and then do it, with the code evidence recorded in the Drift field so the next walk finds it.

I would **not** write the missing `audit.record()` calls into `timeoff.py`, `staff.py` and `billing.py` on my own initiative. That's a code change to fix a drift under a ruling that may be about to change, and it isn't what you asked for.

**No spine edit.** Nothing new was built this week, and `decided`-but-not-built rows don't move the spine; `api` already lists an `audit` module. `worker` should gain the audit purge task and `db` the `audit_event` table only when FEAT-011 actually lands as ruled. I'd leave `Last confirmed: 2026-08-30` alone, since today isn't a desk visit.

## Phase 6 — Delegation

None. This is roughly fifteen small files and one document; splitting it across workers would cost more in reconciliation than it saves, and the cross-row findings (AX-010's fired trigger, AX-019 against AX-009) only surface when one reader holds the whole ledger at once. I'd note that I can't launch the multi-agent review myself either — if you want the ledger edits reviewed before you land, `/code-review ultra` is yours to trigger.

## Phase 7 — What I'd report

A short note at the top of the ledger and a summary in chat, ordered so the first three things are the ones you'd want before the plane doors close:

1. **FEAT-011 did not ship as ruled** — three of five mutating routers, including staff personal data, write no audit record; AX-009 left `in-flight` with drift recorded, not `built`. Awaiting your ruling A/B/C.
2. **AX-010's upgrade trigger fired and was missed** — 63 venues and 1,230 ms p95 at The Hearth Group, both limbs, against a row still reading `not-now`.
3. **AX-005 is stale and possibly in drift** — FEAT-014 shipped 2026-09-05; billing's inline Stripe calls carry no idempotency key.
4. All eight rows closed, in ledger order: 013 not-now, 014 not-now, 015 n-a, 016 decided, 017 decided, 018 decided (conditional on the contract), 019 decided, 020 decided — with the note that I used the ledger's stances rather than stamping all eight `decided`, and that "skip" became `n-a`-with-a-reason so no row is left open.
5. Build order if the eight become work: **AX-016 first** (incident history plus the Q4 widget deadline), then AX-020's three alerts (cheap, and INC-31 says they're overdue), then AX-017's rehearsal, then AX-018/AX-019 together on the contract.
6. Open questions parked for you: the AX-019 erasure vs AX-009 append-only conflict; AX-012's blank trigger; AX-021/AX-022 opened but unruled.