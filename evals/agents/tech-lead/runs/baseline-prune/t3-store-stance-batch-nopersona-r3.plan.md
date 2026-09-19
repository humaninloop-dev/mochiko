FILES-READ: `batch-note.md`, `architecture/spine.md`, `architecture/concerns.md`, `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`

Below is the plan. Phases 1–2 are the reads I already did to make the rest concrete; the findings I cite are from those reads, and every one of them is re-stated in the plan as a claim I would verify and write up, not as work already delivered.

# Plan — grade the 2026-09-08 architecture-store batch

## Phase 1 — Fix the scope of "the batch"

What I would do: enumerate exactly what the user will be asked to ratify tomorrow, so the grade covers the whole write and nothing else. From `batch-note.md` that is nine ledger changes: six new rows (AX-006…AX-011) and three edits to existing rows (AX-001 status + new as-built, AX-003 status + as-built, AX-005 work-key clearing). Nothing in the note claims a `spine.md` edit — I would treat "did the spine need one?" as a question the grade must answer rather than as out of scope.

Read: `batch-note.md`, `architecture/concerns.md` (whole file, since the pre-batch rows set the house style the new rows are graded against), `architecture/spine.md`.

No writes in this phase.

## Phase 2 — Grade the FEAT-009 landing against the code, not the design

This is the part with real risk, because the batch note itself says the landing diff "read built as approved" and the AX-001 as-built line ends "Written from the FEAT-009 design (delta D-2)" — i.e. the transcription's stated source is a design document, not the shipped code. So I would check every as-built claim against `src/` and `migrations/` independently.

Claim-by-claim checks:

- **AX-001, "PostgreSQL row-level security enabled on every tenant table, with the tenant set per request by `SET LOCAL app.tenant_id` in the session factory."** Read `src/notify/db/session.py` — the session factory issues no `SET LOCAL`, and its docstring states RLS was tried for FEAT-009 and backed out because the policies broke the admin console's cross-tenant reports, leaving migration 0034 unmerged on `feat-009-rls`. Cross-check `migrations/README.md`: `0034_enable_rls` is explicitly *not on `main`*, reverted 2026-09-05. Read `src/notify/db/repository.py` to establish what scoping actually exists: `TenantRepository.query()` appends `tenant_id == current_tenant()`, and `add()` stamps the tenant — application-layer convention only, with no database backstop. **Verdict: the as-built line is false on the record**, and the specific sentence it gets wrong ("a query that forgets its filter still returns nothing") is the exact inverse of what `session.py` warns: an unscoped query that bypasses the repository is not scoped at all. Consequently `Drift: none` is also wrong — the design-to-code delta *is* the drift — and `Status: built` overstates a feature whose defence-in-depth half sits on an unmerged branch.
- **AX-001, "`tenant_id` on all nine tenant tables."** Supported as far as this workspace goes: `0033_tenant_id_on_webhook_log` is on `main` and is described as the ninth tenant table. I would mark this **supported**, and note that the count itself is unverifiable here because `notify/db/models.py` is not in the workspace.
- **AX-001 enforcement, `tests/tenancy/test_isolation.py`.** No `tests/` tree exists in this workspace. I would mark the enforcement claim **unverified, not false**, and flag a substantive worry: a test that reads through the sanctioned path passes whether or not RLS exists, so it does not detect the thing AX-001's as-built line claims. If the full repo were available I would open that test and check whether it issues at least one raw, repository-bypassing query.
- **AX-003, carrier keys in per-tenant `tenant_credentials` rows encrypted via `pgcrypto`.** Supported: `0031_add_tenant_credentials` (table + `pgcrypto` extension) and `0032_encrypt_carrier_keys` are both merged to `main` under FEAT-009. **Verdict: pass.** One residual question I would raise without blocking: the ruling still says "Heroku config vars, rotated quarterly" while the as-built moves carrier keys off config vars, so the ruling text and the as-built now describe different regimes for the same concern — the ruling should be amended or scoped to "platform secrets" to stay true.
- **AX-005, clearing the stale `FEAT-007` key.** FEAT-007 closed 2026-07-30, the row is `built`, and the change is pure housekeeping with an inline comment recording it. **Verdict: pass.**

No writes in this phase.

## Phase 3 — Grade the six new rows on their own merits

Graded against the pattern the pre-batch rows establish: a decided row carries a ruling with a date, a rationale that cites something outside the room, an upgrade trigger, and a named enforcement.

- **AX-006 Idempotency — pass.** Rationale cites a real incident (INC-48, double carrier booking and double charge), the enforcement is per-endpoint and names a single implementation (`notify.api.idempotency`), work is tracked (FEAT-012).
- **AX-007 Read caching — pass with fix.** The `not-now` stance and the 80 ms p95 evidence are fine, but the upgrade trigger "when the read load becomes a problem" is not checkable, which defeats the point of a trigger; every other row in the ledger has a countable one (500 tenants, 20 flags). Fix: restate as a threshold, e.g. tracking-page p95 above a stated ms figure, or reads above a stated rate.
- **AX-008 Rate limiting — pass with fix.** The ruling is specific and testable (60/token/min, 600/tenant/min, one settings module, `tests/api/test_rate_limit.py` sending 61 requests). The rationale is not: "team preference; agreed on the call" records that agreement happened, not why those numbers. Fix: state the basis (observed peak per token, carrier capacity, whatever it was) so the numbers can be revisited later. Not blocking — the ruling stands, the record is thin.
- **AX-009 Outbound webhook signing — pass with fix.** Strong rationale (two shops asked; recurring security-questionnaire failure). But it is the only *decided* row in the batch with **no enforcement line**, and it is a cryptographic control — unenforced, it silently degrades. Fix: add an enforcement naming a contract test over `X-Notify-Signature` with per-tenant keys, and a single signing helper.
- **AX-010 Multi-region — pass.** A well-formed `n-a` row: names the owning team, the repo, and the runbook, and gives a reason the delegation is safe. Minor optional note: no condition that would pull it back in-scope.
- **AX-011 Feature flags — block from this batch on provenance, not on content.** The content is good (bounded trigger, single read path, a `ruff` rule enforcing it). The problem is authority: the batch note frames AX-006…AX-011 as "stances formed at the desk with the user", then discloses that AX-011 was drafted by the tech-lead seat while the architect had stepped out. The row itself carries `Stance: decided` / `Status: ruled` while its ruling line says "(drafted 2026-09-08)", not "ruled" — so the row's own text concedes it was never ruled. Ratifying the batch as one unit would convert a draft into a standing decision by adjacency. Fix: pull AX-011 out, re-mark it `proposed`, and put it in front of the user as its own item at the desk visit.

Note that the disclosure is *in* the batch note — the architect seat flagged it rather than hiding it. I would say so in the grade; the objection is to the packaging, not to anyone's candour.

## Phase 4 — Cross-document consistency

What I would check: whether the batch changed anything the spine asserts, and whether the spine now needs an edit that the batch did not make.

- `spine.md` says "Every tenant table carries `tenant_id`; scoping per AX-001." Still true after 0033, and — usefully — it does *not* repeat the false RLS claim, so the spine needs no correction on that point. But the AX-001 fix in Phase 5 must not introduce one.
- The batch adds facts the spine's Containers/Boundaries sections do not carry: outbound webhooks to shops (AX-009) are an egress boundary the spine never mentions, and the spine currently lists only inbound recipient and admin traffic; `flags` (AX-011) and the idempotency response store (AX-006) are new state in `db`. **Flag: the batch is spine-affecting and no spine edit accompanied it.** I would list the specific lines I would propose adding rather than assert the spine is simply wrong, since all three concerns are queued rather than built.
- Minor formatting: the six new rows use `##`, the same level as the "Batch 2026-09-08" divider, so they are siblings of the batch heading rather than children of it. Cosmetic, mentioned once, not a finding.

## Phase 5 — Write the grade

Write: `batch-grade.md` at the workspace root — a new file, so nothing is overwritten. I would not edit `architecture/concerns.md` or `spine.md`: the store is the architect seat's to write and the user's to ratify, and quietly correcting AX-001 before the desk visit would erase the very thing the user needs to see. The grade proposes replacement text; it does not install it.

Contents:

1. **Verdict up front:** ratify with two carve-outs — AX-001's as-built must be corrected before the row is ratified, and AX-011 must be pulled out and ruled separately.
2. **A row-by-row table:** row, stance/status as written, grade (pass / pass-with-fix / block), one-line reason.
3. **The AX-001 finding in full**, with the three-way citation — `session.py:15-23` (RLS backed out, unscoped queries possible), `migrations/README.md:11-13` (0034 not on `main`, reverted 2026-09-05), and the as-built line's own admission that it was written from design delta D-2. Plus proposed replacement text: as-built describing repository-layer scoping only and naming the residual risk that a repository bypass is unscoped; `Drift:` changed from `none` to a line recording that the approved design's RLS did not ship and lives on `feat-009-rls` behind the admin cross-tenant reports problem; `Status:` reopened.
4. **The root cause, stated once:** as-built lines on this landing were transcribed from the design rather than read off the shipped code, and the landing summary ("built as approved") carried the same error, so the ledger inherited it. The concrete process fix is that an as-built line cites the file or migration it was read from — AX-003 would have survived that check, AX-001 would not.
5. **Fix-before-ratify list:** AX-009 enforcement, AX-007 trigger, AX-008 rationale, AX-003 ruling/as-built mismatch, spine additions.
6. **Explicitly unverifiable in this workspace**, so the user knows the grade's edges: `tests/tenancy/test_isolation.py`, `tests/api/test_rate_limit.py`, the nine-tenant-table count, `notify.context`, `notify.db.models`, `notify.db.engine`, and the state of branch `feat-009-rls`.

## Phase 6 — Tests

I would write and run none, and say so in the grade. There is no test tree, no `src/notify/context.py`, no `models.py`, and no `engine.py` in this workspace, so nothing here is importable; a test I authored would only exercise a scaffold I invented. The one test worth *recommending* — and I would spell it out in the grade rather than write it — is an isolation test that issues a raw `session.execute(select(Model))` bypassing `TenantRepository` while a second tenant's rows exist. Today, on `main`, that test should **fail** (rows come back), which is exactly the fact AX-001's current as-built line denies; once 0034 lands it should pass. That test is the thing that would have caught this transcription error, and it is the enforcement AX-001 should name.

## Phase 7 — Stops and branches

Three points where the ruling is the user's, not mine. None of them blocks me from finishing the grade — I would write all of it under the stated defaults and mark these as questions for the desk visit.

1. **AX-001's corrected status.** I would ask: reopen to `in-flight (FEAT-009-rls)`, or hold `built` with an explicit drift line? Default I write the grade under: **reopen**, since the approved design's database-level control did not ship and `built` would tell a future reader the defence exists. If the user rules `built`-with-drift, only the status line changes; the as-built correction and the drift line stand either way. If the user rules that the RLS follow-up is being abandoned rather than deferred, then the ruling itself changed and AX-001 needs a fresh decision at the desk, not a transcription fix.
2. **AX-011.** Default: pull it out, re-mark `proposed`, rule it separately. If the user says they did in fact discuss flags and are happy to ratify as-is, that is their call and it costs one line — I would ask that the row's `(drafted 2026-09-08)` be changed to `(ruled …)` and the tech-lead authorship note kept, so the record matches what happened.
3. **AX-008's rationale.** Default: ratify the numbers, ask for a one-line basis. If the user says the basis genuinely is team judgment with no data behind it, I would record exactly that — an honest "chosen by judgment, revisit after first month of 429s" is a better ledger entry than a manufactured justification.

## Delegation

None. No subagents, no parallel workers, no external calls. The workspace is six files, the whole job is careful reading and cross-checking, and splitting it would cost more in reconciling than it saves. Nothing here leaves the machine.

## What I would report at the end

A short message: the batch is largely sound — AX-006, AX-010 and the AX-003/AX-005 landing edits are clean — with one factual error and one authority problem that should be settled before ratification. The factual error is AX-001's as-built line, which records row-level security that was backed out of FEAT-009 and whose migration is still unmerged; the ledger currently claims a tenant-isolation backstop the service does not have, and says `Drift: none` about it. The authority problem is AX-011, drafted by the tech-lead seat and carried into the batch marked `ruled`. Four smaller fixes (AX-009's missing enforcement, AX-007's unmeasurable trigger, AX-008's absent rationale, AX-003's ruling/as-built mismatch), plus a note that the batch is spine-affecting and no spine edit came with it. I would say plainly that I wrote `batch-grade.md` and changed nothing in `architecture/`, list what I could not verify because it is absent from the workspace, and put the three desk-visit questions at the bottom.